#![cfg(feature = "test-bpf")]

use borsh::{BorshDeserialize, BorshSerialize};
use solana_amm::{X_TOK_SEED, Y_TOK_SEED, entrypoint::process_instruction, id};
use solana_program_test::{processor, tokio, ProgramTest};
use solana_program::{msg, system_instruction};
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    transaction::Transaction,
    account::Account
};



#[tokio::test]
async fn test_start() {
    let mut program_test = ProgramTest::new(
        "solana_amm",
        id(),
        processor!(process_instruction),
    );

    program_test.add_account(
        Pubkey::create_with_seed(&id(), X_TOK_SEED, &id()).unwrap(),
        Account {
            lamports: 69_000,
            owner: id().clone(),
            ..Account::default()
        }
    );
    program_test.add_account(
        Pubkey::create_with_seed(&id(), Y_TOK_SEED, &id()).unwrap(),
        Account {
            lamports: 35_000,
            owner: id().clone(),
            ..Account::default()
        }
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;



    assert_eq!(1, 1);
}
