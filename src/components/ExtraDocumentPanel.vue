<template>
  <div class="extra-document-panel">
    <div class="tables">
      <div class="tab" :class="{'active': submenu === 'basic-info'}" @click="emits('update:submenu', 'basic-info')">
        {{ $t('document.basicInformation') }}
      </div>
      <div class="tab" :class="{'active': submenu === 'linked-folder'}"
           @click="emits('update:submenu', 'linked-folder')">
        {{ $t("document.linkedFolder") }}
      </div>
    </div>
    <extra-document-panel-basic-info :id="id" v-if="submenu === 'basic-info'" class="content"/>
    <extra-document-panel-linked-nodes :id="id" v-else class="content"/>
  </div>
</template>

<script setup>
import ExtraDocumentPanelBasicInfo from "./ExtraDocumentPanelBasicInfo.vue";
import ExtraDocumentPanelLinkedNodes from "./ExtraDocumentPanelLinkedFolder.vue"

defineProps({
  submenu: String,
  id: Number
})

let emits = defineEmits(['update:submenu'])

</script>

<style lang="scss" scoped>
.extra-document-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  user-select: none;

  .tables {
    display: flex;
    justify-items: center;
    padding: 10px 10px;
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

  .content {
    flex: 1;
    height: 0;
  }

}
</style>