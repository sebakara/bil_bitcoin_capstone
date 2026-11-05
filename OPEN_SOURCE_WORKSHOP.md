# Technical Open-Source Workshop (Final Session)

## 1. Observe the project's review requirements
Review:
https://github.com/bitcoin/bitcoin/blob/master/CONTRIBUTING.md
https://github.com/bitcoin/bips

Avoid submitting speculative Bitcoin Core patches. Start with testing, docs,
small reproducible bug reports, and reviewing existing pull requests.

## 2. Practice local code review with THIS Rust capstone
```sh
git init
git add .
git commit -m "Add regtest capstone labs"
cargo fmt --all -- --check
cargo check --all-targets
cargo test
git diff --check
```
Then modify `src/lib.rs`: replace checked subtraction in `fee_sats()` with
ordinary subtraction and run `cargo test` with inputs=10/outputs=11; discuss
why checked arithmetic matters. Restore original code before committing.

## 3. Trace a genuine Bitcoin technical proposal
BIP 141: segregated witness — transaction weight/virtual size.
BIP 174: PSBT — coordinating signatures without sharing private keys.
BIP 173: Bech32 — address encoding and error detection.
Read the Motivation, Specification, and Backward Compatibility sections;
be explicit whether you are discussing deployed behavior or an exploratory draft.

## 4. BitDevs Kigali prototype format
- 10 minutes: one recent Bitcoin Core or rust-bitcoin pull request
- 20 minutes: code path, tests, threat model
- 15 minutes: BIP / protocol design trade-offs
- 15 minutes: local demo, reproduce bug or run regression test
- 10 minutes: open questions & next review assignments
These are proposed agenda timings, not an announcement of an existing event.
