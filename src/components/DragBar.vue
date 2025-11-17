<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
  value: number;
  type: string;
  onChange: (ratio: number) => void;
  onEnd?: () => void;
}>();

const bar = ref<HTMLElement | null>(null);
const dragging = ref(false);

function startDrag(e: MouseEvent) {
  dragging.value = true;
  handleDrag(e);
  window.addEventListener("mousemove", handleDrag);
  window.addEventListener("mouseup", stopDrag);
}

function handleDrag(e: MouseEvent) {
  if (!dragging.value || !bar.value) return;
  const { left, width } = bar.value.getBoundingClientRect();
  const ratio = Math.min(1, Math.max(0, (e.clientX - left) / width));
  props.onChange(ratio);
}

function stopDrag() {
  if (!dragging.value) return;
  dragging.value = false;
  props.onEnd?.();
  window.removeEventListener("mousemove", handleDrag);
  window.removeEventListener("mouseup", stopDrag);
}
</script>

<template>
  <div class="slider-wrapper">
    <div v-if="props.type" class="slider-label">{{ props.type }}</div>
    <div class="slider-bar" @mousedown="startDrag" ref="bar">
      <div class="slider-level" :style="{ width: value + '%' }" />
    </div>
  </div>
</template>

<style scoped>
.slider-wrapper {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}
.slider-label {
  font-size: 0.85rem;
  color: #444;
}
.slider-bar {
  width: 100%;
  height: 10px;
  background: #ddd;
  border-radius: 5px;
  cursor: pointer;
  position: relative;
  user-select: none;
}
.slider-level {
  height: 100%;
  border-radius: 5px;
  pointer-events: none;
  user-select: none;
  background: rgb(255, 162, 0);
}
</style>
