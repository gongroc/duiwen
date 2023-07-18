<template>
  <div class="main-extra-panel" :style="{'height': isFull ? '100%': panelHeight +'px'}" :class="{'full':isFull}">
    <resize-bar :min="100" :max="600" v-model:content="panelHeight" :is-vertical="false" css="bar" v-if="!isFull"/>
    <div id="toolbar">
      <el-icon class="btn" @click="isFull = !isFull">
        <ArrowDown v-if="isFull"/>
        <ArrowUp v-else/>
      </el-icon>
      <el-icon class="btn" @click="onClose">
        <Close/>
      </el-icon>
    </div>
    <slot/>
  </div>
</template>

<script setup>
import {ref} from "vue";
import ResizeBar from "./ResizeBar.vue";

let isFull = ref(false)
let panelHeight = ref(350)

let emits = defineEmits(['close'])

function onClose() {
  emits("close")
}

</script>

<style lang="scss" scoped>

.full {
  //top: 32px;
  //bottom: 30px !important;

}

.main-extra-panel {
  width: 100%;
  bottom: 0;
  right: 0;
  background: $color-basic-white;
  display: flex;
  flex-direction: column;
  position: absolute;
  z-index: 99;

  .bar{
    //border-top: 1px solid $color-base-border;
    background: $color-base-border;
  }

  #toolbar {
    position: absolute;
    right: 0;
    top: 0;
    z-index: 99;

    .btn {
      height: 25px;
      width: 25px;
      cursor: pointer;
      font-size: 12px;
      color: $color-secondary-text;
      margin: 3px;
    }

    .btn:hover {
      color: $color-primary-text;
    }

  }


}
</style>