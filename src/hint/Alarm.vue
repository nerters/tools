<template>
  <div class="alarm-box" @click="close" >
    <div class="content">
      <p>{{ title }}</p>
      <p>已经触发{{ ((Math.floor(currentTime/1000)) - runTime) }} 秒</p>
    </div>
  </div>
  <div v-if="cronType == 'appointedTime'" style="position: fixed; top: -4px; right: 5px;">
      <el-button
          size="small"
        
          @click="stopCron()"
        >
          停止
        </el-button>
    </div>
</template>
  
  <script setup lang="ts">
  import { onMounted, ref } from 'vue';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';
  const appWindow = getCurrentWindow();

  const title = ref("")
  const cronId = ref("")
  const cronType = ref("interval")


  const currentTime = ref(Date.now())
  const runTime = ref(0);

  // 每秒更新一次时间戳
  setInterval(() => {
    currentTime.value = Date.now();
  }, 1000);


  onMounted(async () => { 
    let infoId = await appWindow.title();
    console.log("id:" + infoId)
    cronId.value = infoId;
    let cronData:any = await invoke("get_cron_info", {id: infoId});
    title.value = cronData.content;
    cronType.value = cronData.cron_type
    console.log("cronType:" + cronType.value)
    await appWindow.setTitle(cronData.name);
    runTime.value = cronData.update_time
  })

  async function close() {
      console.log(3);
      await invoke("use_cron", {id: cronId.value});
      appWindow.close()
      //appWindow.emit("ref_cron_list", true);
  }

  async function stopCron() {
    await invoke("stop_cron", {id: cronId.value});
    appWindow.close()
  }


  </script>
  
  <style scoped>
  .alarm-box {
    position: fixed;
    width: 100%;
    height: 100%;
   

    background-color: rgba(0, 0, 0, 0.5);
    /* margin-top: -8px; */
    /* margin-left: -8px; */
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
  