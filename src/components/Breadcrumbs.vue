<template>
  <div class="breadcrumbs" data-tauri-drag-region>
    <div class="items line-one" data-tauri-drag-region>
      <template v-if="!isFavoritesList">
        <div class="item line-one" @click="gotoDrawer">{{ drawer.title }}</div>
        <template v-for="item in items">
          <el-icon class="split">
            <ArrowRight/>
          </el-icon>
          <div class="item line-one" :class="{'active':isActive(item)}" @click="gotoFolder(item)">{{ item.title }}</div>
        </template>
      </template>
    </div>
  </div>
</template>

<script setup>
import {ArrowRight} from "@element-plus/icons-vue"
import {watch, reactive} from "vue";

let items = reactive([])

let emits = defineEmits(['update:folder'])
let props = defineProps({
  folder: Object,
  drawer: Object,
  isFavoritesList: Boolean
})

watch(_ => props.folder, _ => {
  push()
})

watch(_ => props.isFavoritesList, _ => {
  items = []
})

function isActive(item) {
  return props.folder && props.folder.id === item.id;
}

function push() {
  if (props.folder) {
    if (items.length > 0) {
      let last = items[items.length - 1]
      if (last.id !== props.folder.id) {
        items.push(props.folder)
      }
    } else {
      items.push(props.folder)
    }
  } else {
    items = []
  }
}

function gotoDrawer() {
  items = []
  emits("update:folder", undefined)
}

function gotoFolder(item) {
  if (item.id === props.folder.id) {
    return
  }
  let index = items.indexOf(item)
  emits("update:folder", items[index])
  items.splice(index + 1, items.length)
}

</script>

<style lang="scss" scoped>
.breadcrumbs {
  user-select: none;
  border-bottom: 1px solid $color-base-border;
  margin-top: 5px;
  padding-left: 8px;

  .title {
    color: $color-placeholder-text;
  }

  .items {
    display: flex;
    align-items: center;
    justify-items: center;
    height: 30px;
    //max-width: 40%;
    font-size: 12px !important;

    .item {
      cursor: pointer;
      color: $color-placeholder-text;
    }

    .item:hover {
      color: $color-brand;
    }

    .active {
      color: $color-basic-black;
    }

    .active:hover {
      color: $color-basic-black;
      cursor: default;
    }

    .split {
      //padding: 0 5px;
      color: $color-dark-border;
      font-size: 10px;
      margin: 0 3px;
    }
  }


}
</style>