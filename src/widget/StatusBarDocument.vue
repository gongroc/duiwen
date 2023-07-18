<template>
  <div id="status-bar-document">
    <div id="left">
      <slot name="left"/>
    </div>
    <div id="right">
      <slot name="right"/>
    </div>
  </div>
</template>

<script setup>
</script>

<style lang="scss" scoped>
#status-bar-document {
  display: flex;
  justify-items: center;
  align-items: center;
  height: 28px;
  font-size: 13px;
  color: $color-secondary-text;
  justify-content: space-between;
  width: 100%;

  #left {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: row;
  }

  #right {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: row-reverse;
  }
}
</style>