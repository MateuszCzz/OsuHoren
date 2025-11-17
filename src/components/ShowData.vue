<script setup lang="ts">
import { RecycleScroller } from "vue-virtual-scroller";
import type { SongType } from "../types/song-type";
import { useSongStore } from "../stores/song-store";
import "vue-virtual-scroller/dist/vue-virtual-scroller.css";
import { ref, watch } from "vue";

const songStore = useSongStore();

//TODO:FIND BETTER SOLUTION THAN THIS RERENDER HACK FOR RENDER ISSUE PINIA+SCROLLER
// force render of scrollr by changing key
const scrollerKey = ref<number>(0);
watch(songStore.songs, () => {
  scrollerKey.value++;
});

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
    <h3>Results ({{ songStore.songs.length }})</h3>
    <RecycleScroller
      :key="scrollerKey"
      class="scroller"
      :items="songStore.songs"
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
