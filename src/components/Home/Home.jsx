import { open } from '@tauri-apps/plugin-dialog';
import './Home.css'
import * as path from '@tauri-apps/api/path'
import { exists, mkdir, readFile, writeFile } from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';
import ePub from 'epubjs';

const BOOK_DIRECTORY_PATH = 'eReaderBooks'
const DATA_FILE_NAME = 'data.json';
const home = await path.localDataDir();
async function handleNewBook() {
    const selected = await open({
        multiple: false,
        directory: false,
        filters: [{ name: 'Ebooks', extensions: ['epub'] }]
    })
    let booksDirectoryStatus = await invoke('create_books_directory', {"bookDirectoryPath": (await path.join(home, BOOK_DIRECTORY_PATH)).toString()});
    if (!booksDirectoryStatus) {
        return;
    }
    let dataFileStatus = await invoke('create_data_file', {"dataFilePath": (await path.join(home, BOOK_DIRECTORY_PATH, DATA_FILE_NAME)).toString()});
    if (!dataFileStatus) {
        return;
    }
    let data = JSON.parse(await invoke('read_data_file', {"dataFilePath": (await path.join(home, BOOK_DIRECTORY_PATH, DATA_FILE_NAME)).toString()}));
    if(Object.hasOwn(data, 'status')) return;
    await invoke('copy_book', {
        "bookFilePath": selected.toString(),
        "bookDirectoryPath": (await path.join(home, BOOK_DIRECTORY_PATH)).toString(),
        "bid": data.bid + 1
    })
    let bookPath = await path.join(home, BOOK_DIRECTORY_PATH, (data.bid+1).toString(), 'book.epub');
    let book_bin = await readFile(bookPath)
    let book = ePub(book_bin, "binary");
    
    let coverPath = await book.loaded.cover;
    if(coverPath){
        let coverName = coverPath.split('/').pop();
        coverPath = coverPath.substring(1);
        let coverWritePath = await path.join(home,BOOK_DIRECTORY_PATH, (data.bid+1).toString(), coverName); 
        JSZip.loadAsync(book_bin).then(async (zip) => {
            let coverImageData = await zip.file(coverPath).async("uint8array");
            await writeFile(coverWritePath, coverImageData);
        })
    }
    data.books.push({
        "bid": data.bid+1,
        "title": (await book.loaded.metadata).title,
        "bookFolderPath": (await path.join(home, BOOK_DIRECTORY_PATH, (data.bid+1).toString())).toString(),
        "bookFilePath": (await path.join(home, BOOK_DIRECTORY_PATH, (data.bid+1).toString(), 'book.epub')).toString(),
        "coverImagePath": (coverPath != null)? coverWritePath.toString() : "",
    })
    data.bid += 1;
    data.book_count += 1;
    await invoke("write_data_file", {dataFilePath: (await path.join(home, BOOK_DIRECTORY_PATH, DATA_FILE_NAME)).toString(), data: JSON.stringify(data)});
}
function Home () {



    return (
        <button class='createBtn' onClick={handleNewBook}>+</button>
    )
} 

export default Home;