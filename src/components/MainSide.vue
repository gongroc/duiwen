<template>
  <div id="main-side">
    <el-scrollbar>
      <div id="result-items" class="wrapper">
        <template v-for="item in (keyword ? searchResult: items)">
          <side-item-document v-if="item.is_article" :title="item.title" :favorite="item.favorite" :intro="item.intro"
                              :id="item.id" @changed="onChanged" :class="{'drag-item': !isToplevel}"
                              @click="onArticleClicked(item)"
                              :active="opened && opened.type === 'article' && opened.id === item.id"/>
          <side-item-folder :title="item.title" @click="onFolderClicked(item)" :intro="item.intro"
                            :id="item.id"
                            @changed="onChanged" :relation-id="item.relation_id"
                            :is-toplevel="isToplevel" :class="{'drag-item':!isToplevel}" v-else/>
        </template>
      </div>
    </el-scrollbar>
  </div>
</template>

<script setup>
import {ref, watch, onMounted, onBeforeMount, computed} from "vue";
import {invoke} from "@tauri-apps/api";
import SideItemFolder from "./SideItemFolder.vue";
import {Topic} from "../common/event.js";
import Fuse from "fuse.js";
import SideItemDocument from "./SideItemDocument.vue";
import {useStore} from "vuex";
import Sortable from "sortablejs";
import {Event} from "../common/event"

const store = useStore()
const event = new Event()

let emits = defineEmits(["update:folder", "articleClicked", "folderClicked"])
let props = defineProps({
  drawer: Object,
  keyword: String,
  folder: Object,
  opened: Object
})

// 配置
let cfg = computed(_ => store.state.setting.side)
// 顶级文件夹或文件夹内容
let items = ref([])
// 搜索结果
let searchResult = ref([])

let isToplevel = computed(_ => {
  return props.folder === undefined
})

onBeforeMount(async _ => {
  await bindEvent()
})

onMounted(async _ => {
  await fetchData()
  await initDragDrop()
})

watch(_ => props.drawer, async _ => {
  await fetchData()
})

watch(_ => props.keyword, async _ => {
  await onSearch()
})

watch(_ => props.folder, async _ => {
  await fetchData()
})

watch(cfg, async _ => {
  await fetchData()
}, {deep: true})

async function onChanged() {
  await fetchData()
  if (props.keyword) {
    await onSearch()
  }
}

async function fetchData() {
  if (isToplevel.value) {
    await fetchTopLevel()
  } else {
    await fetchSubset()
  }
}

async function fetchTopLevel() {
  items.value = []
  items.value = await invoke("find_drawer_top_level_folders", {
    drawerId: props.drawer.id
  })

  // NOTE：拖动排序后，再返回文件夹组，有的时候会有保存数据
  let el = document.getElementById("result-items")
  for (let item of el.getElementsByClassName("drag-item")) {
    el.removeChild(item)
  }
}

async function fetchSubset() {
  let subset_type = "";
  if (!(cfg.value.showArticle === cfg.value.showFolder)) {
    if (cfg.value.showArticle) {
      subset_type = "article"
    } else if (cfg.value.showFolder) {
      subset_type = "folder"
    }
  }
  items.value = await invoke("find_folder_subsets", {
    folderId: props.folder.id,
    sortByIdx: !!cfg.value.sortByDefault,
    subsetType: subset_type
  })
}

async function bindEvent() {
  event.subscribe(Topic.FOLDER_CREATED, async _ => {
    await fetchData()
  })
  event.subscribe(Topic.ARTICLE_CREATED, async _ => {
    await fetchData()
  })
  event.subscribe(Topic.ARTICLE_TITLE_CHANGED, onTitleChanged)
  event.subscribe(Topic.ARTICLE_DELETED, onArticleDeleted)
  event.subscribe(Topic.ARTICLE_INTRO_CHANGED, onArticleIntroChanged)
}

async function onArticleDeleted(id) {
  await fetchData()
}

async function onArticleIntroChanged() {
  await fetchData()
}


async function onSearch() {
  let fuse = new Fuse(items.value, {
    keys: ['title']
  })
  let results = []
  for (let el of fuse.search(props.keyword)) {
    results.push(el.item)
  }
  searchResult.value = results
}

function onChangeFolder(item) {
  emits("update:folder", item)
}

function onArticleClicked(item) {
  emits('articleClicked', item)
}

function onFolderClicked(item) {
  emits("update:folder", item)
  emits('folderClicked', item)
}


let dragFlag = false;

async function initDragDrop() {
  Sortable.create(document.getElementById("result-items"), {
    animation: 150,
    draggable: ".drag-item",
    chosenClass: "chosen",
    forceFallback: true,
    onChoose: evt => {
      if (dragFlag) {
        evt.preventDefault()
      }
      dragFlag = true
    },
    onUnchoose: evt => {
      dragFlag = false
    },
    onEnd: async evt => {
      await exchangeIndex(evt.oldIndex, evt.newIndex)
      dragFlag = false
    },
  })
}

async function exchangeIndex(oldItemIndex, newItemIndex) {
  let oldItem = items.value[oldItemIndex]
  let newItem = items.value[newItemIndex]
  let oldIndex = oldItem.index
  let newIndex = newItem.index
  if (oldIndex === newIndex) {
    return
  }

  await invoke("modify_sequence_index", {
    id: oldItem.sequece_id,
    index: newIndex
  })
  await invoke("modify_sequence_index", {
    id: newItem.sequece_id,
    index: oldIndex
  })
  oldItem.index = newIndex
  newItem.index = oldIndex
}

function onTitleChanged({id, title}) {
  for (let item of items.value) {
    if (item.id === id && item.is_article) {
      item.title = title
      break
    }
  }

  for (let item of searchResult.value) {
    if (item.id === id) {
      item.title = title
      break
    }
  }
}

</script>

<style lang="scss" scoped>
#main-side {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;

  .wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;

    .chosen {
      opacity: 0.1;
    }
  }
}
</style>