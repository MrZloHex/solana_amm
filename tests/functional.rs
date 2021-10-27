#![cfg(feature = "test-bpf")]

use solana_amm::{TOKEN_A_SEED, TOKEN_B_SEED, entrypoint::process_instruction, id};
use solana_program_test::{processor, tokio, ProgramTest};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};

#[tokio::test]
async fn test_transfer_x() {
    let mut program_test = ProgramTest::new("solana_amm", id(), processor!(process_instruction));

    let xtok_acc = Pubkey::create_with_seed(&id(), TOKEN_A_SEED, &id()).unwrap();
    let ytok_acc = Pubkey::create_with_seed(&id(), TOKEN_B_SEED, &id()).unwrap();
    let xtok_user = Pubkey::new_unique();
    let ytok_user = Pubkey::new_unique();

    program_test.add_account(
        xtok_acc,
        Account {
            lamports: 19,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        ytok_acc,
        Account {
            lamports: 19,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        xtok_user,
        Account {
            lamports: 350,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        ytok_user,
        Account {
            lamports: 18,
            owner: id().clone(),
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            id(),
            &[0, 0, 86, 1, 0, 0, 0, 0, 0, 0],
            vec![
                AccountMeta::new(xtok_acc, false),
                AccountMeta::new(ytok_acc, false),
                AccountMeta::new(xtok_user, false),
                AccountMeta::new(ytok_user, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let x_acc = banks_client.get_account(xtok_acc).await.unwrap().unwrap();
    let x_lam = x_acc.lamports;
    assert_eq!(x_lam, 361);

    let y_acc = banks_client.get_account(ytok_acc).await.unwrap().unwrap();
    let y_lam = y_acc.lamports;
    assert_eq!(y_lam, 1);

    let x_user = banks_client.get_account(xtok_user).await.unwrap().unwrap();
    let x_lam_u = x_user.lamports;
    assert_eq!(x_lam_u, 8);

    let y_user = banks_client.get_account(ytok_user).await.unwrap().unwrap();
    let y_lam_u = y_user.lamports;
    assert_eq!(y_lam_u, 36);
}

#[tokio::test]
async fn test_transfer_y() {
    let mut program_test = ProgramTest::new("solana_amm", id(), processor!(process_instruction));
    // BUG: IF YOU REMOVE THIS TESTS WILL CRAHES
    let _ = Pubkey::new_unique();

    let xtok_acc = Pubkey::create_with_seed(&id(), TOKEN_A_SEED, &id()).unwrap();
    let ytok_acc = Pubkey::create_with_seed(&id(), TOKEN_B_SEED, &id()).unwrap();
    let xtok_user = Pubkey::new_unique();
    let ytok_user = Pubkey::new_unique();

    program_test.add_account(
        xtok_acc,
        Account {
            lamports: 20,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        ytok_acc,
        Account {
            lamports: 20,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        xtok_user,
        Account {
            lamports: 15,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        ytok_user,
        Account {
            lamports: 120,
            owner: id().clone(),
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            id(),
            &[0, 1, 60, 0, 0, 0, 0, 0, 0, 0],
            vec![
                AccountMeta::new(xtok_acc, false),
                AccountMeta::new(ytok_acc, false),
                AccountMeta::new(xtok_user, false),
                AccountMeta::new(ytok_user, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let x_acc = banks_client.get_account(xtok_acc).await.unwrap().unwrap();
    let x_lam = x_acc.lamports;
    assert_eq!(x_lam, 5);

    let y_acc = banks_client.get_account(ytok_acc).await.unwrap().unwrap();
    let y_lam = y_acc.lamports;
    assert_eq!(y_lam, 80);

    let x_user = banks_client.get_account(xtok_user).await.unwrap().unwrap();
    let x_lam_u = x_user.lamports;
    assert_eq!(x_lam_u, 30);

    let y_user = banks_client.get_account(ytok_user).await.unwrap().unwrap();
    let y_lam_u = y_user.lamports;
    assert_eq!(y_lam_u, 60);
}

#[tokio::test]
async fn test_transfer_float() {
    let mut program_test = ProgramTest::new("solana_amm", id(), processor!(process_instruction));
    // BUG: IF YOU REMOVE THIS TESTS WILL CRAHES
    let _ = Pubkey::new_unique();

    let xtok_acc = Pubkey::create_with_seed(&id(), TOKEN_A_SEED, &id()).unwrap();
    let ytok_acc = Pubkey::create_with_seed(&id(), TOKEN_B_SEED, &id()).unwrap();
    let xtok_user = Pubkey::new_unique();
    let ytok_user = Pubkey::new_unique();

    program_test.add_account(
        xtok_acc,
        Account {
            lamports: 25,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        ytok_acc,
        Account {
            lamports: 20,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        xtok_user,
        Account {
            lamports: 80,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        ytok_user,
        Account {
            lamports: 10,
            owner: id().clone(),
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            id(),
            &[0, 0, 76, 0, 0, 0, 0, 0, 0, 0],
            vec![
                AccountMeta::new(xtok_acc, false),
                AccountMeta::new(ytok_acc, false),
                AccountMeta::new(xtok_user, false),
                AccountMeta::new(ytok_user, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let x_acc = banks_client.get_account(xtok_acc).await.unwrap().unwrap();
    let x_lam = x_acc.lamports;
    assert_eq!(x_lam, 101);

    let y_acc = banks_client.get_account(ytok_acc).await.unwrap().unwrap();
    let y_lam = y_acc.lamports;
    assert_eq!(y_lam, 4);

    let x_user = banks_client.get_account(xtok_user).await.unwrap().unwrap();
    let x_lam_u = x_user.lamports;
    assert_eq!(x_lam_u, 4);

    let y_user = banks_client.get_account(ytok_user).await.unwrap().unwrap();
    let y_lam_u = y_user.lamports;
    assert_eq!(y_lam_u, 26);
}
