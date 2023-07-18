<template>
  <el-scrollbar>
    <div class="basic-info" v-if="article">
      <kv-item>
        <template #k>
          {{ $t('common.operate') }}
        </template>
        <template #v>
          <el-button icon="star" size="small" @click="toggleFaviorte">
            {{ $t(favorite ? 'common.uncollect' : 'common.collect') }}
          </el-button>
          <el-button size="small">{{ $t('document.exportPDF') }}</el-button>
          <el-button @click="onDelete" size="small">{{ $t("common.delete") }}</el-button>
        </template>
      </kv-item>

      <kv-item>
        <template #k>
          {{ $t('common.introduction') }}
        </template>
        <template #v>
          <el-input type="textarea" :rows="5" v-model="intro" @input="onModifyIntro"/>
        </template>
      </kv-item>

      <kv-item>
        <template #k>
          {{ $t('common.diskUsage') }}
        </template>
        <template #v>
          300kb &nbsp;&nbsp; <a href="javascript:">{{ $t('common.openFolder') }}</a>
        </template>
      </kv-item>

      <kv-item>
        <template #k>
          {{ $t('common.creationTime') }}
        </template>
        <template #v>
          {{ article.created_at }}
        </template>
      </kv-item>

      <kv-item>
        <template #k>
          {{ $t('common.latestUpdate') }}
        </template>
        <template #v>
          {{ article.updated_at }}
        </template>
      </kv-item>

    </div>
  </el-scrollbar>
</template>

<script setup>
import KvItem from "../widget/KvItem.vue";
import {onBeforeMount, ref, watch} from "vue";
import {invoke} from "@tauri-apps/api";
import {Event, Topic} from "../common/event.js";
import {useI18n} from "vue-i18n";
import {ElMessageBox} from "element-plus";

const {t} = useI18n()
const event = new Event()
let article = ref()
let intro = ref()
let favorite = ref()
let props = defineProps({
  id: Number
})

async function onDelete() {
  await ElMessageBox.confirm(t('document.confirmDelete') + '?')
  await invoke('remove_article', {id: props.id})
  event.publish(Topic.ARTICLE_DELETED, props.id)
}

watch(_ => props.id, async _ => {
  await fetchData()
  await fetchFaviorte()
})

watch(article, value => {
  intro.value = value.intro
})

onBeforeMount(async _ => {
  await fetchData()
  await fetchFaviorte()
})

async function fetchData() {
  article.value = await invoke("find_article_by_id", {
    id: props.id
  })
}

async function fetchFaviorte() {
  favorite.value = await invoke('find_article_favorite', {
    articleId: props.id
  })
}

async function toggleFaviorte() {
  await invoke(favorite.value ? "remove_article_favorite" : "add_article_favorite", {
    articleId: props.id
  })
  await fetchFaviorte()
}

async function onModifyIntro() {
  await invoke('modify_article_intro', {
    id: props.id,
    intro: intro.value
  })

  event.publish(Topic.ARTICLE_INTRO_CHANGED, {
    id: props.id,
    intro: intro.value
  })

}


</script>

<style lang="scss" scoped>
.basic-info {
  user-select: none;

  .el-textarea {
    width: 450px;
  }

  a {
    font-size: 12px;
    color: $color-secondary-text;
  }

  a:hover {
    color: $color-brand;
  }
}
</style>