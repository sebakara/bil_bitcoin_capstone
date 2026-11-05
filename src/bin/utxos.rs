//! Show confirmed wallet UTXOs and sum values exactly in satoshis.
use bil_bitcoin_capstone::{parse_btc_to_sats, rpc, sats_to_btc, AppResult};
use serde_json::value::RawValue;

#[derive(serde::Deserialize)]
struct Utxo<'a> {
    txid: String,
    vout: u32,
    #[serde(borrow)]
    amount: &'a RawValue,
    confirmations: u64,
    safe: bool,
}

fn main() -> AppResult<()> {
    for wallet in ["class_alice", "class_bob"] {
        let response = rpc(Some(wallet), &["listunspent", "1"])?;
        let entries: Vec<Utxo<'_>> = serde_json::from_str(&response)?;
        let mut total_sats: u64 = 0;
        let mut safe_sats: u64 = 0;

        println!("\n{wallet}: {} confirmed UTXO(s)", entries.len());
        for (i, item) in entries.iter().enumerate() {
            let sats = parse_btc_to_sats(item.amount.get())?;
            total_sats = total_sats.checked_add(sats).ok_or("UTXO total overflow")?;
            if item.safe {
                safe_sats = safe_sats.checked_add(sats).ok_or("Safe UTXO total overflow")?;
            }
            if i < 10 {
                println!(
                    "{}:{} | {} sats | {} confirmation(s) | safe={}",
                    item.txid, item.vout, sats, item.confirmations, item.safe
                );
            }
        }
        if entries.len() > 10 {
            println!("... {} additional UTXOs not listed individually", entries.len() - 10);
        }
        println!("Confirmed UTXO sum: {} sats ({} BTC)", total_sats, sats_to_btc(total_sats));
        println!("Subset Bitcoin Core marks safe: {} sats", safe_sats);
    }
    println!("\nNote: wallet-reported UTXOs; not the global network's UTXO set.");
    Ok(())
}
