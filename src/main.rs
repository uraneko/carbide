mod devices;
mod input_event;

fn main() {
    cli();
    // input_event::read("event17");
}

fn cli() {
    let mut args = std::env::args();
    args.next();

    eprintln!("## {:?}", args);

    let arg = args.next().unwrap_or("".into());

    match &arg[..2] {
        "-L" => List::new(&arg, args).list(),
        "-R" => input_event::read("event17"),
        "-H" => Help::new(&arg, args).help(),
        _ => panic!("unrecognized option flag"),
    }
}

struct List {
    // works only with the filter (f) option
    strict: bool,
    // prints the name and anything else turned on amongst the reductive options
    name_reductive: bool,
    // prints the event file name and anything else turned on amongst the reductive options
    event_reductive: bool,
    // applys filters by name to the device list
    filters: Option<Vec<String>>,
}

impl List {
    fn new(arg: &str, args: std::env::Args) -> Self {
        Self {
            strict: arg.contains('s'),
            name_reductive: arg.contains('n'),
            event_reductive: arg.contains('e'),
            filters: if arg.contains('f') {
                Some(args.collect::<Vec<String>>())
            } else {
                None
            },
        }
    }

    fn list(self) {
        let mut devices = devices::get_devices();
        if self.filters.is_some() {
            match self.strict {
                true => {
                    devices = devices::into_filter_devices_strict(devices, self.filters.unwrap());
                }
                false => devices = devices::into_filter_devices(devices, self.filters.unwrap()),
            }
        }

        match [self.name_reductive, self.event_reductive] {
            [true, true] => {
                devices
                    .iter()
                    .map(|d| (d.name(), d.event()))
                    .inspect(|(n, e)| println!("[\n    name: '{}',\n    event: '{}'\n]", n, e))
                    .count();
            }
            [false, false] => println!("{:#?}", devices),
            [true, false] => println!(
                "{:#?}",
                devices.iter().map(|d| d.name()).collect::<Vec<&str>>()
            ),
            [false, true] => println!(
                "{:#?}",
                devices.iter().map(|d| d.event()).collect::<Vec<&str>>()
            ),
        }
    }
}

fn read(arg: &str, args: std::env::Args) {}

fn help() {
    // NOTE colors orange and violet
    let red: String = clr(241, 153, 123);
    let blue: String = clr(167, 123, 213);
    // NOTE colors red and blue
    // let red: String = clr(241, 53, 123);
    // let blue: String = clr(127, 123, 233);
    println!(
        "{}\n\n{}{}\n\n{}\n\n{}\t{}\n{}\t{}\n{}\t{}",
        "Linux input device event logger",
        blue.clone() + "Usage:" + END + " ",
        red.clone() + "crb [COMMAND][OPTIONS] [ARGUMENTS]" + END,
        blue.clone() + "COMMANDS:" + END + " ",
        blue.clone() + "-L (list)",
        red.clone() + "list the input devices detected on this host machine",
        blue.clone() + "-R (read)",
        red.clone() + "read the input events of a selected input device",
        blue.clone() + "-H (help)",
        red.clone() + "print this help message",
    );
}

fn examples() {}

use colors::{clr, END};

mod colors {
    const START: &str = "\x1b[1;38;2";
    pub(super) const END: &str = "\x1b[0m";

    pub(super) fn clr(r: u8, g: u8, b: u8) -> String {
        format!("{};{};{};{}m", START, r, g, b)
    }
}

struct Help {
    show_examples: bool,
    commands: Vec<String>,
}

impl Help {
    fn new(arg: &str, args: std::env::Args) -> Self {
        Self {
            show_examples: arg.contains("e"),
            commands: args.collect::<Vec<String>>(),
        }
    }

    fn help(self) {
        if self.show_examples {
            examples();
            return;
        }
        help();
    }
}

struct Read {
    save_output: bool,
    raw_events: bool,
    print_output: bool,
}

impl Read {
    fn new(arg: &str) -> Self {
        Self {
            save_output: arg.contains('s'),
            raw_events: arg.contains('r'),
            print_output: arg.contains('p'),
        }
    }
}
