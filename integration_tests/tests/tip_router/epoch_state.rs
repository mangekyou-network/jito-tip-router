#[cfg(test)]
mod tests {
    use crate::fixtures::{test_builder::TestBuilder, TestResult};
    use jito_tip_router_core::{
        constants::MAX_OPERATORS, epoch_state::AccountStatus, ncn_fee_group::NcnFeeGroup,
    };

    #[tokio::test]
    async fn cannot_create_epoch_before_starting_valid_epoch() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();

        fixture.warp_epoch_incremental(1000).await?;

        const OPERATOR_COUNT: usize = 1;
        const VAULT_COUNT: usize = 1;

        let test_ncn = fixture
            .create_initial_test_ncn(OPERATOR_COUNT, VAULT_COUNT, Some(100))
            .await?;

        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let config = tip_router_client.get_ncn_config(ncn).await?;
        let starting_valid_epoch = config.starting_valid_epoch();

        let bad_epoch = starting_valid_epoch - 1;

        let result = tip_router_client
            .do_full_initialize_epoch_state(ncn, bad_epoch)
            .await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn cannot_create_after_epoch_marker() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let mut stake_pool_client = fixture.stake_pool_client();
        let pool_root = stake_pool_client.do_initialize_stake_pool().await?;
        const OPERATOR_COUNT: usize = 1;
        const VAULT_COUNT: usize = 1;

        let test_ncn = fixture
            .create_initial_test_ncn(OPERATOR_COUNT, VAULT_COUNT, None)
            .await?;

        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let epoch = fixture.clock().await.epoch;

        fixture.snapshot_test_ncn(&test_ncn).await?;
        fixture.vote_test_ncn(&test_ncn).await?;
        fixture
            .reward_test_ncn(&test_ncn, 10_000, &pool_root)
            .await?;
        fixture.close_epoch_accounts_for_test_ncn(&test_ncn).await?;

        let epoch_marker = tip_router_client.get_epoch_marker(ncn, epoch).await?;
        assert_eq!(epoch_marker.epoch(), epoch);

        let result = tip_router_client
            .do_full_initialize_epoch_state(ncn, epoch)
            .await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_all_test_ncn_functions_pt1() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();

        const OPERATOR_COUNT: usize = 2;
        const VAULT_COUNT: usize = 3;

        let test_ncn = fixture
            .create_initial_test_ncn(OPERATOR_COUNT, VAULT_COUNT, Some(100))
            .await?;
        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let epoch = fixture.clock().await.epoch;

        {
            fixture.add_epoch_state_for_test_ncn(&test_ncn).await?;
            let epoch_state = tip_router_client.get_epoch_state(ncn, epoch).await?;
            assert_eq!(epoch_state.epoch(), epoch);
        }

        {
            fixture.add_admin_weights_for_test_ncn(&test_ncn).await?;
            let epoch_state = tip_router_client.get_epoch_state(ncn, epoch).await?;
            assert!(epoch_state.set_weight_progress().is_complete());
            assert_eq!(
                epoch_state.set_weight_progress().tally(),
                VAULT_COUNT as u64
            );
            assert_eq!(
                epoch_state.set_weight_progress().total(),
                VAULT_COUNT as u64
            );
            assert_eq!(epoch_state.vault_count(), VAULT_COUNT as u64);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_all_test_ncn_functions_pt2() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();

        const OPERATOR_COUNT: usize = 2;
        const VAULT_COUNT: usize = 3;

        let test_ncn = fixture
            .create_initial_test_ncn(OPERATOR_COUNT, VAULT_COUNT, Some(100))
            .await?;
        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let epoch = fixture.clock().await.epoch;

        fixture.add_epoch_state_for_test_ncn(&test_ncn).await?;
        fixture.add_admin_weights_for_test_ncn(&test_ncn).await?;

        {
            fixture.add_epoch_snapshot_to_test_ncn(&test_ncn).await?;
            let epoch_state = tip_router_client.get_epoch_state(ncn, epoch).await?;
            assert_eq!(epoch_state.operator_count(), OPERATOR_COUNT as u64);
            assert!(!epoch_state.epoch_snapshot_progress().is_invalid());
        }

        {
            fixture
                .add_operator_snapshots_to_test_ncn(&test_ncn)
                .await?;
            let epoch_state = tip_router_client.get_epoch_state(ncn, epoch).await?;

            for i in 0..OPERATOR_COUNT {
                assert_eq!(epoch_state.operator_snapshot_progress(i).tally(), 0);
                assert_eq!(
                    epoch_state.operator_snapshot_progress(i).total(),
                    VAULT_COUNT as u64
                );
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_all_test_ncn_functions_pt3() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();

        const OPERATOR_COUNT: usize = 2;
        const VAULT_COUNT: usize = 3;

        let test_ncn = fixture
            .create_initial_test_ncn(OPERATOR_COUNT, VAULT_COUNT, Some(100))
            .await?;
        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let epoch = fixture.clock().await.epoch;

        fixture.add_epoch_state_for_test_ncn(&test_ncn).await?;
        fixture.add_admin_weights_for_test_ncn(&test_ncn).await?;
        fixture.add_epoch_snapshot_to_test_ncn(&test_ncn).await?;
        fixture
            .add_operator_snapshots_to_test_ncn(&test_ncn)
            .await?;

        {
            fixture
                .add_vault_operator_delegation_snapshots_to_test_ncn(&test_ncn)
                .await?;
            let epoch_state = tip_router_client.get_epoch_state(ncn, epoch).await?;

            assert!(epoch_state.epoch_snapshot_progress().is_complete());
            assert_eq!(
                epoch_state.epoch_snapshot_progress().tally(),
                OPERATOR_COUNT as u64
            );
            assert_eq!(
                epoch_state.epoch_snapshot_progress().total(),
                OPERATOR_COUNT as u64
            );

            for i in 0..OPERATOR_COUNT {
                assert_eq!(
                    epoch_state.operator_snapshot_progress(i).tally(),
                    VAULT_COUNT as u64
                );
                assert_eq!(
                    epoch_state.operator_snapshot_progress(i).total(),
                    VAULT_COUNT as u64
                );
                assert!(epoch_state.operator_snapshot_progress(i).is_complete());
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_all_test_ncn_functions_pt4() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();

        const OPERATOR_COUNT: usize = 2;
        const VAULT_COUNT: usize = 3;

        let test_ncn = fixture
            .create_initial_test_ncn(OPERATOR_COUNT, VAULT_COUNT, Some(100))
            .await?;
        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let epoch = fixture.clock().await.epoch;

        fixture.add_epoch_state_for_test_ncn(&test_ncn).await?;
        fixture.add_admin_weights_for_test_ncn(&test_ncn).await?;
        fixture.add_epoch_snapshot_to_test_ncn(&test_ncn).await?;
        fixture
            .add_operator_snapshots_to_test_ncn(&test_ncn)
            .await?;
        fixture
            .add_vault_operator_delegation_snapshots_to_test_ncn(&test_ncn)
            .await?;

        {
            fixture.add_ballot_box_to_test_ncn(&test_ncn).await?;
            fixture.cast_votes_for_test_ncn(&test_ncn).await?;
            let epoch_state = tip_router_client.get_epoch_state(ncn, epoch).await?;

            let clock = fixture.clock().await;
            let epoch_schedule = fixture.epoch_schedule().await;

            assert!(!epoch_state.was_tie_breaker_set());
            assert_eq!(epoch_state.voting_progress().tally(), OPERATOR_COUNT as u64);
            assert_eq!(
                epoch_state.get_slot_consensus_reached().unwrap(),
                clock.slot
            );
            assert_eq!(
                epoch_state
                    .get_epoch_consensus_reached(&epoch_schedule)
                    .unwrap(),
                clock.epoch
            );
            assert!(epoch_state.voting_progress().is_complete());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_all_test_ncn_functions_pt5() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut stake_pool_client = fixture.stake_pool_client();
        let mut tip_router_client = fixture.tip_router_client();

        const OPERATOR_COUNT: usize = 2;
        const VAULT_COUNT: usize = 3;
        const OPERATOR_FEE_BPS: u16 = 1000;
        const BASE_FEE_BPS: u16 = 1000;
        const NCN_FEE_BPS: u16 = 1000;
        const TOTAL_REWARDS: u64 = 10_000;

        let expected_ncn_rewards = 5000;
        let expected_operator_router_rewards = expected_ncn_rewards / OPERATOR_COUNT as u64;

        let pool_root = stake_pool_client.do_initialize_stake_pool().await?;
        let test_ncn = fixture
            .create_custom_initial_test_ncn(
                OPERATOR_COUNT,
                VAULT_COUNT,
                OPERATOR_FEE_BPS,
                BASE_FEE_BPS,
                NCN_FEE_BPS,
            )
            .await?;

        let ncn = test_ncn.ncn_root.ncn_pubkey;
        let epoch = fixture.clock().await.epoch;

        fixture.add_epoch_state_for_test_ncn(&test_ncn).await?;
        fixture.add_admin_weights_for_test_ncn(&test_ncn).await?;
        fixture.add_epoch_snapshot_to_test_ncn(&test_ncn).await?;
        fixture
            .add_operator_snapshots_to_test_ncn(&test_ncn)
            .await?;
        fixture
            .add_vault_operator_delegation_snapshots_to_test_ncn(&test_ncn)
            .await?;
        fixture.add_ballot_box_to_test_ncn(&test_ncn).await?;
        fixture.cast_votes_for_test_ncn(&test_ncn).await?;

        fixture.add_routers_for_test_ncn(&test_ncn).await?;
        stake_pool_client
            .update_stake_pool_balance(&pool_root)
            .await?;

        fixture
            .route_in_base_rewards_for_test_ncn(&test_ncn, TOTAL_REWARDS, &pool_root)
            .await?;
        fixture
            .route_in_ncn_rewards_for_test_ncn(&test_ncn, &pool_root)
            .await?;

        let epoch_state = tip_router_client.get_epoch_state(ncn, epoch).await?;

        for i in 0..MAX_OPERATORS {
            for group in NcnFeeGroup::all_groups() {
                if i < OPERATOR_COUNT && group == NcnFeeGroup::default() {
                    assert_eq!(
                        epoch_state
                            .ncn_distribution_progress(i, group)
                            .unwrap()
                            .total(),
                        expected_operator_router_rewards
                    );
                    assert_eq!(
                        epoch_state
                            .ncn_distribution_progress(i, group)
                            .unwrap()
                            .tally(),
                        expected_operator_router_rewards
                    );
                    assert!(epoch_state
                        .ncn_distribution_progress(i, group)
                        .unwrap()
                        .is_complete());
                } else if i >= OPERATOR_COUNT {
                    assert_eq!(
                        epoch_state
                            .account_status()
                            .ncn_reward_router(i, group)
                            .unwrap(),
                        AccountStatus::DNE
                    );
                }
            }
        }

        Ok(())
    }
}
