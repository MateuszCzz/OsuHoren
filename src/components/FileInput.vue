<script setup lang="ts">
import { ref, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface DataSetupResult {
  status: string;
  message: string | null;
  songs: any;
}

const status = ref(
  "Please select your osu.db file found in your osu! standard directory."
);
const songs = ref<string | null>(null);

// Listen for backend events
let unlisten: () => void;

onUnmounted(() => {
  if (unlisten) unlisten();
});

async function onClick() {
  // register listener
  if (!unlisten) {
    unlisten = await listen<DataSetupResult>("data-setup-result", (event) => {
      const result = event.payload;
      console.log(result);
      status.value = result.status;
      songs.value = result.songs ? JSON.stringify(result.songs, null, 2) : null;
    });
  }

  // trigger backend file picker
  invoke("setup_osu_data");
}
</script>

<template>
  <div>
    <button @click="onClick">Select osu.db</button>
    <p>{{ status }}</p>
    <p v-if="songs">Songs: {{ songs }}</p>
  </div>
</template>
