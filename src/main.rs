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
