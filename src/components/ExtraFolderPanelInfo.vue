<template>
  <div class="extra-folder-panel-info" v-if="folder">
    <div class="title">
      <input :placeholder="$t('folder.folderName')" v-model="folder.title" @input="onModifyTitle"/>
      <el-dropdown size="small" placement="bottom-end">
        <el-icon class="opt">
          <Setting/>
        </el-icon>
        <template #dropdown>
          <el-dropdown-item icon="delete">
            {{ $t('common.delete') }}
          </el-dropdown-item>
        </template>
      </el-dropdown>
    </div>
    <markdown-editor :key="'folder-intro-' + id" :content="folder.intro" @change="onModifyIntro" simple editing/>
  </div>
</template>

<script setup>
import MarkdownEditor from "../widget/MarkdownEditor.vue";
import {onMounted, ref, watch} from 'vue'
import {invoke} from "@tauri-apps/api";

let folder = ref()
let props = defineProps({
  id: Number
})
let title = ref()

watch(_ => props.id, async _ => {
  await fetchData()
})

onMounted(async _ => {
  await fetchData()
})

async function fetchData() {
  folder.value = await invoke('find_folder', {
    id: props.id
  })
}

async function onModifyTitle() {
  await invoke("modify_folder_title", {
    id: props.id,
    title: folder.value.title
  })
}

async function onModifyIntro(value) {
  await invoke("modify_folder_intro", {
    id: props.id,
    intro: value
  })
}


</script>

<style lang="scss" scoped>
.extra-folder-panel-info {
  flex: 1;
  flex-direction: column;
  height: 0;
  overflow-y: hidden;
  display: flex;

  .title {
    display: flex;
    align-items: center;
    border-bottom: 1px solid $color-base-border;

    input {
      border: none;
      outline: none;
      padding: 15px 30px;
      font-size: 18px;
      font-weight: bold;
      flex: 1;
    }

    .opt {
      margin-right: 20px;
      cursor: pointer;
      color: $color-secondary-text;
    }

  }

  .editor {
    flex: 1 !important;
  }

}
</style>
