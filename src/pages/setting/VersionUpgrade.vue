<template>
  <box-base :title="$t('setting.versionUpgrade')">
    <el-form label-width="150px">
      <el-form-item :label="$t('setting.current')">
        <div class="item">
          <el-badge is-dot>
            <div class="version">
              v1.0.0
            </div>
          </el-badge>
          <div>
            <el-button>{{ $t('setting.check') }}</el-button>
            <el-button link>{{ $t('setting.updateRecord') }}</el-button>
          </div>
        </div>
      </el-form-item>
    </el-form>
  </box-base>
</template>

<script setup>
import BoxBase from "../../widget/BoxBase.vue";
</script>

<style lang="scss" scoped>
:deep(.el-badge__content.is-dot) {
  top: 2px;
  right: 0;
}

.version{
  margin-top: -5px;
  font-size: 14px;
}

</style>