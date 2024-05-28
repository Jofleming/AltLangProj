use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::path::Path;
use std::str::FromStr;
use regex::Regex;
use ordered_float::OrderedFloat;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Cell {
    oem: Option<String>,
    model: Option<String>,
    launch_announced: Option<i32>,
    launch_status: Option<String>,
    body_dimensions: Option<String>,
    body_weight: Option<OrderedFloat<f64>>,
    body_sim: Option<String>,
    display_type: Option<String>,
    display_size: Option<OrderedFloat<f64>>,
    display_resolution: Option<String>,
    features_sensors: Option<String>,
    platform_os: Option<String>,
}

impl Cell {
    fn new(
        oem: Option<String>, model: Option<String>, launch_announced: Option<i32>, launch_status: Option<String>,
        body_dimensions: Option<String>, body_weight: Option<OrderedFloat<f64>>, body_sim: Option<String>, display_type: Option<String>,
        display_size: Option<OrderedFloat<f64>>, display_resolution: Option<String>, features_sensors: Option<String>, platform_os: Option<String>
    ) -> Cell {
        Cell {
            oem, model, launch_announced, launch_status, body_dimensions, body_weight, body_sim,
            display_type, display_size, display_resolution, features_sensors, platform_os,
        }
    }

    fn from_csv_row(parts: Vec<&str>) -> Cell {
        let oem = if parts[0].is_empty() { None } else { Some(parts[0].to_string()) };
        let model = if parts[1].is_empty() { None } else { Some(parts[1].to_string()) };
        let launch_announced = Cell::extract_year(parts[2]);
        let launch_status = if parts[3].is_empty() { None } else { Some(parts[3].to_string()) };
        let body_dimensions = if parts[4].is_empty() { None } else { Some(parts[4].to_string()) };
        let body_weight = Cell::extract_weight(parts[5]);
        let body_sim = if parts[6].is_empty() { None } else { Some(parts[6].to_string()) };
        let display_type = if parts[7].is_empty() { None } else { Some(parts[7].to_string()) };
        let display_size = Cell::extract_display_size(parts[8]);
        let display_resolution = if parts[9].is_empty() { None } else { Some(parts[9].to_string()) };
        let features_sensors = if parts[10].is_empty() { None } else { Some(parts[10].to_string()) };
        let platform_os = if parts[11].is_empty() { None } else { Some(parts[11].to_string()) };

        Cell::new(oem, model, launch_announced, launch_status, body_dimensions, body_weight, body_sim,
            display_type, display_size, display_resolution, features_sensors, platform_os)
    }

    fn extract_year(year_str: &str) -> Option<i32> {
        let re = Regex::new(r"\b(\d{4})\b").unwrap();
        if let Some(cap) = re.captures(year_str) {
            return cap.get(1).map(|m| i32::from_str(m.as_str()).unwrap());
        }
        None
    }

    fn extract_weight(weight_str: &str) -> Option<OrderedFloat<f64>> {
        let re = Regex::new(r"(\d+) g.*").unwrap();
        if let Some(cap) = re.captures(weight_str) {
            return cap.get(1).map(|m| OrderedFloat(f64::from_str(m.as_str()).unwrap()));
        }
        None
    }

    fn extract_display_size(size_str: &str) -> Option<OrderedFloat<f64>> {
        let re = Regex::new(r"(\d+(\.\d+)?) inches.*").unwrap();
        if let Some(cap) = re.captures(size_str) {
            return cap.get(1).map(|m| OrderedFloat(f64::from_str(m.as_str()).unwrap()));
        }
        None
    }
}

fn read_data<P: AsRef<Path>>(filename: P) -> Result<Vec<Cell>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut cells = Vec::new();
    let mut seen_cells = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        let cell = Cell::from_csv_row(parts);

        if seen_cells.insert(cell.clone()) {
            cells.push(cell);
        }
    }

    Ok(cells)
}

fn main() -> Result<()> {
    let filename = "cells.csv";
    let cells = read_data(filename)?;

    for cell in cells {
        println!("{:?}", cell);
    }

    Ok(())
}
