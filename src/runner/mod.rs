use std::process::Command;
use std::time::{SystemTime};
use regex::Regex;
use crate::platform::Platform;
use crate::result;
use serde::{Deserialize};

pub const TYPE_TEST_DRIVER: &str = "test_driver";
pub const TYPE_INTEGRATION_TEST: &str = "integration_test";
pub const FVM: &str = "fvm";

#[derive(Debug, Deserialize)]
pub struct TestRunner {
    pub vm: Option<String>,
    pub device: Device,
    pub report: Report,
    pub tests: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub android: Platform,
    pub ios: Platform,
}

#[derive(Debug, Deserialize)]
pub struct Report {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Runner {
    pub vm: String,
    pub flavor: String,
    pub device_id: Option<String>,
    pub device_name: String,
    pub r#type: String,
}

impl TestRunner {
    pub fn new() -> Self {
        Self {
            vm: Some(String::new()),
            device: Device::new(),
            report: Report::new(),
            tests: vec![],
        }
    }
}

impl Report {
    pub fn new() -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl Device {
    pub fn new() -> Self {
        Self {
            android: Platform::new(),
            ios: Platform::new(),
        }
    }
}

impl Runner {
    pub fn run(&self, test: &str) -> result::Result {
        let mut rs = result::Result::new();

        rs.test = test.to_string().clone();

        // Check device id
        let device_name = match &self.device_id {
            Some(device) => device.as_str(),
            None => self.device_name.as_str(),
        };

        // Check script by type
        let mut args: Vec<String> = Vec::new();
        if self.r#type == TYPE_TEST_DRIVER {
            let target = format!("--target=integration_test/{}", test);
            args = vec![
                String::from("drive"),
                String::from("--driver=test_driver/integration_test.dart"),
                target,
                String::from("--flavor"), format!("{}", self.flavor.as_str()),
                String::from("-d"), format!("{}", device_name),
            ];
        } else if self.r#type == TYPE_INTEGRATION_TEST {
            let integration_test = format!("integration_test/{}", test);
            args = vec![
                String::from("test"),
                integration_test,
                String::from("--flavor"), format!("{}", self.flavor.as_str()),
                String::from("-d"), format!("{}", device_name),
            ];
        }

        println!("Testing {}", test);

        let mut command = format!("flutter {}", args.join(" "));
        let mut cmd: Command;

        if self.vm == FVM {
            command = format!("fvm flutter {}", args.join(" "));
            println!("├── {}", command);
            println!("├── ...");
            cmd = Command::new("fvm");
            cmd.args(&["flutter"]).args(&args);
        } else {
            println!("├── {}", command);
            println!("├── ...");
            cmd = Command::new("flutter");
            cmd.args(&args);
        }

        let output = cmd.output();

        match output {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let stderr_str = String::from_utf8_lossy(&output.stderr);

                let re = Regex::new(r"Some tests failed").unwrap();

                if output.status.success() && !re.is_match(&output_str) {
                    println!("└── [PASSED]");
                    rs.status = "PASSED".to_string();
                } else {
                    println!("└── [FAILED]");
                    rs.status = "FAILED".to_string();
                    rs.error = format!("{}\n{}", output_str, stderr_str);
                }
            }
            Err(err) => {
                println!("└── [FAILED]");
                rs.status = "FAILED".to_string();
                rs.error = err.to_string();
            }
        }

        rs.end_time = SystemTime::now();
        println!(" after {:.2} m", rs.get_time_minute());
        rs
    }
}
