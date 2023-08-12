use serde_json::{from_reader, Value};
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

    // Display top level keys
    println!("Top level keys");
    if let Value::Object(map) = &json {
        for (key, _value) in map {
            println!("  - {}", key);
        }
    }
    println!("");

    // Display value
    println!("Display value");
    println!(
        "  json[\"band\"][0][\"member\"][3]: {}",
        json["band"][0]["member"][3]
    );
    println!(
        "  json[\"band\"][1][\"member\"][3] (not exist): {}",
        json["band"][1]["member"][3]
    );
    println!("");

    // Value check

    println!("Value check");
    if let Value::Null = json["band"][0]["member"][3] {
        println!("  json[\"band\"][0][\"member\"][3] has no value");
    }
    if let Value::Null = json["band"][1]["member"][3] {
        println!("  json[\"band\"][1][\"member\"][3] has no value");
    }
    println!("");

    // Display value by get

    println!("Display value by get");
    println!(
        "  json[\"band\"][0][\"member\"].get(3).unwrap(): {}",
        json["band"][0]["member"].get(3).unwrap()
    );
    println!(
        "  json[\"band\"][0].get(\"member\").unwrap()[3]: {}",
        json["band"][0].get("member").unwrap()[3]
    );
    println!("");

    Ok(())
}
