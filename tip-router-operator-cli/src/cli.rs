use std::path::PathBuf;

use clap::{Parser, Subcommand};
use solana_sdk::pubkey::Pubkey;

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
    pub ledger_path: PathBuf,

    #[arg(short, long)]
    pub account_paths: Option<Vec<PathBuf>>,

    #[arg(short, long)]
    pub full_snapshots_path: Option<PathBuf>,

    #[arg(short, long)]
    pub snapshot_output_dir: PathBuf,
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
    },
}
