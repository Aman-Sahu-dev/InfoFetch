use std::{fs, path::Path};
pub struct Spec {
    pub capacity: Option<u8>,
    pub health: Option<f64>,
    pub cycles: Option<u32>,
    pub status: Option<String>,
}
impl Spec {
    const BAT_PATH: &'static str = "/sys/class/power_supply/BAT0";
    pub fn spec() -> Spec {
        Spec {
            capacity: Self::capacity(),
            health: Self::health(),
            cycles: Self::cycles(),
            status: Self::status(),
        }
    }
    pub fn sysfs(file: &str) -> Option<String> {
        let path = Path::new(Self::BAT_PATH).join(file);
        let content = fs::read_to_string(path).ok()?;
        Some(content.trim().to_string())
    }

    pub fn capacity() -> Option<u8> {
        Self::sysfs("capacity")?.parse().ok()
    }
    pub fn health() -> Option<f64> {
        let full_str = Self::sysfs("energy_full").or_else(|| Self::sysfs("charge_full"))?;
        let design_str =
            Self::sysfs("energy_full_design").or_else(|| Self::sysfs("charge_full_design"))?;
        let full: f64 = full_str.parse().ok()?;
        let design: f64 = design_str.parse().ok()?;
        if design == 0.0 {
            return None;
        }
        Some((full / design) * 100.0)
    }
    pub fn cycles() -> Option<u32> {
        Self::sysfs("cycle_count")?.parse().ok()
    }
    pub fn status() -> Option<String> {
        Self::sysfs("status")
    }
}
