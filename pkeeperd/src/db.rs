pub fn init_db() {
    let connection = sqlite::open("test.db").unwrap();
    let query =
        "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, schedule TEXT);";
    connection.execute(query).unwrap();
}
