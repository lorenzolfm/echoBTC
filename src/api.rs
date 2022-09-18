use std::collections::HashMap;
use crate::database::insert_id;
use serde::{Deserialize, Serialize};
use reqwest::{blocking::Client, header::AUTHORIZATION};

const BASE_URL: &str = "https://api.twitter.com/2";
const GET_RECENT_TWEETS: &str = "/tweets/search/recent";
const POST_RETWEET: &str = "/users/2597897487/retweets";

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    data: Vec<Tweet>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    text: String,
    author_id: String,
}

fn get_url(endpoint: &str) -> String {
    let mut url = BASE_URL.to_string();
    url.push_str(endpoint);

    url
}

pub fn get_tweets(client: &Client) -> Vec<Tweet> {
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

pub fn post_retweet(client: &Client, tweet_id: &str, database: &sqlite::Connection) {
    let url = get_url(POST_RETWEET);
    let authorization_header = get_oauth1_header(&url);
    let body = HashMap::from([
        ("tweet_id", tweet_id)
    ]);

    let res = client
        .post(url)
        .header(AUTHORIZATION, authorization_header)
        .json(&body)
        .send()
        .unwrap();

    println!("Retweeted {}", tweet_id);

    if res.status() == 200 {
        insert_id(&database, &tweet_id);
    }
}

#[cfg(test)]
mod tests {
    mod get_url {
        use crate::api::get_url;

        #[test]
        fn should_concat_endpoint_to_base_url() {
            assert_eq!(get_url("1"), "https://api.twitter.com/21")
        }
    }

    mod get_bearer_token_auth_header {
        use crate::api::get_bearer_token_header;
        use crate::env::Env;

        fn get_test_env() -> Env {
            Env::create_test_env()
        }

        #[test]
        fn should_return_expect_header() {
            let env = get_test_env();

            assert_eq!(
                get_bearer_token_header(env.get_bearer_token().to_string()),
                "Bearer 1".to_string()
            )
        }
    }

    mod get_oauth1_header {
        use super::get_test_env;
        use crate::api::get_oauth1_header;

        #[test]
        fn should_return_expected_header() {
            let env = get_test_env();
            let url = "test_url".to_string();

            let header = get_oauth1_header(&url, &env);
            println!("{}", header);

            assert_eq!(
                &header[0..29],
                "OAuth oauth_consumer_key=\"2\","
            );
            assert_eq!(
                &header[56..91],
                "oauth_signature_method=\"HMAC-SHA1\","
            );
            assert_eq!(
                &header[120..136],
                "oauth_token=\"4\",",
            );
        }
    }
}
