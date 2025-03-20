mod devices;
// mod input_event;

fn main() {
    cli();
}

fn cli() {
    let mut args = std::env::args();
    args.next();

    eprintln!("## {:?}", args);

    let arg = args.next().unwrap_or("".into());

    match &arg[..2] {
        "-L" => List::new(&arg, args).print_devices(),
        "-R" => read(&arg, args),
        "-H" => help(&arg, args),
        _ => panic!("unrecognized option flag"),
    }
}

// fn list(arg: &str, args: std::env::Args) {
//     match arg {
//         "-L" => {
//             let devs = devices::get_devices();
//
//             println!(
//                 "{:#?}",
//                 devs.iter().map(|d| d.name()).collect::<Vec<&str>>()
//             );
//         }
//         "-Lf" | "-Lfn" => {
//             let pats = args.collect::<Vec<String>>();
//             let devs = devices::get_devices();
//             let fltr = devices::filter_devices(&devs, &pats);
//
//             println!(
//                 "{:#?}",
//                 fltr.iter().map(|d| d.name()).collect::<Vec<&str>>()
//             );
//         }
//         "-Lfs" | "-Lfsn" => {
//             let pats = args.collect::<Vec<String>>();
//             let devs = devices::get_devices();
//             let fltr = devices::filter_devices_strict(&devs, &pats);
//
//             println!(
//                 "{:#?}",
//                 fltr.iter().map(|d| d.name()).collect::<Vec<&str>>()
//             );
//         }
//         _ => eprintln!("error, unrecognized argument"),
//     }
// }

struct List {
    strict: bool,
    name_only: bool,
    filters: Option<Vec<String>>,
}

impl List {
    fn new(arg: &str, args: std::env::Args) -> Self {
        Self {
            strict: arg.contains("s"),
            name_only: arg.contains("n"),
            filters: if arg.contains("f") {
                Some(args.collect::<Vec<String>>())
            } else {
                None
            },
        }
    }

    fn print_devices(self) {
        let mut devices = devices::get_devices();
        if self.filters.is_some() {
            match self.strict {
                true => {
                    devices = devices::into_filter_devices_strict(devices, self.filters.unwrap());
                }
                false => devices = devices::into_filter_devices(devices, self.filters.unwrap()),
            }
        }

        if !self.name_only {
            println!("{:#?}", devices);
            return;
        }

        let devices = devices.iter().map(|d| d.name()).collect::<Vec<&str>>();
        println!("{:#?}", devices);
    }
}

fn read(arg: &str, args: std::env::Args) {}

fn help(arg: &str, _: std::env::Args) {
    let red: String = clr(241, 153, 123);
    let blue: String = clr(167, 123, 213);
    println!(
        "{}\n\n{}{}\n\n{}\n\n{}\t{}\n{}\t{}\n{}\t{}",
        "Linux input device event logger",
        blue.clone() + "Usage:" + END + " ",
        red.clone() + "fen [COMMAND][OPTIONS] [ARGUMENTS]" + END,
        blue.clone() + "COMMANDS:" + END + " ",
        blue.clone() + "-L (list)",
        red.clone() + "list the input devices detected on this host machine",
        blue.clone() + "-R (read)",
        red.clone() + "read the input events of a selected input device",
        blue.clone() + "-H (help)",
        red.clone() + "print this help message",
    );
}

use colors::{clr, END};

mod colors {
    const START: &str = "\x1b[1;38;2";
    pub(super) const END: &str = "\x1b[0m";

    pub(super) fn clr(r: u8, g: u8, b: u8) -> String {
        format!("{};{};{};{}m", START, r, g, b)
    }
}
