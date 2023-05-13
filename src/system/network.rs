use ::sysinfo::{NetworkExt, System, SystemExt};

pub struct NetworkInfo {
    pub interface_name: String,
    pub rx: u64,
    pub tx: u64,
}

impl NetworkInfo {
    pub fn new() -> Vec<NetworkInfo> {
        let mut sys = System::new_all();
        let mut network_info: Vec<NetworkInfo> = Vec::new();
        sys.refresh_networks();
        for (interface_name, network) in sys.networks() {
            network_info.push(NetworkInfo {
                interface_name: String::from(interface_name),
                rx: network.received(),
                tx: network.transmitted(),
            });
        }
        network_info
    }
}
