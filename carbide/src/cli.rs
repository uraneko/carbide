use crate::input_event;

mod help;
mod list;
mod read;

use help::Help;
use list::List;
use read::Read;

pub fn cli() {
    let mut args = std::env::args();
    args.next();

    eprintln!("## {:?}", args);

    let arg = args.next().unwrap_or("".into());

    match &arg[..2] {
        "-L" => List::new(&arg, args).list(),
        "-R" => Read::new(&arg, args).read(),
        "-H" => Help::new(&arg, args).help(),
        _ => panic!("unrecognized option flag"),
    }
}
