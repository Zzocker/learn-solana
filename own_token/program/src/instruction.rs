use crate::{error::TokenError};
use solana_program::{
    msg, pubkey::{Pubkey, PUBKEY_BYTES}, program_error::ProgramError,
};


pub enum TokenInstruction{
    InitializeMint{
        mint_authority: Pubkey,
        decimals: u8,
        freeze_authority: Option<Pubkey>  
    },
}

impl TokenInstruction{
    pub fn unpack(input: &[u8]) -> Result<TokenInstruction,TokenError>{
        msg!("TokenInstruction.unpack");
        let (method,rest) = input.split_first().ok_or(TokenError::InvalidInstructionData)?;
        msg!("method = {}",*method);
        return match *method{
            0 => {
                let (mint_authority,rest) = Self::unpack_pubkey(rest)?;
                let (decimals,rest) = rest.split_first().ok_or(TokenError::InvalidInstructionData)?;
                let (freeze_authority,_)  = Self::unpack_option_pubkey(rest)?;
                Ok(TokenInstruction::InitializeMint{
                    mint_authority: mint_authority,
                    decimals: *decimals,
                    freeze_authority: freeze_authority,
                })
            },
            _ => Err(TokenError::InvalidMethod) 
        };
    }

    fn unpack_pubkey(src: &[u8]) -> Result<(Pubkey,&[u8]),TokenError>{
        let pk = src.get(..PUBKEY_BYTES)
        .map(Pubkey::new)
        .ok_or(TokenError::InvalidInstructionData)?;

        return Ok((pk,&src[PUBKEY_BYTES..]));
    }

    fn unpack_option_pubkey(src: &[u8]) -> Result<(Option<Pubkey>,&[u8]),TokenError>{
        let (option,rest) = src.split_first().ok_or(TokenError::InvalidInstructionData)?;
        return match *option {
            0 => Ok((None,&src[PUBKEY_BYTES..])),
            1 => {
                let (pk,rest) = Self::unpack_pubkey(rest)?;
                Ok((Some(pk),rest))
            },
            _ => Err(TokenError::InvalidInstructionData) 
        };
    }
}