use solana_program::{
    program_option::COption,
    pubkey::Pubkey,
    program_error::ProgramError
};

use arrayref::{array_refs,mut_array_refs};


pub(crate) fn unpack_coption_key(src: &[u8; 36]) -> Result<COption<Pubkey>,ProgramError> {
    let (tag,body) = array_refs![src,4,32];
    return match *tag{
        [0,0,0,0] => Ok(COption::None),
        [1,0,0,0] => Ok(COption::Some(Pubkey::new_from_array(*body))),
        _ => Err(ProgramError::InvalidAccountData)
    };
}

pub(crate) fn pack_coption_key(src: &COption<Pubkey>,dst: &mut [u8; 36]){
    let (tag,body) = mut_array_refs![dst,4,32];
    match src{
        COption::None =>{
            *tag = [0; 4];            
        },
        COption::Some(key) =>{
            *tag = [1,0,0,0];
            body.copy_from_slice(key.as_ref());
        }
    }
}