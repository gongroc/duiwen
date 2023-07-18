<template>
  <status-bar-item-base :active="active" has-hover>
    <el-icon>
      <Folder/>
    </el-icon>
    {{$t('folder.linkedFolders')}}
  </status-bar-item-base>

</template>

<script setup>
import StatusBarItemBase from "../widget/StatusBarItemBase.vue";

defineProps({
  active: Boolean
})

</script>

<style scoped>

</style>