use borsh::BorshDeserialize;
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey, system_instruction};

use crate::{errors::PDAError, instruction};
use crate::instruction::{TokenType, TransferAMM, InstructionType, CreateSettlementAccounts};
use crate::{id, TOKEN_A_SEED, TOKEN_B_SEED};

pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let instruction = InstructionType::try_from_slice(instruction_data)?;

    match instruction {
        InstructionType::TransferAMM(transfer_config) => {
            match transfer_config.get_token_type() {
                TokenType::A => handle_income_tok_a(accounts, &transfer_config),
                TokenType::B => handle_income_tok_b(accounts, &transfer_config),
            }
        },
        InstructionType::CreateSettlementAccounts(set_accs_config) => 
            create_settlement_accounts(accounts, set_accs_config),
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

pub fn create_settlement_accounts(accounts: &[AccountInfo], settle_acc_config: CreateSettlementAccounts) -> ProgramResult {
    let accs_iter = &mut accounts.iter();
    let payer_acc = next_account_info(accs_iter)?;
    let token_a_acc = next_account_info(accs_iter)?;
    let system_info = next_account_info(accs_iter)?;


    // let pubkey_a = Pubkey::create_with_seed(&id(), TOKEN_A_SEED, &id())?;
    let (pubkey_a, bump_seed) = Pubkey::find_program_address(&[TOKEN_A_SEED.as_bytes()], &id());
    let signer_seeds: &[&[_]] = &[TOKEN_A_SEED.as_bytes(), &[bump_seed]];
    invoke_signed(
        &system_instruction::create_account(
            payer_acc.key,
            &pubkey_a,
            settle_acc_config.get_tokens_a(),
            0,
            &id()
        ),
        &[payer_acc.clone(), token_a_acc.clone(), system_info.clone()],
        &[&signer_seeds]
    )?;

    // let pubkey_b = Pubkey::create_with_seed(&id(), TOKEN_B_SEED, &id())?;
    // let signer_seeds: &[&[_]] = &[TOKEN_B_SEED.as_bytes()];
    // invoke_signed(
    //     &system_instruction::create_account_with_seed(
    //         payer_acc.key,
    //         &pubkey_b,
    //         &id(),
    //         TOKEN_B_SEED,
    //         settle_acc_config.get_tokens_b(),
    //         0,
    //         &id()
    //     ),
    //     &[payer_acc.clone()],
    //     &[&signer_seeds]
    // )?;

    Ok(())
}
