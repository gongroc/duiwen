<template>
  <div id="side-search">
    <el-input :placeholder="$t('side.search')" size="small" prefix-icon="search"
              v-model="value" @clear="value = ''" :clearable="true"/>
  </div>
</template>

<script setup>
import {ref, watch} from 'vue'

let value = ref()

let emits = defineEmits(["update:keyword"])
defineProps({
  keyword: String
})

watch(value, _ => {
  emits("update:keyword", value.value)
})

</script>

<style lang="scss" scoped>
#side-search {
  padding: 0 10px;
  user-select: none;
}
</style>