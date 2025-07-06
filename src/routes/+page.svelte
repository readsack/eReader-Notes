<script>
  import { open } from "@tauri-apps/plugin-dialog";
  import {path} from "@tauri-apps/api"
  import { invoke } from '@tauri-apps/api/core';
  var count = $state("No File Selected");

  async function uploadFileDialog() {
    const file = await open({
      multiple: false,
      directory: false,
    });
    console.log(file)
    if (file) {
      count = file;
      invoke("handle_book_upload", { bookPath: file })
    }
  }
</script>

<div>
  {count}
  <button onclick={uploadFileDialog}>Upload Book</button>
</div>

<style></style>
