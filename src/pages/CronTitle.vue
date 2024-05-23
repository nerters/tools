<script setup lang="ts">
import { Ref, onMounted, reactive, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrent } from '@tauri-apps/api/window';
import { Container, Draggable } from "vue3-smooth-dnd"

import { ask } from '@tauri-apps/plugin-dialog';
import clo from "../assets/mdi_close.svg";

const currentTime = ref(Date.now())

const addCardData = ref(false);

const isHovered = ref(false);

const handleMouseOver = () => {
    isHovered.value = true;
  };

  const handleMouseLeave = () => {
    isHovered.value = false;
  };

// const content = ref("")
// const interval = ref("")
// const name = ref("")

const sizeForm = reactive({
  cronId: "",
  cronName: '',
  cronContent: '',
  cronType: '',
  interval: "0",
  appointedTime:new Date(),
  category: "",
  pid: "",
})

// 每秒更新一次时间戳
setInterval(async () => {
  currentTime.value = Date.now();
}, 1000);

onMounted(async () => { 

    await getCurrent().listen<any>("get_cron_info", (event) => {
      let temp = event.payload;
      const columnIndex = scene.children.map((item: { id: any; name: any; }) => item.id).indexOf(temp.pid);
      if (columnIndex < 0) {
        for ( let i = 0; i < scene.children.length; i++) {
          let index = scene.children[i].children.map((item: { id: any; name: any; }) => item.id).indexOf(temp.pid);
          if (index >= 0) {
            let itemIndex = scene.children[i].children[index].children.map((item: { id: any; name: any; }) => item.id).indexOf(temp.id);
            scene.children[i].children[index].children[itemIndex] = temp;
          }
        }
      } else {
        const index = scene.children[columnIndex].children.map((item: { id: any; name: any; }) => item.id).indexOf(temp.id);
        if (index >= 0) {
          let card = scene.children[columnIndex].children[index];
          temp.children = card.children;
          scene.children[columnIndex].children[index] = temp;
        }
      }
    });


    scene.children = await invoke("get_tree_cron");

    console.log("------" + JSON.stringify(scene.children))
})






function addCard(category: string, pid: string) {
    console.log("category---" + category)
    console.log("pid---" + pid)
    addCardData.value = !addCardData.value;
    sizeForm.category = category;
    sizeForm.pid = pid;
}

async function addCardSubmit() {

    if (sizeForm.cronName && (sizeForm.interval || sizeForm.appointedTime) && sizeForm.cronContent) {

        let id: string = await invoke("savn_cron", {name: sizeForm.cronName, content: sizeForm.cronContent, cronType: sizeForm.cronType, interval:parseInt(sizeForm.interval), 
          appointedTime: Math.floor(sizeForm.appointedTime.getTime()/1000), category: sizeForm.category, pid: sizeForm.pid});
      
        scene.children = await invoke("get_tree_cron");
        //await props.saveCacheFile(cacheFileName, JSON.stringify(dataList.value))
        addCardData.value = !addCardData.value;
        addCardData.value = false;
        sizeForm.cronName = "";
        sizeForm.cronContent = "";
        sizeForm.cronType = "";
        sizeForm.interval = "0";
        sizeForm.appointedTime = new Date();

        sizeForm.category = "";
        sizeForm.pid = "";
    }
}


async function floatingWindow(dataId: string) {
  await invoke("floating_window", {id: dataId});
}


async function removeItem(dataId:string) {
  if (dataId === '-1' || dataId === '0') {
    alert("系统默认不能删除!")
  } else {
    const answer = await ask('是否删除?', {
      title: 'Tauri',
      kind: 'warning',
      okLabel: '是',
      cancelLabel: '否',
    });

    if (answer) {
      await invoke("del_cron", {id: dataId});
      scene.children = await invoke("get_tree_cron");
    }
  }
    //await props.saveCacheFile(cacheFileName, JSON.stringify(dataList.value))
}




//-------

const lorem = `Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. 
Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. 
Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.`


const cardColors = [
  'azure',
  'beige',
  'bisque',
  'blanchedalmond',
  'burlywood',
  'cornsilk',
  'gainsboro',
  'ghostwhite',
  'ivory',
  'khaki'
]

const pickColor = () => {
  const rand = Math.floor(Math.random() * 10)
  return cardColors[rand]
}

const applyDrag = async (id: string,  arr: any, dragResult: any) => {
  const { removedIndex, addedIndex, payload } = dragResult
  if (removedIndex === null && addedIndex === null) return arr

  const result = [...arr]
  let itemToAdd = payload

  if (removedIndex !== null) {
    itemToAdd = result.splice(removedIndex, 1)[0]
  }

  if (addedIndex !== null) {
    itemToAdd.pid = id;
    result.splice(addedIndex, 0, itemToAdd)
  }
  for (let i = 0; i < result.length; i++) {
    console.log("更新")
    let temp = result[i];
    await invoke("update_cron", {id: temp.id, name: temp.name, content: temp.content, interval: temp.interval, appointedTime: temp.appointed_time, 
                                  pid: temp.pid, sort: i, category: temp.category, isUse: temp.is_use, cronType: temp.cron_type});
  }
  return result
}


    let scene = reactive({
      type: 'container',
      props: {
        orientation: 'vertical'
      },
      children:[
        {
          id:"Lorem1",
          pid:"1",
          name:"Lorem",
          props: {
            orientation: 'horizontal',
            className: 'card-container'
          },
          children:[
            {
              type: 'draggable',
              id: `Lorem`,
              pid:"1",
              name:"Lorem",
              props: {
                className: 'card',
                style: {backgroundColor: pickColor()},
                orientation: 'vertical',
              },
              children:[
                {
                  type: 'draggable',
                  id: `Lorem`,
                  name:"Lorem",
                  pid:"1",
                  content:"",
                  is_use:0,
                  cron_type:"",
                  interval:0,
                  update_time:0,
                  appointed_time:0,
                  props: {
                    className: 'card',
                    style: {backgroundColor: pickColor()},
                    orientation: 'vertical',
                  },
                  data: lorem.slice(0, Math.floor(Math.random() * 150) + 30)
                }
              ],
            }
          ]
        }
      ]
    })



    async function onColumnDrop (dropResult: any) {
      const scene1 = Object.assign({}, scene)
      scene1.children = await applyDrag("-2", scene1.children, dropResult)
      scene.children = scene1.children
    }

    async function onCardDrop (columnId: any, dropResult: { removedIndex: any; addedIndex: any; payload?: any; }) {
      console.log("-----" + JSON.stringify(dropResult))
      if (dropResult.removedIndex !== null || dropResult.addedIndex !== null) {
        const scene1 = Object.assign({}, scene)
        const column = scene1.children.filter((p: { id: any; }) => p.id === columnId)[0]
        const columnIndex = scene1.children.indexOf(column)
        const newColumn = Object.assign({}, column)

        if((dropResult.removedIndex == null && dropResult.addedIndex >= 0)){
          // your action / api call
          dropResult.payload.loading = true
          // simulate api call
          setTimeout(function(){ dropResult.payload.loading = false }, (Math.random() * 5000) + 1000); 
        }

        newColumn.children = await applyDrag(scene1.children[columnIndex].id, newColumn.children, dropResult)
        scene1.children.splice(columnIndex, 1, newColumn)

        scene.children = scene1.children
      }
    }
    
    function getCardPayload (columnId: any) {
      return (index: number) => {
        console.log("index******" + index)
        return scene.children.filter(p => p.id === columnId)[0].children[index]
      }
    }

    async function onItemDrop (id1: any, id2: any, dropResult: any) {

      if (dropResult.removedIndex !== null || dropResult.addedIndex !== null) {
        const scene1 = Object.assign({}, scene)
        const columnList = scene1.children.filter((p: { id: any; }) => p.id === id1)[0]
        const column = columnList.children.filter((p: { id: any; }) => p.id === id2)[0]
        const columnIndex = columnList.children.indexOf(column)
        const newColumn = Object.assign({}, column)

        if((dropResult.removedIndex == null && dropResult.addedIndex >= 0)){
          // your action / api call
          
          dropResult.payload.loading = true
          // simulate api call
          setTimeout(function(){ dropResult.payload.loading = false }, (Math.random() * 5000) + 1000); 
        }
        if (dropResult.removedIndex !== null && dropResult.removedIndex - newColumn.children.length >= 0) {
          dropResult.removedIndex = dropResult.removedIndex - newColumn.children.length;
        }
        if (dropResult.addedIndex !== null && dropResult.addedIndex - newColumn.children.length >= 0) {
          dropResult.addedIndex = dropResult.addedIndex - newColumn.children.length;
        }
        
        newColumn.children = await applyDrag(columnList.children[columnIndex].id, newColumn.children, dropResult)
        columnList.children.splice(columnIndex, 1, newColumn)

        scene.children = scene1.children
      }
    }

    function getItemPayload (id1: any, id2: any) {
      return (index: number) => {
        let data = scene.children.filter(p => p.id === id1)[0];
        let list:Array<any> = data.children.filter(p => p.id === id2)[0].children;
        return list[index]
      }
    }

</script>

<template>


    <el-dialog v-model="addCardData" title="添加卡片" width="380">
      <el-form
          ref="form"
          style="max-width: 600px"
          :model="sizeForm"
          label-width="auto"
          label-position="left"
          size="small"
        >
          <el-form-item label="名称">
            <el-input v-model="sizeForm.cronName" />
          </el-form-item>
          <el-form-item label="描述">
            <el-input v-model="sizeForm.cronContent" />
          </el-form-item>

          <el-form-item label="类型">
            <el-select
              v-model="sizeForm.cronType"
              placeholder="请选择"
            >
              <el-option label="时间间隔" value="interval" />
              <el-option label="指定时间点" value="appointedTime" />
            </el-select>
          </el-form-item>

          <div v-if="sizeForm.cronType === 'interval'">
            <el-form-item label="时间间隔">
              <el-input v-model="sizeForm.interval" />
            </el-form-item>
          </div>
          <div v-if="sizeForm.cronType === 'appointedTime'">
            <el-form-item label="指定时间点">
              <el-date-picker
                v-model="sizeForm.appointedTime"
                type="datetime"
                placeholder="Select date and time"
              />
             
            </el-form-item>
          </div>
          
          <el-form-item>
            <el-button type="primary" @click="addCardSubmit">创建</el-button>
            <el-button @click="addCardData = false">取消</el-button>
          </el-form-item>
        </el-form>
    </el-dialog>


      <div class="card-scene">
        <Container orientation="vertical" @drop="onColumnDrop($event)">
          <Draggable v-for="column in scene.children" :key="column.id">
            <div class="card_list">
              <div class="card_titlebar">
                <div class="card_titleName" data-tauri-drag-region >
                  {{ column.name }}
                </div>
                <div>
                  <div class="card_titlebar_button">
                    <el-button  type="" key="plain" @click="addCard('type', column.id)" text >添加</el-button>
                  </div>
                  <div class="card_titlebar_button" id="titlebar-close" @click="removeItem(column.id)">
                    <img :src="clo" alt="close" />
                  </div>
                </div>
              </div>

              <Container  class="h-full flex overflow-x-auto gap-8 p-8"
              style="width: 100%; height: 100%; display:list-item"
                tag="div"
                orientation="horizontal"
                group-name="col-items"
                :shouldAcceptDrop="(e:any, payload:any) =>  (e.groupName === 'col-items' && !payload.loading)"
                :get-child-payload="getCardPayload(column.id)"
                :drop-placeholder="{ className: 
                  `bg-primary bg-opacity-20  
                  border-dotted border-2 
                  border-primary rounded-lg mx-4 my-2`, 
                animationDuration: '200', 
                showOnTop: true }"
                drag-class="bg-primary dark:bg-primary 
                  border-2 border-primary-hover text-white 
                  transition duration-100 ease-in z-50
                  transform rotate-6 scale-110"
                drop-class="transition duration-100 
                  ease-in z-50 transform 
                  -rotate-2 scale-90"
                @drop="(e:any) => onCardDrop(column.id, e)"
                >
                <Draggable v-for="item in column.children" :key="item.id" class="bg-gray-200 dark:bg-gray-700 rounded-lg h-full w-96 flex-shrink-0 shadow-xl">
                  <div class="card_card" >
                    
                    <div class="card_titlebar">
                      <div class="card_titleName" data-tauri-drag-region >
                        {{ item.name }}
                      </div>

                      <div>
                        <div class="card_titlebar_button">
                          <el-button  type="" key="plain" @click="addCard('cron', item.id)" text >添加</el-button>
                        </div>
                        <div class="card_titlebar_button" id="titlebar-close" @click="removeItem(item.id)">
                          <img :src="clo" alt="close" />
                        </div>
                      </div>
                    </div>

                    <Container class="flex-grow overflow-y-auto overflow-x-hidden" 
                        tag="div"
                        orientation="vertical"
                        group-name="col-items1"
                        style="  height: 215px;"
                        :shouldAcceptDrop="(e:any, payload:any) =>  (e.groupName === 'col-items1' && !payload.loading)"
                        :get-child-payload="getItemPayload(column.id, item.id)"
                        :drop-placeholder="{ className: 
                          `bg-primary bg-opacity-20  
                          border-dotted border-2 
                          border-primary rounded-lg mx-4 my-2`, 
                        animationDuration: '200', 
                        showOnTop: true }"
                        drag-class="bg-primary dark:bg-primary 
                          border-2 border-primary-hover text-white 
                          transition duration-100 ease-in z-50
                          transform rotate-6 scale-110"
                        drop-class="transition duration-100 
                          ease-in z-50 transform 
                          -rotate-2 scale-90"
                        @drop="(e: any) => onItemDrop(column.id, item.id, e)">
                        
                        <Draggable v-for="i in item.children" :key="i.id">
                            <div class="cursor-move my-2 mx-4 rounded-lg shadow-md bg-gray-100 dark:bg-gray-800 hover:border-2 border-primary"  >
                              <div class="p-4 space-y-2" >
                                <div style="width: 300px; font-size: 13px; height: 32px; box-shadow: var(--el-box-shadow-dark); padding-top: 5px;" shadow="hover" >
                                    <div style="display: flex; justify-content: space-between; align-items: center; margin-left: 5px; margin-right: 5px;">
                                        <div style="width: 100px;">
                                            {{ i.content }}
                                         
                                        </div>
                                        <div v-if="i.is_use === 0 && i.cron_type ==='interval'" style="width: 100px;">
                                          倒计时{{ i.interval - ((Math.floor(currentTime/1000)) - i.update_time) }} 秒
                                          
                                        </div>
                                        <div v-if="i.is_use === 0 && i.cron_type ==='appointedTime'" style="width: 100px;">
                                          倒计时{{ i.appointed_time - ((Math.floor(currentTime/1000))) }} 秒
                                        </div>
                                        <div v-if="i.is_use === 1" style="color: red; width: 100px;">
                                          触发
                                        </div>
                                        <div >
                                          <el-button plain @click="floatingWindow(i.id)">固定</el-button>
                                        </div>                
                                        <span @click="removeItem(i.id)">x</span>
                                    </div>
                                </div>


                              </div>
                            </div>
                          </Draggable>



                    </Container>
                  </div>

                </Draggable>
              </Container>
            </div>
          </Draggable>
        </Container>

      </div>

      <div style="width: 100%; display: flex; justify-content: center;">
        <div class="addCronType" :class="{ 'hovered': isHovered }" @mouseover="handleMouseOver" @mouseleave="handleMouseLeave" @click="addCard('type', '-2')">
          <div class="add-image" >
            <span >+</span>
          </div>

        </div>
      </div>


</template>


<style>
.card_titlebar {
  z-index: 1000;
  height: 30px;
  background: #ffffff;
  user-select: none;
  display: flex;
  width: 100%;
  margin-left: 0;
  margin-right: 0;
  justify-content: space-between;
  align-items: flex-end;
}

.card_titleName {
  padding-left: 5px;
  display: flex;
  justify-content: center;
  align-items: center;

}

.card_titlebar_button {
  padding-right: 5px;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  user-select: none;
  -webkit-user-select: none;
}
.card_titlebar_button:hover {
  background: #a1a6a7;
}



.remove {
    right: 2px;
    top: 0;
    cursor: pointer;
}


.card_list{
  box-shadow: var(--el-box-shadow-dark);
  height: 300px;
  margin: 5px;
  overflow-y: hidden;
  overflow-x: auto; /* 启用垂直滚动条 */
}

.card_card {
  /* display: flex;
  flex-wrap: wrap;
  align-content: flex-start; */
  box-shadow: var(--el-box-shadow-dark);
  width: 300px;
  margin: 5px;
  height: 245px;

  overflow-y: auto; /* 启用垂直滚动条 */
  padding: 5px; /* 可选，添加内边距 */ 
}

/* 滚动条整体部分 */
.card_card::-webkit-scrollbar {
  width: 5px; /* 滚动条宽度 */
}

/* 滚动条轨道部分 */
.card_card::-webkit-scrollbar-track {
  background: #f1f1f1; /* 轨道背景色 */
  border-radius: 6px; /* 可选，圆角 */
}

/* 滚动条滑块部分 */
.card_card::-webkit-scrollbar-thumb {
  background: #888; /* 滑块颜色 */
  border-radius: 6px; /* 可选，圆角 */
}

/* 滑块悬停效果 */
.card_card::-webkit-scrollbar-thumb:hover {
  background: #555; /* 悬停时滑块颜色 */
}





/* 滚动条整体部分 */
.card_list::-webkit-scrollbar {
  height: 5px; /* 滚动条宽度 */
}

/* 滚动条轨道部分 */
.card_list::-webkit-scrollbar-track {
  background: #f1f1f1; /* 轨道背景色 */
  border-radius: 6px; /* 可选，圆角 */
}

/* 滚动条滑块部分 */
.card_list::-webkit-scrollbar-thumb {
  background: #888; /* 滑块颜色 */
  border-radius: 6px; /* 可选，圆角 */
}

/* 滑块悬停效果 */
.card_list::-webkit-scrollbar-thumb:hover {
  background: #555; /* 悬停时滑块颜色 */
}


.addCronType {
  position: relative;
  width: 100%;
  height: 30px;
  border: 2px dashed #ccc;
  border-radius: 8px;
  display: flex;
  justify-content: center;
  align-items: center;
  transition: border-color 0.3s;
}

.addCronType.hovered {
  border-color: #265abb; /* 当鼠标移入时边框颜色变为红色 */
}

</style>