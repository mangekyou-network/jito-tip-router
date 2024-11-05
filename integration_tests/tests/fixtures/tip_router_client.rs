// How will this differ from the other clients? will i need the other clients just to function?
// Preference using the generated clients here instead of writing SDK

// Future testing schemes:
// - Create lots of vaults (need )
// - Create lots of operators
//   - Operators are gonna be voting
// - register them all (or not)
//   - Lots of ticket creating instructions
// - make sure they all get the right fees
//

// Tip router setups
// - Run the instrucitons

use jito_bytemuck::AccountDeserialize;
use jito_tip_router_client::instructions::{InitializeConfigBuilder, SetConfigFeesBuilder};
use jito_tip_router_core::{error::TipRouterError, ncn_config::NcnConfig};
use solana_program::{
    instruction::InstructionError, native_token::sol_to_lamports, pubkey::Pubkey,
    system_instruction::transfer,
};
use solana_program_test::BanksClient;
use solana_sdk::{
    commitment_config::CommitmentLevel,
    signature::{Keypair, Signer},
    transaction::{Transaction, TransactionError},
};

use super::restaking_client::NcnRoot;
use crate::fixtures::{TestError, TestResult};

pub struct TipRouterClient {
    banks_client: BanksClient,
    payer: Keypair,
}

impl TipRouterClient {
    pub const fn new(banks_client: BanksClient, payer: Keypair) -> Self {
        Self {
            banks_client,
            payer,
        }
    }

    pub async fn process_transaction(&mut self, tx: &Transaction) -> TestResult<()> {
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                tx.clone(),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    pub async fn _airdrop(&mut self, to: &Pubkey, sol: f64) -> TestResult<()> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(&self.payer.pubkey(), to, sol_to_lamports(sol))],
                    Some(&self.payer.pubkey()),
                    &[&self.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    pub async fn get_config(&mut self) -> TestResult<NcnConfig> {
        let config_pda = NcnConfig::find_program_address(&jito_tip_router_program::id()).0;
        let config = self.banks_client.get_account(config_pda).await?.unwrap();
        Ok(*NcnConfig::try_from_slice_unchecked(config.data.as_slice()).unwrap())
    }

    pub async fn do_initialize_config(&mut self, ncn: Pubkey, ncn_admin: Pubkey) -> TestResult<()> {
        self._airdrop(&self.payer.pubkey(), 1.0).await?;

        self.initialize_config(ncn, ncn_admin, ncn_admin, 0, 0, 0)
            .await
    }

    pub async fn initialize_config(
        &mut self,
        ncn: Pubkey,
        fee_wallet: Pubkey,
        tie_breaker_admin: Pubkey,
        dao_fee_bps: u64,
        ncn_fee_bps: u64,
        block_engine_fee_bps: u64,
    ) -> TestResult<()> {
        let config_pda = NcnConfig::find_program_address(&jito_tip_router_program::id()).0;

        let ix = InitializeConfigBuilder::new()
            .config(config_pda)
            .ncn(ncn)
            .fee_wallet(fee_wallet)
            .tie_breaker_admin(tie_breaker_admin)
            .payer(self.payer.pubkey())
            .restaking_program_id(jito_restaking_program::id())
            .dao_fee_bps(dao_fee_bps)
            .ncn_fee_bps(ncn_fee_bps)
            .block_engine_fee_bps(block_engine_fee_bps)
            .instruction();

        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            blockhash,
        ))
        .await
    }

    pub async fn do_set_config_fees(
        &mut self,
        dao_fee_bps: u64,
        ncn_fee_bps: u64,
        block_engine_fee_bps: u64,
        fee_wallet: Pubkey,
        ncn_root: &NcnRoot,
    ) -> TestResult<()> {
        let config_pda = NcnConfig::find_program_address(&jito_tip_router_program::id()).0;
        self._airdrop(&ncn_root.ncn_admin.pubkey(), 1.0).await?;
        self.set_config_fees(
            config_pda,
            dao_fee_bps,
            ncn_fee_bps,
            block_engine_fee_bps,
            fee_wallet,
            &ncn_root,
        )
        .await
    }

    pub async fn set_config_fees(
        &mut self,
        config_pda: Pubkey,
        dao_fee_bps: u64,
        ncn_fee_bps: u64,
        block_engine_fee_bps: u64,
        fee_wallet: Pubkey,
        ncn_root: &NcnRoot,
    ) -> TestResult<()> {
        let ix = SetConfigFeesBuilder::new()
            .config(config_pda)
            .ncn(ncn_root.ncn_pubkey)
            .ncn_admin(ncn_root.ncn_admin.pubkey())
            .restaking_program_id(jito_restaking_program::id())
            .new_dao_fee_bps(dao_fee_bps)
            .new_ncn_fee_bps(ncn_fee_bps)
            .new_block_engine_fee_bps(block_engine_fee_bps)
            .new_fee_wallet(fee_wallet)
            .instruction();

        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&ncn_root.ncn_admin.pubkey()),
            &[&ncn_root.ncn_admin],
            blockhash,
        ))
        .await
    }
}

#[inline(always)]
#[track_caller]
pub fn assert_tip_router_error<T>(
    test_error: Result<T, TestError>,
    tip_router_error: TipRouterError,
) {
    assert!(test_error.is_err());
    assert_eq!(
        test_error.err().unwrap().to_transaction_error().unwrap(),
        TransactionError::InstructionError(0, InstructionError::Custom(tip_router_error as u32))
    );
}
