use serde::Deserialize;
use serde_json::from_reader;
use std::{fs::File, io::BufReader};

const FILE_PATH: &str = "../resource/band.json";

#[derive(Deserialize, Debug)]
struct Member {
    name: String,
    age: u32,
}

#[derive(Deserialize, Debug)]
struct Band {
    band_name: String,
    member: Vec<Member>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file: Result<File, std::io::Error> = File::open(FILE_PATH);
    let file = match file {
        Ok(o) => o,
        Err(e) => {
            println!("Failed to open file ({}).", FILE_PATH);
            return Err(Box::new(e));
        }
    };
    let reader: BufReader<File> = BufReader::new(file);

    let json: Result<Band, serde_json::Error> = from_reader::<BufReader<File>, Band>(reader);

    let json = match json {
        Ok(o) => o,
        Err(e) => {
            println!("Failed to parse json.");
            return Err(Box::new(e));
        }
    };

    println!("{}", json.band_name);
    println!("  member");

    for i in json.member.iter() {
        println!("  - {:<8} ({})", i.name, i.age);
    }

    Ok(())
}
