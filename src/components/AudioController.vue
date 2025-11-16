<script setup lang="ts">
import { ref, watch, computed, onMounted, onUnmounted } from "vue";
import { SongData } from "../types/Types";

const props = defineProps<{
  songs: SongData[];
  currentSong: SongData | null;
}>();

const player = ref<HTMLAudioElement | null>(null);
const volumeBar = ref<HTMLDivElement | null>(null);
// song current play moment postion
const seekBar = ref<HTMLDivElement | null>(null);
const isPlaying = ref(false);

const songDuration = ref<number>(0);
const currentDuration = ref<number>(0);
const playbackSpeed = ref<number>(1);
const volume = ref<number>(20);

const formattedCurrent = computed(() => formatTime(currentDuration.value));
const formattedTotal = computed(() => formatTime(songDuration.value));
const seekPercent = computed(() =>
  songDuration.value ? (currentDuration.value / songDuration.value) * 100 : 0
);

function togglePlay() {
  if (!player.value) return;

  if (isPlaying.value) {
    player.value.pause();
    isPlaying.value = false;
  } else {
    player.value.play();
    isPlaying.value = true;
  }
}
// change playback speed NM -> DT -> HF ->...
const speeds = [1, 1.5, 0.5];
function togglePlaybackSpeed() {
  if (!player.value) return;
  const i = speeds.indexOf(playbackSpeed.value);
  playbackSpeed.value = speeds[(i + 1) % speeds.length];
  player.value.playbackRate = playbackSpeed.value;
}
//TODO: look into pitcher for that sweet NC
// https://stackoverflow.com/questions/37206304/changing-the-pitch-of-an-audio-element-without-external-libraries

// watch changes of chosen song
const audioSrc = computed(() => {
  if (!props.currentSong) return null;
  return URL.createObjectURL(props.currentSong.audioFile);
});

watch(audioSrc, (newUrl, oldUrl) => {
  if (oldUrl) URL.revokeObjectURL(oldUrl);
  if (player.value) player.value.src = newUrl || "";
  // account for autoplay
  isPlaying.value = !!newUrl;
});

// handle when audio loads
function onLoadHandle() {
  if (!player.value) return;

  if (player.value.readyState >= 2) {
    // restore saved settings
    player.value.volume = volume.value / 100;
    player.value.playbackRate = playbackSpeed.value;
    // get song duration
    songDuration.value = player.value.duration;
  }
}

// handle when current play time changes / when song plays
function onTimeUpdateHandle() {
  if (!player.value) return;

  currentDuration.value = player.value.currentTime;
}

// helper functions to handle mouse drag on toggle bars like volume or playfield
// is user dragging a element
const dragging = ref<null | DragType>(null);

type DragType = "volume" | "seek";

// mouse down
function startDrag(e: MouseEvent, t: DragType) {
  dragging.value = t;
  handleDrag(e, t);
}

// mouse over
// user dragging change values
function onDrag(e: MouseEvent, t: DragType) {
  if (dragging.value !== t) return;
  handleDrag(e, t);
}

// mouse up
// remove listeners
function stopDrag(t: DragType) {
  if (dragging.value !== t) return;
  dragging.value = null;
  if (t === "seek") player.value?.play();
}

function handleDrag(e: MouseEvent, t: DragType) {
  if (!player.value) return;

  const bar = t === "volume" ? volumeBar.value : seekBar.value;
  if (!bar) return;

  const rect = bar.getBoundingClientRect();
  const x = Math.max(0, Math.min(rect.width, e.clientX - rect.left));
  const ratio = x / rect.width;

  if (t === "volume") {
    player.value.volume = ratio;
    volume.value = ratio * 100;
  } else {
    player.value.currentTime = ratio * songDuration.value;
    player.value.pause();
  }
}

// helper to format time in seconds to human time
function formatTime(seconds: number) {
  const m = Math.floor(seconds / 60)
    .toString()
    .padStart(2, "0");
  const s = Math.floor(seconds % 60)
    .toString()
    .padStart(2, "0");
  return `${m}:${s}`;
}

onMounted(() => {
  if (player.value) {
    // when song is loaded
    player.value.addEventListener("loadeddata", onLoadHandle);
    player.value.addEventListener("timeupdate", onTimeUpdateHandle);
    // player.value.addEventListener("ended", handleEnded);
  }
});

// garbage event listeners
onUnmounted(() => {
  if (player.value) {
    player.value.removeEventListener("loadeddata", onLoadHandle);
    player.value.removeEventListener("timeupdate", onTimeUpdateHandle);
    // player.value.removeEventListener("ended", handleEnded);
  }
});
</script>

<template>
  <div class="player">
    <div class="info">
      <div class="title">{{ currentSong ? currentSong.title : "Title" }}</div>
      <div class="artist">
        {{ currentSong ? currentSong.artist : "Artist" }}
      </div>

      <div class="time">
        <span>{{ currentDuration ? formattedCurrent : "00:00" }}</span>
        <span>/</span>
        <span>{{ songDuration ? formattedTotal : "00:00" }}</span>
      </div>
    </div>

    <div class="controls">
      <button @click="togglePlay">
        {{ isPlaying ? "Pause" : "Play" }}
      </button>

      <button @click="togglePlaybackSpeed">
        {{ playbackSpeed == 1 ? "NM" : playbackSpeed == 1.5 ? "DT" : "HF" }}
      </button>
    </div>

    <div class="volume-section">
      <span class="volume-label">Volume: {{ Math.floor(volume) }}%</span>

      <div
        class="volume-bar"
        @mousedown="startDrag($event, 'volume')"
        @mousemove="onDrag($event, 'volume')"
        @mouseup="stopDrag('volume')"
        ref="volumeBar"
      >
        <div class="volume-level" :style="{ width: volume + '%' }" />
      </div>
    </div>

    <div class="seek-section">
      <div
        class="seek-bar"
        @mousedown="startDrag($event, 'seek')"
        @mousemove="onDrag($event, 'seek')"
        @mouseup="stopDrag('seek')"
        ref="seekBar"
      >
        <div class="seek-level" :style="{ width: seekPercent + '%' }" />
      </div>
    </div>

    <audio ref="player" autoplay loop />
  </div>
</template>

<style scoped>
.player {
  background-color: beige;
  padding: 30px;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  width: 260px;
  font-family: sans-serif;
}

.info {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
}

.title {
  font-weight: 600;
  font-size: 1.1rem;
}

.artist {
  color: #666;
  font-size: 0.9rem;
}

.time {
  display: flex;
  gap: 0.3rem;
  font-size: 0.85rem;
  color: #333;
}

.controls {
  display: flex;
  gap: 0.5rem;
}

.controls button {
  padding: 0.4rem 0.7rem;
  border-radius: 6px;
  border: 1px solid #ccc;
  background: #f4f4f4;
  cursor: pointer;
}

.volume-section,
.seek-section {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

.volume-label {
  font-size: 0.85rem;
  color: #444;
}

.volume-bar,
.seek-bar {
  user-select: none;
  width: 100%;
  height: 10px;
  background: #ddd;
  cursor: pointer;
  position: relative;
  border-radius: 5px;
}

.volume-level {
  user-select: none;
  pointer-events: none;
  height: 100%;
  background: #4caf50;
  border-radius: 5px;
}

.seek-level {
  user-select: none;
  pointer-events: none;
  height: 100%;
  background: chocolate;
  border-radius: 5px;
}
</style>
