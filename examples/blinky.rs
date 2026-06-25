#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{ rprintln, rtt_init_print};

use cw32l012_pac as pac;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    dp.sysctrl.ahben().modify(|_, w| {
        unsafe {
            w.key().bits(0x5A5A);
        }
        w.gpioc().set_bit()
    });

    dp.gpioc.analog().modify(|_, w| w.pin13().clear_bit());
    dp.gpioc.dir().modify(|_, w| w.pin13().clear_bit());
    dp.gpioc.opendrain().modify(|_, w| w.pin13().clear_bit());

    rprintln!("CW32L012 boot ok!");

    let mut led_high: bool = false;

    loop {
        if led_high {
            dp.gpioc.bsrr().write(|w| w.brr13().set_bit());
            rprintln!("LED ON!");
        } else {
            dp.gpioc.bsrr().write(|w| w.bss13().set_bit());
            rprintln!("LED OFF!");
        }

        led_high = !led_high;

        delay(50_000);
    }
    

}

fn delay(mut n:u32) {
    while n != 0{
        cortex_m::asm::nop();
        n -= 1;
    }
}

