<script setup lang="ts">
import '../styles.css'
import { ref } from "vue";
import Title from "../pages/Title.vue";
import { writeTextFile, readTextFile, create, exists, BaseDirectory } from '@tauri-apps/plugin-fs';

const titleType = ref("");

const routerViewRef = ref()

const jsonFormatting = () => {
  console.log(1)
  routerViewRef.value.formatting()
}

const jsonCompress = () => {
  console.log(2)
  routerViewRef.value.compress()
}

const jsonExit = () => {
  console.log(2)
  routerViewRef.value.edit()
}

const jsonMerge = () => {
  console.log(3)
  routerViewRef.value.jsonMerge()
}


const getCacheFile = async (cacheFileName: string | URL) : Promise<String> => {
  console.log("获取文件")
      //判断默认文件是否存在
    const exit = await exists(cacheFileName, { baseDir: BaseDirectory.AppData });
    if (!exit) {
    await create(cacheFileName, { baseDir: BaseDirectory.AppData })
    }
    let data = await readTextFile(cacheFileName, { baseDir: BaseDirectory.AppData })
    if (data) {
        // 转成json字符串并格式化
        return data;
    }
    return "";
}


const saveCacheFile = async (cacheFileName: string | URL, data: string) => {
  await writeTextFile(cacheFileName, data, { baseDir: BaseDirectory.AppData }) 
}



</script>
<template>
  <Title :type="titleType" :jsonFormatting="jsonFormatting" :jsonCompress="jsonCompress" :jsonMerge="jsonMerge" :jsonExit="jsonExit" />

  <div style="padding-top: 30px; height: inherit;">

    <router-view v-slot:="{Component,route}" @title-type="(type: string) => {titleType = type}" :getCacheFile="getCacheFile" :saveCacheFile="saveCacheFile">
		
      <component ref="routerViewRef" :is="Component" :key="route.path"/>
      
    </router-view>

    <!-- <router-view @title-type="(type: string) => {titleType = type}"></router-view> -->
  </div>

</template>

<style>

</style>
