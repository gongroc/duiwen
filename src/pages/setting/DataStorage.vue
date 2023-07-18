<template>
  <box-base :title="$t('setting.dataStorage')">
    <el-form label-width="150px">
      <el-form-item :label="$t('setting.dataDirectory')">
        <div class="data-path-wrapper">
          <div class="path-select">
            <el-input v-model="dataPath" disabled/>
            <el-button>{{$t('common.choose')}}</el-button>
          </div>
          <div class="data-counter">
            <div class="item">
              {{$t('common.diskUsage')}}：100MB
            </div>
            <div class="item">
              <el-icon>
                <Document/>
              </el-icon>
              100MB
            </div>
            <div class="item">
              <el-icon>
                <Picture/>
              </el-icon>
              100MB
            </div>
            <div class="item">
              <el-icon>
                <Files/>
              </el-icon>
              100MB
            </div>
          </div>
        </div>
      </el-form-item>
      <el-form-item label="">
        <el-button icon="download">{{$t("setting.export")}}</el-button>
        <el-button icon="upload">{{$t("setting.import")}}</el-button>
        <el-button icon="delete">{{$t("setting.clear")}}</el-button>
      </el-form-item>
    </el-form>
  </box-base>
</template>

<script setup>
import BoxBase from "../../widget/BoxBase.vue";
import {ref} from "vue";

let dataPath = ref('c://duiwen/data/')

</script>

<style lang="scss" scoped>
.data-path-wrapper {
  width: 400px;

  .path-select {

    .el-input {
      margin-right: 10px;
      flex: 1;
    }
  }

  .data-counter {
    font-size: 12px;
    color: $color-secondary-text;
    display: flex;
    align-items: center;
    justify-items: center;

    .item {
      display: flex;
      align-items: center;
      justify-items: center;

      .el-icon {
        margin-right: 2px;
      }
    }

    .item:after {
      content: "|";
      padding: 0 10px;
      opacity: 0.2;
    }

    .item:last-child:after {
      content: none;
      padding: 0;
    }

  }

}
</style>