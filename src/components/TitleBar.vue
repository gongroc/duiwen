<template>
  <div class="title-bar">
    <el-scrollbar class="items-wrapper" ref="scrollbarRef" @wheel="onWheel" @scroll="onScroll" :noresize="true">
      <div class="items" id="title-bar-items">
        <template v-if="!isFavoritesList">
          <title-item :title="drawer.title" type="drawer" :active="opened && opened.type === 'drawer'"
                      @click="onDrawerClicked"
                      v-if="drawer && cfg.showDrawerPanel"/>
          <title-item :title="folder.title" type="folder" :active="opened && opened.type === 'graph'"
                      @click="onGraphClicked"
                      v-if="folder && cfg.showGraph"/>
        </template>

        <title-item v-for="item in artiles" :title="item.title" :key="item.id"
                    :active="opened && opened.type === 'article' && opened.id === item.id"
                    :not-saved="item.notSaved" type="article"
                    @close-all="onCloseAll" @close-other="onCloseOther(item)" @close="onClose(item)"
                    @click="onArticleClicked(item)" @pin="onPin(item)" @unpin="onUnpin(item)" :pin="item.pined"
                    @open-in-new-window="onOpenInNewWindow(item)"/>
      </div>
    </el-scrollbar>
    <div class="options">
      <el-dropdown placement="bottom-end" size="small">
        <el-icon class="btn">
          <MoreFilled/>
        </el-icon>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item icon="files" @click="onSaveAll">
              {{ $t('common.saveAll') }}
            </el-dropdown-item>
            <el-dropdown-item icon="close" @click="onCloseAll">
              {{ $t('common.closeAll') }}
            </el-dropdown-item>
            <template v-if="cfg">
              <el-dropdown-item :icon="cfg.showGraph ? 'select':'_'" @click="onToggleGraph" divided>
                {{ $t('common.displayDiagram') }}
              </el-dropdown-item>
              <el-dropdown-item :icon="cfg.showDrawerPanel ? 'select':'_'"
                                @click="onToggleDrawerPanel">
                {{ $t('common.displayGroupPanel') }}
              </el-dropdown-item>
            </template>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>
  </div>
</template>

<script setup>
import {ref, onMounted, computed, nextTick, watch} from "vue";
import TitleItem from "./TitleItem.vue";
import Sortable from "sortablejs";
import {useStore} from "vuex";
import {Event, Topic} from "../common/event.js";
import {invoke} from "@tauri-apps/api";
import {useRouter} from "vue-router";
import {WebviewWindow} from "@tauri-apps/api/window";

const store = useStore()
const event = new Event()
const router = useRouter()

let props = defineProps({
  opened: Object,
  artiles: {
    type: Array,
    default: _ => []
  },
  drawer: Object,
  folder: Object,
  isFavoritesList: Boolean
})
let emits = defineEmits(['update:artiles', 'update:opened', 'openDrawer', 'openGraph', 'openArticle'])
let cfg = computed(_ => store.state.setting.titles)
let scrollbarRef = ref(null)
let pos = ref(0)

function onWheel(evt) {
  let offset = pos.value
  const step = 30
  offset = evt.wheelDelta < 0 ? offset + step : offset - step
  scrollbarRef.value.setScrollLeft(offset <= 0 ? 0 : offset)
}

function onScroll(val) {
  if (val.scrollLeft) {
    pos.value = val.scrollLeft
  }
}

onMounted(_ => {
  initDragDrop()
  bindEvent()
})

function initDragDrop() {
  Sortable.create(document.getElementById("title-bar-items"), {
    animation: 200,
    draggable: ".title-item",
    chosenClass: "chosen",
    handle: ".title-item-drag",
    forceFallback: true,
  })
}

function onDrawerClicked() {
  emits('openDrawer', props.drawer.id)
}

function onGraphClicked() {
  emits('openGraph', props.folder.id)
}

function onArticleClicked(item) {
  emits('openArticle', item)
}

function onCloseAll() {
  emits("update:artiles", [])
  if (!props.opened || props.opened.type !== 'article') {
    return
  }
  emits('update:opened', undefined)
}

async function onClose(item) {
  let index = props.artiles.indexOf(item)
  props.artiles.splice(index, 1)
  if (props.opened.id === item.id) {
    emits("update:opened", undefined)
  }
}

function onCloseOther(item) {
  emits('update:artiles', [item])
  if (props.opened && item.id !== props.opened.id) {
    emits("update:opened", undefined)
  }
}

function onPin(item) {
  item.pined = true
  sortArticles()
}

function onUnpin(item) {
  item.pined = false
  sortArticles()
}

function sortArticles() {
  props.artiles.sort(a => a.pined ? -1 : 1)
}

function onToggleGraph() {
  store.commit('setting/toggleTitlesShowGraph')
  if (!cfg.value.showGraph && props.opened && props.opened.type === 'graph') {
    emits("update:opened", undefined)
  }
}

function onToggleDrawerPanel() {
  store.commit('setting/toggleTitlesShowDrawerPanel')
  if (!cfg.value.showDrawerPanel && props.opened.type === 'drawer') {
    emits("update:opened", undefined)
  }
}

function bindEvent() {
  event.subscribe(Topic.ARTICLE_CONTENT_CHANGED, id => {
    onChangeArticleNotSaved(id, true)
  })
  event.subscribe(Topic.ARTICLE_CONTENT_SAVED, id => {
    onChangeArticleNotSaved(id, false)
  })

}

function onChangeArticleNotSaved(id, flag) {
  for (let artile of props.artiles) {
    if (artile.id === id) {
      artile.notSaved = flag
    }
  }
}

async function onSaveAll() {
  for (let item of store.state.snapshot.article) {
    await invoke("modify_article_content", {
      id: item.id,
      content: item.content
    })
    await store.commit("snapshot/removeAllArticle")
    props.artiles.find(a => a.id === item.id).notSaved = false
  }
}

async function onOpenInNewWindow(item) {
  let url = router.resolve({
    path: "/independent",
    query: {
      id: item.id
    }
  })
  let newWindow = new WebviewWindow("article-" + item.id, {
    url: url.href,
    decorations: false,
    visible: false,
    width: 1024
  })

  await newWindow.listen("window-closed", async (evt) => {
    let id = evt.payload
    if (!id) {
      return
    }

    let newArticle = await invoke("find_article_by_id", {
      id: id
    })
    store.commit("snapshot/removeIndependent", id)

    let tmp = Object.assign(props.artiles, {});
    if (tmp.findIndex(a => a.id === id) < 0) {
      tmp.push({
        id: id,
        title: newArticle.title,
        notSaved: false,
        pined: false,
      })
    }
    emits("update:artiles", tmp)

    if (!props.opened) {
      emits('update:opened', {
        id: id,
        title: newArticle.title,
        type: "article"
      })
    }

  })

  await newWindow.listen("changed", async evt => {
    let topic = evt.payload.topic
    let payload = evt.payload.payload
    event.publish(topic, payload)
  })

  if (props.opened && item.id === props.opened.id && props.opened.type === 'article') {
    emits('update:opened', undefined)
  }
  store.commit('snapshot/pushIndependent', item.id)

  let tmp = Object.assign(props.artiles, {});
  let index = tmp.findIndex(a => a.id === item.id)
  tmp.splice(index, 1)

  emits('update:artiles', tmp)

}


</script>

<style lang="scss" scoped>

:deep(.el-scrollbar__bar.is-horizontal) {
  height: 4px;
}

.title-bar {
  display: flex;
  justify-items: center;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid $color-base-border;

  .items-wrapper {
    margin-bottom: -1px;
    height: 41px;
  }

  .items {
    display: flex;
    justify-items: center;
    align-items: center;
    border: none;
    font-size: 0;
    flex: 1;

    .chosen {
      opacity: 0.2;
    }

  }

  .options {
    padding: 8px 15px 0 15px;

    .btn {
      cursor: pointer;
      font-size: 16px;
    }
  }

}
</style>