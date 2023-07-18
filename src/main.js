import 'element-plus/dist/index.css'
import "./assets/style/global.scss";
import {createApp} from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import router from "./common/router.js";
import i18n from "./common/i18n.js";
import {Event} from "./common/event"

const app = createApp(App);
import store from "./store/index"

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}


app.use(i18n)
app.use(router)
app.use(store)
app.use(ElementPlus)
app.mount("#app");

app.config.globalProperties.$event = new Event()
