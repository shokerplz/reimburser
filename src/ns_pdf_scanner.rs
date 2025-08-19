use std::{f32, path::PathBuf};

use crate::data::Trip;
use anyhow::Result;

use chrono::NaiveDate;
use regex::Regex;

use pdfium_render::prelude::*;

pub fn ns_pdf_scanner(pdf: PathBuf) -> Result<(Vec<Trip>, Vec<Trip>)> {
    let pdfium = Pdfium::default();

    let re_ns = Regex::new(
        r"^(?P<date>\d{2}-\d{2}-\d{4})\s+NS\s+(?P<kenmerk>40%\s+korting\s+buiten\s+de\s+spits|Vol\s+tarief\s+in\s+de\s+spits|40%\s+korting\s+in\s+het\s+weekend)\s+(?P<from>.+?)\s+(?P<to>.+?)\s+(?P<class>\d+)\s+€\s*(?P<price>[\d\.,]+)\s*$",
    )?;
    let re_gvb = Regex::new(
        r"^(?P<date>\d{2}-\d{2}-\d{4})\s+GVB\s+Lijn(?:\s+\d+)?\s+(?P<from>.+?)\s+(?P<to>.+?)\s+€\s*(?P<price>[\d\.,]+)\s*$",
    )?;

    let doc = pdfium.load_pdf_from_file(&pdf, None)?;

    let gvb_trips: Vec<Trip> = Vec::new();
    let mut ns_trips: Vec<Trip> = Vec::new();

    for (i, page) in doc.pages().iter().enumerate() {
        if let Ok(text) = page.text() {
            for line in text.all().split("\n") {
                if let Some(cap_ns) = re_ns.captures(&line) {
                    ns_trips.push(Trip::new(
                        NaiveDate::parse_from_str(&cap_ns["date"], "%d-%m-%Y")?,
                        "NS".to_string(),
                        cap_ns["from"].trim().to_string(),
                        cap_ns["to"].trim().to_string(),
                        parse_price(&cap_ns["price"]).unwrap(),
                    ));
                }
            }
        }
    }

    println!("{:?}", ns_trips);

    let ns_result: Vec<Trip> = Vec::new();
    let gvb_result: Vec<Trip> = Vec::new();
    Ok((ns_result, gvb_result))
}

fn parse_price(s: &str) -> Option<f32> {
    let normalized = s.replace(",", ".");
    normalized.parse::<f32>().ok()
}
