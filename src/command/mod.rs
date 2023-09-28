use std::error::Error;
use tokio::task;
use tokio::time::{sleep, Duration};
use crate::{platform, reporter, result, runner};

pub struct Command {
    pub vm: String,
    pub platform: String,
    pub flavor: String,
    pub device: String,
    pub r#type: String,
    pub path: String,
    pub report: String,
    pub tests: Vec<String>,
}

impl Command {
    pub fn is_android(&self) -> bool {
        self.platform == platform::ANDROID
    }

    pub fn is_ios(&self) -> bool {
        self.platform == platform::IOS
    }

    pub async fn execute(&mut self) -> Result<(),  Box<dyn Error>> {
        // Config test runner
        let mut run = runner::Runner {
            vm: self.vm.clone(),
            flavor: String::new(),
            device: String::new(),
            r#type: self.r#type.clone(),
        };

        let ios = platform::ios::IOS {
            min: 14,
            max: 16,
        };
        let android = platform::android::Android {
            port: 5554,
            device: self.device.clone(),
        };

        match &self.device {
            d if !d.is_empty() => {
                run.device = d.clone();
                if self.is_ios() {
                    println!("iOS Simulator");
                    println!("Boot simulator {}", d);
                    if ios.simulator_running_by_device(d).is_err() {
                        ios.boot_simulator(d)?;
                        println!("├── booting... 15s");
                        sleep(Duration::from_secs(15)).await;
                        println!("└── [BOOTED]");
                    } else {
                        println!("└── [BOOTED]");
                    }
                } else if self.is_android() {
                    println!("Android Emulator");
                    println!("Boot emulator {}", d);
                    if android.emulator_running().is_err() {
                        let android_clone = android.clone();
                        task::spawn(async move {
                            if let Err(err) = android_clone.boot_emulator() {
                                println!("└── {}", err);
                            }
                        }).await?;
                        println!("├── booting... 15s");
                        sleep(Duration::from_secs(15)).await;

                        if android.emulator_running().is_err() {
                            println!("└── [FAILED]");
                        } else {
                            println!("└── [BOOTED]");
                        }
                    } else {
                        println!("└── [BOOTED]");
                    }
                }
            }
            _ if self.is_android() => {
                if let Ok(device) = android.emulator_running() {
                    run.device = device;
                }
            }
            _ if self.is_ios() => {
                if let Ok(device) = ios.simulator_running() {
                    run.device = device;
                }
            }
            _ => return Err(format!("--device not found").into()),
        }

        match &self.flavor {
            f if !f.is_empty() => {
                run.flavor = f.clone();
            }
            _ if self.is_android() => {
                run.flavor = "automateGoogle".to_string();
            }
            _ if self.is_ios() => {
                run.flavor = "automate".to_string();
            }
            _ => return Err(format!("--flavor not found").into()),
        }

        if self.report.is_empty() {
            self.report = format!("report-{}.html", self.platform);
        } else {
            self.report = self.report.replace("{}", self.platform.as_str());
        }

        // Test files
        let mut tests = self.tests.clone();
        if self.path.contains("_test.dart") {
            tests = vec![self.path.clone()];
        }

        // Run test
        let mut results: Vec<result::Result> = Vec::new();
        println!("Run {} tests on {}", tests.len(), run.device);
        for t in &tests {
            let result = run.run(t);
            results.push(result);
        }

        // Generate report to html file
        let mut report = reporter::Reporter {
            file_name: self.report.clone(),
        };
        if self.report.contains(".html") {
            report.file_name = self.report.clone();
        }

        report.generate(results)?;

        Ok(())
    }
}