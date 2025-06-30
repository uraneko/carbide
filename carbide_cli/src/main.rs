use std::io::Write;

use carbide::devices;
use carbide::input_event;
use clap::{Args, Parser};

fn main() -> Result<(), CLIError> {
    let res = match Carbide::parse() {
        Carbide::Find(q) => q.run(),
        Carbide::Listen(l) => l.run(),
    }?;

    let stdout = std::io::stdout().lock();
    std::io::BufWriter::new(stdout).write_all(res.as_bytes())?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CLIError {
    IOError,
}

impl std::fmt::Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CLIError {}

impl From<std::io::Error> for CLIError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError
    }
}

trait CommandLauncher {
    fn run(self) -> Result<String, CLIError>;
}

#[derive(Debug, Parser, PartialEq, Eq, Clone, Hash)]
enum Carbide {
    Listen(Listen),
    Find(Find),
}

#[derive(Debug, Args, PartialEq, Eq, Clone, Hash)]
struct Find {
    // print device name
    #[arg(long, short = 'n')]
    name: bool,
    // print device event
    #[arg(long, short = 'e')]
    event: bool,
    // print full device metadata
    #[arg(long, short = 'f')]
    full: bool,
    // device filters by name
    #[arg(long, short = 'F', num_args = 1.., value_delimiter = ' ')]
    filters: Option<Vec<String>>,
    // TODO
    #[arg(long, short = 'I')]
    ignore_case: bool,
    #[arg(long, short = 'E')]
    exact_matches: bool,
}

impl CommandLauncher for Find {
    fn run(self) -> Result<String, CLIError> {
        let mut devices = devices::get_devices();
        if let Some(mut pats) = self.filters {
            if self.ignore_case {
                devices::ignore_case(&mut devices, &mut pats);
            }

            devices = if self.exact_matches {
                devices::into_filter_devices_exact_matches(devices, pats)
            } else {
                devices::into_filter_devices(devices, pats)
            };
        }
        if devices.is_empty() {
            return Ok("No devices matching the passed filters were found".into());
        }

        Ok(match [self.name, self.event] {
            [true, true] => devices
                .iter()
                .map(|d| (d.name(), d.event()))
                .map(|(n, e)| format!("{{\n    name: '{}',\n    event: '{}'\n}}", n, e))
                .reduce(|acc, d| acc + "\n" + &d)
                .unwrap(),
            [false, false] => format!("{:#?}", devices),
            [true, false] => format!(
                "{:#?}",
                devices.iter().map(|d| d.name()).collect::<Vec<&str>>()
            ),
            [false, true] => format!(
                "{:#?}",
                devices.iter().map(|d| d.event()).collect::<Vec<&str>>()
            ),
        })
    }
}

#[derive(Debug, Args, PartialEq, Eq, Clone, Hash)]
#[command(alias = "lisn")]
struct Listen {
    #[arg(long, short = 'r')]
    raw: bool,
    #[arg(long, short = 'o')]
    output: Option<String>,
    #[arg(long, short = 'p')]
    print: bool,
    #[arg(long, short = 'e')]
    event: u8,
}

impl CommandLauncher for Listen {
    fn run(self) -> Result<String, CLIError> {
        let event = format!("event{}", self.event);

        input_event::read(&event);

        Ok(format!("listening on input event {}", self.event))
    }
}
