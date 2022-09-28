pub fn connect(db_name: &str) -> sqlite::Connection {
    let connection = sqlite::open(db_name).unwrap();

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

pub fn insert_id(connection: &sqlite::Connection, id: &str) {
    let statement = format!("INSERT INTO tweet_ids VALUES ('{}');", id);

    match connection.execute(&statement) {
        Ok(_) => {}
        Err(error) => {
            println!("{}", error);
            panic!("Unable to save id");
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

#[cfg(test)]
mod test {
    use serial_test::serial;
    use crate::database::connect;

    pub const TEST_DB: &str = ".test.sqlite";

    pub fn reset_db(connection: &sqlite::Connection) {
        connection.execute("DELETE FROM tweet_ids;").unwrap();
    }

    mod insert_id {
        use crate::database::insert_id;

        #[test]
        #[super::serial]
        fn should_insert_id_to_db() {
            let conn = super::connect(super::TEST_DB);

            for i in 0..5 {
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

            super::reset_db(&conn);
        }
    }

    mod already_sent {
        use crate::database::already_sent;

        #[test]
        #[super::serial]
        fn should_return_false_if_id_not_in_db() {
            let conn = super::connect(super::TEST_DB);

            assert!(!already_sent(&conn, "1"));
            assert!(!already_sent(&conn, "2"));
            assert!(!already_sent(&conn, "3"));

            super::reset_db(&conn);
        }

        #[test]
        #[super::serial]
        fn should_return_true_if_id_in_db() {
            let conn = super::connect(super::TEST_DB);

            conn.execute(
                "
                INSERT INTO tweet_ids VALUES ('1');
                INSERT INTO tweet_ids VALUES ('2');
                INSERT INTO tweet_ids VALUES ('3');
                ",
            )
            .unwrap();

            assert!(already_sent(&conn, "1"));
            assert!(already_sent(&conn, "2"));
            assert!(already_sent(&conn, "3"));

            super::reset_db(&conn);
        }
    }
}
