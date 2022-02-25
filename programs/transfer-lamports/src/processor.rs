use solana_program::{
    pubkey::Pubkey,
    account_info::{AccountInfo, next_account_info},
    entrypoint::{ProgramResult},
    msg,
    program_error::{ProgramError}
};

const U64_BYTES: usize = 8;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult{

    let amount =instruction_data.get(..U64_BYTES).
    and_then(|slice|slice.try_into().ok())
    .map(u64::from_le_bytes)
    .ok_or(ProgramError::InvalidInstructionData)?;

    msg!("amount to transfer: {} lamports",amount);

    let account_iter = &mut accounts.iter();

    let source = next_account_info(account_iter)?;
    let destination = next_account_info(account_iter)?;

    **source.try_borrow_mut_lamports()?-=amount;
    **destination.try_borrow_mut_lamports()?+=amount;

    return Ok(()); 
}