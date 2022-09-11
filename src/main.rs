use dotenv::dotenv;
use reqwest::header::AUTHORIZATION;

const URL: &str = "https://api.twitter.com/2/tweets/search/recent?query=%40echoBTC";

fn get_bearer_token() -> String {
    dotenv().ok();

    let bearer_token = std::env::var("BEARER_TOKEN")
        .expect("BEARER_TOKEN environment variable must be set.");

    format!("Bearer {}", bearer_token)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .get(URL)
        .header(AUTHORIZATION, get_bearer_token())
        .send()?;

    println!("{:#?}", res.text()?);

    Ok(())
}
