<script setup lang="ts">
import { ref } from "vue";
import Title from "../pages/Title.vue";

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

const jsonMerge = () => {
  console.log(3)
  routerViewRef.value.jsonMerge()
}

</script>
<template>
  <Title :type="titleType" :jsonFormatting="jsonFormatting" :jsonCompress="jsonCompress" :jsonMerge="jsonMerge" />

  <div style="padding-top: 30px; height: inherit;">

    <router-view v-slot:="{Component,route}" @title-type="(type: string) => {titleType = type}">
		
      <component ref="routerViewRef" :is="Component" :key="route.path"/>
      
    </router-view>

    <!-- <router-view @title-type="(type: string) => {titleType = type}"></router-view> -->
  </div>

</template>

<style>

</style>
