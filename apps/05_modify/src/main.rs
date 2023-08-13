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
    let mut json: Value = match json {
        Ok(o) => o,
        Err(e) => {
            println!("Failed to parse json.");
            return Err(Box::new(e));
        }
    };

    // Display json
    println!("{}", json);
    println!("");

    let target_member: &Value = &json["band"][0]["member"][3];
    if target_member.is_null() {
        return Err("Failed to get member".into());
    }

    let target_name: Option<&mut Value> = json["band"][0]["member"][3].get_mut("name");
    let target_name: &mut Value = match target_name {
        Some(s) => s,
        None => {
            return Err("Failed to get name".into());
        }
    };

    // Display new name
    let new_name: String = String::from("Kita");
    println!("\"Kita-ikuyo-tte-ahoka-i\"");
    println!("Fix : {} -> {}", target_name, new_name);

    // Modify name
    *target_name = Value::String(new_name);

    // Display json
    println!("{}", json);
    println!("");

    // Display new name 2
    let new_name_2: String = String::from("Hitori");
    println!("\"Goto sa...\"");
    println!("Fix : {} -> {}", json["band"][0]["member"][2]["name"], new_name_2);

    // Modify name
    json["band"][0]["member"][2]["name"] = Value::String(new_name_2);

    // Display json
    println!("{}", json);
    println!("");

    // Insert
    println!("Insert instrument");
    let instrument = "Guiter".to_string();

    json["band"][0]["member"][2]["instrument"] = Value::String(instrument.clone());

    let target_member: Option<&mut serde_json::Map<String, Value>> = json["band"][0]["member"][3].as_object_mut();
    if let Some(s) = target_member {
        s.insert(String::from("instrument"), instrument.clone().into());
    }

    // Display json
    println!("{}", json);

    Ok(())
}
