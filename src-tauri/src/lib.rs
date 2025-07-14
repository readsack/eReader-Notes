mod book_import;




#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![book_import::create_books_directory, book_import::create_data_file, book_import::read_data_file, book_import::copy_book, book_import::write_data_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
