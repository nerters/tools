<script setup lang="ts">
import { ref, onMounted, watch, Ref, computed } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();

import { writeTextFile, readTextFile, create, exists, BaseDirectory } from '@tauri-apps/plugin-fs';
import Title from "../pages/Title.vue";

const cacheFileName = 'tools.json';

const layout:Ref<Array<any>>= ref([])

const temp:Array<any> = [{"x":0,"y":0,"w":2,"h":4,"i":"JSON","type":"sys","code":"JSON","static":false,"moved":false},{"x":5,"y":0,"w":2,"h":2,"type":"sys","code":"测试1","i":"测试1","moved":false},{"x":6,"y":2,"w":2,"h":2,"type":"sys","code":"1","i":"1","moved":false},{"x":6,"y":4,"w":2,"h":2,"type":"sys","code":"2","i":"2","moved":false},{"x":7,"y":0,"w":2,"h":2,"type":"sys","code":"3","i":"3","moved":false},{"x":2,"y":0,"w":2,"h":2,"type":"sys","code":"Merge","i":"Merge","moved":false},{"x":2,"y":2,"w":2,"h":2,"type":"sys","code":"Img","desc":"图片工具","i":"图片","moved":false}]


onMounted(async () => { 

    //判断默认文件是否存在
    const exit = await exists(cacheFileName, { baseDir: BaseDirectory.AppData });
    if (!exit) {
      await create(cacheFileName, { baseDir: BaseDirectory.AppData })
    }
    let data = await readTextFile(cacheFileName, { baseDir: BaseDirectory.AppData })
    if (data) {
      layout.value = JSON.parse(data)
      if (layout.value.length == 0) {
        layout.value = temp
      }
    } else {
      layout.value = temp
    }


    index.value = layout.value.length;

    const userComputed = computed(() => {
        return JSON.parse(JSON.stringify(layout.value))
    })

    watch(userComputed, async (newValue, oldValue) => {
      if (JSON.stringify(newValue) != JSON.stringify(oldValue)) {
        await writeTextFile(cacheFileName, JSON.stringify(layout.value), { baseDir: BaseDirectory.AppData });
      }
    },
    { deep: true })

})


const editGridData = ref(false);
const addGridData = ref<any>();
const colNum = ref(12);
const index = ref(0);


function removeItem(val:string) {
    const index = layout.value.map(item => item.i).indexOf(val);
    layout.value.splice(index, 1);
}

watch(addGridData, () => {
  console.log("addGridData")
  if (addGridData.value) {
    addItem(addGridData.value.name, addGridData.value.desc, addGridData.value.code)
  }
})

function addItem(name: String, desc: String, code: String) {
    // Add a new item. It must have a unique key!
    layout.value.push({
        x: (layout.value.length * 2) % (colNum.value || 12),
        y: layout.value.length + (colNum.value || 12), // puts it at the bottom
        w: 2,
        h: 2,
        type:"sys",
        code:code?code:name,
        desc:desc,
        i: name,
    });
    // Increment the counter to ensure key is always unique.
    index.value += 1;
}

function openFun(type: String) {
  if (!editGridData.value) {
    console.log(type);
    router.push({path:"/main/"+type});  //跳转到对应菜单选项的页面
  }
}

</script>

<template>
  <Title @add-grid="(grid: any) => {addGridData = grid}"  @edit-grid="(grid: boolean) => {editGridData = grid}" :type="'grid'"/>

  <div id="grid_lay" style="margin-top: 30px; overflow-y:auto; height: inherit;">
    <grid-layout
      :layout.sync="layout"
      :col-num="colNum"
      :row-height="30"
      :is-draggable="editGridData"
      :is-resizable="editGridData"
      :is-mirrored="false"
      :vertical-compact="true"
      :margin="[10, 10]"
      :use-css-transforms="true"
    >

        <grid-item v-for="item in layout"
            :x="item.x"
            :y="item.y"
            :w="item.w"
            :h="item.h"
            :i="item.i"
            :key="item.i"
            @click="openFun(item.code)"
            >
            <div v-if="item.i.toString() === 'JSON'" style="text-align: center; " >{{ item.i }}</div>
            <div v-if="item.i.toString() != 'JSON'" style="text-align: center; " >{{ item.i }}</div>
            <span class="remove" v-if="editGridData" @click="removeItem(item.i)">x</span>
            <div style="text-align: center; ">{{ item.desc }}</div>
        </grid-item>
    </grid-layout>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}


.remove {
    position: absolute;
    right: 2px;
    top: 0;
    cursor: pointer;
}


.vue-grid-item:not(.vue-grid-placeholder) {
    /* background: #f3f0f0; */
    background: #e9e6e6;
    /* border: 1px solid black; */
}
</style>
