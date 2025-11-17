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
  <main class="container">
    <p>{{ fileProcessor.status }}</p>
    <!-- show till file selection, unmount only after worker is done -->
    <div v-show="showTitleScreen">
      <TitleScreen v-if="fileProcessor.status !== 'done'" />
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
