<template>
    
    <div class="alarm-box" data-tauri-drag-region >
      <div style="display: flex; align-items: center; justify-content: space-between; width: 100%; padding-left: 5px; padding-right: 5px; margin-top: -2px;" data-tauri-drag-region>
        <div class="content" style="display: flex; align-items: center; justify-content: center; 
                                    white-space: nowrap; /* 不换行 */ overflow: hidden; /* 隐藏超出部分 */ text-overflow: ellipsis; /* 用省略号表示隐藏的文本 */
                                    margin-right: 5px; " 
                                    data-tauri-drag-region>
          {{ title }}
        </div>
        <div class="titlebar-button" id="titlebar-close" @click="close" style="margin-right: 0px; width: 20px; text-align: center;">
            X
        </div>
      </div>
      <div class="content" style="display: flex; align-items: center; padding-top: -5px;" data-tauri-drag-region>
        <Countdown :deadline="runTime" countdownSize="2.5rem" :showLabels="false" :showDays="showDay" />
      </div>

        
    </div>
  </template>
  
  <script setup lang="ts">
  import { onMounted, ref } from 'vue';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';
  import {Countdown} from 'vue3-flip-countdown';

  const appWindow = getCurrentWindow();

  const title = ref("")
  const cronId = ref("")
  //const currentTime = ref(Date.now())
  const runTime = ref(timestampToTime(Date.now()));
  const showDay = ref(window.innerWidth > 300);

  

  // 每秒更新一次时间戳
  //setInterval(() => {
  //  currentTime.value = Date.now();
  //}, 1000);

  async function getData() {
    console.log("************")
    let infoId = await appWindow.title();
    cronId.value = infoId;
    let cronData:any = await invoke("get_cron_info", {id: infoId});
    title.value = cronData.content;
    await appWindow.setTitle(cronData.name);
    console.log(cronData.cron_type)
    if (cronData.cron_type === "interval") {
      runTime.value = timestampToTime((cronData.update_time + cronData.interval) * 1000)
    } else if (cronData.cron_type === "appointedTime") {
      runTime.value = timestampToTime((cronData.appointed_time) * 1000)
    }
  }


onMounted(async () => { 
    await getData();

    await getCurrentWindow().listen<any>("get_cron_info", (event) => {
        let temp = event.payload;
        if (cronId.value === temp.id) {
          if (temp.cron_type === "interval") {
            runTime.value = timestampToTime((temp.update_time + temp.interval) * 1000)
          } else if (temp.cron_type === "interval") {
            runTime.value = timestampToTime((temp.appointed_time) * 1000)
          }
          console.log(runTime.value);
        }

    });

})





async function close() {
    appWindow.close()
}


function timestampToTime (timestamp: number): string {
  if (!timestamp) {
    return ''
  }
  // 时间戳为10位需*1000，时间戳为13位不需乘1000
  const date = timestamp.toString.length === 10 ? new Date(timestamp * 1000) : new Date(timestamp)
  const Y = `${date.getFullYear()}-`
  const M = `${getzf(date.getMonth()+1)}-`
  const D = `${getzf(date.getDate())} `
  const h = `${getzf(date.getHours())}:`
  const m = `${getzf(date.getMinutes())}:`
  const s = getzf(date.getSeconds())
  return Y + M + D + h + m + s
}

//日期补0
function getzf(num: number): string | number {
  const numShow: string | number = num < 10 ? `0${num}` : num
  return numShow
}



  </script>
  
  <style scoped>
  .alarm-box {
    position: fixed;
    width: 100%;
    height: 100%;
   
    color: white;
    display: flex;
    /* justify-content: center; */
    justify-content: space-around;
    align-content: flex-start;
    flex-wrap: wrap;

    background: rgba(61, 58, 58, 0.5); /* 半透明白色背景 */
    backdrop-filter: blur(15px); /* 设置模糊程度 */
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2); /* 添加阴影效果 */


  }
  


    .titlebar-button {
    display: inline-flex;
    justify-content: flex-start;
    align-items: center;
    width: 15px;
    height: 30px;
    user-select: none;
    -webkit-user-select: none;
    color: #ffffff;
    /* background-color: rgba(0, 0, 0, 0.5); */
    }
    .titlebar-button:hover {
    background: #a1a6a7;
    }

    .content {
      user-select: none; /* 标准语法 */
    }
  </style>
  