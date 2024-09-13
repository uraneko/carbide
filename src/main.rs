use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::thread::{spawn, JoinHandle};

// mod keys;
mod builder;
mod input_event;

use builder::ProgramBuilder;

// use keys::key;

//  how to decode input event bytes into some key event can be found at
//  "https://www.kernel.org/doc/Documentation/input/input.txt"

const INPUT_DEVICES: &str = "/proc/bus/input/devices";

impl ProgramBuilder {
    /// parses the fields of this instance of the program builder
    /// consuming it and running the program
    pub fn run(self) -> Option<Vec<JoinHandle<()>>> {
        let mut writer = std::io::stdout().lock();

        let devices = parse_devices();
        match self.method {
            // print all
            0 => {
                // FIXME: use writer.write instead
                println!("{:#?}", devices);
            }
            // query devices
            2 => {
                let devices = self
                    .patterns
                    .into_iter()
                    .map(|p| filter_devices(&devices, &p))
                    .flatten()
                    .collect::<Vec<&InputDevice>>();
                println!("{:#?}", devices);
            }
            // bind readers to files
            3 => {
                let handles = self
                    .patterns
                    .into_iter()
                    .map(|p| {
                        let e = find_device(&devices, &p);
                        e
                    })
                    .filter(|eo| eo.is_some())
                    .map(|eo| eo.unwrap().to_string())
                    .map(|e| bind_logger(e))
                    .collect::<Vec<JoinHandle<()>>>();

                println!("{:?}", handles);

                return Some(handles);
            }
            // print help message
            1 => {
                help(&mut writer);
            }
            _ => unreachable!(),
        }

        None
    }
}

fn help(writer: &mut std::io::StdoutLock) {
    _ = writer.write(
        b"
running the program without any arguments prints all the input devices found in
'/proc/bus/input/devices' then exits
-q query for some pattern amongst the available devices

-b bind a reader to an eventx file and start listening
    if neither -r nor -d are given with this then -d is assumed
    if neither -l nor -t are provided then -t is assumed

-r reader outputs raw bytes data

-d reader outputs decoded input_event struct data

-l|-lc|-lo <file> log the read data to an output log file
    -lo will overwrite the provided log file if it exists 
    -lc will append to the file 
    if -l is provided alone then -lc is assumed

-t print the read data to the terminal stdout

-h print this help message
",
    )
}

fn main() {
    // decode_bytes(&[28]);
    // return;
    let builder = match parse_args() {
        Ok(builder) => builder,
        Err(e) => panic!(
            "couldn't parse args into builder, aborting program\n{:?}",
            e
        ),
    };

    println!("{:?}", builder);

    let handles = builder.run();

    if let Some(handles) = handles {
        handles.into_iter().for_each(|h| h.join().unwrap());
    }
}

fn parse_args() -> Result<ProgramBuilder, Error> {
    let mut builder = ProgramBuilder::new();

    let mut args = std::env::args();
    args.next();

    if args.len() == 0 {
        return Ok(builder);
    }

    let mut pats = vec![];

    while let Some(arg) = args.next() {
        match arg.trim() {
            "-q" => {
                builder.method_mut(2);
                builder.push(pats.drain(..).collect());
            }
            "-b" => {
                builder.method_mut(3);
                builder.push(pats.drain(..).collect());
            }
            "-h" => {
                builder.method_mut(1);
                return Ok(builder);
            }
            "-r" => builder.data_mut(2),
            "-d" => builder.data_mut(1),
            "-t" => builder.output_mut(1),
            "-l" | "-lc" | "-lo" => {
                builder.output_mut(2);
                let fp =
                    match args.next() {
                        Some(fp) => fp,
                        None => return Err(Error::other(
                            "the -l log file param was turned on, yet no file path was provided",
                        )),
                    };

                let fp = Path::new(&fp);

                let mut oo = OpenOptions::new();
                oo.write(true).create(true);
                match arg.trim() {
                    "-l" | "-lc" => oo.append(true),
                    "-lo" => oo.truncate(true),
                    _ => unreachable!(),
                };

                // may panic on unwrap,
                // read ['https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html#method.open'] Errors section for more
                builder.log_file_mut(oo.open(fp).unwrap());
            }
            pat => pats.push(pat.to_string()),
        }
    }

    if !pats.is_empty() {
        builder.push(pats);
    }

    // assert!(0 && builder.output == 0 || builder.method == );

    Ok(builder)
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

fn filter_devices<'a, 'b>(devices: &'a [InputDevice], pat: &'b [String]) -> Vec<&'a InputDevice>
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
    const IES_SIZE: usize = 24;
    let mut buf = [0u8; IES_SIZE];

    let refresh = 1000 / 60;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(refresh));

        gpf.read(&mut buf).unwrap();
        println!("{:?}", buf);
    }
}

fn find_device<'a>(devices: &'a [InputDevice], pat: &[String]) -> Option<&'a String> {
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

fn bind_logger(event: String) -> JoinHandle<()> {
    spawn(move || log_input(&event))
}
