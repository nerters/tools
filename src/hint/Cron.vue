<template>
    <div class="alarm-box" @click="close" >
      <div class="content">
        <p>{{ title }}</p>
        
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { onMounted, ref } from 'vue';
  import { getCurrent } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';
  const appWindow = getCurrent();

  const title = ref("")
  const cronId = ref("")
  onMounted(async () => { 
    let tit = await appWindow.title();
    let split = tit.split("_+||+_");
    title.value = split[0];
    cronId.value = split[1]
  })

  async function close() {
      console.log(3);
      appWindow.close()
      await invoke("use_cron", {id: cronId.value});
      appWindow.emit("ref_cron_list", true);
  }


  </script>
  
  <style scoped>
  .alarm-box {
    position: fixed;
    width: 100%;
    height: 100%;
   

    background-color: rgba(0, 0, 0, 0.5);
    margin-top: -8px;
    margin-left: -8px;
    color: white;
  }
  
  .content {
    text-align: center;
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
  