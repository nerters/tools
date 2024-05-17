<script setup lang="ts">
import { ref, onMounted, onUnmounted, Ref } from 'vue';
//import { writeTextFile, readTextFile, create, exists, BaseDirectory } from '@tauri-apps/plugin-fs';
//import { basicSetup, EditorState, EditorView } from '@codemirror/basic-setup';
import {EditorState} from "@codemirror/state"
import {EditorView, keymap} from "@codemirror/view"
import {defaultKeymap} from "@codemirror/commands"
import { basicSetup } from 'codemirror';
import { json } from "@codemirror/lang-json";
import {MergeView} from "@codemirror/merge"


const emit = defineEmits(["titleType"])
emit("titleType", "json");
//json缓存文件
const cacheFileName = 'tools-json.json';
// 初始化
const editorRef = ref();
const mergeRef = ref();
const editorView = ref();
const mergeView = ref();
const temp = ref("");
const tableData:Ref<Array<{ [key: string]: any }>> = ref([{"te":"te"}]);
const editorToMerge = ref(false);

const props = defineProps({
  getCacheFile:{
    type:Function,
    default: () => null,
  },
  saveCacheFile:{
    type:Function,
    default: () => null,
  },
})

onMounted(async () => { 
    //判断默认文件是否存在
    let data = await props.getCacheFile(cacheFileName);
    console.log(data)
    if (data) {
        // 转成json字符串并格式化
        temp.value = JSON.stringify(JSON.parse(data), null, '\t')
        tableData.value[0] = JSON.parse(temp.value);
    }

    initEditor();
    initMergeView();
    window.addEventListener('keypress', handleKeyPress);
})

onUnmounted(() => {
  window.removeEventListener('keypress', handleKeyPress);
});

const initEditor = () => {
  if (typeof editorView.value !== "undefined") {
    editorView.value.destroy();
  }
  const startState:EditorState = EditorState.create({
    doc:temp.value,
    extensions: [keymap.of(defaultKeymap), basicSetup, json()],
  });
  if (editorRef.value) {

    editorView.value = new EditorView({
      state: startState,
      parent: editorRef.value,
    });
  }
};



const initMergeView = () => {
  let doc = `{}`

  mergeView.value = new MergeView({
    a: {
      doc,
      extensions: [keymap.of(defaultKeymap), basicSetup, json()]
    },
    b: {
      doc: doc,
      extensions: [
        keymap.of(defaultKeymap),
        basicSetup,
        json()
        // EditorView.editable.of(false),
        // EditorState.readOnly.of(false)
      ]
    },
    parent: mergeRef.value
  })

}




//待修改
const handleKeyPress = async (event: { ctrlKey: any; key: string; preventDefault: () => void; }) => {
  if (event.ctrlKey && event.key === '\u0013') {
    event.preventDefault(); // 阻止默认行为，防止页面刷新
    console.log('Ctrl+S pressed');
    let view:EditorView = editorView.value;
    if (view) {
      let state = view.state;
      if (state) {
          // 执行你需要的操作
          await props.saveCacheFile(cacheFileName, state.doc.toString());
          //await writeTextFile(cacheFileName, state.doc.toString(), { baseDir: BaseDirectory.AppData }) 
      }
    }


  }
};


//格式化
const formatting = () => {
  let view:EditorView = editorView.value;
    if (view) {
      let state = view.state;
      if (state.doc.toString()) {
        view.dispatch()
        view.setState(EditorState.create({
            doc: JSON.stringify(JSON.parse(state.doc.toString()), null, '\t'),
            extensions:[keymap.of(defaultKeymap), basicSetup]
        }))
      }
    }
}



const editMode = ref<{ [key: string]: boolean }>({});
    const editedValue = ref('');


const editCell = (rowIndex: number, key: string|number) => {
  let data = [JSON.parse(editorView.value.state)];
  editMode.value[rowIndex + '-' + key] = true;
  editedValue.value = data[rowIndex][key];
};

// const saveCell = (rowIndex: number, key: string|number) => {
//   let data = [JSON.parse(editorView.value.state)];
//   data[rowIndex][key] = editedValue.value;
//   editMode.value[rowIndex + '-' + key] = false;
// };



//压缩
const compress = () => {
    let state = editorView.value.state;
    if (state.doc.toString()) {
        editorView.value.setState(EditorState.create({
            doc: JSON.stringify(JSON.parse(state.doc.toString())),
            extensions:[keymap.of(defaultKeymap), basicSetup]
        }))
    }
}

const editorHeight = ref("height: 100%; width: 100%;")
const tableShow = ref(false);

//压缩
const edit = () => {
  console.log(tableShow.value)
  if (!tableShow.value) {
    let state = editorView.value.state;
    if (state.doc.toString()) {
      tableData.value[0] = JSON.parse(state.doc.toString());
    }

    editorHeight.value = "height: 100%; width: 50%;"
    tableShow.value = true;
  } else {
    editorHeight.value = "height: 100%; width: 100%;"
    tableShow.value = false
  }
}


//json对比
const jsonMerge = () => {
  editorToMerge.value = !editorToMerge.value;
}


//开放给父级
defineExpose({
    formatting,compress,jsonMerge,edit
})
</script>

<template>
  <div style="padding: 5px;  display: flex; justify-content: space-between; height: 100%; width: 100%;" v-show="!editorToMerge">
    <div class="main"  ref="editorRef"  :style="editorHeight">

    </div>

    <div v-show="tableShow" style="width: 50%; height: inherit; overflow-y:auto;" >
      <h2>table</h2>
      <table>
        <thead>
          <tr>
            <th key="0">key</th>
            <th key="0">value</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(row, rowIndex) in tableData" :key="rowIndex">
            <tr v-for="(value, key, columnIndex) in row" :key="columnIndex" @click="editCell(rowIndex, key)">
              <td v-if="!editMode[rowIndex + '-' + key]">{{ key }}</td>
              <td v-if="!editMode[rowIndex + '-' + key]">{{ value }}</td>
              <!-- <input v-else v-model="editedValue" @blur="saveCell(rowIndex, key)" /> -->
            </tr>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- <div v-show="tableShow" style="width: 50%;">
      <h2>table</h2>
      <table>
        <thead>
          <tr>
            <th v-for="(key, index) in Object.keys(tableData[0])" :key="index">{{ key }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(row, rowIndex) in tableData" :key="rowIndex">
            <td v-for="(value, key, columnIndex) in row" :key="columnIndex" @click="editCell(rowIndex, key)">
              <span v-if="!editMode[rowIndex + '-' + key]">{{ value }}</span>
              <input v-else v-model="editedValue" @blur="saveCell(rowIndex, key)" />
            </td>
          </tr>
        </tbody>
      </table>
    </div> -->


  </div>


  <div class="merge" v-show="editorToMerge" ref="mergeRef"  style="height: 100%;">

  </div>
</template>


<style >
.main {
  width: 100%;
  height: 100%;
}
/* required! */
.cm-editor {
  height: 100%;
}



table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  border: 1px solid #ccc;
  padding: 5px;
}

input {
  width: 100%;
  padding: 2px;
  box-sizing: border-box;
}


</style>
