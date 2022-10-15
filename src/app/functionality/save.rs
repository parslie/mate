use std::{io::{self, Write}, path::Path, fs::File};

use crate::app::Data;

pub fn save(data: &mut Data, force_overwrite: bool) -> Result<bool, io::Error> {
    let file_exists = Path::new(data.save_prompt.get_answer().as_str()).exists();

    if file_exists && force_overwrite || !file_exists {
        let mut file = File::create(data.save_prompt.get_answer().as_str())?;
        file.write_all(data.file.to_string().as_bytes())?;
    } else {
        return Ok(false); // Will not overwrite file
    }

    return Ok(true);
}