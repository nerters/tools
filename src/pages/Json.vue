<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { writeTextFile, readTextFile, create, exists, BaseDirectory } from '@tauri-apps/plugin-fs';
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
const editorToMerge = ref(false);

onMounted(async () => { 
    //判断默认文件是否存在
    const exit = await exists(cacheFileName, { baseDir: BaseDirectory.AppData });
    if (!exit) {
    await create(cacheFileName, { baseDir: BaseDirectory.AppData })
    }
    let data = await readTextFile(cacheFileName, { baseDir: BaseDirectory.AppData })
    if (data) {
        // 转成json字符串并格式化
        temp.value = JSON.stringify(JSON.parse(data), null, '\t')
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
    console.log(event);
  if (event.ctrlKey && event.key === '\u0013') {
    event.preventDefault(); // 阻止默认行为，防止页面刷新
    console.log('Ctrl+S pressed');
    let state = editorView.value.state;
    if (state) {
        // 执行你需要的操作
        await writeTextFile(cacheFileName, state.doc.toString(), { baseDir: BaseDirectory.AppData }) 
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


//json对比
const jsonMerge = () => {
  editorToMerge.value = !editorToMerge.value;
}


//开放给父级
defineExpose({
    formatting,compress,jsonMerge
})
</script>

<template>
  <div class="main" v-show="!editorToMerge" ref="editorRef"  style="height: 100%;">

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
</style>
