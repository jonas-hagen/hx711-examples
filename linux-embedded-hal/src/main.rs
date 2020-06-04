
#[macro_use(block)]
extern crate nb;

use embedded_hal::blocking::delay::DelayUs;

extern crate linux_embedded_hal as hal;
extern crate hx711;

use std::{thread, time};
use hx711::Hx711;
use hal::Pin;
use hal::sysfs_gpio::Direction;

struct NoDelay();

impl NoDelay {
    pub fn new() -> Self {
        NoDelay()
    }
}

impl DelayUs<u32> for NoDelay {
    /// No delay, linux gpio sysfs is slow enough.
    fn delay_us(&mut self, _us: u32) {
    }
}


fn main() {
    println!("Using HX711 with DOUT on GPIO23 and PD_SCK on GPIO24");
    let dout = Pin::new(23);   // Header pin 16
    let pd_sck = Pin::new(24); // Header pin 18

    dout.export().unwrap();
    pd_sck.export().unwrap();
    while !dout.is_exported() {};
    while !pd_sck.is_exported() {};

    dout.set_direction(Direction::In).unwrap();
    pd_sck.set_direction(Direction::Low).unwrap();

    let mut hx711 = Hx711::new(NoDelay::new(), dout, pd_sck).unwrap();

    block!(hx711.retrieve()).unwrap();
    let mut zero_value: f32 = 0.0;
    for i in 0..20 {
        let reading = block!(hx711.retrieve()).unwrap() as f32;
        println!("{:>2}: {}", i, reading);
        zero_value += reading;
    }
    zero_value /= 20.0;

    println!("Tara: {}", zero_value);

    let n = 5;
    loop {
        let mut value: f32 = 0.0;
        for _ in 0..n {
            let reading = block!(hx711.retrieve()).unwrap() as f32;
            value += reading;
        }
        value /= n as f32;
        println!("{:>8}  {:>8}", value as i32, (value-zero_value) as i32);
        thread::sleep(time::Duration::from_millis(10));
    }
}
