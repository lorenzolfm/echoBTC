use dotenv::dotenv;
use echo_btc::{
    api::{get_tweets, post_retweet},
    database::{already_sent, connect},
    env::Env,
};
use reqwest::blocking::Client;
use std::{thread, time};

fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let env = Env::new();
    let bearer_token = env.get_bearer_token();
    let db = connect(".sqlite");
    let client = Client::new();

    loop {
        if let Some(tweets) = get_tweets(&client, bearer_token) {
            (0..tweets.len()).for_each(|i| {
                let tweet_id = &tweets[i].id;

                if !already_sent(&db, tweet_id) {
                    post_retweet(&client, tweet_id, &db, &env)
                }
            });

            thread::sleep(time::Duration::from_secs(20));
        }
    }
}
