mod info;
use info::battery::Spec;

fn main() {
    let battery = Spec::spec();

    println!("Capacity: {}%", battery.capacity.unwrap_or(0));
    println!("Health:   {:.2}%", battery.health.unwrap_or(0.0));
    println!("Cycles:   {}", battery.cycles.unwrap_or(0));
    println!("Status:   {}", battery.status.as_deref().unwrap_or("N/A"));
}
