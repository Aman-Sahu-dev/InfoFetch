use std::fs;
pub struct Battery {
    pub capacity: u8,
}
impl Battery {
    pub fn get_capacity() -> Battery {
        let capacity = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").unwrap();
        let capacity: u8 = capacity.trim().parse().unwrap();
        Battery { capacity }
    }
}
