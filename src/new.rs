use std::fs::File;
use std::io::Read;
use std::thread::{spawn, JoinHandle};

use crate::devices::{scan_devices, InputDevice};
use crate::log::Writer;

const INPUT_DEVICES: &str = "/proc/bus/input/devices";

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

// if -b, --bind was found
// this handles 1 binding arguments series
fn pbind(args: &mut std::env::Args, bindings: &mut Vec<(u8, Vec<String>)>) {
    let mut opts = 0u8;
    let mut pats = vec![];
    while let Some(arg) = args.next() {
        if arg == "-e" || arg == "--event" {
            opts |= 1;
        } else if "-l" == arg || "--logfile" == arg {
            opts |= 2;
        } else if "-lc" == arg || "--logfile-continue" == arg {
            opts |= 6;
        } else if "-lo" == arg || "--logfile-overwrite" == arg {
            opts |= 10;
        } else if "-ld" == arg || "--logfile-with-date" == arg {
            opts |= 18;
        } else if "-b" == arg || "--bind" == arg {
            pbind(args, bindings);
        } else {
            pats.push(arg);
        }
    }

    bindings.push((opts, pats));
}

// if -q, --query was found
// this handles 1 query arguments series
fn pquery(args: &mut std::env::Args, queries: &mut Vec<Vec<String>>) {
    let mut pats = vec![];
    while let Some(arg) = args.next() {
        if "-q" == arg || "--query" == arg {
            pquery(args, queries);
        } else {
            pats.push(arg);
        }
    }

    queries.push(pats);
}

enum Patterns {
    Query(Vec<Vec<String>>),
    Bind(Vec<(u8, Vec<String>)>),
    None,
}

fn args() -> Patterns {
    let mut args = std::env::args();
    args.next();

    if args.len() == 0 {
        println!("{}", std::fs::read_to_string(INPUT_DEVICES).unwrap());
        std::process::exit(0);
    } else if args.len() == 1 {
        if let Some(help) = args.next() {
            if ["-h", "--help"].contains(&help.trim()) {
                println!("{}", HELP_MESSAGE);
            }
        }
        std::process::exit(0);
    } else {
        if let Some(arg) = args.next() {
            if ["-q", "--query"].contains(&arg.trim()) {
                let mut queries = vec![];
                pquery(&mut args, &mut queries);

                return Patterns::Query(queries);
            } else if ["-b", "--bind"].contains(&arg.trim()) {
                let mut bindings = vec![];
                pbind(&mut args, &mut bindings);

                return Patterns::Bind(bindings);
            }
        }
    }

    Patterns::None
}

enum Method {
    Query,

    Bind {
        raw: bool,
        stdout: bool,
        overwrite: bool,
        with_date: bool,
    },
}

struct Action {
    devices: Vec<InputDevice>,
    method: Method,
}

fn actions(args: Patterns) -> (char, Vec<Action>) {
    let mut devices = scan_devices();
    match args {
        Patterns::Query(queries) => (
            'q',
            queries
                .into_iter()
                .map(|q| dquery(q, &mut devices))
                .collect::<Vec<Action>>(),
        ),

        Patterns::Bind(bindings) => (
            'b',
            bindings
                .into_iter()
                .map(|b| dbind(b, &mut devices))
                .collect::<Vec<Action>>(),
        ),

        Patterns::None => unreachable!(),
    }
}

fn dquery(pats: Vec<String>, devices: &mut Vec<InputDevice>) -> Action {
    Action {
        devices: {
            let mut rm = devices
                .into_iter()
                .enumerate()
                .inspect(|(i, _)| print!("{}, ", i))
                .filter(|(_, d)| pats.iter().all(|p| d.name().contains(p)))
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();
            rm.sort_by(|a, b| b.cmp(a));

            // BUG: index out of range
            println!("\nlen: {}", devices.len());
            rm.into_iter().map(|i| devices.remove(i)).collect()
        },
        method: Method::Query,
    }
}

fn dbind(pats: (u8, Vec<String>), devices: &mut Vec<InputDevice>) -> Action {
    let args = pats.0;
    let pats = pats.1;
    Action {
        devices: {
            let mut rm = devices
                .into_iter()
                .enumerate()
                .filter(|(_, d)| pats.iter().all(|p| d.name().contains(p)))
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();
            rm.sort_by(|a, b| b.cmp(a));

            rm.into_iter().map(|i| devices.remove(i)).collect()
        },
        method: Method::Bind {
            raw: args & 1 == 0,
            stdout: args & 2 == 0,
            overwrite: args & 8 == 8,
            with_date: args & 16 == 16,
        },
    }
}

//

struct Logger {
    name: String,
    event: String,
    raw: bool,
    stdout: bool,
    with_date: bool,
    overwrite: bool,
}

fn baction(action: Action) -> Vec<Logger> {
    let Method::Bind {
        raw,
        stdout,
        overwrite,
        with_date,
    } = action.method
    else {
        unreachable!()
    };

    action
        .devices
        .into_iter()
        .map(|d| Logger {
            name: d.name().to_string(),
            event: d.event().unwrap().to_string(),
            raw,
            stdout,
            overwrite,
            with_date,
        })
        .collect()
}

fn bactions(actions: Vec<Action>) -> Vec<Logger> {
    actions.into_iter().map(|a| baction(a)).flatten().collect()
}

fn logs(logger: Logger) {
    let gpp = format!("/dev/input/{}", logger.event);
    let mut gpf = File::open(gpp).expect("input event file not found");
    println!("{:?}", gpf);

    // the 24 bytes is the size of the input_event c struct found in /usr/include/linux/input.h header file
    // but this only works for linux x86_64, on other systems, this struct may have a different size
    // mainly because of differences in the representation of time values (4bytes, 8bytes, etc.)
    // if a size that is smaller than the size of input_event is provided, this program would
    // crash, if a bigger size is provided, the program works fine
    // TODO: detect system and derive input_event struct size to be used
    const IES_SIZE: usize = 24;
    let mut buf = [0u8; IES_SIZE];

    let refresh = 1000 / 60;

    let mut log = String::new();
    let mut writer = Writer::new(
        &logger.name,
        logger.stdout,
        logger.with_date,
        logger.overwrite,
        logger.raw,
    );

    // let mut brk = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(refresh));

        gpf.read(&mut buf).unwrap();
        _ = writer.write(&buf, &mut log);

        // brk += 1;
        // if brk == 4 {
        //     println!("\r\n");
        //     brk = 0;
        // }
    }
}

fn bind_logger(logger: Logger) -> JoinHandle<()> {
    spawn(move || logs(logger))
}

fn bind_loggers(loggers: Vec<Logger>) -> Vec<JoinHandle<()>> {
    loggers.into_iter().map(|l| bind_logger(l)).collect()
}

fn qdevices(actions: Vec<Action>) -> Vec<InputDevice> {
    actions.into_iter().map(|a| a.devices).flatten().collect()
}

fn query(actions: Vec<Action>) {
    println!("queried devices: \r\n{:#?}", qdevices(actions));
}

pub(crate) fn run() -> Option<Vec<JoinHandle<()>>> {
    let args = args();
    let (m, actions) = actions(args);
    match m {
        'b' => {
            let loggers = bactions(actions);

            Some(bind_loggers(loggers))
        }
        'q' => {
            query(actions);

            None
        }
        _ => unreachable!(),
    }
}
