mod platform;
mod reporter;
mod result;
mod runner;
mod command;
mod config;
mod file;

use clap::Parser;
use crate::command::Command;
use crate::runner::TestRunner;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    platform: String,

    #[arg(short, long, default_value = "config.yaml")]
    config: String,

    #[arg(long, default_value = "")]
    flavor: String,

    #[arg(short, long, default_value = "")]
    device: String,

    #[arg(short, long, default_value = "")]
    file: String,

    #[arg(short, long, default_value = "")]
    report: String,

    #[arg(short, long, default_value = "integration_test")]
    r#type: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("{:?}", args);

    let mut cmd = Command {
        vm: String::new(),
        platform: args.platform.clone(),
        device_id: None,
        device_name: args.device,
        flavor: args.flavor,
        path: args.file,
        report: args.report,
        r#type: args.r#type,
        tests: vec![],
    };

    // Get config from file
    let runner: TestRunner = config::read_yaml(args.config.as_str());
    if let Some(vm) = runner.vm {
        cmd.vm = vm;
    }
    cmd.report = runner.report.name.clone();
    cmd.tests = runner.tests.clone();
    if cmd.is_android() {
        cmd.flavor = runner.device.android.flavor.clone();
        cmd.device_id = runner.device.android.device_id.clone();
        cmd.device_name = runner.device.android.device_name.clone();
    } else if cmd.is_ios() {
        cmd.flavor = runner.device.ios.flavor.clone();
        cmd.device_id = runner.device.android.device_id.clone();
        cmd.device_name = runner.device.ios.device_name.clone();
    }

    if cmd.is_android() || cmd.is_ios() {
        return cmd.execute().await;
    }

    Err(format!("Unsupported").into())
}