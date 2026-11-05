# BIL instructor notes — combined final session (120 minutes)

| Time | Teach with slides | Have students run |
|---|---|---|
| 0–10 min | Bitcoin Core vs Rust architecture; Regtest-only safeguards | Node verification |
| 10–25 min | BTC/sats, hypothetical vs network fee | `units`; `cargo test` |
| 25–42 min | Wallet, addresses, coinbase maturity | `setup` |
| 42–55 min | Preview vs sending, UTXO transaction flow | `payment` (no flag) |
| 55–80 min | Transactions, confirmations, actual fee | `payment -- --send` |
| 80–98 min | TXID/WTXID, decoding vs validation | `inspect -- TXID` |
| 98–108 min | Wallet UTXO set, change, safety flag | `utxos` |
| 108–120 min | Testing, code reviews, BitDevs Kigali | `cargo test`; review exercise |

**When students' machines are not ready:** Instructor demonstrates Regtest commands via shared screen; students run the deterministic `units` lab on their own system later. Do not spend more than 15 minutes of a two-hour class troubleshooting individual installations.

**Important cautions:** Avoid presenting TXID equality as a full transaction verification; avoid assuming fixed fee values or fixed wallet balances; do not imply wallet `listunspent` is the entire Bitcoin UTXO set. Source has a safe default preview and requires `--send` to broadcast.

**Run-through (before class):** `cargo fmt --all`, `cargo check --all-targets`, `cargo test`, `cargo run --bin units`, `cargo run --bin setup`, `cargo run --bin payment`, `cargo run --bin payment -- --send`, `cargo run --bin inspect -- <TXID>`, `cargo run --bin utxos`.

**BitDevs Kigali teaching prompt:** Pick a small Bitcoin-related PR or BIP; ask students to describe the change, reproduce it in Regtest when relevant, observe actual output, find a test case, and separate facts from assumptions.
