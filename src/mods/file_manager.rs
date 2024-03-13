use std::{fs, path::Path, path::PathBuf};
use log::{error, info, warn};
use fs_extra::{move_items};
use fs_extra::dir::CopyOptions;
use glob::{glob, Paths};


pub fn create_folder(folder: &str) -> std::io::Result<()> {
    let path = Path::new(folder);
    if !path.exists() {
        match fs::create_dir_all(path) {
            Ok(_) => info!("Folder created: {:?}", &path.display()),
            Err(e) => {
                error!("Cannot create folder:{:?} -- Error: {e}", &path.display());
                println!("1");
                panic!()
            }
        }
    }
    Ok(())
}



pub fn move_file_to(source: &str, destination: &str) -> std::io::Result<()> {

    let mut source_path: Vec<PathBuf> = vec![];
    let destination_path = Path::new(&destination);

    let s = Path::new(&source);
    source_path.push(s.to_path_buf());

    let options = CopyOptions::new();
    match move_items(&source_path, &destination_path, &options) {
        Ok(_) => info!("Files have been moved to folder: {:?}", &destination_path),
        Err(e) => {
            error!("Files cannot be moved from folder: {:?} -  Error details: {:?}", &destination_path, e.to_string());
            println!("1");
            panic!()
        }
    }
    Ok(())
}



pub fn folder_file_list (folder: &str) -> Result<Vec<PathBuf>, std::io::Error> {

    let mut list= Vec::new();
    let pattern: &str = &(folder.to_owned() + "/**/*.csv");

    let entries: Paths; 
    match glob(&pattern) {
        Ok(o) => entries = o,
        Err(e) => {
            error!("Failed to read file pattern, error: {}", e.msg);
            println!("1");
            panic!();
        },
    }
    
    for entry in entries {
        match entry {
            Ok(path) => {
                info!("Add file to list: {:?}", &path.display());
                //list.push(path.clone());
                list.push(path);
            },
            Err(e) => error!("Failed to read file pattern - {:?}", e.to_string()),
        }
    }

    match list.len() > 0 {
        true => {
            info!("Files list created, total files: {:?}", list.len());
            Ok(list)
        },
        false => {
            warn!("The folder is empty: {:?}", &folder);
            println!("1");
            panic!()
        },
    }
}




pub fn remove_folder_content (folder: &str) -> std::io::Result<()> {
    let folder_path = Path::new(&folder); 

    match folder_path.exists() {
        true => {
            for entry in fs::read_dir(folder_path)? {
                if let Err(e) = fs::remove_file(&entry?.path()) {
                    error!("Not able to delete previous files -- Error: {e}");
                }
            }
            info!("Successfully removed previous files from directory: {:?}", &folder);
            Ok(())
        },
        false => {
            info!("No directory to delete");
            Ok(())
        },
    }
}