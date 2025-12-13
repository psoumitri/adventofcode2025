use std::fs;
use std::env;

pub fn read_lines( filename: &str ) -> Result<Vec<String>, String> { 
    let path = env::current_dir()
        .map_err(|e| format!("Failed to resolve path: {}", e))?
        .display()
        .to_string();
    let fullpath = path + "/src/" + filename;    
    let contents = fs::read_to_string(fullpath)
        .map_err(|e| format!("Failed to read file: {} with error: {}", filename, e))?;
    Ok(contents.split('\n').map(|s| s.to_string()).collect::<Vec<String>>())
}
