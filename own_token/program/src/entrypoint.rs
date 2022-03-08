use solana_program::{
    entrypoint,
    pubkey::Pubkey,
    account_info::AccountInfo,
    msg, program_error::PrintProgramError,
};
use crate::{
    processor::Processor, error::TokenError
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> entrypoint::ProgramResult{
    msg!("entrypoint.process_instruction");
    if let Err(error) = Processor::process(program_id, accounts, instruction_data){
        error.print::<TokenError>();
        return Err(error);
    }
    return  Ok(());
}