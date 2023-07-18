<template>
  <div class="kv-item">
    <div class="key">
      <slot name="k"/>
    </div>
    <div class="value">
      <slot name="v"/>
    </div>
  </div>
</template>

<script setup>
</script>

<style lang="scss" scoped>
.kv-item {
  display: flex;
  margin-bottom: 20px;

  .key {
    width: 100px;
    text-align: right;
    margin-right: 25px;
    color: $color-secondary-text;
    font-size: 13px;
  }

  .value {
    flex: 1;
  }
}
</style>