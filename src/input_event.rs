use std::ffi::c_int;
use std::fs::File;
use std::io::{Read, Write};

// BUG probably have to use c types from the c library or std c types
#[repr(C)]
#[derive(Debug)]
pub struct input_event {
    time: timeval,
    type_: u16,
    code: u16,
    value: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct timeval {
    tv_sec: i64,  // time_t
    tv_usec: u64, // long int
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

pub(crate) fn read(event: &str) {
    let mut reader = File::open(EVENTS_DIR.to_string() + event).unwrap();
    let mut buf: [u8; BUF_SIZE] = [0u8; BUF_SIZE];

    let mut brk = 0;
    loop {
        brk += 1;

        _ = reader.read(&mut buf).unwrap();
        // _ = writer.write_all(&buf).unwrap();
        println!("{:?}", buf);
        if brk == 1 || brk == 3 {
            println!("{}\n", input_event::from_buf(&buf));
        }
        if brk == 4 {
            println!("-------------- event received ---------------");
            brk = 0;
        }
    }
}
