use rusqlite::{Connection, Result as sqlresult};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    name: String,    // changed into name, before it was day
    total_time: u16,
    wake_up: String,
    sleep: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    schedule: Vec<Day>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    users: Vec<User>,
}

impl Config {
    pub fn new() -> Config {
        // let raw_users = find a way to extract data from the database
        let conn = Connection::open("test.db").expect("Error 101 - It was not be possible connect to the database");

        let mut stmt = conn
            .prepare("SELECT id, name, schedule FROM utenti")
            .expect("Error 201 - It was not be possible load the database");
        let person_iter = stmt
            .query_map([], |row| {
                let temp: String = row.get(2).expect("Error 301 - It was not be possible query the database");
                let b: Vec<Day> = serde_json::from_str(&temp).unwrap();
                Ok(User {
                    name: row.get(1).expect("Error 401 - It was not be possible load the name's row"),  // Error messages changed
                    schedule: b,
                })
            })
            .unwrap();

        let mut users: Vec<User> = Vec::new();

        for person in person_iter {
            users.push(person.unwrap());
        }

        // for every user in raw_users;
        //   create empty vec

        //   create all days for a given schedule
        //   pack all days into the vector created before
        //   create a new User
        //   users.push(user);
        //println!("{}", raw_users);
        Config { users }
    }
}
