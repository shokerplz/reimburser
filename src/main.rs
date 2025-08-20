use std::path::PathBuf;

use crate::ns_pdf_scanner::ns_pdf_scanner;
use clap::Parser;

mod data;
mod ns_pdf_scanner;
mod trip_filter;

/// Simple
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", ns_pdf_scanner(PathBuf::from(args.input)).unwrap());
    //    trip_filter::trip_filter();
}
