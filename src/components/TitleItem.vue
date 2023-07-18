<template>
  <div class="title-item" :class="{'active': active}">
    <el-dropdown trigger="contextmenu" size="small" placement="bottom">
      <div class="title title-item-drag" @click="emits('click')">
        <el-badge type="primary" :hidden="!notSaved" is-dot>
          <el-icon class="type">
            <Folder v-if="type === 'folder'"/>
            <Document v-if="type === 'article'"/>
            <MessageBox v-if="type === 'drawer'"/>
          </el-icon>
        </el-badge>
        <div class="content line-one">
          {{ title }}
        </div>
      </div>
      <template #dropdown>
        <el-dropdown-menu>
          <template v-if="type === 'article'">
            <el-dropdown-item @click="emits('close')">{{ $t('common.close') }}</el-dropdown-item>
            <el-dropdown-item @click="emits('closeOther')">{{ $t('common.closeOther') }}</el-dropdown-item>
            <el-dropdown-item @click="emits('closeAll')">{{ $t('common.closeAll') }}</el-dropdown-item>
            <el-dropdown-item @click="emits('openInNewWindow')" :disabled="independenLength === 5">
              {{ $t('common.openInNewWindow') }}
            </el-dropdown-item>
            <el-dropdown-item @click="emits(pin ? 'unpin': 'pin')" divided>
              {{ $t(pin ? 'common.unpin' : 'common.pin') }}
            </el-dropdown-item>
          </template>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
    <el-icon class="close-btn" @click="emits('close')" v-if="type === 'article'">
      <Close/>
    </el-icon>

  </div>
</template>

<script setup>
import {computed} from "vue";
import {useStore} from "vuex";

const props = defineProps({
  title: String,
  active: Boolean,
  isFolder: Boolean,
  type: {
    type: String,
    default: "article"
  },
  notSaved: Boolean,
  pin: Boolean,
})
const emits = defineEmits(['close', 'click', 'closeOther', 'closeAll', 'pin', 'unpin', 'update:notSaved', 'openInNewWindow'])
const store = useStore()
let independenLength = computed(_ => store.state.snapshot.independents.length)


</script>

<style lang="scss" scoped>

.title-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  justify-items: center;
  min-width: 120px;
  width: 120px;
  padding: 0 10px;
  background: $color-base-fill;
  border-right: 1px solid $color-base-border;
  user-select: none;
  cursor: pointer;
  border-bottom: 1px solid $color-base-border;


  .title {
    display: flex;
    align-items: center;
    justify-items: center;
    margin-right: 5px;
    width: 80px;
    height: 40px;

    .type {
      margin-right: 5px;
    }

    .content {
      flex: 1;
      font-size: 12px;
      width: 100%;
    }

  }

  .close-btn {
    display: none;
    font-size: 12px;
    color: $color-placeholder-text;
  }

  .close-btn:hover {
    display: block;
    color: $color-primary-text;
    border-bottom-color: $color-base-border;
  }

}

.title-item:hover {
  .close-btn {
    display: block;
  }
}

.active {
  background: white;
  border-bottom-color: white;

  .close-btn {
    display: block;
  }

}

</style>