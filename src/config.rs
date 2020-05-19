use std::str::FromStr;
use std::error::Error;

pub struct Config {
    pub method: String,
    pub amount: u64,
    pub uri: String,
    pub authorization: String,
    pub body: serde_json::Value,
}

impl FromStr for Config {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json: serde_json::Value = serde_json::from_str(&s)?;

        let method = json.get("method").unwrap().as_str().expect("no method specified");
        let amount = json.get("amount").unwrap().as_u64().expect("no amount specified");
        let uri = json.get("uri").unwrap().as_str().expect("no uri specified");
        let authorization = json.get("authorization").unwrap().as_str().expect("no authorization specified");
        let body = json.get("body").expect("no body specified");

        Ok(Config {
            method: method.to_owned(),
            amount: amount,
            uri: uri.to_owned(),
            authorization: authorization.to_owned(),
            body: body.to_owned(),
        })
    }
} 