<template>
  <main-layout v-model:extra-visible="extra.visible">
    <template #menu>
      <div id="board">
        <side-group-board @new-folder-click="createFolderDialogVisible = true" v-model:drawer="drawer"
                          :folder="folder" v-model:is-favorites-list="isFavoritesList"/>
        <side-search v-model:keyword="keyword"/>
      </div>

      <favorites-side v-if="isFavoritesList" :keyword="keyword" :opened="titles.opened"
                      @article-clicked="onPushArticle"/>
      <main-side :drawer="drawer" :keyword="keyword" v-model:folder="folder"
                 @article-clicked="onPushArticle" @folder-clicked="onOpenFolder"
                 v-if="drawer && !isFavoritesList" :opened="titles.opened"/>

    </template>

    <template #status>
      <status-bar-document>
        <template #left>
          <status-bar-menu-counter :folder="folder"/>
        </template>
        <template #right>

          <template v-if="titles.opened">
            <template v-if="titles.opened.type === 'article'">
              <status-bar-document-linked-folder @click="onOpenExtra('article','linked-folder')"
                                                 :active="extra.visible && extra.type === 'article' && extra.submenu === 'linked-folder'"/>
              <status-bar-document-basic-info @click="onOpenExtra('article','basic-info')"
                                              :active="extra.visible && extra.type === 'article' && extra.submenu === 'basic-info'"/>
              <status-bar-document-save/>
            </template>
            <template v-if="titles.opened.type === 'graph'">
              <status-bar-folder-linked-documents @click="onOpenExtra('folder','linked-article')"
                                                  :active="extra.visible && extra.type === 'folder' && extra.submenu === 'linked-article'"/>
              <status-bar-folder-linked-folder @click="onOpenExtra('folder','linked-folder')"
                                               :active="extra.visible && extra.type === 'folder' && extra.submenu === 'linked-folder'"/>
              <status-bar-folder-detail @click="onOpenExtra('folder','detail')"
                                        :active="extra.visible && extra.type === 'folder' && extra.submenu === 'detail'"/>
            </template>
          </template>

        </template>
      </status-bar-document>
    </template>

    <breadcrumbs v-model:folder="folder" :drawer="drawer" :is-favorites-list="isFavoritesList" v-if="drawer"/>

    <title-bar @click="onChange" v-model:opened="titles.opened" v-model:artiles="titles.artiles" :drawer="drawer"
               @open-drawer="onOpenDrawer" @open-graph="onOpenGraph" @open-article="onOpenArticle"
               :folder="folder" :is-favorites-list="isFavoritesList"/>
    <main-empty v-if="!titles.opened"/>
    <template v-else>
      <main-editor :id="titles.opened.id" v-if="titles.opened.type === 'article'"/>
      <MainFolderGraph3D v-if="titles.opened.type === 'graph'" :drawer="drawer" :folder="folder"/>
      <main-group-detail v-if="titles.opened.type === 'drawer'"/>
    </template>

    <template #extra>
      <extra-document-panel :id="titles.opened.id" v-model:submenu="extra.submenu"
                            v-if="extra.visible && extra.type === 'article'"/>
      <extra-folder-panel :id="folder.id" v-model:submenu="extra.submenu"
                          v-if="extra.visible && extra.type === 'folder'"/>
    </template>

    <template #tool-bar>
      <tool-bar-history/>
    </template>

    <template v-if="false" #extra-layer>
      <tutorial/>
    </template>

  </main-layout>
</template>

<script setup>
import MainLayout from "../widget/MainLayout.vue";
import SideGroupBoard from "../components/SideGroupBoard.vue"
import SideSearch from "../components/SideSearch.vue";
import Breadcrumbs from "../components/Breadcrumbs.vue";
import MainEditor from "../components/MainEditor.vue";
import StatusBarMenuCounter from "../components/StatusBarMenuCounter.vue";
import StatusBarDocument from "../widget/StatusBarDocument.vue";
import StatusBarDocumentSave from "../components/StatusBarDocumentSave.vue";
import StatusBarDocumentLinkedFolder from "../components/StatusBarDocumentLinkedFolder.vue";
import StatusBarDocumentBasicInfo from "../components/StatusBarDocumentBasicInfo.vue";
import ExtraDocumentPanel from "../components/ExtraDocumentPanel.vue";
import {onBeforeMount, onMounted, ref, watch} from "vue";
import MainFolderGraph3D from "../components/MainFolderGraph3D.vue";
import StatusBarFolderDetail from "../components/StatusBarFolderDetail.vue";
import ExtraFolderPanel from "../components/ExtraFolderPanel.vue";
import StatusBarFolderLinkedFolder from "../components/StatusBarFolderLinkedFolder.vue";
import StatusBarFolderLinkedDocuments from "../components/StatusBarFolderLinkedDocuments.vue";
import ToolBarHistory from "../components/ToolBarHistory.vue";
import MainGroupDetail from "../components/MainGroupDetail.vue";
import TitleBar from "../components/TitleBar.vue";
import MainEmpty from "../components/MainEmpty.vue";
import Tutorial from "../components/Tutorial.vue"
import {ipc} from "../common/utils.js";
import {ElNotification} from 'element-plus'
import {useI18n} from "vue-i18n";
import MainSide from "../components/MainSide.vue";
import FavoritesSide from "../components/FavoritesSide.vue";
import {Event, Topic} from "../common/event.js";
import {invoke} from "@tauri-apps/api";
import store from "../store/index.js";

const {t} = useI18n()
const event = new Event()

// 当前文件夹组
let drawer = ref()
// 当前文件夹
let folder = ref()
// 搜索关键词
let keyword = ref()
// 是否为收藏夹列表
let isFavoritesList = ref(false)
let extra = ref({
  visible: false,
  type: '',
  submenu: ''
})

let titles = ref({
  artiles: [],
  opened: undefined,
})



watch(_ => titles.value.opened, (value, oldValue, onCleanup) => {
  let flag = false
  if (value) {
    if (oldValue !== undefined && value.type !== oldValue.type) {
      flag = true
    }
  } else {
    flag = true
  }
  if (flag) {
    extra.value.visible = false
  }
}, {deep: true})

watch(drawer, _ => {
  folder.value = undefined
})

watch(isFavoritesList, value => {
  if (value) {
    folder.value = undefined
    titles.value.opened = undefined
  }
})

onMounted(async _ => {
  await bindEvent()
  store.commit("snapshot/removeAllIndependent")
})

function onOpenExtra(type, submenu) {

  if (extra.value.visible) {
    if (extra.value.type === type && extra.value.submenu === submenu) {
      extra.value.visible = false
    }
  } else {
    extra.value.visible = true
  }

  extra.value.type = type
  extra.value.submenu = submenu
}


async function onPushArticle(item) {
  let flag = false;
  for (let artile of titles.value.artiles) {
    if (artile.id === item.id) {
      flag = true
      break
    }
  }
  if (!flag) {
    titles.value.artiles.push({
      id: item.id,
      title: item.title,
      notSaved: false,
      pined: false,
    })
  }
  await onOpenArticle(item)
}

async function onOpenArticle(item) {
  titles.value.opened = {
    id: item.id,
    title: item.title,
    type: 'article'
  }
  await invoke('push_article_record', {
    articeId: item.id
  })
}

async function onOpenFolder(item) {
  // titles.value.opened = {
  //   id: item.id,
  //   title: item.title,
  //   type: "folder"
  // }

  await invoke('push_folder_record', {
    folderId: item.id
  })
}

async function onOpenDrawer() {
  titles.value.opened = {
    type: 'drawer'
  }
}

async function onOpenGraph() {
  titles.value.opened = {
    type: 'graph'
  }
}

async function bindEvent() {
  event.subscribe(Topic.ARTICLE_DELETED, clearTitlesWhenArticleChanged)
  event.subscribe(Topic.ARTICLE_CREATED, autoOpenWhenArticleCreated)
  event.subscribe(Topic.ARTICLE_TITLE_CHANGED, onArticleTitleChanged)
  event.subscribe(Topic.COMMAND_OPEN_FOLDER, onOpenFolderCommand)
  event.subscribe(Topic.COMMAND_OPEN_ARTICLE, onOpenArticleCommand)
}

async function onOpenFolderCommand(id) {
  folder.value = await invoke("find_folder", {id: id})
}

async function onOpenArticleCommand(id) {
  let article = await invoke("find_article_by_id", {
    id: id
  })
  await onPushArticle(article)
}

function clearTitlesWhenArticleChanged(articleId) {
  let deletedItem;
  let index;

  for (let idx in titles.value.artiles) {
    let item = titles.value.artiles[idx]
    if (item.id === articleId) {
      deletedItem = item
      index = idx
      break
    }
  }

  if (deletedItem) {
    if (titles.value.opened && deletedItem.id === titles.value.opened.id) {
      titles.value.opened = undefined
    }
    titles.value.artiles.splice(index, 1)
  }
}

async function autoOpenWhenArticleCreated(articleId) {
  let article = await invoke('find_article_by_id', {
    id: articleId
  })
  await onPushArticle(article)
}

function onArticleTitleChanged({id, title}) {
  for (let artile of titles.value.artiles) {
    if (artile.id === id) {
      artile.title = title
      break
    }
  }
}


let extraPanel = ref({visible: false, type: ''})
let createFolderDialogVisible = ref(false)


let current = ref({id: 1, isDocument: true, type: 'document'})

watch(current, (value, oldValue) => {
  if (value.type !== oldValue.type) {
    extraPanel.value.visible = false
  }
})

watch(extraPanel, value => {
  if (!value.visible) {
    value.type = ''
  }
}, {deep: true})

function onChange(id, type) {
  current.value = {
    id,
    type,
  }
}

function toggleExtraPanel(type) {
  let panel = extraPanel.value
  let visible = panel.visible
  if (visible) {
    if (current.value.type === type.startsWith("document")) {
      visible = panel.type !== type;
    } else {
      visible = !visible
    }
  } else {
    visible = true
  }

  extraPanel.value = {
    type,
    visible
  }
}

onBeforeMount(async _ => {
  initCore().then(async _ => {
    // TODO 初始化成功后，加载数据
    await fetchCurrentDrawer()
  }).catch(_ => {
    // TODO 初始化失败后，显示一个遮罩，告知失败原因，及提示重试
  })
})

async function initCore() {
  try {
    await ipc("init_core")
  } catch (err) {
    ElNotification({
      type: "error",
      title: t("failedToInitializeCore"),
      message: err,
      position: 'bottom-right',
      showClose: false,
    })
  }
}


async function fetchCurrentDrawer() {
  drawer.value = await ipc("find_current_drawer")
}


</script>

<style lang="scss" scoped>
#board {
  padding-bottom: 5px;
}


</style>