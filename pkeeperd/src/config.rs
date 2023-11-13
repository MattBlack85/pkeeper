use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Day {
    name: String,
    total_time: u16,
    wake_up: String,
    sleep: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    schedule: Vec<Day>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    users: Vec<User>,
}

impl Config {
    pub fn new() -> Config {
        // let raw_users = find a way to extract data from the database
      pub fn extract_users() -> Result<()> {
        let raw_users = r#"
        {
          "users": [
        {
            "name": "foo",
            "schedule": [
        {
          "day": "monday",
          "total_time": 180,
          "wake_up": "09:00",
          "sleep": "21:00"
        }, {
          "day": "wednesday",
          "total_time": 60,
          "wake_up": "09:00",
          "sleep": "21:00"
        }
          ]
        }, {
          "name": "bar",
          "schedule": [
        {
          "day": "monday",
          "total_time": 180,
          "wake_up": "09:00",
          "sleep": "21:00"
        }, {
          "day": "friday",
          "total_time": 60,
          "wake_up": "09:00",
          "sleep": "21:00"
        }
          ]
        }"#;

        let data: Config = serde_json::from_str(raw_users)?;
        Ok(())
      }

        let p = extract_users();
        let users: Vec<User> = Vec::new();
        
        // for every user in raw_users;
        //   create empty vec
        //   create all days for a given schedule
        //   pack all days into the vector created before
        //   create a new User
        //   users.push(user);
        Config { users }
    }
}

