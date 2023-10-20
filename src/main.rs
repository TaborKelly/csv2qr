use log::debug;

use clap::Parser;
use csv2qr::{parse_records, generate_qrs, generate_pdf , Result};
use std::path;

const ABOUT: &str = "
A simple command line tool for generating QR codes from a CSV file.
";
#[derive(Parser, Debug)]
#[clap(author, version, about = ABOUT, long_about = None)]
struct Args {
    /// Turn on debug output
    #[clap(short, long)]
    debug: bool,

    /// CSV file to parse
    #[clap(index = 1)]
    pub csv_path: path::PathBuf,

    /// Output directory
    #[clap(index = 2, default_value = ".")]
    pub output_path: path::PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.debug {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    }
    debug!("args: {:?}", args);

    let records = parse_records(&args.csv_path)?;
    generate_qrs(&records, &args.output_path)?;
    generate_pdf(&records, &args.output_path)?;

    Ok(())
}
