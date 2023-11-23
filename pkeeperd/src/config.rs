use rusqlite::{Connection, Result as sqlresult};
use serde::{Deserialize, Serialize};
use serde_json::Result;

//  fn read_json_typed<T>(raw_json: &str) -> T {
//      let parsed: T = serde_json::from_str(raw_json).unwrap();
//      return parsed;
//  }

#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    day: String,
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

        let conn = Connection::open("/Users/matteo/Desktop/prova.db").expect("errore");

        let mut stmt = conn
            .prepare("SELECT id, name, schedule FROM utenti")
            .expect("errore");
        let person_iter = stmt
            .query_map([], |row| {
                let temp: String = row.get(2).expect("errore");
                let b: Vec<Day> = serde_json::from_str(&temp).unwrap();
                Ok(User {
                    name: row.get(1).expect("errore"),
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
