use dotenv::dotenv;
use reqwest::{blocking::Client, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://api.twitter.com/2";
const GET_RECENT_TWEETS: &str = "/tweets/search/recent";

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    data: Vec<Tweet>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Tweet {
    id: String,
    text: String,
    author_id: String,
}

fn get_bearer_token_auth_header() -> String {
    let bearer_token = std::env::var("BEARER_TOKEN")
        .expect("BEARER_TOKEN environment variable must be set");

    format!("Bearer {}", bearer_token)
}

fn get_url(endpoint: &str) -> String {
    let mut url = BASE_URL.to_string();
    url.push_str(endpoint);

    url
}

fn get_tweets(client: &Client) -> Vec<Tweet> {
    let url = get_url(GET_RECENT_TWEETS);
    let authorization_header = get_bearer_token_auth_header();
    let query = [
        ("query", "@echoBTC"),
        ("expansions", "author_id")
    ];

    client
        .get(url)
        .header(AUTHORIZATION, authorization_header)
        .query(&query)
        .send()
        .unwrap()
        .json::<Response>()
        .unwrap()
        .data
}

fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let client = Client::new();

    println!("{:#?}", get_tweets(&client));

    Ok(())
}
