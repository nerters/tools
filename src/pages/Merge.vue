<script setup lang="ts">
import { ref, onMounted } from 'vue';
//import { writeTextFile, readTextFile, create, exists, BaseDirectory } from '@tauri-apps/plugin-fs';
//import { basicSetup, EditorState, EditorView } from '@codemirror/basic-setup';
import {MergeView} from "@codemirror/merge"
import { basicSetup} from "codemirror"

// 初始化
const editorRef = ref();
const editorView = ref();

onMounted(async () => { 


    initEditor();


})

const initEditor = () => {
  let doc = `one
  two
  three
  four
  five`

  editorView.value = new MergeView({
    a: {
      doc,
      extensions: basicSetup
    },
    b: {
      doc: doc.replace(/t/g, "T") + "\nSix",
      extensions: [
        basicSetup,
        // EditorView.editable.of(false),
        // EditorState.readOnly.of(false)
      ]
    },
    parent: editorRef.value
  })

}



</script>

<template>
  <div class="main" ref="editorRef"  style="height: 100%;">

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
