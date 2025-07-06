// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use serde_json::json;
const BOOK_DIR:&str = "\\Documents\\eReaderBooks";

#[tauri::command]
fn handle_book_upload(book_path: String) {
  println!("{}", book_path);
  
  let books_folder = String::from(env::home_dir().unwrap().to_str().unwrap()) + BOOK_DIR;
  if(Path::new(&books_folder).exists()){
    book_uploaded(book_path);
  }
  else{
    match fs::create_dir(books_folder){
        Ok(_) => {
            book_uploaded(book_path);
        },
        Err(e) => {
            println!("Cannot Create Books Directory \n {}", e);
            return;
        }
    }

  }
}


fn book_uploaded(book_path: String){
    let books_folder = String::from(env::home_dir().unwrap().to_str().unwrap()) + BOOK_DIR;
    let mut book_name = PathBuf::from(book_path.clone());
    book_name.set_extension("");
    let mut book_folder_path = books_folder.clone() + "\\" + book_name.file_name().unwrap().to_str().unwrap();
    let data_file = String::from(books_folder.clone() + "/data.json");
    let data_path = Path::new(&data_file);
    match fs::create_dir(book_folder_path.clone()){
        Ok(_) => {
            fs::copy(book_path.clone(), book_folder_path.clone() + "\\" + Path::new(&book_path.clone()).file_name().unwrap().to_str().unwrap())
                .expect("Cannot Copy Book File");
        }
        Err(e)=> {
            println!("Cannot Create Book's Directory \n {}", e);
            return;
        }
    }
    let mut id;
    if !data_path.exists(){
        id = 1;
        let data = json!({
            "book_count": 1,
            "books": [
                {
                    "name": book_name,
                    "id": 1,
                    "path": book_folder_path.clone(),
                }
            ]
        });
        match fs::write(data_file, data.to_string()){
            Ok(_) => {
//                println!("Book DataUploaded Successfully");
            },
            Err(e) => {
                println!("Cannot Write Data File \n {}", e);
                return;
            }
        }
    }
    else{
        let mut data = match fs::read_to_string(data_file.clone()){
            Ok(data) => {
                match serde_json::from_str::<serde_json::Value>(&data){
                    Ok(data) => data,
                    Err(e) => {
                        println!("Cannot Parse Data File \n {}", e);
                        return;
                    }
                }
            },
            Err(e) => {
                println!("Cannot Read Data File \n {}", e);
                return;
            }
        };
        let book_count = data["book_count"].as_u64().unwrap() + 1;
        id = book_count.clone();
        let mut data_copy = data.clone();
        let books = data_copy["books"].as_array_mut().unwrap();
        books.push(json!({
            "name": book_name,
            "id": book_count,
            "path": book_folder_path.clone(),
        }));
        data["book_count"] = book_count.into();
        data["books"] = json!(books);
        match fs::write(data_file.clone(), data.to_string()){
            Ok(_) => {},
            Err(e) => {
                println!("Cannot Write Data File \n {}", e);
                return;
            }
        }
    }


}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![handle_book_upload])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
