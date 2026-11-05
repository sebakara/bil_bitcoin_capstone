use bil_bitcoin_capstone::{address, balance_sats, ensure_wallet, mine, AppResult};

fn main() -> AppResult<()> {
    for name in ["class_miner", "class_alice", "class_bob"] {
        ensure_wallet(name)?;
    }

    let miner_balance = balance_sats("class_miner")?;
    if miner_balance < 200_000_000 {
        let miner_address = address("class_miner")?;
        mine(101, &miner_address)?;
        println!("Mined 101 Regtest blocks for spendable test funds.");
    } else {
        println!("Miner already funded; no new blocks needed.");
    }
    println!("Setup complete; miner balance is now {} sats.",
             balance_sats("class_miner")?);
    Ok(())
}
