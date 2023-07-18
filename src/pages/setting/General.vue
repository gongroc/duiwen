<template>
  <box-base :title="$t('setting.generalSettings')">

    <el-form label-width="150px">

      <el-form-item :label="$t('setting.displayLanguage')">
        <el-radio-group @change="onLanguageChanged" v-model="lang" class="line-one">
          <el-radio label="zh-CN">中文</el-radio>
          <el-radio label="en-US">English</el-radio>
        </el-radio-group>
      </el-form-item>

      <el-form-item :label="$t('setting.closeWindow')">
        <el-radio-group v-model="quitProgram" class="line-one">
          <el-radio :label="false">{{ $t('setting.hideWindow') }}</el-radio>
          <el-radio :label="true">{{ $t('setting.exitProgram') }}</el-radio>
        </el-radio-group>
      </el-form-item>

    </el-form>

  </box-base>
</template>

<script setup>
import BoxBase from "../../widget/BoxBase.vue";
import {ref, watch, onBeforeMount} from "vue";
import {Event, Topic} from "../../common/event.js";
import {useStore} from "vuex";

const store = useStore()
let lang = ref('zh-CN')
let quitProgram = ref(false)
const event = new Event()

onBeforeMount(async _ => {
  await init()
})

async function init() {
  lang.value = store.state.setting.language
}

async function onLanguageChanged(value) {
  store.commit("setting/updateLanguage", value)
  event.publish(Topic.LANGUAGE_CHANGED, value)
}


</script>

<style lang="scss" scoped>

</style>