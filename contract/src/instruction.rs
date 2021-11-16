use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::CustomError::InvalidInstruction;

pub enum TokenSaleInstruction {
    InitTokenSale {
        swap_sol_amount: u64,
        swap_token_amount: u64,
    },
    BuyToken {
        data: u64,
    },
    // add more instruction what you want
}

//function of enum
impl TokenSaleInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        //check instruction type
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        //unpack the rest data for each instruction
        return match tag {
            0 => Ok(Self::InitTokenSale {
                swap_sol_amount: Self::unpack_byte(rest, 0)?,
                swap_token_amount: Self::unpack_byte(rest, 1)?,
            }),
            1 => Ok(Self::BuyToken {
                data: Self::unpack_byte(rest, 0)?,
            }),
            _ => Err(InvalidInstruction.into()),
        };
    }
    fn unpack_byte(input: &[u8], byte_index: usize) -> Result<u64, ProgramError> {
        let start_bit = byte_index * 8;
        let end_bit = start_bit + 8;

        let data = input
            .get(start_bit..end_bit)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;

        return Ok(data);
    }
}
