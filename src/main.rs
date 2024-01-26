use clap::Parser;
use error_chain::error_chain;
use reqwest::Response;
use std::{
    thread,
    time::{Duration, Instant},
};

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

    /// Basic auth - username
    #[arg(short = 'n', long)]
    username: Option<String>,

    /// Basic auth - password
    #[arg(short, long)]
    password: Option<String>,
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
        let client = reqwest::Client::new();
        let res: Response;
        match args.username {
            Some(ref username) => {
                res = client
                    .get(&args.url)
                    .basic_auth(username, args.password.as_deref())
                    .send()
                    .await?;
            }
            None => {
                res = client.get(&args.url).send().await?;
            }
        }

        // Stop Timing
        let duration = start.elapsed();

        // Only deal with successful requests
        let status_code = res.status();
        if status_code.is_success() {
            println!("Request took {:?}", duration);
            durations.push(duration);
            thread::sleep(Duration::from_millis(args.delay));
        } else {
            println!("Status Code:{}", status_code.as_str());
            break;
        }
    }

    let sum: Duration = durations.iter().sum();
    let mean = sum.as_secs_f64() / durations.len() as f64 * 1000.0;
    println!("Average Operation: {:.2?}ms", mean);
    Ok(())
}
