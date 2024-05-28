use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug, Clone)]
struct Cell {
    oem: Option<String>,
    model: Option<String>,
    launch_announced: Option<i32>,
    launch_status: Option<String>,
    body_dimensions: Option<String>,
    body_weight: Option<f64>,
    body_sim: Option<String>,
    display_type: Option<String>,
    display_size: Option<f64>,
    display_resolution: Option<String>,
    features_sensors: Option<String>,
    platform_os: Option<String>,
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.oem.hash(state);
        self.model.hash(state);
        self.launch_announced.hash(state);
        self.launch_status.hash(state);
        self.body_dimensions.hash(state);
        self.body_weight.map(|w| w.to_bits()).hash(state);
        self.body_sim.hash(state);
        self.display_type.hash(state);
        self.display_size.map(|s| s.to_bits()).hash(state);
        self.display_resolution.hash(state);
        self.features_sensors.hash(state);
        self.platform_os.hash(state);
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.oem == other.oem &&
        self.model == other.model &&
        self.launch_announced == other.launch_announced &&
        self.launch_status == other.launch_status &&
        self.body_dimensions == other.body_dimensions &&
        self.body_weight.map(|x| x.to_bits()) == other.body_weight.map(|x| x.to_bits()) &&
        self.body_sim == other.body_sim &&
        self.display_type == other.display_type &&
        self.display_size.map(|x| x.to_bits()) == other.display_size.map(|x| x.to_bits()) &&
        self.display_resolution == other.display_resolution &&
        self.features_sensors == other.features_sensors &&
        self.platform_os == other.platform_os
    }
}

impl Eq for Cell {}

impl Cell {
    fn from_csv_row(parts: Vec<&str>) -> Cell {
        let oem = if parts[0].is_empty() || parts[0] == "-" { None } else { Some(parts[0].to_string()) };
        let model = if parts[1].is_empty() || parts[1] == "-" { None } else { Some(parts[1].to_string()) };
        let launch_announced = Cell::extract_year(parts[2]);
        let launch_status = if parts[3].is_empty() || parts[3] == "-" { None } else { Some(parts[3].to_string()) };
        let body_dimensions = if parts[4].is_empty() || parts[4] == "-" { None } else { Some(parts[4].to_string()) };
        let body_weight = Cell::extract_weight(parts[5]);
        let body_sim = if parts[6].is_empty() || parts[6] == "No" || parts[6] == "-" { None } else { Some(parts[6].to_string()) };
        let display_type = if parts[7].is_empty() || parts[7] == "-" { None } else { Some(parts[7].to_string()) };
        let display_size = Cell::extract_display_size(parts[8]);
        let display_resolution = if parts[9].is_empty() || parts[9] == "-" { None } else { Some(parts[9].to_string()) };
        let features_sensors = if parts[10].is_empty() || parts[10] == "-" { None } else { Some(parts[10].to_string()) };
        let platform_os = Cell::extract_os(parts[11]);

        Cell {
            oem,
            model,
            launch_announced,
            launch_status,
            body_dimensions,
            body_weight,
            body_sim,
            display_type,
            display_size,
            display_resolution,
            features_sensors,
            platform_os,
        }
    }

    fn extract_year(year_str: &str) -> Option<i32> {
        let re = Regex::new(r"\b(\d{4})\b").unwrap();
        if let Some(caps) = re.captures(year_str) {
            return caps.get(1).map(|m| m.as_str().parse::<i32>().unwrap());
        }
        None
    }

    fn extract_weight(weight_str: &str) -> Option<f64> {
        let re = Regex::new(r"(\d+) g").unwrap();
        if let Some(caps) = re.captures(weight_str) {
            return caps.get(1).map(|m| m.as_str().parse::<f64>().unwrap());
        }
        None
    }

    fn extract_display_size(size_str: &str) -> Option<f64> {
        let re = Regex::new(r"(\d+(\.\d+)?) inches").unwrap();
        if let Some(caps) = re.captures(size_str) {
            return caps.get(1).map(|m| m.as_str().parse::<f64>().unwrap());
        }
        None
    }

    fn extract_os(os_str: &str) -> Option<String> {
        let re = Regex::new(r"^[^,]+").unwrap();
        if let Some(caps) = re.captures(os_str) {
            return caps.get(0).map(|m| m.as_str().to_string());
        }
        None
    }
}

fn read_data(filename: &str) -> Vec<Cell> {
    let mut cells = Vec::new();
    let mut seen_cells = HashSet::new();

    if let Ok(file) = File::open(filename) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(',').collect();
                let cell = Cell::from_csv_row(parts);

                if seen_cells.insert(cell.clone()) {
                    cells.push(cell);
                }
            }
        }
    }

    cells
}

fn main() {
    let filename = "cells.csv";
    let cells = read_data(filename);

    // Call methods to answer the questions
    highest_avg_weight(&cells);
    announced_and_released_diff_year(&cells);
    single_feature_sensors(&cells);
    most_launched_year(&cells);
}

fn highest_avg_weight(cells: &[Cell]) {
    let mut weights: HashMap<String, Vec<f64>> = HashMap::new();

    for cell in cells {
        if let (Some(oem), Some(weight)) = (&cell.oem, cell.body_weight) {
            weights.entry(oem.clone()).or_insert_with(Vec::new).push(weight);
        }
    }

    let mut highest_avg = 0.0;
    let mut highest_oem = String::new();

    for (oem, weights) in weights {
        let avg_weight: f64 = weights.iter().sum::<f64>() / weights.len() as f64;
        if avg_weight > highest_avg {
            highest_avg = avg_weight;
            highest_oem = oem;
        }
    }

    println!("Company with highest average weight: {} with average weight {}", highest_oem, highest_avg);
}

fn announced_and_released_diff_year(cells: &[Cell]) {
    for cell in cells {
        if let (Some(announced), Some(launch_status)) = (cell.launch_announced, &cell.launch_status) {
            if launch_status.contains("Released") {
                let re = Regex::new(r"Released (\d{4})").unwrap();
                if let Some(caps) = re.captures(launch_status) {
                    let released_year = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    if announced != released_year {
                        if let (Some(oem), Some(model)) = (&cell.oem, &cell.model) {
                            println!("{} {} was announced in {} and released in {}", oem, model, announced, released_year);
                        }
                    }
                }
            }
        }
    }
}

fn single_feature_sensors(cells: &[Cell]) {
    let count = cells.iter().filter(|cell| {
        cell.features_sensors.as_ref().map_or(false, |sensors| sensors.split(',').count() == 1)
    }).count();

    println!("Number of phones with only one feature sensor: {}", count);
}

fn most_launched_year(cells: &[Cell]) {
    let mut year_count: HashMap<i32, usize> = HashMap::new();

    for cell in cells {
        if let Some(announced) = cell.launch_announced {
            if announced > 1999 {
                *year_count.entry(announced).or_insert(0) += 1;
            }
        }
    }

    if let Some((&year, &count)) = year_count.iter().max_by_key(|&(_, count)| count) {
        println!("Year with the most phones launched after 1999: {} with {} launches", year, count);
    }
}
