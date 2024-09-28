use std::collections::{HashMap, HashSet};
use std::env::{args, Args};
use std::io::Error;
use std::thread::{spawn, JoinHandle};

use crate::devices::{scan_devices, InputDevice};
use crate::log::Writer;

const HELP_MESSAGE: &str = "running the program without any arguments prints all the input devices found in
'/proc/bus/input/devices' then exits

\x1b[1;38;2;184;239;184m-q\x1b[0m                   Query for some pattern amongst the available devices' names

\x1b[1;38;2;184;239;184m-b\x1b[0m                   Bind a reader to an event<n> file and start listening
                     If neither -r nor -d is given then -d is automatically used

\x1b[1;38;2;184;239;184m-r\x1b[0m                   Reader outputs raw bytes data
\x1b[1;38;2;184;239;184m-d\x1b[0m                   Reader outputs decoded input_event struct data

\x1b[1;38;2;184;239;184m-t\x1b[0m                   Print the read data to the terminal stdout
\x1b[1;38;2;184;239;184m-l, -lc <file>\x1b[0m       Log the read data to an output log file, appends to file 
\x1b[1;38;2;184;239;184m-lo <file>\x1b[0m           Log the read data to an output log file, overwrites to file 

\x1b[1;38;2;184;239;184m-h\x1b[0m                   Print this help message";

#[derive(Hash, PartialEq, Eq)]
struct Entry {
    args: u8,
    pats: Vec<String>,
}

pub(crate) fn parse_args() -> HashSet<Entry> {
    let mut args = args();
    args.next();
    let mut entries = HashSet::new();
    // parse_entry(&mut args) != Err(Error::other("no args to process"))
    while args.len() > 0 {
        if let Ok(entry) = parse_entry(&mut args) {
            entries.insert(entry);
        }
    }

    entries
}

// FIXME: query should not take unrelated new arg, or should return it
// FIXME: bind should also farm pats
fn parse_entry(args: &mut Args) -> Result<Entry, Error> {
    // if args.len() == 0 {
    //     return Err(Error::other("no args to process"));
    // }
    let mut a = 0u8;
    match args.next().unwrap().trim() {
        "-h" => {
            a = 1;
            return Ok(Entry {
                args: a,
                pats: vec![],
            });
        }

        "-q" => {
            a = 2;
            let mut pats = vec![];
            while let Some(arg) = args.next() {
                if arg.starts_with('-') {
                    return Err(Error::other(
                        "bad args, query doesn't take any additional arguments",
                    ));
                }
                pats.push(arg);
            }
            return Ok(Entry { args: a, pats });
        }

        "-b" => {
            a = 4;
            let mut pats = vec![];
            while let Some(arg) = args.next() {
                if arg.starts_with('-') {
                    match arg.trim() {
                        "-r" | "--raw" => a |= 1,                 // 5
                        "-e" | "--events" => a |= 2,              // 6
                        "-t" | "--terminal" => a |= 8,            // 12
                        "-l" | "--logfile" => a |= 9,             // 13
                        "-lc" | "--logfile-continue" => a |= 16,  // 20
                        "-lo" | "--logfile-overwrite" => a |= 17, // 21
                        "-ld" | "--logfile-with-date" => a |= 18, // 22
                        _ => return Err(Error::other("bad args")),
                    }
                }
                pats.push(arg);
            }
            return Ok(Entry { pats, args: a });
        }
        _ => Err(Error::other("another arg, another error")),
    }
}

pub(crate) fn logger(args: u8, devices: HashSet<InputDevice>) -> JoinHandle<()> {
    match args {
        4 => {
            let mut writer = Writer::Stdout(std::io::stdout().lock());
        }
    }
}

/// parses the fields of this instance of the program builder
/// consuming it and running the program
pub(crate) fn run(entries: HashSet<Entry>) -> Option<Vec<JoinHandle<()>>> {
    let devices = parse_devices();
    if entries.is_empty() {
        // no args, print all devices case
        let mut writer = Writer::Stdout(std::io::stdout().lock());
        writer.write(format!("{:#?}", devices).as_bytes());

        return None;
    } else if entries.iter().find(|e| e.args == 1).is_some() {
        // help case
        let mut writer = Writer::Stdout(std::io::stdout().lock());
        writer.write(format!("{}", HELP_MESSAGE).as_bytes());

        return None;
    }

    match entries.iter().find(|e| e.args == 2).is_some() {
        true => {
            let devices = query_devices(entries, devices);
            let mut writer = Writer::Stdout(std::io::stdout().lock());
            writer.write(format!("queried input devices: {:#?}", devices).as_bytes());

            return None;
        }
        false => {
            let mut handles = vec![];
            let mut map = query_devices_for_bind(entries, devices).into_iter();
            while let Some((args, devices)) = map.next() {
                logger(args, devices);
            }

            return Some(handles);
        }
    }
}
