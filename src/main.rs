use std::path::PathBuf;

use crate::ns_pdf_scanner::ns_pdf_scanner;
use clap::Parser;
use data::Trip;
use prettytable::{Table, row};
use trip_filter::{trip_station_filter, trip_workday_filter};

mod data;
mod ns_pdf_scanner;
mod trip_filter;

/// Simple
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Invoice from NS in PDF. Can be downloaded from: https://www.ns.nl/mijnns#/betaaloverzicht
    #[arg(short = 'f', long)]
    input: String,
    /// List of train stations you depart from. Multiple stations can be specified using
    /// multiple argumets. Example --from-ns A --from-ns B
    #[arg(long)]
    from_ns: Vec<String>,
    /// List of train stations you arrive to. Multiple stations can be specified using
    /// multiple argumets. Example --to-ns A --to-ns B
    #[arg(long)]
    to_ns: Vec<String>,
    /// Optional. List of GVB stations you depart from. Multiple stations can be specified using
    /// multiple argumets. Example --from-gvb A --from-gvb B
    #[arg(long)]
    from_gvb: Option<Vec<String>>,
    /// Optional. List of GVB stations you arrive to. Multiple stations can be specified using
    /// multiple argumets. Example --to-gvb A --to-gvb B
    #[arg(long)]
    to_gvb: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();
    let trips = ns_pdf_scanner(PathBuf::from(args.input)).unwrap();

    let (mut filtered_ns_trips, mut filtered_gvb_trips) = (Vec::<Trip>::new(), Vec::<Trip>::new());
    filtered_ns_trips = trip_workday_filter(trip_station_filter(trips.0, args.from_ns, args.to_ns));
    if args.from_gvb.is_some() && args.to_gvb.is_some() {
        filtered_gvb_trips = trip_workday_filter(trip_station_filter(
            trips.1,
            args.from_gvb.unwrap(),
            args.to_gvb.unwrap(),
        ));
    }
    let mut table = Table::new();
    table.add_row(row!["Provider", "Date", "From", "To", "Price"]);
    for trip in &filtered_ns_trips {
        table.add_row(row![
            "NS",
            trip.date,
            trip.from,
            trip.to,
            format!("{:.2}", trip.price)
        ]);
    }
    for trip in &filtered_gvb_trips {
        table.add_row(row![
            "GVB",
            trip.date,
            trip.from,
            trip.to,
            format!("{:.2}", trip.price)
        ]);
    }

    table.printstd();
    let ns_total: f32 = filtered_ns_trips.iter().map(|t| t.price).sum();
    let gvb_total: f32 = filtered_gvb_trips.iter().map(|t| t.price).sum();
    let grand_total = ns_total + gvb_total;
    println!("\nNS subtotal:  {:.2}", ns_total);
    if gvb_total > 0.0 {
        println!("GVB subtotal: {:.2}", gvb_total);
    }
    println!("-------------------");
    println!("Grand total: {:.2}", grand_total);
}
