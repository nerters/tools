<template>
  <div class="alarm-box" @dblclick="close" >
    <div class="content" v-html="title">
      
    </div>
  </div>
</template>
  
  <script setup lang="ts">
  import { onMounted, ref } from 'vue';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';
  const appWindow = getCurrentWindow();

  const title = ref("")
  const cronId = ref("")



  onMounted(async () => { 
    title.value = await appWindow.title();
    title.value = title.value.replace(/\n/g, '<br/>')
    console.log(title.value)
  })

  async function close() {
      appWindow.close()
      await invoke("use_cron", {id: cronId.value});
      //appWindow.emit("ref_cron_list", true);
  }



  </script>
  
  <style scoped>
  .alarm-box {
    /* position: fixed; */
    width: 100%;
    height: 150px;
   

    background-color: rgba(0, 0, 0, 0.5);
    /* margin-top: -8px; */
    /* margin-left: -8px; */
    color: white;
  }
  
  .content {
    margin-left: 10px;
    overflow-y: hidden;
    overflow-x: auto; /* 启用垂直滚动条 */
  }
  
  button {
    margin-top: 10px;
    background-color: #007bff;
    border: none;
    color: white;
    padding: 5px 10px;
    border-radius: 5px;
    cursor: pointer;
  }
  
  button:hover {
    background-color: #0056b3;
  }

  

  </style>
  