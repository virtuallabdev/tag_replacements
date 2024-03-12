use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let mut file_content = String::new();
    let mut tag_vec = Vec::new();

    // Read the XML file content
    let mut file = File::open("your_file.xml")?;
    file.read_to_string(&mut file_content)?;

    // Regex to capture opening and closing HTML tags
    let tag_regex = Regex::new(r"<[^>]+>")?;

    // Find all HTML tags and store them in the vector
    for cap in tag_regex.find_iter(&file_content) {
        tag_vec.push(cap.as_str().to_owned());
    }

    // Replace all HTML tags with the special character "§"
    let mut processed_content = file_content.clone();
    for tag in tag_vec.iter() {
        processed_content = processed_content.replace(tag, "§");
    }

    // Print the processed content with replaced tags
    println!("Content with replaced tags: {}", processed_content);

    // Restore the original HTML tags by replacing "§" back
    let mut final_content = processed_content.clone();
    for tag in tag_vec.iter() {
        final_content = final_content.replace("§", tag);
    }

    // Print the final content with restored tags
    println!("Content with restored tags: {}", final_content);

    Ok(())
}