#[cfg(test)]
mod tests {
    use jito_tip_router_core::error::TipRouterError;
    use solana_program::instruction::InstructionError;
    use solana_sdk::signature::{Keypair, Signer};

    use crate::fixtures::{
        assert_ix_error, test_builder::TestBuilder, tip_router_client::assert_tip_router_error,
        TestResult,
    };

    #[tokio::test]
    async fn test_initialize_ncn_config_ok() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let ncn_root = fixture.setup_ncn().await?;
        tip_router_client
            .do_initialize_config(ncn_root.ncn_pubkey, ncn_root.ncn_admin.pubkey())
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_initialize_ncn_config_double_init_fails() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let ncn_root = fixture.setup_ncn().await?;
        tip_router_client
            .do_initialize_config(ncn_root.ncn_pubkey, ncn_root.ncn_admin.pubkey())
            .await?;
        fixture.warp_slot_incremental(1).await?;
        let transaction_error = tip_router_client
            .do_initialize_config(ncn_root.ncn_pubkey, ncn_root.ncn_admin.pubkey())
            .await;
        assert_ix_error(transaction_error, InstructionError::InvalidAccountOwner);
        Ok(())
    }

    #[tokio::test]
    async fn test_initialize_ncn_config_invalid_ncn_fails() -> TestResult<()> {
        let fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let fake_ncn = Keypair::new();
        let fake_admin = Keypair::new();
        let transaction_error = tip_router_client
            .do_initialize_config(fake_ncn.pubkey(), fake_admin.pubkey())
            .await;
        assert_ix_error(transaction_error, InstructionError::InvalidAccountOwner);
        Ok(())
    }

    #[tokio::test]
    async fn test_initialize_ncn_config_fees_exceed_max_fails() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let ncn_root = fixture.setup_ncn().await?;

        let transaction_error = tip_router_client
            .initialize_config(
                ncn_root.ncn_pubkey,
                ncn_root.ncn_admin.pubkey(),
                ncn_root.ncn_admin.pubkey(),
                10_001,
                0,
                0,
            )
            .await;
        assert_tip_router_error(transaction_error, TipRouterError::FeeCapExceeded);
        Ok(())
    }
}
