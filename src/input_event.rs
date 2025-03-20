use std::ffi::c_int;

use crate::code;
use crate::type_;

#[repr(C)]
#[derive(Debug)]
pub struct input_event {
    time: timeval,
    type_: u16,
    code: u16,
    value: i32,
}

impl std::fmt::Display for input_event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "input_event {{ time: {:?}, type: {}, code: {}, value: {} }}",
            self.time,
            type_(self.type_),
            code(self.code),
            self.value
        )
    }
}

impl input_event {
    pub fn from_bytes(bytes: &[u8; 24]) -> Self {
        Self {
            time: timeval::from_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]),
            // FIXME: type bytes are probably actually value bytes
            // which doesnt make much sense but im not sure
            type_: u16::from_le_bytes([bytes[16], bytes[17]]),
            code: u16::from_le_bytes([bytes[18], bytes[19]]),
            value: i32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct timeval {
    tv_sec: i32,  // time_t
    tv_usec: u32, // suseconds_t
}

impl timeval {
    fn from_bytes(bytes: [u8; 8]) -> Self {
        Self {
            tv_sec: i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            tv_usec: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        }
    }
}

// fn decode_bytes(bytes: &[u8]) {
//     let k = key(bytes[0]);
//     println!("{}", k);
// }

// TODO: parse the bytes input into a human readable format; c ffi the input_event struct
