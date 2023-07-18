<template>
  <div class="simple-layout">
    <div class="head" data-tauri-drag-region>
      <div class="logo" data-tauri-drag-region>
        <img src="../assets/logo.png" alt="">
      </div>
      <div class="topbar">
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
    </div>
    <div class="body">
      <slot/>
    </div>
  </div>
</template>

<script setup>
import {appWindow} from "@tauri-apps/api/window";
let emits = defineEmits(['beforeClose'])

async function onWindowMinus() {
  await appWindow.minimize()
}

async function onWindowFullscreen() {
  if (await appWindow.isMaximized()) {
    await appWindow.unmaximize()
  } else {
    await appWindow.maximize()
  }
}

async function onWindowClose() {
  await emits('beforeClose')
  await appWindow.close()
}

</script>

<style lang="scss" scoped>
.simple-layout {
  position: relative;
  height: 100vh;
  width: 100%;
  display: flex;
  flex-direction: column;

  .head {
    display: flex;
    justify-content: space-between;
    border-bottom: 1px solid $color-base-border;
    align-items: center;

    .logo {
      margin-left: 30px;

      img {
        height: 12px;
        filter: grayscale(1);
        opacity: 0.5;
      }
    }

    .topbar {

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

  .body {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

}
</style>