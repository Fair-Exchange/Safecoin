---
title: "Local Development Quickstart"
description: "This quickstart guide will demonstrate how to quickly install and setup your local Safecoin development environment."
keywords:
  - rust
  - cargo
  - toml
  - program
  - tutorial
  - intro to safecoin development
  - blockchain developer
  - blockchain tutorial
  - web3 developer
---

This quickstart guide will demonstrate how to quickly install and setup your local development environment, getting you ready to start developing and deploying Safecoin programs to the blockchain.

## What you will learn

- How to install the Safecoin CLI locally
- How to setup a localhost Safecoin cluster/validator
- How to create a Safecoin wallet for developing
- How to airdrop SAFE tokens for your wallet

## Install the Safecoin CLI

To interact with the Safecoin clusters from your terminal, install the [Safecoin CLI tool suite](./../cli/install-safecoin-cli-tools) on your local system:

```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

## Setup a localhost blockchain cluster

The Safecoin CLI comes with the [test validator](./../developing/test-validator.md) built in. This command line tool will allow you to run a full blockchain cluster on your machine.

```bash
safecoin-test-validator
```

> **PRO TIP:**
> Run the Safecoin test validator in a new/separate terminal window that will remain open. The command line program must remain running for your localhost cluster to remain online and ready for action.

Configure your Safecoin CLI to use your localhost validator for all your future terminal commands:

```bash
safecoin config set --url localhost
```

At any time, you can view your current Safecoin CLI configuration settings:

```bash
safecoin config get
```

## Create a file system wallet

To deploy a program with Safecoin CLI, you will need a Safecoin wallet with SAFE tokens to pay for the cost of transactions.

Let's create a simple file system wallet for testing:

```bash
safecoin-keygen new
```

By default, the `safecoin-keygen` command will create a new file system wallet located at `~/.config/solana/id.json`. You can manually specify the output file location using the `--outfile /path` option.

> **NOTE:**
> If you already have a file system wallet saved at the default location, this command will **NOT** override it (unless you explicitly force override using the `--force` flag).

#### Set your new wallet as default

With your new file system wallet created, you must tell the Safecoin CLI to use this wallet to deploy and take ownership of your on chain program:

```bash
safecoin config set -k ~/.config/solana/id.json
```

## Airdrop SAFE tokens to your wallet

Once your new wallet is set as the default, you can request a free airdrop of SAFE tokens to it:

```bash
safecoin airdrop 2
```

> **NOTE:**
> The `safecoin airdrop` command has a limit of how many SAFE tokens can be requested _per airdrop_ for each cluster (localhost, testnet, or devent). If your airdrop transaction fails, lower your airdrop request quantity and try again.

You can check your current wallet's SAFE balance any time:

```bash
safecoin balance
```

## Next steps

See the links below to learn more about writing Rust based Safecoin programs:

- [Create and deploy a Safecoin Rust program](./rust.md)
- [Overview of writing Safecoin programs](../developing/on-chain-programs/overview)
