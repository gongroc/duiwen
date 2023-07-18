<template>
  <div class="side-item-base" :class="{'active':active}">
    <div class="head">
      <div class="title ">

        <template v-if="$slots.icon">
          <slot name="icon"/>
        </template>

        <div class="content line-one" @click="emits('titleClick')">
          <slot name="title"/>
        </div>
      </div>

      <el-dropdown placement="bottom-end" v-if="$slots.items" size="small">
        <el-icon class="icon">
          <MoreFilled/>
        </el-icon>
        <template #dropdown>
          <el-dropdown-menu>
            <slot name="items"/>
          </el-dropdown-menu>
        </template>
      </el-dropdown>

    </div>
    <div class="body" v-if="$slots.info" @click="emits('infoClick')">
      <slot name="info"/>
    </div>

  </div>
</template>

<script setup>
const props = defineProps({
  active: {
    type: Boolean,
    default: _ => {
      return false
    }
  }
})

const emits = defineEmits(['titleClick', 'infoClick'])

</script>

<style lang="scss" scoped>

.active {
  background: $color-page-background;
}

.side-item-base {
  padding: 5px 0;

  .head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    justify-items: center;
    padding: 5px 10px;
    user-select: none;

    .icon {
      color: $color-placeholder-text;
      margin-left: 5px;
      font-size: 12px;
    }

    .title {
      display: flex;
      align-items: center;
      color: $color-primary-text;
      cursor: pointer;

      .content {
        margin-left: 7px;
        text-decoration: none;
        flex: 1;
        font-weight: bold;
      }

    }

  }

  .body {
    margin-left: 30px;
    font-size: 12px;
    opacity: 0.5;
    user-select: none;
    color: $color-primary-text;
    cursor: pointer;

  }

}
</style>