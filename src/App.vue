<template>
  <el-config-provider :locale="locale" :button="config_btn">
    <router-view/>
  </el-config-provider>
</template>


<script setup>
import {computed, reactive, onBeforeMount, onBeforeUnmount, getCurrentInstance} from 'vue'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import en from 'element-plus/dist/locale/en.mjs'
import {Event, Topic} from "./common/event.js"
import message_hub from "./common/message_hub.js";
import {useStore} from "vuex";

const store = useStore()
const {proxy} = getCurrentInstance()
const locale = computed(_ => {
  return store.state.setting.language === 'zh-CN' ? zhCn : en
})
const config_btn = reactive({
  autoInsertSpace: true,
})
const event = new Event();

onBeforeMount(_ => {
  bindEvent()
})

onBeforeUnmount(_ => {
  unbindEvent()
})

function bindEvent() {
  message_hub()
  event.subscribe(Topic.LANGUAGE_CHANGED, onLanguageChanged)
}

function unbindEvent() {
  event.unsubscribe(Topic.LANGUAGE_CHANGED, onLanguageChanged)
}

function onLanguageChanged(lang) {
  proxy.$i18n.locale = lang
}


</script>

<style lang="scss">
</style>