# Student worksheet — Bitcoin + Rust (Sessions 5 & 6)

Work independently on your own **Regtest node**. Record commands and observations, not merely screenshots.

## Exercise 1 — Units and exact fees (no node needed)
- Run `cargo run --bin units`.
- Explain why 25,000 sats equals 0.00025000 BTC.
- Explain why the 800-sat example is **not** a real network transaction fee.
- Change the sample input/output totals to 80,000 and 79,500; predict and verify the result (500 sats).
- Run `cargo test` and identify the test for outputs exceeding inputs.

**Expected:** deterministic textual results; checked subtraction never returns a negative unsigned fee.

## Exercise 2 — Create Bitcoin Core wallets
- Verify `"chain": "regtest"` in `getblockchaininfo`.
- Run `cargo run --bin setup`.
- Verify `class_miner`, `class_alice`, and `class_bob` using `bitcoin-cli -regtest listwallets`.
- Explain why 101 blocks were initially mined.

**Expected:** three wallets and mature test Bitcoin for miner. Block count and balances are dynamic.

## Exercise 3 — Preview and send
- Run `cargo run --bin payment` first. Confirm it does not mine or broadcast.
- Record the displayed balances and planned amount.
- Run `cargo run --bin payment -- --send` exactly once.
- Record the TXID, **actual** fee in sats, and confirmed balances.
- Explain why Bob's received 100,000 sats and Alice's balance changes by more than 100,000 sats.

**Expected:** Bob gains 100,000 sats relative to his prior confirmed balance (unless you changed wallet state separately). Actual fee varies. Alice may also have been automatically funded first.

## Exercise 4 — Decode the payment
- Run `cargo run --bin inspect -- YOUR_TXID`.
- Record TXID, WTXID, input count, output count, vsize and confirmations.
- Explain why decoding a transaction does not verify all Bitcoin consensus rules.
- Explain what happens when the supplied TXID is malformed or not found in Alice's wallet.

**Expected:** decoded TXID exactly matches supplied TXID; wallet-reported confirmations are positive after mining.

## Exercise 5 — Unspent transaction outputs
- Run `cargo run --bin utxos`.
- Record the number of UTXOs, confirmed UTXO sum, and a `safe` flag for each wallet.
- Explain what a change output is, using Alice's transaction.
- Explain why `listunspent` is not the global blockchain UTXO set.

**Expected:** confirmed UTXOs; sum calculated using checked integer arithmetic.

## Technical review and Git exercise
- Run `cargo fmt --all`, `cargo check --all-targets`, `cargo test`.
- Create a branch, change one test or improve error wording, and inspect `git diff`.
- Write a short technical review: What changed? What does the code actually prove? What does it not prove?

**Submit:** README describing environment, outputs from each exercise, TXID and fee (Regtest only), and answers to the five sets of questions.
