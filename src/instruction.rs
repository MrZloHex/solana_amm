use borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum TokenType {
    A,
    B,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct TransferAMM {
    token_type: TokenType,
    quantity: u64,
}

impl TransferAMM {
    pub fn get_token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_quantity(&self) -> u64 {
        self.quantity
    }
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct CreateSettlementAccounts {
    quantity_tokens_a: u64,
    quantity_tokens_b: u64
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub enum InstructionType {
    /// Accounts:
    /// 0. `[]` program's account with tokens type A, PDA
    /// 1. `[]` program's account with tokens type B, PDA
    /// 2. `[]` user's account with tokens type A
    /// 3. `[]` user's account with tokens type B
    TransferAMM(TransferAMM),

    /// Accounts:
    /// 0. `[signer, writable]` payer's account, who will provide tokens for program's accounts
    CreateSettlementAccounts(CreateSettlementAccounts)
}

#[cfg(test)]
mod test {
    use borsh::{BorshDeserialize, BorshSerialize};

    use crate::instruction::{
        TokenType,
        TransferAMM,
        InstructionType,
        CreateSettlementAccounts
    };

    #[test]
    fn test_transfer_serialization() {
        let data = InstructionType::TransferAMM( TransferAMM {
            token_type: TokenType::A,
            quantity: 35_000,
        })
        .try_to_vec()
        .unwrap();
        assert_eq!(data, [0, 0, 184, 136, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_transfer_deserialization() {
        let data = InstructionType::TransferAMM( TransferAMM {
            token_type: TokenType::B,
            quantity: 35_000,
        });
        let instr = InstructionType::try_from_slice(&[0, 1, 184, 136, 0, 0, 0, 0, 0, 0]).unwrap();
        assert_eq!(data, instr);
    }

    #[test]
    fn test_create_acc_serialization() {
        let data = InstructionType::CreateSettlementAccounts( CreateSettlementAccounts {
            quantity_tokens_a: 1234,
            quantity_tokens_b: 4567
        })
        .try_to_vec()
        .unwrap();
        assert_eq!(data, [1, 210, 4, 0, 0, 0, 0, 0, 0, 215, 17, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_create_acc_deserialization() {
        let data = InstructionType::CreateSettlementAccounts( CreateSettlementAccounts {
            quantity_tokens_a: 7890,
            quantity_tokens_b: 3465
        });
        let instr = InstructionType::try_from_slice(&[1, 210, 30, 0, 0, 0, 0, 0, 0, 137, 13, 0, 0, 0, 0, 0, 0]).unwrap();
        assert_eq!(data, instr);
    }
}
