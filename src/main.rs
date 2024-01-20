use error_chain::error_chain;
use std::{
    thread,
    time::{Duration, Instant},
};
use clap::Parser;

/// Simple program to benchmark API endpoints, respecting rate limiting
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The URL of the API endpoint you want to benchmark
    #[arg(short, long)]
    url: String,

    /// The number of requests to make
    #[arg(short, long, default_value_t = 10)]
    requests: u8,
    
    /// The delay between requests (in ms)
    #[arg(short, long, default_value_t = 1000)]
    delay: u64,
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut durations: Vec<Duration> = vec![];
    for _ in 0..args.requests {
        // Start Timing
        let start = Instant::now();
        // Perform request
        let res = reqwest::get(&args.url).await?;
        // Stop Timing
        let duration = start.elapsed();

        // Only deal with successful requests
        let status_code = res.status();
        if status_code.is_success() {
            println!("Request took {:?}", duration);
        } else {
            println!("Status Code:{}", status_code.as_str());
            break;
        }
        durations.push(duration);
        thread::sleep(Duration::from_millis(args.delay));
    }

    let sum: Duration = durations.iter().sum();
    let mean = sum.as_secs_f64() / durations.len() as f64 * 1000.0;
    println!("Average Operation: {:.2?}ms", mean);
    Ok(())
}
