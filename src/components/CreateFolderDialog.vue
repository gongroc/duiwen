<template>
  <el-dialog :model-value="visible" @close="onClose" width="500">

    <template #header>
      <div id="title">
        {{ $t("folder.createFolder") }}
      </div>
    </template>

    <div id="row" class="row">

      <div id="ipt">
        <el-cascader v-model="endFolderId" :options="subsets" v-if="type === 3" style="width: 100%;"/>
        <el-input :placeholder="$t('folder.pleaseEnterNewFolderName')" v-model="title" v-else/>
      </div>

      <div id="select">
        <el-select v-model="type">
          <el-option :value="1" :label="$t('folder.topLevelFolder')"/>
          <el-option :value="2" :label="$t('folder.newSubfolder')" :disabled="!folder"/>
          <el-option :value="3" :label="$t('folder.associateSubfolders')" :disabled="!folder"/>
        </el-select>
      </div>

    </div>

    <el-input :placeholder="$t('folder.linkLabel')" class="row" v-if="type !== 1" v-model="label"/>
    <el-input type="textarea" :rows="5" :placeholder="$t('folder.folderIntro')" v-if="type !== 3" v-model="intro"/>

    <template #footer>
      <div id="footer">
        <div id="info">
          {{ $t("folder.currentGroup") }}: 1000,
          {{ $t("folder.currentFolder") }}: 222
        </div>
        <div id="buttons">
          <el-button @click="onClose">{{ $t('common.cancel') }}</el-button>
          <el-button type="primary" @click="onSubmit" :disabled="!!!canSubmit">{{ $t("common.submit") }}</el-button>
        </div>
      </div>
    </template>

  </el-dialog>
</template>

<script setup>
import {ref, computed, getCurrentInstance, watch} from "vue"
import {invoke} from "@tauri-apps/api";
import {Topic} from "../common/event.js";
import {ElMessage} from "element-plus";
import {useI18n} from "vue-i18n";

const {t} = useI18n();
const {proxy} = getCurrentInstance()

let props = defineProps({
  visible: Boolean,
  drawer: Object,
  folder: Object
})

let emits = defineEmits(['update:visible'])
// 创建类型
let type = ref(1)
// 文件夹名
let title = ref("")
// 文件夹介绍
let intro = ref("")
// 关系名
let label = ref("")
// 终点文件夹
let endFolderId = ref()
// 级联搜索全部节点
let subsets = ref([])

function onClose() {
  emits("update:visible", false)
}

watch(type, async value => {
  if (value === 3) {
    await findAllSubset()
  }
})

let canSubmit = computed(_ => {
  switch (type.value) {
    case 1:
      return title.value
    case 2:
      return title.value && label.value && props.folder
    case 3:
      return endFolder.value && label.value
  }
  return false
})

async function onSubmit() {
  try {
    switch (type.value) {
      case 1:
        await onCreateTopLevel()
        break
      case 2:
        await onCreateSubfolder()
        break
      case 3:
        await onLink()
        break
    }
    proxy.$event.publish(Topic.FOLDER_CREATED)
  } catch (err) {
    console.log(err)
  }
}

async function onCreateTopLevel() {
  await invoke("create_top_level_folder", {
    drawerId: props.drawer.id,
    title: title.value,
    intro: intro.value
  })
  title.value = ""
  intro.value = ""
  ElMessage({
    message: t("common.createSuccess"),
    type: "success"
  });
}

async function onCreateSubfolder() {
  if (!!!props.folder) {
    return
  }
  await invoke("create_subfolder", {
    title: title.value,
    intro: intro.value,
    startId: props.folder.id,
    linkTitle: label.value
  })
  ElMessage({
    message: t("common.createSuccess"),
    type: "success"
  });
}

async function onLink() {
  await invoke("link_folder", {
    startId: props.folder.id,
    endId: endFolderId.value,
    title: label.value
  })
  await findAllSubset()
  endFolderId.value = undefined
  title.value = ''
  ElMessage({
    message: t("common.linkSuccess"),
    type: "success"
  });
}

async function findAllSubset() {
  subsets.value = await invoke('find_folder_cascader', {
    drawerId: props.drawer.id
  })
}


</script>

<style lang="scss" scoped>

#title {
  font-size: 18px;
  font-weight: bold;
}

.row {
  margin-bottom: 15px;
}

#row {
  display: flex;

  #ipt {
    flex: 4;
    margin-right: 10px;

    .el-select {
      width: 100%;
    }

  }

  #select {
    flex: 2;
  }

}

#footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  color: $color-secondary-text;
}


</style>