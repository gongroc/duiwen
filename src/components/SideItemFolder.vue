<template>
  <side-item-base @info-click="onClick" @title-click="onClick">
    <template #icon>
      <el-icon>
        <Folder/>
      </el-icon>
    </template>
    <template #title>
      {{ title }}
    </template>
    <template #info v-if="intro && !isToplevel">
      {{ intro }}
    </template>
    <template #items>
      <el-dropdown-item icon="link" @click="unlink" v-if="!isToplevel && !!relationId">
        {{ $t('common.unlink') }}
      </el-dropdown-item>
      <el-dropdown-item icon="delete" @click="onDelete">
        {{ $t('common.delete') }}
      </el-dropdown-item>
    </template>

  </side-item-base>
</template>

<script setup>
import SideItemBase from "../widget/SideItemBase.vue";
import {invoke} from "@tauri-apps/api";
import {ElMessageBox} from 'element-plus'
import {useI18n} from "vue-i18n";
import {getCurrentInstance} from "vue";
import {Topic} from "../common/event.js";

const {t} = useI18n()
const {proxy} = getCurrentInstance()
let emits = defineEmits(['changed', "click"])

let props = defineProps({
  title: String,
  intro: String,
  hideUnlink: Boolean,
  isToplevel: Boolean,
  id: Number,
  relationId: Number,
  favorite: Object
})

async function onDelete() {
  await ElMessageBox.confirm(t("folder.deleteInfo"), t("folder.confirmDelete") + "?")
  await invoke("remove_folder", {id: props.id})
  emits('changed')
  proxy.$event.publish(Topic.FOLDER_DELETED)
}

async function unlink() {
  await ElMessageBox.confirm(t("folder.unlinkInfo"), t("folder.confirmUnlink") + '?')
  await invoke("unlink_folder", {
    relationId: props.relationId
  })
  emits("changed")
  proxy.$event.publish(Topic.FOLDER_UNLINKED)
}

function onClick() {
  emits("click")
}

</script>

<style lang="scss" scoped>

</style>