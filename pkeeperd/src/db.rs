pub fn init_db() {
    let connection = sqlite::open("test.db").unwrap();
    let query = "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, config TEXT);";
    connection.execute(query).unwrap();
}
