use std::fmt::{Debug, Formatter};

use solana_program::clock::Clock;
use solana_program_test::{processor, BanksClientError, ProgramTest, ProgramTestContext};

use super::{
    restaking_client::{NcnRoot, RestakingProgramClient},
    tip_router_client::TipRouterClient,
    vault_client::VaultProgramClient,
    TestResult,
};

pub struct TestBuilder {
    context: ProgramTestContext,
}

impl Debug for TestBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestBuilder",)
    }
}

impl TestBuilder {
    pub async fn new() -> Self {
        let mut program_test = ProgramTest::new(
            "jito_tip_router_program",
            jito_tip_router_program::id(),
            processor!(jito_tip_router_program::process_instruction),
        );
        program_test.add_program(
            "jito_vault_program",
            jito_vault_program::id(),
            processor!(jito_vault_program::process_instruction),
        );
        program_test.add_program(
            "jito_restaking_program",
            jito_restaking_program::id(),
            processor!(jito_restaking_program::process_instruction),
        );
        program_test.prefer_bpf(true);

        Self {
            context: program_test.start_with_context().await,
        }
    }

    pub async fn warp_slot_incremental(
        &mut self,
        incremental_slots: u64,
    ) -> Result<(), BanksClientError> {
        let clock: Clock = self.context.banks_client.get_sysvar().await?;
        self.context
            .warp_to_slot(clock.slot.checked_add(incremental_slots).unwrap())
            .map_err(|_| BanksClientError::ClientError("failed to warp slot"))?;
        Ok(())
    }

    pub async fn clock(&mut self) -> Clock {
        self.context.banks_client.get_sysvar().await.unwrap()
    }

    pub fn tip_router_client(&self) -> TipRouterClient {
        TipRouterClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    pub fn restaking_program_client(&self) -> RestakingProgramClient {
        RestakingProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    pub fn vault_client(&self) -> VaultProgramClient {
        VaultProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    pub async fn setup_ncn(&mut self) -> TestResult<NcnRoot> {
        let mut restaking_program_client = self.restaking_program_client();

        restaking_program_client.do_initialize_config().await?;
        let ncn_root = restaking_program_client.do_initialize_ncn().await?;

        Ok(ncn_root)
    }

    // Extend this to setup operators, vaults and relationships as neede
}
