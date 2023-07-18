<template>
  <status-bar-item-base :active="active" has-hover>
    <el-icon>
      <InfoFilled/>
    </el-icon>
    {{$t('folder.describe')}}
  </status-bar-item-base>
</template>

<script setup>
import StatusBarItemBase from "../widget/StatusBarItemBase.vue";

defineProps({
  active: Boolean
})

</script>

<style lang="scss" scoped>

</style>