use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TipRouterError {
    #[error("No more table slots available")]
    NoMoreTableSlots = 0x2000,
    #[error("Zero in the denominator")]
    DenominatorIsZero = 0x2100,
    #[error("Overflow")]
    ArithmeticOverflow = 0x2101,
    #[error("Modulo Overflow")]
    ModuloOverflow = 0x2102,
    #[error("New precise number error")]
    NewPreciseNumberError = 0x2103,
    #[error("Cast to imprecise number error")]
    CastToImpreciseNumberError = 0x2104,

    #[error("Incorrect weight table admin")]
    IncorrectWeightTableAdmin = 0x2200,
    #[error("Cannnot create future weight tables")]
    CannotCreateFutureWeightTables = 0x2201,
    #[error("Weight mints do not match - length")]
    WeightMintsDoNotMatchLength = 0x2202,
    #[error("Weight mints do not match - mint hash")]
    WeightMintsDoNotMatchMintHash = 0x2203,
}

impl<T> DecodeError<T> for TipRouterError {
    fn type_of() -> &'static str {
        "jito::weight_table"
    }
}

impl From<TipRouterError> for ProgramError {
    fn from(e: TipRouterError) -> Self {
        Self::Custom(e as u32)
    }
}

impl From<TipRouterError> for u64 {
    fn from(e: TipRouterError) -> Self {
        e as Self
    }
}

impl From<TipRouterError> for u32 {
    fn from(e: TipRouterError) -> Self {
        e as Self
    }
}
