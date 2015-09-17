// main.rs
#[macro_use]
extern crate log4rs;
extern crate rustc_serialize;
extern crate vault;

use std::default::Default;
use std::path::Path;

use rustc_serialize::json;

use vault::replay::Replay;

fn main() {
    // Initialize logging
    log4rs::init_file("log.toml", Default::default()).unwrap();

    // Create a path to the desired file
    let path = Path::new("/home/ryan/replays/city_17_4v4.rec");

    let mut replay = Replay::new(&path);
    replay.parse();

    let encoded = json::encode(&replay).unwrap();
    //let encoded = replay.to_json();
    println!("{}", encoded);
}