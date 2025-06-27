fn read(arg: &str, args: std::env::Args) {}

pub struct Read {
    save_output: bool,
    raw_events: bool,
    print_output: bool,
    event_num: Option<u8>,
}

impl Read {
    pub fn new(arg: &str, mut args: std::env::Args) -> Self {
        Self {
            save_output: arg.contains('s'),
            raw_events: arg.contains('r'),
            print_output: !arg.contains('p'),
            event_num: arg
                .contains('e')
                .then(|| args.next().map(|evnum| evnum.parse::<u8>().unwrap()))
                .unwrap(),
        }
    }

    pub fn read(self) {
        let event = format!("event{}", self.event_num.unwrap_or(0));

        crate::input_event::read(&event)
    }
}
