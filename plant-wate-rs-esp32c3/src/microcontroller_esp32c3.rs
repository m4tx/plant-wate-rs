use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use esp_idf_hal::adc;
use esp_idf_hal::adc::{AdcDriver, ADC1};
use esp_idf_hal::gpio::{
    Gpio0, Gpio1, Gpio2, Gpio3, Gpio4, Gpio5, Gpio6, Gpio7, Gpio8, Output, PinDriver,
};
use esp_idf_hal::peripherals::Peripherals;
use plant_wate_rs_core::uc::{
    AnalogInput, AnalogValue, DigitalOutput, GpioId, Microcontroller, GPIO_0, GPIO_2,
};

pub enum AnalogInputEsp32c3Pin<'a> {
    Gpio0(adc::AdcChannelDriver<'a, Gpio0, adc::Atten11dB<ADC1>>),
}

pub struct AnalogInputEsp32c3<'a> {
    id: GpioId,
    pin: AnalogInputEsp32c3Pin<'a>,
    adc_driver: Rc<RefCell<AdcDriver<'a, ADC1>>>,
}

impl<'a> AnalogInputEsp32c3<'a> {
    pub fn new(
        id: GpioId,
        pin: AnalogInputEsp32c3Pin<'a>,
        adc_driver: Rc<RefCell<AdcDriver<'a, ADC1>>>,
    ) -> Self {
        Self {
            id,
            pin,
            adc_driver,
        }
    }
}

impl<'a> AnalogInput for AnalogInputEsp32c3<'a> {
    fn get_value(&mut self) -> AnalogValue {
        let value = match &mut self.pin {
            AnalogInputEsp32c3Pin::Gpio0(pin) => self.adc_driver.borrow_mut().read(pin).unwrap(),
        };

        AnalogValue::new(value)
    }
}

pub enum DigitalOutputEsp32c3Pin<'a> {
    Gpio2(PinDriver<'a, Gpio2, Output>),
}

pub struct DigitalOutputEsp32c3<'a> {
    pin: DigitalOutputEsp32c3Pin<'a>,
}

impl<'a> DigitalOutputEsp32c3<'a> {
    pub fn new(pin: DigitalOutputEsp32c3Pin<'a>) -> Self {
        Self { pin }
    }
}

impl<'a> DigitalOutput for DigitalOutputEsp32c3<'a> {
    fn set_high(&mut self) {
        match &mut self.pin {
            DigitalOutputEsp32c3Pin::Gpio2(pin) => {
                pin.set_high().unwrap();
            }
        }
    }

    fn set_low(&mut self) {
        match &mut self.pin {
            DigitalOutputEsp32c3Pin::Gpio2(pin) => {
                pin.set_low().unwrap();
            }
        }
    }
}

pub struct MicrocontrollerEsp32c3<'a> {
    gpio_0: Cell<Option<Gpio0>>,
    gpio_1: Cell<Option<Gpio1>>,
    gpio_2: Cell<Option<Gpio2>>,
    gpio_3: Cell<Option<Gpio3>>,
    gpio_4: Cell<Option<Gpio4>>,
    gpio_5: Cell<Option<Gpio5>>,
    gpio_6: Cell<Option<Gpio6>>,
    gpio_7: Cell<Option<Gpio7>>,
    gpio_8: Cell<Option<Gpio8>>,
    adc_driver_1: Rc<RefCell<AdcDriver<'a, ADC1>>>,
}

impl<'a> Microcontroller for MicrocontrollerEsp32c3<'a> {
    type AnalogInput = AnalogInputEsp32c3<'a>;
    type DigitalOutput = DigitalOutputEsp32c3<'a>;

    fn wait(&self, duration: Duration) {
        thread::sleep(duration);
    }

    fn get_analog_input(&mut self, id: GpioId) -> Self::AnalogInput {
        let pin = if id == GPIO_0 {
            let pin = self.gpio_0.take().expect("GPIO0 already taken");
            AnalogInputEsp32c3Pin::Gpio0(
                adc::AdcChannelDriver::<Gpio0, adc::Atten11dB<ADC1>>::new(pin).unwrap(),
            )
        } else {
            panic!("{} not supported as analog input", id)
        };

        AnalogInputEsp32c3::new(id, pin, self.adc_driver_1.clone())
    }

    fn get_digital_output(&mut self, id: GpioId) -> Self::DigitalOutput {
        let pin = if id == GPIO_2 {
            let pin = self
                .gpio_2
                .take()
                .unwrap_or_else(|| panic!("{} already taken", id));
            DigitalOutputEsp32c3Pin::Gpio2(PinDriver::output(pin).unwrap())
        } else {
            panic!("{} not supported as digital output", id)
        };

        DigitalOutputEsp32c3::new(pin)
    }
}

impl<'a> MicrocontrollerEsp32c3<'a> {
    pub fn new() -> Self {
        let peripherals = Peripherals::take().unwrap();
        let adc_driver_1: AdcDriver<'_, ADC1> = AdcDriver::new(
            peripherals.adc1,
            &adc::config::Config::new().calibration(true),
        )
        .unwrap();

        Self {
            gpio_0: Cell::new(Some(peripherals.pins.gpio0)),
            gpio_1: Cell::new(Some(peripherals.pins.gpio1)),
            gpio_2: Cell::new(Some(peripherals.pins.gpio2)),
            gpio_3: Cell::new(Some(peripherals.pins.gpio3)),
            gpio_4: Cell::new(Some(peripherals.pins.gpio4)),
            gpio_5: Cell::new(Some(peripherals.pins.gpio5)),
            gpio_6: Cell::new(Some(peripherals.pins.gpio6)),
            gpio_7: Cell::new(Some(peripherals.pins.gpio7)),
            gpio_8: Cell::new(Some(peripherals.pins.gpio8)),
            adc_driver_1: Rc::new(RefCell::new(adc_driver_1)),
        }
    }
}
