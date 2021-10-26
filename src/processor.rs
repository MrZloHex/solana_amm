use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg
};


pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _input: &[u8]
    ) -> ProgramResult {
        msg!("Starting");
        Ok(())
    }
}
