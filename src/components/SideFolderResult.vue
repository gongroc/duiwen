<template>
  <el-scrollbar>
    <div class="wrapper" id="result-items">
      <side-item-folder @click="onClick(222,'folder')" title="NodeJS" :key="99" class="side-item"/>
      <side-item-document :active="item.id === current.id" @click="onClick(item.id,'document')"
                          v-for="item in items" :key="item.id" :title="item.title" class="side-item"/>
    </div>
  </el-scrollbar>
</template>

<script setup>
import SideItemDocument from "./SideItemDocument.vue";
import Sortable from 'sortablejs';
import {ref, onMounted} from "vue"
import SideItemFolder from "./SideItemFolder.vue";

let props = defineProps({
  items: Array,
  current: Object
})
const emits = defineEmits(['change', 'update:items'])

onMounted(_ => {
  initDragDrop()
})

function onClick(id, flag) {
  emits("change", id, flag)
}

function initDragDrop() {
  Sortable.create(document.getElementById("result-items"), {
    animation: 200,
    draggable: ".side-item",
    chosenClass: "chosen",
    forceFallback: true,
    onEnd: (evt) => {
      let temp = props.items
      const item = temp.splice(evt.oldIndex, 1)[0];
      temp.splice(evt.newIndex, 0, item);
      emits("update:items", temp)
    },
  })
}


</script>

<style lang="scss" scoped>

.wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  .chosen {
    opacity: 0.2;
  }

}

</style>