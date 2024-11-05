use std::{fmt, fmt::Debug};

use solana_program::pubkey::Pubkey;
use solana_program_test::BanksClient;
use solana_sdk::signature::Keypair;

pub struct VaultRoot {
    pub vault_pubkey: Pubkey,
    pub vault_admin: Keypair,
}

impl Debug for VaultRoot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VaultRoot {{ vault_pubkey: {}, vault_admin: {:?} }}",
            self.vault_pubkey, self.vault_admin
        )
    }
}

#[allow(dead_code)]
pub struct VaultProgramClient {
    banks_client: BanksClient,
    payer: Keypair,
}

impl VaultProgramClient {
    pub const fn new(banks_client: BanksClient, payer: Keypair) -> Self {
        Self {
            banks_client,
            payer,
        }
    }
}
