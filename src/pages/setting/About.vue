<template>
  <box-base :title="$t('setting.about')">
      <el-form label-width="150px">
        <el-form-item :label="$t('setting.contactDetails')">
          <a href="#">{{$t('setting.email')}}</a>
        </el-form-item>

        <el-form-item :label="$t('setting.thirdPartyLibrary')">
          <a href="#">{{ $t('setting.viewTheListOfSDKs') }}</a>
        </el-form-item>

        <el-form-item :label="$t('setting.author')">
          <a href="#">{{$t('setting.viewAuthor')}}</a>
        </el-form-item>

        <el-form-item label="">
          当前为开发版本，请勿外传
        </el-form-item>

      </el-form>
  </box-base>
</template>

<script setup>
import BoxBase from "../../widget/BoxBase.vue";
</script>

<style scoped>

</style>