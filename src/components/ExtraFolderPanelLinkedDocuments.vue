<template>
  <div class="extra-folder-panel-linked-documents data-page">
    <el-table :data="tableData" stripe>
      <el-table-column type="selection" width="30" fixed="left"/>
      <el-table-column prop="title" width="200" :label="$t('document.title')"/>
      <el-table-column prop="remark" :label="$t('common.introduction')"/>
      <el-table-column width="300" :label="$t('document.linkedFolder')">
        <template #default="scope">
          <el-tag v-for="item in scope.row.folders" :key="item" style="margin-right: 5px" size="small">
            {{ item }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column width="200" :label="$t('common.operate')">
        <el-button size="small">
          {{$t('common.unlink')}}
        </el-button>
      </el-table-column>
    </el-table>

    <div class="footer">
      <div class="buttons">
        <el-button size="small">
          {{$t('folder.unlinkSelected')}}
        </el-button>
      </div>

      <div class="counter">
        {{$t('folder.linkedDocumentCounter', {count:100})}}
      </div>

    </div>

  </div>
</template>

<script setup>
import {ref} from "vue";

let tableData = ref([
  {title: '文章标题假字请勿阅读', remark: '', folders: ['node', 'java', '编程语言']},
  {title: '文章标题假字请勿阅读', remark: '', folders: ['node', 'java', '编程语言']},
  {title: '文章标题假字请勿阅读', remark: '', folders: ['node', 'java', '编程语言']},
  {title: '文章标题假字请勿阅读', remark: '', folders: ['node', 'java', '编程语言']},
  {title: '文章标题假字请勿阅读', remark: '', folders: ['node', 'java', '编程语言']},
  {title: '文章标题假字请勿阅读', remark: '', folders: ['node', 'java', '编程语言']},
])

function handleSelectionChange() {

}


</script>

<style lang="scss" scoped>
.extra-folder-panel-linked-documents {

}
</style>