<template>
  <div class="search-folder-form">
    <el-cascader v-model="content" :options="subsets" style="width: 100%" :show-all-levels="false"
                 :props="{multiple:true, emitPath:true, checkStrictly: true}"/>
  </div>
</template>

<script setup>
import {invoke} from "@tauri-apps/api";
import {onMounted, ref, watch} from 'vue'

let drawer = ref()
let subsets = ref([])
let content = ref()

let props = defineProps({
  ids: Object,
})

let emits = defineEmits(['update:ids'])

watch(content, _ => {
  let temp = []
  if (content.value) {
    temp = content.value[content.value.length - 1]
  }
  emits('update:ids', temp)
})
watch(drawer, async _ => {
  await fetchSubsets()
})

onMounted(async _ => {
  await fetchDrawer()
})

async function fetchDrawer() {
  drawer.value = await invoke("find_current_drawer")
}

async function fetchSubsets() {
  subsets.value = await invoke('find_folder_cascader', {
    drawerId: drawer.value.id
  })
}

</script>

<style lang="scss" scoped>
.search-folder-form {

}
</style>