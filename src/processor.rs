use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::errors::PDAError;
use crate::instruction::{TokenType, TransferAMM};
use crate::{id, TOKEN_A_SEED, TOKEN_B_SEED};

pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let transfer_config = TransferAMM::try_from_slice(instruction_data)?;

    match transfer_config.get_token_type() {
        TokenType::A => handle_income_tok_a(accounts, &transfer_config),
        TokenType::B => handle_income_tok_b(accounts, &transfer_config),
    }
}

pub fn handle_income_tok_a(accounts: &[AccountInfo], transfer_config: &TransferAMM) -> ProgramResult {
    let acc_iter = &mut accounts.iter();
    let prog_acc_tok_a = next_account_info(acc_iter)?;
    let prog_acc_tok_b = next_account_info(acc_iter)?;
    let user_acc_tok_a = next_account_info(acc_iter)?;
    let user_acc_tok_b = next_account_info(acc_iter)?;

    if !check_pda_acc(prog_acc_tok_a.key, TOKEN_A_SEED) {
        return Err(PDAError::WrongPDA.into());
    }
    if !check_pda_acc(prog_acc_tok_b.key, TOKEN_B_SEED) {
        return Err(PDAError::WrongPDA.into());
    }

    let x: u64 = prog_acc_tok_a.lamports();
    let y: u64 = prog_acc_tok_b.lamports();
    let k: u64 = x * y;
    let dx: u64 = transfer_config.get_quantity();
    let dy: u64 = y - (k / (x + dx));

    **prog_acc_tok_a.try_borrow_mut_lamports()? += dx;
    **prog_acc_tok_b.try_borrow_mut_lamports()? -= dy;
    **user_acc_tok_a.try_borrow_mut_lamports()? -= dx;
    **user_acc_tok_b.try_borrow_mut_lamports()? += dy;
    Ok(())
}

pub fn handle_income_tok_b(accounts: &[AccountInfo], transfer_config: &TransferAMM) -> ProgramResult {
    let acc_iter = &mut accounts.iter();
    let prog_acc_tok_a = next_account_info(acc_iter)?;
    let prog_acc_tok_b = next_account_info(acc_iter)?;
    let user_acc_tok_a = next_account_info(acc_iter)?;
    let user_acc_tok_b = next_account_info(acc_iter)?;

    if !check_pda_acc(prog_acc_tok_a.key, TOKEN_A_SEED) {
        return Err(PDAError::WrongPDA.into());
    }
    if !check_pda_acc(prog_acc_tok_b.key, TOKEN_B_SEED) {
        return Err(PDAError::WrongPDA.into());
    }

    let x: u64 = prog_acc_tok_a.lamports();
    let y: u64 = prog_acc_tok_b.lamports();
    let k: u64 = x * y;
    let dy: u64 = transfer_config.get_quantity();
    let dx: u64 = x - (k / (y + dy));

    **prog_acc_tok_a.try_borrow_mut_lamports()? -= dx;
    **prog_acc_tok_b.try_borrow_mut_lamports()? += dy;
    **user_acc_tok_a.try_borrow_mut_lamports()? += dx;
    **user_acc_tok_b.try_borrow_mut_lamports()? -= dy;
    Ok(())
}

pub fn check_pda_acc(acc: &Pubkey, seed: &str) -> bool {
    *acc == Pubkey::create_with_seed(&id(), seed, &id()).unwrap()
}
