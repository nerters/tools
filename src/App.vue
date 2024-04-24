<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMounted} from "vue";

  onMounted(async () => { 
    await invoke("open_docs");
  })

</script>

<template>
  <div style="height: inherit;">
    <router-view></router-view>
  </div>

</template>