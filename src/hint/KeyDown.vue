<!-- Keycap.vue -->
<template>
  <div  class="keycap-container">
    <div class="keycap" v-for="item in keys" :style="{ backgroundColor: randomColor() }">
      
      <span class="keycap-text">
        {{ item.key }}
      </span>
    </div>
  </div>

  </template>


<style scoped>
.keycap-container {
  display: flex;             /* 使用 Flex 布局 */
  flex-wrap: wrap;           /* 自动换行 */
  gap: 10px;                 /* 键帽之间的间距 */
  justify-content: center;   /* 居中对齐 */

}



.keycap {
  width: 60px;              /* 键帽宽度 */
  height: 60px;             /* 键帽高度 */
  background-color: #ddd;   /* 键帽背景色 */
  border-radius: 8px;       /* 圆角 */
  box-shadow: 0px 4px 8px rgba(0, 0, 0, 0.2); /* 阴影 */
  display: flex;            /* Flex 布局，居中对齐 */
  align-items: center;
  justify-content: center;
  font-size: 20px;          /* 字体大小 */
  font-weight: bold;        /* 字体粗细 */
  color: #333;              /* 字体颜色 */
  transition: transform 0.2s; /* 点击效果的动画 */
  cursor: pointer;          /* 鼠标指针 */
}

/* 点击效果 */
.keycap:active {
  transform: translateY(2px); /* 点击后稍微下沉的效果 */
}


</style>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getCurrentWindow, LogicalPosition, LogicalSize } from '@tauri-apps/api/window';
import setPromiseInterval from 'set-promise-interval'
const appWindow = getCurrentWindow();

const title = ref("")
const width = ref(0)
const height = ref(0)
const factor = ref(0.0)
const keys = ref<Array<{
  key: string,
  time: number
}>>([]);



onMounted(async () => { 
  title.value = await appWindow.title();
  title.value = title.value.replace(/\n/g, '<br/>')
  await getCurrentWindow().listen<any>("key_down_msg", (event) => {
        let data = event.payload;
        let temp = data.key;
        console.log(data.width);
        console.log(temp);
        width.value = data.width / data.factor;
        console.log(width.value);
        height.value =  data.height / data.factor;
        factor.value = data.factor;
        console.log("----------" + ((width.value / 2) - keys.value.length * 35))
        appWindow.setSize(new LogicalSize((keys.value.length + 1) * 70 + 10  ,60));
        appWindow.setPosition(new LogicalPosition((width.value / 2) - (keys.value.length + 1) * 35 - 5, height.value - 60 * 2));
        if (temp.includes("Key")) {
          temp = temp.replace("Key", "");
        }
        if (temp.includes("Num")) {
          temp = temp.replace("Num", "");
        }
        if (temp == "Backspace") {
          temp = "del";
        }
        if (temp.includes("Control")) {
          temp = "crl";
        }
        if (temp.includes("Shift")) {
          temp = "Shift";
        }
        if (temp.includes("Arrow")) {
          temp = temp.replace("Arrow", "");
        }
        //释放Escape
        if (temp.includes("Escape")) {
          temp = 'esc';
        }
        

        if (keys.value.length == 0) {
          appWindow.show();
        }

        keys.value.push({
          key: temp,
          time: Date.now()
        })
    });
})

function randomColor() {
      const r = Math.floor(Math.random() * 256); // 随机红色值
      const g = Math.floor(Math.random() * 256); // 随机绿色值
      const b = Math.floor(Math.random() * 256); // 随机蓝色值
      const a = 0.7; // 设置透明度为 30%
      return `rgba(${r}, ${g}, ${b}, ${a})`; // 返回随机颜色
    }

// 每秒更新一次时间戳
setPromiseInterval(() => {
  if (keys.value.length == 0) {
    appWindow.setSize(new LogicalSize(0 ,60));
    appWindow.hide();
  } else {
    const now = Date.now();
    for (let i = 0; i < keys.value.length; i++) {
      console.log(keys.value[i].time)
      console.log(now)
      if (now - keys.value[i].time > 6000) {
        console.log('刪除' + keys.value.length)
        keys.value.splice(i, 1);
        if (keys.value.length == 0) {
          appWindow.setSize(new LogicalSize(0 ,60));
          appWindow.hide();
        } else {
          appWindow.setSize(new LogicalSize((keys.value.length) * 70 + 10 ,60));
          appWindow.setPosition(new LogicalPosition((width.value / 2) - (keys.value.length) * 35 - 5, height.value - 60 * 2));
        }
      }
      
    }
  }

}, keys.value.length == 0 ? 1000: 0);



</script>
  


