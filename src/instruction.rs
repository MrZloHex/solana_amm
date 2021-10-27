use borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum TokenType {
    X,
    Y,
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

#[cfg(test)]
mod test {
    use borsh::{BorshDeserialize, BorshSerialize};

    use crate::instruction::{TokenType, TransferAMM};

    #[test]
    fn test_serialization() {
        let data = TransferAMM {
            token_type: TokenType::X,
            quantity: 35_000,
        }
        .try_to_vec()
        .unwrap();
        assert_eq!(data, [0, 184, 136, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_deserialization() {
        let data = TransferAMM {
            token_type: TokenType::X,
            quantity: 35_000,
        };
        let instr = TransferAMM::try_from_slice(&[0, 184, 136, 0, 0, 0, 0, 0, 0]).unwrap();
        assert_eq!(data, instr);
    }
}
