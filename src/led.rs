use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::gpio::{gpioa::PA5, Output, PushPull};

pub type LD2 = PA5<Output<PushPull>>;

pub struct Led(LD2);

impl Led {
    pub fn new(ld2: LD2) -> Self {
        Led(ld2)
    }

    pub fn off(&mut self) {
        self.0.set_low().ok();
    }

    pub fn on(&mut self) {
        self.0.set_high().ok();
    }
}