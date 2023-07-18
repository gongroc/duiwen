<template>
  <div class="linked-nodes">
    <el-table :data="items" size="large">
      <el-table-column prop="name" :label="$t('common.folder')" width="100" fixed="left">
        <template #default="scope">
          <a href="javascript:" class="link-item" @click="onJumpFolder(scope.row)">
            <el-icon>
              <Folder/>
            </el-icon>
            {{ scope.row.title }}
          </a>
        </template>
      </el-table-column>
      <el-table-column prop="remark" :label="$t('common.remark')"/>
      <el-table-column prop="linked_at" :label="$t('common.linkedTime')" width="200"/>
      <el-table-column :label="$t('common.operate')" width="200" fixed="right">
        <template #default="scope">
          <el-button @click="onShowRemarkDialog(scope.row)" size="small">{{ $t('common.editRemark') }}</el-button>
          <el-button size="small" @click="onUnlink(scope.row)" :disabled="items.length <= 1">{{
              $t("common.delete")
            }}
          </el-button>
        </template>
      </el-table-column>
    </el-table>

    <div class="buttons">
      <el-button icon="folderAdd" size="small" @click="linkForm.visible = true">{{
          $t('common.linkFolder')
        }}
      </el-button>
    </div>

    <el-dialog v-model="remarkForm.dialog" :title="$t('common.editRemark')" :show-close="false">
      <el-input type="textarea" :rows="4" v-model="remarkForm.content"/>
      <template #footer>
        <el-button size="default" @click="remarkForm.dialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button size="default" type="primary" @click="onSubmitModifyRemark">{{ $t('common.submit') }}
        </el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="linkForm.visible" :show-close="false" style="width: 450px" :title="$t('common.linkFolder')">
      <search-folder-form v-model:ids="linkForm.folderIds"/>
      <template #footer>
        <el-button size="default" @click="linkForm.visible = false">{{ $t('common.cancel') }}</el-button>
        <el-button size="default" type="primary" @click="onLink">{{ $t('common.submit') }}
        </el-button>
      </template>
    </el-dialog>

  </div>


</template>

<script setup>
import {onBeforeMount, ref, watch} from "vue"
import {invoke} from "@tauri-apps/api";
import {ElMessageBox} from "element-plus";
import {useI18n} from "vue-i18n";
import SearchFolderForm from "./SearchFolderForm.vue";
import {Event, Topic} from "../common/event.js";

const event = new Event()
const {t} = useI18n()
let items = ref([])
let remarkForm = ref({
  dialog: false,
  content: '',
  id: undefined
})

let linkForm = ref({
  visible: false,
  folderIds: []
})

let props = defineProps({
  id: Number
})

watch(_ => props.id, async _ => {
  await fetchData()
})

onBeforeMount(async _ => {
  await fetchData()
})

function onShowRemarkDialog(item) {
  remarkForm.value = {
    id: item.id,
    content: item.remark,
    dialog: true
  }
}

async function onSubmitModifyRemark() {
  await invoke("modify_article_folder_remark", {
    id: remarkForm.value.id,
    remark: remarkForm.value.content
  })
  await fetchData()
  remarkForm.value.dialog = false
}

async function onUnlink(item) {
  await ElMessageBox.confirm(t("document.confirmUnlinkFolder") + '?', item.title)
  await invoke("article_unlink_folder", {
    folderId: item.folder_id,
    articleId: item.article_id
  })
  await fetchData()
}

async function fetchData() {
  items.value = await invoke('find_article_folders', {
    articleId: props.id
  })
}

async function onLink() {
  for (let folderId of linkForm.value.folderIds) {
    let id = Number.parseInt(folderId)
    await invoke("article_link_folder", {
      articleId: props.id,
      folderId: id
    })
  }
  await fetchData()
}

function onJumpFolder(item) {
  event.publish(Topic.COMMAND_OPEN_FOLDER, item.folder_id)
}


</script>

<style lang="scss" scoped>

.linked-nodes {
  display: flex;
  flex-direction: column;
  //overflow: hidden;

  .el-table {
    height: 0;
    flex: 1;
  }


  .buttons {
    padding: 0 15px;
    height: 50px;
    display: flex;
    align-items: center;
    justify-items: center;
  }

  .link-item {
    display: flex;
    align-items: center;
    justify-items: center;
    color: $color-primary-text;

    .el-icon {
      margin-right: 3px;
    }

  }

  .link-item:hover {
    color: $color-brand;
  }
}


</style>