use crate::devices;

pub struct List {
    // works only with the filter (f) option
    strict: bool,
    // prints the name and anything else turned on amongst the reductive options
    name_reductive: bool,
    // prints the event file name and anything else turned on amongst the reductive options
    event_reductive: bool,
    // applys filters by name to the device list
    filters: Option<Vec<String>>,
}

impl List {
    pub fn new(arg: &str, args: std::env::Args) -> Self {
        Self {
            strict: arg.contains('s'),
            name_reductive: arg.contains('n'),
            event_reductive: arg.contains('e'),
            filters: if arg.contains('f') {
                Some(args.collect::<Vec<String>>())
            } else {
                None
            },
        }
    }

    pub fn list(self) {
        let mut devices = devices::get_devices();
        if self.filters.is_some() {
            match self.strict {
                true => {
                    devices = devices::into_filter_devices_strict(devices, self.filters.unwrap());
                }
                false => devices = devices::into_filter_devices(devices, self.filters.unwrap()),
            }
        }

        match [self.name_reductive, self.event_reductive] {
            [true, true] => {
                devices
                    .iter()
                    .map(|d| (d.name(), d.event()))
                    .inspect(|(n, e)| println!("[\n    name: '{}',\n    event: '{}'\n]", n, e))
                    .count();
            }
            [false, false] => println!("{:#?}", devices),
            [true, false] => println!(
                "{:#?}",
                devices.iter().map(|d| d.name()).collect::<Vec<&str>>()
            ),
            [false, true] => println!(
                "{:#?}",
                devices.iter().map(|d| d.event()).collect::<Vec<&str>>()
            ),
        }
    }
}
