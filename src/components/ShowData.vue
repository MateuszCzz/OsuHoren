<script setup lang="ts">
import { RecycleScroller } from "vue-virtual-scroller";
import type { SongType } from "../types/song-type";
import "vue-virtual-scroller/dist/vue-virtual-scroller.css";
import { ref, watch } from "vue";
import { useAudioPlayerStore } from "../stores/audio-player-store";

const audioPlayerStore = useAudioPlayerStore();

//TODO:FIND BETTER SOLUTION THAN THIS RERENDER HACK FOR RENDER ISSUE PINIA+SCROLLER
// force render of scrollr by changing key
const scrollerKey = ref<number>(0);
watch(audioPlayerStore.songStore.songs, () => {
  scrollerKey.value++;
});

function chooseSong(song: SongType) {
  audioPlayerStore.chooseSong(song);
}
</script>

<template>
  <div class="song-list">
    <span class="header">
      Total Songs: {{ audioPlayerStore.songStore.songs.length }}
    </span>
    <RecycleScroller
      :key="scrollerKey"
      class="scroller"
      :items="audioPlayerStore.songStore.songs"
      :item-size="50"
      key-field="id"
      v-slot="{ item }"
    >
      <div class="song-item" @click.stop="chooseSong(item)">
        <p class="song-title">{{ item.id }} - {{ item.title_ascii }}</p>
      </div>
    </RecycleScroller>
  </div>
</template>

<style scoped>
.song-list {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.header {
  padding: 4px 0;
  text-align: center;
  font-weight: bold;
}

.scroller {
  flex: 1;
  overflow-y: auto;
  scrollbar-color: rgb(60, 60, 60) rgb(30, 30, 30);
  scrollbar-width: thin;
}

.song-item {
  height: 50px;
  padding: 0 12px;
  display: flex;
  align-items: center;
  cursor: pointer;
}

.song-title {
  margin: 0;
  padding: 0;
  white-space: nowrap;
  overflow: hidden;
}
</style>
