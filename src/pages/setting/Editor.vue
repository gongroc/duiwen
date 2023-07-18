<template>
  <box-base :title="$t('setting.editSettings')">

    <el-form label-width="150px">
      <el-form-item :label="$t('setting.storageMethod')">
        <el-radio-group v-model="isAutoSave">
          <el-radio :label="true">{{$t('setting.autoSave')}}</el-radio>
          <el-radio :label="false">{{$t('setting.saveManually')}}</el-radio>
        </el-radio-group>
      </el-form-item>
      <el-form-item :label="$t('setting.openDocumentLimit')">
        <el-input-number size="small" v-model="docMax"/>
      </el-form-item>
      <el-form-item :label="$t('setting.limitExceeded')">
        <el-radio-group v-model="stopOpen">
          <el-radio :label="true">{{$t('setting.preventOpening')}}</el-radio>
          <el-radio :label="false">{{$t('setting.closeOldDocument')}}</el-radio>
        </el-radio-group>
      </el-form-item>
      <el-form-item :label="$t('setting.whenOpen')">
        <el-radio-group v-model="keepEditing">
          <el-radio :label="true">{{$t('setting.revertLastEdit')}}</el-radio>
          <el-radio :label="false">{{$t('setting.rootFolder')}}</el-radio>
        </el-radio-group>
      </el-form-item>

    </el-form>

  </box-base>
</template>

<script setup>
import BoxBase from "../../widget/BoxBase.vue";
import {ref} from "vue";

let isAutoSave = ref(true)
let docMax = ref(10)
let stopOpen = ref(true)
let keepEditing = ref(true)

</script>


<style scoped>

</style>