use borsh::{self, BorshSerialize, BorshDeserialize};

use crate::{Y_TOK_SEED, X_TOK_SEED, id};
use solana_program::pubkey::Pubkey;
use solana_program::instruction::{AccountMeta, Instruction};

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum TokenType {
    X,
    Y
}


#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct TransferAMM {
    token_type: TokenType,
    quantity: u64
}


impl TransferAMM {
    // pub fn tranfer(token_type: u8, quantity: u64) -> Result<Instruction, u8> {
    //     if token_type != 1 || token_type != 2 {
    //         return Err(1);
    //     }
    //     let trans_amm = TransferAMM {
    //         token_type: TokenType::X,
    //         quantity
    //     };

    //     let x_acc = Pubkey::create_with_seed(&id(), X_TOK_SEED, &id()).unwrap();
    //     let y_acc = Pubkey::create_with_seed(&id(), Y_TOK_SEED, &id()).unwrap();

    //     let instr = Instruction::new_with_borsh(
    //         id(),
    //         &trans_amm,
    //         vec![
    //             AccountMeta::new(x_acc, false),
    //             AccountMeta::new(y_acc, false),
    //         ]
    //     );

    //     Ok(instr)
    // }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn get_quantity(&self) -> u64 {
        self.quantity.clone()
    }
}

#[cfg(test)]
mod test {
    use borsh::{BorshSerialize, BorshDeserialize};

    use crate::instruction::{TokenType, TransferAMM};

    #[test]
    fn test_serialization() {
        let data = TransferAMM {
            token_type: TokenType::X,
            quantity: 35_000
        }
        .try_to_vec()
        .unwrap();
        assert_eq!(
            data,
            [0, 184, 136, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_deserialization() {
        let data = TransferAMM {
            token_type: TokenType::X,
            quantity: 35_000
        };
        let instr = TransferAMM::try_from_slice(&[0, 184, 136, 0, 0, 0, 0, 0, 0]).unwrap();
        assert_eq!(
            data,
            instr
        );
    }
}


