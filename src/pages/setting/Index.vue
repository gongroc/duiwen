<template>
  <main-layout>
    <div class="header" data-tauri-drag-region>
      <div class="title" data-tauri-drag-region>
        {{ $t('setting.settings') }}
      </div>
      <div class="nav">
        <div class="item" @click="jumpHandler('general')" :class="{'active': current === 'general'}">
          {{ $t('setting.generalSettings') }}
        </div>
        <div class="item" @click="jumpHandler('editor')" :class="{'active': current === 'editor'}">
          {{ $t('setting.editSettings') }}
        </div>
        <div class="item" @click="jumpHandler('folder')" :class="{'active': current === 'folder'}">
          {{ $t('setting.folderSettings') }}
        </div>
        <div class="item" @click="jumpHandler('data-storage')" :class="{'active': current === 'data-storage'}">
          {{ $t('setting.dataStorage') }}
        </div>
        <div class="item" @click="jumpHandler('version-upgrade')" :class="{'active': current === 'version-upgrade'}">
          {{ $t('setting.versionUpgrade') }}
        </div>
        <div class="item" @click="jumpHandler('about')" :class="{'active': current === 'about'}">
          {{ $t('setting.about') }}
        </div>
      </div>
    </div>

    <el-scrollbar ref="scrollbarRef" @scroll="onScrollbar">
      <div class="body">
        <general ref="generalRef"/>
        <el-divider/>
        <editor ref="editorRef"/>
        <el-divider/>
        <folder ref="folderRef"/>
        <el-divider/>
        <data-storage ref="dataStorageRef"/>
        <el-divider/>
        <version-upgrade ref="versionUpgradeRef"/>
        <el-divider/>
        <about ref="aboutRef"/>
      </div>
      <div class="space"></div>
    </el-scrollbar>

  </main-layout>
</template>

<script setup>
import MainLayout from "../../widget/MainLayout.vue";
import General from "./General.vue";
import VersionUpgrade from "./VersionUpgrade.vue";
import About from "./About.vue";
import Editor from "./Editor.vue";
import DataStorage from "./DataStorage.vue";
import Folder from "./Folder.vue";
import {ref, onMounted} from 'vue'

let generalRef = ref(null)
let editorRef = ref(null)
let folderRef = ref(null)
let dataStorageRef = ref(null)
let versionUpgradeRef = ref(null)
let aboutRef = ref(null)
let scrollbarRef = ref(null)

let current = ref()

onMounted(_ => {
  initScrollbar()
})

function onScrollbar(val) {
  let scrollTop = val.scrollTop
  if (isArea(generalRef.value.$el, scrollTop)) {
    current.value = 'general'
  } else if (isArea(editorRef.value.$el, scrollTop)) {
    current.value = 'editor'
  } else if (isArea(folderRef.value.$el, scrollTop)) {
    current.value = 'folder'
  } else if (isArea(dataStorageRef.value.$el, scrollTop)) {
    current.value = 'data-storage'
  } else if (isArea(versionUpgradeRef.value.$el, scrollTop)) {
    current.value = 'version-upgrade'
  } else if (isArea(aboutRef.value.$el, scrollTop)) {
    current.value = 'about'
  }
}

function isArea(el, offset) {
  let top = el.offsetTop
  let bottom = el.clientHeight + top
  return offset >= top && offset <= bottom
}


function jumpHandler(to) {
  let offset = 0
  switch (to) {
    case 'general':
      offset = generalRef.value.$el.offsetTop
      break
    case 'editor':
      offset = editorRef.value.$el.offsetTop
      break
    case 'folder':
      offset = folderRef.value.$el.offsetTop
      break
    case 'data-storage':
      offset = dataStorageRef.value.$el.offsetTop
      break
    case 'version-upgrade':
      offset = versionUpgradeRef.value.$el.offsetTop
      break
    case 'about':
      offset = aboutRef.value.$el.offsetTop
      break
  }
  scrollbarRef.value.setScrollTop(offset + 1)
}


function initScrollbar() {
  scrollbarRef.value.setScrollTop(generalRef.value.$el.offsetTop)
  current.value = 'general'
}


</script>

<style lang="scss" scoped>

.header {
  height: 150px;
  border-bottom: 1px solid $color-base-border;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 30px 30px 0 30px;
  margin-top: 4px;

  .title {
    font-size: 24px;
    font-weight: bold;
  }

  .nav {
    display: flex;
    font-size: 14px;

    .item {
      cursor: pointer;
      padding: 6px 0;
      border-bottom: 1px solid rgba(0, 0, 0, 0);
      margin-bottom: -1px;
      color: $color-secondary-text;
      margin-right: 25px;
      overflow: hidden;
      word-break: keep-all;
    }

    .item:last-child {
      margin-right: 0;
    }

    .item:hover {
      color: $color-primary-text;
    }

    .active {
      border-color: $color-brand;
      color: $color-primary-text;
    }

  }

}

.body {
  flex: 1;
  padding: 20px 0 0 30px;

}

.space {
  height: 500px;
}

</style>