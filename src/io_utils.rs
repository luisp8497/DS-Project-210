// io_utils.rs
// Handles reading team data from a CSV and writing analysis results to a file.

use std::fs::File;
use std::io::Write;
use csv::Reader;
use crate::models::Team;

/// Reads a CSV and returns a vector of Team structs.
/// Filters out rows with missing names or all-zero stats.
pub fn read_csv(path: &str) -> Vec<Team> {
    let mut rdr = Reader::from_path(path).expect("CSV file not found");
    let headers = rdr.headers().unwrap().clone();
    let name_col = headers.iter().position(|h| h == "Full Team Name").expect("'Full Team Name' column not found");
    let season_col = headers.iter().position(|h| h == "Season").expect("'Season' column not found");

    // Filter columns that are not name/season/seed to get stats
    let stat_cols: Vec<_> = headers.iter().enumerate()
        .filter(|&(i, h)| i != name_col && i != season_col && h != "Seed")
        .map(|(i, _)| i)
        .collect();

    let mut teams = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let name = record.get(name_col).unwrap_or("").to_string();
        let season = record.get(season_col).unwrap_or("").to_string();

        // Parse stats from the remaining columns
        let stats: Vec<f64> = stat_cols.iter()
            .map(|&i| record.get(i).unwrap_or("0").parse::<f64>().unwrap_or(0.0))
            .collect();

        if !name.is_empty() && stats.iter().any(|&x| x != 0.0) {
            teams.push(Team {
                name: format!("{} ({})", name, season),
                season,
                stats
            });
        }
    }

    teams
}

/// Writes string content to a file at the specified path.
pub fn write_to_file(path: &str, content: &str) {
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(content.as_bytes()).expect("Unable to write data");
}
