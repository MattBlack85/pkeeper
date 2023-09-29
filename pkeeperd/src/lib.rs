use dirs;
use std::fs::File;
use std::path::PathBuf;

fn check_config_exists() {}

pub fn read_config() {
    let config_dir: Option<PathBuf> = dirs::config_local_dir();

    if let Some(config_path) = config_dir {
        check_config_exists();
        let file = File::open(format!("{}/pkeeper/config.ini", &config_path.display())).unwrap();
    };
}

fn write_config() {
    todo!()
}

fn check_remaining_time() {
    todo!()
}
