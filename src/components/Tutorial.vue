<template>
  <div id="tutorial" data-tauri-drag-region>
    <div class="tutorial-wrapper">
      <div class="body">
        <transition>
          <img src="../assets/tutorial/1.jpg" alt="" v-if="current === 1">
        </transition>
        <transition>
          <img src="../assets/tutorial/2.jpg" alt="" v-if="current === 2">
        </transition>
        <transition>
          <img src="../assets/tutorial/3.jpg" alt="" v-if="current === 3">
        </transition>
      </div>
      <div class="footer">
        <div class="left">
          <div class="btn" @click="onPre">
            <el-icon>
              <ArrowLeft/>
            </el-icon>
          </div>
        </div>
        <div class="center">
          <el-button size="small" type="primary">
            {{$t('common.start')}}
          </el-button>
        </div>
        <div class="right">
          <div class="btn" @click="onNext">
            <el-icon>
              <ArrowRight/>
            </el-icon>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import {ref, onMounted} from 'vue'

let current = ref(1)
let stop = false

onMounted(_ => {
  play()
})

function onNext() {
  current.value = current.value === 3 ? 1 : current.value + 1
  stop = true
}

function onPre() {
  current.value = current.value === 1 ? 3 : current.value - 1
  stop = true
}

function play() {
  if (stop) {
    return
  }
  current.value = current.value === 1 ? 3 : current.value - 1
  setTimeout(_ => {
    play()
  }, 3500)
}


</script>

<style lang="scss">
#tutorial {
  position: absolute;
  height: 100%;
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  background: rgba(0, 0, 0, 0.5);

  .tutorial-wrapper {
    background: white;
    width: 700px;
    height: 500px;
    border-radius: 7px;
    display: flex;
    flex-direction: column;
    overflow: hidden;

    .body {
      flex: 1;
      height: 0;
      overflow: hidden;

      img {
        width: 100%;
      }
    }

    .footer {
      height: 50px;
      display: flex;
      justify-content: center;
      align-items: center;
      border-top: 1px solid $color-base-border;

      > div {
        flex: 1;
        text-align: center;

        .btn {
          cursor: pointer;
          background: $color-base-fill;
          display: inline-flex;
          width: 25px;
          height: 25px;
          border-radius: 50%;
          justify-content: center;
          align-items: center;
        }

        .btn:hover {
          color: white;
          background: $color-brand;
        }

      }
    }
  }
}

.v-enter-active, .v-leave-active {
  transition: opacity 0.1s ease;
}

.v-enter-from, .v-leave-to {
  opacity: 0.6;
}


</style>