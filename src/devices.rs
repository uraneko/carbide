use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

const INPUT_DEVICES: &str = "/proc/bus/input/devices";

#[derive(Debug)]
pub(crate) struct InputDevice {
    i: DeviceId,
    n: String,
    p: String,
    s: String,
    u: Option<u8>,
    h: HashSet<String>,
    b: DeviceBitMaps,
}

#[derive(Debug)]
struct DeviceId {
    bus_type: u16,
    vendor: u16,
    product: u16,
    version: u16,
}

#[derive(Debug)]
struct DeviceBitMaps {
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
    pub(crate) fn name(&self) -> &str {
        &self.n
    }

    pub(crate) fn event(&self) -> Option<&String> {
        self.h.iter().find(|h| h.contains("event"))
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
        i: {
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

            println!("id: {:#?}", map);

            DeviceId {
                bus_type: map.remove("Bus").unwrap(),
                vendor: map.remove("Vendor").unwrap(),
                product: map.remove("Product").unwrap(),
                version: map.remove("Version").unwrap(),
            }
        },

        n: {
            println!("n");
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
            println!("p");
            let Some(mut phys) = s.next() else {
                panic!("input devices file gave bad data")
            };
            if !phys.starts_with("P: Phys=") {
                panic!("Phys chunk wasn't Phys chunk\nOr it was but doesn't start with 'P: Phys='")
            }
            phys.drain(8..).collect()
        },
        s: {
            println!("s");
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
            println!("u");
            let Some(mut uniq) = s.next() else {
                panic!("input devices file gave bad data")
            };
            if !uniq.starts_with("U: Uniq=") {
                panic!("Uniq chunk wasn't Uniq chunk\nOr it was but doesn't start with 'U: Uniq='")
            }
            uniq.drain(8..).collect::<String>().parse().ok()
        },
        h: {
            println!("h");
            let Some(handlers) = s.next() else {
                panic!("input devices file gave bad data")
            };
            if !handlers.starts_with("H: Handlers=") {
                panic!("Handlers chunk wasn't Handlers chunk\nOr it was but doesn't start with 'H: Handlers='")
            }

            handlers
                .replace("H: Handlers=", "")
                .split(' ')
                .map(|s| s.to_owned())
                .filter(|h| !h.is_empty())
                .collect::<HashSet<String>>()
        },

        b: {
            println!("b");
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

            println!("props: {:#?}\n    \"Key\": {:?},\n", map, k);

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
                if !d.n.contains(p) {
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

const HEX_A: &str = "10";
const HEX_B: &str = "11";
const HEX_C: &str = "12";
const HEX_D: &str = "13";
const HEX_E: &str = "14";
const HEX_F: &str = "15";

fn hex_decode(value: &str) -> Result<u64, std::io::Error> {
    if value.contains(|c: char| !c.is_ascii_digit() && !('a'..'f').contains(&c)) {
        return Err(std::io::Error::other("not a valid hex int"));
    }

    let [mut a, mut b, mut c, mut d, mut e, mut f]: [usize; 6] = [0; 6];
    value.chars().for_each(|ch| match ch {
        'a' => a += 1,
        'b' => b += 1,
        'c' => c += 1,
        'd' => d += 1,
        'e' => e += 1,
        'f' => f += 1,
        _ => (),
    });

    Ok(value
        .replacen('a', HEX_A, a)
        .replacen('b', HEX_B, b)
        .replacen('c', HEX_C, c)
        .replacen('d', HEX_D, d)
        .replacen('e', HEX_E, e)
        .replacen('f', HEX_F, f)
        .parse()
        .unwrap())
}
