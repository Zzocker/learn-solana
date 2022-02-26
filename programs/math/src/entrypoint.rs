use std::thread::AccessError;

use solana_program::{
    entrypoint,
    entrypoint::{ProgramResult},
    pubkey::{Pubkey},
    account_info::{AccountInfo},
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult{
    return Ok(());
}