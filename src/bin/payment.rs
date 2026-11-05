//! Preview a Regtest payment by default; use `--send` for the real test transfer.
use bil_bitcoin_capstone::{
    address, balance_sats, mine,
    sats_to_btc, send_btc, wallet_tx_fee_sats, AppResult,
};

const PAYMENT_SATS: u64 = 100_000; // 0.001 BTC; not the network fee.
const ALICE_MIN_SATS: u64 = 200_000;
const ALICE_TOP_UP_BTC: &str = "0.01000000";

fn main() -> AppResult<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let should_send = match args.as_slice() {
        [] => false,
        [flag] if flag == "--preview" => false,
        [flag] if flag == "--send" => true,
        _ => return Err("Usage: cargo run --bin payment -- [--preview | --send]".into()),
    };

    // Run the `setup` executable first. Even preview mode is read-only:
    // no wallet creation/loading, mining, or broadcast occurs here.
    let alice_before = balance_sats("class_alice")?;
    let bob_before = balance_sats("class_bob")?;

    println!("Network: REGTEST ONLY");
    println!("Alice confirmed before: {} BTC", sats_to_btc(alice_before));
    println!("Bob confirmed before:   {} BTC", sats_to_btc(bob_before));
    println!("Planned amount to Bob:  {} sats ({} BTC)", PAYMENT_SATS, sats_to_btc(PAYMENT_SATS));

    if !should_send {
        println!("\nPREVIEW ONLY: no transaction was sent, no blocks were mined.");
        if alice_before < ALICE_MIN_SATS {
            println!("Alice would first be funded from the miner if you choose --send.");
        }
        println!("Actual fee unknown until a transaction is built and sent by Bitcoin Core.");
        println!("Use: cargo run --bin payment -- --send");
        return Ok(());
    }

    // Real activity happens only with an explicit --send flag.
    let miner_address = address("class_miner")?;
    if alice_before < ALICE_MIN_SATS {
        let alice_address = address("class_alice")?;
        let funding_txid = send_btc("class_miner", &alice_address, ALICE_TOP_UP_BTC)?;
        mine(1, &miner_address)?;
        println!("Funded Alice on Regtest. Funding TXID: {funding_txid}");
    }

    let bob_address = address("class_bob")?;
    let payment_txid = send_btc("class_alice", &bob_address, &sats_to_btc(PAYMENT_SATS))?;
    mine(1, &miner_address)?;
    let actual_fee_sats = wallet_tx_fee_sats("class_alice", &payment_txid)?;

    let alice_after = balance_sats("class_alice")?;
    let bob_after = balance_sats("class_bob")?;
    println!("\nPAYMENT CONFIRMED ON REGTEST");
    println!("Payment TXID: {payment_txid}");
    println!("Bob received: {} sats", PAYMENT_SATS);
    println!("Actual transaction fee: {} sats ({} BTC)", actual_fee_sats, sats_to_btc(actual_fee_sats));
    println!("Alice confirmed after: {} BTC", sats_to_btc(alice_after));
    println!("Bob confirmed after:   {} BTC", sats_to_btc(bob_after));
    println!("Inspect it: cargo run --bin inspect -- {payment_txid}");
    println!("Note: each additional --send creates another real Regtest payment.");
    Ok(())
}
