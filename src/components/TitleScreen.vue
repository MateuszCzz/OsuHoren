<script setup lang="ts">
import FileInput from "../components/FileInput.vue";
</script>
<template>
  <div class="container">
    <img src="../assets/Logo.png" alt="Logo" class="logo" />
    <FileInput class="file-selector" />
    <p>
      "Please select your osu.db file found in your osu! standard directory."
    </p>
  </div>
</template>
<style scoped>
.container {
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
