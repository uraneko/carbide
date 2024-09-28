mod new;

mod devices;
mod format;
mod input__event;
mod log;

// TODO: 2 things left to do
// fix query and bind bugs
// finish decoding the input event bytes into an input_event struct instance

use format::{codes::code, types::type_};

//  how to decode input event bytes into some key event can be found at
//  "https://www.kernel.org/doc/Documentation/input/input.txt"

fn main() {
    // decode_bytes(&[28]);
    // return;
    let handles = new::run();

    if let Some(handles) = handles {
        handles.into_iter().for_each(|h| h.join().unwrap());
    }
}
