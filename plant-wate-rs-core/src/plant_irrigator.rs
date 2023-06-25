use std::fmt::{Debug, Display, Formatter};
use std::time::Duration;

use log::info;

use crate::uc::{AnalogInput, AnalogValue, DigitalOutput, Microcontroller};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SensorCalibrationResult {
    min_value: AnalogValue,
    max_value: AnalogValue,
}

impl SensorCalibrationResult {
    #[inline]
    pub fn new(min_value: AnalogValue, max_value: AnalogValue) -> Self {
        debug_assert!(min_value < max_value);

        Self {
            min_value,
            max_value,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Percentage(u8);

impl Percentage {
    #[inline]
    pub fn new(value: u8) -> Self {
        debug_assert!(value <= 100);

        Self(value)
    }

    #[inline]
    pub const fn value(&self) -> u8 {
        self.0
    }
}

impl From<f32> for Percentage {
    fn from(value: f32) -> Self {
        debug_assert!(value >= 0.0);
        debug_assert!(value <= 1.0);

        let int_value = (value * 100.0).round() as u8;
        Self(int_value)
    }
}

impl Display for Percentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TargetMoistureLevel {
    min_value: Percentage,
    max_value: Percentage,
}

impl TargetMoistureLevel {
    #[inline]
    pub fn new(min_value: Percentage, max_value: Percentage) -> Self {
        debug_assert!(min_value < max_value);

        Self {
            min_value,
            max_value,
        }
    }

    #[inline]
    pub const fn min_value(&self) -> Percentage {
        self.min_value
    }

    #[inline]
    pub const fn max_value(&self) -> Percentage {
        self.max_value
    }
}

impl Display for TargetMoistureLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}—{}", self.min_value, self.max_value)
    }
}

#[derive(Debug)]
pub struct PlantIrrigator<MicrocontrollerImpl: Microcontroller> {
    soil_moisture_sensor: MicrocontrollerImpl::AnalogInput,
    pump_enabled: MicrocontrollerImpl::DigitalOutput,

    calibration_result: SensorCalibrationResult,
    target_moisture_level: TargetMoistureLevel,
}

const PUMP_ON_TIME: Duration = Duration::from_millis(500);

impl<MicrocontrollerImpl: Microcontroller> PlantIrrigator<MicrocontrollerImpl> {
    #[inline]
    pub fn new(
        soil_moisture_sensor: MicrocontrollerImpl::AnalogInput,
        pump_enabled: MicrocontrollerImpl::DigitalOutput,
        calibration_result: SensorCalibrationResult,
        target_moisture_level: TargetMoistureLevel,
    ) -> Self {
        Self {
            soil_moisture_sensor,
            pump_enabled,
            calibration_result,
            target_moisture_level,
        }
    }

    pub fn execute(&mut self, microcontroller: &MicrocontrollerImpl) -> IrrigationStatus {
        let moisture = self.soil_moisture_sensor.get_value();

        let min_val = self.calibration_result.min_value;
        let max_val = self.calibration_result.max_value;
        let moisture_clamped = moisture.value().clamp(min_val.value(), max_val.value());
        let moisture_ratio = (moisture_clamped - min_val.value()) as f32
            / (max_val.value() - min_val.value()) as f32;
        let moisture_percentage: Percentage = (1.0 - moisture_ratio).into();

        info!(
            "Moisture value: {}; min: {}, max: {}, percentage: {}",
            moisture, min_val, max_val, moisture_percentage
        );
        info!("Target level: {}", self.target_moisture_level);

        if moisture_percentage < self.target_moisture_level.min_value {
            info!("Actual level below target, watering...");

            self.pump_enabled.set_high();
            microcontroller.wait(PUMP_ON_TIME);
            self.pump_enabled.set_low();

            IrrigationStatus::Watered
        } else {
            info!("Actual level above target, not watering");

            IrrigationStatus::NotWatered
        }
    }
}

#[derive(Debug)]
pub enum IrrigationStatus {
    Watered,
    NotWatered,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::mock_uc::{MockMicrocontroller, MockMicrocontrollerAction};
    use crate::uc::{GPIO_0, GPIO_1};

    #[test_log::test]
    fn water_when_below_target() {
        let (mock_uc, mut plant_irrigator) = create_test_data();

        mock_uc.set_analog_value(GPIO_1, AnalogValue::new(2000));
        plant_irrigator.execute(&mock_uc);

        let actual_actions = mock_uc.actions();
        let expected_actions = vec![
            MockMicrocontrollerAction::GpioSetAsDigitalOutput(GPIO_0),
            MockMicrocontrollerAction::GpioSetAsAnalogInput(GPIO_1),
            MockMicrocontrollerAction::AnalogGpioGetValue(GPIO_1, AnalogValue::new(2000)),
            MockMicrocontrollerAction::DigitalGpioHigh(GPIO_0),
            MockMicrocontrollerAction::Wait(PUMP_ON_TIME),
            MockMicrocontrollerAction::DigitalGpioLow(GPIO_0),
        ];
        assert_eq!(actual_actions, expected_actions);
    }

    #[test_log::test]
    fn dont_water_when_above_target() {
        let (mock_uc, mut plant_irrigator) = create_test_data();

        mock_uc.set_analog_value(GPIO_1, AnalogValue::new(1000));
        plant_irrigator.execute(&mock_uc);

        let actual_actions = mock_uc.actions();
        let expected_actions = vec![
            MockMicrocontrollerAction::GpioSetAsDigitalOutput(GPIO_0),
            MockMicrocontrollerAction::GpioSetAsAnalogInput(GPIO_1),
            MockMicrocontrollerAction::AnalogGpioGetValue(GPIO_1, AnalogValue::new(1000)),
        ];
        assert_eq!(actual_actions, expected_actions);
    }

    fn create_test_data() -> (MockMicrocontroller, PlantIrrigator<MockMicrocontroller>) {
        let mut mock_uc = MockMicrocontroller::new();
        let pumb_enabled = mock_uc.get_digital_output(GPIO_0);
        let soil_sensor = mock_uc.get_analog_input(GPIO_1);

        let calibration_result =
            SensorCalibrationResult::new(AnalogValue::new(500), AnalogValue::new(2200));
        let target_moisture_level =
            TargetMoistureLevel::new(Percentage::new(40), Percentage::new(70));
        let plant_irrigator: PlantIrrigator<MockMicrocontroller> = PlantIrrigator::new(
            soil_sensor,
            pumb_enabled,
            calibration_result,
            target_moisture_level,
        );

        (mock_uc, plant_irrigator)
    }
}
