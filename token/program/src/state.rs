use solana_program::{
    program_option::COption,
    pubkey::Pubkey,
    program_pack::{IsInitialized,Sealed,Pack},
    program_error::ProgramError,
};

use arrayref::{array_ref,array_refs,array_mut_ref,mut_array_refs};

use crate::{
    error::TokenError,
    utils::{unpack_coption_key, pack_coption_key},
};

// Mint Data
#[repr(C)]
#[derive(Debug,Clone, Copy,Default,PartialEq)]
pub struct Mint{
    /// Optional authority used to mint new tokens. The mint authority may only be provided during
    /// mint creation. If no mint authority is present then the mint has a fixed supply and no
    /// further token can be minted
    pub mint_authority: COption<Pubkey>,
    /// Total supply of tokens.
    pub supply: u64,
    /// Number of base 10 digits to the right of the decimals place.
    pub decimals: u8,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
    /// Optional authority to freeze token accounts.
    pub freeze_authority: COption<Pubkey>
}

impl IsInitialized for Mint{
    fn is_initialized(&self) -> bool{
        return self.is_initialized;
    }
}

impl Sealed for Mint {}

impl Pack for Mint{
    const LEN: usize = 82; // 4+32+8+1+1+4+32
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst,0,82];
        let (
            mint_authority_dst,
            supply_dst,
            decimals_dst,
            is_initialized_dst,
            freeze_authority_dst
        ) = mut_array_refs![dst,36,8,1,1,36];
        let &Mint{
            mint_authority,
            supply,
            decimals,
            is_initialized,
            freeze_authority
        } = self;
        pack_coption_key(&mint_authority, mint_authority_dst);
        *supply_dst = supply.to_le_bytes();
        decimals_dst[0] = decimals;
        is_initialized_dst[0] = is_initialized as u8;
        pack_coption_key(&freeze_authority, freeze_authority_dst);
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError>{
        let src = array_ref![src,0,82];
        let (mint_authority,supply,decimals,is_initialized,freeze_authority) = array_refs![src,36,8,1,1,36];
        let mint_authority = unpack_coption_key(mint_authority)?;
        let supply = u64::from_le_bytes(*supply);
        let decimals = decimals[0];
        let is_initialized = match is_initialized{
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        let freeze_authority = unpack_coption_key(freeze_authority)?;

        return Ok(Mint{
            mint_authority: mint_authority,
            supply: supply,
            decimals: decimals,
            is_initialized: is_initialized,
            freeze_authority: freeze_authority,
        });
    }
}