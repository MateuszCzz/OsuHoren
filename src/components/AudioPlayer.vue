<script setup lang="ts">
import { ref, watchEffect } from "vue";
import { SongData } from "../types/Types";

const audioUrl = ref<string | null>(null);

const props = defineProps<{
  currentSong: SongData;
}>();



// Preparing the song url for music player
watchEffect(() => {
  if (props.currentSong) {
    audioUrl.value = URL.createObjectURL(props.currentSong.audioFile);
  }
});
</script>

<template>
  <div v-if="audioUrl">
    <audio ref="player" :src="audioUrl" controls autoplay />
  </div>
</template>

<style scoped></style>
