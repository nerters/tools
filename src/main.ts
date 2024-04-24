import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

import router from './router'
import App from './App.vue'

import VueGridLayout from 'vue-grid-layout'

const app = createApp(App)

app.use(ElementPlus).use(VueGridLayout).use(router)

app.mount('#app')