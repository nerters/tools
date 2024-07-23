<template>
    <div style="display: flex;">
        <div style="padding: 5px; width: 45%;">
            
            <el-descriptions
                :column="1"
                size="large"
                direction="horizontal"
            >
                <el-descriptions-item label="时间戳">
                    <el-text v-text="currentTime" style="width: 240px" placeholder="Please input"></el-text>
                    <el-button text size="default" @click="copyText(currentTime)" >复制</el-button>
                </el-descriptions-item>


                <el-descriptions-item label="输入时间戳" >
                    <div>
                        <div>
                            <label>
                                时间戳：<el-input v-model="inputTime" style="width: 43%;"/>
                            </label>
                        </div>

                        <div>
                            <label>
                                格式化时间：<el-input  v-model="inputTimeStr" style="width: 43%;"/>
                            </label>
                        </div>
                        
                    </div>

                </el-descriptions-item>


            </el-descriptions>

        </div>


        <div style="padding: 5px; width: 45%;">

            <el-descriptions
                :column="1"
                size="large"
                direction="horizontal"
            >

                <el-descriptions-item label="北京时间">
                    {{ currentTimeStr }}
                    <el-select v-model="timeFormat" placeholder="Select" style="width: 40px">
                        <el-option
                        key="{y}-{m}-{d} {h}:{i}:{s}"
                        label="年-月-日 时:分:秒"
                        value="{y}-{m}-{d} {h}:{i}:{s}"
                        />

                        <el-option
                        key="{y}年{m}月{d}日 {h}:{i}:{s}"
                        label="年月日 时:分:秒"
                        value="{y}年{m}月{d}日 {h}:{i}:{s}"
                        />

                        <el-option
                        key="{y}/{m}/{d} {h}/{i}/{s}"
                        label="年/月/日 时/分/秒"
                        value="{y}/{m}/{d} {h}/{i}/{s}"
                        />
                    </el-select>
                    <el-button text size="default" @click="copyText(currentTimeStr)" >复制</el-button>
                </el-descriptions-item>

                <el-descriptions-item label="">
                    <div>
                            <label>
                                时间格式：<el-input  v-model="inputTimeFormat" style="width: 43%;"/>
                            </label>
                            <el-button text size="default" @click="inputFormat()" >转化</el-button>
                        </div>
                </el-descriptions-item>

            
            </el-descriptions>
        </div>

    </div>

</template>

<script lang="ts" setup>
import { ElMessage } from 'element-plus'

import { Jh_timeStampToTime } from '../utils/timeUtils'
import { ref } from 'vue';

const currentTime = ref(Date.now());
const timeFormat = ref("{y}-{m}-{d} {h}:{i}:{s}");
const currentTimeStr = ref(Jh_timeStampToTime(currentTime.value, timeFormat.value));


const inputTime = ref(Date.now());
const inputTimeFormat = ref("{y}-{m}-{d} {h}:{i}:{s}");
const inputTimeStr = ref(Jh_timeStampToTime(currentTime.value, timeFormat.value));

// 每秒更新一次时间戳
setInterval(() => {
  currentTime.value = Date.now();
  currentTimeStr.value = Jh_timeStampToTime(currentTime.value, timeFormat.value)
}, 1000);

function copyText(text: string|number) {
  navigator.clipboard.writeText(text.toString()).then(
    () => ElMessage('复制成功'),
    () => ElMessage('复制失败')
  );
}


function inputFormat() {
    console.log(inputTime.value)
    console.log(inputTimeFormat.value)
    inputTimeStr.value = Jh_timeStampToTime(parseInt(inputTime.value.toString()), inputTimeFormat.value)
}




</script>

<style>

.el-descriptions__body {
    opacity:1;
    
}

</style>