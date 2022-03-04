
use solana_program::{
    pubkey::{Pubkey,PUBKEY_BYTES}, 
    program_error::ProgramError,
    program_option::COption,

};
use crate::error::TokenError;

/// Instructions supported by the token program
#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub enum TokenInstruction{

    /// Initialize a new mint and optionally opposites all newly minted
    /// token in an account
    /// 
    /// The `InitializeMint` instruction require no signers and MUST be
    /// included with same Transaction as the system program's
    /// `CreateAccount` instruction that create the account being initialized.
    /// Otherwise another party can acquire ownership of the uninitialized
    /// account.
    /// 
    /// All extensions must be initialized before calling this instruction
    /// 
    /// Accounts expected by this instruction
    /// 
    ///     0. `[writable]` the mint to initialize
    ///     1. `[]` Rent sysvar
    InitializeMint{
        /// Number of base 10 digits to the right of the decimal place.
        decimals: u8,
        /// The authority/multi-signature to mint token
        mint_authority: Pubkey,
        /// The freeze authority/multi-signature of the mint
        freeze_authority: COption<Pubkey>
    }
}

impl TokenInstruction{
    pub fn unpack(input: &[u8]) -> Result<Self,ProgramError>{
        use TokenError::InvalidInstruction;
        let (tag,rest) = input.split_first().ok_or(InvalidInstruction)?;
        return match *tag{
            0 => {
                let (decimals,rest) = rest.split_first().ok_or(InvalidInstruction)?;
                let (mint_authority,rest) = Self::unpack_pubkey(rest)?;
                let (freeze_authority,_rest) = Self::unpack_pubkey_option(rest)?;
                Ok(Self::InitializeMint{
                    decimals: *decimals,
                    mint_authority: mint_authority,
                    freeze_authority: freeze_authority
                })
            },
            _ => Err(InvalidInstruction.into())
        };
    }

    pub(crate) fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey,&[u8]),ProgramError>{
        let pk = input.get(..PUBKEY_BYTES)
        .map(Pubkey::new)
        .ok_or(TokenError::InvalidInstruction)?;
        return Ok((pk,&input[PUBKEY_BYTES..]));
    }

    pub(crate) fn unpack_pubkey_option(input: &[u8]) -> Result<(COption<Pubkey>,&[u8]),ProgramError>{
        return match input.split_first(){
            Some((&0,rest)) => Ok((COption::None,rest)),
            Some((&1,rest)) => {
                let (pk,rest) = Self::unpack_pubkey(rest)?;
                Ok((COption::Some(pk),rest))
            }
            _ => Err(TokenError::InvalidInstruction.into()) 
        };
    }
}