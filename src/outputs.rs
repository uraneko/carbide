use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::thread::{spawn, JoinHandle};

use crate::format::{codes::code, types::type_};
use crate::input__event::input_event;
use crate::inputs::{parse_args, run};
use crate::log::{log, log_with_date, Writer};

fn log_event(e: &str, writer: &mut Writer) {
    let gpp = format!("/dev/input/{}", e);
    let mut gpf = File::open(gpp).expect("input event file not found");
    println!("{:?}", gpf);

    // the 24 bytes is the size of the input_event c struct found in /usr/include/linux/input.h header file
    // but this only works for linux x86_64, on other systems, this struct may have a different size
    // mainly because of differences in the representation of time values (4bytes, 8bytes, etc.)
    // if a size that is smaller than the size of input_event is provided, this program would
    // crash, if a bigger size is provided, the program works fine
    // TODO: detect system and derive input_event struct size to be used
    const IES_SIZE: usize = 24;
    let mut buf = [0u8; IES_SIZE];

    let refresh = 1000 / 60;

    // let mut brk = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(refresh));

        gpf.read(&mut buf).unwrap();
        writer.write(format!("[{:?}] {{{:?}}}", std::time::SystemTime::now(), buf).as_bytes());

        writer.write(
            &format!(
                "[{:?}] {{{:?}}}",
                std::time::SystemTime::now(),
                input_event::from_bytes(&buf)
            )
            .as_bytes(),
        );

        // brk += 1;
        // if brk == 4 {
        //     println!("\r\n");
        //     brk = 0;
        // }
    }
}

fn bind_logger(event: String, out: &str, name: &str) -> JoinHandle<()> {
    spawn(move || log_event(&event, &mut Writer::None))
}
