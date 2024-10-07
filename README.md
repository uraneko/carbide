<h1 align="center">
    logius
</h1>

[<img alt="github" src="https://img.shields.io/badge/github-uraneko.logius-A5915F?style=for-the-badge&logo=github&labelColor=3a3a3a" height="25">](https://github.com/uraneko/logius) 
[<img alt="crates.io" src="https://img.shields.io/crates/v/logius.svg?style=for-the-badge&color=E40046&logo=rust&labelColor=3a3a3a" height="25">](https://crates.io/crates/logius) 
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-logius-495c9f?style=for-the-badge&logo=docsdotrs&labelColor=3a3a3a" height="25">](https://docs.rs/logius) 
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/uraneko/ragout/rust-ci.yml?branch=main&style=for-the-badge&labelColor=3a3a3a" height="25">](https://github.com/uraneko/logius/actions?query=branch%3Amain)
[<img alt="license" src="https://img.shields.io/github/license/uraneko/logius?style=for-the-badge&labelColor=3a3a3a&color=ECD53F" height="25">](https://github.com/uraneko/logius/blob/main/LICENSE)

logius is a local keylogger for input devices.

> [!CAUTION]
> Under no circumstances should anyone attempt to make use of this project for nefarious, malicious or ILLEGAL activities. Not a single contributor of this project shall be made to bear any degree of responsibility for said behavior.


## State
This project is still in development and doesn't have a working 0.1.0 version yet.

## Support 
Current support is only for linux.

## Features 
✓ list devices:  list all available input devices

✓ query devices: query for input devices by patterns in the devices names

✓ listen to devices: listen on to an input devices input_event raw input data

- decode input_event: correctly decoding the input buffer bytes into an input_event struct.

✓ log data type: either log received input as raw bytes buffer or as decoded input_event structs

! log data location: either log data to the terminal output or to a specific log file.

> [!CAUTION]
> Although most features are implemented, the project is in a buggy state and will take a little more to be functional.

<br>
✗ not yet implemented 

~ not yet implemented, low priority.

- work in progress

✓ implemented 

! implemented but buggy

## Installation
## cargo

> [!IMPORTANT] 
> This is not yet implemented.

```bash 
cargo install logius --locked 
```

## From Source
```bash 
git clone https://github.com/uraneko/logius
cd logius
cargo build -r --locked
# binary should be found under ./target/release/logius
```

## Examples
## terminal output 
```bash
# assuming you have an input device called "CUST0001:00 04F3:30AA Mouse"
logius -b "30AA Mouse" -e -t
# this logs decoded input_events from this specific mouse device to the terminal
```

> [!IMPORTANT] 
> Follows the [SemVer Spec](https://semver.org/) versioning scheme.
> Until the crate hits version 1.0.0, there are no rules, nonetheless, I'll try to make sense.

