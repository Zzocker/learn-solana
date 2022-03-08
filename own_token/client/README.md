# Own Token

Replica of solana-token

## InitializeMint

Life-cycle of any token start with mint. First a mint is initialized with `mint_authority`, `decimals`, `freeze_authority`. `mint_authority` is only allowed to mint new token. `decimals` is number of base 10 digits to be right of decimals place. `freeze_authority` is optional pubkey has power `freeze` token account.

### Request Format

- [0] = 0 (InitializeMint)
- [1..33] = `mint_authority`
- [33] = `decimals`
- [34] = `freeze_authority_option` (0|1)
- [35..67] = `freeze_authority`


## Program Request Format

> `instruction_data[0]` (u8) is method identifier  

## States

### Mint

- Fields
  - is_initialized (bool) : check if account data is initialized or not
  - supply (u64) : total token created
  - decimals (u8) : number of base 10 digits to be right of decimals place.
  - mint_authority: Option(PubKey) : PubKey of mint authority, if None then no more token can be minted
  - freeze_authority : Option(PubKey) : PubKey of freeze_authority
- Data Representation
  - [0] : is_initialized
  - [1..9] : supply
  - [9] : decimals
  - [10] : mint_authority_option
  - [11..43] : mint_authority
  - [43] : freeze_authority_option
  - [44..76] : freeze_authority