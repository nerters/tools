<script setup lang="ts">
import { Ref, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrent } from '@tauri-apps/api/window';

const dataList:Ref<any> = ref([]);

// interface CardData {
//     content: String,
//     interval: number,
//     name: String
// }

const currentTime = ref(Date.now())

// 每秒更新一次时间戳
setInterval(async () => {
  currentTime.value = Date.now();
  await getCurrent().listen<any>("get_cron_info", (event) => {
    let temp = event.payload;
    for (let i = 0; i < dataList.value.length; i++) {
      if (dataList.value[i].id === temp.id) {
        dataList.value[i] = temp;
        break;
      } 
    }
  });
}, 1000);

// // 每秒更新一次时间戳
// setInterval(async () => {
//   dataList.value = await invoke("get_list_cron");
// }, 10000);

onMounted(async () => { 
    dataList.value = await invoke("get_list_cron");
    console.log(await invoke("get_list_cron"))
})


const addCardData = ref(false);

const content = ref("")
const interval = ref("")
const name = ref("")

function addCard() {
    addCardData.value = !addCardData.value;
}

async function addCardSubmit() {
    if (name.value && interval.value && content.value) {

        let id = await invoke("savn_cron", {name: name.value, content: content.value, interval:parseInt(interval.value), appointedTime:0});
        dataList.value.push({
            content: content.value,
            interval: interval.value,
            name: name.value,
            id: id
        })
        //await props.saveCacheFile(cacheFileName, JSON.stringify(dataList.value))
        addCardData.value = !addCardData.value;
        content.value = "";
        name.value = "";
        interval.value = ""
    }
}


async function floatingWindow(dataId: string) {
  await invoke("floating_window", {id: dataId});
}


async function removeItem(val:string) {
    const index = dataList.value.map((item: { name: any; }) => item.name).indexOf(val);
    let data = dataList.value[index]
    dataList.value.splice(index, 1);
    await invoke("del_cron", {id: data.id});
    //await props.saveCacheFile(cacheFileName, JSON.stringify(dataList.value))
}

</script>

<template>
    <el-button @click="addCard">添加</el-button>
    <div class="flex flex-wrap gap-4" style="padding: 5px;">  
        <el-card v-for="card in dataList" style="width: 480px" shadow="hover" >
            <div style="display: flex; justify-content: space-between;">
                <div>
                    {{ card.content }}
                </div>
                <div v-if="card.is_use === 0">
                  倒计时{{ card.interval - ((Math.floor(currentTime/1000)) - card.update_time) }} 秒
                </div>
                <div v-if="card.is_use === 1" style="color: red;">
                  触发
                </div>

                <el-button plain @click="floatingWindow(card.id)">固定</el-button>

                <span class="remove" @click="removeItem(card.name)">x</span>
            </div>
        </el-card>
    </div>



    <el-dialog v-model="addCardData" title="添加卡片" width="380">
        <div>
          <label>
            名称: <el-input
                    v-model="name"
                    style="width: 70%"
                    label="名称"
                    placeholder="Please input"
                />
          </label>
        </div>
        <div>
          <label>
            描述: <el-input
                    v-model="content"
                    style="width: 70%"
                    label="描述"
                    placeholder="Please input"
                />
           
          </label>
        </div>
        <div>
          <label>
            时间间隔: <el-input
                    v-model="interval"
                    style="width: 70%"
                    label="code"
                    placeholder="Please input"
                />
            
          </label>
        </div>



        <el-button type="primary" @click="addCardSubmit" plain>提交</el-button>
      </el-dialog>


</template>


<style>
.remove {
    
    right: 2px;
    top: 0;
    cursor: pointer;
}

</style>