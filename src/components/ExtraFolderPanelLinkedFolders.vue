<template>
  <div class="extra-folder-panel-linked-folders data-page">
    <el-table :data="items">
      <el-table-column prop="title" label="文件夹名"/>
      <el-table-column prop="label" label="关系名"/>
      <el-table-column prop="linked_at" label="关联时间"/>
      <el-table-column :label="$t('common.operate')" width="240">

        <template #default="scope">
          <el-button size="small" icon="delete" @click="onUnlink(scope.row.relation_id)">{{
              $t('common.delete')
            }}
          </el-button>
          <el-button size="small" icon="edit" @click="onOpenModifyRationDialog(scope.row)">
            {{ $t('folder.modifyRelation') }}
          </el-button>
        </template>

      </el-table-column>
    </el-table>

    <div class="footer">
      <div class="buttons">
        <el-button size="small">
          {{ $t('common.linkFolder') }}
        </el-button>
      </div>
      <div class="counter">
        {{ $t('folder.subset') }} 30，{{ $t('folder.father') }} 10
      </div>
    </div>

    <el-dialog v-model="relationLabelForm.visiable" :show-close="false" style="width: 350px"
               :title="$t('folder.modifyRelationLabel')">
      <el-input v-model="relationLabelForm.title"/>
      <template #footer>
        <el-button size="default" @click="relationLabelForm.visiable = false">{{ $t('common.cancel') }}</el-button>
        <el-button size="default" type="primary" @click="onModifyRationLabel">{{ $t('common.submit') }}</el-button>
      </template>
    </el-dialog>

  </div>


</template>

<script setup>

import {ref, onMounted, watch} from "vue";
import {invoke} from "@tauri-apps/api";
import {ElMessageBox} from "element-plus";
import {useI18n} from "vue-i18n";

const {t} = useI18n()
let items = ref([])
let props = defineProps({
  id: Number
})

let relationLabelForm = ref({
  relationId: undefined,
  title: "",
  visiable: false
})


watch(_ => props.id, async _ => {
  await fetchData()
})

onMounted(async _ => {
  await fetchData()
})

async function fetchData() {
  items.value = await invoke('find_subfolders', {
    folderId: props.id
  })
}

async function onUnlink(relationId) {
  await ElMessageBox.confirm(t("folder.confirmUnlink") + '?')
  await invoke('unlink_folder', {
    relationId: relationId
  })
  await fetchData()
}

function onOpenModifyRationDialog(item) {
  console.log(item)
  relationLabelForm.value.visiable = true
  relationLabelForm.value.relationId = item.relation_id
  relationLabelForm.value.title = item.label
}

async function onModifyRationLabel() {
  await invoke("modify_relation_title", {
    title: relationLabelForm.value.title,
    relationId: relationLabelForm.value.relationId
  })
  relationLabelForm.value.visiable = false
  await fetchData()
}


</script>

<style lang="scss" scoped>
.extra-folder-panel-linked-folders {

}
</style>