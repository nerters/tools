<template>
    <el-row :gutter="20" style=" margin: 5px;">
        <el-col :span="12"><div class="grid-content ep-bg-purple" />
            <div> 公钥：
                <el-input
                    v-model="publicKey"
                    style="width: 100%"
                    :rows="8"
                    type="textarea"
                    label="公钥"
                    placeholder="Please input"
                />
            </div>

            <div> 内容：
                <el-input
                    v-model="content"
                    style="width: 100%"
                    :rows="4"
                    type="textarea"
                    label="公钥"
                    placeholder="Please input"
                />
            </div>
            <el-button type="primary" style="width: 100%; margin-top: 5px;" plain @click="rsaEncrypt">加密</el-button>
            <div> 结果：
                <el-input
                    v-model="result"
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
                    v-model="privateKey"
                    style="width: 100%"
                    :rows="8"
                    type="textarea"
                    label="公钥"
                    placeholder="Please input"
                />
            </div>

            <div> 内容：
                <el-input
                    v-model="privateContent"
                    style="width: 100%"
                    :rows="4"
                    type="textarea"
                    label="公钥"
                    placeholder="Please input"
                />
            </div>
            <el-button type="primary" style="width: 100%; margin-top: 5px;" plain @click="rsaDecrypt">解密</el-button>
            <div> 结果：
                <el-input
                    v-model="privateResult"
                    style="width: 100%"
                    :rows="4"
                    type="textarea"
                    label="公钥"
                    placeholder="Please input"
                />
            </div>
        </el-col>
    </el-row>
</template>


<script lang="ts" setup>
import { Ref, onMounted, ref, watch } from 'vue'
import JSEncrypt from 'jsencrypt';
import { writeTextFile, readTextFile, create, exists, BaseDirectory } from '@tauri-apps/plugin-fs';


const publicKey = ref('')

const content = ref('')

const result = ref('')

const privateKey = ref('')

const privateContent = ref('')

const privateResult = ref('')

const temp:Ref<any> = ref();


//json缓存文件
const cacheFileName = 'rsaPageCache.json';

onMounted(async () => { 
    //判断默认文件是否存在
    const exit = await exists(cacheFileName, { baseDir: BaseDirectory.AppData });
    if (!exit) {
        await create(cacheFileName, { baseDir: BaseDirectory.AppData })
    }
    let data = await readTextFile(cacheFileName, { baseDir: BaseDirectory.AppData })
    if (data) {
        console.log(data)
        // 转成json字符串并格式化
        temp.value = JSON.parse(data)
        publicKey.value = temp.value.publicKey;
        privateKey.value = temp.value.privateKey;
    } else {
        temp.value = {publicKey: "", privateKey: ""}
    }
})

watch(privateKey, async () => {
    temp.value = {publicKey: temp.value.publicKey, privateKey: privateKey.value}
    if (privateKey.value) {
        await writeTextFile(cacheFileName, JSON.stringify(temp.value), { baseDir: BaseDirectory.AppData });
      }
})

watch(publicKey, async () => {
    temp.value = {publicKey: publicKey.value, privateKey: temp.value.privateKey}
    if (publicKey.value) {
        await writeTextFile(cacheFileName, JSON.stringify(temp.value), { baseDir: BaseDirectory.AppData });
      }
})


function rsaEncrypt() {
    const crypt = new JSEncrypt();
    crypt.setKey(publicKey.value);
    let rst = crypt.encrypt(content.value);
    if (rst) {
        result.value = rst;
    }
}


function rsaDecrypt() {
    const crypt = new JSEncrypt();
    crypt.setPrivateKey(privateKey.value);
    let rst = crypt.decrypt(privateContent.value);
    if (rst) {
        privateResult.value = rst;
    }
}


</script>