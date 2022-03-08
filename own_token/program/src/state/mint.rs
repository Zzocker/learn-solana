use solana_program::{
    pubkey::Pubkey,
    program_pack::{Pack,Sealed,IsInitialized},
    program_error::ProgramError
};

use arrayref::{array_ref, array_refs,array_mut_ref,mut_array_refs};

use crate::error::TokenError;
use super::utils::{pack_option_pubkey,unpack_option_pubkey};
pub struct Mint{
    pub is_initialized: bool,
    pub supply: u64,
    pub decimals: u8,
    pub mint_authority : Option<Pubkey>,
    pub freeze_authority: Option<Pubkey>,
}
impl Sealed for Mint{}
impl IsInitialized for Mint{
    fn is_initialized(&self) -> bool{
        return self.is_initialized;
    }
}

impl Pack for Mint{
    const LEN: usize = 76;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst,0,76];
        let (
            is_initialized_dst ,
            supply_dst ,
            decimals_dst ,
            mint_authority_dst ,
            freeze_authority_dst  
        ) = mut_array_refs![dst,1,8,1,33,33];
        is_initialized_dst[0] = self.is_initialized as u8;
        *supply_dst = self.supply.to_le_bytes();
        decimals_dst[0] = self.decimals;
        pack_option_pubkey(&self.mint_authority, mint_authority_dst);
        pack_option_pubkey(&self.freeze_authority, freeze_authority_dst);
    }
    
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError>{
        let src = array_ref![src,0,76];
        let (
            is_initialized,
            supply,
            decimals,
            mint_authority,
            freeze_authority
        ) = array_refs![src,1,8,1,33,33];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(TokenError::InvalidAccountData.into())
        };
        let supply = u64::from_le_bytes(*supply);
        let decimals = decimals[0] as u8;
        let mint_authority = unpack_option_pubkey(mint_authority)?;
        let freeze_authority = unpack_option_pubkey(freeze_authority)?;

        return Ok(Mint{
            is_initialized : is_initialized,
            supply : supply,
            decimals : decimals,
            mint_authority : mint_authority,
            freeze_authority : freeze_authority,
        });
    }

}