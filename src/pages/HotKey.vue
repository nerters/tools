<script setup lang="ts">

import { onMounted, reactive, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const tableData = ref([]);
const updateData = ref(false);
const addData = ref(false);

const sizeForm = reactive({
      id: '',
      key: '',
      key1: '',
      key2: '',
      path: '',
      url: '',
      shell: '',
      desc: '',
      overopen: '0',
    })

onMounted(async () => { 
    tableData.value = await invoke("get_hot_key_list");
    console.log( tableData.value)
})

async function update() {
    await invoke("update_hot_key",{id: sizeForm.id, path: sizeForm.path, key: sizeForm.key1 + "+" + sizeForm.key2, desc: sizeForm.desc, overopen: parseInt(sizeForm.overopen.toString()), url: sizeForm.url, shell: sizeForm.shell});
    sizeForm.id = '';
    sizeForm.key = '';
    sizeForm.path = '';
    sizeForm.key1 = '';
    sizeForm.key2 = '';
    sizeForm.desc = '';
    sizeForm.url = '';
    sizeForm.shell = '';
    sizeForm.overopen = '0';
    //刷新
    tableData.value = await invoke("get_hot_key_list");
    updateData.value = false;
}

async function add() {
    console.log("==")
    await invoke("add_hot_key",{path: sizeForm.path, key: sizeForm.key1 + "+" + sizeForm.key2, desc: sizeForm.desc, overopen: parseInt(sizeForm.overopen.toString()), url: sizeForm.url, shell: sizeForm.shell});
    sizeForm.id = '';
    sizeForm.key = '';
    sizeForm.path = '';
    sizeForm.key1 = '';
    sizeForm.key2 = '';
    sizeForm.desc = '';
    sizeForm.url = '';
    sizeForm.shell = '';
    sizeForm.overopen = '0';
    //刷新
    tableData.value = await invoke("get_hot_key_list");
    addData.value = false;
}

async function del(id: String) {
    console.log("--")
    await invoke("delete_hot_key",{id: id,});
    //刷新
    tableData.value = await invoke("get_hot_key_list");
}



</script>
<template>

  <el-table :data="tableData" stripe style="width: 100%" @row-dblclick="(row: any, _column: any, _event: Event) => {
    updateData = true;
    sizeForm.id = row.id;
    sizeForm.key = row.key;
    sizeForm.path = row.path;
    sizeForm.key1 = row.key1;
    sizeForm.key2 = row.key2;
    sizeForm.desc = row.desc;
    sizeForm.url = row.url;
    sizeForm.shell = row.shell;
    sizeForm.overopen = row.overopen.toString();
    }">

    <el-table-column prop="key" label="快捷键" width="180" />
    <el-table-column prop="path" label="功能" width="180" />
    <el-table-column prop="desc" label="描述" />
    <el-table-column prop="overopen" label="功能页多开" >
      <template #default="scope">
        <span v-if="scope.row.overopen == 1">开启</span>
        <span v-if="scope.row.overopen == 0">关闭</span>
      </template>
    </el-table-column>

    <el-table-column align="right">
      <template #header>
        <el-button
          size="small"
        
          @click="addData = true"
        >
          新增
        </el-button>
      </template>
      <template #default="scope">
        <el-button
          size="small"
          type="danger"
          @click="del(scope.row.id)"
        >
          删除
        </el-button>
      </template>
    </el-table-column>
    
  </el-table>


  <el-dialog v-model="updateData" title="更新卡片" width="380">

    <el-form
    ref="form"
    style="max-width: 600px"
    :model="sizeForm"
    label-width="auto"
    label-position="left"
    size="small"
    >
    <el-form-item label="快捷键">
  
        <el-row :gutter="50">
          <el-col :span="12">
            <el-select
              v-model="sizeForm.key1"
              placeholder="请选择"
              style="width: 125px;"
              >
              <el-option label="shift+ctrl" value="shift+control" />
              <el-option label="ctrl+z" value="control+KeyZ" />
            </el-select>
          </el-col>
          <el-col :span="12">
            <el-select
              v-model="sizeForm.key2"
              placeholder="请选择"
              style="width: 125px;"
              >
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="1" value="Digit1" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="2" value="Digit2" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="3" value="Digit3" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="4" value="Digit4" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="5" value="Digit5" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="6" value="Digit6" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="7" value="Digit7" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="8" value="Digit8" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="9" value="Digit9" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="A" value="KeyA" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="B" value="KeyB" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="C" value="KeyC" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="D" value="KeyD" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="E" value="KeyE" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="F" value="KeyF" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="G" value="KeyG" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="H" value="KeyH" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="I" value="KeyI" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="G" value="KeyG" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="K" value="KeyK" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="L" value="KeyL" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="M" value="KeyM" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="N" value="KeyN" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="O" value="KeyO" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="P" value="KeyP" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="Q" value="KeyQ" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="R" value="KeyR" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="S" value="KeyS" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="T" value="KeyT" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="U" value="KeyU" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="V" value="KeyV" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="W" value="KeyW" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="X" value="KeyX" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="Y" value="KeyY" />
              <el-option v-if="sizeForm.key1 == 'shift+control'" label="Z" value="KeyZ" />

            </el-select>
          </el-col>
        </el-row>

    </el-form-item>
    <el-form-item label="类型">
        <el-select
          v-model="sizeForm.path"
          placeholder="请选择"
          >
          <el-option label="打开功能页" value="openFun" />
          <el-option label="打开网址" value="webPage" />
          <el-option label="执行命令" value="doShell" />
          <el-option label="打开应用" value="openProgram" />
          <el-option label="唤醒ollama" value="ollama" />
        </el-select>
    </el-form-item>

    <el-form-item v-if="sizeForm.path == 'openFun'" label="功能">
        <el-select
            v-model="sizeForm.url"
            placeholder="请选择"
            >
            <el-option label="JSON" value="Json" />
            <el-option label="RSA密码" value="RsaPage" />
            <el-option label="定时器" value="CronTitle" />
        </el-select>
    </el-form-item>

    <el-form-item v-if="sizeForm.path == 'webPage'" label="网址">
        <el-input v-model="sizeForm.url" />
    </el-form-item>
    <el-form-item v-if="sizeForm.path == 'doShell'" label="命令">
        <el-input v-model="sizeForm.shell" type="textarea" style="width: 240px"/>
    </el-form-item>
    <el-form-item v-if="sizeForm.path == 'ollama'" label="模型">
            <el-input v-model="sizeForm.shell" type="textarea" />
        </el-form-item>
    <el-form-item label="描述">
        <el-input v-model="sizeForm.desc" />
    </el-form-item>

    <el-form-item label="功能页多开">
        <el-select
        v-model="sizeForm.overopen"
        placeholder="请选择"
        >
        <el-option label="开启" value="1" />
        <el-option label="关闭" value="0" />
        </el-select>
    </el-form-item>
   

    
    <el-form-item>
        <el-button type="primary" @click="update()">更新</el-button>
        <el-button @click="updateData = false">取消</el-button>
    </el-form-item>
    </el-form>

  </el-dialog>


    <el-dialog v-model="addData" title="新增卡片" width="380">

        <el-form
        ref="form"
        style="max-width: 600px"
        :model="sizeForm"
        label-width="auto"
        label-position="left"
        size="small"
        >
        <el-form-item label="快捷键">
          <el-row :gutter="50">
            <el-col :span="12">
              <el-select
                v-model="sizeForm.key1"
                placeholder="请选择"
                style="width: 125px;"
                >
                <el-option label="shift+ctrl" value="shift+control" />
                <el-option label="shift+ctrl+alt" value="shift+control+alt" />
              </el-select>
            </el-col>
            <el-col :span="12">
              <el-select
                v-model="sizeForm.key2"
                placeholder="请选择"
                style="width: 125px;"
                >
                <el-option label="1" value="Digit1" />
              <el-option label="2" value="Digit2" />
              <el-option label="3" value="Digit3" />
              <el-option label="4" value="Digit4" />
              <el-option label="5" value="Digit5" />
              <el-option label="6" value="Digit6" />
              <el-option label="7" value="Digit7" />
              <el-option label="8" value="Digit8" />
              <el-option label="9" value="Digit9" />
              <el-option label="A" value="KeyA" />
              <el-option label="B" value="KeyB" />
              <el-option label="C" value="KeyC" />
              <el-option label="D" value="KeyD" />
              <el-option label="E" value="KeyE" />
              <el-option label="F" value="KeyF" />
              <el-option label="G" value="KeyG" />
              <el-option label="H" value="KeyH" />
              <el-option label="I" value="KeyI" />
              <el-option label="G" value="KeyG" />
              <el-option label="K" value="KeyK" />
              <el-option label="L" value="KeyL" />
              <el-option label="M" value="KeyM" />
              <el-option label="N" value="KeyN" />
              <el-option label="O" value="KeyO" />
              <el-option label="P" value="KeyP" />
              <el-option label="Q" value="KeyQ" />
              <el-option label="R" value="KeyR" />
              <el-option label="S" value="KeyS" />
              <el-option label="T" value="KeyT" />
              <el-option label="U" value="KeyU" />
              <el-option label="V" value="KeyV" />
              <el-option label="W" value="KeyW" />
              <el-option label="X" value="KeyX" />
              <el-option label="Y" value="KeyY" />
              <el-option label="Z" value="KeyZ" />
              </el-select>
            </el-col>
          </el-row>
        </el-form-item>
        <el-form-item label="类型">
            <el-select
              v-model="sizeForm.path"
              placeholder="请选择"
              >
              <el-option label="打开功能页" value="openFun" />
              <el-option label="打开网址" value="webPage" />
              <el-option label="执行命令" value="doShell" />
              <el-option label="打开应用" value="openProgram" />
              <el-option label="唤醒ollama" value="ollama" />
            </el-select>
        </el-form-item>

        <el-form-item v-if="sizeForm.path == 'openFun'" label="功能">
            <el-select
                v-model="sizeForm.url"
                placeholder="请选择"
                >
                <el-option label="JSON" value="Json" />
                <el-option label="RSA密码" value="RsaPage" />
                <el-option label="定时器" value="CronTitle" />
            </el-select>
        </el-form-item>

        <el-form-item v-if="sizeForm.path == 'webPage'" label="网址">
            <el-input v-model="sizeForm.url" />
        </el-form-item>
        <el-form-item v-if="sizeForm.path == 'doShell'" label="命令">
            <el-input v-model="sizeForm.shell" type="textarea" style="width: 240px"/>
        </el-form-item>
        <el-form-item v-if="sizeForm.path == 'ollama'" label="模型">
            <el-input v-model="sizeForm.shell" type="textarea" />
        </el-form-item>
        <el-form-item label="描述">
            <el-input v-model="sizeForm.desc" />
        </el-form-item>

        <el-form-item label="功能页多开">
            <el-select
            v-model="sizeForm.overopen"
            placeholder="请选择"
            >
            <el-option label="开启" value="1" />
            <el-option label="关闭" value="0" />
            </el-select>
        </el-form-item>



        <el-form-item>
            <el-button type="primary" @click="add()">新增</el-button>
            <el-button @click="addData = false">取消</el-button>
        </el-form-item>
        </el-form>

    </el-dialog>


</template>

<style>

</style>
