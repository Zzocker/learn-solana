use solana_program::{
    entrypoint,
    entrypoint::{ProgramResult},
    pubkey::{Pubkey},
    account_info::{AccountInfo}
};


entrypoint!(process_instruction);

fn process_instruction(
    program_id : &Pubkey,
    accounts: &[AccountInfo],
    instruction_data : &[u8] 
) -> ProgramResult{
    return crate::processor::process_instruction(program_id, accounts, instruction_data);
}