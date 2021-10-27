use borsh::{self, BorshSerialize, BorshDeserialize};
use solana_program::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey};

use crate::{X_TOK_SEED, Y_TOK_SEED, id};



#[derive(BorshSerialize, BorshDeserialize, Clone, Copy)]
pub enum TokenType {
    X,
    Y
}


#[derive(BorshSerialize, BorshDeserialize)]
pub struct TransferAMM {
    // Token Type:
    // - 1: X TOKEN
    // - 2: Y TOKEN
    token_type: TokenType,
    quantity: u64
}


impl TransferAMM {
    // pub fn tranfer(token_type: u8, quantity: u64) -> Result<Instruction, u8> {
    //     if token_type != 1 || token_type != 2 {
    //         return Err(1);
    //     }
    //     let trans_amm = TransferAMM {
    //         token_type,
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
    use borsh::BorshSerialize;

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
}


