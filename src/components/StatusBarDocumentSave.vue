<template>
  <el-tooltip :content="$t('common.clickSave')">
    <status-bar-item-base has-hover @click="onClick">
      <el-icon>
        <Document/>
      </el-icon>
      {{ $t('document.saveDocument') }}
    </status-bar-item-base>
  </el-tooltip>
</template>

<script setup>
import StatusBarItemBase from "../widget/StatusBarItemBase.vue";
import {Event, Topic} from "../common/event.js";

const event = new Event()

function onClick() {
  event.publish(Topic.COMMOND_SAVE_ARTICLE)
}

</script>

<style lang="scss" scoped>

</style>