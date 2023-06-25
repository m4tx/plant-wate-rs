use std::cell::{RefCell};
use std::collections::HashMap;

use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;
use crate::uc::{AnalogInput, AnalogValue, DigitalOutput, GpioId, Microcontroller};

#[derive(Debug)]
pub struct MockDigitalOutput {
    id: GpioId,
    action_log: ActionLog,
}

impl DigitalOutput for MockDigitalOutput {
    fn set_high(&mut self) {
        self.action_log.add(MockMicrocontrollerAction::DigitalGpioHigh(self.id));
    }

    fn set_low(&mut self) {
        self.action_log.add(MockMicrocontrollerAction::DigitalGpioLow(self.id));
    }
}

impl MockDigitalOutput {
    fn new(id: GpioId, action_log: ActionLog) -> Self {
        Self {
            id,
            action_log,
        }
    }
}

#[derive(Debug)]
pub struct MockAnalogInput {
    id: GpioId,
    value: Rc<RefCell<AnalogValue>>,
    action_log: ActionLog,
}

impl AnalogInput for MockAnalogInput {
    fn get_value(&mut self) -> AnalogValue {
        let value = *(*self.value).borrow();
        self.action_log.add(MockMicrocontrollerAction::AnalogGpioGetValue(self.id, value));
        value
    }
}

impl MockAnalogInput {
    fn new(id: GpioId, value: Rc<RefCell<AnalogValue>>, action_log: ActionLog) -> Self {
        Self {
            id,
            value,
            action_log,
        }
    }
}

#[derive(Debug)]
enum MockGpio {
    AnalogInput(Rc<RefCell<AnalogValue>>),
    DigitalOutput,
}

#[derive(Debug)]
pub struct MockMicrocontroller {
    action_log: ActionLog,
    gpio: HashMap<GpioId, MockGpio>,
}

impl Microcontroller for MockMicrocontroller {
    type AnalogInput = MockAnalogInput;
    type DigitalOutput = MockDigitalOutput;

    fn wait(&self, duration: Duration) {
        self.action_log.add(MockMicrocontrollerAction::Wait(duration));
    }

    fn get_analog_input(&mut self, id: GpioId) -> Self::AnalogInput {
        self.check_gpio_none(id);
        let value = Rc::new(RefCell::new(AnalogValue::new(0)));
        self.gpio.insert(id, MockGpio::AnalogInput(value.clone()));
        self.action_log.add(MockMicrocontrollerAction::GpioSetAsAnalogInput(id));

        MockAnalogInput::new(id, value, self.action_log.clone())
    }

    fn get_digital_output(&mut self, id: GpioId) -> Self::DigitalOutput {
        self.check_gpio_none(id);
        self.gpio.insert(id,MockGpio::DigitalOutput);
        self.action_log.add(MockMicrocontrollerAction::GpioSetAsDigitalOutput(id));

        MockDigitalOutput::new(id, self.action_log.clone())
    }
}

impl MockMicrocontroller {
    pub fn new() -> Self {
        Self {
            action_log: ActionLog::new(),
            gpio: Default::default(),
        }
    }

    fn check_gpio_none(&self, id: GpioId) {
        if self.gpio.contains_key(&id) {
            panic!("{} already enabled!", id);
        }
    }

    pub fn set_analog_value(&self, id: GpioId, value: AnalogValue) {
        let gpio = &self.gpio[&id];
        if let MockGpio::AnalogInput(val) = gpio {
            *val.deref().borrow_mut() = value;
        } else {
            panic!("{} is not analog input!", id);
        }
    }

    pub fn actions(&self) -> Vec<MockMicrocontrollerAction> {
        self.action_log.actions()
    }
}

#[derive(Debug, Clone)]
struct ActionLog {
    actions: Rc<RefCell<Vec<MockMicrocontrollerAction>>>,
}

impl ActionLog {
    fn new() -> Self {
        Self {
            actions: Default::default(),
        }
    }

    fn add(&self, action: MockMicrocontrollerAction) {
        self.actions.borrow_mut().push(action);
    }

    fn actions(&self) -> Vec<MockMicrocontrollerAction> {
        self.actions.deref().borrow().clone()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MockMicrocontrollerAction {
    Wait(Duration),
    GpioSetAsDigitalOutput(GpioId),
    GpioSetAsAnalogInput(GpioId),
    DigitalGpioHigh(GpioId),
    DigitalGpioLow(GpioId),
    AnalogGpioGetValue(GpioId, AnalogValue),
}
