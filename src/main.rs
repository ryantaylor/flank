// main.rs
#[macro_use]
extern crate log4rs;
extern crate rustc_serialize;
extern crate vault;
extern crate getopts;

use std::default::Default;
use std::env;
use std::path::Path;

use rustc_serialize::json;
use getopts::Options;

use vault::replay::Replay;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("l", "log", "enable logging to stdout");

    let matches = match opts.parse(&args[1..]) {
        Ok(mtch) => mtch,
        Err(err) => panic!(err.to_string()),
    };

    if matches.opt_present("l") {
        log4rs::init_file("log.toml", Default::default()).unwrap();
    }

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        return;
    };

    // Create a path to the desired file
    let path = Path::new(&input);

    let mut replay = Replay::new(&path);
    replay.parse();

    let encoded = json::encode(&replay).unwrap();
    //let encoded = replay.to_json();
    println!("{}", encoded);
}