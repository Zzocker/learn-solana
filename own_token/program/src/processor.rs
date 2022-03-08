use solana_program::account_info::next_account_info;
use solana_program::program_pack::Pack;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{
    pubkey::Pubkey,
    msg,
    account_info::AccountInfo,
    entrypoint::ProgramResult
};

use crate::error::TokenError;
use crate::instruction::TokenInstruction;
use crate::state::{
    mint::Mint,
};

pub struct Processor{}
impl Processor{

    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8]
    ) -> ProgramResult{
        msg!("Processor.process");
        let method = TokenInstruction::unpack(input)?;
        return match method {
            TokenInstruction::InitializeMint{
                mint_authority,
                decimals,
                freeze_authority
            } =>{
                Processor::initialize_mint(accounts, mint_authority, decimals, freeze_authority)
            }
        };
    }

    fn initialize_mint(
        accounts: &[AccountInfo],
        mint_authority: Pubkey,
        decimals: u8,
        freeze_authority: Option<Pubkey>
    ) -> ProgramResult{
        msg!("Processor.initialize_mint : mint_authority: {}, decimals: {}, freeze_authority : {:?}",mint_authority,decimals,freeze_authority);
        let account_info_iter = &mut accounts.iter();
        let mint_info = next_account_info(account_info_iter)?;
        if mint_info.data_len() < Mint::LEN{
            return Err(TokenError::InsufficientAccountDataSize.into());
        }
        let rent = Rent::get()?;
        if !rent.is_exempt(mint_info.lamports(), mint_info.data_len()){
            return Err(TokenError::RentNotExempted.into());
        }
        let mut mint_data = mint_info.data.borrow_mut();
        let mut mint = Mint::unpack_unchecked(&mint_data)?;

        if mint.is_initialized {
            return Err(TokenError::AlreadyInUse.into());
        }

        mint.is_initialized = true;
        mint.supply = 0;
        mint.decimals = decimals;
        mint.mint_authority = Some(mint_authority);
        mint.freeze_authority = freeze_authority;

        Mint::pack(mint, &mut mint_data)?;
        return Ok(());
    }
}