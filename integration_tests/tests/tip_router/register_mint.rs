#[cfg(test)]
mod tests {
    use solana_sdk::{signature::Keypair, signer::Signer};

    use crate::fixtures::{test_builder::TestBuilder, TestResult};

    #[tokio::test]
    async fn test_register_mint_success() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let ncn_root = fixture.setup_ncn().await?;

        // Setup initial state
        tip_router_client.setup_tip_router(&ncn_root).await?;

        // Setup vault and tickets
        let vault = Keypair::new();
        let vault_ncn_ticket = Keypair::new();
        let ncn_vault_ticket = Keypair::new();

        // Register mint
        tip_router_client
            .do_register_mint(
                ncn_root.ncn_pubkey,
                vault.pubkey(),
                vault_ncn_ticket.pubkey(),
                ncn_vault_ticket.pubkey(),
            )
            .await?;

        // Verify mint was registered by checking tracked mints
        let tracked_mints = tip_router_client
            .get_tracked_mints(ncn_root.ncn_pubkey)
            .await?;
        assert_eq!(tracked_mints.mint_count(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_register_mint_fails_without_initialization() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let ncn_root = fixture.setup_ncn().await?;

        // Try to register mint without initialization
        let vault = Keypair::new();
        let vault_ncn_ticket = Keypair::new();
        let ncn_vault_ticket = Keypair::new();

        let result = tip_router_client
            .do_register_mint(
                ncn_root.ncn_pubkey,
                vault.pubkey(),
                vault_ncn_ticket.pubkey(),
                ncn_vault_ticket.pubkey(),
            )
            .await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_register_mint_duplicate() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let ncn_root = fixture.setup_ncn().await?;

        // Setup initial state
        tip_router_client.setup_tip_router(&ncn_root).await?;

        // Setup vault and tickets
        let vault = Keypair::new();
        let vault_ncn_ticket = Keypair::new();
        let ncn_vault_ticket = Keypair::new();

        // Register mint first time
        tip_router_client
            .do_register_mint(
                ncn_root.ncn_pubkey,
                vault.pubkey(),
                vault_ncn_ticket.pubkey(),
                ncn_vault_ticket.pubkey(),
            )
            .await?;

        // Register same mint again
        tip_router_client
            .do_register_mint(
                ncn_root.ncn_pubkey,
                vault.pubkey(),
                vault_ncn_ticket.pubkey(),
                ncn_vault_ticket.pubkey(),
            )
            .await?;

        // Verify mint was only registered once
        let tracked_mints = tip_router_client
            .get_tracked_mints(ncn_root.ncn_pubkey)
            .await?;
        assert_eq!(tracked_mints.mint_count(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_register_mint_fails_with_weight_table() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let ncn_root = fixture.setup_ncn().await?;

        tip_router_client.setup_tip_router(&ncn_root).await?;

        // TODO create ncn and vault with 1 mint, register mint, initialize weight table
        // TODO verify weight table locks register_mint

        Ok(())
    }
}
