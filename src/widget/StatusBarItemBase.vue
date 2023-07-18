<template>
  <div class="status-bar-item" :class="{'is-hover':hasHover,'active':active}">
    <slot/>
  </div>
</template>

<script setup>

defineProps({
  hasHover: Boolean,
  active: Boolean
})

</script>

<style lang="scss" scoped>

.active {
  background: $color-base-border;
  color: $color-primary-text;
}

.status-bar-item {
  user-select: none;
  padding: 0 10px;
  font-size: 12px;
  height: 28px;
  display: flex;
  justify-items: center;
  align-items: center;

  :deep(.el-icon) {
    margin-right: 3px;
  }

  :deep(.split) {
    width: 10px;
  }

}

.is-hover {
  cursor: pointer;
}

.is-hover:hover {
  color: $color-primary-text;
}

</style>