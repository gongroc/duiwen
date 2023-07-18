import {createRouter, createWebHashHistory} from "vue-router"
import Home from "../pages/Home.vue";
import Setting from "../pages/setting/Index.vue"
import Init from "../pages/Init.vue"
import Independent from "../pages/Independent.vue"

const routes = [
    {
        path: '/home',
        component: Home,
    },
    {
        path: '/setting',
        component: Setting,
    },
    {
        path: '/init',
        component: Init
    },
    {
        path: "/independent",
        component: Independent
    },
    {path: '/', redirect: '/init'}
]

export default createRouter({
    history: createWebHashHistory(),
    routes,
})