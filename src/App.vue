<script setup lang="ts">
import { ref } from "vue";
import type { SongData } from "./types/Types";

import TitleScreen from "./components/TitleScreen.vue";
import ShowData from "./components/ShowData.vue";
import AudioController from "./components/AudioController.vue";

const currentSong = ref<SongData | null>(null);
const songs = ref<SongData[]>([]);
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

  if (msg.partial) {
    // squash array
    songs.value = [...songs.value, ...msg.data];
    return;
  }

  if (msg.done) {
    //destroy >:(
    mountTitleScreen.value = false;
    status.value = "Finished processing files.";
  }
}

function playSong(selectedSong: SongData) {
  currentSong.value = selectedSong;
}
</script>

<template>
  <main class="container">
    <!-- show till file selection, unmount only after worker is done -->
    <div v-show="showTitleScreen">
      <TitleScreen
        v-if="mountTitleScreen"
        @processFileStream="processFileStream"
      />
      <p>{{ status }}</p>
    </div>

    <div v-show="!showTitleScreen">
      <AudioController :songs="songs" :currentSong="currentSong" />
      <ShowData :songs="songs" @songChosen="playSong" />
    </div>
  </main>
</template>

<style>
body {
  background: #2c2c2cff;
}
</style>
