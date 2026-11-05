# BIL — Building Bitcoin Applications with Rust

**Student project for combined Sessions 5 & 6** · Blockchain & Innovation Landscape

This is a **real, executable Rust Cargo project** designed to teach Bitcoin engineering on a **private Bitcoin Core Regtest network**. No cryptocurrency purchase or real money is required.

## 1. What you will build

| Program | Purpose | Changes Regtest state? |
|---|---|---|
| `units` | Exact BTC/sat conversion and a hypothetical fee example | No |
| `setup` | Create/load miner, Alice and Bob wallets; mature miner rewards | **Yes:** may mine 101 blocks |
| `payment` | Preview transfer, balances and unknown fee | **No by default** |
| `payment -- --send` | Fund Alice if necessary, pay Bob, confirm transaction, print **actual** fee | **Yes:** broadcasts payment and mines block(s) |
| `inspect -- TXID` | Decode Alice's transaction and verify the decoded TXID equals supplied ID | No |
| `utxos` | List wallet-confirmed UTXOs and total their satoshi amounts | No |

**Important:** Transaction decoding checks data representation and TXID equality, **not** signatures or full Bitcoin consensus validity. Wallet RPC confirms the state observed by the local Regtest node.

## 2. Prerequisites

- Rust toolchain: `rustc --version` and `cargo --version`. Install from https://rustup.rs/
- Bitcoin Core command-line tools: `bitcoind --version` and `bitcoin-cli --version`. Install from https://bitcoincore.org/en/download/
- Internet on first build for Cargo dependencies (`bitcoin`, `serde`, `serde_json`).

### macOS / Linux

```bash
# Run only if the node is not already running.
bitcoind -regtest -daemon -fallbackfee=0.00001
bitcoin-cli -regtest getblockchaininfo
```

### Windows

Open a separate terminal and run `bitcoind -regtest -fallbackfee=0.00001` (leave it open); in another terminal run `bitcoin-cli -regtest getblockchaininfo`. Executable names must be on PATH. Run all subsequent `cargo` commands inside the project directory.

**Verify the response says `"chain": "regtest"` before running any wallet operations.**

## 3. First-time setup

In a terminal, navigate into the folder that contains `Cargo.toml`, then:

```bash
cargo fmt --all
cargo check --all-targets
cargo test
cargo run --bin units
cargo run --bin setup
```

Expected deterministic `units` output:

```text
25,000 sats = 0.00025000 BTC
Sample fee = 800 sats
```

That **800-sat example is simple subtraction**, not a fee paid to the Bitcoin network.

`setup` creates/loads three wallet names: `class_miner`, `class_alice`, `class_bob`. On a new Regtest node it mines 101 blocks so the miner has a mature coinbase reward. The node stores wallets/blocks between runs; repeat output may differ.

## 4. Make a payment safely (preview first)

```bash
cargo run --bin payment
# No payment or mining occurs. Look at the preview.

cargo run --bin payment -- --send
# Real REGTEST payment of 100,000 sats to Bob; actual fee printed.
```

- `--preview` is an explicit synonym for default preview. The wallets must already be loaded (run `setup` first).
- With `--send`, Alice receives a 0.01 BTC funding transfer if her balance is too low, then sends Bob 0.001 BTC. Bitcoin Core selects inputs, change, and fees. The script mines a confirmation block, and queries Bitcoin Core for the **actual fee paid**.
- Repeating `--send` **sends another payment**. Nothing is reset between runs.
- Record the printed TXID, actual fee (sats), and before/after confirmed balances. No fixed TXID or network fee is expected.

## 5. Inspect the transaction and UTXOs

Use the **actual TXID** printed by the last payment:

```bash
cargo run --bin inspect -- PUT_YOUR_TXID_HERE
cargo run --bin utxos
```

`inspect` fetches Alice's wallet transaction, parses the hex with `rust-bitcoin`, confirms calculated TXID equals the supplied ID, and shows TXID/WTXID, number of inputs/outputs, vsize, output amounts, and wallet-reported confirmations. It does **not** calculate the fee from output sums, and does **not** independently verify scripts or consensus.

`utxos` asks Bitcoin Core for each wallet's confirmed unspent outputs, prints up to 10 entries per wallet, and **sums all entries** in integer satoshis using checked addition. The `safe` flag reflects Bitcoin Core wallet metadata rather than a guarantee of immediate spendability under every condition.

## 6. Code structure and responsibilities

```text
bil_bitcoin_capstone/
├── Cargo.toml
├── README.md
├── STUDENT_EXERCISES.md
├── INSTRUCTOR_NOTES.md
└── src/
    ├── lib.rs             # Bitcoin Core RPC, wallets, exact amounts, fees, tests
    └── bin/
        ├── units.rs       # Deterministic calculations
        ├── setup.rs       # Wallet creation and miner rewards
        ├── payment.rs     # Safe preview and explicit transfer
        ├── inspect.rs     # Transaction decoding + TXID equality
        └── utxos.rs       # Confirmed unspent outputs and totals
```

See `FUNCTION_EXPLANATIONS.md` for a line-by-line walkthrough of the most important functions. `INSTRUCTOR_NOTES.md` suggests a 120-minute teaching route.

## 7. Common errors

| Message / symptom | Fix |
|---|---|
| `bitcoin-cli not found` | Install Bitcoin Core and add its binaries to PATH. |
| `Could not connect to server` | Start `bitcoind -regtest` and check `getblockchaininfo`. |
| Wallet not found | Run `cargo run --bin setup` first. |
| Insufficient funds | Run setup first; ensure the miner is funded. The send path funds Alice when needed. |
| Inspector says transaction not found | Use a TXID generated by **Alice's wallet**, not a random mainnet TXID. |
| Different balances or fees than classmates | Regtest state, wallet UTXOs and fee rates vary; this is expected. |
| Cargo cannot fetch crates | Internet access is required once to download dependencies. |

## 8. Safety and limits

- **Regtest only**. The RPC helper hard-codes `-regtest`.
- Do not share private keys, RPC cookies, or real wallet files.
- `payment` default is a safe preview; only `--send` broadcasts a Regtest transfer.
- `units` arithmetic is fully local; `setup`, `payment`, `inspect`, and `utxos` require a running node.
- Neither parsing a transaction nor reading wallet RPC responses is an independent consensus validation proof.
- This classroom project is not a production-grade wallet, and should not be used with mainnet.

## Official references

- https://bitcoincore.org/en/doc/
- https://github.com/rust-bitcoin/rust-bitcoin
- https://doc.rust-lang.org/book/
- https://github.com/bitcoin/bips
