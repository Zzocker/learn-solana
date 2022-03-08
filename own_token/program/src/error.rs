use thiserror::Error;
use solana_program::{
    program_error::{ProgramError,PrintProgramError}, decode_error::DecodeError,
    msg
};

use num_derive::{FromPrimitive};
use num_traits;

#[derive(Clone, Copy,Debug,PartialEq,Error,FromPrimitive)]
pub enum TokenError{
    /// invalid method called by the client
    #[error("invalid method called")]
    InvalidMethod = 0,
    #[error("An instruction's data contents was invalid")]
    InvalidInstructionData,

    #[error("An account's data contents was invalid")]
    InvalidAccountData,

    #[error("insufficient account data size")]
    InsufficientAccountDataSize,

    #[error("rent not exempted")]
    RentNotExempted,

    #[error("account already in use")]
    AlreadyInUse
}

impl From<TokenError> for ProgramError{
    fn from(e: TokenError) -> Self{
        return ProgramError::Custom(e as u32);
    }
}

impl<T> DecodeError<T> for TokenError{
    fn type_of() -> &'static str{
        return "TokenError";
    }
}

impl PrintProgramError for TokenError{
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + num_traits::FromPrimitive{
            match &self {
                TokenError::InvalidMethod => msg!("Error: invalid method called"),
                TokenError::InvalidInstructionData => msg!("Error: an instruction's data contents was invalid"),
                TokenError::InvalidAccountData => msg!("Error: an account's data contents was invalid"),
                TokenError::InsufficientAccountDataSize => msg!("insufficient account data size"),
                TokenError::RentNotExempted => msg!("rent not exempted"),
                TokenError::AlreadyInUse => msg!("account already in use"),
            };
        }
}