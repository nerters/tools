<template>
  <div class="keycap-container">
    <div class="keycap" v-for="(item, index) in keys" :key="item.key + index" :style="{ backgroundColor: randomColor() }">
      <span class="keycap-text">
        <i :class="getIconClass(item.key)" style="font-size: 24px; color: #333;"></i>
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
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
});

// 获取键对应的 Font Awesome 图标
const getIconClass = (key: string): string => {
  switch (key) {
    // 字母键
    case 'KeyA': return 'fas fa-a'; 
    case 'KeyB': return 'fas fa-b'; 
    case 'KeyC': return 'fas fa-c'; 
    case 'KeyD': return 'fas fa-d'; 
    case 'KeyE': return 'fas fa-e'; 
    case 'KeyF': return 'fas fa-f'; 
    case 'KeyG': return 'fas fa-g'; 
    case 'KeyH': return 'fas fa-h'; 
    case 'KeyI': return 'fas fa-i'; 
    case 'KeyJ': return 'fas fa-j'; 
    case 'KeyK': return 'fas fa-k'; 
    case 'KeyL': return 'fas fa-l'; 
    case 'KeyM': return 'fas fa-m'; 
    case 'KeyN': return 'fas fa-n'; 
    case 'KeyO': return 'fas fa-o'; 
    case 'KeyP': return 'fas fa-p'; 
    case 'KeyQ': return 'fas fa-q'; 
    case 'KeyR': return 'fas fa-r'; 
    case 'KeyS': return 'fas fa-s'; 
    case 'KeyT': return 'fas fa-t'; 
    case 'KeyU': return 'fas fa-u'; 
    case 'KeyV': return 'fas fa-v'; 
    case 'KeyW': return 'fas fa-w'; 
    case 'KeyX': return 'fas fa-x'; 
    case 'KeyY': return 'fas fa-y'; 
    case 'KeyZ': return 'fas fa-z'; 

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

    case 'Kp1': return 'fas fa-1'; 
    case 'Kp2': return 'fas fa-2'; 
    case 'Kp3': return 'fas fa-3'; 
    case 'Kp4': return 'fas fa-4'; 
    case 'Kp5': return 'fas fa-5'; 
    case 'Kp6': return 'fas fa-6'; 
    case 'Kp7': return 'fas fa-7'; 
    case 'Kp8': return 'fas fa-8'; 
    case 'Kp9': return 'fas fa-9'; 
    case 'Kp0': return 'fas fa-0'; 

    // 控制键
    case 'Backspace': return 'fas fa-backspace'; 
    case 'Enter': return 'fas fa-arrow-right'; 
    case 'Return': return 'fas fa-arrow-right'; 
    case 'Escape': return 'fas fa-times'; 
    case 'Tab': return 'fas fa-arrow-right'; 
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
    case '[': return 'fas fa-brackets'; 
    case ']': return 'fas fa-brackets'; 
    case '\\': return 'fas fa-backslash'; 
    case ';': return 'fas fa-semicolon'; 
    case "'": return 'fas fa-quote-right'; 
    case ',': return 'fas fa-comma'; 
    case '.': return 'fas fa-period'; 
    case 'Dot': return 'fas fa-period';
    case '/': return 'fas fa-slash'; 
    case 'Slash': return 'fas fa-slash'; 

    // 默认图标
    default: return 'fas fa-question'; // 未定义的键显示问号图标
  }
};



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
</script>

<style scoped>
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
  font-size: 20px;
}

.keycap-text i {
  font-size: 24px;
  color: #333;
}
</style>