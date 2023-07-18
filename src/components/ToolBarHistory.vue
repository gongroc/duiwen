<template>
  <el-popover v-model:visible="visible" placement="bottom-end" trigger="click" width="240px"
              popper-class="history-popper">
    <template #reference>
      <a href="javascript:" class="history-btn" :class="{'history-active':visible}">
        <el-icon>
          <ArrowDown/>
        </el-icon>
      </a>
    </template>

    <div class="history-content">
      <div class="head">
        <el-input :placeholder="$t('common.searchHistoryRecord')" prefix-icon="search" size="small" v-model="keyword"/>
      </div>
      <el-scrollbar height="400px">
        <div class="body">
          <div class="item" v-for="item in (keyword ? searchResult: records)" :key="item.id" @click="onClick(item)">
            <el-icon>
              <Document v-if="item.target_type === 'article'"/>
              <Folder v-else/>
            </el-icon>
            {{ item.title }}
          </div>
        </div>
      </el-scrollbar>
      <div class="footer">
        <div class="item" @click="onClear">
          <el-icon>
            <Delete/>
          </el-icon>
          {{ $t('common.clearHistoryRecord') }}
        </div>
      </div>

    </div>

  </el-popover>
</template>

<script setup>
import {onMounted, ref, watch} from 'vue'
import {invoke} from "@tauri-apps/api";
import Fuse from "fuse.js";
import {Topic, Event} from "../common/event.js";
import {ElMessageBox} from "element-plus";
import {useI18n} from "vue-i18n";

const event = new Event()
const {t} = useI18n()
let visible = ref(false)
let records = ref([])
let searchResult = ref([])
let keyword = ref()

watch(keyword, async _ => {
  onSearch()
})
watch(visible, async _ => {
  await fetchData()
})

onMounted(async value => {
  if (value) {
    await fetchData()
  }
})

async function fetchData() {
  records.value = await invoke('find_all_record')
}

function onSearch() {
  let fuse = new Fuse(records.value, {
    keys: ['title'],
  })
  let results = []
  for (let el of fuse.search(keyword.value)) {
    results.push(el.item)
  }
  searchResult.value = results
}

async function onClear() {
  await ElMessageBox.confirm(t("common.confirmClearAllRecords") + '?')
  await invoke("remove_all_record")
  records.value = []
  searchResult.value = []
}

async function onClick(item) {
  event.publish(item.target_type === 'article' ? Topic.COMMAND_OPEN_ARTICLE : Topic.COMMAND_OPEN_FOLDER, item.target_id)
}


</script>

<style lang="scss">

.history-popper {
  padding: 0 !important;
}

.history-btn {
  font-size: 12px;
  padding: 9px 12px 9px 12px;
  color: $color-basic-black;
}

.history-btn:hover {
  background: $color-dark-fill;
}

.history-active {
  background: $color-dark-fill;
}

.history-content {
  .head {
    border-bottom: 1px solid $color-base-border;
    padding: 10px;

    .el-input {
      font-size: 12px;
    }
  }

  .body {
    flex: 1;

    .item {
      font-size: 13px;
      padding: 8px 17px;
      display: flex;
      color: $color-basic-black;
      cursor: pointer;

      .el-icon {
        margin-right: 5px;
        margin-top: 2px;
      }
    }

    .item:hover {
      background: $color-base-fill;
      color: $color-brand;
    }

  }

  .footer {
    border-top: 1px solid $color-base-border;
    display: flex;
    flex-direction: row-reverse;

    .item {
      display: flex;
      align-items: center;
      padding: 7px 10px;
      font-size: 12px;
      cursor: pointer;
      color: $color-secondary-text;

      .el-icon {
        margin-right: 3px;
      }
    }

    .item:hover {
      color: $color-brand;
    }

  }

}

</style>