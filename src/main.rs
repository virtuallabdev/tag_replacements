use log4rs;
use log::info;
use std::env;

use anyhow::{Context, Result, Error};
use tokio;

use std::process;

mod mods;
//use mods::worker::process_files;



#[tokio::main]
async fn main() -> anyhow::Result<(), Error> {

    info!("{:?}", env::current_dir().unwrap().to_str().unwrap().to_owned() + "\\config\\log4rs.yml");
    log4rs::init_file(env::current_dir().unwrap().to_str().unwrap().to_owned() + "\\config\\log4rs.yml", Default::default()).unwrap();

    //let list = file_manager::get_file_list("D:\\Temp\\test\\test_folder").await?;

    // match process_files().await? {
    //     true => println!("0"), // everhthing is ok
    //     false => println!("2"), // warning email
    // }
 
    Ok(())
}