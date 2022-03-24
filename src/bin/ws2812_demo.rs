#![no_std]
#![no_main]

use cortex_m_rt::*;
use embedded_hal::spi::{Mode, Phase, Polarity};
use nucleo_l476rg as _;
use smart_leds::{gamma, hsv::hsv2rgb, hsv::Hsv, SmartLedsWrite, RGB8};
use stm32l4xx_hal::{delay::Delay, prelude::*, spi::Spi, stm32};
use ws2812_spi::Ws2812;

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut pwr = p.PWR.constrain(&mut rcc.apb1r1);

    // TRY the other clock configuration
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let clocks = rcc
        .cfgr
        .sysclk(80_u32.mhz())
        .pclk1(80_u32.mhz())
        .pclk2(80_u32.mhz())
        .freeze(&mut flash.acr, &mut pwr);

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);

    // let mut nss = gpiob
    //     .pb0
    //     .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut dc = gpiob
        .pb1
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    // The `L3gd20` abstraction exposed by the `f3` crate requires a specific pin configuration to
    // be used and won't accept any configuration other than the one used here. Trying to use a
    // different pin configuration will result in a compiler error.
    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    // nss.set_high();
    dc.set_low();

    let mut spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        MODE,
        3_u32.mhz(),
        // 100.kHz(),
        clocks,
        &mut rcc.apb2,
    );

    // nss.set_low();
    // let data = [0x3C];
    // spi.write(&data).unwrap();
    // spi.write(&data).unwrap();
    // spi.write(&data).unwrap();
    // nss.set_high();

    // when you reach this breakpoint you'll be able to inspect the variable `_m` which contains the
    // gyroscope and the temperature sensor readings
    // asm::bkpt();

    let mut ws = Ws2812::new(spi);
    const LED_NUM: usize = 60;
    let mut data = [RGB8::default(); LED_NUM];

    loop {
        for j in 0..256 {
            for i in 0..LED_NUM {
                // rainbow cycle using HSV, where hue goes through all colors in circle
                // value sets the brightness
                let hsv = Hsv {
                    hue: ((i * 3 + j) % 256) as u8,
                    sat: 255,
                    val: 100,
                };

                data[i] = hsv2rgb(hsv);
            }
            // before writing, apply gamma correction for nicer rainbow
            ws.write(gamma(data.iter().cloned())).unwrap();
            delay.delay_ms(10u8);
        }
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
