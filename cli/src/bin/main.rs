use clap::Parser;
use jito_mev_tip_distribution_ncn_cli::cli_args::Args;

fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.name);
}
