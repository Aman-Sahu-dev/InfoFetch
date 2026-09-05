mod info;
pub fn main() {
    let battery = info::battery::Battery::get_capacity();
    println!("Battery: {}", battery.capacity);
}
