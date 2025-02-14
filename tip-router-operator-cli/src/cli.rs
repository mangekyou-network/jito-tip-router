use std::path::PathBuf;

use clap::{Parser, Subcommand};
use solana_sdk::pubkey::Pubkey;

<<<<<<< HEAD
#[derive(Parser)]
#[clap(version = "1.0")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(long, default_value = "~/.config/solana/id.json")]
    pub keypair_path: String,

    #[clap(long, default_value = "https://api.devnet.solana.com")]
    pub rpc_url: String,

    // VRF-specific options
    #[clap(long)]
    pub vrf_enabled: bool,

    #[clap(long, default_value = "BfwfooykCSdb1vgu6FcP75ncUgdcdt4ciUaeaSLzxM4D")]
    pub vrf_coordinator_program: String,

    #[clap(long, default_value = "4qqRVYJAeBynm2yTydBkTJ9wVay3CrUfZ7gf9chtWS5Y")]
    pub vrf_verify_program: String,

    #[arg(short, long)]
    pub operator_address: String,

    #[arg(short, long)]
=======
#[derive(Clone, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long, env)]
    pub keypair_path: String,

    #[arg(short, long, env)]
    pub operator_address: String,

    #[arg(short, long, env, default_value = "http://localhost:8899")]
    pub rpc_url: String,

    #[arg(short, long, env)]
>>>>>>> cf534adfb33ea5afa9eccb11b35199f5b149fea2
    pub ledger_path: PathBuf,

    #[arg(short, long, env)]
    pub full_snapshots_path: Option<PathBuf>,

    #[arg(short, long, env)]
    pub backup_snapshots_dir: PathBuf,

    #[arg(short, long, env)]
    pub snapshot_output_dir: PathBuf,
<<<<<<< HEAD
}

#[derive(Subcommand)]
pub enum Commands {
    Run {
        #[clap(long)]
        ncn_address: String,

        #[clap(long)]
        tip_distribution_program_id: String,

        #[clap(long)]
        tip_payment_program_id: String,

        #[clap(long)]
        enable_snapshots: bool,

        // VRF-specific options
        #[clap(long)]
        vrf_subscription: Option<String>,
=======

    #[arg(short, long, env)]
    pub meta_merkle_tree_dir: PathBuf,

    #[arg(long, env, default_value = "false")]
    pub submit_as_memo: bool,

    /// The price to pay for priority fee
    #[arg(long, env, default_value_t = 1)]
    pub micro_lamports: u64,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Clone)]
pub enum Commands {
    Run {
        #[arg(short, long, env)]
        ncn_address: Pubkey,

        #[arg(long, env)]
        tip_distribution_program_id: Pubkey,

        #[arg(long, env)]
        tip_payment_program_id: Pubkey,

        #[arg(long, env)]
        tip_router_program_id: Pubkey,

        #[arg(long, env, default_value = "false")]
        enable_snapshots: bool,

        #[arg(long, env, default_value = "3")]
        num_monitored_epochs: u64,

        #[arg(long, env, default_value = "false")]
        start_next_epoch: bool,

        #[arg(long, env)]
        override_target_slot: Option<u64>,

        #[arg(long, env, default_value = "false")]
        set_merkle_roots: bool,

        #[arg(long, env, default_value = "false")]
        claim_tips: bool,
    },
    SnapshotSlot {
        #[arg(short, long, env)]
        ncn_address: Pubkey,

        #[arg(long, env)]
        tip_distribution_program_id: Pubkey,

        #[arg(long, env)]
        tip_payment_program_id: Pubkey,

        #[arg(long, env)]
        tip_router_program_id: Pubkey,

        #[arg(long, env, default_value = "false")]
        enable_snapshots: bool,

        #[arg(long, env)]
        slot: u64,
    },
    SubmitEpoch {
        #[arg(short, long, env)]
        ncn_address: Pubkey,

        #[arg(long, env)]
        tip_distribution_program_id: Pubkey,

        #[arg(long, env)]
        tip_router_program_id: Pubkey,

        #[arg(long, env)]
        epoch: u64,
    },
    ClaimTips {
        /// Tip distribution program ID
        #[arg(long, env)]
        tip_distribution_program_id: Pubkey,

        /// Tip router program ID
        #[arg(long, env)]
        tip_router_program_id: Pubkey,

        /// NCN address
        #[arg(long, env)]
        ncn_address: Pubkey,

        /// The epoch to Claim tips for
        #[arg(long, env)]
        epoch: u64,
>>>>>>> cf534adfb33ea5afa9eccb11b35199f5b149fea2
    },
}
