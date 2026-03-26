pub mod settings;

pub use settings::Settings;

pub fn init() -> Settings {
    Settings::new()
}
