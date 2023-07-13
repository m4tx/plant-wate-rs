use std::time::Duration;

use crate::plant_irrigator_controller::PlantIrrigatorController;
use crate::uc::Microcontroller;

pub struct Controller<MicrocontrollerImpl: Microcontroller> {
    uc: MicrocontrollerImpl,
    plant_irrigator_ctrl: PlantIrrigatorController<MicrocontrollerImpl>,
}

impl<MicrocontrollerImpl: Microcontroller> Controller<MicrocontrollerImpl> {
    #[must_use]
    pub fn new(mut microcontroller: MicrocontrollerImpl) -> Self {
        let plant_irrigator_ctrl = PlantIrrigatorController::new(&mut microcontroller);

        Self {
            uc: microcontroller,
            plant_irrigator_ctrl,
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_cycle();
        }
    }

    pub fn run_cycle(&mut self) {
        self.plant_irrigator_ctrl.run_cycle(&self.uc);
        self.uc.wait(Duration::from_millis(1000));
    }
}
