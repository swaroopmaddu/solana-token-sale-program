# Solana Token Sale Program

Swap SPL tokens for SOL at a fixed price

## Development

### Environment Setup

1. Install the latest Rust stable from https://rustup.rs/
2. Install Solana v1.6.1 or later from https://docs.solana.com/cli/install-solana-cli-tools
3. Install the `libudev` development package for your distribution (`libudev-dev` on Debian-derived distros, `libudev-devel` on Redhat-derived).

### Build

The normal cargo build is available for building programs against your host machine:
```
$ cargo build
```

To build a specific program, such as SPL Token, for the Solana BPF target:
```
$ cd token/program
$ cargo build-bpf
```

### Deploy

```
$ solana program deploy target/deploy/spl_tokensale_solana.so
```

### Set Environment Variables
Before running RPC Client, Change .example-env to .env. And refer to the below and fill NEEDED section.

```
CUSTOM_PROGRAM_ID=NEEDED
SELLER_PUBLIC_KEY=NEEDED
SELLER_PRIVATE_KEY=NEEDED
BUYER_PUBLIC_KEY=NEEDED
BUYER_PRIVATE_KEY=NEEDED
TOKEN_PUBKEY=NEEDED
SELLER_TOKEN_ACCOUNT_PUBKEY=Dont'mind this
TEMP_TOKEN_ACCOUNT_PUBKEY=Dont'mind this
TOKEN_SALE_PROGRAM_ACCOUNT_PUBKEY=Dont'mind this
```

### Test

```bash
$ cd client
$ npm install
$ npm run all
```

# Disclaimer

Use this contract at your own risk. This program was not audited.

Reference https://github.com/myungjunChae/solana_token_sale
