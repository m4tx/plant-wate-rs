use std::time::Duration;

use crate::plant_irrigator::{
    Percentage, PlantIrrigator, SensorCalibrationResult, TargetMoistureLevel,
};
use crate::uc::{AnalogValue, Microcontroller, GPIO_0, GPIO_2};

pub struct PlantIrrigatorController<MicrocontrollerImpl: Microcontroller> {
    plant_irrigator_1: PlantIrrigator<MicrocontrollerImpl>,
}

impl<MicrocontrollerImpl: Microcontroller> PlantIrrigatorController<MicrocontrollerImpl> {
    pub fn new(microcontroller: &mut MicrocontrollerImpl) -> Self {
        let sensor_1 = microcontroller.get_analog_input(GPIO_0);
        let pump_1 = microcontroller.get_digital_output(GPIO_2);
        let calibration_result =
            SensorCalibrationResult::new(AnalogValue::new(1027), AnalogValue::new(2526));
        let target_moisture_level =
            TargetMoistureLevel::new(Percentage::new(40), Percentage::new(70));

        Self {
            plant_irrigator_1: PlantIrrigator::new(
                sensor_1,
                pump_1,
                calibration_result,
                target_moisture_level,
            ),
        }
    }

    pub fn run_cycle(&mut self, microcontroller: &MicrocontrollerImpl) {
        self.plant_irrigator_1.execute(microcontroller);
        microcontroller.wait(Duration::from_secs(5));
    }
}
