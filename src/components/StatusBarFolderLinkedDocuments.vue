<template>
  <status-bar-item-base :active="active" has-hover>
    <el-icon>
      <Document/>
    </el-icon>
    {{$t('folder.linkedDocuments')}}
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