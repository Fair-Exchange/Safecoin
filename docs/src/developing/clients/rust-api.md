---
title: Rust API
---

Safecoin's Rust crates are [published to crates.io][crates.io] and can be found
[on docs.rs with the "solana-" prefix][docs.rs].

[crates.io]: https://crates.io/search?q=solana-
[docs.rs]: https://docs.rs/releases/search?query=solana-

Some important crates:

- [`safecoin-program`] &mdash; Imported by programs running on Safecoin, compiled
  to BPF. This crate contains many fundamental data types and is re-exported from
  [`safecoin-sdk`], which cannot be imported from a Safecoin program.

- [`safecoin-sdk`] &mdash; The basic off-chain SDK, it re-exports
  [`safecoin-program`] and adds more APIs on top of that. Most Safecoin programs
  that do not run on-chain will import this.

- [`safecoin-client`] &mdash; For interacting with a Safecoin node via the
  [JSON RPC API](jsonrpc-api).

- [`safecoin-clap-utils`] &mdash; Routines for setting up a CLI, using [`clap`],
  as used by the main Safecoin CLI.

[`safecoin-program`]: https://docs.rs/safecoin-program
[`safecoin-sdk`]: https://docs.rs/safecoin-sdk
[`safecoin-client`]: https://docs.rs/safecoin-client
[`safecoin-clap-utils`]: https://docs.rs/safecoin-clap-utils
[`clap`]: https://docs.rs/clap
