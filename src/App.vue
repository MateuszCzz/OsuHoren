<script setup lang="ts">
import { ref } from "vue";

import TitleScreen from "./components/TitleScreen.vue";
import ShowData from "./components/ShowData.vue";
import AudioController from "./components/AudioController.vue";

const status = ref<string>("");
const showTitleScreen = ref(true);
const mountTitleScreen = ref(true);

// Handle feedback from file processing worker
function processFileStream(msg: any) {
  if (msg.error) {
    status.value = `Error: ${msg.message}`;
    return;
  }

  if (msg.start) {
    // Hide early but keep mounted
    if (showTitleScreen.value) showTitleScreen.value = false;
    return;
  }

  if (msg.done) {
    //destroy >:(
    mountTitleScreen.value = false;
    status.value = "Finished processing files.";
  }
}
</script>

<template>
  <main class="container">
    <p>{{ status }}</p>
    <!-- show till file selection, unmount only after worker is done -->
    <div v-show="showTitleScreen">
      <TitleScreen
        v-if="mountTitleScreen"
        @processFileStream="processFileStream"
      />
    </div>

    <div v-show="!showTitleScreen">
      <AudioController />
      <ShowData />
    </div>
  </main>
</template>

<style>
body {
  background: #2c2c2cff;
}
</style>
