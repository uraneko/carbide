use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

const INPUT_DEVICES: &str = "/proc/bus/input/devices";

#[derive(Debug)]
pub struct InputDevice {
    i: DeviceId,
    n: String,
    p: String,
    s: String,
    u: Option<u8>,
    h: HashSet<String>,
    b: DeviceBitMaps,
}

#[derive(Debug)]
pub struct DeviceId {
    bus_type: u16,
    vendor: u16,
    product: u16,
    version: u16,
}

#[derive(Debug)]
pub struct DeviceBitMaps {
    prop: Option<u64>,
    ev: Option<u64>,
    key: Option<String>,
    rel: Option<u64>,
    abs: Option<u64>,
    msc: Option<u64>,
    led: Option<u64>,
    snd: Option<u64>,
    ff: Option<u64>,
    sw: Option<u64>,
}

impl InputDevice {
    pub fn name(&self) -> &str {
        &self.n
    }

    pub fn event(&self) -> &str {
        self.h
            .iter()
            .find(|h| h.contains("event"))
            .map_or("_", |e| e)
    }
}

pub fn get_devices() -> Vec<InputDevice> {
    let mut f = File::open(INPUT_DEVICES).unwrap();
    let mut s = String::new();

    _ = File::read_to_string(&mut f, &mut s).unwrap();

    s.split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|dev| get_device(&dev))
        .collect()
}

pub fn get_device(device: &str) -> InputDevice {
    let mut s = device.split('\n').map(|s| s.to_owned());

    InputDevice {
        i: {
            // println!("i");
            let Some(id) = s.next() else {
                panic!("bad string")
            };
            if !id.starts_with("I: Bus=") {
                panic!("Id chunk wasn't Id chunk\nOr it was but doesn't start with 'I: Bus='")
            }
            let id = id[3..].split(' ').collect::<Vec<&str>>();

            let mut map = id
                .into_iter()
                .map(|s| {
                    let mut s = s.splitn(2, '=');
                    (
                        s.next().unwrap(),
                        u16::from_str_radix(
                            &s.next().unwrap().split_whitespace().collect::<String>(),
                            16,
                        )
                        .unwrap(),
                    )
                })
                .collect::<HashMap<&str, u16>>();

            DeviceId {
                bus_type: map.remove("Bus").unwrap(),
                vendor: map.remove("Vendor").unwrap(),
                product: map.remove("Product").unwrap(),
                version: map.remove("Version").unwrap(),
            }
        },

        n: {
            // println!("n");
            let Some(mut name) = s.next() else {
                panic!("input devices file gave bad data")
            };
            if !name.starts_with("N: Name=") {
                panic!("Name chunk wasn't Name chunk\nOr it was but doesn't start with 'N: Name='")
            }
            let mut name = name.drain(8..).collect::<String>();
            name.pop();
            name.remove(0);

            name
        },
        p: {
            // println!("p");
            let Some(mut phys) = s.next() else {
                panic!("input devices file gave bad data")
            };
            if !phys.starts_with("P: Phys=") {
                panic!("Phys chunk wasn't Phys chunk\nOr it was but doesn't start with 'P: Phys='")
            }
            phys.drain(8..).collect()
        },
        s: {
            // println!("s");
            let Some(mut sysfs) = s.next() else {
                panic!("input devices file gave bad data")
            };
            if !sysfs.starts_with("S: Sysfs=") {
                panic!(
                    "Sysfs chunk wasn't Sysfs chunk\nOr it was but doesn't start with 'S: Sysfs='"
                )
            }
            sysfs.drain(9..).collect()
        },
        u: {
            // println!("u");
            let Some(mut uniq) = s.next() else {
                panic!("input devices file gave bad data")
            };
            if !uniq.starts_with("U: Uniq=") {
                panic!("Uniq chunk wasn't Uniq chunk\nOr it was but doesn't start with 'U: Uniq='")
            }
            uniq.drain(8..).collect::<String>().parse().ok()
        },
        h: {
            // println!("h");
            let Some(handlers) = s.next() else {
                panic!("input devices file gave bad data")
            };
            if !handlers.starts_with("H: Handlers=") {
                panic!(
                    "Handlers chunk wasn't Handlers chunk\nOr it was but doesn't start with 'H: Handlers='"
                )
            }

            handlers
                .replace("H: Handlers=", "")
                .split(' ')
                .map(|s| s.to_owned())
                .filter(|h| !h.is_empty())
                .collect::<HashSet<String>>()
        },

        b: {
            // println!("b");
            let mut map = HashMap::new();

            let mut k: Option<String> = None;
            while let Some(prop) = s.next() {
                if !prop.starts_with("B: ") {
                    panic!(
                        "BitMap chunk wasn't BitMap chunk\nOr it was but doesn't start with 'B: '"
                    )
                }
                let p = &mut prop.replace("B: ", "");
                let mut p = p.split('=').map(|s| s.to_owned());

                let key = p.next().unwrap();
                if key == "KEY" {
                    k = p.next();
                    continue;
                }

                map.insert(
                    key,
                    u64::from_str_radix(
                        &p.next().unwrap().split_whitespace().collect::<String>(),
                        16,
                    )
                    .unwrap(),
                );
            }

            DeviceBitMaps {
                prop: map.remove("PROP"),
                ev: map.remove("EV"),
                key: k,
                rel: map.remove("REL"),
                abs: map.remove("ABS"),
                msc: map.remove("MSC"),
                led: map.remove("LED"),
                snd: map.remove("SND"),
                ff: map.remove("FF"),
                sw: map.remove("SW"),
            }
        },
    }
}

pub fn ignore_case(devices: &mut Vec<InputDevice>, pats: &mut Vec<String>) {
    devices.iter_mut().for_each(|d| d.n = d.n.to_lowercase());
    pats.iter_mut().for_each(|p| *p = p.to_lowercase());
}

pub fn into_filter_devices(devices: Vec<InputDevice>, pats: Vec<String>) -> Vec<InputDevice> {
    devices
        .into_iter()
        .filter(|d| {
            let name = d.name();
            pats.iter().any(|p| name.contains(p))
        })
        .collect()
}

pub fn into_filter_devices_strict(
    devices: Vec<InputDevice>,
    pats: Vec<String>,
) -> Vec<InputDevice> {
    devices
        .into_iter()
        .filter(|d| {
            let name = d.name();
            pats.iter().all(|p| name.contains(p))
        })
        .collect()
}

pub fn as_filter_devices<'a>(devices: &'a [InputDevice], pats: &[String]) -> Vec<&'a InputDevice> {
    devices
        .into_iter()
        .filter(|d| {
            let name = d.name();
            pats.iter().any(|p| name.contains(p))
        })
        .collect()
}
pub fn as_filter_devices_strict<'a>(
    devices: &'a [InputDevice],
    pats: &[String],
) -> Vec<&'a InputDevice> {
    devices
        .into_iter()
        .filter(|d| {
            let name = d.name();
            pats.iter().all(|p| name.contains(p))
        })
        .collect()
}
