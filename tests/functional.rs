#![cfg(feature = "test-bpf")]

use solana_amm::{TOKEN_A_SEED, TOKEN_B_SEED, entrypoint::process_instruction, id};
use solana_program_test::{processor, tokio, ProgramTest};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_program
};

#[tokio::test]
async fn test_transfer_x() {
    let mut program_test = ProgramTest::new("solana_amm", id(), processor!(process_instruction));

    let prog_tok_a = Pubkey::create_with_seed(&id(), TOKEN_A_SEED, &id()).unwrap();
    let prog_tok_b = Pubkey::create_with_seed(&id(), TOKEN_B_SEED, &id()).unwrap();
    let user_tok_a = Pubkey::new_unique();
    let user_tok_b = Pubkey::new_unique();

    program_test.add_account(
        prog_tok_a,
        Account {
            lamports: 19,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        prog_tok_b,
        Account {
            lamports: 19,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        user_tok_a,
        Account {
            lamports: 350,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        user_tok_b,
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
                AccountMeta::new(prog_tok_a, false),
                AccountMeta::new(prog_tok_b, false),
                AccountMeta::new(user_tok_a, false),
                AccountMeta::new(user_tok_b, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let acc_tok_a = banks_client.get_account(prog_tok_a).await.unwrap().unwrap();
    let lam_tok_a = acc_tok_a.lamports;
    assert_eq!(lam_tok_a, 361);

    let acc_tok_b = banks_client.get_account(prog_tok_b).await.unwrap().unwrap();
    let lam_tok_b = acc_tok_b.lamports;
    assert_eq!(lam_tok_b, 1);

    let u_acc_tok_a = banks_client.get_account(user_tok_a).await.unwrap().unwrap();
    let lam_tok_a_u = u_acc_tok_a.lamports;
    assert_eq!(lam_tok_a_u, 8);

    let u_acc_tok_b = banks_client.get_account(user_tok_b).await.unwrap().unwrap();
    let lam_tok_b_u = u_acc_tok_b.lamports;
    assert_eq!(lam_tok_b_u, 36);
}

#[tokio::test]
async fn test_transfer_y() {
    let mut program_test = ProgramTest::new("solana_amm", id(), processor!(process_instruction));
    // BUG: IF YOU REMOVE THIS TESTS WILL CRAHES
    let _ = Pubkey::new_unique();

    let prog_tok_a = Pubkey::create_with_seed(&id(), TOKEN_A_SEED, &id()).unwrap();
    let prog_tok_b = Pubkey::create_with_seed(&id(), TOKEN_B_SEED, &id()).unwrap();
    let user_tok_a = Pubkey::new_unique();
    let user_tok_b = Pubkey::new_unique();

    program_test.add_account(
        prog_tok_a,
        Account {
            lamports: 20,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        prog_tok_b,
        Account {
            lamports: 20,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        user_tok_a,
        Account {
            lamports: 15,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        user_tok_b,
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
                AccountMeta::new(prog_tok_a, false),
                AccountMeta::new(prog_tok_b, false),
                AccountMeta::new(user_tok_a, false),
                AccountMeta::new(user_tok_b, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let acc_tok_a = banks_client.get_account(prog_tok_a).await.unwrap().unwrap();
    let lam_tok_a = acc_tok_a.lamports;
    assert_eq!(lam_tok_a, 5);

    let acc_tok_b = banks_client.get_account(prog_tok_b).await.unwrap().unwrap();
    let lam_tok_b = acc_tok_b.lamports;
    assert_eq!(lam_tok_b, 80);

    let u_acc_tok_a = banks_client.get_account(user_tok_a).await.unwrap().unwrap();
    let lam_tok_a_u = u_acc_tok_a.lamports;
    assert_eq!(lam_tok_a_u, 30);

    let u_acc_tok_b = banks_client.get_account(user_tok_b).await.unwrap().unwrap();
    let lam_tok_b_u = u_acc_tok_b.lamports;
    assert_eq!(lam_tok_b_u, 60);
}

#[tokio::test]
async fn test_transfer_float() {
    let mut program_test = ProgramTest::new("solana_amm", id(), processor!(process_instruction));
    // BUG: IF YOU REMOVE THIS TESTS WILL CRAHES
    let _ = Pubkey::new_unique();

    let prog_tok_a = Pubkey::create_with_seed(&id(), TOKEN_A_SEED, &id()).unwrap();
    let prog_tok_b = Pubkey::create_with_seed(&id(), TOKEN_B_SEED, &id()).unwrap();
    let user_tok_a = Pubkey::new_unique();
    let user_tok_b = Pubkey::new_unique();

    program_test.add_account(
        prog_tok_a,
        Account {
            lamports: 25,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        prog_tok_b,
        Account {
            lamports: 20,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        user_tok_a,
        Account {
            lamports: 80,
            owner: id().clone(),
            ..Account::default()
        },
    );
    program_test.add_account(
        user_tok_b,
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
                AccountMeta::new(prog_tok_a, false),
                AccountMeta::new(prog_tok_b, false),
                AccountMeta::new(user_tok_a, false),
                AccountMeta::new(user_tok_b, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let acc_tok_a = banks_client.get_account(prog_tok_a).await.unwrap().unwrap();
    let lam_tok_a = acc_tok_a.lamports;
    assert_eq!(lam_tok_a, 101);

    let acc_tok_b = banks_client.get_account(prog_tok_b).await.unwrap().unwrap();
    let lam_tok_b = acc_tok_b.lamports;
    assert_eq!(lam_tok_b, 4);

    let u_acc_tok_a = banks_client.get_account(user_tok_a).await.unwrap().unwrap();
    let lam_tok_a_u = u_acc_tok_a.lamports;
    assert_eq!(lam_tok_a_u, 4);

    let u_acc_tok_b = banks_client.get_account(user_tok_b).await.unwrap().unwrap();
    let lam_tok_b_u = u_acc_tok_b.lamports;
    assert_eq!(lam_tok_b_u, 26);
}

#[tokio::test]
async fn test_settlement_accounts() {
    let mut program_test = ProgramTest::new("solana_amm", id(), processor!(process_instruction));

    // let _ = Pubkey::new_unique();
    let user_payer = Keypair::new();
    // let user_payer = Pubkey::new_unique();
    let (xtok_acc, bump_seed) = Pubkey::find_program_address(&[TOKEN_A_SEED.as_bytes()], &id());

    program_test.add_account(
        user_payer.pubkey(),
        Account {
            lamports: 15_000,
            owner: id(),
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            id(),
            // 2345
            // 4567
            &[1, 41, 9, 0, 0, 0, 0, 0, 0, 215, 17, 0, 0, 0, 0, 0, 0],
            vec![
                AccountMeta::new(user_payer.pubkey(), true),
                AccountMeta::new(xtok_acc, false),
                AccountMeta::new_readonly(system_program::id(), false)
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &user_payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}