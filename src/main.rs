use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    entry: String,
    token: String,
}

fn config() -> Result<Config, Box<dyn Error>> {
    let file = File::open("./config.json")?;
    let reader = BufReader::new(file);

    let data = serde_json::from_reader(reader)?;

    Ok(data)
}

fn main() -> rusqlite::Result<()> {
    let config = config().unwrap();
    print!("{}", config.entry);
    print!("{}", config.token);

    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        params![],
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map(params![], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
