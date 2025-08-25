use chrono::{Datelike, NaiveDate};

use crate::data;

enum Direction {
    ToWork,
    ToHome,
    None,
}

// There's no single reason to test this imho
pub fn trip_workday_filter(all_trips: Vec<data::Trip>) -> Vec<data::Trip> {
    let mut result: Vec<data::Trip> = Vec::new();
    for trip in all_trips {
        if trip.date.weekday().number_from_monday() <= 5 {
            result.push(trip);
        }
    }
    result
}

// Input trips should be sorted by date. Exactly like in PDF from NS
pub fn trip_station_filter(
    all_trips: Vec<data::Trip>,
    from: Vec<String>,
    to: Vec<String>,
) -> Vec<data::Trip> {
    let mut result: Vec<data::Trip> = Vec::new();
    let mut tmp_trips: Vec<data::Trip> = Vec::new();
    let mut in_chain = false;
    let mut direction = Direction::None;
    let mut current_date: chrono::NaiveDate = NaiveDate::from_yo_opt(2025, 1).unwrap();
    let mut index = 0;
    while index < all_trips.len() {
        let trip = &all_trips[index];
        if trip.price == 0.0 {
            index += 1;
            continue;
        }
        if !in_chain {
            current_date = trip.date;
            if from.contains(&trip.from) && to.contains(&trip.to)
                || from.contains(&trip.to) && to.contains(&trip.from)
            {
                direction = Direction::None;
                in_chain = false;
                result.push(trip.clone());
            } else if from.contains(&trip.from) {
                direction = Direction::ToWork;
                tmp_trips.push(trip.clone());
                in_chain = true;
            } else if to.contains(&trip.from) {
                direction = Direction::ToHome;
                tmp_trips.push(trip.clone());
                in_chain = true;
            }
        } else {
            if trip.date != current_date || tmp_trips.last().expect("Well, I don't know how tmp_trips can be empty here, but you achieved unachivable goal, congrats!").to != trip.from {
                tmp_trips.clear();
                in_chain = false;
                continue;
            }
            tmp_trips.push(trip.clone());
            match direction {
                Direction::ToHome if from.contains(&trip.to) => {
                    result.append(&mut tmp_trips);
                    in_chain = false;
                    direction = Direction::None;
                }
                Direction::ToWork if to.contains(&trip.to) => {
                    result.append(&mut tmp_trips);
                    in_chain = false;
                    direction = Direction::None;
                }
                _ => {}
            }
        }
        index += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trip_station_filter_simple() {
        let date = chrono::NaiveDate::from_ymd_opt(2025, 6, 24).unwrap();

        let all_trips = vec![
            data::Trip::new(
                date,
                Provider::NS,
                "Hilversum".into(),
                "Amsterdam Centraal".into(),
                5.0,
            ), // match
            data::Trip::new(
                date,
                Provider::NS,
                "Amsterdam Zuid".into(),
                "Hilversum".into(),
                5.0,
            ), // match
            data::Trip::new(
                date,
                Provider::NS,
                "Hilversum".into(),
                "Utrecht Centraal".into(),
                5.0,
            ), // no match
            data::Trip::new(
                date,
                Provider::NS,
                "Rotterdam".into(),
                "Hilversum".into(),
                5.0,
            ), // no match
        ];

        let from_stations = vec!["Hilversum".into()];
        let to_stations = vec!["Amsterdam Centraal".into(), "Amsterdam Zuid".into()];

        let filtered = trip_station_filter(all_trips, from_stations, to_stations);

        assert_eq!(filtered.len(), 2);
        assert!(
            filtered
                .iter()
                .any(|t| t.from == "Hilversum" && t.to == "Amsterdam Centraal")
        );
        assert!(
            filtered
                .iter()
                .any(|t| t.from == "Amsterdam Zuid" && t.to == "Hilversum")
        );
    }

    #[test]
    fn test_trip_filter_multi_leg() {
        let date = chrono::NaiveDate::from_ymd_opt(2025, 6, 24).unwrap();

        let all_trips = vec![
            data::Trip::new(
                date,
                Provider::NS,
                "Hilversum".into(),
                "Duivendrecht".into(),
                3.0,
            ), // match
            data::Trip::new(
                date,
                Provider::NS,
                "Duivendrecht".into(),
                "Amsterdam Centraal".into(),
                4.0,
            ), // match
            data::Trip::new(
                date,
                Provider::NS,
                "Amsterdam Centraal".into(),
                "Duivendrecht".into(),
                4.0,
            ), // match
            data::Trip::new(
                date,
                Provider::NS,
                "Duivendrecht".into(),
                "Hilversum".into(),
                3.0,
            ), // match
            data::Trip::new(
                date,
                Provider::NS,
                "Utrecht Centraal".into(),
                "Rotterdam".into(),
                10.0,
            ), // no match
        ];

        let from_stations = vec!["Hilversum".into()];
        let to_stations = vec!["Amsterdam Centraal".into()];

        let filtered = trip_station_filter(all_trips, from_stations, to_stations);

        assert_eq!(filtered.len(), 4);

        assert!(
            filtered
                .iter()
                .any(|t| t.from == "Hilversum" && t.to == "Duivendrecht")
        );
        assert!(
            filtered
                .iter()
                .any(|t| t.from == "Duivendrecht" && t.to == "Amsterdam Centraal")
        );

        assert!(
            filtered
                .iter()
                .any(|t| t.from == "Amsterdam Centraal" && t.to == "Duivendrecht")
        );
        assert!(
            filtered
                .iter()
                .any(|t| t.from == "Duivendrecht" && t.to == "Hilversum")
        );
    }
}
