use ::sysinfo::{CpuExt, System, SystemExt};

pub struct CpuInfo {
    pub vendor_id: String,
    pub brand: String,
    pub frequency: u64,
    pub usage: f32,
}

impl CpuInfo {
    pub fn new() -> Vec<CpuInfo> {
        let mut sys = System::new_all();
        let mut cpu_info: Vec<CpuInfo> = Vec::new();
        sys.refresh_cpu();
        for cpu in sys.cpus() {
            cpu_info.push(CpuInfo {
                vendor_id: String::from(cpu.vendor_id()),
                brand: String::from(cpu.brand()),
                frequency: cpu.frequency(),
                usage: cpu.cpu_usage(),
            });
        }
        cpu_info
    }
}
