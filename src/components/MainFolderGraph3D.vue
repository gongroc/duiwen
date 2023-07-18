<template>
  <div id="page">
    <div id="graph"></div>
  </div>

</template>

<script setup>
import ForceGraph3D from '3d-force-graph';
import SpriteText from 'three-spritetext';
import {getCurrentInstance, onBeforeUnmount, onMounted, ref, watch} from "vue";
import {invoke} from "@tauri-apps/api";
import {Topic} from "../common/event.js";

const {proxy} = getCurrentInstance()

let instance = ref()
let canvasSize = ref()
let nodes = ref()
const key = ref(Math.random())


let props = defineProps({
  drawer: Object,
  folder: Object
})


onMounted(async _ => {
  await fetchData()
  await bindEvent()
})

onBeforeUnmount(async _ => {
  await unbindEvent()
  await clear()
})

watch(canvasSize, async _ => {
  await resetCanvasSize()
}, {deep: true})

watch(nodes, async _ => {
  await render()
})

watch(_ => props.drawer, async _ => {
  await fetchData()
})

async function fetchData() {
  nodes.value = await invoke("find_relation_map", {})
}

async function queryCanvasSize() {
  let page = document.getElementById('page')
  if (!!!page) {
    return
  }
  canvasSize.value = {
    width: page.offsetWidth,
    height: page.offsetHeight
  }
}

async function resetCanvasSize() {
  let canvas = instance.value
  let size = canvasSize.value
  if (!!!canvas || !!!size) {
    return
  }
  canvas.width(size.width)
  canvas.height(size.height)
}

async function bindEvent() {
  window.addEventListener('resize', async _ => {
    await queryCanvasSize()
  })
  proxy.$event.subscribe(Topic.FOLDER_CREATED, async _ => {
    await fetchData()
  })
  proxy.$event.subscribe(Topic.FOLDER_UNLINKED, async _ => {
    await fetchData()
  })
  proxy.$event.subscribe(Topic.FOLDER_DELETED, async _ => {
    await fetchData()
  })
}

async function unbindEvent() {
  window.removeEventListener('resize', async _ => {
    await queryCanvasSize()
  })
}

async function clear() {
  if (!instance.value) {
    return
  }
  let graph = instance.value
  graph.graphData({nodes: [], links: []})
  graph.pauseAnimation()
  graph.renderer().dispose();
  graph.renderer().forceContextLoss();
  graph.renderer().context = undefined;
  instance.value = null
}

async function render() {
  await queryCanvasSize()
  let graph = new ForceGraph3D()(document.getElementById('graph'))
      .graphData(nodes.value)
      .height(canvasSize.value.height)
      .showNavInfo(false)
      .backgroundColor("white")
      .linkDirectionalParticleSpeed(1)
      .linkDirectionalArrowLength(2)
      .linkDirectionalArrowRelPos(1.01)
      .linkDirectionalArrowColor(() => 'rgb(0, 0, 0)')
      .linkThreeObjectExtend(true)
      .linkColor(() => 'rgb(0, 0, 0)')
      .linkThreeObject(link => {
        const sprite = new SpriteText(`${link.label}`);
        sprite.color = 'black';
        sprite.textHeight = 3;
        return sprite;
      })
      .linkPositionUpdate((sprite, {start, end}) => {
        const middlePos = Object.assign(...['x', 'y', 'z'].map(c => ({
          [c]: start[c] + (end[c] - start[c]) / 2
        })));
        Object.assign(sprite.position, middlePos);
      })
      .nodeThreeObject(node => {
        const sprite = new SpriteText(node.title);

        if (node.id === -1){
          sprite.backgroundColor = 'blue'
          sprite.color = 'white';
        } else {
          sprite.backgroundColor = 'red'
          sprite.color = 'white';
        }

        sprite.borderRadius = 2;
        sprite.padding = 3;
        sprite.textHeight = 2
        sprite.material.depthWrite = false
        return sprite;
      })
      .onNodeClick(node => {
        setNodeCenter(node)
      })

  instance.value = graph

}

function setNodeCenter(node) {
  const distance = 200;
  const distRatio = 1 + distance / Math.hypot(node.x, node.y, node.z);
  const newPos = node.x || node.y || node.z
      ? {x: node.x * distRatio, y: node.y * distRatio, z: node.z * distRatio}
      : {x: 0, y: 0, z: distance};
  instance.value.cameraPosition(
      newPos,
      node,
      500
  );
}

</script>

<style lang="scss" scoped>
#page {
  flex: 1;
  overflow: hidden;
  position: relative;

}
</style>