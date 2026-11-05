use bil_bitcoin_capstone::{fee_sats, parse_btc_to_sats, sats_to_btc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sats = parse_btc_to_sats("0.00025000")?;
    let fee = fee_sats(50_000, 49_200)?;

    println!("25,000 sats = {} BTC", sats_to_btc(sats));
    println!("Sample fee = {fee} sats");
    Ok(())
}
