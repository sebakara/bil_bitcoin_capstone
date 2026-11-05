use serde_json::{value::RawValue, Value};
use std::error::Error;
use std::process::Command;

pub type AppResult<T> = Result<T, Box<dyn Error>>;

/// Run bitcoin-cli against the local Regtest node.
/// Command arguments are passed separately (not interpolated into a shell).
pub fn rpc(wallet: Option<&str>, args: &[&str]) -> AppResult<String> {
    let mut command = Command::new("bitcoin-cli");
    command.arg("-regtest");
    if let Some(name) = wallet {
        command.arg(format!("-rpcwallet={name}"));
    }
    let output = command.args(args).output().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            std::io::Error::new(e.kind(), "bitcoin-cli not found in PATH")
        } else {
            e
        }
    })?;
    if !output.status.success() {
        return Err(format!(
            "bitcoin-cli failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ).into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

pub fn rpc_json(wallet: Option<&str>, args: &[&str]) -> AppResult<Value> {
    Ok(serde_json::from_str(&rpc(wallet, args)?)?)
}

/// Create wallet if absent; otherwise load it only when not already loaded.
/// Does not reset existing wallet data.
pub fn ensure_wallet(name: &str) -> AppResult<()> {
    let loaded: Vec<String> = serde_json::from_str(&rpc(None, &["listwallets"])?)?;
    if loaded.iter().any(|wallet| wallet == name) {
        println!("Wallet already loaded: {name}");
        return Ok(());
    }

    let directory = rpc_json(None, &["listwalletdir"])?;
    let exists = directory["wallets"]
        .as_array()
        .ok_or("Invalid listwalletdir response")?
        .iter()
        .any(|entry| entry["name"].as_str() == Some(name));

    if exists {
        rpc(None, &["loadwallet", name])?;
        println!("Loaded wallet: {name}");
    } else {
        rpc(None, &["createwallet", name])?;
        println!("Created wallet: {name}");
    }
    Ok(())
}

pub fn address(wallet: &str) -> AppResult<String> {
    rpc(Some(wallet), &["getnewaddress"])
}

pub fn mine(blocks: u32, receive_address: &str) -> AppResult<()> {
    rpc(None, &["generatetoaddress", &blocks.to_string(), receive_address])?;
    Ok(())
}

/// Return the wallet's balance requiring at least one confirmation.
/// Bitcoin Core getbalance defaults to minconf=0; specify 1 explicitly.
pub fn balance_sats(wallet: &str) -> AppResult<u64> {
    let text = rpc(Some(wallet), &["getbalance", "*", "1"])?;
    Ok(parse_btc_to_sats(&text)?)
}

/// Ask Bitcoin Core wallet to select coins, sign and broadcast a real Regtest transaction.
/// This does not estimate or return the fee; get it later from gettransaction.
pub fn send_btc(from: &str, to: &str, amount_btc: &str) -> AppResult<String> {
    let sats = parse_btc_to_sats(amount_btc)?;
    if sats == 0 {
        return Err("Payment amount must exceed zero sats".into());
    }
    rpc(Some(from), &["sendtoaddress", to, amount_btc])
}

/// Format integer satoshis as BTC without floating point.
pub fn sats_to_btc(sats: u64) -> String {
    format!("{}.{:08}", sats / 100_000_000, sats % 100_000_000)
}

/// Parse decimal BTC text with at most 8 fractional digits (no exponent).
pub fn parse_btc_to_sats(text: &str) -> Result<u64, String> {
    let text = text.trim();
    if text.is_empty() || text.starts_with('-') || text.starts_with('+') {
        return Err("BTC amount must be unsigned decimal text".into());
    }
    let mut parts = text.split('.');
    let whole = parts.next().unwrap_or("");
    let fraction = parts.next().unwrap_or("");
    if parts.next().is_some() || whole.is_empty() ||
       !whole.bytes().all(|ch| ch.is_ascii_digit()) ||
       !fraction.bytes().all(|ch| ch.is_ascii_digit()) ||
       fraction.len() > 8 {
        return Err("Invalid BTC decimal amount (maximum 8 decimal places)".into());
    }
    let units: u64 = whole.parse().map_err(|_| "BTC whole part overflow")?;
    let fractional = if fraction.is_empty() {
        0
    } else {
        let base: u64 = fraction.parse().map_err(|_| "BTC fraction overflow")?;
        base * 10u64.pow((8 - fraction.len()) as u32)
    };
    units.checked_mul(100_000_000)
        .and_then(|value| value.checked_add(fractional))
        .ok_or_else(|| "BTC amount overflow".into())
}

/// Bitcoin transaction fee = sum(inputs) - sum(outputs).
/// This function only computes a fee from known numeric values.
pub fn fee_sats(inputs: u64, outputs: u64) -> Result<u64, &'static str> {
    inputs.checked_sub(outputs).ok_or("Outputs exceed inputs")
}

/// Borrow raw JSON number text to avoid f64 rounding when reading BTC fees.
#[derive(serde::Deserialize)]
pub struct TxFee<'a> {
    #[serde(borrow)]
    pub fee: Option<&'a RawValue>,
}

/// Get a Bitcoin Core outgoing-wallet fee from a JSON gettransaction response.
/// Bitcoin Core reports a negative BTC amount; convert it to positive sats.
pub fn outgoing_fee_sats_from_json(response: &str) -> AppResult<u64> {
    let tx: TxFee<'_> = serde_json::from_str(response)?;
    let token = tx.fee.ok_or("No outgoing fee field for this transaction")?.get();
    let positive = token
        .strip_prefix('-')
        .ok_or("Expected a negative outgoing-wallet fee")?;
    Ok(parse_btc_to_sats(positive)?)
}

/// Read an actual outgoing fee, not a fee inferred from output totals.
pub fn wallet_tx_fee_sats(wallet: &str, txid: &str) -> AppResult<u64> {
    let response = rpc(Some(wallet), &["gettransaction", txid])?;
    outgoing_fee_sats_from_json(&response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_roundtrip() {
        assert_eq!(sats_to_btc(25_000), "0.00025000");
        assert_eq!(parse_btc_to_sats("0.00025000").unwrap(), 25_000);
    }

    #[test]
    fn decimal_input_validation() {
        assert!(parse_btc_to_sats("0.000000001").is_err());
        assert!(parse_btc_to_sats("-1").is_err());
    }

    #[test]
    fn fees_are_checked() {
        assert_eq!(fee_sats(50_000, 49_200), Ok(800));
        assert!(fee_sats(10, 11).is_err());
    }

    #[test]
    fn decimal_amount_edges() {
        assert_eq!(parse_btc_to_sats("0.00000001").unwrap(), 1);
        assert_eq!(parse_btc_to_sats("1.25").unwrap(), 125_000_000);
        assert!(parse_btc_to_sats("0.000000001").is_err());
        assert!(parse_btc_to_sats("184467440738").is_err());
        assert!(parse_btc_to_sats("abc").is_err());
    }

    #[test]
    fn fee_arithmetic_edge_cases() {
        assert_eq!(fee_sats(0, 0), Ok(0));
        assert_eq!(fee_sats(100_000, 0), Ok(100_000));
        assert!(fee_sats(99, 100).is_err());
    }

    #[test]
    fn amount_parser_rejects_invalid_formats() {
        for text in ["", " ", "abc", "0.000000001", "-0.1", "+0.1", "1.2.3", "1e-8", "0,01"] {
            assert!(parse_btc_to_sats(text).is_err(), "must reject {text:?}");
        }
        assert_eq!(parse_btc_to_sats("0"), Ok(0));
        assert_eq!(parse_btc_to_sats("1.25000000"), Ok(125_000_000));
    }

    #[test]
    fn outgoing_fee_is_exact() {
        let example = r#"{"fee":-0.00000140,"confirmations":1}"#;
        assert_eq!(outgoing_fee_sats_from_json(example).unwrap(), 140);
        assert!(outgoing_fee_sats_from_json(r#"{"confirmations":1}"#).is_err());
        assert!(outgoing_fee_sats_from_json(r#"{"fee":0.00000140}"#).is_err());
    }
}
