# `HX711` examples

> Examples for the platform agnostic driver to interface with the HX711 (load cell amplifier and 24 bit ADC)

## `linux-embedded-hal` (Raspberry Pi)

Connect `DOUT` to GPIO23 (Pin 16) and `SD_CLK` to GPIO24 (Pin 18).

Cross build it with:

```
cargo build --target armv7-unknown-linux-musleabihf
```

then transfer to the Raspberry Pi. 
Start with a low nice value to make the timing better.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

