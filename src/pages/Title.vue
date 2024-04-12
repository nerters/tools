<script setup lang="ts">
    import { ref } from "vue";
    import icon from "../assets/32x32.png";
    import clo from "../assets/mdi_close.svg";
    import max from "../assets/mdi_maximize.svg";
    import min from "../assets/mdi_minimize.svg";
    import { Window } from '@tauri-apps/api/window';
    import { useRouter } from "vue-router";
    const appWindow = new Window('main');
    const router = useRouter();

    const emit = defineEmits(["addGrid", "editGrid"])

    function minimize() {
        console.log(1);
        appWindow.minimize()
    }
    function maximize() {
        console.log(2);
        appWindow.toggleMaximize()
    }
    function close() {
        console.log(3);
        appWindow.close()
    }

    const addGridData = ref(false);
    function addGrid() {
      setShow.value = false;
      addGridData.value = !addGridData.value;
    }

    function addGridSubmit() {
      console.log(gridName.value)
      emit("addGrid", {name: gridName.value, desc: gridDesc.value, code: gridCode.value});
      addGridData.value = !addGridData.value;
      gridName.value = "";
      gridDesc.value = "";
      gridCode.value = ""
    }

    const editGridData = ref(false);
    function editGrid() {
      setShow.value = false;
      editGridData.value = !editGridData.value;
      emit("editGrid", editGridData.value);
    }

    const gridName = ref('')
    const gridDesc = ref('')
    const gridCode = ref('')
    const setShow = ref(false);

    function homePage() {
      router.push("/");
    }

    const props = defineProps({
      type: String,
      jsonFormatting:{
        type:Function,
        default: () => null,
      },
      jsonCompress:{
        type:Function,
        default: () => null,
      },
      jsonMerge:{
        type:Function,
        default: () => null,
      }
    })

    function jsonFormatting() {
      setShow.value = false;
      props.jsonFormatting()
    }

    function jsonCompress() {
      setShow.value = false;
      props.jsonCompress()
    }

    function jsonMerge() {
      setShow.value = false;
      props.jsonMerge()
    }

    function test() {
      console.log(1)
    }

</script>

<template>

    <div data-tauri-drag-region class="titlebar">
      <div class="titleName" data-tauri-drag-region >
          <div style="width: 30px; height: 30px;" @click="homePage">
            <img  :src="icon"  alt="minimize"  />
          </div>

          <div style="display: flex;" >
            <el-popover placement="bottom" :width="100" trigger="click" :visible="setShow">
              <template #reference>
                <el-button  type="" key="plain" text @click="setShow = true" >设置</el-button>
                <!-- <el-button style="margin-right: 16px">Click to activate</el-button> -->
              </template>
              <template #default>
                <el-scrollbar max-height="400px" v-if="props.type === 'grid'">
                  <el-button  style="width: 100%;" text @click="addGrid">新建</el-button>
                  <el-button  style="width: 100%; margin-left: 0px;" text @click="editGrid" v-if="editGridData">固定</el-button>
                  <el-button  style="width: 100%; margin-left: 0px;" text @click="editGrid" v-if="!editGridData">编辑</el-button>
                </el-scrollbar>

                <el-scrollbar max-height="400px" v-if="props.type === 'json'">
                  <el-button  style="width: 100%;" text @click="jsonFormatting" >格式化</el-button>
                  <el-button  style="width: 100%; margin-left: 0px;" text @click="jsonCompress" >压缩</el-button>
                </el-scrollbar>
              </template>
            </el-popover>

            <el-button  type="" key="plain" text @click="jsonMerge" style="margin-left: -15px;" v-if="props.type === 'json'" >对比</el-button>
          </div>
      </div>

      <div>
        <div class="titlebar-button" id="titlebar-minimize" @click="minimize">
          <img
            :src="min"
            alt="minimize"
          />
        </div>
        <div class="titlebar-button" id="titlebar-maximize" @click="maximize">
          <img
            :src="max"
            alt="maximize"
          />
        </div>
        <div class="titlebar-button" id="titlebar-close" @click="close">
          <img :src="clo" alt="close" />
        </div>
      </div>





      <el-dialog v-model="addGridData" title="添加卡片" width="380">
        <div>
          <label>
            名称: <input v-model="gridName" style="width: auto" placeholder="Please input" /> 
          </label>
        </div>
        <div>
          <label>
            描述: <input v-model="gridDesc" style="width: auto" placeholder="Please input" /> 
          </label>
        </div>
        <div>
          <label>
            code: <input v-model="gridCode" style="width: auto" placeholder="Please input" /> 
          </label>
        </div>



        <el-button type="primary" @click="addGridSubmit" plain>提交</el-button>
      </el-dialog>





    </div>
</template>

<style scoped>
.titlebar {
  z-index: 1000;
  height: 30px;
  background: #ffffff;
  user-select: none;
  display: flex;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  justify-content: space-between;
  align-items: flex-end;
}
.titleName {
  display: flex;
  justify-content: center;
  align-items: center;

}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  user-select: none;
  -webkit-user-select: none;
}
.titlebar-button:hover {
  background: #a1a6a7;
}
</style>
