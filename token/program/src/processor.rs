
use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    account_info::{AccountInfo, next_account_info},
    msg, program_option::COption,
    sysvar::{rent::{Rent}, Sysvar}, program_pack::Pack
};

use crate::{
    instruction::TokenInstruction, error::TokenError,
    state::{Mint}
};
pub struct Processor;

impl Processor{
    pub fn process(
        program_id : &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8]
    ) -> ProgramResult{
        let instruction = TokenInstruction::unpack(input)?;
        match instruction {
            TokenInstruction::InitializeMint{
                decimals,
                mint_authority,
                freeze_authority
            } => {
                msg!("Instruction: InitializeMint");
                return Self::process_initialize_mint(accounts, decimals, mint_authority, freeze_authority);
            }
        }
    }
    fn _process_initialize_mint(
        accounts: &[AccountInfo],
        decimals: u8,
        mint_authority: Pubkey,
        freeze_authority: COption<Pubkey>,
        rent_sysvar_account: bool
    )-> ProgramResult{
        let account_info_iter = &mut accounts.iter();
        let mint_info = next_account_info(account_info_iter)?;
        let mint_data_len = mint_info.data_len();

        let rent = if rent_sysvar_account {
            Rent::from_account_info(next_account_info(account_info_iter)?)?
        } else {
            Rent::get()?
        };

        if !rent.is_exempt(mint_info.lamports(), mint_data_len) {
            return Err(TokenError::NotRentExempt.into());
        }

        let mut mint_data = mint_info.data.borrow_mut();

        let mut mint = Mint::unpack_unchecked(&mut mint_data)?;
        if mint.is_initialized{
            return Err(TokenError::AlreadyInUse.into());
        }

        mint.mint_authority = COption::Some(mint_authority);
        mint.supply = 0;
        mint.decimals = decimals;
        mint.is_initialized = true;
        mint.freeze_authority = freeze_authority;
        Mint::pack(mint, &mut mint_data)?;
        return Ok(());
    }
    pub fn process_initialize_mint(
        accounts: &[AccountInfo],
        decimals: u8,
        mint_authority: Pubkey,
        freeze_authority: COption<Pubkey>
    ) -> ProgramResult{
        return Self::_process_initialize_mint(accounts, decimals, mint_authority, freeze_authority, true);
    }
}