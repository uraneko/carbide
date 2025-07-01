<h1 align="center">
    carbide
</h1>

[<img alt="github" src="https://img.shields.io/badge/github-uraneko.carbide-A5915F?style=for-the-badge&logo=github&labelColor=3a3a3a" height="25">](https://github.com/uraneko/carbide) 
[<img alt="crates.io" src="https://img.shields.io/crates/v/carbide.svg?style=for-the-badge&color=E40046&logo=rust&labelColor=3a3a3a" height="25">](https://crates.io/crates/carbide) 
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-carbide-495c9f?style=for-the-badge&logo=docsdotrs&labelColor=3a3a3a" height="25">](https://docs.rs/carbide) 
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/uraneko/ragout/rust-ci.yml?branch=main&style=for-the-badge&labelColor=3a3a3a" height="25">](https://github.com/uraneko/carbide/actions?query=branch%3Amain)
[<img alt="license" src="https://img.shields.io/github/license/uraneko/carbide?style=for-the-badge&labelColor=3a3a3a&color=ECD53F" height="25">](https://github.com/uraneko/carbide/blob/main/LICENSE)

carbide is a linux input event reader.

> [!CAUTION]
> Under no circumstances should anyone attempt to make use of this project for nefarious, malicious or ILLEGAL activities. Not a single contributor of this project shall be made to bear any degree of responsibility for said behavior.

##### Contents
* Features
* Installation
* Examples
* License
* Disclaimer 

### Features
* list & query devices by name patterns
* read devices input events 

```bash
# for more info, run 
crb -H 
```

### Installation
###### cargo

> [!IMPORTANT] 
> This is not yet implemented.

```bash 
cargo install carbide --locked 
```

###### From Source
```bash 
git clone https://github.com/uraneko/carbide
cd carbide
cargo build -r --locked
# binary should be found under ./target/release/crb
```

### Examples

###### terminal output 
```bash
# assuming you have an input device called "CUST0001:00 04F3:30AA Mouse"
crb -b "30AA Mouse" -e -t
# this logs decoded input_events from this specific mouse device to the terminal
```

More examples can be found <a href="examples">here</a>.

### MSRV

msrv is rustc/cargo 1.85.1
