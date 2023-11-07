use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    let conn = Connection::open("./db/test.db").expect("open db error");
    let _ = conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    );

    let mut stmt = conn
        .prepare("SELECT id, name, data FROM person")
        .expect("select error");

    let person_iter = stmt
        .query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })
        .expect("");

    for person in person_iter {
        // person.unwrap();
        println!("Found person {:?}", person.unwrap());
    }
    println!("Hello, world!");
}
