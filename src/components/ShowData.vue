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
  <div class="container">
    <h3>Results ({{ audioPlayerStore.songStore.songs.length }})</h3>
    <RecycleScroller
      :key="scrollerKey"
      class="scroller"
      :items="audioPlayerStore.songStore.songs"
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
  height: 230px;
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
