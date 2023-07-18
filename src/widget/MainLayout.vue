<template>
  <div id="main-layout">
    <div id="side-bar" v-if="!hideSideBar" data-tauri-drag-region>
      <div id="nav">
        <router-link to="/home">
          <el-icon>
            <Files/>
          </el-icon>
        </router-link>
      </div>
      <div>
        <router-link to="/setting">
          <el-icon>
            <Setting/>
          </el-icon>
        </router-link>
      </div>
    </div>
    <div id="content">
      <div id="wrapper">
        <template v-if="$slots['menu']">
          <div id="menu" v-bind:style="{ width: menuSideWidth + 'px' }">
            <slot name="menu"/>
          </div>
          <resize-bar v-model:content="menuSideWidth" :min="100" :max="600" :is-vertical="true"/>
        </template>
        <div id="main">
          <div id="tool-bar">
            <slot name="tool-bar"/>
            <a href="javascript:" @click="onWindowMinus">
              <el-icon>
                <Minus/>
              </el-icon>
            </a>
            <a href="javascript:" @click="onWindowFullscreen">
              <el-icon>
                <FullScreen/>
              </el-icon>
            </a>
            <a href="javascript:" @click="onWindowClose">
              <el-icon>
                <Close/>
              </el-icon>
            </a>

          </div>
          <slot/>
          <template v-if="$slots.extra && extraVisible">
            <main-extra-panel @close="onExtraClose">
              <slot name="extra"/>
            </main-extra-panel>
          </template>
        </div>
      </div>
      <div id="status-bar" v-if="$slots.status">
        <slot name="status"/>
      </div>
    </div>
    <slot name="extra-layer"/>
  </div>
</template>

<script setup>
import ResizeBar from "../widget/ResizeBar.vue"
import {appWindow} from "@tauri-apps/api/window"
import MainExtraPanel from './MainExtraPanel.vue'
import {ref} from "vue";

let menuSideWidth = ref(200)
defineProps({
  extraVisible: Boolean,
  hideSideBar: Boolean
})


let emits = defineEmits(['update:extraVisible'])

function onExtraClose() {
  emits('update:extraVisible', false)
}


async function onWindowMinus() {
  await appWindow.minimize();
}

async function onWindowFullscreen() {
  if (await appWindow.isMaximized()) {
    await appWindow.unmaximize();
  } else {
    await appWindow.maximize();
  }
}

async function onWindowClose() {
  await appWindow.close();
}

</script>

<style lang="scss" scoped>
#main-layout {
  display: flex;
  height: 100vh;
  min-height: 100%;
  padding: 0;
  position: relative;

  #side-bar {
    border-right: 1px solid $color-base-border;
    width: 45px;
    min-width: 45px;
    text-align: center;
    padding: 20px 4px 10px 0;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    font-size: 16px;
    margin-left: 5px;

    a {
      color: $color-primary-text;
      font-size: 18px;
      opacity: 0.5;
    }

    a:hover {
      opacity: 1;
    }

    .router-link-active {
      opacity: 1;
    }

  }

  #content {
    flex: 1;
    height: 100vh;
    display: flex;
    flex-direction: column;

    #wrapper {
      flex: 1;
      height: 0;
      display: flex;

      #menu {
        border-right: 1px solid $color-base-border;
        display: flex;
        flex-direction: column;
      }

      #main {
        flex: 1;
        width: 0;
        display: flex;
        flex-direction: column;
        position: relative;

        #tool-bar {
          position: absolute;
          top: 0;
          right: 0;

          a {
            font-size: 12px;
            padding: 9px 12px 7.5px 12px;
            display: inline-block;
            color: $color-basic-black;
          }

          a:hover {
            background: $color-dark-fill;
          }

        }

      }
    }

    #status-bar {
      display: flex;
      justify-items: center;
      align-items: center;
      border-top: 1px solid $color-base-border;
      background: $color-lighter-fill;
    }

  }


}
</style>