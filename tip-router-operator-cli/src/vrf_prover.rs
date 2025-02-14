use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
        instruction::{AccountMeta, Instruction},
    },
    mangekyou::kamui_vrf::{
        ecvrf::{ECVRFKeyPair, ECVRFProof},
        VRFProof,
        VRFKeyPair,
    },
    kamui_program::{
        instruction::{VrfCoordinatorInstruction, VerifyVrfInput},
        state::{Subscription, VrfResult},
    },
    anyhow::Result,
    log::{info, error},
    std::{str::FromStr, time::Duration},
    rand::thread_rng,
    borsh::BorshDeserialize,
};

pub struct VrfProver {
    rpc_client: RpcClient,
    vrf_coordinator: Pubkey,
    vrf_verify: Pubkey,
    subscription: Pubkey,
    keypair: Keypair,
    vrf_keypair: ECVRFKeyPair,
}

impl VrfProver {
    pub fn new(
        rpc_url: &str,
        keypair: Keypair,
        vrf_coordinator: &str,
        vrf_verify: &str,
        subscription: &str,
    ) -> Result<Self> {
        let rpc_client = RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        );
        
        let vrf_keypair = ECVRFKeyPair::generate(&mut thread_rng());
        
        Ok(Self {
            rpc_client,
            vrf_coordinator: Pubkey::from_str(vrf_coordinator)?,
            vrf_verify: Pubkey::from_str(vrf_verify)?,
            subscription: Pubkey::from_str(subscription)?,
            keypair,
            vrf_keypair,
        })
    }

    pub async fn generate_and_submit_proof(&self, seed: [u8; 32]) -> Result<()> {
        // Generate VRF proof
        let (output, proof) = self.vrf_keypair.output(&seed);
        let proof_bytes = proof.to_bytes();
        let public_key_bytes = self.vrf_keypair.pk.as_ref().to_vec();

        info!("Generated proof:");
        info!("  Gamma: {:?}", hex::encode(&proof_bytes[0..32]));
        info!("  Challenge: {:?}", hex::encode(&proof_bytes[32..48]));
        info!("  Scalar: {:?}", hex::encode(&proof_bytes[48..80]));
        
        // Verify proof first
        let verify_input = VerifyVrfInput {
            alpha_string: seed.to_vec(),
            proof_bytes: proof_bytes.clone(),
            public_key_bytes: public_key_bytes.clone(),
        };

        let verify_ix = Instruction::new_with_borsh(
            self.vrf_verify,
            &verify_input,
            vec![AccountMeta::new(self.keypair.pubkey(), true)],
        );

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let verify_tx = Transaction::new_signed_with_payer(
            &[verify_ix],
            Some(&self.keypair.pubkey()),
            &[&self.keypair],
            recent_blockhash,
        );
        
        info!("Verifying proof...");
        self.rpc_client.send_and_confirm_transaction(&verify_tx)?;
        
        // Submit proof to coordinator
        let fulfill_ix = VrfCoordinatorInstruction::FulfillRandomness {
            proof: proof_bytes.to_vec(),
            public_key: public_key_bytes,
        };
        
        let fulfill_ix_data = borsh::to_vec(&fulfill_ix)?;
        let fulfill_ix = Instruction {
            program_id: self.vrf_coordinator,
            accounts: vec![
                AccountMeta::new(self.keypair.pubkey(), true),
                AccountMeta::new(self.subscription, false),
            ],
            data: fulfill_ix_data,
        };

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let fulfill_tx = Transaction::new_signed_with_payer(
            &[fulfill_ix],
            Some(&self.keypair.pubkey()),
            &[&self.keypair],
            recent_blockhash,
        );
        
        info!("Submitting proof to coordinator...");
        self.rpc_client.send_and_confirm_transaction(&fulfill_tx)?;
        
        Ok(())
    }

    pub async fn start_proving(&self) -> Result<()> {
        info!("Starting VRF prover...");
        info!("VRF Coordinator: {}", self.vrf_coordinator);
        info!("VRF Verify: {}", self.vrf_verify);
        info!("Subscription: {}", self.subscription);

        loop {
            // Check for new VRF requests
            if let Ok(requests) = self.get_pending_requests().await {
                for request in requests {
                    if let Err(e) = self.generate_and_submit_proof(request.seed).await {
                        error!("Error processing request: {}", e);
                    }
                }
            }
            
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    async fn get_pending_requests(&self) -> Result<Vec<VrfRequest>> {
        // Get subscription data
        let subscription_data = self.rpc_client.get_account_data(&self.subscription)?;
        let subscription = Subscription::try_from_slice(&subscription_data[8..])?; // Skip discriminator

        // Get request account
        let (request_account, _) = Pubkey::find_program_address(
            &[
                b"request",
                self.subscription.as_ref(),
                subscription.nonce.to_le_bytes().as_ref(),
            ],
            &self.vrf_coordinator,
        );

        // Check if request exists and is pending
        if let Ok(request_data) = self.rpc_client.get_account_data(&request_account) {
            if let Ok(request) = VrfRequest::try_from_slice(&request_data[8..]) {
                if request.status == RequestStatus::Pending {
                    return Ok(vec![request]);
                }
            }
        }

        Ok(vec![])
    }
}

#[derive(BorshDeserialize)]
struct VrfRequest {
    pub seed: [u8; 32],
    pub status: RequestStatus,
}

#[derive(BorshDeserialize, PartialEq)]
enum RequestStatus {
    Pending,
    Fulfilled,
    Failed,
} 