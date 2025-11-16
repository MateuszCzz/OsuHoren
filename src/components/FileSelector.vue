<script setup lang="ts">
import LoadFilesWorker from "../workers/LoadFilesWorker?worker";

const emit = defineEmits(["processFileStream"]);

const worker = new LoadFilesWorker();

//handle worker output and kill it when useless
worker.onmessage = (e) => {
  emit("processFileStream", e.data);

  if (e.data.done) {
    worker.terminate();
  }
};

function handleFileSelect(e: Event) {
  const input = e.target as HTMLInputElement;
  const files = Array.from(input?.files || []);

  // no files found
  if (!files.length) {
    emit("processFileStream", {
      error: true,
      message: "No Files selected.",
    });
    return;
  }

  // check if the main uber folder is "songs"
  const folder = files[0].webkitRelativePath.split("/")[0].toLowerCase();
  if (folder !== "songs") {
    emit("processFileStream", {
      error: true,
      message: "Please select songs folder from your game directory.",
    });

    return;
  }
  //hide title screen
  emit("processFileStream", { start: true });

  // deploy worker to handle the rest of the heavy lifting
  worker.postMessage(files);
}
</script>

<template>
  <input class="file-input" type="file" webkitdirectory multiple @change="handleFileSelect" />
</template>
