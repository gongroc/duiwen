<template>
  <status-bar-item-base :active="active" has-hover>
    <el-icon>
      <Folder/>
    </el-icon>
    {{$t("document.linkedFolder")}}[1]
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