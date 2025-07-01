use std::io::Write;

use carbide::devices;
use carbide::input_event;
use clap::{Args, Parser, ValueEnum};

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
    CantGiveAccessToParentDir,
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
    /// whether to parse the input_events or keep them as raw bytes
    #[arg(long, short = 'r')]
    raw_bytes: bool,
    /// where to output the read input event value
    /// possible values are <stdout> and <some_path_name> in the current directory
    #[arg(long, short = 'o')]
    output: Option<ListenOutput>,
    /// whether to truncate the output file if it already exists
    #[arg(long, short = 't')]
    truncate: bool,
    /// the event number of the input device the program would listen in to
    /// you can list devices names + event numbers by running
    /// ```bash
    /// carb find -en -F "<your filters>" "<go here>"
    /// # run
    /// $ carb find --help
    /// for more on the find command
    /// ```
    #[arg(long, short = 'e')]
    event: u8,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
enum ListenOutput {
    #[default]
    Stdout,
    File(std::path::PathBuf),
    All(std::path::PathBuf),
}

impl std::str::FromStr for ListenOutput {
    type Err = CLIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stdout" => Ok(Self::Stdout),
            val if val.contains("..") || val.starts_with("/") =>
            // val if val.contains("/") =>
            {
                Err(CLIError::CantGiveAccessToParentDir)
            }
            val if val.starts_with('+') => {
                let path: std::path::PathBuf = val[1..].into();

                Ok(Self::All(path))
            }
            val => {
                let path: std::path::PathBuf = val.into();

                Ok(Self::File(path))
            }
        }
    }
}

impl CommandLauncher for Listen {
    fn run(self) -> Result<String, CLIError> {
        let event = format!("event{}", self.event);
        let output = self.output.unwrap_or(ListenOutput::default());

        match output {
            ListenOutput::File(p) => {
                input_event::read_to_file(p, self.truncate, self.raw_bytes, &event);
            }
            ListenOutput::All(p) => {
                input_event::read_to_all(p, self.truncate, self.raw_bytes, &event);
            }
            ListenOutput::Stdout => {
                input_event::read_to_stdout(&event, self.raw_bytes);
            }
        }

        Ok(format!("listening on input event {}", self.event))
    }
}
