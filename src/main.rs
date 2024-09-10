use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

const INPUT_DEVICES: &str = "/proc/bus/input/devices";

fn main() {
    // gamepad input file path
    // event19 was gamepad 2
    // 18 was gp1
    // 17 was kbd
    // and 10 was touchpad
    // TODO: which event file pertains to which input device
    // TODO: make a struct InputDevice
    // then: read '/proc/bus/input/devices'
    // then: parse results into InputDevice instances

    let mut args = std::env::args();
    args.next();

    let pats = parse_args();
    println!("{:?}", pats);

    // println!("{:?}", parse_devices());
}

fn parse_args() -> Vec<Vec<String>> {
    let mut args = std::env::args();
    args.next();

    let mut v = vec![];
    let mut vv = vec![];
    while let Some(arg) = args.next() {
        if &arg == "-d" {
            if !vv.is_empty() {
                v.push(vv.drain(..).collect());
            }
        } else {
            vv.push(arg)
        }
    }

    v
}

#[derive(Debug)]
struct InputDevice {
    bus: String,
    name: String,
    phys: String,
    sysfs: String,
    uniq: Option<u8>,
    handlers: HashSet<String>,
    props: HashMap<String, String>,
}

fn parse_devices() -> Vec<InputDevice> {
    let mut f = File::open(INPUT_DEVICES).unwrap();
    let mut s = String::new();

    _ = File::read_to_string(&mut f, &mut s).unwrap();

    let mut s = s.split("\n\n").into_iter().map(|s| s.to_owned());

    let mut v = vec![];

    while let Some(indev) = s.next() {
        if !indev.is_empty() {
            v.push(parse_device(&indev));
        }
    }

    v
}

fn parse_device(device: &str) -> InputDevice {
    let mut s = device.split('\n').map(|s| s.to_owned());

    InputDevice {
        bus: {
            let Some(bus) = s.next() else {
                panic!("input devices file gave bad data")
            };
            assert_eq!(&bus[..7], "I: Bus=");
            bus
        },

        name: {
            let Some(name) = s.next() else {
                panic!("input devices file gave bad data")
            };
            assert_eq!(&name[..8], "N: Name=");
            name
        },
        phys: {
            let Some(phys) = s.next() else {
                panic!("input devices file gave bad data")
            };
            assert_eq!(&phys[..8], "P: Phys=");
            phys
        },
        sysfs: {
            let Some(sysfs) = s.next() else {
                panic!("input devices file gave bad data")
            };
            assert_eq!(&sysfs[..9], "S: Sysfs=");
            sysfs
        },
        uniq: {
            let Some(uniq) = s.next() else {
                panic!("input devices file gave bad data")
            };
            assert_eq!(&uniq[..8], "U: Uniq=");
            uniq.parse().ok()
        },
        handlers: {
            let Some(handlers) = s.next() else {
                panic!("input devices file gave bad data")
            };
            assert_eq!(&handlers[..12], "H: Handlers=");
            handlers
                .split(' ')
                .map(|s| s.to_owned())
                .collect::<HashSet<String>>()
        },

        props: {
            let mut map = HashMap::new();
            while let Some(prop) = s.next() {
                assert_eq!(&prop[..3], "B: ");
                let p = &mut prop.replace("B: ", "");
                let mut p = p.split('=').map(|s| s.to_owned());
                map.insert(p.next().unwrap(), p.next().unwrap_or("".to_string()));
            }
            map
        },
    }
}

fn log_input(e: &str) {
    let gpp = format!("/dev/input/{}", e);
    let mut gpf = File::open(gpp).expect("input binary file not found");
    println!("{:?}", gpf);

    // the 24 bytes is the size of the input_event c struct found in /usr/include/linux/input.h header file
    // but this only works for linux x86_64, on other systems, this struct may have a different size
    // mainly because of differences in the representation of time values (4bytes, 8bytes, etc.)
    // if a size that is smaller than the size of input_event is provided, this program would
    // crash, if a bigger size is provided, the program works fine
    // TODO: detect system and derive input_event struct size to be used
    const IES_SIZE: usize = 28;
    let mut buf = [0u8; IES_SIZE];

    let refresh = 1000 / 60;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(refresh));

        gpf.read(&mut buf).unwrap();
        println!("{:?}", buf);
    }
}

fn find_device<'a>(devices: &'a [InputDevice], pat: &[&'a str]) -> Option<&'a String> {
    let dev = devices.iter().find(|d| {
        let mut condition = true;
        for p in pat {
            if !d.name.contains(p) {
                condition = false;
                break;
            }
        }

        condition
    });
    if dev.is_some() {
        return dev.unwrap().handlers.iter().find(|h| h.contains("event"));
    }

    None
}

use std::thread::{spawn, JoinHandle};

fn bind_logger(event: &'static str) -> JoinHandle<()> {
    spawn(move || log_input(event))
}
