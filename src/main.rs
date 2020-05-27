mod config;
use config::Config;

use hyper::{Client, Request, Body};
use hyper::http::request::Builder;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

use std::time::Instant;
use std::error::Error;

use serde_json::{Value, Map};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = std::fs::read_to_string("./config.json")?.parse()?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    for request_number in 0..config.requests_to_send {
        let body = match config.body.as_str() {
            Some(body) => body,
            None => "",
        };

        send_request(&client, request_number, &config.method, &config.uri, &config.headers, body.to_owned()).await?;
    }

    Ok(())
}

fn build_request(method: &str, uri: &str, headers: &Map<String, Value>) -> Builder {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri);

    for (key, value) in headers {
        let value = serde_json::to_string(value).expect("failed to read header value as string");
        builder = builder.header(key.as_str(), value);
    }

    builder
}

async fn send_request(client: &Client<HttpsConnector<HttpConnector>>, request_number: u64, method: &str, uri: &str, headers: &Map<String, Value>, body: String) -> Result<(), Box<dyn Error>> {
    let duration = Instant::now();

    let builder = build_request(method, uri, headers);
    let request = builder.body(Body::from(body))?;
    let response = client.request(request).await?;

    let elapsed = duration.elapsed();

    let status = response.status();

    println!("{}: {:#?} {}ms", request_number, status, elapsed.as_millis());
    
    Ok(())
}