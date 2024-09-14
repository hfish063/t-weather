use clap::Parser;
use dotenv;
use ui::start;

mod api;
mod ui;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Receive weather forecast for this location
    #[arg(short, long)]
    location: String,

    /// Optional: specify the length of forecast (in days)
    #[arg(short, long, default_value_t = 1)]
    forecast: u8,
}

fn main() {
    dotenv::dotenv().ok();

    let args = Args::parse();

    let _ = start();
}
