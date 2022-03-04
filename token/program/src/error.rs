
use {
    solana_program::{program_error::ProgramError, decode_error::DecodeError},
    num_derive::FromPrimitive,
    thiserror::Error

};

/// Errors that may be returned by the Token program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum TokenError{
    // 0
    /// Lamports balance below rent-exempt threshold
    #[error("Lamports balance below rent-exempt threshold")]
    NotRentExempt,
    /// Insufficient funds for the operation requested
    #[error("Insufficient funds")]
    InsufficientFund,

    // 5
    /// The account cannot be initialized because it is already being used
    #[error("already in use")]
    AlreadyInUse,
    // 10
    /// Invalid Instruction
    #[error("Invalid instruction")]
    InvalidInstruction
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
