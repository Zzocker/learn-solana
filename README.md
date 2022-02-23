# Learn Solana

Learning solana by building programs and their corresponding clis.

## Prerequisite

1. Install Node: [nvm](https://github.com/nvm-sh/nvm#installing-and-updating)
2. Install Rust: https://rustup.rs/
3. Install Solana: https://docs.solana.com/cli/install-solana-cli-tools
4. Setup Local wallet:`solana-keygen new`
5. Connect to `testnet`: `solana config set --url testnet`
6. Request for airdrop of 2 Sol: `solana airdrop 2`

## Hello World

Simplest solana program in which program write message `Hello World` upon invoking the program.

- Compile Program : `npm run build::hello-world`
- Deploy : `npm run deploy::hello-world`
- Run the example : `npm run hello-world`