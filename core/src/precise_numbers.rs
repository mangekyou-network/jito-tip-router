use bytemuck::{Pod, Zeroable};
use shank::ShankType;
use spl_math::precise_number::PreciseNumber;

use crate::error::TipRouterError;

// All arithmetic should be done with PreciseNumber, and cast down to the appropriate types only until all
// calculations are complete.

#[derive(Default, Debug, Clone, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct PreciseNumberWrapper {
    value: [u8; 16], // 128
}

impl PreciseNumberWrapper {
    const fn from_u128(value: u128) -> Self {
        let value = value.to_le_bytes();
        Self { value }
    }

    const fn to_u128(&self) -> u128 {
        u128::from_le_bytes(self.value)
    }

    fn to_precise_number(&self) -> Result<PreciseNumber, TipRouterError> {
        PreciseNumber::new(self.to_u128()).ok_or(TipRouterError::NewPreciseNumberError)
    }

    fn from_precise_number(precise_number: PreciseNumber) -> Result<Self, TipRouterError> {
        let value = precise_number
            .to_imprecise()
            .ok_or(TipRouterError::CastToImpreciseNumberError)?;

        Ok(Self::from_u128(value))
    }
}

#[derive(Default, Debug, Clone, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct PreciseWeight {
    number: PreciseNumberWrapper, // 128
}

impl PreciseWeight {
    pub const fn from_weight(weight: u128) -> Self {
        let number = PreciseNumberWrapper::from_u128(weight);
        Self { number }
    }

    pub const fn to_weight(&self) -> u128 {
        self.number.to_u128()
    }

    pub fn weight_to_precise_number(&self) -> Result<PreciseNumber, TipRouterError> {
        self.number.to_precise_number()
    }

    pub fn precise_number_to_weight(precise_number: PreciseNumber) -> Result<Self, TipRouterError> {
        let number = PreciseNumberWrapper::from_precise_number(precise_number)?;
        Ok(Self { number })
    }
}

#[derive(Default, Debug, Clone, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct PreciseTokenSupply {
    number: PreciseNumberWrapper, // 128
}

impl PreciseTokenSupply {
    pub const PRECISION_FACTOR: u128 = 1_000_000_000;

    fn from_token_supply(token_supply: u64) -> Result<Self, TipRouterError> {
        let value = (token_supply as u128)
            .checked_mul(Self::PRECISION_FACTOR)
            .ok_or(TipRouterError::ArithmeticOverflow)?;

        let number = PreciseNumberWrapper::from_u128(value);

        Ok(Self { number })
    }

    fn to_token_supply(self) -> Result<u64, TipRouterError> {
        let value = self.number.to_u128();
        let token_supply = value
            .checked_div(Self::PRECISION_FACTOR)
            .ok_or(TipRouterError::ArithmeticOverflow)?;

        token_supply
            .try_into()
            .map_err(|_| TipRouterError::ArithmeticOverflow)
    }

    pub fn token_supply_to_precise_number(
        token_supply: u64,
    ) -> Result<PreciseNumber, TipRouterError> {
        let value = Self::from_token_supply(token_supply)?;

        value.number.to_precise_number()
    }

    pub fn precise_number_to_token_supply(
        precise_number: PreciseNumber,
    ) -> Result<u64, TipRouterError> {
        let number = PreciseNumberWrapper::from_precise_number(precise_number)?;
        let value = Self { number };
        value.to_token_supply()
    }
}
