use std::{f32, path::PathBuf};

use crate::data::ALL_GVB_STATIONS;
use crate::data::ALL_NS_STATIONS;
use crate::data::Provider;
use crate::data::Trip;
use anyhow::Result;

use chrono::NaiveDate;
use regex::Regex;

use pdfium_render::prelude::*;

pub fn ns_pdf_scanner(pdf: PathBuf) -> Result<(Vec<Trip>, Vec<Trip>)> {
    let pdfium = Pdfium::default();

    let re_ns = Regex::new(
        r"^(?P<date>\d{2}-\d{2}-\d{4})\s+NS\s+(?P<kenmerk>.+spits|.+weekend)\s+(?P<from_to>.+?)\s+(?P<class>\d+)\s+€\s*(?P<price>[\d\.,]+)\s*$",
    )?;
    let re_gvb = Regex::new(
        r"^(?P<date>\d{2}-\d{2}-\d{4})\s+GVB\s+(?P<kenmerk>Lijn(\s\d+)?)\s+(?P<from_to>.+?)\s+€\s*(?P<price>[\d\.,]+)\s*$",
    )?;

    let doc = pdfium.load_pdf_from_file(&pdf, None)?;

    let mut gvb_trips: Vec<Trip> = Vec::new();
    let mut ns_trips: Vec<Trip> = Vec::new();

    for (_, page) in doc.pages().iter().enumerate() {
        if let Ok(text) = page.text() {
            for line in text.all().split("\n") {
                if let Some(cap_ns) = re_ns.captures(&line) {
                    let (from, to) = extract_stations(&cap_ns["from_to"], Provider::NS);
                    ns_trips.push(Trip::new(
                        NaiveDate::parse_from_str(&cap_ns["date"], "%d-%m-%Y")?,
                        Provider::NS,
                        from,
                        to,
                        parse_price(&cap_ns["price"]).unwrap(),
                    ));
                }
                if let Some(cap_gvb) = re_gvb.captures(&line) {
                    let (from, to) = extract_stations(&cap_gvb["from_to"], Provider::GVB);
                    if !to.is_empty() && !from.is_empty() {
                        gvb_trips.push(Trip::new(
                            NaiveDate::parse_from_str(&cap_gvb["date"], "%d-%m-%Y")?,
                            Provider::GVB,
                            from,
                            to,
                            parse_price(&cap_gvb["price"]).unwrap(),
                        ));
                    }
                }
            }
        }
    }

    Ok((ns_trips, gvb_trips))
}

fn parse_price(s: &str) -> Option<f32> {
    let normalized = s.replace(",", ".");
    normalized.parse::<f32>().ok()
}

fn extract_stations(s: &str, provider: Provider) -> (String, String) {
    let (mut start, mut end) = (String::new(), String::new());
    let station_list: &'static [&'static str] = match provider {
        Provider::NS => &ALL_NS_STATIONS[..],
        Provider::GVB => &ALL_GVB_STATIONS[..],
    };
    for station in station_list {
        if s.starts_with(station) {
            start = station.to_string();
        }
        if s.ends_with(station) {
            end = station.to_string();
        }
    }
    (start, end)
}
