use std::fs::{File, OpenOptions};
use std::io::{Read,Write};
use regex::Regex;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Data {
    data: Vec<String>, // Adjust the type based on your actual vector content
}



fn worker() -> std::io::Result<()> {
    let mut file_content = String::new();
    let mut tag_vec = Vec::new();
    let placeholder: &str = " § ";

    // Read the XML file content
    let mut file = File::open("C:/Users/user/Documents/repos/rust/tag_replacements/test_files/test.xml")?;
    file.read_to_string(&mut file_content)?;

    // Regex to capture opening and closing HTML tags
    let tag_regex = Regex::new(r"<[^>]+>").unwrap();

    // Find all HTML tags and store them in the vector
    for cap in tag_regex.find_iter(&file_content) {
        tag_vec.push(cap.as_str().to_owned());
    }


    let data_to_save = Data { data: tag_vec };
    let serialized_data = serde_json::to_string(&data_to_save)?;




    // Replace all HTML tags with the special character "§"
    let mut processed_content = file_content.clone();
    for tag in tag_vec.iter() {
        processed_content = processed_content.replace(tag, placeholder);
    }

    let converted_file_path = "C:/Users/user/Documents/repos/rust/tag_replacements/test_files/converted.txt";

    // Open the file in write mode (will create if it doesn't exist)
    let mut converted_file = File::create(converted_file_path)?;

    // Write the string content to the file
    converted_file.write_all(processed_content.as_bytes())?;


    // Print the processed content with replaced tags
    println!("Content with replaced tags: {}", processed_content);

    // Restore the original HTML tags by replacing "§" back
    let mut final_content = processed_content.clone();
    for tag in tag_vec.iter() {
        final_content = final_content.replace(placeholder, tag);
    }

    let translated_file_path = "C:/Users/user/Documents/repos/rust/tag_replacements/test_files/translated.txt";

    // Open the file in write mode (will create if it doesn't exist)
    let mut translated_file = File::create(translated_file_path)?;
    translated_file.write_all(final_content.as_bytes())?;


    // Print the final content with restored tags
    println!("Content with restored tags: {}", final_content);
    Ok(())
}



fn main() -> std::io::Result<()> {
    worker()?;
    Ok(())
}