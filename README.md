# Solana BPF Boilerplate

<p align="center">
  <a href="https://solana.com">
    <img alt="Solana" src="https://i.imgur.com/uBVzyX3.png" width="250" />
  </a>
</p>
Simple template for building smart contract(Rust) and RPC Client(web3.js) on Solana<p><p>

This boilerplate provides the following.

- Simple smart contract that can be published on Solana Online.
  - Store data through byte buffer on account what you create.
- RPC Client interacts with published smart contracts
  - Send transaction about creating an account for store data of the published program, and Initializing published program.

## Prerequisite

- Install node (v14 recommended)
- Install npm
- Install the latest Rust stable from https://rustup.rs/
- Install Solana v1.7.11 or later from https://docs.solana.com/cli/install-solana-cli-tools

## Quick Start

- Build Rust
- Setup Solana Config and Test-Validator(Single node cluster)
- Set Environment
- Run RPC Client

### Build Rust

Build Rust and get `.so` file. You should conduct following command on `contract` folder

```
cargo build-bpf
```

Run test-validator

```
solana-test-validator -r --bpf-program 5yznMbghv1Z2Gb4uaKkY9fxb9PkLMrSn9xgjswucHq4X ${.so}
```

> If you want to get more information about validator. <p>
> Use `solana-test-validator -h`

### Setup Solana

Set CLI config url to localhost cluster

```
solana config set --url localhost
```

Create CLI Keypair
If this is your first time using the Solana CLI, you will need to generate a new keypair:

```
solana-keygen new
```

### Set Environment

Before running RPC Client, Change `.example-env` to `.env`.<p>
And refer to the below and fill `USER_PUBLICE_KEY` & `USE_PRIVATE_KEY`

```
CUSTOM_PROGRAM_ID=5yznMbghv1Z2Gb4uaKkY9fxb9PkLMrSn9xgjswucHq4X
USER_PUBLIC_KEY=6Nr8iPDAAkR54aEfhuc5LD86paVfeFkvETwj3pdf73o5
USER_PRIVATE_KEY=[180,232,26,240,241,108,253,206,155,230,59,44,39,19,191,11,17,180,126,71,236,92,212,4,110,29,2,0,180,168,52,132,79,227,47,188,174,13,151,40,50,237,0,221,198,50,72,164,14,48,23,225,87,115,28,241,156,41,239,176,116,23,157,72]
```

### Run RPC Client

Install the necessary dependencies for running RPC Client. You should conduct following command on `client` folder

```
npm install
```

Run RPC Client for interaction with on-chain program what you published.

```
npm run transaction
```

## Further

### SmartContract

- Pack & Unpack with Barsh

### RPC Client

- Setup.ts for create wallet account
