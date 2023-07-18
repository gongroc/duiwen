<template>
  <side-item-base :active="active" @title-click="onClick" @info-click="onClick">
    <template #icon>
      <el-icon>
        <Document/>
      </el-icon>
    </template>
    <template #title>
      {{ title }}
    </template>
    <template #info>
      {{ intro }}
    </template>
    <template #items>
      <el-dropdown-item icon="star" @click="cancelFavorite" v-if="favorite">
        {{ $t('common.uncollect') }}
      </el-dropdown-item>
      <el-dropdown-item icon="star" @click="addFavorite" v-else>
        {{ $t('common.collect') }}
      </el-dropdown-item>
      <el-dropdown-item divided icon="delete" @click="onDelete">
        {{ $t('common.delete') }}
      </el-dropdown-item>
    </template>
  </side-item-base>
</template>

<script setup>
import SideItemBase from "../widget/SideItemBase.vue";
import {invoke} from "@tauri-apps/api";
import {ElMessageBox} from "element-plus";
import {useI18n} from "vue-i18n";
import {Topic} from "../common/event.js";
import {getCurrentInstance} from "vue";

const {proxy} = getCurrentInstance()

const {t} = useI18n()

let emits = defineEmits(['changed', 'click'])

let props = defineProps({
  active: Boolean,
  title: String,
  intro: String,
  favorite: Object,
  id: Number
})

async function onDelete() {
  await ElMessageBox.confirm(t("document.confirmDelete"))
  await invoke("remove_article", {
    id: props.id
  })
  emits('changed')
  proxy.$event.publish(Topic.ARTICLE_DELETED, props.id)
}

async function addFavorite() {
  await invoke("add_article_favorite", {
    articleId: props.id
  })
  emits("changed")
}

async function cancelFavorite() {
  await invoke("remove_article_favorite", {
    articleId: props.id
  })
  emits("changed")
}

function onClick() {
  emits("click")
}

</script>

<style lang="scss" scoped>

</style>