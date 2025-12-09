use std::fs;

pub fn read_instructions( filename: &str ) -> Result<Vec<String>, String> { 
    let contents = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file: {} with error: {}", filename, e))?;
    Ok(contents.split('\n').map(|s| s.to_string()).collect::<Vec<String>>())
}
