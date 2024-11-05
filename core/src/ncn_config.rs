// Global configuration for the tip router

// Contains:
// Main NCN address - updatable?
// Admins
// - config admin - should this be here? or just use main NCN admin?
// - Weight table upload admin (hot wallet) (this exists in NCN, do we want it here too? since custom weight table)
// - Tie breaker admin (hot wallet) (depending on tie breaker process?)
// DAO fee share
// NCN fee share

use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use shank::{ShankAccount, ShankType};
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::{discriminators::Discriminators, error::TipRouterError, MAX_FEE_BPS};

// PDA'd ["CONFIG"]
#[derive(Debug, Clone, Copy, Zeroable, ShankType, Pod, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct NcnConfig {
    /// The Restaking program's NCN admin is the signer to create and update this account
    pub ncn: Pubkey,

    pub tie_breaker_admin: Pubkey,

    // Separate fee admin here or just admin?
    pub fees: Fees,

    /// Bump seed for the PDA
    pub bump: u8,

    /// Reserved space
    reserved: [u8; 127],
}

impl Discriminator for NcnConfig {
    const DISCRIMINATOR: u8 = Discriminators::Config as u8;
}

impl NcnConfig {
    pub fn new(
        ncn: Pubkey,
        tie_breaker_admin: Pubkey,
        fee_wallet: Pubkey,
        dao_fee_bps: u64,
        ncn_fee_bps: u64,
        block_engine_fee_bps: u64,
        current_epoch: u64,
    ) -> Self {
        Self {
            ncn,
            tie_breaker_admin,
            fees: Fees::new(
                fee_wallet,
                dao_fee_bps,
                ncn_fee_bps,
                block_engine_fee_bps,
                current_epoch,
            ),
            bump: 0,
            reserved: [0; 127],
        }
    }

    pub fn seeds() -> Vec<Vec<u8>> {
        vec![b"config".to_vec()]
    }

    pub fn find_program_address(program_id: &Pubkey) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds();
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Loads the NCN [`Config`] account
    ///
    /// # Arguments
    /// * `program_id` - The program ID
    /// * `account` - The account to load
    /// * `expect_writable` - Whether the account should be writable
    ///
    /// # Returns
    /// * `Result<(), ProgramError>` - The result of the operation
    pub fn load(
        program_id: &Pubkey,
        account: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if account.owner.ne(program_id) {
            msg!("Config account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("Config account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable {
            msg!("Config account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Config account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.key.ne(&Self::find_program_address(program_id).0) {
            msg!("Config account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}

/// Fee account. Allows for fee updates to take place in a future epoch without requiring an update.
/// This is important so all operators calculate the same Merkle root regardless of when fee changes take place.
#[derive(Debug, Clone, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct Fees {
    fee_1: Fee,
    fee_2: Fee,
}

impl Fees {
    pub fn new(
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

    pub fn block_engine_fee(&self, current_epoch: u64) -> u64 {
        self.current_fee(current_epoch).block_engine_fee_bps
    }

    pub fn dao_fee(&self, current_epoch: u64) -> u64 {
        // TODO adjust based on block engine fee
        self.current_fee(current_epoch).dao_share_bps
    }

    pub fn ncn_fee(&self, current_epoch: u64) -> u64 {
        // TODO adjust based on block engine fee
        self.current_fee(current_epoch).ncn_share_bps
    }

    pub fn fee_wallet(&self, current_epoch: u64) -> Pubkey {
        self.current_fee(current_epoch).wallet
    }

    fn get_updatable_fee_mut(&mut self, current_epoch: u64) -> Result<&mut Fee, TipRouterError> {
        let next_epoch = current_epoch
            .checked_add(1)
            .ok_or(TipRouterError::ArithmeticOverflow)?;

        // If either fee is scheduled for next epoch, return that one
        if self.fee_1.activation_epoch == next_epoch {
            return Ok(&mut self.fee_1);
        }
        if self.fee_2.activation_epoch == next_epoch {
            return Ok(&mut self.fee_2);
        }

        // Otherwise return the one with lower activation epoch
        if self.fee_1.activation_epoch <= self.fee_2.activation_epoch {
            Ok(&mut self.fee_1)
        } else {
            Ok(&mut self.fee_2)
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
        let fee = self.get_updatable_fee_mut(current_epoch)?;
        if let Some(new_dao_fee_bps) = new_dao_fee_bps {
            if new_dao_fee_bps > MAX_FEE_BPS {
                return Err(TipRouterError::FeeCapExceeded);
            }
            fee.dao_share_bps = new_dao_fee_bps;
        }
        if let Some(new_ncn_fee_bps) = new_ncn_fee_bps {
            if new_ncn_fee_bps > MAX_FEE_BPS {
                return Err(TipRouterError::FeeCapExceeded);
            }
            fee.ncn_share_bps = new_ncn_fee_bps;
        }
        if let Some(new_block_engine_fee_bps) = new_block_engine_fee_bps {
            if new_block_engine_fee_bps > MAX_FEE_BPS {
                return Err(TipRouterError::FeeCapExceeded);
            }
            fee.block_engine_fee_bps = new_block_engine_fee_bps;
        }
        if let Some(new_wallet) = new_wallet {
            fee.wallet = new_wallet;
        }
        fee.activation_epoch = current_epoch
            .checked_add(1)
            .ok_or(TipRouterError::ArithmeticOverflow)?;
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
            dao_share_bps,
            ncn_share_bps,
            block_engine_fee_bps,
            activation_epoch: epoch,
        }
    }
}

// TODO Some tests for fees
