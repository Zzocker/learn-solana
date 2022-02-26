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

## Transfer Lamports

Point To Note:

- Only a data account's owner can modify its data and debit lamports
- Anyone is allowed to credit lamports to a data account
- Rent
    - Storing data on accounts costs SOL to maintain, and it is funded by what is called rent. If you maintain a minimum balance equivalent to 2 years of rent payments in an account, your account will be exempt from paying rent. You can retrieve rent by closing the account and sending the lamports back to your wallet.

- Compile Program : `npm run build::transfer-lamports`
- Deploy : `npm run deploy::transfer-lamports`
- Run the example : `npm run transfer-lamports`

## Math

Program to store `i64` state in program derived address (pda) and perform method based operation. Supported methods :

- Add(num: i64): perform addition to stored value and update its value.
- Subtract(num: i64) : perform subtraction to stored value and update its value.
- Multiply(name: i64) : perform multiplication to stored value and update its value.

References:
- [Seed for generating PDA](https://stackoverflow.com/questions/68878330/what-is-the-seeds-in-creating-account-or-finding-the-account-in-solana-and-coul)
