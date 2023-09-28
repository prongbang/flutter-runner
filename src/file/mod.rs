use std::fs;
use std::path::Path;

pub fn check_or_create_is_not_exist_dir(dir: &str) {
    // Create a Path object from the file path
    let path = Path::new(dir);

    // Extract the directory path
    if let Some(dir) = path.parent() {
        // The 'dir' variable contains the directory path
        if let Some(dir_str) = dir.to_str() {
            // Check if the directory exists
            if !fs::metadata(dir_str).is_ok() {
                // If it doesn't exist, create it
                fs::create_dir(dir_str).expect("Cannot create directory");
            }
        }
    } else {
        println!("Invalid file path: {}", dir);
    }
}
