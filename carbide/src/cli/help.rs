fn help() {
    // NOTE colors orange and violet
    let red: String = clr(241, 153, 123);
    let blue: String = clr(167, 123, 213);
    // NOTE colors red and blue
    // let red: String = clr(241, 53, 123);
    // let blue: String = clr(127, 123, 233);
    println!(
        "{}\n\n{}{}\n\n{}\n\n{}\t{}\n{}\t{}\n{}\t{}",
        "Linux input_event parser",
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

use colors::{END, clr};

mod colors {
    const START: &str = "\x1b[1;38;2";
    pub(super) const END: &str = "\x1b[0m";

    pub(super) fn clr(r: u8, g: u8, b: u8) -> String {
        format!("{};{};{};{}m", START, r, g, b)
    }
}

pub struct Help {
    show_examples: bool,
    commands: Vec<String>,
}

impl Help {
    pub fn new(arg: &str, args: std::env::Args) -> Self {
        Self {
            show_examples: arg.contains("e"),
            commands: args.collect::<Vec<String>>(),
        }
    }

    pub fn help(self) {
        if self.show_examples {
            examples();
            return;
        }
        help();
    }
}
