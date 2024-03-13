
use crate::mods::file_manager::*;
use crate::mods::settings::*;
use crate::mods::converter::*;

use anyhow::Error;
use log::{error, info, warn};
use serde::*;




pub async fn process_files() -> std::io::Result<bool> {
    let settings = Settings::new().unwrap();

    //convert("C:/Lavori/Repos/tag_replacements/test_files/unprocessed/CMS-TEXTE-IT-20230922-145744.XML").unwrap();
    
    let is_ok = true;


    Ok(is_ok)
}
