<template>
  <div style="padding: 5px;  display: flex; justify-content: space-between;">

      <div class="image-uploader" :class="{ 'hovered': isHovered }" @mouseover="handleMouseOver" @mouseleave="handleMouseLeave">
        <div class="image-container" v-if="imageUrl">
          <img :src="imageUrl" alt="Uploaded Image">
        </div>
        <div class="add-image" @click="openFileInput">
          <span v-if="!imageUrl">+</span>
        </div>

      </div>



    <div>
      <div>
        <el-select
          v-model="type"
          placeholder="请选择"
          
          >
          <el-option label="调整图片尺寸" value="size" />
          <el-option label="压缩图片" value="compress" />
          <el-option label="转黑白" value="grayscale" />
        </el-select>
        <div v-if="type == 'size'" style="margin-top: 10px;">
          <label>
            高：    <el-input  v-model="height"  style="width: 110px" maxlength="4" max="4000" size="large"  placeholder="Please Input"/>
          </label>

          <label>
            低：    <el-input  v-model="width"  style="width: 110px" maxlength="4" max="4000" size="large"  placeholder="Please Input"/>
          </label>
        </div>
        <div v-if="type == 'compress'" style="margin-top: 10px;">
          <label>
            精度：    <el-input  v-model="quality"  style="width: 248px" maxlength="4" max="4000" size="large"  placeholder="Please Input"/>
          </label>
        </div>

      </div>


      <el-button type="primary" style="width: 300px; margin-top: 10px" plain @click="change">转换</el-button>
    </div>




    <div class="image-uploader" :class="{ 'hovered': isHoveredRes }" @mouseover="handleMouseOverRes" @mouseleave="handleMouseLeaveRes">
        <div class="image-container" v-if="imageUrl">
          <img :src="resultImg" alt="Uploaded Image">
        </div>

      </div>

  </div>

</template>
  
  <script lang="ts" setup>
  import { ref } from 'vue'
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { open } from "@tauri-apps/plugin-dialog"
  import { invoke } from "@tauri-apps/api/core";
  
  const imageUrl = ref()
  const filePaht = ref()

  const type = ref("size");
  const height = ref("12")
  const width = ref("12")
  const quality = ref("100")

  defineProps({
      getCacheFile:{
        type:Function,
        default: () => null,
      },
      saveCacheFile:{
        type:Function,
        default: () => null,
      },
    })

  const resultImg = ref()

  const openFileInput = async () => {

      const file:string[] | null = await open({
        multiple: true,
        directory: false,
      });
      if (file) {
        filePaht.value = file[0]
        imageUrl.value = convertFileSrc(file[0]);
      }
  };

  const isHovered = ref(false);

  const handleMouseOver = () => {
    isHovered.value = true;
  };

  const handleMouseLeave = () => {
    isHovered.value = false;
  };

  const isHoveredRes = ref(false);

  const handleMouseOverRes = () => {
    isHoveredRes.value = true;
  };

  const handleMouseLeaveRes = () => {
    isHoveredRes.value = false;
  };


  async function change() {
    let rest:string = await invoke("compress_img", {filePath: filePaht.value, nwidth: parseInt(width.value),  nheight: parseInt(height.value),
    typeF: type.value, quality: parseInt(quality.value)});
    resultImg.value = "data:image/png;base64," + rest
    console.log(rest)
  }


  </script>
  
  <style scoped>


.image-uploader {
  position: relative;
  width: 200px;
  height: 200px;
  border: 2px dashed #ccc;
  border-radius: 8px;
  display: flex;
  justify-content: center;
  align-items: center;
  transition: border-color 0.3s;
}

.image-uploader.hovered {
  border-color: #265abb; /* 当鼠标移入时边框颜色变为红色 */
}

.image-container {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.image-container img {
  width: 100%;
  height: 100%;
  /* object-fit: cover; 等比例缩小图片 */
  object-fit: contain; /* 等比例缩小图片，保持原始宽高比 */
}

.add-image {
  position: absolute;
  width: 100%;
  height: 100%;
  cursor: pointer;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 36px;
}

.add-image span {
  color: #333;
}
</style>