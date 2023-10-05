use dirs;
use std::fs;
use std::fs::File;
use std::path::Path;

#[cfg(target_family = "windows")]
fn retrieve_users() {}

#[cfg(target_family = "unix")]
//fn retrieve_users() {}


//  first attempt
// fn check_config_exists(path: &String){
//    if Path::new(path).exists() {
//        
//    }
//    else{
  //      match fs::create_dir(path[..path.len() - 10].to_string()) {
  //          Ok(()) => (),
  //          Err(e) => println!("Got an error: {}", e),
  //      }
  //  }
//}


//  second attempt
 fn check_config_exists(path: &String) {
    if Path::new(path).exists(){

    } 
    
    else if Path::new(&path[..path.len() - 10].to_string()).exists() {
        if !Path::new(path).exists() {
            match File::create(path) {
                Ok(_) => (),
                Err(e) => println!("Got an error: {}", e),
            }
        }
    }

    else {
         match fs::create_dir(path[..path.len() - 10].to_string()) {
             Ok(()) => (),
             Err(e) => println!("Got an error: {}", e),
         }

         match File::create(path) {
             Ok(_) => (),
             Err(e) => println!("Got an error: {}", e),
         }
    }
 }





//  CODICE ORIGINALE
// fn check_config_exists(path: &String) {
//     if !Path::new(path).exists() {
//         match fs::create_dir(path[..path.len() - 10].to_string()) {
//             Ok(()) => (),
//             Err(e) => println!("Got an error: {}", e),
//         }

//         match File::create(path) {
//             Ok(_) => (),
//             Err(e) => println!("Got an error: {}", e),
//         }
//     }
// }

pub fn read_config() {
    match dirs::config_local_dir() {
        Some(config_path) => {
            let full_path = format!("{}/pkeeper/config.ini", &config_path.display());
            check_config_exists(&full_path);
//            let _file = File::open(&full_path).unwrap();
        }
        None => std::process::exit(101),
    }
}

#[allow(dead_code)]
fn write_config() {
    todo!()
}

#[allow(dead_code)]
fn check_remaining_time() {
    todo!()
}
