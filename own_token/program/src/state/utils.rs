use solana_program::{
    pubkey::{Pubkey}
};

use arrayref::{array_refs,mut_array_refs};

use crate::{
    error::TokenError
};

pub fn unpack_option_pubkey(src: &[u8; 33]) -> Result<Option<Pubkey>,TokenError>{
    let (option,key) = array_refs![src,1,32];
    return match option[0] {
        0 => Ok(None),
        1 => Ok(Some(Pubkey::new_from_array(*key))),
        _ => Err(TokenError::InvalidAccountData)
    };
}

pub fn pack_option_pubkey(src: &Option<Pubkey>,dst: &mut [u8; 33]){
    let (option,body) = mut_array_refs![dst,1,32];
    match src {
        None =>{
            *option = [0];
        },
        Some(key) =>{
            *option = [1];
            body.copy_from_slice(key.as_ref());
        }
    }
}