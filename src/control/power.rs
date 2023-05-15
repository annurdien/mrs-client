use system_shutdown::{force_reboot, force_shutdown};

pub trait PowerControl {
    fn shutdown(&self) -> Result<(), String>;
    fn reboot(&self) -> Result<(), String>;
}

impl PowerControl for () {
    fn shutdown(&self) -> Result<(), String> {
        match force_shutdown() {
            Ok(()) => Ok(()),
            Err(_) => Err(String::from("Failed to shutdown")),
        }
    }

    fn reboot(&self) -> Result<(), String> {
        match force_reboot() {
            Ok(()) => Ok(()),
            Err(_) => Err(String::from("Failed to reboot")),
        }
    }
}
