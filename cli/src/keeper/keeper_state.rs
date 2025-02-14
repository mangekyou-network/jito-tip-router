use crate::{
    getters::{
        get_account, get_all_operators_in_ncn, get_all_vaults_in_ncn, get_is_epoch_completed,
        get_tip_router_config, get_total_rewards_to_be_distributed,
    },
    handler::CliHandler,
};
use anyhow::{anyhow, Ok, Result};
use jito_bytemuck::AccountDeserialize;

use jito_tip_router_core::{
    ballot_box::BallotBox,
    base_reward_router::{BaseRewardReceiver, BaseRewardRouter},
    config::Config as TipRouterConfig,
    epoch_snapshot::{EpochSnapshot, OperatorSnapshot},
    epoch_state::{EpochState, State},
    ncn_fee_group::NcnFeeGroup,
    ncn_reward_router::{NcnRewardReceiver, NcnRewardRouter},
    vault_registry::VaultRegistry,
    weight_table::WeightTable,
};
use solana_sdk::{account::Account, pubkey::Pubkey};

#[derive(Default)]
pub struct KeeperState {
    pub epoch: u64,
    pub ncn: Pubkey,
    pub vaults: Vec<Pubkey>,
    pub operators: Vec<Pubkey>,
    pub tip_router_config_address: Pubkey,
    pub vault_registry_address: Pubkey,
    pub epoch_state_address: Pubkey,
    pub weight_table_address: Pubkey,
    pub epoch_snapshot_address: Pubkey,
    pub operator_snapshots_address: Vec<Pubkey>,
    pub ballot_box_address: Pubkey,
    pub base_reward_router_address: Pubkey,
    pub base_reward_receiver_address: Pubkey,
    pub ncn_reward_routers_address: Vec<Vec<Pubkey>>,
    pub ncn_reward_receivers_address: Vec<Vec<Pubkey>>,
    pub epoch_state: Option<Box<EpochState>>,
    pub current_state: Option<State>,
    pub is_epoch_completed: bool,
}

impl KeeperState {
    pub async fn fetch(&mut self, handler: &CliHandler, epoch: u64) -> Result<()> {
        // Fetch all vaults and operators
        let ncn = *handler.ncn()?;
        self.ncn = ncn;

        let vaults = get_all_vaults_in_ncn(handler).await?;
        self.vaults = vaults;

        let operators = get_all_operators_in_ncn(handler).await?;
        self.operators = operators;

        let (tip_router_config_address, _, _) =
            TipRouterConfig::find_program_address(&handler.tip_router_program_id, &ncn);
        self.tip_router_config_address = tip_router_config_address;

        let (vault_registry_address, _, _) =
            VaultRegistry::find_program_address(&handler.tip_router_program_id, &ncn);
        self.vault_registry_address = vault_registry_address;

        let (epoch_state_address, _, _) =
            EpochState::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
        self.epoch_state_address = epoch_state_address;

        let (weight_table_address, _, _) =
            WeightTable::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
        self.weight_table_address = weight_table_address;

        let (epoch_snapshot_address, _, _) =
            EpochSnapshot::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
        self.epoch_snapshot_address = epoch_snapshot_address;

        for operator in self.operators.iter() {
            let (operator_snapshot_address, _, _) = OperatorSnapshot::find_program_address(
                &handler.tip_router_program_id,
                operator,
                &ncn,
                epoch,
            );
            self.operator_snapshots_address
                .push(operator_snapshot_address);
        }

        let (ballot_box_address, _, _) =
            BallotBox::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
        self.ballot_box_address = ballot_box_address;

        let (base_reward_router_address, _, _) =
            BaseRewardRouter::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
        self.base_reward_router_address = base_reward_router_address;

        let (base_reward_receiver_address, _, _) =
            BaseRewardReceiver::find_program_address(&handler.tip_router_program_id, &ncn, epoch);
        self.base_reward_receiver_address = base_reward_receiver_address;

        for operator in self.operators.iter() {
            let mut ncn_reward_routers_address = Vec::default();
            let mut ncn_reward_receivers_address = Vec::default();

            for ncn_fee_group in NcnFeeGroup::all_groups() {
                let (ncn_reward_router_address, _, _) = NcnRewardRouter::find_program_address(
                    &handler.tip_router_program_id,
                    ncn_fee_group,
                    operator,
                    &ncn,
                    epoch,
                );
                ncn_reward_routers_address.push(ncn_reward_router_address);

                let (ncn_reward_receiver_address, _, _) = NcnRewardReceiver::find_program_address(
                    &handler.tip_router_program_id,
                    ncn_fee_group,
                    operator,
                    &ncn,
                    epoch,
                );
                ncn_reward_receivers_address.push(ncn_reward_receiver_address);
            }

            self.ncn_reward_routers_address
                .push(ncn_reward_routers_address);
            self.ncn_reward_receivers_address
                .push(ncn_reward_receivers_address);
        }

        self.update_epoch_state(handler).await?;

        // To ensure that the state is fetched for the correct epoch
        self.epoch = epoch;

        Ok(())
    }

    pub async fn update_epoch_state(&mut self, handler: &CliHandler) -> Result<()> {
        let is_epoch_completed = get_is_epoch_completed(handler, self.epoch).await?;
        if is_epoch_completed {
            self.is_epoch_completed = true;
            return Ok(());
        } else {
            self.is_epoch_completed = false;
        }

        let raw_account = get_account(handler, &self.epoch_state_address).await?;

        if raw_account.is_none() {
            self.epoch_state = None;
            return Ok(());
        }

        let raw_account = raw_account.unwrap();

        if raw_account.data.len() < EpochState::SIZE {
            self.epoch_state = None;
            return Ok(());
        }

        let account = Box::new(*EpochState::try_from_slice_unchecked(
            raw_account.data.as_slice(),
        )?);
        self.epoch_state = Some(account);

        self.update_current_state(handler).await?;

        Ok(())
    }

    pub async fn tip_router_config(&self, handler: &CliHandler) -> Result<Option<TipRouterConfig>> {
        let raw_account = get_account(handler, &self.tip_router_config_address).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();
            let account = TipRouterConfig::try_from_slice_unchecked(raw_account.data.as_slice())?;
            Ok(Some(*account))
        }
    }

    pub async fn vault_registry(&self, handler: &CliHandler) -> Result<Option<VaultRegistry>> {
        let raw_account = get_account(handler, &self.vault_registry_address).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();
            let account = VaultRegistry::try_from_slice_unchecked(raw_account.data.as_slice())?;
            Ok(Some(*account))
        }
    }

    pub async fn weight_table(&self, handler: &CliHandler) -> Result<Option<WeightTable>> {
        let raw_account = get_account(handler, &self.weight_table_address).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();
            let account = WeightTable::try_from_slice_unchecked(raw_account.data.as_slice())?;
            Ok(Some(*account))
        }
    }

    pub async fn epoch_snapshot(&self, handler: &CliHandler) -> Result<Option<EpochSnapshot>> {
        let raw_account = get_account(handler, &self.epoch_snapshot_address).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();

            let account = EpochSnapshot::try_from_slice_unchecked(raw_account.data.as_slice())?;
            Ok(Some(*account))
        }
    }

    pub async fn operator_snapshot(
        &self,
        handler: &CliHandler,
        operator_index: usize,
    ) -> Result<Option<OperatorSnapshot>> {
        let raw_account =
            get_account(handler, &self.operator_snapshots_address[operator_index]).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();
            let account = OperatorSnapshot::try_from_slice_unchecked(raw_account.data.as_slice())?;
            Ok(Some(*account))
        }
    }

    pub async fn ballot_box(&self, handler: &CliHandler) -> Result<Option<Box<BallotBox>>> {
        let raw_account = get_account(handler, &self.ballot_box_address).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();
            let account = Box::new(*BallotBox::try_from_slice_unchecked(
                raw_account.data.as_slice(),
            )?);
            Ok(Some(account))
        }
    }

    pub async fn base_reward_router(
        &self,
        handler: &CliHandler,
    ) -> Result<Option<BaseRewardRouter>> {
        let raw_account = get_account(handler, &self.base_reward_router_address).await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();
            let account = BaseRewardRouter::try_from_slice_unchecked(raw_account.data.as_slice())?;
            Ok(Some(*account))
        }
    }

    pub async fn base_reward_receiver(&self, handler: &CliHandler) -> Result<Option<Account>> {
        let raw_account = get_account(handler, &self.base_reward_receiver_address).await?;

        Ok(raw_account)
    }

    pub async fn ncn_reward_router(
        &self,
        handler: &CliHandler,
        operator_index: usize,
        ncn_fee_group: NcnFeeGroup,
    ) -> Result<Option<NcnRewardRouter>> {
        let raw_account = get_account(
            handler,
            &self.ncn_reward_routers_address[operator_index][ncn_fee_group.group_index()?],
        )
        .await?;

        if raw_account.is_none() {
            Ok(None)
        } else {
            let raw_account = raw_account.unwrap();
            let account = NcnRewardRouter::try_from_slice_unchecked(raw_account.data.as_slice())?;
            Ok(Some(*account))
        }
    }

    pub async fn ncn_reward_receiver(
        &self,
        handler: &CliHandler,
        operator_index: usize,
        ncn_fee_group: NcnFeeGroup,
    ) -> Result<Option<Account>> {
        let raw_account = get_account(
            handler,
            &self.ncn_reward_receivers_address[operator_index][ncn_fee_group.group_index()?],
        )
        .await?;

        Ok(raw_account)
    }

    pub fn epoch_state(&self) -> Result<&EpochState> {
        self.epoch_state
            .as_ref()
            .map(|boxed| boxed.as_ref())
            .ok_or_else(|| anyhow!("Epoch state does not exist"))
    }

    pub async fn update_current_state(&mut self, handler: &CliHandler) -> Result<()> {
        let rpc_client = handler.rpc_client();
        let current_slot = rpc_client.get_epoch_info().await?.absolute_slot;
        let epoch_schedule = rpc_client.get_epoch_schedule().await?;

        let (valid_slots_after_consensus, epochs_after_consensus_before_close) = {
            let config = get_tip_router_config(handler).await?;
            (
                config.valid_slots_after_consensus(),
                config.epochs_after_consensus_before_close(),
            )
        };

        let epoch_state = self.epoch_state()?;

        let state = if epoch_state.set_weight_progress().tally() > 0 {
            let weight_table_result = self.weight_table(handler).await?;

            if weight_table_result.is_none() {
                return Err(anyhow!("Weight table does not exist"));
            }

            epoch_state.current_state_patched(
                &epoch_schedule,
                valid_slots_after_consensus,
                epochs_after_consensus_before_close,
                weight_table_result.unwrap().st_mint_count() as u64,
                current_slot,
            )
        } else {
            epoch_state.current_state(
                &epoch_schedule,
                valid_slots_after_consensus,
                epochs_after_consensus_before_close,
                current_slot,
            )
        };

        self.current_state = Some(state?);

        Ok(())
    }

    pub fn current_state(&self) -> Result<State> {
        let state = self
            .current_state
            .as_ref()
            .ok_or_else(|| anyhow!("Current state does not exist"))?;

        Ok(*state)
    }

    pub async fn detect_stall(&mut self, handler: &CliHandler) -> Result<bool> {
        let current_state = self.current_state()?;

        if current_state == State::Vote {
            return Ok(true);
        }

        if current_state == State::Distribute {
            let total_rewards_to_be_distributed =
                get_total_rewards_to_be_distributed(handler, self.epoch).await?;

            // If dust rewards, then stall
            if total_rewards_to_be_distributed < 10_000 {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
