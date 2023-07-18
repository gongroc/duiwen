<template>
  <div id="side-group-board">
    <el-popover placement="bottom-end" :width="270" popper-class="popover" :visible="popperVisible">
      <template #reference>
        <div id="group-display" @click="popperVisible = !popperVisible">
          <div id="title" class="line-one" v-if="drawer">
            {{ isFavoritesList ? "收藏夹" : drawer.title }}
          </div>
          <el-icon>
            <CaretTop v-if="popperVisible"/>
            <CaretBottom v-else/>
          </el-icon>
        </div>
      </template>

      <div id="side-group-board-wrapper">
        <div id="head">
          <el-input prefix-icon="search" :placeholder="$t('group.search')" size="small" v-model="search.keyword"/>
        </div>
        <div id="body">

          <div id="group-items">
            <div class="favorites" @click="onFavoritesClick">
              <div class="title">
                <el-icon>
                  <Star/>
                </el-icon>
                {{ $t('side.favorites') }}
              </div>
              <div class="counter">
                30
              </div>
            </div>
            <el-scrollbar height="250px">
              <div class="item " v-for="item in search.items" :key="item.id" @click="onDrawerChange(item)">
                <div class="title line-one">
                  <el-icon>
                    <Folder/>
                  </el-icon>
                  {{ item.title }}
                </div>
                <div class="options">
                  <div class="counter">
                    30
                  </div>
                </div>
              </div>
            </el-scrollbar>
          </div>

        </div>
        <div id="footer">
          <el-button size="small" icon="plus" @click="create.visiable = true">
            {{ $t("group.newGroup") }}
          </el-button>
        </div>

      </div>

    </el-popover>
    <div id="buttons" class="line-one">
      <el-tooltip :content="$t('common.newDocument')" v-if="!isFavoritesList && folder">
        <el-icon class="btn" @click="onCreateArticle">
          <DocumentAdd/>
        </el-icon>
      </el-tooltip>
      <el-tooltip :content="$t('common.linkFolder')" v-if="!isFavoritesList">
        <el-icon class="btn" @click="createFolderVisible = true">
          <FolderAdd/>
        </el-icon>
      </el-tooltip>
      <el-dropdown placement="bottom-end" size="small">
        <el-icon class="btn">
          <Menu/>
        </el-icon>
        <template #dropdown v-if="cfg">
          <el-dropdown-menu>
            <template v-if="!isFavoritesList && !!folder">
              <el-dropdown-item :icon=" cfg.showFolder ?  'select' : 'NULL'"
                                @click="store.commit('setting/toggleSideShowFolder')">
                {{ $t('side.showFolder') }}
              </el-dropdown-item>
              <el-dropdown-item :icon="cfg.showArticle ? 'select' : 'NULL'"
                                @click="store.commit('setting/toggleSideShowArticle')">
                {{ $t('side.showDocument') }}
              </el-dropdown-item>
            </template>
            <el-dropdown-item :icon="cfg.sortByDefault ? 'select' : 'NULL'"
                              @click="store.commit('setting/updateSideSortType',true)"
                              :divided="!isFavoritesList && !!folder">
              {{ $t('side.defaultSort') }}
            </el-dropdown-item>
            <el-dropdown-item :icon="!cfg.sortByDefault ? 'select' : 'NULL'"
                              @click="store.commit('setting/updateSideSortType',false)">
              {{ $t('side.latestEdit') }}
            </el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>

    <el-dialog :title="$t('group.newGroup')" width="400px" v-model="create.visiable">
      <el-input :placeholder="$t('group.name')" v-model="create.title" autofocus/>
      <template #footer>
        <el-button @click="create.visiable = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" :disabled="create.title.length < 2" @click="onSubmitCreate">{{
            $t('common.submit')
          }}
        </el-button>
      </template>
    </el-dialog>

    <create-folder-dialog v-model:visible="createFolderVisible" :drawer="drawer" :folder="folder"/>

  </div>

</template>

<script setup>
import {computed, onMounted, reactive, ref, watch} from 'vue'
import {invoke} from "@tauri-apps/api";
import {ElMessage} from "element-plus";
import {useI18n} from "vue-i18n"
import CreateFolderDialog from "./CreateFolderDialog.vue";
import {Topic} from "../common/event.js";
import {useStore} from "vuex";
import {Event} from "../common/event.js";

const event = new Event()
const store = useStore()
const {t} = useI18n()

let emits = defineEmits(['update:drawer', 'update:isFavoritesList'])
let popperVisible = ref(false)
let createFolderVisible = ref(false)
let cfg = computed(_ => store.state.setting.side)

let props = defineProps({
  drawer: Object,
  folder: Object,
  isFavoritesList: Boolean
})

let search = reactive({
  keyword: "",
  items: []
})

let create = reactive({
  title: "",
  visiable: false
})

onMounted(async _ => {
  await onSearch()
})

watch(_ => search.keyword, async _ => {
  await onSearch()
})

async function onCreateArticle() {
  if (!props.folder) {
    return
  }
  let article_id = await invoke("create_article", {
    title: t("document.untitled"),
    folderId: props.folder.id
  })
  event.publish(Topic.ARTICLE_CREATED, article_id)
}

function onDrawerChange(item) {
  emits("update:drawer", item)
  emits("update:isFavoritesList", false)
  popperVisible.value = false
}

function onFavoritesClick() {
  emits("update:isFavoritesList", true)
  popperVisible.value = false
}

async function onSearch() {
  search.items = await invoke("search_drawer", {keyword: search.keyword})
}

async function onSubmitCreate() {
  try {
    await invoke("create_drawer", {title: create.title})
    create.title = ""
  } catch (err) {
    ElMessage({
      type: "error",
      message: t("failedToCreateGroup")
    })
  }
  await onSearch()
}

</script>

<style lang="scss">

.popover {
  padding: 0 !important;
}

#side-group-board-wrapper {
  display: flex;
  flex-direction: column;

  #head {
    padding: 15px 10px;
  }

  #body {
    flex: 1;
    border-bottom: 1px solid $color-base-border;
    border-top: 1px solid $color-base-border;

    #group-items {

      .favorites {
        display: flex;
        align-items: center;
        justify-items: center;
        padding: 10px 17px;
        border-bottom: 1px solid $color-light-border;
        cursor: pointer;
        color: $color-primary-text;
        font-size: 12px;

        .title {
          display: flex;
          align-items: center;
          flex: 1;
          width: 0;
          margin-right: 10px;
          font-weight: bold;

          .el-icon {
            margin-right: 5px;
          }

        }

        .counter {
          width: 30px;
          text-align: right;
          color: $color-placeholder-text;
        }

      }

      .item {
        display: flex;
        padding: 12px 17px;
        border-bottom: 1px solid $color-light-border;
        cursor: pointer;
        color: $color-primary-text;
        font-size: 12px;

        .title {
          display: flex;
          flex: 1;
          width: 0;
          margin-right: 10px;
          font-weight: bold;

          .el-icon {
            margin-right: 5px;
            margin-top: 2px;
          }

        }

        .options {
          width: 50px;
          text-align: right;

          .counter {
            color: $color-placeholder-text;
          }

          .btn {
            display: none;
            text-align: right;
          }
        }
      }

      .item:hover {
        background: $color-dark-fill;
      }

      .item:last-child {
        border-bottom: none;
      }

    }
  }

  #footer {
    padding: 10px;
    text-align: right;
    background: $color-base-background;
  }

}

#side-group-board {
  padding: 0 10px;
  margin: 18px 0 10px 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
  justify-items: center;

  #group-display {
    font-size: 13px;
    cursor: pointer;
    user-select: none;
    display: flex;
    align-items: center;

    #title {
      margin-right: 5px;
      //font-weight: bold;
    }

  }


  #buttons {
    display: flex;
    align-items: center;
    justify-items: center;
    user-select: none;

    .btn {
      margin-left: 10px;
      cursor: pointer;
      color: $color-primary-text;
      font-size: 15px;
      opacity: 0.7;
      outline: none;
      user-select: none;
    }

    .btn:hover {
      color: $color-brand;
    }

  }

}

</style>