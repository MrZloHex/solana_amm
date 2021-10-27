use borsh::BorshDeserialize;
use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::{X_TOK_SEED, Y_TOK_SEED, id};
use crate::errors::PDAError;
use crate::instruction::{TokenType, TransferAMM};

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8]
    ) -> ProgramResult {
        msg!("Starting");
        let instruction = TransferAMM::try_from_slice(input)?;

        let acc_iter = &mut accounts.iter();
        let xtok_info_p = next_account_info(acc_iter)?;
        let ytok_info_p = next_account_info(acc_iter)?;
        // let xtok_info_u = next_account_info(acc_iter)?;
        // let ytok_info_u = next_account_info(acc_iter)?;

        if !check_pda_acc(xtok_info_p.key, X_TOK_SEED) {
            return Err(PDAError::WrongPDA.into());
        }
        if !check_pda_acc(ytok_info_p.key, Y_TOK_SEED) {
            return Err(PDAError::WrongPDA.into());
        }
        // if !xtok_info_u.is_signer {
        //     return Err(ProgramError::MissingRequiredSignature)
        // }
        // if !ytok_info_u.is_signer {
        //     return Err(ProgramError::MissingRequiredSignature)
        // }

        let k = xtok_info_p.lamports() * ytok_info_p.lamports();
        // let dx = instruction.get_quantity();
        // let dy = ytok_info_p.lamports() - (k / (xtok_info_p.lamports() + dx));
        
        
        match instruction.get_token_type() {
            TokenType::X => {
                **xtok_info_p.try_borrow_mut_lamports()? -= 35000;
                **ytok_info_p.try_borrow_mut_lamports()? += 35000;
            },
            TokenType::Y => {
                **xtok_info_p.try_borrow_mut_lamports()? -= 34000;
                **ytok_info_p.try_borrow_mut_lamports()? += 34000;
            },
        }

        Ok(())
    }
}

pub fn check_pda_acc(acc: &Pubkey, seed: &str) -> bool {
    *acc == Pubkey::create_with_seed(&id(), seed, &id()).unwrap()
}
