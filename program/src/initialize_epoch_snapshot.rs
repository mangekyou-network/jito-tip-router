use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::loader::{load_system_account, load_system_program};
use jito_restaking_core::ncn::Ncn;
use jito_tip_router_core::{
    account_payer::AccountPayer, config::Config, epoch_marker::EpochMarker,
    epoch_snapshot::EpochSnapshot, epoch_state::EpochState, error::TipRouterError, fees,
    weight_table::WeightTable,
};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

/// Initializes an Epoch Snapshot
pub fn process_initialize_epoch_snapshot(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    epoch: u64,
) -> ProgramResult {
    let [epoch_marker, epoch_state, config, ncn, weight_table, epoch_snapshot, account_payer, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    EpochState::load_and_check_is_closing(program_id, epoch_state, ncn.key, epoch, true)?;
    Config::load(program_id, config, ncn.key, false)?;
    Ncn::load(&jito_restaking_program::id(), ncn, false)?;
    AccountPayer::load(program_id, account_payer, ncn.key, true)?;
    EpochMarker::check_dne(program_id, epoch_marker, ncn.key, epoch)?;

    load_system_account(epoch_snapshot, true)?;
    load_system_program(system_program)?;

    let current_slot = Clock::get()?.slot;
    let ncn_epoch = epoch;

    WeightTable::load(program_id, weight_table, ncn.key, ncn_epoch, false)?;

    // Weight table needs to be finalized before the snapshot can be taken
    let vault_count = {
        let weight_table_data = weight_table.data.borrow();
        let weight_table_account = WeightTable::try_from_slice_unchecked(&weight_table_data)?;

        if !weight_table_account.finalized() {
            msg!("Weight table must be finalized before initializing epoch snapshot");
            return Err(TipRouterError::WeightTableNotFinalized.into());
        }

        weight_table_account.vault_count()
    };

    let (epoch_snapshot_pubkey, epoch_snapshot_bump, mut epoch_snapshot_seeds) =
        EpochSnapshot::find_program_address(program_id, ncn.key, ncn_epoch);
    epoch_snapshot_seeds.push(vec![epoch_snapshot_bump]);

    if epoch_snapshot_pubkey.ne(epoch_snapshot.key) {
        msg!("Incorrect epoch snapshot PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!(
        "Initializing Epoch snapshot {} for NCN: {} at epoch: {}",
        epoch_snapshot.key,
        ncn.key,
        ncn_epoch
    );
    AccountPayer::pay_and_create_account(
        program_id,
        ncn.key,
        account_payer,
        epoch_snapshot,
        system_program,
        program_id,
        EpochSnapshot::SIZE,
        &epoch_snapshot_seeds,
    )?;

    let ncn_fees: fees::Fees = {
        let ncn_config_data = config.data.borrow();
        let ncn_config_account = Config::try_from_slice_unchecked(&ncn_config_data)?;
        *ncn_config_account.fee_config.current_fees(ncn_epoch)
    };

    let operator_count: u64 = {
        let ncn_data = ncn.data.borrow();
        let ncn_account = Ncn::try_from_slice_unchecked(&ncn_data)?;
        ncn_account.operator_count()
    };

    if operator_count == 0 {
        msg!("No operators to snapshot");
        return Err(TipRouterError::NoOperators.into());
    }

    let mut epoch_snapshot_data: std::cell::RefMut<'_, &mut [u8]> =
        epoch_snapshot.try_borrow_mut_data()?;
    epoch_snapshot_data[0] = EpochSnapshot::DISCRIMINATOR;
    let epoch_snapshot_account =
        EpochSnapshot::try_from_slice_unchecked_mut(&mut epoch_snapshot_data)?;

    *epoch_snapshot_account = EpochSnapshot::new(
        ncn.key,
        ncn_epoch,
        epoch_snapshot_bump,
        current_slot,
        &ncn_fees,
        operator_count,
        vault_count,
    );

    // Update Epoch State
    {
        let mut epoch_state_data = epoch_state.try_borrow_mut_data()?;
        let epoch_state_account = EpochState::try_from_slice_unchecked_mut(&mut epoch_state_data)?;
        epoch_state_account.update_initialize_epoch_snapshot(operator_count);
    }

    Ok(())
}
