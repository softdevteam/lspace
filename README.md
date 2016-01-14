# lspace
L-Space presentation system in Rust

## Requirements
Currently, L-Space requires the use of [Rust nightlies](https://www.rust-lang.org/downloads.html),
rather than a stable version. Please install the latest nightly before attempting to build.

## Building the L-Space viewer tool for Eco
Please run:
```cargo test --release```

The JSON viewer tool is an example program, so it is only built as part of the test suite. It also needs to
be built in release mode.

Eco looks for the release version, not the debug version that is built with:
```cargo build```
