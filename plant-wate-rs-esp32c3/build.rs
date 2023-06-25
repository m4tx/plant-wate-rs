#[derive(Debug)]
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> anyhow::Result<()> {
    let cfg_path = std::path::Path::new("../cfg.toml");
    println!("cargo:rerun-if-changed={}", cfg_path.to_string_lossy());
    println!("cargo:rerun-if-changed=cfg.toml");

    // Check if the `cfg.toml` file exists and has been filled out.
    // if !cfg_path.exists() {
    //     anyhow::bail!("You need to create a `cfg.toml` file with your Wi-Fi
    // credentials! Use `cfg.toml.example` as a template."); }
    //
    // // The constant `CONFIG` is auto-generated by `toml_config`.
    // let app_config = CONFIG;
    // eprintln!("{:?}", app_config);
    // if app_config.wifi_ssid.is_empty()
    //     || app_config.wifi_psk.is_empty()
    //     || app_config.wifi_ssid == "SSID"
    //     || app_config.wifi_psk == "password"
    // {
    //     anyhow::bail!("You need to set the Wi-Fi credentials in `cfg.toml`!");
    // }

    // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")
}
