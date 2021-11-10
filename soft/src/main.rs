#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32l4::stm32l412;

#[entry]
fn main() -> ! {
    let peripherals = stm32l412::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    gpioa.odr.modify(|_, w| w.odr0().set_bit());
    loop {}
}