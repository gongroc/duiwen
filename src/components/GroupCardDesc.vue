<template>
  <card-base>
    <template #title>
      <el-icon>
        <InfoFilled/>
      </el-icon>
      {{$t('group.intro')}}
    </template>

    <template #options>
      <span class="count">
        0/100
      </span>
    </template>

    <textarea :placeholder="$t('common.intro')"/>
  </card-base>
</template>

<script setup>
import CardBase from "../widget/CardBase.vue"
</script>

<style lang="scss" scoped>
textarea {
  outline: none;
  border: none;
  width: 100%;
  height: 100%;
  resize: none;
  padding: 10px;
}

.count{
  color: $color-secondary-text;
}

</style>