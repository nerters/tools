<script setup lang="ts">
    import { reactive, ref } from "vue";
    import icon from "../assets/32x32.png";
    import clo from "../assets/mdi_close.svg";
    import max from "../assets/mdi_maximize.svg";
    import min from "../assets/mdi_minimize.svg";
    import { getCurrent } from '@tauri-apps/api/window';
    import { useRouter } from "vue-router";
    const appWindow = getCurrent();
    const router = useRouter();

    const emit = defineEmits(["addGrid", "editGrid"])


    const sizeForm = reactive({
      gridName: '',
      gridDesc: '',
      gridCode: '',
      gridType: "",
      gridUri:"",

      date1:"",
      date2: '',
      delivery: false,
      type: [],
      resource: '',
      desc: '',
    })

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
      if (sizeForm.gridName && sizeForm.gridCode) {
        emit("addGrid", {name: sizeForm.gridName, desc: sizeForm.gridDesc, code: sizeForm.gridCode});
        addGridData.value = false;
        sizeForm.gridName = "";
        sizeForm.gridDesc = "";
        sizeForm.gridCode = ""
      } else {
        alert("请填写数据");
      }

    }

    const editGridData = ref(false);
    function editGrid() {
      setShow.value = false;
      editGridData.value = !editGridData.value;
      emit("editGrid", editGridData.value);
    }


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
      jsonExit:{
        type:Function,
        default: () => null,
      },
      jsonMerge:{
        type:Function,
        default: () => null,
      },
      readCatalog:{
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

    function jsonExit() {
      setShow.value = false;
      props.jsonExit()
    }


    function openCatalog() {
      props.readCatalog()
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
                <div v-if="props.type === 'grid' || props.type === 'json'">
                  <el-button  type="" key="plain"  text @click="setShow = true" >设置</el-button>
                </div>

                <div v-if="props.type === 'read'">
                  <el-button  type="" key="plain" text @click="openCatalog" >目录</el-button>
                </div>

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
                  <el-button  style="width: 100%; margin-left: 0px;" text @click="jsonExit" >编辑</el-button>
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

        <el-form
          ref="form"
          style="max-width: 600px"
          :model="sizeForm"
          label-width="auto"
          label-position="left"
          size="small"
        >
          <el-form-item label="名称">
            <el-input v-model="sizeForm.gridName" />
          </el-form-item>
          <el-form-item label="描述">
            <el-input v-model="sizeForm.gridDesc" />
          </el-form-item>
          <el-form-item label="标识">
            <el-input v-model="sizeForm.gridCode" />
          </el-form-item>

          <el-form-item label="类型">
            <el-select
              v-model="sizeForm.gridType"
              placeholder="请选择"
            >
              <el-option label="打开网站" value="openWeb" />
              <el-option label="功能页面" value="funPage" />
              <el-option label="占位" value="placeholder" />
            </el-select>
          </el-form-item>
          <div v-if="sizeForm.gridType === 'openWeb'">
            <el-form-item label="网址">
              <el-input v-model="sizeForm.gridUri" />
            </el-form-item>
          </div>
          <div v-if="sizeForm.gridType === 'funPage'">
            <el-form-item label="网址123">
              <el-input v-model="sizeForm.gridUri" />
            </el-form-item>
          </div>

          <div v-if="sizeForm.gridType === 'funPage1'">

            <el-form-item label="Activity time">
              <el-col :span="11">
                <el-date-picker
                  v-model="sizeForm.date1"
                  type="date"
                  aria-label="Pick a date"
                  placeholder="Pick a date"
                  style="width: 100%"
                />
              </el-col>
              <el-col class="text-center" :span="1" style="margin: 0 0.5rem">-</el-col>
              <el-col :span="11">
                <el-time-picker
                  v-model="sizeForm.date2"
                  aria-label="Pick a time"
                  placeholder="Pick a time"
                  style="width: 100%"
                />
              </el-col>
            </el-form-item>
            <el-form-item label="Activity type">
              <el-checkbox-group v-model="sizeForm.type">
                <el-checkbox-button value="Online activities" name="type">
                  Online activities
                </el-checkbox-button>
                <el-checkbox-button value="Promotion activities" name="type">
                  Promotion activities
                </el-checkbox-button>
              </el-checkbox-group>
            </el-form-item>
            <el-form-item label="Resources">
              <el-radio-group v-model="sizeForm.resource">
                <el-radio border value="Sponsor">Sponsor</el-radio>
                <el-radio border value="Venue">Venue</el-radio>
              </el-radio-group>
            </el-form-item>

          </div>
          
          <el-form-item>
            <el-button type="primary" @click="addGridSubmit">创建</el-button>
            <el-button @click="addGridData = false">取消</el-button>
          </el-form-item>
        </el-form>
        
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
