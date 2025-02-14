use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::loader::load_system_program;
use jito_restaking_core::{
    config::Config, ncn::Ncn, ncn_operator_state::NcnOperatorState, operator::Operator,
};
use jito_tip_router_core::{
    account_payer::AccountPayer,
    config::Config as NcnConfig,
    epoch_snapshot::{EpochSnapshot, OperatorSnapshot},
    epoch_state::EpochState,
    loaders::load_ncn_epoch,
    stake_weight::StakeWeights,
    utils::get_new_size,
};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

pub fn process_realloc_operator_snapshot(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    epoch: u64,
) -> ProgramResult {
    let [epoch_state, ncn_config, restaking_config, ncn, operator, ncn_operator_state, epoch_snapshot, operator_snapshot, account_payer, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    EpochState::load(program_id, epoch_state, ncn.key, epoch, true)?;
    NcnConfig::load(program_id, ncn_config, ncn.key, false)?;
    Config::load(&jito_restaking_program::id(), restaking_config, false)?;
    Ncn::load(&jito_restaking_program::id(), ncn, false)?;
    Operator::load(&jito_restaking_program::id(), operator, false)?;
    NcnOperatorState::load(
        &jito_restaking_program::id(),
        ncn_operator_state,
        ncn,
        operator,
        false,
    )?;
    EpochSnapshot::load(program_id, epoch_snapshot, ncn.key, epoch, true)?;
    AccountPayer::load(program_id, account_payer, ncn.key, true)?;

    load_system_program(system_program)?;

    let (operator_snapshot_pda, operator_snapshot_bump, _) =
        OperatorSnapshot::find_program_address(program_id, operator.key, ncn.key, epoch);

    if operator_snapshot_pda != *operator_snapshot.key {
        msg!("Operator snapshot account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    if operator_snapshot.data_len() < OperatorSnapshot::SIZE {
        let new_size = get_new_size(operator_snapshot.data_len(), OperatorSnapshot::SIZE)?;
        msg!(
            "Reallocating operator snapshot from {} bytes to {} bytes",
            operator_snapshot.data_len(),
            new_size
        );
        AccountPayer::pay_and_realloc(
            program_id,
            ncn.key,
            account_payer,
            operator_snapshot,
            new_size,
        )?;
    }

    let should_initialize = operator_snapshot.data_len() >= OperatorSnapshot::SIZE
        && operator_snapshot.try_borrow_data()?[0] != OperatorSnapshot::DISCRIMINATOR;

    if should_initialize {
        let current_slot = Clock::get()?.slot;
        let (_, ncn_epoch_length) = load_ncn_epoch(restaking_config, current_slot, None)?;

        let (is_active, ncn_operator_index): (bool, u64) = {
            let ncn_operator_state_data = ncn_operator_state.data.borrow();
            let ncn_operator_state_account =
                NcnOperatorState::try_from_slice_unchecked(&ncn_operator_state_data)?;

            // If the NCN removes an operator, it should immediately be barred from the snapshot
            let ncn_operator_okay = ncn_operator_state_account
                .ncn_opt_in_state
                .is_active(current_slot, ncn_epoch_length)?;

            // If the operator removes itself from the ncn, it should still be able to participate
            // while it is cooling down
            let operator_ncn_okay = ncn_operator_state_account
                .operator_opt_in_state
                .is_active_or_cooldown(current_slot, ncn_epoch_length)?;

            let ncn_operator_index = ncn_operator_state_account.index();

            (ncn_operator_okay && operator_ncn_okay, ncn_operator_index)
        };

        let vault_count = {
            let epoch_snapshot_data = epoch_snapshot.data.borrow();
            let epoch_snapshot_account =
                EpochSnapshot::try_from_slice_unchecked(&epoch_snapshot_data)?;
            epoch_snapshot_account.vault_count()
        };

        let (operator_fee_bps, operator_index): (u16, u64) = {
            let operator_data = operator.data.borrow();
            let operator_account = Operator::try_from_slice_unchecked(&operator_data)?;
            (
                operator_account.operator_fee_bps.into(),
                operator_account.index(),
            )
        };

        let mut operator_snapshot_data = operator_snapshot.try_borrow_mut_data()?;
        operator_snapshot_data[0] = OperatorSnapshot::DISCRIMINATOR;
        let operator_snapshot_account =
            OperatorSnapshot::try_from_slice_unchecked_mut(&mut operator_snapshot_data)?;

        operator_snapshot_account.initialize(
            operator.key,
            ncn.key,
            epoch,
            operator_snapshot_bump,
            current_slot,
            is_active,
            ncn_operator_index,
            operator_index,
            operator_fee_bps,
            vault_count,
        )?;

        // Increment operator registration for an inactive operator
        if !is_active {
            let mut epoch_snapshot_data = epoch_snapshot.try_borrow_mut_data()?;
            let epoch_snapshot_account =
                EpochSnapshot::try_from_slice_unchecked_mut(&mut epoch_snapshot_data)?;

            epoch_snapshot_account.increment_operator_registration(
                current_slot,
                0,
                &StakeWeights::default(),
            )?;
        }

        // Update Epoch State
        {
            let mut epoch_state_data = epoch_state.try_borrow_mut_data()?;
            let epoch_state_account =
                EpochState::try_from_slice_unchecked_mut(&mut epoch_state_data)?;
            epoch_state_account
                .update_realloc_operator_snapshot(ncn_operator_index as usize, is_active)?;
        }
    }

    Ok(())
}
