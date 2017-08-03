# `libbeaglebone` &ensp;
[![Build Status](https://travis-ci.org/ekmecic/libbeaglebone.svg?branch=master)](https://travis-ci.org/ekmecic/libbeaglebone)
[![](https://img.shields.io/crates/v/libbeaglebone.svg)](https://crates.io/crates/libbeaglebone)
[![](https://docs.rs/libbeaglebone/badge.svg)](https://docs.rs/libbeaglebone)

`libbeaglebone` is a WIP Rust library that aims to provide a friendly interface for common embedded functionality for the BeagleBone family of devices.

## Features
libbeaglebone will be feature-complete when the following interfaces are all implemented:
- [x] GPIO
- [x] PWM
- [x] ADC
- [x] UART
- [x] SPI
- [x] I2C

Note: `libbeaglebone` is still in development and is alpha quality software. Don't trust your life (or anyone else's on this code!)

## Usage
Simply add `libbeaglebone = "0.5.0"` under `[dependencies]` in your `Cargo.toml` and you're all set.

## Examples
There are example programs available in the `examples/` directory.
You can compile them by running:
```bash
cargo build --example blinker
```
substituting `blinker` for the filename of the example you'd like to compile.
Once compilation is complete, you can find the binary in the `target/debug/examples` directory.

## Acknowlegements
I'd like to thank (in no particular order):
* Trevor Woerner for his mentoring
* The BeagleBoard organization for their help
* Google for their sponsorship of the project

## License
`libbeaglebone` is licensed under version 3 of the GPL license.

## See also
* [rust-sysfs-gpio](https://github.com/rust-embedded/rust-sysfs-gpio)
* [rust-sysfs-pwm](https://github.com/rust-embedded/rust-sysfs-pwm)
* [rust-spidev](https://github.com/rust-embedded/rust-spidev)
* [rust-i2cdev](https://github.com/rust-embedded/rust-i2cdev)
* [gpio-rs](https://github.com/mbr/gpio-rs)
* [Cylus](https://github.com/Vikaton/cylus)
* [Cuprum Pi](https://github.com/inre/cupi)
