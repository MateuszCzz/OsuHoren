<script setup lang="ts">
import { ref, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface FilePickerResult {
  path: string | null;
  error: string | null;
  warning: string | null;
  songs: string | null;
}

const status = ref(
  "Please select your osu.db file found in your osu! standard directory."
);
const filePath = ref<string | null>(null);
const songs = ref<string | null>(null);

// Listen for backend events
let unlisten: () => void;

onUnmounted(() => {
  if (unlisten) unlisten();
});

async function onClick() {
  // register listener first
  if (!unlisten) {
    unlisten = await listen<FilePickerResult>("file-picker-result", (event) => {
      const result = event.payload;
      if (result.error) {
        status.value = `Error: ${result.error}`;
        filePath.value = null;
        songs.value = null;
      } else {
        status.value = `Selected: ${result.path}`;
        filePath.value = result.path;
        songs.value = result.songs;
      }
    });
  }

  // trigger backend file picker
  invoke("load_osu_db");
}
</script>

<template>
  <div>
    <button @click="onClick">Select osu.db</button>
    <p>{{ status }}</p>
    <p v-if="filePath">Path: {{ filePath }}</p>
    <p v-if="songs">Songs: {{ songs }}</p>
  </div>
</template>
