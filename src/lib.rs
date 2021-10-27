pub mod errors;
pub mod instruction;
pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub const X_TOK_SEED: &str = "xtoken";
pub const Y_TOK_SEED: &str = "ytoken";

solana_program::declare_id!("4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM");
