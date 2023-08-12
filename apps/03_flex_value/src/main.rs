use serde_json::{Value, from_reader};
use std::{fs::File, io::BufReader};

const FILE_PATH: &str = "../resource/live_music_club.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load file
    let file: Result<File, std::io::Error> = File::open(FILE_PATH);
    let file = match file {
        Ok(o) => o,
        Err(e) => {
            println!("Failed to open file ({}).", FILE_PATH);
            return Err(Box::new(e));
        }
    };
    let reader: BufReader<File> = BufReader::new(file);

    // Parse json
    let json: Result<Value, serde_json::Error> = from_reader::<BufReader<File>, Value>(reader);
    let json = match json {
        Ok(o) => o,
        Err(e) => {
            println!("Failed to parse json.");
            return Err(Box::new(e));
        }
    };

    // Display json
    println!("{:?}", json);

    Ok(())
}
