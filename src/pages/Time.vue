<template>
    <div>
        <label>
            时间戳：<el-input v-model="currentTime" style="width: 240px" placeholder="Please input" />
        </label>
    </div>

</template>

<script lang="ts" setup>
import { ref } from 'vue'
const currentTime = ref(Date.now())

    // 每秒更新一次时间戳
    setInterval(() => {
      currentTime.value = Date.now();
    }, 1000);

</script>