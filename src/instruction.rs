use borsh::{BorshSerialize, BorshDeserialize};


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum AmmInstruction {
    Transfer
}


