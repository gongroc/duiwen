<template>
  <div ref="resizeBar" :class="['resize-bar',css]">
  </div>
</template>

<script setup>
import {onMounted, ref} from "vue";

const emits = defineEmits(["update:content"])

const props = defineProps({
  isVertical: {
    type: Boolean,
    required: true,
  },
  max: {
    type: Number,
    required: true
  },
  min: {
    type: Number,
    required: true
  },
  content: {
    type: Number,
    required: true
  },
  css: String,
})

const resizeBar = ref()


onMounted(() => {
  renderStyle()
  onDrag()
})

function onDrag() {
  let bar = resizeBar.value
  bar.onmousedown = e => {
    let offset = props.isVertical ? e.clientX : e.clientY
    let current = props.content
    document.onmousemove = e => {
      let diff = props.isVertical ? (current + (e.clientX - offset)) : (current - (e.clientY - offset))
      if (diff >= props.min && diff <= props.max) {
        emits("update:content", diff)
      }
    }
    document.onmouseup = _ => {
      document.onmousemove = null
      document.onmouseup = null
    }
  }
}

function renderStyle() {
  let bar = resizeBar.value
  if (props.isVertical) {
    bar.style.width = "1px"
    bar.style.height = "100%"
    bar.style.cursor = "e-resize"
  } else {
    bar.style.height = "1px"
    bar.style.width = "100%"
    bar.style.cursor = "n-resize"
  }
  bar.onmouseover = _ => {
    if (props.isVertical) {
      bar.style.width = "3px"
    } else {
      bar.style.height = "3px"
    }
  }

  bar.onmouseleave = _ => {
    if (props.isVertical) {
      bar.style.width = "1px"
    } else {
      bar.style.height = "1px"
    }
  }

}


</script>

<style lang="scss" scoped>
.resize-bar {
  user-select: none;
}

.resize-bar:hover {
  background: $color-dark-border;
}

</style>