// main.rs

#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate clap;
//extern crate getopts;
#[macro_use]
extern crate log4rs;
#[macro_use]
//extern crate nom;
//extern crate postgres;
extern crate rustc_serialize;
extern crate vault;

use std::collections::HashMap;
//use std::default::Default;
use std::env;
//use std::fs::File;
//use std::io::Read;
use std::path::Path;
//use std::str::{FromStr, from_utf8_unchecked};

//use getopts::Options;
//use nom::{digit, eof, not_line_ending, space};
//use nom::IResult::*;
//use postgres::{Connection, SslMode};
use rustc_serialize::json;

//use vault::Vault;
use vault::{Command, Config};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();

    // let mut opts = Options::new();
    // opts.optopt("c", "cmdout", "write command info to stdout", "PLAYERID");
    // opts.optopt("d", "db", "update blueprints in database", "BLUEPRINTS");
    // opts.optflag("n", "nocmd", "cleans command structures before writing output");
    // opts.optflag("w", "wipecmd", "parses commands but wipes them before display");
    // opts.optflag("p", "cpm", "write command per minute information to stdout");
    // opts.optflag("l", "log", "enable logging to stdout");
    // opts.optflag("a", "array", "output replays as array inside wrapper JSON object");
    // opts.optflag("s", "strict", "reject input without a valid file extension");
    // opts.optflag("v", "version", "print version information");
    // opts.optflag("h", "help", "print this help menu");

    let matches = clap_app!(flank =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: "Ryan Taylor <ryan.taylor9@gmail.com>")
        (about: "CLI CoH2 replay parser")
        // args
        (@arg REPLAY: +required "Replay file to be parsed")
        // flags
        (@arg NOCMD: -n --nocmd "Skips command parsing")
        (@arg WIPECMD: -w --wipecmd "Parses command info but wipes them before output")
        (@arg STRICT: -s --strict "Rejects files without the .rec extension")
        (@arg LOG: -l --log "Enables debug logging to stdout")
        // options
        (@arg PLAYERID: -c --cmdout +takes_value "Writes command info to stdout")
    ).get_matches();

    let strict = matches.is_present("STRICT");
    let commands = !matches.is_present("NOCMD");
    let command_bytes = matches.is_present("PLAYERID");
    let config = Config::new(strict, commands, command_bytes, true);

    // try to enable logging
    if matches.is_present("LOG") {
        match env::home_dir() {
            Some(val) => {
                let mut home_dir = val;
                home_dir.push(".flank/log.toml");
                log4rs::init_file(home_dir.as_path(), Default::default()).unwrap();
            },
            None => println!("couldn't get path to log config")
        }
    }

    // we can unwrap here because REPLAY is required
    let file = matches.value_of("REPLAY").unwrap();
    let path = Path::new(&file);
    let mut replay = match vault::parse_replay(&path, Some(config)) {
        Ok(rep) => rep,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    // wipe commands if requested
    if matches.is_present("WIPECMD") {
        replay.commands = HashMap::new();
    }

    if let Some(pid) = matches.value_of("PLAYERID") {
        match pid {
            "all" => {
                for (_, commands) in &replay.commands {
                    for command in commands {
                        display_command(command);
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

                if let Some(commands) = replay.commands.get(&id) {
                    for command in commands {
                        display_command(command);
                    }
                }
            }
        }
    } else {
        println!("{}", json::encode(&replay).unwrap());
    }
    // } else {
    //     println!("{}", json::encode(&replay).unwrap());
    // }

    // let matches = match opts.parse(&args[1..]) {
    //     Ok(mtch) => mtch,
    //     Err(err) => panic!(err.to_string()),
    // };

    // if matches.opt_present("v") {
    //     print_version();
    //     return;
    // }

    // if matches.opt_present("h") {
    //     print_usage(&program, opts);
    //     return;
    // }

    // if let Some(blueprints) = matches.opt_str("d") {
    //     parse_blueprints(&blueprints);
    //     return;
    // }

    // let strict = matches.opt_present("s");
    // let commands = !matches.opt_present("n");
    // let command_bytes = matches.opt_present("c");

    // if matches.opt_present("l") {
    //     match env::home_dir() {
    //         Some(val) => {
    //             let mut home_dir = val;
    //             home_dir.push(".flank/log.toml");
    //             log4rs::init_file(home_dir.as_path(), Default::default()).unwrap();
    //         },
    //         None => println!("couldn't get path to log config")
    //     }
        
    // }

    // let input = if !matches.free.is_empty() {
    //     matches.free[0].clone()
    // } else {
    //     print_usage(&program, opts);
    //     return;
    // };

    // // Create a path to the desired file
    // let path = Path::new(&input);
    // let config = Config::new(strict, commands, command_bytes);
    // let mut results = match Vault::parse_with_config(&path, config) {
    //     Ok(vault) => vault,
    //     Err(err) => {
    //         println!("{}", err);
    //         return;
    //     }
    // };

    // if let Some(pid) = matches.opt_str("c") {
    //     match pid.as_ref() {
    //         "all" => {
    //             for replay in &results.replays {
    //                 for (_, commands) in &replay.commands {
    //                     for command in commands {
    //                         command.display();
    //                     }
    //                 }
    //                 // for (_, player) in &replay.players {
    //                 //     for command in &player.commands {
    //                 //         command.display();
    //                 //     }
    //                 // }
    //             }
    //         },
    //         _ => {
    //             let id = match pid.parse::<u8>() {
    //                 Ok(val) => val,
    //                 Err(err) => {
    //                     println!("{}", err);
    //                     return;
    //                 }
    //             };

    //             for replay in &results.replays {
    //                 if let Some(commands) = replay.commands.get(&id) {
    //                     for command in commands {
    //                         command.display();
    //                     }
    //                 }
    //                 // if let Some(player) = replay.players.get(&id) {
    //                 //     for command in &player.commands {
    //                 //         command.display();
    //                 //     }
    //                 // }
    //             }
    //         }
    //     }
    // }
    // else if matches.opt_present("a") {
    //     println!("{}", results.to_json().unwrap());
    // }
    // else if matches.opt_present("p") {
    //     for replay in &results.replays {
    //         println!("ticks: {}", replay.duration);
    //         for player in &replay.players {
    //             println!("P{} - {} - cpm: {}", player.id, player.name, player.cpm);
    //         }
    //     }
    // }
    // else if matches.opt_present("w") {
    //     for replay in &mut results.replays {
    //         replay.commands = HashMap::new();
    //         println!("{}", replay.to_json().unwrap());
    //     }
    // }
    // else {
    //     for replay in &results.replays {
    //         println!("{}", replay.to_json().unwrap());
    //     }
    // }
}

fn display_command(command: &Command) {
    let mut output = String::new();
    if let Some(ref bytes) = command.bytes {
        output.push_str(&format!("P{}: ", command.player_id));
        for byte in bytes {
            output.push_str(&format!("{:02X} ", byte));
        }
    };

    output.push_str(&format!("{:?} {}", command.command_type, command.entity_id));
    println!("{}", output);
}

// fn print_usage(program: &str, opts: Options) {
//     let brief = format!("Usage: {} [options] FILE", program);
//     print!("{}", opts.usage(&brief));
// }

// fn print_version() {
//     println!("flank v{}", env!("CARGO_PKG_VERSION"));
//     vault::print_version();
// }

// fn parse_blueprints(blueprints: &str) {
//     let conn = Connection::connect("postgresql://ryan@localhost/cohdb", &SslMode::None).unwrap();
//     let mut entity_types: HashMap<i32, String> = HashMap::new();

//     let stmt = conn.prepare("SELECT * FROM \"ENTITY_TYPES\"").unwrap();
//     for row in stmt.query(&[]).unwrap() {
//         entity_types.insert(row.get(0), row.get(1));
//     }

//     //println!("loaded entity types...");

//     //println!("{}", blueprints);

//     let mut file = File::open(blueprints).unwrap();
//     let mut file_string = String::new();
//     file.read_to_string(&mut file_string).unwrap();

//     named!(parse_id <&[u8], i32>,
//         chain!(
//                 tag!("ID:")                     ~
//                 space?                          ~
//             id: map!(call!(digit), buf_to_i32)  ~
//                 tag!(",")                       ~
//                 space?,
//             || { id }
//         ));

//     named!(parse_type <&[u8], i32>,
//         chain!(
//                 tag!("Type:")                   ~
//                 space?                          ~
//             tp: map!(call!(digit), buf_to_i32)  ~
//                 tag!(",")                       ~
//                 space?,
//             || { tp }
//         ));

//     named!(parse_name <&[u8], String>,
//         chain!(
//                 tag!("Name:")                   ~
//                 space?                          ~
//             nm: map!(call!(not_line_ending), to_string),
//             || { nm.to_owned() }
//         ));

//     named!(eol, alt!(tag!("\r\n") | tag!("\n") | tag!("\u{2028}") | tag!("\u{2029}")));

//     named!(parse_entry <&[u8], (i32, i32, String)>,
//         chain!(
//                 id: parse_id    ~
//                 tp: parse_type  ~
//                 nm: parse_name  ~
//                     alt!(eof | eol),
//                 || {
//                     //println!("{} {} {}", id, tp, nm);
//                     (id, tp, nm)
//                 }
//             ));

//     named!(parse <&[u8], (Vec<(i32, i32, String)>)>, many0!(parse_entry));

//     let blueprint_entries = match parse(&file_string.as_bytes()[..]) {
//         Done(_, entries) => entries,
//         _ => panic!(),
//     };

//     //println!("{}", blueprint_entries.len());

//     let mut num_entities = 0;
//     let mut num_entities_relevant = 0;
//     let mut num_entities_added = 0;

//     let stmt = conn.prepare("INSERT INTO \"ENTITIES\" (\"ID\", \"TYPE\", \"NAME\") VALUES ($1, $2, $3)").unwrap();

//     for (id, tp, nm) in blueprint_entries.into_iter() {
//         num_entities += 1;
//         if let Some(type_name) = entity_types.get(&tp) {
//             num_entities_relevant += 1;
//             let inserts = match stmt.execute(&[&id, &type_name, &nm]) {
//                 Ok(num) => num,
//                 _ => 0,
//             };
//             num_entities_added += inserts;
//         };
//     }

//     println!("entries found: {}", num_entities);
//     println!("relevant entries: {}", num_entities_relevant);
//     println!("entries added: {}", num_entities_added);
// }

// fn to_string(s: &[u8]) -> &str {
//     unsafe { from_utf8_unchecked(s) }
// }

// fn to_i32(s: &str) -> i32 {
//     FromStr::from_str(s).unwrap()
// }

// fn buf_to_i32(s: &[u8]) -> i32 {
//     to_i32(to_string(s))
// }