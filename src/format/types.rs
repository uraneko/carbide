pub fn type_(bytes: u16) -> &'static str {
    match bytes {
        0x00 => stringify!(EV_SYN),
        0x01 => stringify!(EV_KEY),
        0x02 => stringify!(EV_REL),
        0x03 => stringify!(EV_ABS),
        0x04 => stringify!(EV_MSC),
        0x05 => stringify!(EV_SW),
        0x11 => stringify!(EV_LED),
        0x12 => stringify!(EV_SND),
        0x14 => stringify!(EV_REP),
        0x15 => stringify!(EV_FF),
        0x16 => stringify!(EV_PWR),
        0x17 => stringify!(EV_FF_STATUS),
        0x1f => stringify!(EV_MAX),
        0x20 => stringify!(EV_CNT),
        _ => panic!("badly decoded the input_event"),
    }
}
