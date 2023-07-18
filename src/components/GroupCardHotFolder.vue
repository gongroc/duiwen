<template>
  <card-base>
    <template #title>
      <el-icon>
        <Folder/>
      </el-icon>
      {{ $t('group.name') }}
    </template>

    <el-scrollbar>
      <div class="items">
        <div class="item" v-for="item in 6" :key="item">
          <div class="prefix" :style="{opacity: (1- item / 100) }">
            {{ item }}
          </div>
          <div class="content">
            <div class="title">
              NodeJS
            </div>
            <el-tooltip :content="$t('group.hotTooltip',{docCount:100,folderCount:10})" placement="top">
              <div class="count">
                <el-icon>
                  <Link/>
                </el-icon>
                20
              </div>
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
  margin: 5px 0;

  .item {
    display: flex;
    align-items: center;
    padding: 5px 10px;


    .prefix {
      width: 15px;
      height: 15px;
      background: rgb(210, 210, 210);
      display: flex;
      justify-content: center;
      align-items: center;
      margin-right: 7px;
      border-radius: 2px;
      font-size: 9px;
    }

    .content {
      display: flex;
      justify-content: space-between;
      align-items: center;
      flex: 1;

      .title {
        color: $color-primary-text;
        flex: 1;
        margin-right: 20px;
        cursor: pointer;
      }

      .count {
        display: flex;
        align-items: center;
        color: $color-placeholder-text;
        font-size: 12px;

        .el-icon {
          margin-right: 1px;
        }

      }

    }

  }

  .item:hover {
    .content {
      .title {
        color: $color-brand;
      }
    }
  }


}
</style>