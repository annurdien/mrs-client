use sysinfo::{CpuExt, System, SystemExt};

#[derive(Debug)]
pub struct CoreInfo {
    vendor_id: String,
    brand: String,
    frequency: u64,
    usage: f32,
}

impl CoreInfo {
    pub fn from_processor(processor: &impl CpuExt) -> Self {
        Self {
            vendor_id: processor.vendor_id().to_string(),
            brand: processor.brand().to_string(),
            frequency: processor.frequency(),
            usage: processor.cpu_usage(),
        }
    }
}

pub struct CpuInfo {
    cores: Vec<CoreInfo>,
}

impl CpuInfo {
    pub fn new() -> Self {
        let mut system = System::new_all();
        let mut cores = Vec::new();

        system.refresh_cpu();
        for processor in system.cpus() {
            cores.push(CoreInfo::from_processor(processor));
        }

        Self { cores }
    }

    pub fn usage(&self) -> f32 {
        let total_usage: f32 = self.cores.iter().map(|c| c.usage).sum();
        total_usage / self.cores.len() as f32
    }

    pub fn frequency(&self) -> u64 {
        let total_frequency: u64 = self.cores.iter().map(|c| c.frequency).sum();
        total_frequency / self.cores.len() as u64
    }

    pub fn vendor_id(&self) -> &str {
        &self.cores[0].vendor_id
    }

    pub fn brand(&self) -> &str {
        &self.cores[0].brand
    }
}
