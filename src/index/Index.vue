<script setup lang="ts">
import '../styles.css'
import { ref, onMounted, watch, Ref, computed, reactive } from "vue";
import { useRouter } from "vue-router";
import { invoke } from '@tauri-apps/api/core';

import { writeTextFile, readTextFile, create, exists, BaseDirectory } from '@tauri-apps/plugin-fs';
import Title from "../pages/Title.vue";

import { open } from '@tauri-apps/plugin-shell';

const router = useRouter();

const cacheFileName = 'tools.json';

const layout:Ref<Array<any>>= ref([])

const temp:Array<any> = [{"x":2,"y":0,"w":2,"h":2,"type":"funPage","is_sys":0,"code":"Merge","desc":"对比两段文字","i":"Merge","id":"128285195614912513","template_id":"","run_code":"","uri":"","update_time":0,"moved":false},{"x":10,"y":0,"w":2,"h":2,"type":"funPage","is_sys":0,"code":"Img","desc":"图片工具","i":"图片","id":"128285195615076353","template_id":"","run_code":"","uri":"","update_time":0,"moved":false},{"x":0,"y":2,"w":2,"h":2,"type":"funPage","is_sys":0,"code":"RsaPage","desc":"RSA加密","i":"RSA","id":"128285195615240193","template_id":"","run_code":"","uri":"","update_time":0,"moved":false},{"x":2,"y":2,"w":2,"h":2,"type":"funPage","is_sys":0,"code":"Time","desc":"时间相关","i":"时间","id":"128285195615469569","template_id":"","run_code":"","uri":"","update_time":0,"moved":false},{"x":4,"y":0,"w":6,"h":3,"type":"funPage","is_sys":0,"code":"CronTitle","desc":"定时器","i":"定时提醒","id":"128285195615698945","template_id":"","run_code":"","uri":"","update_time":0,"moved":false},{"x":0,"y":0,"w":2,"h":2,"type":"funPage","is_sys":0,"code":"JSON","desc":"json格式","i":"JSON","id":"128285229827686401","template_id":"","run_code":"","uri":"","update_time":0,"moved":false},{"x":0,"y":4,"w":12,"h":1,"type":"placeholder","is_sys":0,"code":"fengexian","desc":"","i":"分割线","id":"128285476236427265","template_id":"","run_code":"","uri":"","update_time":0,"moved":false}]


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
    
    layout.value = await merge_data();
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
const editInfo = ref(false);


async function removeItem(val:string) {
    const index = layout.value.map(item => item.code).indexOf(val);
    let data = layout.value[index];
    if (data.id) {
      await invoke("delete_grid_by_id", {id: data.id});
    }
    layout.value.splice(index, 1);
}

function exitItem(id: string) {
  const index = layout.value.map(item => item.id).indexOf(id);
  let data = layout.value[index];
  gridForm.id = data.id;
  gridForm.gridName = data.i;
  gridForm.gridDesc = data.desc;
  gridForm.gridCode = data.code;
  gridForm.gridType = data.type;
  gridForm.gridUri = data.uri;
  gridForm.gridX = data.x;
  gridForm.gridY = data.y; // puts it at the bottom
  gridForm.gridW = data.w;
  gridForm.gridH = data.h;
  editInfo.value = true;
}


async function updateGrid() {
  await invoke("update_grid", {id: gridForm.id, name: gridForm.gridName, describe: gridForm.gridDesc, uri: gridForm.gridUri, code: gridForm.gridCode,
     classify: gridForm.gridType, x: gridForm.gridX, y: gridForm.gridY, w: gridForm.gridW, h: gridForm.gridH});
  editInfo.value = false;
  let data:any = await invoke("get_grid_by_id", {id: gridForm.id});



  const index = layout.value.map(item => item.id).indexOf(gridForm.id);
  layout.value.splice(index, 1);
  layout.value.push({
        x: data.x,
        y: data.y, // puts it at the bottom
        w: data.w,
        h: data.h,
        type: data.classify,
        is_sys: data.is_sys,
        code: data.code,
        desc: data.describe,
        i: data.name,
        id: data.id,
        template_id: data.template_id,
        run_code: data.run_code,
        uri: data.uri,
        update_time:data.update_time,
  })


  gridForm.id = "";
  gridForm.gridName = "";
  gridForm.gridDesc = "";
  gridForm.gridCode = "";
  gridForm.gridType = "";
  gridForm.gridUri = "";
  gridForm.gridX = 0;
  gridForm.gridY = 0; // puts it at the bottom
  gridForm.gridW = 2;
  gridForm.gridH = 2;


}

const gridForm = reactive({
      id: "",
      gridName: '',
      gridDesc: '',
      gridCode: '',
      gridType: "",
      gridUri:"",
      gridX: 0,
      gridY: 0,
      gridW: 2,
      gridH: 2,
    })



watch(addGridData, async () => {
  if (addGridData.value) {
    await addItem(addGridData.value.name, addGridData.value.desc, addGridData.value.code, addGridData.value.uri, addGridData.value.classify)
  }
})


watch(editGridData, async () => {
  if (!editGridData.value) {
    layout.value = await merge_data();
  }
})

async function addItem(name: String, desc: String, code: String, uri: String, classify: String) {
    // Add a new item. It must have a unique key!
    let x = (layout.value.length * 2) % (colNum.value || 12);
    let y = layout.value.length + (colNum.value || 12);
    let dataId = await invoke("add_grid", {name: name, describe: desc, uri: uri, code: code?code:name, classify: classify, x: x, y: y, w: 2, h: 2 });
    layout.value.push({
        x: x,
        y: y, // puts it at the bottom
        w: 2,
        h: 2,
        type: "funPage",
        code: code?code:name,
        desc: desc,
        i: name,
        id: dataId ? dataId.toString():"",
    });
    // Increment the counter to ensure key is always unique.
    index.value += 1;
}

function openFun(id: String) {
  if (!editGridData.value) {
    const index = layout.value.map(item => item.id).indexOf(id);
    let data = layout.value[index];
    if (data.type === "openWeb") {
      open(data.uri)
    } else if (data.type === "funPage") {
      let path = "/main/"+data.code;
      router.push({path: path});  //跳转到对应菜单选项的页面
    }

  }
}

//router.push({path:  "/main/CronTitle"}); 


async function merge_data():Promise<any[]> {
  let tempList = [];
    for (let i = 0; i < layout.value.length; i++) {
      let data = layout.value[i];
      tempList.push({
        x: data.x,
        y: data.y, // puts it at the bottom
        w: data.w,
        h: data.h,
        classify: data.type,
        is_sys: data.is_sys?data.is_sys:0,
        code: data.code,
        describe: data.desc?data.desc:"",
        name: data.i,
        id: data.id ? data.id.toString():"",
        template_id: data.template_id?data.template_id:"",
        run_code: data.run_code?data.run_code:"",
        uri:data.uri?data.uri:"",
        
        
        create_time:0,
        creator_lid:"",
        creator_name:"",
        updater_lid:"",
        updater_name:"",
        up_ver:0,
        sort:0,
        tenant_id:0,
        deleted:0,
        update_time:0,
      })
    }

    //console.log("tempList", JSON.stringify(tempList));

    let gridList: Array<any> = await invoke("grid_merge_data", {dataList: tempList});
    let list = [];
    for (let i = 0; i < gridList.length; i++) {
      let data = gridList[i];
      list.push({
        x: data.x,
        y: data.y, // puts it at the bottom
        w: data.w,
        h: data.h,
        type: data.classify,
        is_sys: data.is_sys,
        code: data.code,
        desc: data.describe,
        i: data.name,
        id: data.id,
        template_id: data.template_id,
        run_code: data.run_code,
        uri: data.uri,
        update_time:data.update_time,
      })
    }
    return list;
}


</script>

<template>
  <Title @add-grid="(grid: any) => {addGridData = grid}"  @edit-grid="(grid: boolean) => {editGridData = grid}" :type="'grid'"/>

  <div id="grid_lay" style="margin-top: 30px; height: auto; margin: 0 -5 0; padding: 0;">
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
            @click="openFun(item.id)"
            >

            <div style="position: absolute; font-size: 8px; margin-top: -5px; margin-left: 3px ;" v-if="item.type === 'openWeb'"> w </div>
            <div style="position: absolute; font-size: 8px; margin-top: -5px; margin-left: 3px ;" v-if="item.type === 'funPage'"> f </div>
            <div style="position: absolute; font-size: 8px; margin-top: -5px; margin-left: 3px ;" v-if="item.type === 'placeholder'"> p </div>

            <div>
              <div v-if="item.i.toString() === 'JSON'" style="text-align: center; " >
                {{ item.i }}
              </div>
              <div v-if="item.i.toString() != 'JSON'" style="text-align: center; " >{{ item.i }}</div>
              <span class="update" v-if="editGridData" @click="exitItem(item.id)">0</span>
              <span class="remove" v-if="editGridData" @click="removeItem(item.code)">x</span>
            </div>

            <div style="text-align: center; ">{{ item.desc }}</div>
        </grid-item>
    </grid-layout>
  </div>



  <el-dialog v-model="editInfo" title="添加卡片" width="380">
    <el-form
      ref="form"
      style="max-width: 600px"
      :model="gridForm"
      label-width="auto"
      label-position="left"
      size="small"
    >
      <el-form-item label="名称">
        <el-input v-model="gridForm.gridName" />
      </el-form-item>
      <el-form-item label="描述">
        <el-input v-model="gridForm.gridDesc" />
      </el-form-item>
      <el-form-item label="标识">
        <el-input v-model="gridForm.gridCode" />
      </el-form-item>

      <el-form-item label="类型">
        <el-select
          v-model="gridForm.gridType"
          placeholder="请选择"
        >
          <el-option label="打开网站" value="openWeb" />
          <el-option label="功能页面" value="funPage" />
          <el-option label="占位" value="placeholder" />
        </el-select>
      </el-form-item>
      <div v-if="gridForm.gridType === 'openWeb'">
        <el-form-item label="网址">
          <el-input v-model="gridForm.gridUri" />
        </el-form-item>
      </div>
      <!-- <div v-if="gridForm.gridType === 'funPage'">
        <el-form-item label="网址123">
          <el-input v-model="gridForm.gridUri" />
        </el-form-item>
      </div> -->
      
      <el-form-item>
        <el-button type="primary" @click="updateGrid()">更新</el-button>
        <el-button @click="editInfo = false">取消</el-button>
      </el-form-item>
    </el-form>
  </el-dialog>




</template>

<style>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}


.update {
    position: absolute;
    right: 10px;
    top: 0;
    cursor: pointer;
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

html,
body,
#app {
    height: auto;
    margin: 0;
    padding: 0;
    scrollbar-width: none
}

</style>
