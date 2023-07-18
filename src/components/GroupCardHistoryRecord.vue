<template>
  <card-base>
    <template #title>

      {{$t('common.historyRecord')}}
    </template>

    <el-scrollbar>
      <div class="items">
        <div class="item" v-for="item in 10" :key="item">
          <div class="title line-one">
            <el-icon >
              <Folder v-if="item % 2"/>
              <Document v-else/>
            </el-icon>
            NodeJS
          </div>
          <div class="ts">
            <el-tooltip content="2022-11-11 11:11" placement="top">
              3天前
            </el-tooltip>
          </div>
        </div>
      </div>
    </el-scrollbar>

  </card-base>
</template>

<script setup>
import CardBase from "../widget/CardBase.vue"
</script>

<style lang="scss" scoped>
.items {
  .item {
    display: flex;
    justify-content: space-between;
    padding: 10px;

    .title {
      color: $color-primary-text;
      display: flex;
      align-items: center;
      flex: 1;
      margin-right: 20px;
      cursor: pointer;
      font-size: 13px;
      overflow: hidden;
      white-space: nowrap;

      .el-icon {
        margin-right: 3px;
      }
    }

    .ts {
      color: $color-secondary-text;
      font-size: 9px;
    }

  }

  .item:hover {
    .title {
      color: $color-brand;
    }
  }

}
</style>