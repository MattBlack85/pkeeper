pub struct Day {
    name: String,
    total_time: u16,
    wake_up: String,
    sleep: String,
}

pub struct User {
    name: String,
    schedule: Vec<Day>,
}

pub struct Config {
    users: Vec<User>,
}

impl Config {
    pub fn new() -> Config {
        // let raw_users = find a way to extract data from the database
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
