use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::FarmFundError::InvalidInstruction;

pub enum FarmFundInstruction {
    InitCampaign {
        amount: u64,
    },
    FundCampaign {
        amount: u64,
    },
}

impl FarmFundInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitCampaign {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::FundCampaign {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
