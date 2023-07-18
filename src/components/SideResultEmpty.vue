<template>
  <div class="side-result-empty">

    <div class="desc">
      {{ $t('empty.sideIntro') }}
    </div>
    <el-button type="primary" icon="document">
      {{ $t('document.createNewDocument') }}
    </el-button>
    <el-divider>
      <div class="divider-info">
        {{ $t('empty.or') }}
      </div>
    </el-divider>
    <el-button type="primary" icon="folder">
      {{ $t("common.linkFolder") }}
    </el-button>

  </div>
</template>

<script setup>

</script>

<style lang="scss" scoped>
.side-result-empty {
  display: flex;
  flex-direction: column;
  flex: 1;
  padding: 10px;
  overflow: hidden;

  .desc {
    margin-bottom: 15px;
  }

  .divider-info {
    font-size: 12px;
    color: $color-secondary-text;
  }

}

</style>