<template>

  <el-tabs
    v-model="editableTabsValue"
    type="card"
    editable
    class="demo-tabs"
    @edit="handleTabsEdit"
  >
    <el-tab-pane
      v-for="item in editableTabs"
      :key="item.name"
      :label="item.title"
      :name="item.name"
    >
        <el-row :gutter="20" style=" margin: 5px;">
            <el-col :span="12"><div class="grid-content ep-bg-purple" />
                <div> 公钥：
                    <el-input
                        v-model="item.publicKey"
                        style="width: 100%"
                        :rows="8"
                        type="textarea"
                        label="公钥"
                        placeholder="Please input"
                    />
                </div>

                <div> 内容：
                    <el-input
                        v-model="item.content"
                        style="width: 100%"
                        :rows="4"
                        type="textarea"
                        label="公钥"
                        placeholder="Please input"
                    />
                </div>
                <el-button type="primary" style="width: 100%; margin-top: 5px;" plain @click="rsaEncrypt(item)">加密</el-button>
                <div> 结果：
                    <el-input
                        v-model="item.result"
                        style="width: 100%"
                        :rows="4"
                        type="textarea"
                        label="公钥"
                        placeholder="Please input"
                    />
                </div>

            </el-col>
            <el-col :span="12"><div class="grid-content ep-bg-purple" />
                <div> 私钥：
                    <el-input
                        v-model="item.privateKey"
                        style="width: 100%"
                        :rows="8"
                        type="textarea"
                        label="公钥"
                        placeholder="Please input"
                    />
                </div>

                <div> 内容：
                    <el-input
                        v-model="item.privateContent"
                        style="width: 100%"
                        :rows="4"
                        type="textarea"
                        label="公钥"
                        placeholder="Please input"
                    />
                </div>
                <el-button type="primary" style="width: 100%; margin-top: 5px;" plain @click="rsaDecrypt(item)">解密</el-button>
                <div> 结果：
                    <el-input
                        v-model="item.privateResult"
                        style="width: 100%"
                        :rows="4"
                        type="textarea"
                        label="公钥"
                        placeholder="Please input"
                    />
                </div>
            </el-col>
        </el-row>
    </el-tab-pane>
  </el-tabs>



</template>


<script lang="ts" setup>
import { Ref, onMounted, ref, watch } from 'vue'
import JSEncrypt from 'jsencrypt';
import { ElMessage, ElMessageBox, TabPaneName } from 'element-plus';


const editableTabsValue = ref('1')
const editableTabs:Ref<Array<any>> = ref([])
let tabLen = editableTabs.value.length
const handleTabsEdit = (
  targetName: TabPaneName | undefined,
  action: 'remove' | 'add'
) => {
  if (action === 'add') {
    

    ElMessageBox.prompt('请输入标签名', '添加', {
    confirmButtonText: 'OK',
    cancelButtonText: 'Cancel',
    
    inputErrorMessage: 'Invalid Email',
  })
    .then(({ value }) => {
        const newTabName = `${++tabLen}`
        editableTabs.value.push({
        title: value,
        name: newTabName,
        content: value,
        })
        editableTabsValue.value = newTabName
    })
    .catch(() => {
      ElMessage({
        type: 'info',
        message: '取消输入',
      })
    })
  } else if (action === 'remove') {
    const tabs = editableTabs.value
    if (editableTabsValue.value === targetName) {
      tabs.forEach((tab, index) => {
        if (tab.name === targetName) {
          const nextTab = tabs[index + 1] || tabs[index - 1]
          if (nextTab) {
            editableTabsValue.value = nextTab.name
          }
        }
      })
    }
    editableTabs.value = tabs.filter((tab) => tab.name !== targetName)
  }
}



const prop = defineProps({
      getCacheFile:{
        type:Function,
        default: () => null,
      },
      saveCacheFile:{
        type:Function,
        default: () => null,
      },
    })


//json缓存文件
const cacheFileName = 'rsaPageCache.json';

onMounted(async () => {
    let data = await prop.getCacheFile(cacheFileName)
  
    if (data) {
        // 转成json字符串并格式化
        editableTabs.value = JSON.parse(data);
        if (editableTabs.value.length > 0) {
            editableTabsValue.value = editableTabs.value[0].name;
            tabLen = editableTabs.value.length
        }
        console.log(editableTabs.value.length);
    } else {
        editableTabs.value = [];
    }
})

watch(() => editableTabs, async () => {
    console.log(11111)
    await prop.saveCacheFile(cacheFileName, JSON.stringify(editableTabs.value));
},{deep: true,})




function rsaEncrypt(item: any) {
    const crypt = new JSEncrypt();
    crypt.setKey(item.publicKey);
    let rst = crypt.encrypt(item.content);
    if (rst) {
        item.result = rst;
    }
}


function rsaDecrypt(item: any) {
    const crypt = new JSEncrypt();
    crypt.setPrivateKey(item.privateKey);
    let rst = crypt.decrypt(item.privateContent);
    if (rst) {
        item.privateResult = rst;
    }
}


</script>

<style>

</style>