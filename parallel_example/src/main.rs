use rayon::prelude::*;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    time::Duration,
};

#[derive(Debug)]
struct Coordenate {
    lat: f64,
    lon: f64,
}
impl FromStr for Coordenate {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Some(input_rest) = input.strip_prefix("lat:") else {
            return Err("Does not start with `lat:`");
        };

        let Some(lon_pos) = input_rest.find(",lon:") else {
            return Err("Does not contain `,lon:`");
        };

        let lat_str = &input_rest[..lon_pos];
        let lat = lat_str
            .parse::<f64>()
            .map_err(|_| "Latitude is not a valid f64")?;

        let lon_str = &input_rest[lon_pos + ",lon:".len()..];
        let lon = lon_str
            .parse::<f64>()
            .map_err(|_| "Longitude is not a valid f64")?;

        Ok(Self { lat, lon })
    }
}

fn main() {
    let numbers = [1, 5, 3, 20, 8, 100, 4];

    let _processed_numbers: Vec<u32> = numbers
        .into_par_iter()
        .map(|num| number_processsor(num))
        .collect();

    let mut result: Vec<Coordenate> = vec![];

    // open and read file
    let file = File::open("coordenates.txt").unwrap();

    // iterate over the lines
    let file_buffer = BufReader::new(file);
    for line in file_buffer.lines() {
        let line = line.unwrap();

        // per line, parse the line
        let coordenate: Coordenate = line.parse().unwrap();

        // save the results on a vector
        result.push(coordenate);
    }

    println!("{result:?}");
    // show the vector in stdout
}

fn number_processsor(input: u32) -> u32 {
    println!("Start: {input}");
    std::thread::sleep(Duration::from_secs(1));
    println!("End: {input}");
    input + 1
}
