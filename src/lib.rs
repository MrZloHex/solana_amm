pub mod errors;
pub mod instruction;
pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub const TOKEN_A_SEED: &str = "token_a";
pub const TOKEN_B_SEED: &str = "token_b";

solana_program::declare_id!("4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM");
