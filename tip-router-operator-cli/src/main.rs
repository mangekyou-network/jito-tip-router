use {
    anyhow::Result,
    clap::Parser,
    ellipsis_client::EllipsisClient,
    log::{error, info},
    solana_rpc_client::rpc_client::RpcClient,
    solana_sdk::{signer::keypair::read_keypair_file, pubkey::Pubkey},
    tip_router_operator_cli::{
        cli::{Cli, Commands},
        process_epoch::{get_previous_epoch_last_slot, process_epoch, wait_for_next_epoch},
        vrf_monitor::VrfMonitor,
        vrf_prover::VrfProver,
    },
    std::str::FromStr,
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    let keypair = read_keypair_file(&cli.keypair_path).expect("Failed to read keypair file");
    let rpc_client = EllipsisClient::from_rpc(
        RpcClient::new(cli.rpc_url.clone()),
        &read_keypair_file(&cli.keypair_path).expect("Failed to read keypair file"),
    )?;

    match &cli.command {
        Commands::Run {
            ncn_address,
            tip_distribution_program_id,
            tip_payment_program_id,
            enable_snapshots,
            vrf_subscription,
        } => {
            info!("Running Tip Router...");
            
            // Start VRF services if enabled
            if cli.vrf_enabled {
                if let Some(subscription) = vrf_subscription {
                    // Start VRF monitor
                    let monitor = VrfMonitor::new(
                        &cli.rpc_url,
                        keypair.clone(),
                        &cli.vrf_coordinator_program,
                        subscription,
                    )?;
                    
                    // Start VRF prover
                    let prover = VrfProver::new(
                        &cli.rpc_url,
                        keypair.clone(),
                        &cli.vrf_coordinator_program,
                        &cli.vrf_verify_program,
                        subscription,
                    )?;
                    
                    // Spawn monitoring task
                    tokio::spawn(async move {
                        if let Err(e) = monitor.start_monitoring().await {
                            error!("VRF monitor error: {}", e);
                        }
                    });
                    
                    // Spawn proving task
                    tokio::spawn(async move {
                        if let Err(e) = prover.start_proving().await {
                            error!("VRF prover error: {}", e);
                        }
                    });
                } else {
                    error!("VRF enabled but no subscription provided!");
                    return Ok(());
                }
            }

            let ncn_pubkey = Pubkey::from_str(ncn_address)?;
            let tip_distribution_program_id = Pubkey::from_str(tip_distribution_program_id)?;
            let tip_payment_program_id = Pubkey::from_str(tip_payment_program_id)?;

            let mut current_epoch = rpc_client.get_epoch().await?;
            let mut last_processed_epoch = current_epoch;

            loop {
                current_epoch = rpc_client.get_epoch().await?;

                if current_epoch > last_processed_epoch {
                    info!("Processing epoch {}", current_epoch);

                    let previous_epoch_last_slot =
                        get_previous_epoch_last_slot(&rpc_client, current_epoch).await?;

                    process_epoch(
                        &rpc_client,
                        &keypair,
                        ncn_pubkey,
                        tip_distribution_program_id,
                        tip_payment_program_id,
                        current_epoch,
                        previous_epoch_last_slot,
                        *enable_snapshots,
                    )
                    .await?;

                    last_processed_epoch = current_epoch;
                }

                wait_for_next_epoch(&rpc_client, current_epoch).await?;
            }
        }
    }
}
