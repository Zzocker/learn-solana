use solana_program::{
    entrypoint,
    entrypoint::{ProgramResult},
    pubkey::{Pubkey},
    account_info::{AccountInfo},
    msg,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id : &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult{
    msg!("Hello World");
    return Ok(());
}