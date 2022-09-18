use dotenv::dotenv;

pub struct Env {
    bearer_token: String,
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    token_secret: String,
}

impl Env {
    pub fn new() -> Self {
        dotenv().ok();

        let bearer_token =
            std::env::var("BEARER_TOKEN").expect("BEARER_TOKEN environment variable must be set");
        let consumer_key =
            std::env::var("API_KEY").expect("API_KEY environment variable must be set.");
        let consumer_secret = std::env::var("API_SECRET_KEY")
            .expect("API_SECRET_KEY environment variable must be set.");
        let access_token =
            std::env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN environment variable must be set.");
        let token_secret = std::env::var("ACCESS_TOKEN_SECRET")
            .expect("ACCESS_TOKEN_SECRET environment variable must be set.");

        Env {
            bearer_token,
            consumer_key,
            consumer_secret,
            access_token,
            token_secret,
        }
    }

    pub fn get_bearer_token(&self) -> &String {
        &self.bearer_token
    }

    pub fn get_consumer_key(&self) -> &String {
        &self.consumer_key
    }

    pub fn get_consumer_secret(&self) -> &String {
        &self.consumer_secret
    }

    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }

    pub fn get_token_secret(&self) -> &String {
        &self.token_secret
    }
}
