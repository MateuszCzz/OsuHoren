<script setup lang="ts">
import { RecycleScroller } from "vue-virtual-scroller";
import type { SongType } from "../types/song-type";
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'

const props = defineProps<{
  songs: SongType[];
}>();

const emit = defineEmits<{
  (e: "songChosen", song: SongType): void;
  (e: "fileSelected", file: File | {}): void;
}>();

function chooseSong(song: SongType) {
  emit("songChosen", song);
}
</script>

<template>
  <div class="container">
    <h3>Results ({{ props.songs.length }})</h3>
    <RecycleScroller
      class="scroller"
      :items="songs"
      :item-size="50"
      key-field="id"
      v-slot="{ item }"
    >
      <div class="item-display" @click.stop="chooseSong(item)">
        <p class="title">{{ item.id }} - {{ item.title }}</p>
      </div>
    </RecycleScroller>
  </div>
</template>

<style scoped>
.scroller {
  height: 280px;
  width: 100%;
  overflow-y: auto;
  background-color: #fcdfdfff;
}

.item-display {
  height: 50px;
  padding: 0 12px;
  align-items: center;
  background-color: #9ad49cff;
  overflow-y: hidden;
  display: flex;
}
.itemDisplay p {
  margin: 0;
  padding: 0 12px;
}
</style>
