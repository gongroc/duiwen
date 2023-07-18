<template>
  <div class="markdown-editor">
    <am-toolbar :engine="instance" v-if="instance && !!!simple" :items="toolbarItems" class-name="toolbar"/>
    <div id="editor"></div>
  </div>

</template>

<script setup>
import Engine from "@aomao/engine"
import AmToolbar from '@aomao/toolbar-vue'
import {plugins, cards, toolbarItems, config} from "../common/markdown.js"
import {onBeforeUnmount, onMounted, ref, watch} from "vue"
import {Event, Topic} from "../common/event.js";
import {useI18n} from 'vue-i18n'
import {useStore} from "vuex";

const store = useStore()
const event = new Event()
const {t} = useI18n()

let instance = ref();

let emits = defineEmits(['change', 'saveKeyPressed'])
let props = defineProps({
  simple: Boolean,
  editing: Boolean,
  content: String
})

watch(_ => props.editing, value => {
  instance.value.readonly = !value
})

watch(_ => props.content, value => {
  instance.value.setValue(value)
})

async function initEngine() {
  let engine = new Engine("#editor", {
    config,
    cards,
    plugins,
    readonly: !props.editing,
    placeholder: t("document.text"),
    lang: store.state.setting.language,
  })
  engine.setValue(props.content)
  engine.on("change", _ => {
    emits("change", engine.getValue())
  })
  instance.value = engine
}


onMounted(async _ => {
  await initEngine()
  bindEvent()

})
onBeforeUnmount(_ => {
  unbindEvent()
})

function bindEvent() {
  event.subscribe(Topic.LANGUAGE_CHANGED, onLanguageChanged)
  document.addEventListener('keydown', ev => {
    if (ev.ctrlKey && ev.code === 'KeyS') {
      emits('saveKeyPressed')
    }
  })
}

function unbindEvent() {
  event.unsubscribe(Topic.LANGUAGE_CHANGED, onLanguageChanged)
}

function onLanguageChanged(value) {
  instance.value.lang = value
}

function onSaveKeydown(evt) {
  if (evt.ctrlKey && evt.keyCode === 67) {
    console.log('Ctrl+c');
  }
}


</script>

<style lang="scss" scoped>
.markdown-editor {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  //user-select: text;
  .toolbar {
    z-index: 99;
    border-top: none;
    border-top: 1px solid rgba(0, 0, 0, 0.05);
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
    background: $color-lighter-fill;
  }

  #editor {
    flex: 1;
    //height: 0;
    overflow-y: scroll;
    padding: 20px 30px;
    outline: none;

  }

}
</style>