use dotenv::dotenv;
use echo_btc::{
    api::{get_tweets, post_retweet},
    database::{already_sent, connect},
};
use reqwest::blocking::Client;
use std::{thread, time};

fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let db = connect();
    let client = Client::new();

    loop {
        let tweets = get_tweets(&client);
        for i in 0..tweets.len() {
            let tweet_id = &tweets[i].id;

            if !already_sent(&db, tweet_id) {
                post_retweet(&client, &tweet_id, &db);
                thread::sleep(time::Duration::from_secs(5));
            }
        }

        thread::sleep(time::Duration::from_secs(5));
    }
}
