use solana_program::{
    entrypoint,
    pubkey::Pubkey,
    account_info::AccountInfo,
    msg,
};
use crate::processor::Processor;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> entrypoint::ProgramResult {
    msg!("token program");
    if let Err(error) = Processor::process(program_id, accounts, instruction_data){
        return Err(error);
    }
    return Ok(());
}