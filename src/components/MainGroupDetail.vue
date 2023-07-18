<template>
  <div id="page">
    <div class="head">
      <div class="title">
        <input :placeholder="$t('group.name')"/>
      </div>
      <div class="option">
        <el-dropdown placement="bottom-end">
          <el-icon>
            <MoreFilled/>
          </el-icon>
          <template #dropdown>
            <el-dropdown-item icon="delete">
              {{$t('common.delete')}}
            </el-dropdown-item>
          </template>
        </el-dropdown>
      </div>
    </div>
    <el-scrollbar height="98%">
      <div class="body" id="cards">
        <template v-for="item in cards">
          <group-card-hot-folder class="card" v-if="item === 1"/>
          <group-card-counter class="card" v-if="item === 2"/>
          <group-card-desc class="card" v-if="item === 3"/>
          <group-card-history-record class="card" v-if="item === 4"/>
        </template>
      </div>
    </el-scrollbar>
  </div>
</template>

<script setup>
import GroupCardHotFolder from "./GroupCardHotFolder.vue";
import GroupCardCounter from "./GroupCardCounter.vue";
import GroupCardDesc from "./GroupCardDesc.vue"
import GroupCardHistoryRecord from "./GroupCardHistoryRecord.vue";
import Sortable from "sortablejs";
import {onMounted, ref} from "vue"


let cards = ref([1, 2, 3, 4])

onMounted(_ => {
  initDragDrop()
})

function initDragDrop() {
  console.log('init')
  Sortable.create(document.getElementById("cards"), {
    animation: 200,
    draggable: ".card",
    chosenClass: "chosen",
    handle: ".card-head",
    forceFallback: true,
    onEnd: (evt) => {
      console.log(evt)
      let temp = cards.value
      const item = temp.splice(evt.oldIndex, 1)[0];
      temp.splice(evt.newIndex, 0, item);
      cards.value = temp
    },
  })


}

</script>

<style lang="scss" scoped>
#page {
  flex: 1;
  display: flex;
  flex-direction: column;

  .head {
    display: flex;
    justify-items: center;
    align-items: center;
    border-bottom: 1px solid $color-base-border;

    .title {
      flex: 1;
      width: 0;

      input {
        width: 100%;
        outline: none;
        border: none;
        padding: 15px 25px;
        font-weight: bold;
        font-size: 24px;
        color: $color-primary-text;
      }
    }

    .option {
      user-select: none;
      color: $color-secondary-text;
      display: flex;
      align-items: center;
      justify-items: center;
      margin-left: 15px;
      margin-right: 15px;
      cursor: pointer;
    }
  }

  .body {
    flex: 1;
    height: 0;
    display: flex;
    flex-wrap: wrap;

    .chosen {
      opacity: 0.1;
    }

  }

}
</style>