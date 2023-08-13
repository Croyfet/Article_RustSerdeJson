use serde::Deserialize;
use serde_json::from_reader;
use std::{fs::File, io::BufReader};

const FILE_PATH: &str = "../resource/babel.json";
// {
//     "name": "Enoshima Sea Candle",
//     "height": 60
// }

#[allow(dead_code)] // Not use in this script
#[derive(Deserialize, Debug)] // "Deserialize" for deserialization by serde and "Debug" for displaying in println!
struct Sample {
    name: String,
    height: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open file
    let file: Result<File, std::io::Error> = File::open(FILE_PATH);
    let file: File = match file {
        Ok(o) => o,
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    // Create a buffer to load the file at once
    let reader: BufReader<File> = BufReader::new(file);

    // Deserialize to "struct Sample" from BufReader
    let json: Result<Sample, serde_json::Error> = from_reader::<BufReader<File>, Sample>(reader);

    let json = match json {
        Ok(o) => o,
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    println!("{:?}", json);

    Ok(())
}
