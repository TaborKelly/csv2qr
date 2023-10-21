# csv2qr

## Background

My friend told me that they needed to generate some QR codes. Naturally I wrote them a command line tool in Rust.

## Prerequisites

Install cargo with [rustup](https://rustup.rs/). Then just install `csv2qr` with `cargo add csv2qr`. `rustup` should have setup your `PATH` correctly but on Windows you might have to reboot or something. IDK, I don't use Windows.

## Usage

Usage is meant to be pretty self explanatory and `csv2qr` includes command line help. The only argument that you need to specify is a CSV file where the first column is a "title" and the second column is the value to encode into the QR. There is an example file [here](https://raw.githubusercontent.com/TaborKelly/csv2qr/main/example/example.csv).

```
$ csv2qr -h
csv2qr 0.2.0

A simple command line tool for generating QR codes from a CSV file.

USAGE:
    csv2qr [OPTIONS] <CSV_PATH> [OUTPUT_PATH]

ARGS:
    <CSV_PATH>       CSV file to parse
    <OUTPUT_PATH>    Output directory [default: .]

OPTIONS:
    -d, --debug                Turn on debug output
        --ecc <ECC>            ECC level (low, medium, quartile, or high) [default: medium]
    -h, --help                 Print help information
    -n, --no-pdf               Do not generate the final PDF document, only the intermediate PNG.
                               This will enable save-intermediate automatically
    -s, --save-intermediate    Do not delete the intermediate PNG of the QR code
    -V, --version              Print version information
```

`csv2qr` will created a PDF with the titled/labeled QR code for each row in the CSV.
Eg,
```
$ mkdir /tmp/csv2qr
$ csv2qr example/example.csv /tmp/csv2qr
$ ls -l /tmp/csv2qr
total 7856
-rw-rw-r-- 1 myuser myuser 2008838 Oct 20 17:35 Hack_the_planet.pdf
-rw-rw-r-- 1 myuser myuser 2009061 Oct 20 17:35 I_am_not_a_martyr_I%27m_a_problem.pdf
-rw-rw-r-- 1 myuser myuser 2008905 Oct 20 17:35 Prodigy_-_Mind_Fields.pdf
-rw-rw-r-- 1 myuser myuser 2008892 Oct 20 17:35 This_is_what_I_do.pdf
```

## Special thanks

This is just some simple glue code that stitched other Rust crates together. The real magic happens in the [genpdf](https://crates.io/crates/genpdf) and [qrcode-generator](https://crates.io/crates/qrcode-generator) crates. Tests were made possible by the [bardecoder](https://crates.io/crates/bardecoder) and [image](https://crates.io/crates/image) crate. I also included the [Calling Code font](https://github.com/RookAndPawn/text-to-png/blob/main/text-to-png/src/resources/CallingCode-Regular.ttf) from the source code of the [text-to-png](https://crates.io/crates/text-to-png) crate.

To the best of my knowledge it's all pure Rust. I have tested with Rust 1.66.1 on Kubuntu 22.04 and Rust 1.73.0 on Windows 10. It will _probably_ run on other platforms too.
