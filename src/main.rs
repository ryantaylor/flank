use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use serde_json::Value;
use vault::Replay;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Flank {
    /// Path to a CoH3 replay file
    file: PathBuf,
    /// Clear command output if you only need summary data
    #[arg(short, long)]
    clear: bool,
}

fn main() -> Result<()> {
    let flank = Flank::parse();

    let file = File::open(flank.file)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    match Replay::from_bytes(&buffer) {
        Ok(replay) => {
            let mut json = serde_json::to_value(&replay)?;

            if flank.clear {
                clear_commands(&mut json);
            }

            Ok(println!("{}", json))
        }
        Err(_) => Ok(println!("{{\"error\":\"Parsing failed!\"}}")),
    }
}

fn clear_commands(json: &mut Value) {
    let players = json["players"].as_array_mut().unwrap();
    for player in players {
        let commands = player["commands"].as_array_mut().unwrap();
        commands.clear();
    }
}
