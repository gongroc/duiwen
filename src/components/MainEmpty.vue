<template>
  <div class="page" data-tauri-drag-region>
    <img src="../assets/logo.png" class="logo" alt="" data-tauri-drag-region />
  </div>
</template>

<script setup>

</script>

<style lang="scss" scoped>
.page {
  display: flex;
  flex: 1;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  overflow: hidden;

  .logo {
    width: 200px;
    filter: grayscale(100%);
    opacity: 0.1;
  }

}
</style>