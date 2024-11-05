use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use jito_restaking_core::ncn::Ncn;
use jito_tip_router_core::ncn_config::NcnConfig;
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult,
    program_error::ProgramError, pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

pub fn process_initialize_ncn_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    dao_fee_bps: u64,
    ncn_fee_bps: u64,
    block_engine_fee_bps: u64,
) -> ProgramResult {
    let [config, ncn_account, fee_wallet, tie_breaker_admin, payer, restaking_program_id, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_system_account(config, true)?;
    load_signer(payer, false)?;
    load_system_program(system_program)?;

    Ncn::load(restaking_program_id.key, ncn_account, false)?;

    let (config_pda, config_bump, mut config_seeds) = NcnConfig::find_program_address(program_id);
    config_seeds.push(vec![config_bump]);

    if config_pda != *config.key {
        return Err(ProgramError::InvalidSeeds);
    }

    create_account(
        payer,
        config,
        system_program,
        program_id,
        &Rent::get()?,
        8_u64
            .checked_add(std::mem::size_of::<NcnConfig>() as u64)
            .unwrap(),
        &config_seeds,
    )?;

    let epoch = Clock::get()?.epoch as u64;

    let mut config_data = config.try_borrow_mut_data()?;
    config_data[0] = NcnConfig::DISCRIMINATOR;
    let config = NcnConfig::try_from_slice_unchecked_mut(&mut config_data)?;
    *config = NcnConfig::new(
        *ncn_account.key,
        *tie_breaker_admin.key,
        *fee_wallet.key,
        dao_fee_bps,
        ncn_fee_bps,
        block_engine_fee_bps,
        epoch,
    );
    config.bump = config_bump;

    Ok(())
}
