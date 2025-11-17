<script setup lang="ts">
import { computed } from "vue";

import TitleScreen from "./components/TitleScreen.vue";
import ShowData from "./components/ShowData.vue";
import AudioController from "./components/AudioController.vue";
import { useFileProcessorStore } from "./stores/file-processor-store";

const fileProcessor = useFileProcessorStore();

const showTitleScreen = computed(
  () => fileProcessor.status === "pending" || fileProcessor.status === "error"
);
</script>

<template>
  <main>
    <!-- show title screen till file selection -->
    <div class="title-container" v-if="showTitleScreen">
      <TitleScreen />
    </div>

    <div class="main-content-container" v-else>
      <!-- status bar till worker is working
         
         -->
      <div class="status-bar" v-if="fileProcessor.status !== 'done'">
        <p>{{ fileProcessor.status }}</p>
        <p v-if="fileProcessor.status === 'processing'">
          {{ fileProcessor.doneAmount }} / {{ fileProcessor.uniqueAmount }}
        </p>
      </div>

      <!-- remaining space for song list -->
      <div class="data-show">
        <ShowData />
      </div>

      <!-- player -->
      <div class="player-bar">
        <AudioController />
      </div>
    </div>
  </main>
</template>

<style>
html,
body,
#app,
main {
  height: 100%;
}

html,
body,
#app,
main,
p {
  margin: 0;
  padding: 0;
  color: #ff66cc;
  background-color: #1a1a1a;
}

.title-container {
  height: 100%;
  width: 100%;
  margin: 0;
}

.main-content-container {
  height: 100%;
  width: 100%;
  margin: 0;
  display: flex;
  flex-direction: column;
  height: 100%;
}
.status-bar,
.status-bar p {
  color: black;
  background-color: #ff66cc;
}
.status-bar {
  text-align: center;
  box-sizing: border-box;
  padding: 5px;
}

.data-show {
  height: 100%;
  flex: 1 1 auto;
  overflow: hidden;
  background-color: #1a1a1a;
  color: wheat;
}
</style>
