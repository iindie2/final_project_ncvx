use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

#[derive(Debug)]
struct CleanRecord {
    age: u32,
    sex: u32,
    income: u32,
    reported: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let records = read_records("data/final_project_data.csv")?;
    println!("✅ Total clean rows: {}", records.len());

    Ok(())
}

fn read_records(filename: &str) -> Result<Vec<CleanRecord>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut records = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue; // skip header
        }
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 4 {
            continue;
        }
        let record = CleanRecord {
            age: fields[0].parse().unwrap_or(0),
            sex: fields[1].parse().unwrap_or(0),
            income: fields[2].parse().unwrap_or(0),
            reported: fields[3].parse().unwrap_or(0),
        };
        records.push(record);
    }

    Ok(records)
}
