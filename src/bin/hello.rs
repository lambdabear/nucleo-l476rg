#![no_main]
#![no_std]

use nucleo_l476rg as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    nucleo_l476rg::exit()
}
