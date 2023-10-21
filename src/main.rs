use log::{debug, error};

use clap::Parser;
use csv2qr::{generate_pdf, generate_qrs, parse_records, CsvToQrError, Result};
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

    /// ECC level (low, medium, quartile, or high)
    #[clap(long, default_value = "medium")]
    ecc: String,

    /// Do not delete the intermediate PNG of the QR code
    #[clap(short, long)]
    save_intermediate: bool,

    /// Do not generate the final PDF document, only the intermediate PNG.
    /// This will enable save-intermediate automatically.
    #[clap(short, long)]
    no_pdf: bool,

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

    let ecc_level = match args.ecc.as_str() {
        "low" => qrcode_generator::QrCodeEcc::Low,
        "medium" => qrcode_generator::QrCodeEcc::Medium,
        "high" => qrcode_generator::QrCodeEcc::High,
        "quartile" => qrcode_generator::QrCodeEcc::Quartile,
        _ => {
            error!("unrecognized ecc level: {}", args.ecc);
            return Err(Box::new(CsvToQrError::ParseError));
        }
    };

    let records = parse_records(&args.csv_path, &args.output_path)?;
    generate_qrs(&records, ecc_level)?;
    if !args.no_pdf {
        generate_pdf(&records, args.save_intermediate)?;
    }

    Ok(())
}
