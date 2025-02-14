use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::Keypair,
    },
    anyhow::Result,
    log::{info, error},
    std::{str::FromStr, time::Duration},
};

pub struct VrfMonitor {
    rpc_client: RpcClient,
    vrf_coordinator: Pubkey,
    subscription: Pubkey,
    keypair: Keypair,
}

impl VrfMonitor {
    pub fn new(
        rpc_url: &str,
        keypair: Keypair,
        vrf_coordinator: &str,
        subscription: &str,
    ) -> Result<Self> {
        let rpc_client = RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        );
        
        Ok(Self {
            rpc_client,
            vrf_coordinator: Pubkey::from_str(vrf_coordinator)?,
            subscription: Pubkey::from_str(subscription)?,
            keypair,
        })
    }

    pub fn monitor_subscription(&self) -> Result<()> {
        let subscription_data = self.rpc_client
            .get_account_data(&self.subscription)?;
        
        info!("Subscription Status:");
        info!("  Balance: {} SOL", subscription_data.len() as f64 / 1_000_000_000.0);
        Ok(())
    }

    pub async fn start_monitoring(&self) -> Result<()> {
        info!("Starting VRF monitor...");
        info!("VRF Coordinator: {}", self.vrf_coordinator);
        info!("Subscription: {}", self.subscription);

        loop {
            if let Err(e) = self.monitor_subscription() {
                error!("Error monitoring subscription: {}", e);
            }
            
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    }
} 