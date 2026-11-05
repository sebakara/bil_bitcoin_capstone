use bil_bitcoin_capstone::{rpc, AppResult};
use bitcoin::{consensus::encode::deserialize_hex, Transaction, Txid};
use std::str::FromStr;

fn main() -> AppResult<()> {
    let supplied_text = std::env::args()
        .nth(1)
        .ok_or("Usage: cargo run --bin inspect -- <TXID>")?;
    let expected_txid = Txid::from_str(&supplied_text)?;

    // Only wallet transactions can be queried through gettransaction.
    let response = rpc(Some("class_alice"), &["gettransaction", &supplied_text])?;
    let json: serde_json::Value = serde_json::from_str(&response)?;
    let hex = json["hex"].as_str().ok_or("Missing hex in wallet RPC result")?;
    let confirmations = json["confirmations"]
        .as_i64()
        .ok_or("Missing confirmations in wallet RPC result")?;

    // Parse bytes and calculate their TXID; this is NOT script validation.
    let transaction: Transaction = deserialize_hex(hex)?;
    let computed_txid = transaction.compute_txid();
    if computed_txid != expected_txid {
        return Err(format!(
            "TXID mismatch: supplied {expected_txid}, decoded {computed_txid}"
        )
        .into());
    }

    println!("TXID verified against decoded bytes: {computed_txid}");
    println!("WTXID: {}", transaction.compute_wtxid());
    println!("Confirmations (Bitcoin Core wallet RPC): {confirmations}");
    println!("Inputs: {}", transaction.input.len());
    println!("Outputs: {}", transaction.output.len());
    println!("Virtual size: {} vB", transaction.vsize());

    for (index, output) in transaction.output.iter().enumerate() {
        println!("Output {index}: {} sats", output.value.to_sat());
    }
    println!("This does not validate scripts, spent-input amounts, or chain consensus.");
    println!("Output amounts alone cannot establish the transaction fee.");
    Ok(())
}
