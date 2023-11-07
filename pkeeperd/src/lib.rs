pub mod config;
pub mod db;

#[cfg(target_family = "windows")]
fn retrieve_users() {}

#[cfg(target_family = "unix")]
fn retrieve_users() {}

pub fn read_config() {
    // Query the database and create a new config::Config
}

#[allow(dead_code)]
fn write_config() {
    todo!()
}

#[allow(dead_code)]
fn check_remaining_time() {
    todo!()
}
