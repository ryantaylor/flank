// main.rs
#[macro_use]
extern crate log4rs;
extern crate rustc_serialize;
extern crate vault;
extern crate getopts;

use std::default::Default;
use std::env;
use std::path::Path;

use getopts::Options;

use vault::Vault;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("c", "cmdout", "write command info to stdout", "PLAYERID");
    opts.optflag("p", "cpm", "write command per minute information to stdout");
    opts.optflag("l", "log", "enable logging to stdout");
    opts.optflag("a", "array", "output replays as array inside wrapper JSON object");
    opts.optflag("v", "version", "print version information");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(mtch) => mtch,
        Err(err) => panic!(err.to_string()),
    };

    if matches.opt_present("v") {
        print_version();
        return;
    }

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("l") {
        match env::home_dir() {
            Some(val) => {
                let mut home_dir = val;
                home_dir.push(".flank/log.toml");
                log4rs::init_file(home_dir.as_path(), Default::default()).unwrap();
            },
            None => println!("couldn't get path to log config")
        }
        
    }

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    // Create a path to the desired file
    let path = Path::new(&input);
    let results = match Vault::parse(&path) {
        Ok(vault) => vault,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    if let Some(pid) = matches.opt_str("c") {
        match pid.as_ref() {
            "all" => {
                for replay in &results.replays {
                    for (_, player) in &replay.players {
                        for command in &player.commands {
                            command.display();
                        }
                    }
                }
            },
            _ => {
                let id = match pid.parse::<u8>() {
                    Ok(val) => val,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                for replay in &results.replays {
                    if let Some(player) = replay.players.get(&id) {
                        for command in &player.commands {
                            command.display();
                        }
                    }
                }
            }
        }
    }
    else if matches.opt_present("a") {
        println!("{}", results.to_json().unwrap());
    }
    else if matches.opt_present("p") {
        for replay in &results.replays {
            println!("ticks: {}", replay.duration);
            for (_, player) in &replay.players {
                println!("P{} - cmd: {} cpm: {}", player.id, player.commands.len(), player.cpm);
            }
        }
    }
    else {
        for replay in &results.replays {
            println!("{}", replay.to_json().unwrap());
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    println!("flank v0.1.3");
    vault::print_version();
}