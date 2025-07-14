use std::{fs, io::prelude::*, path::Path, path};

use serde_json::json;
use tauri::utils::config::parse::parse_json;


#[tauri::command]
pub fn create_books_directory(book_directory_path: String) -> bool {
    let dir_exists = fs::exists(&book_directory_path);
   // println!("Checking if directory exists: {}", &book_directory_path);
    match dir_exists{
        Ok(val) => {
            if val { return true };
            match fs::create_dir_all(&book_directory_path) {
                Ok(_) => return true,
                Err(e) => {
                    println!("Failed to create directory: {}", e);
                    return false;
                }
            }
        },
        Err(_) => {
            return false;
        }
    }
}

#[tauri::command]
pub fn create_data_file(data_file_path: String) -> bool {
    let data_file_exists = fs::exists(&data_file_path);
   // println!("Checking if directory exists: {}", &book_directory_path);
    match data_file_exists{
        Ok(val) => {
            if val { return true };
            match fs::File::create_new(&data_file_path) {
                Ok(mut f) => {
                    let init_data = json!({
                        "book_count": 0,
                        "bid": 0,
                        "books": [],
                    });
                    match f.write_all(init_data.to_string().as_bytes()){
                        Ok(_) => return true,
                        Err(e) => {
                            println!("Failed to write to data file: {}", e);
                            return false;
                        }
                    }; 
                    
                },
                Err(e) => {
                    println!("Failed to create data file: {}", e);
                    return false;
                }
            }

        },
        Err(_) => {
            return false;
        }
    }
}

#[tauri::command]
pub fn read_data_file(data_file_path: String) -> String {
    match fs::File::open(&data_file_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    return contents;
                },
                Err(e) => {
                    println!("Failed to read data file: {}", e);
                    return json!({"status": false}).to_string();
                }
            }
        },
        Err(e) => {
            println!("Failed to open data file: {}", e);
            return json!({"status": false}).to_string();
        }
    }
}

#[tauri::command]
pub fn copy_book(book_file_path: String, book_directory_path: String, bid: u64) -> bool {

    let book_folder_path = Path::new(&book_directory_path).join(bid.to_string());
    match fs::create_dir(&book_folder_path) {
        Ok(_) => {
            
            match fs::copy(&book_file_path, &book_folder_path.join("book.epub")) {
                Ok(_) => return true,
                Err(e) => {
                    println!("Failed to copy book file: {}", e);
                    match fs::remove_dir_all(&book_folder_path) {   
                        Ok(_) => println!("Removed book folder after copy failure."),
                        Err(e) => println!("Failed to remove book folder: {}", e),
                    }
                    return false;
                }
            }
        },
        Err(e) => {
            println!("Failed to create book directory: {}", e);
            return false;
        }
    }
}

#[tauri::command]
pub fn write_data_file(data_file_path: String, data: String) -> bool {
    match fs::File::create(&data_file_path) {
        Ok(mut file) => {
            match file.write(data.as_bytes()) {
                Ok(_) => return true,
                Err(e) => {
                    println!("Failed to write to data file: {}", e);
                    return false;
                }
            }
        },
        Err(e) => {
            println!("Failed to create data file: {}", e);
            return false;
        }
    }
}