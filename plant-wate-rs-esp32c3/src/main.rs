use core::str;

use anyhow::Result;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys as _;
use plant_wate_rs_core::controller::Controller;

use crate::microcontroller_esp32c3::MicrocontrollerEsp32c3;

mod microcontroller_esp32c3;
mod wifi;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let _app_config = CONFIG;
    let _sysloop = EspSystemEventLoop::take()?;

    let microcontroller = MicrocontrollerEsp32c3::new();
    let mut controller = Controller::new(microcontroller);

    controller.run();
}
