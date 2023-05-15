use ::sysinfo::{NetworkExt, System, SystemExt};

pub struct NetworkInfo {
    pub interface_name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

impl NetworkInfo {
    pub fn new() -> Vec<Self> {
        let mut sys = System::new_all();
        let mut network_info = Vec::new();
        sys.refresh_networks();
        for (interface_name, network) in sys.networks() {
            network_info.push(Self {
                interface_name: interface_name.to_owned(),
                rx_bytes: network.received(),
                tx_bytes: network.transmitted(),
            });
        }
        network_info
    }
}
