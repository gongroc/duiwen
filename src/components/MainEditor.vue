<template>
  <div id="editor-instance" v-if="article">
    <div id="head">
      <div id="title">
        <input :placeholder="$t('document.title')" :disabled="!article.editing" v-model="article.title"
               @input="onModifyTitle"/>
      </div>
      <div id="options">
        <el-icon class="opt" @click="onToggleEditing">
          <Edit v-if="article.editing"/>
          <View v-else/>
        </el-icon>
        <el-dropdown placement="bottom-end" size="small" v-if="!isIndependent">
          <el-icon class="opt">
            <MoreFilled/>
          </el-icon>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item icon="refresh" @click="onRefresh">
                {{ $t('common.refresh') }}
              </el-dropdown-item>
              <el-dropdown-item icon="promotion">
                {{ $t('document.exportPDF') }}
              </el-dropdown-item>
              <el-dropdown-item icon="starFilled" @click="onToggleFavorite">
                {{ $t(favorite ? 'common.uncollect' : 'common.collect') }}
              </el-dropdown-item>
              <el-dropdown-item icon="folder">
                {{ $t('document.manageFolders') }}
              </el-dropdown-item>
              <el-dropdown-item icon="deleteFilled" @click="onDelete">
                {{ $t('document.deleteDocument') }}
              </el-dropdown-item>
              <el-dropdown-item icon="infoFilled" divided>
                {{ $t('document.checkTheDetails') }}
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>

      </div>
    </div>
    <div id="wrapper">
      <markdown-editor v-model:editing="article.editing" :content="article.content" @change="onChange"
                       @save-key-pressed="onSaveContent"/>
    </div>
  </div>
</template>

<script setup>
import MarkdownEditor from "../widget/MarkdownEditor.vue";
import {computed, onBeforeMount, ref, watch} from "vue"
import {invoke} from "@tauri-apps/api";
import {Topic} from "../common/event.js";
import {useStore} from 'vuex'
import {Event} from "../common/event.js";
import {useRoute} from 'vue-router'
import {ElMessageBox} from "element-plus";
import {useI18n} from "vue-i18n";

const store = useStore()
const event = new Event()
const {t} = useI18n()

let firstOpen = true;
let cfg = computed(_ => store.state.setting.editor)
let article = ref()
let favorite = ref()
let props = defineProps({
  id: Number,
  isIndependent: {
    type: Boolean,
    default: false
  }
})

watch(_ => props.id, async _ => {
  await onFectData()
  await onFetchFavorite()
})

onBeforeMount(async _ => {
  await onFectData()
  await onFetchFavorite()
  await bindEvent()
})

async function bindEvent() {
  event.subscribe(Topic.COMMOND_SAVE_ARTICLE, onSaveContent)
}

async function onRefresh() {
  await onFectData()
  event.publish(Topic.ARTICLE_CONTENT_SAVED, props.id)
}

async function onFectData() {
  article.value = await invoke("find_article_by_id", {
    id: props.id
  })
  recoverSnaphot()
}

async function onFetchFavorite() {
  favorite.value = await invoke("find_article_favorite", {
    articleId: props.id
  })
}

async function onToggleFavorite() {
  await invoke(favorite.value ? 'remove_article_favorite' : 'add_article_favorite', {
    articleId: props.id
  })
  await onFetchFavorite()
}

function recoverSnaphot() {
  let snaphot = store.state.snapshot.article.find(item => item.id === props.id)
  if (snaphot) {
    article.value.content = snaphot.content
    event.publish(Topic.ARTICLE_CONTENT_CHANGED, props.id)
  }
}


async function onChange(value) {
  if (firstOpen) {
    firstOpen = false
    return
  }
  if (article.value.content === value) {
    return
  }
  article.value.content = value
  if (cfg.value.autoSave) {
    await onSaveContent()
  } else {
    store.commit('snapshot/pushArticle', {id: props.id, content: value})
    event.publish(Topic.ARTICLE_CONTENT_CHANGED, props.id)
  }
}

async function onSaveContent() {
  await invoke("modify_article_content", {
    id: props.id,
    content: article.value.content
  })
  store.commit("snapshot/removeArticle", props.id)
  event.publish(Topic.ARTICLE_CONTENT_SAVED, props.id)
}


async function onToggleEditing() {
  await invoke("toggle_article_editing", {
    editing: !article.value.editing,
    id: props.id
  })
  article.value.editing = !article.value.editing
}

async function onModifyTitle() {
  try {
    await invoke('modify_article_title', {
      id: props.id,
      title: article.value.title
    })

    event.publish(Topic.ARTICLE_TITLE_CHANGED, {
      id: props.id,
      title: article.value.title
    })

  } catch (err) {
    console.log(err)
  }
}

async function onDelete() {
  await ElMessageBox.confirm(t("document.confirmDelete" + '?'))
  await invoke("remove_article", {
    id: props.id
  })
  event.publish(Topic.ARTICLE_DELETED, props.id)
}


</script>

<style lang="scss" scoped>
#editor-instance {
  flex: 1;
  height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;

  #head {
    display: flex;
    justify-items: center;
    align-items: center;

    #title {
      flex: 1;
      user-select: none;

      input {
        width: 100%;
        outline: none;
        border: none;
        padding: 15px 30px;
        font-weight: bold;
        font-size: 24px;
        color: $color-primary-text;
      }

      input:disabled {
        background: white;
      }

      input::placeholder {
        color: $color-secondary-text;
      }

    }

    #options {
      user-select: none;
      color: $color-secondary-text;
      display: flex;
      align-items: center;
      justify-items: center;
      margin-left: 15px;

      .opt {
        cursor: pointer;
        font-size: 16px;
        margin-right: 15px;
        display: block;
        user-select: none;
      }

      .opt:hover {
        color: $color-brand;
      }

    }

  }

  #wrapper {
    flex: 1;
    height: 0;
    display: flex;
    flex-direction: column;
  }

}
</style>