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

pub fn insert_id(connection: &sqlite::Connection, id: &str) -> () {
    let statement = format!("INSERT INTO tweet_ids VALUES ('{}');", id);

    match connection.execute(&statement) {
        Ok(_) => {},
        Err(error) => println!("{}", error),
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


#[cfg(test)]
mod test {
    fn reset_db(connection: &sqlite::Connection) {
        connection.execute("DELETE FROM tweet_ids;")
            .unwrap();
    }

    mod insert_id {
        use super::reset_db;
        use crate::database::{connect, insert_id};

        #[test]
        fn should_insert_id_to_db() {
            let conn = connect(".test.sqlite");

            for i in 0..3 {
                insert_id(&conn, &i.to_string())
            }

            let mut expected = 0;

            conn.iterate("SELECT * FROM tweet_ids;", |pairs| {
                for &(_column, value) in pairs.iter() {
                    assert_eq!(value.unwrap(), expected.to_string());
                    expected += 1;
                }

                true
            })
            .unwrap();

            reset_db(&conn);
        }
    }
}
