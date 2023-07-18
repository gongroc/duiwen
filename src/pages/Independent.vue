<template>
  <simple-layout @before-close="onBeforeClose">
    <main-editor :id="id" :is-independent="true"/>
  </simple-layout>
</template>

<script setup>
import SimpleLayout from "../widget/SimpleLayout.vue";
import {onBeforeMount, onMounted, ref} from "vue"
import {appWindow} from "@tauri-apps/api/window";
import {invoke} from "@tauri-apps/api";
import MainEditor from "../components/MainEditor.vue";
import {useStore} from "vuex";
import {useRoute} from "vue-router";
import {emit} from "@tauri-apps/api/event"
import {Event, Topic} from "../common/event.js";

const store = useStore()
const route = useRoute()
const event = new Event()
let id = ref()

onBeforeMount(_ => {
  id.value = Number.parseInt(route.query.id)
  bindEvent()
})

onMounted(_ => {
  showWindow()
})

function showWindow() {
  appWindow.center()
  appWindow.show()
  invoke("set_window")
}

function onBeforeClose() {
  emit("window-closed", id.value)
}

function bindEvent() {
  event.subscribe(Topic.ARTICLE_TITLE_CHANGED, payload => {
    emit('changed', {
      payload: payload,
      topic: Topic.ARTICLE_TITLE_CHANGED
    })
  })
}


</script>

<style lang="scss" scoped>

</style>