use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::{X_TOK_SEED, Y_TOK_SEED, id};


pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        _input: &[u8]
    ) -> ProgramResult {
        msg!("Starting");
        let acc_iter = &mut accounts.iter();
        let xtok_info_p = next_account_info(acc_iter)?;
        let ytok_info_p = next_account_info(acc_iter)?;

        if !check_pda_acc(xtok_info_p.key, X_TOK_SEED) {
            return Err(ProgramError::InvalidSeeds);
        }
        if !check_pda_acc(ytok_info_p.key, Y_TOK_SEED) {
            return Err(ProgramError::InvalidSeeds);
        }
        Ok(())
    }
}

pub fn check_pda_acc(acc: &Pubkey, seed: &str) -> bool {
    *acc == Pubkey::create_with_seed(&id(), seed, &id()).unwrap()
}
