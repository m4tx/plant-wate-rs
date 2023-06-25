use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct GpioId(u8);

impl Display for GpioId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GPIO{}", self.0)
    }
}

impl GpioId {
    #[inline]
    pub const fn new(id: u8) -> Self {
        Self(id)
    }

    #[inline]
    pub const fn value(&self) -> u8 {
        self.0
    }
}

pub const GPIO_0: GpioId = GpioId::new(0);
pub const GPIO_1: GpioId = GpioId::new(1);
pub const GPIO_2: GpioId = GpioId::new(2);
pub const GPIO_3: GpioId = GpioId::new(3);
pub const GPIO_4: GpioId = GpioId::new(4);
pub const GPIO_5: GpioId = GpioId::new(5);
pub const GPIO_6: GpioId = GpioId::new(6);
pub const GPIO_7: GpioId = GpioId::new(7);
pub const GPIO_8: GpioId = GpioId::new(8);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AnalogValue(u16);

impl Display for AnalogValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnalogValue({} mV)", self.0)
    }
}

impl AnalogValue {
    #[inline]
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn value(&self) -> u16 {
        self.0
    }
}

pub trait AnalogInput {
    fn get_value(&mut self) -> AnalogValue;
}

pub trait DigitalOutput {
    fn set_high(&mut self);
    fn set_low(&mut self);
}

pub trait Microcontroller {
    type AnalogInput: AnalogInput;
    type DigitalOutput: DigitalOutput;

    fn wait(&self, duration: Duration);
    fn get_analog_input(&mut self, id: GpioId) -> Self::AnalogInput;
    fn get_digital_output(&mut self, id: GpioId) -> Self::DigitalOutput;
}
