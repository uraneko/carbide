use std::ffi::{c_long, c_uint, c_ulong, c_ushort};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(
    Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Hash, Default, PartialOrd, Ord,
)]

pub struct input_event {
    time: timeval,
    type_: c_ushort,
    code: c_ushort,
    value: c_uint,
}

// repr c here does nothing memory allignment wise
#[repr(C)]
#[derive(
    Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Hash, Default, PartialOrd, Ord,
)]
pub struct timeval {
    // time_t
    tv_sec: c_long,
    // long int
    tv_usec: c_ulong,
}

const EVENTS_DIR: &str = "/dev/input/";
const BUF_SIZE: usize = std::mem::size_of::<input_event>();

// DOCS
// first 16 bytes are the timeval struct 2 fields
// the next 2 bytes are the event type
// the next 2 bytes are the event code
// the last 4 bytes are the event value

impl input_event {
    fn from_buf(buf: &[u8; BUF_SIZE]) -> Self {
        let mut type_ = 0u16;
        type_ |= buf[17] as u16;
        type_ <<= 8;
        type_ |= buf[16] as u16;

        let mut code = 0u16;
        code |= buf[19] as u16;
        code <<= 8;
        code |= buf[18] as u16;

        let mut value = 0u32;
        for byte in buf[20..].iter().rev() {
            value <<= 8;
            value |= *byte as u32;
        }

        Self {
            time: timeval::from_bytes(&buf[..16].try_into().unwrap()),
            type_,
            code,
            value,
        }
    }
}

impl timeval {
    fn from_bytes(bytes: &[u8; 16]) -> Self {
        // let mut tv_sec = 0i64;
        // for byte in bytes[..8].iter().rev() {
        //     tv_sec |= *byte as i64;
        //     tv_sec <<= 8;
        // }
        let tv_sec = i64::from_le_bytes(bytes[..8].try_into().unwrap());

        // let mut tv_usec = 0u64;
        // for byte in bytes[8..].iter().rev() {
        //     tv_usec |= *byte as u64;
        //     tv_usec <<= 8;
        // }
        let tv_usec = u64::from_le_bytes(bytes[8..].try_into().unwrap());

        Self { tv_sec, tv_usec }
    }
}

impl std::fmt::Display for timeval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.tv_sec, self.tv_usec)
    }
}

impl std::fmt::Display for input_event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n   time: {},\n   type: {},\n   code: {},\n   value: {}\n}}",
            self.time, self.type_, self.code, self.value
        )
    }
}

fn open_file(p: PathBuf, trunc: bool) -> File {
    std::fs::OpenOptions::new()
        .write(true)
        .truncate(trunc)
        .append(!trunc)
        .create(true)
        .open(p)
        .unwrap()
}

pub fn read_to_all(p: PathBuf, trunc: bool, raw: bool, event: &str) {
    let f = open_file(p, trunc);
    let stdout = std::io::stdout();
    let (mut reader, mut so_writer, mut f_writer) = (
        prepare_reader(event),
        prepare_writer(stdout),
        prepare_writer(f),
    );
    let mut buf: [u8; BUF_SIZE] = [0u8; BUF_SIZE];

    let mut brk = 0;
    loop {
        _ = write_event_padding(&mut brk, &mut so_writer);
        _ = write_event_padding(&mut brk, &mut f_writer);
        _ = reader.read(&mut buf);
        _ = write_raw_or_parsed(raw, &mut brk, &buf, &mut so_writer);
        _ = write_raw_or_parsed(raw, &mut brk, &buf, &mut f_writer);
    }
}

pub fn read_to_file(p: PathBuf, trunc: bool, raw: bool, event: &str) {
    let f = open_file(p, trunc);
    let (mut reader, mut writer) = prepare_reader_writer(f, event);
    let mut buf: [u8; BUF_SIZE] = [0u8; BUF_SIZE];

    let mut brk = 0;
    loop {
        _ = write_event_padding(&mut brk, &mut writer);
        _ = reader.read(&mut buf);
        _ = write_raw_or_parsed(raw, &mut brk, &buf, &mut writer);
    }
}

pub fn read_to_stdout(event: &str, raw: bool) {
    let stdout = std::io::stdout();
    let (mut reader, mut writer) = prepare_reader_writer(stdout, event);
    let mut buf: [u8; BUF_SIZE] = [0u8; BUF_SIZE];

    let mut brk = 0;
    loop {
        _ = write_event_padding(&mut brk, &mut writer);
        _ = reader.read(&mut buf);
        _ = write_raw_or_parsed(raw, &mut brk, &buf, &mut writer);
    }
}

fn write_raw_or_parsed<T: Write>(
    raw: bool,
    brk: &mut u8,
    buf: &[u8; BUF_SIZE],
    writer: &mut BufWriter<T>,
) -> Result<(), std::io::Error> {
    if raw {
        write_raw_event(brk, buf, writer)
    } else {
        write_event(brk, buf, writer)
    }
}

fn prepare_writer<T: Write>(inner: T) -> BufWriter<T> {
    BufWriter::new(inner)
}

fn prepare_reader(e: &str) -> BufReader<File> {
    let file = File::open(EVENTS_DIR.to_string() + e).unwrap();

    std::io::BufReader::new(file)
}

fn prepare_reader_writer<T: Write>(write_to_me: T, e: &str) -> (BufReader<File>, BufWriter<T>) {
    (prepare_reader(e), prepare_writer(write_to_me))
}

fn write_event_padding<T: Write>(
    brk: &mut u8,
    writer: &mut BufWriter<T>,
) -> Result<(), std::io::Error> {
    if *brk == 0 {
        let res = writer.write_all(b"\n-------------- event received ---------------\n");
        *brk = 4;

        return res;
    }
    *brk -= 1;

    Ok(())
}

fn write_event<T: Write>(
    brk: &mut u8,
    buf: &[u8; BUF_SIZE],
    writer: &mut BufWriter<T>,
) -> Result<(), std::io::Error> {
    if *brk == 1 || *brk == 3 {
        let event = input_event::from_buf(buf);
        _ = writer.write_all(event.to_string().as_bytes());
        _ = writer.write_all(&[10, 13]);
        _ = writer.flush();
    }

    Ok(())
}

fn write_raw_event<T: Write>(
    brk: &mut u8,
    buf: &[u8; BUF_SIZE],
    writer: &mut BufWriter<T>,
) -> Result<(), std::io::Error> {
    if *brk == 1 || *brk == 3 {
        _ = writer.write_all(format!("{:?}", &buf).as_bytes());
        _ = writer.write_all(&[10, 13]);
        _ = writer.flush();
    }

    Ok(())
}
