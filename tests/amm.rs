#![cfg(feature = "test-bpf")]

use borsh::{BorshDeserialize, BorshSerialize};
use solana_amm::{entrypoint::process_instruction, id, instruction::AmmInstruction};
use solana_program_test::{processor, tokio, ProgramTest, ProgramTestContext};
use solana_program::system_instruction;
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    transaction::Transaction
};


struct Env {
    ctx: ProgramTestContext,
    user: Keypair
}

impl Env {
    async fn new() -> Self {
        let program_test = ProgramTest::new("solana_amm", id(), processor!(process_instruction));
        let mut ctx = program_test.start_with_context().await;

        let user = Keypair::new();

        ctx.banks_client.process_transaction(Transaction::new_signed_with_payer(
                &[
                    system_instruction::transfer(
                        &ctx.payer.pubkey(),
                        &user.pubkey(),
                        100_000_000,
                    )
                ],
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer],
                ctx.last_blockhash,
            )).await.unwrap();

        Env {
            ctx,
            user
        }
    }
}


#[tokio::test]
async fn test_start() {
    let mut env = Env::new().await;

    assert_eq!(1, 1);
}
