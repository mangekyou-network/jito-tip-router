use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::pubkey::Pubkey;

#[rustfmt::skip]
#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum WeightTableInstruction {


    /// Initialize the global configuration for this NCN
    #[account(0, writable, name = "config")]
    #[account(1, name = "ncn")]
    #[account(2, name = "fee_wallet")]
    #[account(3, name = "tie_breaker_admin")]
    #[account(4, writable, name = "payer")]
    #[account(5, name = "restaking_program_id")]
    #[account(6, name = "system_program")]
    InitializeConfig {
        dao_fee_bps: u64,
        ncn_fee_bps: u64,
        block_engine_fee_bps: u64,
    },

    /// Initializes the weight table for a given NCN epoch
    #[account(0, name = "restaking_config")]
    #[account(1, name = "ncn")]
    #[account(2, writable, signer, name = "weight_table")]
    #[account(3, writable, signer, name = "weight_table_admin")]
    #[account(4, name = "restaking_program_id")]
    #[account(5, name = "system_program")]
    InitializeWeightTable{
        first_slot_of_ncn_epoch: Option<u64>,
    },

    /// Updates the weight table
    #[account(0, name = "ncn")]
    #[account(1, writable, name = "weight_table")]
    #[account(2, signer, name = "weight_table_admin")]
    #[account(3, name = "restaking_program_id")]
    UpdateWeightTable{
        ncn_epoch: u64,
        weight_numerator: u64,
        weight_denominator: u64,
    },

    #[account(0, name = "ncn")]
    #[account(1, writable, name = "weight_table")]
    #[account(2, signer, name = "weight_table_admin")]
    #[account(3, name = "restaking_program_id")]
    FinalizeWeightTable{
        ncn_epoch: u64,
    },

    /// Updates the fee configuration
    #[account(0, writable, name = "config")]
    #[account(1, name = "ncn")]
    #[account(2, signer, name = "ncn_admin")]
    #[account(3, name = "restaking_program_id")]
    SetConfigFees {
        new_dao_fee_bps: Option<u64>,
        new_ncn_fee_bps: Option<u64>,
        new_block_engine_fee_bps: Option<u64>,
        new_fee_wallet: Option<Pubkey>,
    },

}
