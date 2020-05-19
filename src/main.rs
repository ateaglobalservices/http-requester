mod config;

use std::time::Instant;
use std::error::Error;

use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = std::fs::read_to_string("./config.json")?.parse()?;

    for request_number in 1..config.amount {
        send_request(request_number, &config).await?;
    }

    Ok(())
}

async fn send_request(request_number: u64, config: &Config) -> Result<(), Box<dyn Error>> {
    let duration = Instant::now();

    let request = get_request(config).unwrap();
    let response = request.send().await?;

    let elapsed = duration.elapsed();

    let status = response.status();

    println!("{}: {:#?} {}ms", request_number, status, elapsed.as_millis());
    
    Ok(())
}

fn get_request(config: &Config) -> Option<reqwest::RequestBuilder> {
    if config.method == "GET" {
        return Some(reqwest::Client::new()
            .get(&config.uri)
            .header("Authorization", &config.authorization));
    }
    
    if config.method == "POST" {
        return Some(reqwest::Client::new()
            .post(&config.uri)
            .header("Authorization", &config.authorization)
            .json(&config.body));
    }

    None
}