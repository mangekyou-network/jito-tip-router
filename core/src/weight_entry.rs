use bytemuck::{Pod, Zeroable};
use jito_bytemuck::types::{PodU128, PodU64};
use shank::ShankType;
use solana_program::pubkey::Pubkey;
use spl_math::precise_number::PreciseNumber;

use crate::error::TipRouterError;

#[derive(Debug, Clone, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct WeightEntry {
    mint: Pubkey,
    weight: PodU128,
    slot_set: PodU64,
    slot_updated: PodU64,
    reserved: [u8; 128],
}

impl Default for WeightEntry {
    fn default() -> Self {
        Self {
            mint: Pubkey::default(),
            weight: PodU128::default(),
            slot_set: PodU64::default(),
            slot_updated: PodU64::default(),
            reserved: [0; 128],
        }
    }
}

impl WeightEntry {
    // Weights should have a decimal precision of 1e12
    // Meaning something has the exchange rate of 1.5, it should be stored as 1.5 * 1e12
    // This gives us 12 decimal places of precision
    pub const DECIMAL_PRECISION: u128 = 1_000_000_000_000; // 1e12

    pub fn new(mint: Pubkey) -> Self {
        Self {
            mint,
            weight: PodU128::from(0),
            slot_set: PodU64::from(0),
            slot_updated: PodU64::from(0),
            reserved: [0; 128],
        }
    }

    // Empty entry, no mint
    pub fn is_empty(&self) -> bool {
        self.mint.eq(&Pubkey::default())
    }

    pub fn is_set(&self) -> bool {
        self.slot_set() > 0
    }

    pub fn slot_set(&self) -> u64 {
        self.slot_set.into()
    }

    pub fn slot_updated(&self) -> u64 {
        self.slot_updated.into()
    }

    pub const fn mint(&self) -> Pubkey {
        self.mint
    }

    pub fn weight(&self) -> u128 {
        self.weight.into()
    }

    pub fn precise_weight(&self) -> Result<PreciseNumber, TipRouterError> {
        PreciseNumber::new(self.weight.into()).ok_or(TipRouterError::NewPreciseNumberError)
    }

    pub fn set_weight(&mut self, weight: u128, current_slot: u64) {
        self.weight = PodU128::from(weight);

        if self.slot_set() == 0 {
            self.slot_set = PodU64::from(current_slot);
            self.slot_updated = PodU64::from(current_slot);
        }

        self.slot_updated = PodU64::from(current_slot);
    }
}

#[cfg(test)]
mod tests {
    use solana_program::pubkey::Pubkey;

    use super::*;

    #[test]
    fn test_weight_entry_new() {
        let mint = Pubkey::new_unique();
        let weight_entry = WeightEntry::new(mint);

        assert_eq!(weight_entry.mint(), mint);
        assert_eq!(weight_entry.weight(), 0);
        assert_eq!(weight_entry.slot_set(), 0);
        assert_eq!(weight_entry.slot_updated(), 0);
    }
}
