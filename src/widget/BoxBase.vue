<template>
  <div class="box-base">
    <div class="side">
      <div class="title line-one">
        {{ title }}
      </div>
      <div class="intro" v-if="$slots.intro">
        <slot name="intro"/>
      </div>
    </div>

    <div class="main">
      <slot/>
    </div>
  </div>
</template>

<script setup>

defineProps({
  title: String
})

</script>

<style lang="scss" scoped>
.box-base {
  display: flex;
  padding: 20px 0;

  .side {
    width: 150px;
    display: flex;
    flex-direction: column;
    margin-top: 5px;

    .title {
      color: $color-secondary-text;
      font-size: 14px;
    }

    .intro {
      flex: 1;
      font-size: 12px;
      color: $color-secondary-text;
    }
  }

  .main {
    flex: 1;
    overflow: hidden;
    white-space: nowrap;
  }
}
</style>