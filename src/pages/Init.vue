<template>

  <div class="main">
    <div class="header">
      <div class="toolbar" data-tauri-drag-region>
        <el-icon @click="onClose">
          <Close/>
        </el-icon>
      </div>

      <div class="board" data-tauri-drag-region>
        <div class="info">
          <div class="title">
            {{ $t('init.duiwenNotes') }}
          </div>
          <div class="subtitle">
            {{ $t('init.subtitle') }}
          </div>
        </div>
        <div class="status">
          <el-icon v-if="status.loading">
            <Loading/>
          </el-icon>
          {{ status.text }}
        </div>
      </div>

    </div>
    <div class="body">
      <el-tooltip :content="$t('init.clickToSelectDirectory')">
        <el-input :placeholder="$t('init.workspaceDirectory')" :model-value="dir" @click="onChooseDir"/>
      </el-tooltip>
      <el-button @click="onSubmit">{{ $t('common.submit') }}</el-button>
    </div>

  </div>

</template>

<script setup>
import {appWindow, LogicalSize} from '@tauri-apps/api/window';
import {onBeforeMount, ref} from 'vue'
import {useRouter} from 'vue-router'
import {ipc} from "../common/utils.js";
import {open} from "@tauri-apps/api/dialog"
import {documentDir, sep} from "@tauri-apps/api/path"
import {useI18n} from "vue-i18n";

const {t} = useI18n()
let dir = ref()
const workspace_folder_name = "duiwen"
let status = ref({
  loading: false,
  text: ''
})
let router = useRouter()

onBeforeMount(async _ => {
  if (await checkLoggedWorkspace()) {
    await gotoMainWindow()
  } else {
    await setWindow()
  }
})

async function onSubmit() {
  await setWorkspace();
  await gotoMainWindow()
}

function onClose() {
  appWindow.close()
}

async function setWindow() {
  await appWindow.setSize(new LogicalSize(800, 300))
  await appWindow.setResizable(false)
  await appWindow.center()
  await appWindow.show()
}

async function gotoMainWindow() {
  await appWindow.hide()
  await appWindow.setSize(new LogicalSize(1280, 748))
  await router.push('/home')
  await appWindow.setResizable(true)
  await appWindow.center()
  await appWindow.show()
}

async function onChooseDir() {
  try {
    let path = await open({
      multiple: false,
      title: t("init.chooseWorkspaceDirectory"),
      directory: true
    })
    if (!path || path === dir.value) {
      return
    }
    dir.value = path + sep + workspace_folder_name
  } catch (err) {
    console.log(err)
  }
}

async function checkLoggedWorkspace() {
  try {
    let workspace = await ipc("get_workspace")
    if (workspace) {
      return true
    } else {
      let docDir = await documentDir()
      dir.value = docDir + workspace_folder_name
    }
  } catch (err) {
    console.log(err)
  }
  return false
}

async function setWorkspace() {
  try {
    await ipc("set_workspace", {
      value: dir.value
    })
  } catch (err) {
    console.log(err)
    status.value.text = err
  }
}


</script>

<style lang="scss" scoped>
.main {
  display: flex;
  flex-direction: column;
  height: 100vh;

  .header {
    display: flex;
    flex-direction: column;
    height: 350px;
    background: $color-brand;

    .toolbar {
      text-align: right;

      .el-icon {
        cursor: pointer;
        width: 35px;
        height: 35px;
        color: $color-basic-white;
      }

      .el-icon:hover {
        background: rgba(0, 0, 0, 0.2);
      }
    }

    .board {
      display: flex;
      flex: 1;
      padding: 30px;
      justify-content: space-between;
      align-items: flex-end;

      .info {
        display: flex;
        flex-direction: column;

        .title {
          color: $color-basic-white;
          font-size: 36px;
          font-weight: bold;
          margin-bottom: 25px;
        }

        .subtitle {
          color: $color-basic-white;
        }

      }

      .status {
        color: $color-basic-white;
        opacity: 0.6;
        display: flex;
        align-items: center;

        .el-icon {
          margin-right: 5px;
        }
      }

    }

  }


  .body {
    padding: 30px;
    display: flex;
    flex: 1;

    :deep(.el-input) {
      margin-right: 10px;

      input {
        cursor: pointer;
        caret-color: transparent;
      }
    }
  }


}
</style>