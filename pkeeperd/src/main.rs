use std::{thread, time};

#[allow(dead_code)]
fn main() {
    pkeeperd::read_config();

    loop {
        println!("checking");


//        let path = String::from("/pkeeper/config.ini");
//        let slice = path[..path.len() - 10].to_string();
//        println!("{}", slice);


        let sleep = time::Duration::from_millis(2500);
        thread::sleep(sleep);
    }
}
