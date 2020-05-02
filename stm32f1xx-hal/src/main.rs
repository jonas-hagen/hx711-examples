#![no_std]
#![no_main]
#![feature(unwrap_infallible)]

extern crate panic_itm;

// Core
use cortex_m_rt::entry;

// Device
use hx711::Hx711;
use nb::block;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
};

use core::fmt::Write;


#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut flash = p.FLASH.constrain();

    //let clocks = rcc.cfgr.freeze(&mut flash.acr); // slow clock
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .pclk1(36.mhz())
        .freeze(&mut flash.acr);

    // Configure the hx711 load cell driver:
    //
    // | HX  | dout   -> PA6 | STM |
    // | 711 | pd_sck <- PA7 | 32  |
    //
    let dout = gpioa.pa6.into_floating_input(&mut gpioa.crl);
    let pd_sck = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    let mut hx711 = Hx711::new(dout, pd_sck).into_ok();

    // USART 1
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    let tx_pin = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx_pin = gpioa.pa10;

    let serial = Serial::usart1(
        p.USART1,
        (tx_pin, rx_pin),
        &mut afio.mapr,
        Config::default().baudrate(9600.bps()),
        clocks,
        &mut rcc.apb2,
    );
    let (mut tx, _rx) = serial.split();

    const N: i32 = 8;
    let mut val: i32 = 0;

    // Obtain the tara value
    writeln!(tx, "Obtaining tara ...").unwrap();
    for _ in 0..N {
        val += block!(hx711.retrieve()).into_ok();
    }
    let tara = val / N;
    writeln!(tx, "Tara:   {}", tara).unwrap();

    loop {
        // Measurement loop
        val = 0;
        for _ in 0..N {
            val += block!(hx711.retrieve()).into_ok();
        }
        let weight = val / N - tara;
        writeln!(tx, "{}", weight).unwrap();
    }
}
