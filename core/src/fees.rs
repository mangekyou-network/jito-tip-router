use bytemuck::{Pod, Zeroable};
use shank::ShankType;
use solana_program::{msg, pubkey::Pubkey};
use spl_math::precise_number::PreciseNumber;

use crate::{error::TipRouterError, MAX_FEE_BPS};

/// Fee account. Allows for fee updates to take place in a future epoch without requiring an update.
/// This is important so all operators calculate the same Merkle root regardless of when fee changes take place.
#[derive(Debug, Clone, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct Fees {
    fee_1: Fee,
    fee_2: Fee,
}

impl Fees {
    pub const fn new(
        wallet: Pubkey,
        dao_fee_share_bps: u64,
        ncn_fee_share_bps: u64,
        block_engine_fee_bps: u64,
        current_epoch: u64,
    ) -> Self {
        let fee = Fee::new(
            wallet,
            dao_fee_share_bps,
            ncn_fee_share_bps,
            block_engine_fee_bps,
            current_epoch,
        );
        Self {
            fee_1: fee,
            fee_2: fee,
        }
    }

    const fn current_fee(&self, current_epoch: u64) -> &Fee {
        // If either fee is not yet active, return the other one
        if self.fee_1.activation_epoch > current_epoch {
            return &self.fee_2;
        }
        if self.fee_2.activation_epoch > current_epoch {
            return &self.fee_1;
        }

        // Otherwise return the one with higher activation epoch
        if self.fee_1.activation_epoch >= self.fee_2.activation_epoch {
            &self.fee_1
        } else {
            &self.fee_2
        }
    }

    pub fn check_fees_okay(&self, current_epoch: u64) -> Result<(), TipRouterError> {
        let _ = self.precise_block_engine_fee(current_epoch)?;
        let _ = self.precise_dao_fee(current_epoch)?;
        let _ = self.precise_ncn_fee(current_epoch)?;

        Ok(())
    }

    pub const fn block_engine_fee(&self, current_epoch: u64) -> u64 {
        self.current_fee(current_epoch).block_engine_fee_bps
    }

    pub fn precise_block_engine_fee(
        &self,
        current_epoch: u64,
    ) -> Result<PreciseNumber, TipRouterError> {
        let fee = self.current_fee(current_epoch);

        PreciseNumber::new(fee.block_engine_fee_bps as u128)
            .ok_or(TipRouterError::NewPreciseNumberError)
    }

    /// Calculate fee as a portion of remaining BPS after block engine fee
    /// new_fee = dao_fee_bps / ((10000 - block_engine_fee_bps) / 10000)
    /// = dao_fee_bps * 10000 / (10000 - block_engine_fee_bps)
    pub fn dao_fee(&self, current_epoch: u64) -> Result<u64, TipRouterError> {
        let fee = self.current_fee(current_epoch);
        let remaining_bps = MAX_FEE_BPS
            .checked_sub(fee.block_engine_fee_bps)
            .ok_or(TipRouterError::ArithmeticOverflow)?;
        fee.ncn_share_bps
            .checked_mul(MAX_FEE_BPS)
            .and_then(|x| x.checked_div(remaining_bps))
            .ok_or(TipRouterError::DenominatorIsZero)
    }

    pub fn precise_dao_fee(&self, current_epoch: u64) -> Result<PreciseNumber, TipRouterError> {
        let fee = self.current_fee(current_epoch);

        let remaining_bps = MAX_FEE_BPS
            .checked_sub(fee.block_engine_fee_bps)
            .ok_or(TipRouterError::ArithmeticOverflow)?;

        let precise_remaining_bps = PreciseNumber::new(remaining_bps as u128)
            .ok_or(TipRouterError::NewPreciseNumberError)?;

        let dao_fee = fee
            .ncn_share_bps
            .checked_mul(MAX_FEE_BPS)
            .ok_or(TipRouterError::ArithmeticOverflow)?;

        let precise_dao_fee =
            PreciseNumber::new(dao_fee as u128).ok_or(TipRouterError::NewPreciseNumberError)?;

        precise_dao_fee
            .checked_div(&precise_remaining_bps)
            .ok_or(TipRouterError::DenominatorIsZero)
    }

    /// Calculate fee as a portion of remaining BPS after block engine fee
    /// new_fee = ncn_fee_bps / ((10000 - block_engine_fee_bps) / 10000)
    /// = ncn_fee_bps * 10000 / (10000 - block_engine_fee_bps)
    pub fn ncn_fee(&self, current_epoch: u64) -> Result<u64, TipRouterError> {
        let fee = self.current_fee(current_epoch);

        let remaining_bps = MAX_FEE_BPS
            .checked_sub(fee.block_engine_fee_bps)
            .ok_or(TipRouterError::ArithmeticOverflow)?;
        fee.ncn_share_bps
            .checked_mul(MAX_FEE_BPS)
            .and_then(|x| x.checked_div(remaining_bps))
            .ok_or(TipRouterError::DenominatorIsZero)
    }

    pub fn precise_ncn_fee(&self, current_epoch: u64) -> Result<PreciseNumber, TipRouterError> {
        let fee = self.current_fee(current_epoch);

        let remaining_bps = MAX_FEE_BPS
            .checked_sub(fee.block_engine_fee_bps)
            .ok_or(TipRouterError::ArithmeticOverflow)?;

        let precise_remaining_bps = PreciseNumber::new(remaining_bps as u128)
            .ok_or(TipRouterError::NewPreciseNumberError)?;

        let ncn_fee = fee
            .ncn_share_bps
            .checked_mul(MAX_FEE_BPS)
            .ok_or(TipRouterError::ArithmeticOverflow)?;

        let precise_ncn_fee =
            PreciseNumber::new(ncn_fee as u128).ok_or(TipRouterError::NewPreciseNumberError)?;

        precise_ncn_fee
            .checked_div(&precise_remaining_bps)
            .ok_or(TipRouterError::DenominatorIsZero)
    }

    pub const fn fee_wallet(&self, current_epoch: u64) -> Pubkey {
        self.current_fee(current_epoch).wallet
    }

    fn get_updatable_fee_mut(&mut self, current_epoch: u64) -> &mut Fee {
        // If either fee is scheduled for next epoch, return that one
        if self.fee_1.activation_epoch > current_epoch {
            return &mut self.fee_1;
        }
        if self.fee_2.activation_epoch > current_epoch {
            return &mut self.fee_2;
        }

        // Otherwise return the one with lower activation epoch
        if self.fee_1.activation_epoch <= self.fee_2.activation_epoch {
            &mut self.fee_1
        } else {
            &mut self.fee_2
        }
    }

    pub fn set_new_fees(
        &mut self,
        new_dao_fee_bps: Option<u64>,
        new_ncn_fee_bps: Option<u64>,
        new_block_engine_fee_bps: Option<u64>,
        new_wallet: Option<Pubkey>,
        current_epoch: u64,
    ) -> Result<(), TipRouterError> {
        let current_fees = *self.current_fee(current_epoch);
        let new_fees = self.get_updatable_fee_mut(current_epoch);
        *new_fees = current_fees;

        if let Some(new_dao_fee_bps) = new_dao_fee_bps {
            if new_dao_fee_bps > MAX_FEE_BPS {
                return Err(TipRouterError::FeeCapExceeded);
            }
            new_fees.ncn_share_bps = new_dao_fee_bps;
        }
        if let Some(new_ncn_fee_bps) = new_ncn_fee_bps {
            if new_ncn_fee_bps > MAX_FEE_BPS {
                return Err(TipRouterError::FeeCapExceeded);
            }
            new_fees.ncn_share_bps = new_ncn_fee_bps;
        }
        if let Some(new_block_engine_fee_bps) = new_block_engine_fee_bps {
            // Block engine fee must be less than MAX_FEE_BPS,
            // otherwise we'll divide by zero when calculating
            // the other fees
            if new_block_engine_fee_bps >= MAX_FEE_BPS {
                msg!("Block engine fee cannot equal or exceed MAX_FEE_BPS");
                return Err(TipRouterError::FeeCapExceeded);
            }
            new_fees.block_engine_fee_bps = new_block_engine_fee_bps;
        }
        if let Some(new_wallet) = new_wallet {
            new_fees.wallet = new_wallet;
        }

        let next_epoch = current_epoch
            .checked_add(1)
            .ok_or(TipRouterError::ArithmeticOverflow)?;

        new_fees.activation_epoch = next_epoch;

        self.check_fees_okay(next_epoch)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct Fee {
    wallet: Pubkey,
    dao_share_bps: u64,
    ncn_share_bps: u64,
    block_engine_fee_bps: u64,
    activation_epoch: u64,
}

impl Fee {
    pub const fn new(
        wallet: Pubkey,
        dao_share_bps: u64,
        ncn_share_bps: u64,
        block_engine_fee_bps: u64,
        epoch: u64,
    ) -> Self {
        Self {
            wallet,
            ncn_share_bps,
            dao_share_bps,
            block_engine_fee_bps,
            activation_epoch: epoch,
        }
    }
}

#[cfg(test)]
mod tests {
    use solana_program::pubkey::Pubkey;

    use super::*;

    #[test]
    fn test_update_fees() {
        let mut fees = Fees::new(Pubkey::new_unique(), 100, 200, 300, 5);
        let new_wallet = Pubkey::new_unique();

        fees.set_new_fees(Some(400), None, None, Some(new_wallet), 10)
            .unwrap();
        assert_eq!(fees.fee_1.ncn_share_bps, 400);
        assert_eq!(fees.fee_1.wallet, new_wallet);
        assert_eq!(fees.fee_1.activation_epoch, 11);
    }

    #[test]
    fn test_update_fees_no_changes() {
        let original = Fee::new(Pubkey::new_unique(), 100, 200, 300, 5);
        let mut fees = Fees::new(Pubkey::new_unique(), 100, 200, 300, 5);
        fees.fee_1 = original.clone();

        fees.set_new_fees(None, None, None, None, 10).unwrap();
        assert_eq!(fees.fee_1.ncn_share_bps, original.ncn_share_bps);
        assert_eq!(fees.fee_1.ncn_share_bps, original.ncn_share_bps);
        assert_eq!(
            fees.fee_1.block_engine_fee_bps,
            original.block_engine_fee_bps
        );
        assert_eq!(fees.fee_1.wallet, original.wallet);
        assert_eq!(fees.fee_1.activation_epoch, 11);
    }

    #[test]
    fn test_update_fees_errors() {
        let mut fees = Fees::new(Pubkey::new_unique(), 100, 200, 300, 5);

        assert_eq!(
            fees.set_new_fees(Some(10001), None, None, None, 10),
            Err(TipRouterError::FeeCapExceeded)
        );

        let mut fees = Fees::new(Pubkey::new_unique(), 100, 200, 300, 5);

        assert_eq!(
            fees.set_new_fees(None, None, None, None, u64::MAX),
            Err(TipRouterError::ArithmeticOverflow)
        );

        let mut fees = Fees::new(Pubkey::new_unique(), 100, 200, 300, 5);

        assert_eq!(
            fees.set_new_fees(None, None, Some(MAX_FEE_BPS), None, 10),
            Err(TipRouterError::FeeCapExceeded)
        );
    }

    #[test]
    fn test_check_fees_okay() {
        let fees = Fees::new(Pubkey::new_unique(), 0, 0, 0, 5);

        fees.check_fees_okay(5).unwrap();

        let fees = Fees::new(Pubkey::new_unique(), 0, 0, MAX_FEE_BPS, 5);

        assert_eq!(
            fees.check_fees_okay(5),
            Err(TipRouterError::DenominatorIsZero)
        );
    }

    #[test]
    fn test_current_fee() {
        let mut fees = Fees::new(Pubkey::new_unique(), 100, 200, 300, 5);

        assert_eq!(fees.current_fee(5).activation_epoch, 5);

        fees.fee_1.activation_epoch = 10;

        assert_eq!(fees.current_fee(5).activation_epoch, 5);
        assert_eq!(fees.current_fee(10).activation_epoch, 10);

        fees.fee_2.activation_epoch = 15;

        assert_eq!(fees.current_fee(12).activation_epoch, 10);
        assert_eq!(fees.current_fee(15).activation_epoch, 15);
    }

    #[test]
    fn test_get_updatable_fee_mut() {
        let mut fees = Fees::new(Pubkey::new_unique(), 100, 200, 300, 5);

        let fee = fees.get_updatable_fee_mut(10);
        fee.ncn_share_bps = 400;
        fee.activation_epoch = 11;

        assert_eq!(fees.fee_1.ncn_share_bps, 400);
        assert_eq!(fees.fee_1.activation_epoch, 11);

        fees.fee_2.activation_epoch = 13;

        let fee = fees.get_updatable_fee_mut(12);
        fee.ncn_share_bps = 500;
        fee.activation_epoch = 13;

        assert_eq!(fees.fee_2.ncn_share_bps, 500);
        assert_eq!(fees.fee_2.activation_epoch, 13);

        assert_eq!(fees.get_updatable_fee_mut(u64::MAX).activation_epoch, 11);
    }
}
