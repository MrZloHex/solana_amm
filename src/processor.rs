use borsh::BorshDeserialize;
use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::{X_TOK_SEED, Y_TOK_SEED, id};
use crate::errors::PDAError;
use crate::instruction::{TokenType, TransferAMM};


pub fn process(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8]
) -> ProgramResult {
    msg!("Starting");
    let instruction = TransferAMM::try_from_slice(input)?;
    
    match instruction.get_token_type() {
        TokenType::X => process_x(accounts, &instruction),
        TokenType::Y => process_y(accounts, &instruction)
    }
}

pub fn process_x(
    accounts: &[AccountInfo],
    instruction: &TransferAMM
) -> ProgramResult {
    let acc_iter = &mut accounts.iter();
    let xtok_info_p = next_account_info(acc_iter)?;
    let ytok_info_p = next_account_info(acc_iter)?;
    let xtok_info_u = next_account_info(acc_iter)?;
    let ytok_info_u = next_account_info(acc_iter)?;

    if !check_pda_acc(xtok_info_p.key, X_TOK_SEED) {
        return Err(PDAError::WrongPDA.into());
    }
    if !check_pda_acc(ytok_info_p.key, Y_TOK_SEED) {
        return Err(PDAError::WrongPDA.into());
    }

    let x:  u64 = xtok_info_p.lamports();
    let y:  u64 = ytok_info_p.lamports();
    let k:  u64 = x * y;
    let dx: u64 = instruction.get_quantity();
    let dy: u64 = y - (k / (x + dx));
    
    **xtok_info_p.try_borrow_mut_lamports()? += dx;
    **ytok_info_p.try_borrow_mut_lamports()? -= dy;
    **xtok_info_u.try_borrow_mut_lamports()? -= dx;
    **ytok_info_u.try_borrow_mut_lamports()? += dy;
    Ok(())
}


pub fn process_y(
    accounts: &[AccountInfo],
    instruction: &TransferAMM
) -> ProgramResult {
    let acc_iter = &mut accounts.iter();
    let xtok_info_p = next_account_info(acc_iter)?;
    let ytok_info_p = next_account_info(acc_iter)?;
    let xtok_info_u = next_account_info(acc_iter)?;
    let ytok_info_u = next_account_info(acc_iter)?;

    if !check_pda_acc(xtok_info_p.key, X_TOK_SEED) {
        return Err(PDAError::WrongPDA.into());
    }
    if !check_pda_acc(ytok_info_p.key, Y_TOK_SEED) {
        return Err(PDAError::WrongPDA.into());
    }

    let x:  u64 = xtok_info_p.lamports();
    let y:  u64 = ytok_info_p.lamports();
    let k:  u64 = x * y;
    let dy: u64 = instruction.get_quantity();
    let dx: u64 = x - (k / (y + dy));
    
    **xtok_info_p.try_borrow_mut_lamports()? -= dx;
    **ytok_info_p.try_borrow_mut_lamports()? += dy;
    **xtok_info_u.try_borrow_mut_lamports()? += dx;
    **ytok_info_u.try_borrow_mut_lamports()? -= dy;
    Ok(())
}

pub fn check_pda_acc(acc: &Pubkey, seed: &str) -> bool {
    *acc == Pubkey::create_with_seed(&id(), seed, &id()).unwrap()
}
