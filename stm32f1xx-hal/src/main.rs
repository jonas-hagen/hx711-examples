#![no_std]
#![no_main]

extern crate panic_itm;

// Core
use cortex_m::{iprintln, Peripherals};
use cortex_m_rt::entry;

// Device
use hx711;
use nb::block;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let mut cp = Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

    // Debug out via ITM
    // Make sure to enable ITM via SWD/JTAG using the debug probe
    // otherwise, code blocks here.
    // See: https://github.com/rust-embedded/cortex-m/issues/74
    let stim = &mut cp.ITM.stim;
    iprintln!(&mut stim[1], "Hello!");

    // Configure the hx711 load cell driver:
    //
    // | HX  | dout   -> PA6 | STM |
    // | 711 | pd_sck <- PA7 | 32  |
    //
    let dout = gpioa.pa6.into_floating_input(&mut gpioa.crl);
    let pd_sck = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    let mut hx711 = hx711::Hx711::new(dout, pd_sck);

    const N: i32 = 8;
    let mut val: i32 = 0;

    // Obtain the tara value
    for _ in 0..N {
        val += block!(hx711.retrieve()).unwrap();
    }
    let tara = val / N;
    iprintln!(&mut stim[1], "Tara:   {:>10}", tara);

    loop {
        // Measurement loop
        val = 0;
        for _ in 0..N {
            val += block!(hx711.retrieve()).unwrap();
        }
        let weight = val / N - tara;
        iprintln!(&mut stim[1], "Weight: {:>10}", weight);
    }
}
