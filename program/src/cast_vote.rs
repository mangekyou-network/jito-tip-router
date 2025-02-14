use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_restaking_core::{ncn::Ncn, operator::Operator};
use jito_tip_router_core::{
    ballot_box::{Ballot, BallotBox},
    config::Config as NcnConfig,
    epoch_snapshot::{EpochSnapshot, OperatorSnapshot},
    epoch_state::EpochState,
    error::TipRouterError,
};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

pub fn process_cast_vote(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    meta_merkle_root: &[u8; 32],
    epoch: u64,
) -> ProgramResult {
    let [epoch_state, ncn_config, ballot_box, ncn, epoch_snapshot, operator_snapshot, operator, operator_admin] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Operator is casting the vote, needs to be signer
    load_signer(operator_admin, false)?;

    EpochState::load(program_id, epoch_state, ncn.key, epoch, false)?;
    NcnConfig::load(program_id, ncn_config, ncn.key, false)?;
    Ncn::load(&jito_restaking_program::id(), ncn, false)?;
    Operator::load(&jito_restaking_program::id(), operator, false)?;

    BallotBox::load(program_id, ballot_box, ncn.key, epoch, true)?;
    EpochSnapshot::load(program_id, epoch_snapshot, ncn.key, epoch, false)?;
    OperatorSnapshot::load(
        program_id,
        operator_snapshot,
        operator.key,
        ncn.key,
        epoch,
        false,
    )?;
    let operator_data = operator.data.borrow();
    let operator_account = Operator::try_from_slice_unchecked(&operator_data)?;

    if *operator_admin.key != operator_account.voter {
        return Err(TipRouterError::InvalidOperatorVoter.into());
    }

    let valid_slots_after_consensus = {
        let ncn_config_data = ncn_config.data.borrow();
        let ncn_config = NcnConfig::try_from_slice_unchecked(&ncn_config_data)?;
        ncn_config.valid_slots_after_consensus()
    };

    let mut ballot_box_data = ballot_box.data.borrow_mut();
    let ballot_box = BallotBox::try_from_slice_unchecked_mut(&mut ballot_box_data)?;

    let total_stake_weights = {
        let epoch_snapshot_data = epoch_snapshot.data.borrow();
        let epoch_snapshot = EpochSnapshot::try_from_slice_unchecked(&epoch_snapshot_data)?;

        if !epoch_snapshot.finalized() {
            return Err(TipRouterError::EpochSnapshotNotFinalized.into());
        }

        *epoch_snapshot.stake_weights()
    };

    let operator_stake_weights = {
        let operator_snapshot_data = operator_snapshot.data.borrow();
        let operator_snapshot =
            OperatorSnapshot::try_from_slice_unchecked(&operator_snapshot_data)?;

        *operator_snapshot.stake_weights()
    };

    // if operator_stake_weights.stake_weight() == 0 {
    //     msg!("Operator has zero stake weight, cannot vote");
    //     return Err(TipRouterError::CannotVoteWithZeroStake.into());
    // }

    let slot = Clock::get()?.slot;

    let ballot = Ballot::new(meta_merkle_root);

    ballot_box.cast_vote(
        operator.key,
        &ballot,
        &operator_stake_weights,
        slot,
        valid_slots_after_consensus,
    )?;

    ballot_box.tally_votes(total_stake_weights.stake_weight(), slot)?;

    if ballot_box.is_consensus_reached() {
        msg!(
            "Consensus reached for epoch {} with ballot {:?}",
            epoch,
            ballot_box.get_winning_ballot_tally()?
        );
    }

    // Update Epoch State
    {
        let mut epoch_state_data = epoch_state.try_borrow_mut_data()?;
        let epoch_state_account = EpochState::try_from_slice_unchecked_mut(&mut epoch_state_data)?;
        epoch_state_account.update_cast_vote(
            ballot_box.operators_voted(),
            ballot_box.is_consensus_reached(),
            slot,
        )?;
    }

    Ok(())
}
