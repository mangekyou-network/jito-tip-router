use bytemuck::{Pod, Zeroable};
use shank::ShankType;
use spl_math::precise_number::PreciseNumber;

use crate::error::TipRouterError;

// Weights are stored as the number of tokens
#[derive(Default, Debug, Clone, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct JitoNumber {
    value: [u8; 16], // 128
}

impl JitoNumber {
    pub const PRECISION_FACTOR: u128 = 1_000_000_000;

    const fn _from_u128(value: u128) -> Self {
        let value = value.to_le_bytes();
        Self { value }
    }

    const fn _to_u128(&self) -> u128 {
        u128::from_le_bytes(self.value)
    }

    fn _to_precise_number(&self) -> Result<PreciseNumber, TipRouterError> {
        PreciseNumber::new(self._to_u128()).ok_or(TipRouterError::NewPreciseNumberError)
    }

    fn _from_precise_number(precise_number: PreciseNumber) -> Result<Self, TipRouterError> {
        let value = precise_number
            .to_imprecise()
            .ok_or(TipRouterError::CastToImpreciseNumberError)?;

        Ok(Self::_from_u128(value))
    }

    fn from_token_supply(token_supply: u64) -> Result<Self, TipRouterError> {
        let value = (token_supply as u128)
            .checked_mul(Self::PRECISION_FACTOR)
            .ok_or(TipRouterError::ArithmeticOverflow)?;

        Ok(Self::_from_u128(value))
    }

    fn to_token_supply(self) -> Result<u64, TipRouterError> {
        let value = self._to_u128();
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
        value._to_precise_number()
    }

    pub fn precise_number_to_token_supply(
        precise_number: PreciseNumber,
    ) -> Result<u64, TipRouterError> {
        let value = Self::_from_precise_number(precise_number)?;
        value.to_token_supply()
    }

    pub const fn from_weight(weight: u128) -> Self {
        Self::_from_u128(weight)
    }

    pub const fn to_weight(&self) -> u128 {
        self._to_u128()
    }

    pub fn weight_to_precise_number(&self) -> Result<PreciseNumber, TipRouterError> {
        self._to_precise_number()
    }

    pub fn precise_number_to_weight(precise_number: PreciseNumber) -> Result<Self, TipRouterError> {
        Self::_from_precise_number(precise_number)
    }
}
