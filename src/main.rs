use std::fs::{File, OpenOptions};
use std::io::{Read,Write};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    data: Vec<String>, // Adjust the type based on your actual vector content
}


fn save_file(filenane: &str, folder: &str, content: &str) -> std::io::Result<()> {
    //let data_to_save = Data { data: content.clone() };
    //let serialized_data = serde_json::to_string(&data_to_save).unwrap();
    let base_path = "C:/Users/user/Documents/repos/rust/tag_replacements/test_files";
    let path_str:&str = &(format!("{}/{}/{}", &base_path, &folder, &filenane));
    let path: &Path = Path::new(&path_str);
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();

    Ok(())
}

fn convert(path: &str) -> std::io::Result<()> {
    let mut file_content = String::new();
    let mut tag_vec = Vec::new();
    let placeholder: &str = " § ";

    // Read the XML file content
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut file_content).unwrap();

    

    //let p: Path = Path::new(*path);

    let converted_filename = format!("{}{}", "cooonv", ".txt");

    // Regex to capture opening and closing HTML tags
    let tag_regex = Regex::new(r"<[^>]+>").unwrap();

    // Find all HTML tags and store them in the vector
    for cap in tag_regex.find_iter(&file_content) {
        tag_vec.push(cap.as_str().to_owned());
    }

    let data_to_save = Data { data: tag_vec.clone() };
    let serialized_data = serde_json::to_string(&data_to_save).unwrap();

     // Replace all HTML tags with the special character "§"
     let mut processed_content = file_content.clone();
     for tag in tag_vec.iter() {
         processed_content = processed_content.replace(tag, placeholder);
     }

    save_file(".converted.txt", "converted",  &processed_content).unwrap();
    save_file(".vec.json", "tmp", &serialized_data).unwrap();


    Ok(())
}



fn revert(path: &str) -> std::io::Result<()> {

    Ok(())
}


fn worker() -> std::io::Result<()> {
    
    convert("C:/Users/user/Documents/repos/rust/tag_replacements/test_files/test.xml");

    //C:/Users/user/Documents/repos/rust/tag_replacements/test_files/test.xml

   
    // let mut file = File::create("C:/Users/user/Documents/repos/rust/tag_replacements/test_files/vec.json")?;
    // file.write_all(serialized_data.as_bytes())?;




   

    // let converted_file_path = "C:/Users/user/Documents/repos/rust/tag_replacements/test_files/converted.txt";

    // // Open the file in write mode (will create if it doesn't exist)
    // let mut converted_file = File::create(converted_file_path)?;

    // // Write the string content to the file
    // converted_file.write_all(processed_content.as_bytes())?;
    // // Print the processed content with replaced tags
    // println!("Content with replaced tags: {}", processed_content);


    //---------------------------


   

    // // Restore the original HTML tags by replacing "§" back
    // let mut final_content = processed_content.clone();
    // for tag in tag_vec.iter() {
    //     final_content = final_content.replace(placeholder, tag);
    // }

    // let translated_file_path = "C:/Users/user/Documents/repos/rust/tag_replacements/test_files/translated.txt";

    // // Open the file in write mode (will create if it doesn't exist)
    // let mut translated_file = File::create(translated_file_path)?;
    // translated_file.write_all(final_content.as_bytes())?;


    // // Print the final content with restored tags
    // println!("Content with restored tags: {}", final_content);
    Ok(())
}



fn main() -> std::io::Result<()> {
    worker()?;
    Ok(())
}