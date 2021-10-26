#![cfg(feature = "test-bpf")]

use borsh::{BorshDeserialize, BorshSerialize};
use solana_amm::{entrypoint::process_instruction, instruction::AmmInstruction};
use solana_program_test::{processor, tokio, ProgramTest, ProgramTestContext};
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
};


struct Env {
    ctx: ProgramTestContext,
    user: Keypair
}

impl Env {
    async fn new() -> Self {
        let id = Pubkey::new_unique();
        let program_test = ProgramTest::new("solana-amm", id, processor!(process_instruction));
        let mut ctx = program_test.start_with_context().await;

        let user = Keypair::new();

        Env {
            ctx,
            user
        }
    }
}
