#![no_std]
#![no_main]

extern crate panic_itm;

// Core
use cortex_m_rt::entry;
use cortex_m::{iprintln, Peripherals};

// Device
use stm32f1xx_hal::{
    prelude::*,
    pac,
    delay::Delay,
};
use hx711;

use nb::block;

use embedded_hal::digital::v1_compat::OldOutputPin;
use embedded_hal::digital::v1_compat::OldInputPin;

#[entry]
fn main() -> ! {
    let mut cp = Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    let stim = &mut cp.ITM.stim;
    iprintln!(&mut stim[1], "Hello, world from itm {} !", 1.0);

    let mut rcc = p.RCC.constrain();
    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

    let dout = gpioa.pa6.into_floating_input(&mut gpioa.crl);
    let pd_sck = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);

    let mut hx711 = hx711::Hx711::new(OldInputPin::from(dout), OldOutputPin::from(pd_sck));
    //hx711.enable(&mut delay).unwrap();


    loop {
        let val:i32 = block!(hx711.retrieve()).unwrap();
        iprintln!(&mut stim[1], "{}", val);
        delay.delay_ms(200_u16);
    }
}
