<template>
  <el-tooltip :content="$t('side.returnToParentFolder')">
    <div id="side-back">
      <el-icon>
        <ArrowLeft/>
      </el-icon>
      编程语言
    </div>
  </el-tooltip>
</template>

<script setup>
</script>

<style lang="scss" scoped>

#side-back {
  display: flex;
  height: 30px;
  align-items: center;
  font-size: 12px;
  padding: 0 5px;
  border-top: 1px solid $color-base-border;
  color: $color-secondary-text;
  cursor: pointer;
  user-select: none;

  .el-icon {
    margin-right: 2px;
  }
}

#side-back:hover {
  background: $color-page-background;
  color: $color-primary-text;
}

</style>