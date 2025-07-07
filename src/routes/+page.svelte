<script>
  import { open } from "@tauri-apps/plugin-dialog";
  import {path} from "@tauri-apps/api"
  import { invoke } from '@tauri-apps/api/core';
  import { readFile, BaseDirectory } from '@tauri-apps/plugin-fs';
  var count = $state("No File Selected");
  import ePub from "epubjs"
  //import { ePub } from "$lib/epub.min.js";
  
  async function uploadFileDialog() {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          "name": "ebooks",
          "extensions": ["epub", "mobi", "azw3"]
        }
      ]
    });
    console.log(file)
    if (file) {
      invoke("handle_book_upload", { bookPath: file })
      openBook(file)
      
    }
  }


  function openBook(bookPath){
    readFile(bookPath).then((e) => {
        let book = ePub(e, "binary");
        let rendition = book.renderTo('book')
        rendition.display().then(() => {
          rendition.next()
        });
      })
  }

</script>
<div>
  {count}
  <button onclick={uploadFileDialog}>Upload Book</button>
  <div id="book"></div>

</div>

<style></style>
