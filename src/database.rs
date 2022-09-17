pub fn connect() -> sqlite::Connection {
    let connection = sqlite::open(".sqlite").unwrap();

    let db = connection.execute("CREATE TABLE tweet_ids (tweet_id TEXT);");

    match db {
        Ok(()) => {
            println!("Created new 'tweet_ids' table");
            connection
        }
        Err(_) => {
            println!("Found 'tweet_ids' table");
            connection
        }
    }
}

pub fn already_sent(connection: &sqlite::Connection, id: &str) -> bool {
    let mut already_sent: bool = false;
    let query = format!("SELECT * FROM tweet_ids WHERE tweet_id='{}';", id);

    connection
        .iterate(&query, |_| {
            already_sent = true;

            true
        })
        .unwrap();

    already_sent
}
