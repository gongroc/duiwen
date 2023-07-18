<template>
  <status-bar-item-base id="status-bar-menu-counter" v-if="counter">
    <el-icon>
      <Folder/>
    </el-icon>
    {{ counter.folder }}
    <div class="split"></div>
    <el-icon>
      <Document/>
    </el-icon>
    {{ counter.article }}
  </status-bar-item-base>
</template>

<script setup>
import StatusBarItemBase from "../widget/StatusBarItemBase.vue";
import {ref, watch, onMounted} from "vue"
import {invoke} from "@tauri-apps/api";
import {Topic, Event} from "../common/event.js";

const event = new Event()

let props = defineProps({
  folder: Object
})

let counter = ref()

watch(_ => props.folder, async value => {
  if (value) {
    await fetchData()
  } else {
    counter.value = undefined
  }
})


onMounted(async _ => {
  await bindEvent()
})

async function fetchData() {
  if (!props.folder) {
    return
  }
  counter.value = await invoke("folder_counter", {
    folderId: props.folder.id
  })
}

async function bindEvent() {
  event.subscribe(Topic.ARTICLE_CREATED, fetchData)
  event.subscribe(Topic.ARTICLE_DELETED, fetchData)
  event.subscribe(Topic.FOLDER_CREATED, fetchData)
  event.subscribe(Topic.FOLDER_UNLINKED, fetchData)
  event.subscribe(Topic.FOLDER_DELETED, fetchData)
}


</script>

<style lang="scss" scoped>

</style>