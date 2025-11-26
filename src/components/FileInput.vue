<script setup lang="ts">
import { useFileProcessorStore } from "../stores/file-processor-store";
const fileProcessorStore = useFileProcessorStore();
</script>

<template>
  <div>
    <button @click="fileProcessorStore.initProcessing">Select osu.db</button>
  </div>
</template>
