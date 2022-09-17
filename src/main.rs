use dotenv::dotenv;
use reqwest::{blocking::Client, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time, thread};

const BASE_URL: &str = "https://api.twitter.com/2";
const GET_RECENT_TWEETS: &str = "/tweets/search/recent";
const POST_RETWEET: &str = "/users/2597897487/retweets";

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

fn get_url(endpoint: &str) -> String {
    let mut url = BASE_URL.to_string();
    url.push_str(endpoint);

    url
}

fn get_tweets(client: &Client) -> Vec<Tweet> {
    let url = get_url(GET_RECENT_TWEETS);
    let authorization_header = get_bearer_token_auth_header();
    let query = [("query", "@echoBTC"), ("expansions", "author_id")];

    client
        .get(url)
        .header(AUTHORIZATION, authorization_header)
        .query(&query)
        .send()
        .unwrap()
        .json::<Response>()
        .unwrap()
        .data
        .into_iter()
        .filter(|tweet| { tweet.author_id != "2597897487" })
        .collect()
}

fn get_bearer_token_auth_header() -> String {
    let bearer_token =
        std::env::var("BEARER_TOKEN").expect("BEARER_TOKEN environment variable must be set");

    format!("Bearer {}", bearer_token)
}

fn post_retweet(client: &Client, tweet: &Tweet) {
    let url = get_url(POST_RETWEET);
    let authorization_header = get_oauth1_header(&url);
    let body = HashMap::from([
        ("tweet_id", &tweet.id)
    ]);

    let res = client
        .post(url)
        .header(AUTHORIZATION, authorization_header)
        .json(&body)
        .send()
        .unwrap();

    println!("{:#?}", res.text());
}

fn get_oauth1_header(url: &String) -> String {
    let consumer_key = std::env::var("API_KEY")
        .expect("API_KEY environment variable must be set.");
    let consumer_secret = std::env::var("API_SECRET_KEY")
        .expect("API_SECRET_KEY environment variable must be set.");
    let access_token = std::env::var("ACCESS_TOKEN")
        .expect("ACCESS_TOKEN environment variable must be set.");
    let token_secret = std::env::var("ACCESS_TOKEN_SECRET")
        .expect("ACCESS_TOKEN_SECRET environment variable must be set.");

    let token = oauth::Token::from_parts(consumer_key, consumer_secret, access_token, token_secret);

    oauth::post(url, &(), &token, oauth::HMAC_SHA1)
}

fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let client = Client::new();

    loop {
        let tweets = get_tweets(&client);

        for i in 0..tweets.len() {
            post_retweet(&client, &tweets[i]);
            thread::sleep(time::Duration::from_secs(5));
        }
    }
}
