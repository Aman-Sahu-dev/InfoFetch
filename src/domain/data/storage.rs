use nix::sys::statvfs::statvfs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
pub struct Spec {
    pub drive: Option<String>,
    pub device: Option<String>,
    pub total: Option<String>,
    pub free: Option<String>,
    pub usage: Option<f64>,
    pub mounted_point: Option<String>,
}
impl Spec {
    pub fn spec<P: AsRef<Path>>(target_path: Option<P>) -> Result<Spec, nix::Error> {
        let path_ref = match &target_path {
            Some(p) => p.as_ref(),
            None => Path::new("/"),
        };
        let stats = statvfs(path_ref)?;
        let frag_size: u64 = match stats.fragment_size() as u64 {
            0 => 0 as u64,
            size => size,
        };

        let total: u64 = (stats.blocks() as u64) * frag_size;
        let free: u64 = (stats.blocks_available() as u64) * frag_size;
        let used = total - free;
        let usage = (used as f64 / total as f64) * 100.0;
        let (device, drive) = Self::resolve_mounted_source(path_ref);
        Ok(Spec {
            drive,
            device,
            total: Self::calc(total),
            free: Self::calc(free),
            usage: Some(usage),
            mounted_point: Some(path_ref.to_string_lossy().into_owned()),
        })
    }
    pub fn calc(bytes: u64) -> Option<String> {
        let bytes = bytes as f64;
        let (value, unit) = if bytes < 1024.0 {
            (bytes, "B")
        } else if bytes < 1024_f64.powi(2) {
            (bytes / 1024.0, "Kib")
        } else if bytes < 1024_f64.powi(3) {
            (bytes / 1024_f64.powi(2), "Mib")
        } else if bytes < 1024_f64.powi(4) {
            (bytes / 1024_f64.powi(3), "Gib")
        } else {
            (bytes / 1024_f64.powi(3), "Tib")
        };
        Some(format!("{:.1} {}", value, unit))
    }
    pub fn resolve_mounted_source(target_path: &Path) -> (Option<String>, Option<String>) {
        let file = match File::open("/proc/mounts/") {
            Ok(f) => f,
            Err(_) => return (None, None),
        };
        let reader = BufReader::new(file);

        for line in reader.lines().map_while(Result::ok) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && Path::new(parts[1]) == target_path {
                let device_node = parts[0].to_string();
                let drive_id = Path::new(&device_node)
                    .file_name()
                    .map(|os_str| os_str.to_string_lossy().into_owned())
                    .map(|name| {
                        if name.starts_with("nvme") {
                            name.chars().take(7).collect()
                        } else {
                            name.chars().take(3).collect()
                        }
                    });
                return (Some(device_node), drive_id);
            }
        }
        (None, None)
    }
}
