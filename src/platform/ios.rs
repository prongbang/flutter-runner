use std::process::Command;
use std::error::Error;
use regex::Regex;

pub struct IOS {
    pub min: u16,
    pub max: u16,
}

impl IOS {
    pub fn new() -> Self {
        Self {
            min: 0,
            max: 0,
        }
    }

    pub fn boot_simulator(&self, device: &str) -> Result<(), Box<dyn Error>> {
        let command = format!("xcrun simctl boot \"{}\"", device);
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() || !stderr.is_empty() {
            return Err(format!("Can't boot simulator {}", device).into());
        }

        Ok(())
    }

    pub fn simulator_running_by_device(&self, device: &str) -> Result<(), Box<dyn Error>> {
        let command = format!("xcrun simctl list devices | grep -E \"{}\" | grep Booted", device);
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() || !stdout.contains("Booted") || !stderr.is_empty() {
            return Err(format!("No simulators are currently running").into());
        }

        Ok(())
    }

    pub fn simulator_running(&self) -> Result<String, Box<dyn Error>> {
        let mut models = Vec::new();

        // Generate model names for the specified range
        for j in self.min..=self.max {
            models.push(format!("iPhone {}", j));
            models.push(format!("iPhone {} Pro", j));
            models.push(format!("iPhone {} Max", j));
        }

        // Run the command to list devices and filter by booted simulators
        let command = format!("xcrun simctl list devices | grep -E \"{}\" | grep Booted", models.join("|"));
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() || !stdout.contains("Booted") || !stderr.is_empty() {
            return Err(format!("No simulators are currently running").into());
        } else {
            let simulators = stdout.to_string();

            // Regular expression to match iPhone model names
            let re = Regex::new(r#"(iPhone \d{1,2} Pro Max|iPhone \d{1,2} Pro|iPhone \d{1,2})"#).unwrap();

            // find all matches in the message
            let matches: Vec<&str> = re.find_iter(&simulators).map(|mat| mat.as_str()).collect();

            // Select first simulator
            if let Some(first_match) = matches.get(0) {
                return Ok(first_match.to_string());
            }
        }

        Err(format!("No simulators are currently running, Please run simulator manually").into())
    }
}