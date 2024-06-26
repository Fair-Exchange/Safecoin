---
title: "Hello World Quickstart Guide"
description: 'This "hello world" quickstart guide will demonstrate how to setup, build, and deploy your first Safecoin program in your browser with Safecoin Playground.'
keywords:
  - playground
  - safecoin pg
  - on chain
  - rust
  - native program
  - tutorial
  - intro to safecoin development
  - blockchain developer
  - blockchain tutorial
  - web3 developer
---

For this "hello world" quickstart guide, we will use [Safecoin Playground](https://beta.solpg.io), a browser the based IDE, to develop and deploy our Safecoin program. To use it, you do **NOT** have to install any software on your computer. Simply open Safecoin Playground in your browser of choice, and you are ready to write and deploy Safecoin programs.

## What you will learn

- How to get started with Safecoin Playground
- How to create a Safecoin wallet on Playground
- How to program a basic Safecoin program in Rust
- How to build and deploy a Safecoin Rust program
- How to interact with your on chain program using JavaScript

## Using Safecoin Playground

[Safecoin Playground](https://beta.solpg.io) is browser based application that will let you write, build, and deploy on chain Safecoin programs. All from your browser. No installation needed.

It is a great developer resource for getting started with Safecoin development, especially on Windows.

### Import our example project

In a new tab in your browser, open our example "_Hello World_" project on Safecoin Playground: https://beta.solpg.io/6314a69688a7fca897ad7d1d

Next, import the project into your local workspace by clicking the "**Import**" icon and naming your project `hello_world`.

![Import the get started Safecoin program on Safecoin Playground](/img/quickstarts/solana-get-started-import-on-playground.png)

> If you do **not** import the program into **your** Safecoin Playground, then you will **not** be able to make changes to the code. But you **will** still be able to build and deploy the code to a Safecoin cluster.

### Create a Playground wallet

Normally with [local development](./local.md), you will need to create a file system wallet for use with the Safecoin CLI. But with the Safecoin Playground, you only need to click a few buttons to create a browser based wallet.

> **Note:**
> Your _Playground Wallet_ will be saved in your browser's local storage. Clearing your browser cache will remove your saved wallet. When creating a new wallet, you will have the option to save a local copy of your wallet's keypair file.

Click on the red status indicator button at the bottom left of the screen, (optionally) save your wallet's keypair file to your computer for backup, then click "**Continue**".

After your Playground Wallet is created, you will notice the bottom of the window now states your wallet's address, your SAFE balance, and the Safecoin cluster you are connected to (Devnet is usually the default/recommended, but a "localhost" [test validator](./local.md) is also acceptable).

## Create a Safecoin program

The code for your Rust based Safecoin program will live in your `src/lib.rs` file. Inside `src/lib.rs` you will be able to import your Rust crates and define your logic. Open your `src/lib.rs` file within Safecoin Playground.

### Import the `solana_program` crate

At the top of `lib.rs`, we import the `safecoin-program` crate and bring our needed items into the local namespace:

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};
```

### Write your program logic

Every Safecoin program must define an `entrypoint` that tells the Safecoin runtime where to start executing your on chain code. Your program's [entrypoint](../developing/on-chain-programs/developing-rust#program-entrypoint) should provide a public function named `process_instruction`:

```rust
// declare and export the program's entrypoint
entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // log a message to the blockchain
    msg!("Hello, world!");

    // gracefully exit the program
    Ok(())
}
```

Every on chain program should return the `Ok` [result enum](https://doc.rust-lang.org/std/result/) with a value of `()`. This tells the Safecoin runtime that your program executed successfully without errors.

Our program above will simply [log a message](../developing/on-chain-programs/debugging#logging) of "_Hello, world!_" to the blockchain cluster, then gracefully exit with `Ok(())`.

### Build your program

On the left sidebar, select the "**Build & Deploy**" tab. Next, click the "Build" button.

If you look at the Playground's terminal, you should see your Safecoin program begin to compile. Once complete, you will see a success message.

![Viewing a successful build of your Rust based program](/img/quickstarts/solana-get-started-successful-build.png)

> Note:
> You may receive _warning_ when your program is compiled due to unused variables. Don't worry, these warning will not affect your build. They are due to our very simple program not using all the variables we declared in the `process_instruction` function.

### Deploy your program

You can click the "Deploy" button to deploy your first program to the Safecoin blockchain. Specifically to your selected cluster (e.g. Devnet, Testnet, etc).

After each deployment, you will see your Playground Wallet balance change. By default, Safecoin Playground will automatically request SAFE airdrops on your behalf to ensure your wallet has enough SAFE to cover the cost of deployment.

> Note:
> If you need more SAFE, you can airdrop more by typing airdrop command in the playground terminal:

```sh
safecoin airdrop 2
```

![Build and deploy your Safecoin program to the blockchain](/img/quickstarts/solana-get-started-build-and-deploy.png)

### Find your program id

When executing a program using [web3.js](../developing/clients/javascript-reference.md) or from [another Safecoin program](../developing/programming-model/calling-between-programs.md), you will need to provide the `program id` (aka public address of your program).

Inside Safecoin Playground's **Build & Deploy** sidebar, you can find your `program id` under the **Program Credentials** dropdown.

#### Congratulations!

You have successfully setup, built, and deployed a Safecoin program using the Rust language directly in your browser. Next, we will demonstrate how to interact with your on chain program.

## Interact with your on chain program

Once you have successfully deployed a Safecoin program to the blockchain, you will want to be able to interact with that program.

Like most developers creating dApps and websites, we will interact with our on chain program using JavaScript. Specifically, will use the open source [NPM package](https://www.npmjs.com/package/@safecoin/web3.js) `@safecoin/web3.js` to aid in our client application.

> **NOTE:**
> This web3.js package is an abstraction layer on top of the [JSON RPC API](./../developing/clients/jsonrpc-api.md) that reduced the need for rewriting common boilerplate, helping to simplify your client side application code.

### Initialize client

We will be using Safecoin Playground for the client generation. Create a client folder by running `run` command in the playground terminal:

```bash
run
```

We have created `client` folder and a default `client.ts`. This is where we will work for the rest of our `hello world` program.

### Playground globals

In playground, there are many utilities that are globally available for us to use without installing or setting up anything. Most important ones for our `hello world` program are `web3` for `@safecoin/web3.js` and `pg` for Safecoin Playground utilities.

> Note:
> You can go over all of the available globals by pressing `CTRL+SPACE` (or `CMD+SPACE` on macOS) inside the editor.

### Call the program

To execute your on chain program, you must send a [transaction](../developing/programming-model/transactions.md) to it. Each transaction submitted to the Safecoin blockchain contains a listing of instructions (and the program's that instruction will interact with).

Here we create a new transaction and add a single `instruction` to it:

```js
// create an empty transaction
const transaction = new web3.Transaction();

// add a hello world program instruction to the transaction
transaction.add(
  new web3.TransactionInstruction({
    keys: [],
    programId: new web3.PublicKey(pg.PROGRAM_ID),
  }),
);
```

Each `instruction` must include all the keys involved in the operation and the program ID we want to execute. In this example `keys` is empty because our program only logs `hello world` and doesn't need any accounts.

With our transaction created, we can submit it to the cluster:

```js
// send the transaction to the Safecoin cluster
console.log("Sending transaction...");
const txHash = await web3.sendAndConfirmTransaction(
  pg.connection,
  transaction,
  [pg.wallet.keypair],
);
console.log("Transaction sent with hash:", txHash);
```

> Note:
> The first signer in the signers array is the transaction fee payer by default. We are signing with our keypair `pg.wallet.keypair`.

### Run the application

With the client application written, you can run the code via the same `run` command.

Once your application completes, you will see output similar to this:

```sh
Running client...
  client.ts:
    My address: GkxZRRNPfaUfL9XdYVfKF3rWjMcj5md6b6mpRoWpURwP
    My balance: 5.7254472 SAFE
    Sending transaction...
    Transaction sent with hash: 2Ra7D9JoqeNsax9HmNq6MB4qWtKPGcLwoqQ27mPYsPFh3h8wignvKB2mWZVvdzCyTnp7CEZhfg2cEpbavib9mCcq
```

### Get transaction logs

We will be using `safecoin-cli` directly in playground to get the information about any transaction:

```sh
safecoin confirm -v <TRANSACTION_HASH>
```

Change `<TRANSACTION_HASH>` with the hash you received from calling `hello world` program.

You should see `Hello, world!` in the **Log Messages** section of the output. 🎉

#### Congratulations!!!

You have now written a client application for your on chain program. You are now a Safecoin developer!

PS: Try to update your program's message then re-build, re-deploy, and re-execute your program.

## Next steps

See the links below to learn more about writing Safecoin programs:

- [Setup your local development environment](./local.md)
- [Overview of writing Safecoin programs](../developing/on-chain-programs/overview)
- [Learn more about developing Safecoin programs with Rust](../developing/on-chain-programs/developing-Rust)
- [Debugging on chain programs](../developing/on-chain-programs/debugging)
