mod info;

use info::battery;
use info::storage;

fn main() {
    println!("=== SYSTEM STATUS ===\n");

    // --- Battery Info ---
    let battery_data = battery::Spec::spec();

    println!("Battery Capacity : {}%", battery_data.capacity.unwrap_or(0));
    println!(
        "Battery Health   : {:.1}%",
        battery_data.health.unwrap_or(0.0)
    );
    println!("Battery Cycles   : {}", battery_data.cycles.unwrap_or(0));
    println!(
        "Battery Status   : {}",
        battery_data.status.as_deref().unwrap_or("N/A")
    );
    println!();

    // --- Storage Info ---
    match storage::Spec::spec(None::<String>) {
        Ok(storage_data) => {
            println!(
                "Drive Prefix     : {}",
                storage_data.drive.as_deref().unwrap_or("Unknown")
            );
            println!(
                "Device Node      : {}",
                storage_data.device.as_deref().unwrap_or("Unknown")
            );
            println!(
                "Mount Point      : {}",
                storage_data.mounted_point.as_deref().unwrap_or("N/A")
            );
            println!(
                "Total Space      : {}",
                storage_data.total.as_deref().unwrap_or("N/A")
            );
            println!(
                "Free Space       : {}",
                storage_data.free.as_deref().unwrap_or("N/A")
            );

            match storage_data.usage {
                Some(pct) => println!("Storage Usage    : {:.1}%", pct),
                None => println!("Storage Usage    : N/A"),
            }
        }
        Err(e) => {
            println!("Storage Error    : {}", e);
        }
    }
}
