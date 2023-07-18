<template>
  <card-base>
    <template #title>
      <el-icon>
        <DataAnalysis/>
      </el-icon>
      {{$t('common.statistics')}}
    </template>

    <div class="items">
      <div class="item">
        <div class="title">
          {{$t('common.folder')}}
        </div>
        <div class="count">
          200
        </div>
      </div>
      <div class="item">
        <div class="title">
          {{$t('common.foldAssociation')}}
        </div>
        <div class="count">
          200
        </div>
      </div>
      <div class="item">
        <div class="title">
          {{$t('common.document')}}
        </div>
        <div class="count">
          200
        </div>
      </div>
      <div class="item">
        <div class="title">
          {{$t('common.documentAssociation')}}
        </div>
        <div class="count">
          200
        </div>
      </div>
    </div>

  </card-base>
</template>

<script setup>
import CardBase from "../widget/CardBase.vue"

</script>

<style lang="scss" scoped>
.items {
  display: flex;
  flex-direction: column;
  justify-content: center;
  height: 100%;

  .item {
    display: flex;
    flex: 1;
    align-items: center;
    padding: 0 10px;
    justify-content: space-between;
    border-bottom: 1px solid $color-base-border;

    .title {
      color: $color-secondary-text;
      font-size: 12px;
    }

    .count {
      color: $color-primary-text;
      font-style: italic;
    }

  }

  .item:last-child {
    border-bottom: none;
  }
}
</style>