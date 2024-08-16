# Anchor intro 2

<https://solana.com/developers/courses/onchain-development/anchor-pdas>

## Requirements

- Rust

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

- Solana CLI tools

 `sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"`

- Anchor

`cargo install --git https://github.com/coral-xyz/anchor avm --locked --force`

`avm install latest`

`avm use latest`

## First steps

Run

`anchor build`

This will generate a keypair for the program (target/deploy)

Run this command to update the keys (declare_id!() in lib.rs and Anchor.toml) with the ones generated in the previous step

`anchor keys sync`

## How to build it

`anchor build`

## How to run tests

Make sure to run `yarn install` if you did not do so before runing the tests.

`anchor test`
