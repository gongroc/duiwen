<template>
  <div id="favorites-side">
    <el-scrollbar>
      <template v-for="item in (keyword ? searchResult: items)">
        <side-item-document :id="item.article_id" :favorite="item.favorite" :intro="item.intro" :title="item.title"
                            @changed="onChanged" @click="onClick(item)"/>
      </template>
    </el-scrollbar>
  </div>
</template>

<script setup>
import {onBeforeMount, onMounted, ref, watch} from "vue"
import {invoke} from "@tauri-apps/api";
import Fuse from "fuse.js";
import SideItemDocument from "./SideItemDocument.vue";
import {Event, Topic} from "../common/event.js";

const event = new Event()

let items = ref([])
let searchResult = ref([])

let emits = defineEmits(["articleClicked"])

let props = defineProps({
  keyword: String,
  opened: Object
})

watch(_ => props.keyword, async val => {
  await fetchData()
})

onBeforeMount(_ => {
  bindEvent()
})

onMounted(async _ => {
  await fetchData()
})

function bindEvent() {
  event.subscribe(Topic.ARTICLE_INTRO_CHANGED, fetchData)
  event.subscribe(Topic.ARTICLE_DELETED, fetchData)
  event.subscribe(Topic.ARTICLE_TITLE_CHANGED, fetchData)
}

async function onChanged() {
  await fetchData()
  if (props.keyword) {
    await onSearch()
  }
}


async function fetchData() {
  if (props.keyword) {
    await onSearch()
  } else {
    await fetchFavorites()
  }
}

async function fetchFavorites() {
  items.value = await invoke("find_all_article_favorites")
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

function onClick(item) {
  emits("articleClicked", {
    id: item.article_id,
    title: item.title
  })
}

</script>

<style lang="scss" scoped>
#favorites-side {
  flex: 1;
  display: flex;
  flex-direction: column;

}
</style>