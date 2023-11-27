use pkeeperd::config;
use pkeeperd::db;
use std::{thread, time};

#[allow(dead_code)]
fn main() {
    db::init_db();
    let pconfig = config::Config::new();

    loop {
        println!("checking");
        config::Config::new();
        let sleep = time::Duration::from_millis(2500);
        thread::sleep(sleep);
    }
}
