use std::{thread, time};

fn main() {
    pkeeperd::read_config();

    loop {
        println!("checking");
        let sleep = time::Duration::from_millis(2500);
        thread::sleep(sleep);
    }
}
