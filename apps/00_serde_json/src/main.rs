use serde::Deserialize;
use serde_json::from_str;

const BABEL: &str = r#"{
  "name": "Enoshima Sea Candle",
  "height": 60
}
"#;

const BABEL_INCOMPATIBLE: &str = r#"{
  "name": "Enoshima Sea Candle",
  "h": 60
}
"#;

const BABEL_BROKEN: &str = r#"{
  "name": "Enoshima Sea Candle",
  "height": 60
"#;

#[derive(Deserialize)]
struct Sample {
    name: String,
    height: i32,
}

fn main() {
    // Deserialize to "struct Sample" from &str
    let json_sample: Result<Sample, serde_json::Error> = from_str::<Sample>(BABEL);
    match json_sample {
        Ok(o) => {
            println!("Success to perse BABEL");
            println!("    {} (height: {}m)", o.name, o.height);
        }
        Err(e) => {
            println!("Failed to perse BABEL");
            println!("    Error: {}", e);
        }
    }

    // Deserialize to "struct Sample" from &str
    // The string has no field comment
    let json_incompatible: Result<Sample, serde_json::Error> =
        from_str::<Sample>(BABEL_INCOMPATIBLE);
    if let Err(value) = json_incompatible {
        println!("Failed to perse BABEL_INCOMPATIBLE");
        println!("    Error: {}", value);
    }

    // Deserialize to "struct Sample" from &str
    // The string lacks "}"
    let json_broken: Result<Sample, serde_json::Error> = from_str::<Sample>(BABEL_BROKEN);
    if let Err(value) = json_broken {
        println!("Failed to perse BABEL_BROKEN");
        println!("    Error: {}", value);
    }
}
