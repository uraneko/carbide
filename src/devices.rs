use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

const INPUT_DEVICES: &str = "/proc/bus/input/devices";

#[derive(Debug)]
pub(crate) struct InputDevice {
    bus: String,
    name: String,
    phys: String,
    sysfs: String,
    uniq: Option<u8>,
    handlers: HashSet<String>,
    props: HashMap<String, String>,
}

impl InputDevice {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn event(&self) -> Option<&String> {
        self.handlers.iter().find(|h| h.contains("event"))
    }
}

pub(crate) fn scan_devices() -> Vec<InputDevice> {
    let mut f = File::open(INPUT_DEVICES).unwrap();
    let mut s = String::new();

    _ = File::read_to_string(&mut f, &mut s).unwrap();

    let mut s = s.split("\n\n").into_iter().map(|s| s.to_owned());

    let mut v = vec![];

    while let Some(indev) = s.next() {
        if !indev.is_empty() {
            v.push(scan_device(&indev));
        }
    }

    v
}

pub(crate) fn scan_device(device: &str) -> InputDevice {
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
                .replace("H: Handlers=", "")
                .split(' ')
                .map(|s| s.to_owned())
                .filter(|h| !h.is_empty())
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

pub(crate) fn filter_devices<'a, 'b>(
    devices: &'a [InputDevice],
    pat: &'b [String],
) -> Vec<&'a InputDevice>
where
    'a: 'b,
{
    devices
        .into_iter()
        .filter(|d| {
            let mut condition = true;
            for p in pat {
                if !d.name.contains(p) {
                    condition = false;
                    break;
                }
            }

            condition
        })
        .collect()
}

// fn find_device<'a>(devices: &'a [InputDevice], pat: &[String]) -> Option<&'a String> {
//     let dev = devices.iter().find(|d| {
//         let mut condition = true;
//         for p in pat {
//             if !d.name.contains(p) {
//                 condition = false;
//                 break;
//             }
//         }
//
//         condition
//     });
//
//     if dev.is_some() {
//         return dev.unwrap().handlers.iter().find(|h| h.contains("event"));
//     }
//
//     None
// }

// fn query_devices(devices: Vec<InputDevice>, )
