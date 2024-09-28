use std::fs;
use std::io::Write;

use crate::input__event::input_event;

const LOGS_DIR: &str = "logs";

// with date
pub(crate) fn log_with_date(name: &str) -> fs::File {
    // get current date
    let date = std::process::Command::new("date").output();
    let date = if date.is_ok() {
        format!("{:?}", date.unwrap())
    } else {
        eprintln!("couldn't get time");
        format!("{:?}", std::time::SystemTime::now())
    };

    let name = LOGS_DIR.to_string() + "/" + &date + name;

    // check if logs dir exists, if not make it
    if !std::path::Path::new(LOGS_DIR).is_dir() {
        fs::create_dir(LOGS_DIR).unwrap();
    }

    // open file with necessary options and return it
    fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(name)
        .unwrap()
}

// without date
// if overwrite is on then with_date can not be on
// date can not be used with continue or overwrite
pub(crate) fn log(name: &str, ow: bool) -> fs::File {
    // check if logs dir exists, if not make it
    if !std::path::Path::new(LOGS_DIR).is_dir() {
        fs::create_dir(LOGS_DIR).unwrap();
    }

    let mut file = fs::OpenOptions::new();
    file.write(true);
    file.create(true);

    if ow {
        file.truncate(true).truncate(true);
    } else {
        file.append(true);
    }

    // open file with necessary options and return it
    file.open(name).unwrap()
}

#[derive(Debug)]
pub(super) enum Writer {
    Term {
        writer: std::io::StdoutLock<'static>,
        raw: bool,
    },
    File {
        writer: std::fs::File,
        raw: bool,
    },
}

impl Writer {
    pub fn new(name: &str, stdout: bool, with_date: bool, overwrite: bool, raw: bool) -> Writer {
        if stdout {
            Writer::Term {
                writer: std::io::stdout().lock(),
                raw,
            }
        } else {
            if with_date {
                Writer::File {
                    writer: log_with_date(name),
                    raw,
                }
            } else {
                Writer::File {
                    writer: log(&(LOGS_DIR.to_string() + "/" + name), overwrite),
                    raw,
                }
            }
        }
    }
}

fn parse_bytes<'a>(buf: &[u8; 24], log: &'a mut String) -> &'a [u8] {
    *log = format!("[{:?}] {{{:?}}}\r\n", std::time::SystemTime::now(), buf);

    log.as_bytes()
}
fn parse_event<'a>(buf: &[u8; 24], log: &'a mut String) -> &'a [u8] {
    *log = format!(
        "[{:?}] {{{:?}}}\r\n",
        std::time::SystemTime::now(),
        input_event::from_bytes(&buf)
    );
    log.as_bytes()
}

fn parse<'a>(buf: &[u8; 24], raw: bool, log: &'a mut String) -> &'a [u8] {
    if raw {
        parse_bytes(buf, log)
    } else {
        parse_event(buf, log)
    }
}

impl Writer {
    pub(crate) fn write(&mut self, buf: &[u8; 24], log: &mut String) -> std::io::Result<()> {
        if let Self::Term { writer, raw } = self {
            let logs = parse(buf, *raw, log);
            _ = writer.write(logs);
        } else if let Self::File { writer, raw } = self {
            let logs = parse(buf, *raw, log);
            _ = writer.write(logs);
        }
        Ok(())
    }
}
