# `HX711` examples

> Examples for the platform agnostic driver to interface with the HX711 (load cell amplifier and 24 bit ADC)

## `stm32f1xx-hal` (STM32F103C8, Blue Pill)

Connect HX711 `DOUT` to PA6 and `SD_CLK` to PA7.

Logging is provided by USART1: TX on PA9 (no RX).

The cargo configuration is included in `.cargo/config`, depending on your system, you will need to adjust the linker name.

If using the SEGGER JLink, just run:

```
         > cd hx711-examples/stm32f1xx-hal
         > cargo build
terminalA> /opt/SEGGER/JLink/JLinkGDBServer -device STM32F103C8 -if SWD
terminalB> arm-none-eabi-gdb -x jlink.gdb target/thumbv7m-none-eabi/debug/firmware
terminalC> picocom -b 9600 --imap lfcrlf /dev/ttyUSB0 # you will see weight values here
```

## `linux-embedded-hal` (Raspberry Pi)

Since the HX711 driver uses pin toggling to implement the custom communication protocol, the reading can be interrupted by the kernel. Thus, the RaspberryPi example should be run with isolated CPUs or with a realtime linux scheduler. (Sorry, no instructions yet.)

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

