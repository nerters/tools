<template>
  <div class="keycap-container">
    <div class="keycap" v-for="(item, index) in keys" :key="item.key + index" :style="{ backgroundColor: randomColor() }"  ref="keycapRefs">
      <span class="keycap-text" ref="textRefs">
        
        <i v-if="item.key != getIconClass(item.key)" :class="getIconClass(item.key)" style="font-size: 24px; color: #333;"></i>
        <i v-else>{{strToKey(item.key)}}</i>
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, nextTick } from 'vue';
import { getCurrentWindow, LogicalPosition, LogicalSize } from '@tauri-apps/api/window';
import setPromiseInterval from 'set-promise-interval';

interface Key {
  key: string;
  time: number;
}

const keys = ref<Key[]>([]); // 键数组
const keyMap = new Map<string, number>(); // 用于防止多次按下同一键
const title = ref('');
const width = ref(0);
const height = ref(0);
const factor = ref(0.0);
const appWindow = getCurrentWindow();

const keycapRefs = ref<HTMLElement[]>([]);
const textRefs = ref<HTMLElement[]>([]);

// 获取当前窗口的标题并初始化
onMounted(async () => {
  title.value = await appWindow.title();
  title.value = title.value.replace(/\n/g, '<br/>');

  await appWindow.listen<any>('key_down_msg', (event) => {
    let data = event.payload;
    let temp = data.key;
    let val = keyMap.get(temp);
    if (val && (Date.now() - val < 300)) {
      keyMap.set(temp, Date.now());
      return;
    } else {
      width.value = data.width / data.factor;
      height.value = data.height / data.factor;
      factor.value = data.factor;

      if (keys.value.length === 0) {
        appWindow.show();
      }
      keyMap.set(temp, Date.now());
      keys.value.push({
        key: temp,
        time: Date.now(),
      });
      setWindowSize();
    }
  });

  watch(keys, async () => {
    await nextTick();
    adjustFontSizes();
  });
});

// 获取键对应的 Font Awesome 图标
const getIconClass = (key: string): string => {
  console.log(key)
  switch (key) {
    // 字母键
    case 'A': return 'fas fa-a'; 
    case 'B': return 'fas fa-b'; 
    case 'C': return 'fas fa-c'; 
    case 'D': return 'fas fa-d'; 
    case 'E': return 'fas fa-e'; 
    case 'F': return 'fas fa-f'; 
    case 'G': return 'fas fa-g'; 
    case 'H': return 'fas fa-h'; 
    case 'I': return 'fas fa-i'; 
    case 'J': return 'fas fa-j'; 
    case 'K': return 'fas fa-k'; 
    case 'L': return 'fas fa-l'; 
    case 'M': return 'fas fa-m'; 
    case 'N': return 'fas fa-n'; 
    case 'O': return 'fas fa-o'; 
    case 'P': return 'fas fa-p'; 
    case 'Q': return 'fas fa-q'; 
    case 'R': return 'fas fa-r'; 
    case 'S': return 'fas fa-s'; 
    case 'T': return 'fas fa-t'; 
    case 'U': return 'fas fa-u'; 
    case 'V': return 'fas fa-v'; 
    case 'W': return 'fas fa-w'; 
    case 'X': return 'fas fa-x'; 
    case 'Y': return 'fas fa-y'; 
    case 'Z': return 'fas fa-z'; 

    // 数字键
    case 'Num1': return 'fas fa-1'; 
    case 'Num2': return 'fas fa-2'; 
    case 'Num3': return 'fas fa-3'; 
    case 'Num4': return 'fas fa-4'; 
    case 'Num5': return 'fas fa-5'; 
    case 'Num6': return 'fas fa-6'; 
    case 'Num7': return 'fas fa-7'; 
    case 'Num8': return 'fas fa-8'; 
    case 'Num9': return 'fas fa-9'; 
    case 'Num0': return 'fas fa-0'; 

    case 'KeyPad1': return 'fas fa-1'; 
    case 'KeyPad2': return 'fas fa-2'; 
    case 'KeyPad3': return 'fas fa-3'; 
    case 'KeyPad4': return 'fas fa-4'; 
    case 'KeyPad5': return 'fas fa-5'; 
    case 'KeyPad6': return 'fas fa-6'; 
    case 'KeyPad7': return 'fas fa-7'; 
    case 'KeyPad8': return 'fas fa-8'; 
    case 'KeyPad9': return 'fas fa-9'; 
    case 'KeyPad0': return 'fas fa-0'; 

    // 控制键
    case 'Backspace': return 'fas fa-backspace'; 
    case 'Enter': return 'fas fa-arrow-right'; 
    case 'Return': return 'fas fa-arrow-right'; 
    case 'Escape': return 'fas fa-times';   
    case 'ShiftLeft': return 'fas fa-arrow-up'; 
    case 'ShiftRight': return 'fas fa-arrow-up'; 
    case 'ControlLeft': return 'fas fa-arrow-left'; 
    case 'ControlRight': return 'fas fa-arrow-left'; 
    case 'Alt': return 'fas fa-arrow-down'; 
    case 'Space': return 'fas fa-space-shuttle'; // 空格键可以使用一个特别的图标

    // 箭头键
    case 'UpArrow': return 'fas fa-arrow-up'; 
    case 'DownArrow': return 'fas fa-arrow-down'; 
    case 'LeftArrow': return 'fas fa-arrow-left'; 
    case 'RightArrow': return 'fas fa-arrow-right'; 
    //page  PageDown  PageUp
    case 'PageUp': return 'fas fa-arrow-up'; 
    case 'PageDown': return 'fas fa-arrow-down'; 
    // 其他键
    case '-': return 'fas fa-minus'; //Minus
    case 'Minus': return 'fas fa-minus'; //Minus
    case '=': return 'fas fa-equals'; 
    case 'Equal': return 'fas fa-equals'; 
    case '!': return 'fas fa-exclamation'; 
    case 'exclamation': return 'fas fa-exclamation'; 
    case '@': return 'fas fa-at'; 
    case '#': return 'fas fa-hashtag'; 
    case '$': return 'fas fa-dollar-sign'; 
    case '%': return 'fas fa-percent'; 
    case '^': return 'fas fa-caret-up'; 
    case '&': return 'fas fa-ampersand'; 
    case '*': return 'fas fa-asterisk'; 
    case '(': return 'fas fa-parentheses'; 
    case ')': return 'fas fa-parentheses'; 
    case '_': return 'fas fa-underscore'; 
    case '+': return 'fas fa-plus'; 

    case '\\': return 'fas fa-backslash'; 
    case ';': return 'fas fa-semicolon'; 
    case "'": return 'fas fa-quote-right'; 
    case '.': return 'fas fa-period'; 
    case 'BackSlash': return 'fas fa-slash'; 
    case 'Slash': return 'fas fa-slash'; 

    // 默认图标
    default: return key; // 未定义的键显示问号图标
  }
};

const strToKey = (key: string): string => {
  switch(key) {
    case 'BackQuote': return '~'; 
    case 'CapsLock': return '大写'; 
    case 'NumLock': return '数字';
    case 'L_Shift': return 'shift'; 
    case 'R_Shift': return 'shift'; 
    case 'L_Control': return 'ctrl'; 
    case 'R_Control': return 'ctrl'; 
    case 'L_Alt': return 'alt'; 
    case 'R_Alt': return 'alt'; 
    case 'L_Windows': return 'win'; 
    case 'R_Windows': return 'win'; 
    case 'Comma': return ',';
    case 'Dot': return '.';
    case 'SemiColon': return ';';
    case 'Quote': return '\'';
    case 'L_Bracket': return '['; 
    case 'R_Bracket': return ']'; 

    case 'Insert': return 'ins'; 
    case 'Home': return 'home'; 
    case 'Delete': return 'del'; 
    case 'R_Bracket': return ']'; 
    case 'R_Bracket': return ']'; 
    case 'R_Bracket': return ']'; 
    case 'R_Bracket': return ']'; 
    default: return key; // 未定义的键显示问号图标1
  }
}



// 设置窗口大小
function setWindowSize() {
  appWindow.setSize(new LogicalSize((keys.value.length) * 70 + 10 ,60));
  appWindow.setPosition(new LogicalPosition((width.value / 2) - (keys.value.length) * 35 - 5, height.value - 60 * 2));
}

// 生成随机颜色
const randomColor = (): string => {
  const r = Math.floor(Math.random() * 256);
  const g = Math.floor(Math.random() * 256);
  const b = Math.floor(Math.random() * 256);
  const a = 0.7;
  return `rgba(${r}, ${g}, ${b}, ${a})`;
};

// 每秒更新一次时间戳
setPromiseInterval(async () => {
  console.log("数量：" + keys.value.length)
  if (keys.value.length === 0) {
    appWindow.setSize(new LogicalSize(0 ,60));
    appWindow.hide();
  } else {
    const now = Date.now();
    for (let i = 0; i < keys.value.length; i++) {
      if ((now - keys.value[i].time) > 3000) {
        keys.value.splice(i, 1);
        if (keys.value.length === 0) {
          appWindow.setSize(new LogicalSize(0 ,60));
          appWindow.hide();
        } else {
          setWindowSize();
        }
      }
    }
  }
}, keys.value.length === 0 ? 1000 : 0);




// 调整字体大小以适应容器
const adjustFontSizes = () => {
  keycapRefs.value.forEach((keycap, index) => {
    const text = textRefs.value[index];
    if (text && keycap) {
      let fontSize = 20; // 初始字体大小
      text.style.fontSize = `${fontSize}px`;

      while (text.scrollWidth > keycap.clientWidth - 10 && fontSize > 10) {
        fontSize -= 1;
        text.style.fontSize = `${fontSize}px`;
      }
    }
  });
};


</script>

<style scoped>

.keycap-container1 {
  display: inline-block;
  padding: 10px 15px;
  background-color: #f0f0f0;
  border: 1px solid #ccc;
  border-radius: 5px;
  font-family: Arial, sans-serif;
  font-size: 16px;
  text-align: center;
}


.keycap-container {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  justify-content: center;
  overflow-y: hidden;
}

.keycap {
  width: 60px;
  height: 60px;
  background-color: #ddd;
  border-radius: 8px;
  box-shadow: 0px 4px 8px rgba(0, 0, 0, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: bold;
  color: #333;
  transition: transform 0.2s;
  cursor: pointer;
}

.keycap:active {
  transform: translateY(2px);
}

.keycap-text {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  text-align: center;
  font-size: 20px;
  overflow: hidden;
  white-space: nowrap;
  transition: font-size 0.2s;
}

.keycap-text i {
  font-size: 24px;
  color: #333;
  transition: font-size 0.2s;
}
</style>