<template>
  <box-base :title="$t('setting.folderSettings')">
    <el-form label-width="150px">
      <el-form-item :label="$t('setting.openFolder')">
        <el-checkbox>
          {{$t('setting.showRelationshipDiagram')}}
        </el-checkbox>
      </el-form-item>

      <el-form-item :label="$t('setting.diagramDisplay')">
        <el-checkbox>{{$t('setting.showDetails')}}</el-checkbox>
      </el-form-item>

      <el-form-item :label="$t('setting.buildRelationships')">
        <el-radio-group v-model="sameWorkspace">
          <el-radio :label="true">{{$t('setting.sameGroup')}}</el-radio>
          <el-radio :label="false">{{$t('setting.multipleGroups')}}</el-radio>
        </el-radio-group>
      </el-form-item>

      <el-form-item>
        <template #label>
          {{$t('setting.relationshipHierarchy')}}
        </template>
        <el-slider v-model="value" range show-stops :max="5" style="width: 300px" :show-tooltip="false" :marks="{
          0: '+3',1: '+2',2: '+1',3: '-1',4: '-2',5: '-3'}"/>
      </el-form-item>

    </el-form>
  </box-base>
</template>

<script setup>
import BoxBase from "../../widget/BoxBase.vue";
import {ref} from 'vue'

let value = ref([2, 3])
let sameWorkspace = ref(false)

</script>

<style lang="scss" scoped>

:deep(.el-slider__marks-text) {
  margin-top: 10px;
  font-size: 12px;
}

</style>