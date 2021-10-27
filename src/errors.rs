use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum PDAError {
    #[error("Wrong counter PDA for this account")]
    WrongPDA,
}

impl From<PDAError> for ProgramError {
    fn from(e: PDAError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
