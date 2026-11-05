# How the Rust functions work — instructor/student explanations

## `rpc(wallet: Option<&str>, args: &[&str]) -> AppResult<String>`

- **Intention:** call `bitcoin-cli` on Regtest and collect the response.
- `Command::new("bitcoin-cli")` prepares a process, `-regtest` selects the local development chain.
- `Option<&str>` lets the caller specify `Some("class_alice")` for wallet-scoped commands or `None` for node-level commands.
- `args: &[&str]` borrows a list of CLI arguments, without invoking a shell.
- `.output()` waits for the process; a non-zero status returns Bitcoin Core's error; a successful stdout is converted into trimmed UTF-8 text.
- **Does not do:** implement a Bitcoin node or authenticate a remote wallet by itself.

## `ensure_wallet(name: &str) -> AppResult<()>`

- **Intention:** reuse an existing wallet or create it if missing.
- Queries `listwallets` and `listwalletdir`; calls `loadwallet` or `createwallet` appropriately.
- `()` means no return payload on success.
- **Does not do:** erase or reset existing balances.

## `sats_to_btc(sats: u64) -> String`

- **Intention:** format exact integer satoshis into human-readable BTC text.
- Uses `sats / 100_000_000` for whole BTC, and `sats % 100_000_000` for eight decimal places.
- **Expected:** 25,000 sats gives `0.00025000` BTC.
- **Does not do:** query wallet balances or send a transaction.

## `parse_btc_to_sats(text: &str) -> Result<u64, String>`

- **Intention:** parse BTC decimals without floating-point rounding.
- Splits the text into whole/fraction, rejects invalid characters or >8 decimal places, and uses checked arithmetic.
- **Expected:** `0.00100000` gives 100,000 sats.
- **Does not do:** query fees or balances.

## `fee_sats(inputs: u64, outputs: u64) -> Result<u64, &str>`

- **Intention:** teach the arithmetic identity `sum(inputs) - sum(outputs)`.
- Uses `checked_sub` to reject `outputs > inputs`.
- **Expected:** 50,000−49,200 = 800 sats.
- **Does not do:** retrieve any actual Bitcoin Core transaction fee; it needs known previous-output values.

## `balance_sats(wallet: &str) -> AppResult<u64>`

- **Intention:** request confirmed wallet balance (minimum one confirmation).
- Runs `getbalance "*" 1`, parses the BTC decimal string into exact sats.
- **Does not do:** enumerate transaction outputs or establish a globally verified balance without trusting local node state.

## `address`, `mine`, `send_btc`

- `address` asks a wallet for a fresh receiving address (`getnewaddress`).
- `mine` generates requested Regtest blocks to an address (`generatetoaddress`).
- `send_btc` asks Bitcoin Core's wallet to select coins, sign, and broadcast (`sendtoaddress`). It rejects a zero amount; returns **TXID**, not fee.
- The actual fee is determined by Bitcoin Core's wallet and is retrieved separately.

## `outgoing_fee_sats_from_json` / `wallet_tx_fee_sats`

- **Intention:** extract the actual fee of an outgoing transaction from wallet `gettransaction` JSON.
- Bitcoin Core normally reports outgoing fee as **negative BTC**. The code removes the leading minus and parses decimal BTC exactly to satoshis.
- **Expected:** a `fee` JSON value `-0.00000140` becomes `140` sats.
- **Does not do:** predict the fee before a transaction is built.

## `payment` (preview vs execution)

- `cargo run --bin payment` prints confirmed balances and the planned payment amount, without calling `send_btc` or `mine`.
- `cargo run --bin payment -- --send` may fund Alice first if her balance is low, sends 100,000 sats to Bob, mines a block, then prints the actual fee and confirmed wallet balances.
- **Critical:** running `--send` more than once results in additional transactions.

## `inspect` binary

- Parses a supplied TXID, fetches wallet's `gettransaction` response, decodes the raw transaction bytes with `rust-bitcoin`, calculates TXID and **compares it** with input. Displays WTXID, vsize, inputs, outputs and wallet confirmations.
- **Does not do:** independently validate Bitcoin signatures or consensus; an equal calculated TXID demonstrates byte consistency with the supplied identifier.

## `utxos` binary

- Calls `listunspent 1` (at least one confirmation) for Alice and Bob.
- Parses BTC JSON amounts as decimal text, adds satoshis using `checked_add`, prints up to ten example outputs and summarizes all confirmed outputs.
- `safe` is a Bitcoin Core wallet assessment; this is not a guarantee about every possible spending condition.
- **Does not do:** scan or validate the global network UTXO set.
