<template>
  <div class="card">
    <div class="card-head">
      <div class="title">
        <slot name="title"/>
      </div>
      <div class="options">
        <slot name="options"/>
      </div>
    </div>
    <div class="body">
      <slot/>
    </div>
  </div>
</template>

<script>
</script>

<style lang="scss" scoped>
.card {
  display: flex;
  flex-direction: column;
  border: 1px solid $color-base-border;
  border-radius: 4px;
  position: relative;
  width: calc(100% / 3.26);
  box-sizing: border-box;
  margin-left: 20px;
  margin-top: 20px;
  height: 240px;

  .card-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid $color-base-border;
    padding: 5px 10px;
    background: $color-base-fill;
    font-size: 12px;
    cursor: move;
    .title {
      font-weight: bold;
      display: flex;
      justify-content: center;
      align-items: center;

      :deep(.el-icon) {
        margin-right: 5px;
      }
    }
  }

  .body {
    flex: 1;
    height: 0;
  }
}

</style>