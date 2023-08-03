use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    platform: String,

    #[arg(short, long, default_value = "")]
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

fn main() {
    let args = Args::parse();

    println!("{:?}", args)
}