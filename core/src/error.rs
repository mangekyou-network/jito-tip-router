use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MEVTipDistributionNCNError {
    #[error("No more table slots available")]
    NoMoreTableSlots = 0x2000,
    #[error("Zero in the denominator")]
    DenominatorIsZero = 0x2100,
    #[error("Overflow")]
    ArithmeticOverflow = 0x2101,
    #[error("Modulo Overflow")]
    ModuloOverflow = 0x2102,

    #[error("Incorrect weight table admin")]
    IncorrectWeightTableAdmin = 0x2200,
    #[error("Cannnot create future weight tables")]
    CannotCreateFutureWeightTables = 0x2201,
}

impl<T> DecodeError<T> for MEVTipDistributionNCNError {
    fn type_of() -> &'static str {
        "jito::weight_table"
    }
}

impl From<MEVTipDistributionNCNError> for ProgramError {
    fn from(e: MEVTipDistributionNCNError) -> Self {
        Self::Custom(e as u32)
    }
}

impl From<MEVTipDistributionNCNError> for u64 {
    fn from(e: MEVTipDistributionNCNError) -> Self {
        e as Self
    }
}

impl From<MEVTipDistributionNCNError> for u32 {
    fn from(e: MEVTipDistributionNCNError) -> Self {
        e as Self
    }
}
