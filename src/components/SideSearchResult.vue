<template>
  <div class="side-search-result">
    <side-item-document title="xxx"/>
    <side-item-document title="xxx"/>
    <side-item-document title="xxx"/>
    <side-item-document title="xxx"/>
    <side-item-document title="xxx"/>
    <side-item-document title="xxx"/>
  </div>
</template>

<script setup>
import SideItemDocument from "./SideItemDocument.vue";
</script>

<style lang="scss" scoped>
.side-search-result {
  flex: 1;
  display: flex;
  flex-direction: column;
}
</style>