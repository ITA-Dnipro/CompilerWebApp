#[macro_use] extern crate rocket;

mod http_handlers;
mod filework;
mod languages;

use std::env::{var, set_var, current_dir, args};
use http_handlers::submit;
use rocket::fs::FileServer;
use languages::{static_info::cpp, lang_info::LangInfo};
use std::collections::hash_map::HashMap;

type LangsInfo = HashMap<String, LangInfo>;

#[launch]
fn rocket() -> _ 
{
    let dir_check = std::thread::spawn(check_temp_dir);

    let mut langs_info = LangsInfo::new();
    langs_info.insert("c++".to_owned(), cpp::construct());

    dir_check.join().expect("Fatal error while resolving temp dir path");

    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![submit::post_submit])
        .manage(langs_info)
}

fn check_temp_dir()
{
    let mut env_args = args();
    env_args.next();    // Skip executable's path
    let user_temp_folder = env_args.next();
    
    match user_temp_folder
    {
        Some(temp_dir) => 
        {
            println!("COMPILATION_TEMP_DIR specified by the user.");
            // Check if the value is a dir path
            if std::path::Path::new(&temp_dir).is_dir()
            {
                println!("Setting \"{}\" as COMPILATION_TEMP_DIR.", temp_dir);
                set_var("COMPILATION_TEMP_DIR", temp_dir);

                return;
            }
            else
            {
                println!("\"{}\" is not a valid directory path.", temp_dir);
            }

        },
        None => {}
    }

    match var("COMPILATION_TEMP_DIR") 
    {
        Ok(temp_dir) => 
        {
            println!("COMPILATION_TEMP_DIR already exists.");
            // Check if the value is a dir path
            if std::path::Path::new(&temp_dir).is_dir()
            {
                return;
            }
            else
            {
                println!("\"{}\" is not a valid directory path.", temp_dir);
            }
        },
        Err(_) => 
        {
            println!("Can't read COMPILATION_TEMP_DIR.");
        }
    }
    // Set COMPILATION_TEMP_DIR with a default value
    let mut temp_dir = current_dir().unwrap().to_str().unwrap().to_owned();
    temp_dir.push_str("/tempdata");

    println!("Using default path \"{}\".", temp_dir);
    set_var("COMPILATION_TEMP_DIR", temp_dir);
}
