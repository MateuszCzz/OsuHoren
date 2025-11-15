<script setup lang="ts">
import logo from "../assets/Logo.png";
import FileSelector from "../components/FileSelector.vue";

const emit = defineEmits(["processFileStream"]);

function processFileStream(msg: any) {
  emit("processFileStream", msg);
}
</script>
<template>
  <div class="container">
    <img src="../assets/Logo.png" alt="Logo" class="logo" />
    <FileSelector
      @processFileStream="processFileStream"
      class="file-selector"
    />
  </div>
</template>
<style scoped>
body {
  overflow: hidden;
}
.container {
  margin: 15px 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
  gap: 40px;
  text-align: center;
}

.logo {
  width: 75vw;
  max-width: 500px;
  height: auto;
}

.file-selector {
  width: 200px;
}
</style>
