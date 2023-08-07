use std::error::Error;
use std::process::Command;

#[derive(Clone, Debug)]
pub struct Android {
    pub port: u16,
    pub device: String,
}

impl Android {
    pub fn new() -> Self {
        Self {
            port: 0,
            device: String::new(),
        }
    }

    pub fn boot_emulator(&self) -> Result<(), Box<dyn Error>> {
        let command = format!("~/Library/Android/sdk/emulator/emulator -avd {} &", self.device);
        let status = Command::new("sh")
            .arg("-c")
            .arg(command)
            .status()?;

        if !status.success() {
            return Err(format!("Failed to start emulator for device: {}", self.device).into());
        }

        Ok(())
    }

    pub fn emulator_list(&self) -> Result<String, Box<dyn Error>> {
        let command = "~/Library/Android/sdk/emulator/emulator -list-avds";
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        if !output.status.success() {
            return Err(format!("Failed to list AVDs").into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        Ok(stdout.into())
    }

    pub fn emulator_running(&self) -> Result<String, Box<dyn Error>> {
        let name = format!("emulator-{}", self.port);
        let command = "~/Library/Android/sdk/platform-tools/adb devices";
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        if !output.status.success() {
            return Err(format!("Failed to executor adb devices").into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.contains(&name) {
            Ok(name)
        } else {
            Err(format!("Emulator is not running.").into())
        }
    }
}