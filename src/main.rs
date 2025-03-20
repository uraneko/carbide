mod devices;
// mod input_event;

fn main() {
    cli();
}

fn cli() {
    let mut args = std::env::args();
    args.next();

    eprintln!("## {:?}", args);

    match next(args.next()).as_str() {
        "-L" => {
            let devs = devices::get_devices();

            println!(
                "{:#?}",
                devs.iter().map(|d| d.name()).collect::<Vec<&str>>()
            );
        }
        "-Lf" => {
            let pats = args.collect::<Vec<String>>();
            let devs = devices::get_devices();
            let fltr = devices::filter_devices(&devs, &pats);

            println!(
                "{:#?}",
                fltr.iter().map(|d| d.name()).collect::<Vec<&str>>()
            );
        }
        "-Lfs" => {
            let pats = args.collect::<Vec<String>>();
            let devs = devices::get_devices();
            let fltr = devices::filter_devices_strict(&devs, &pats);

            println!(
                "{:#?}",
                fltr.iter().map(|d| d.name()).collect::<Vec<&str>>()
            );
        }
        _ => eprintln!("error, unrecognized argument"),
    }
}

fn next(arg: Option<String>) -> String {
    arg.unwrap_or("".into())
}
