use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::{X_TOK_SEED, Y_TOK_SEED, id};
use crate::errors::PDAError;


pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _input: &[u8]
    ) -> ProgramResult {
        msg!("Starting");
        let acc_iter = &mut accounts.iter();
        let xtok_info_p = next_account_info(acc_iter)?;
        let ytok_info_p = next_account_info(acc_iter)?;

        if !check_pda_acc(xtok_info_p.key, X_TOK_SEED) {
            return Err(PDAError::WrongPDA.into());
        }
        if !check_pda_acc(ytok_info_p.key, Y_TOK_SEED) {
            return Err(PDAError::WrongPDA.into());
        }

        let k = **xtok_info_p.try_borrow_mut_lamports()? * **ytok_info_p.try_borrow_mut_lamports()?;
        
        
        Ok(())
    }
}

pub fn check_pda_acc(acc: &Pubkey, seed: &str) -> bool {
    *acc == Pubkey::create_with_seed(&id(), seed, &id()).unwrap()
}
