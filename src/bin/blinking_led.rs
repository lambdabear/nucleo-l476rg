#![no_main]
#![no_std]

use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::{stm32, delay::Delay};
use nucleo_l476rg as _; // global logger + panicking-behavior + memory layout
use nucleo_l476rg::led;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Blinking LED...");
    
    let p = stm32::Peripherals::take().expect("Take peripherals error");
    let cp = stm32::CorePeripherals::take().expect("Take core peripherals error");
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut pwr = p.PWR.constrain(&mut rcc.apb1r1);
    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let pa5 = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut led = led::Led::new(pa5);
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.on();
        delay.delay_ms(500_u16);

        led.off();
        delay.delay_ms(500_u16);
    }
}

