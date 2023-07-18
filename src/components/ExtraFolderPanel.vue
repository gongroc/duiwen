<template>
  <div class="extra-folder-panel">
    <div class="tabs">
      <div class="tab" :class="{'active':submenu === 'detail'}" @click="emits('update:submenu','detail')">
        {{ $t("folder.describe") }}
      </div>
      <div class="tab" :class="{'active': submenu === 'linked-folder'}"
           @click="emits('update:submenu','linked-folder')">
        {{ $t("folder.linkedFolders") }}
      </div>
      <div class="tab" :class="{'active': submenu === 'linked-article'}"
           @click="emits('update:submenu','linked-article')">
        {{ $t("folder.linkedDocuments") }}
      </div>
    </div>
    <extra-folder-panel-info v-if="submenu === 'detail'" :id="id"/>
    <extra-folder-panel-linked-folders v-if="submenu === 'linked-folder'" :id="id"/>
    <extra-folder-panel-linked-documents v-if="submenu === 'linked-article'"/>
  </div>
</template>

<script setup>
import ExtraFolderPanelInfo from "./ExtraFolderPanelInfo.vue";
import ExtraFolderPanelLinkedFolders from "./ExtraFolderPanelLinkedFolders.vue";
import ExtraFolderPanelLinkedDocuments from "./ExtraFolderPanelLinkedDocuments.vue";

defineProps({
  panel: {
    type: String,
    default: 'folder-detail'
  },
  submenu: String,
  id: Number
})
let emits = defineEmits(['update:submenu'])

</script>

<style lang="scss" scoped>

.extra-folder-panel {
  flex: 1;
  height: 0;
  overflow-y: hidden;
  display: flex;
  flex-direction: column;

  .tabs {
    display: flex;
    justify-items: center;
    padding: 10px 10px;
    height: 45px;
    user-select: none;

    .tab {
      padding: 3px 10px;
      background: $color-lighter-fill;
      font-size: 10px;
      margin-right: 10px;
      cursor: pointer;
      border-radius: 3px;
      color: $color-secondary-text;
    }

    .tab:last-child {
      margin-right: 0;
    }

    .active {
      background: $color-base-fill;
      color: $color-basic-black;
    }

  }


  :deep(.data-page) {
    display: flex;
    flex-direction: column;
    flex: 1;
    height: 0;

    .el-table {
      flex: 1;
      height: 0;
    }

    .footer {
      padding: 0 15px;
      height: 50px;
      display: flex;
      align-items: center;
      justify-items: center;
      justify-content: space-between;

      .buttons {
        display: flex;
      }

      .counter {
        color: $color-secondary-text;
        font-size: 12px;
      }

    }

  }


}

</style>