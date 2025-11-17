<script setup lang="ts">
import { useFileProcessorStore } from "../stores/file-processor-store";

const fileProcessor = useFileProcessorStore();
</script>

<template>
  <input
    type="file"
    webkitdirectory
    multiple
    @change="fileProcessor.onFileSelect"
  />
</template>
