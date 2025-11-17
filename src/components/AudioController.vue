<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { useAudioPlayerStore } from "../stores/audio-player-store";
import DragBar from "./DragBar.vue";

const audioPlayerStore = useAudioPlayerStore();
const player = ref<HTMLAudioElement | null>(null);

const isPlaying = ref(false);
const songDuration = ref<number>(0);
const currentDuration = ref<number>(0);
const playbackSpeed = ref<number>(1);
const volume = ref<number>(20);

const humanCurrentTime = computed(() => formatTime(currentDuration.value));
const humanTotalTime = computed(() => formatTime(songDuration.value));
const songTitle = computed(() => {
  const title = audioPlayerStore.currentSong?.title || "Title";
  return title.length > 26 ? title.slice(0, 26) + ".." : title;
});
const seekProgress = computed(() =>
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
const playbackValues = [1, 1.5, 0.5];
function togglePlaybackSpeed() {
  if (!player.value) return;
  const i = playbackValues.indexOf(playbackSpeed.value);
  playbackSpeed.value = playbackValues[(i + 1) % playbackValues.length];
  player.value.playbackRate = playbackSpeed.value;
}
//TODO: look into pitcher for that sweet NC
// https://stackoverflow.com/questions/37206304/changing-the-pitch-of-an-audio-element-without-external-libraries

// watch changes of chosen song and prep new url while garbaging old
let oldUrl: string = "";
watch(
  () => audioPlayerStore.currentSong,
  (newSong) => {
    if (oldUrl) URL.revokeObjectURL(oldUrl);

    if (player.value && newSong) {
      oldUrl = URL.createObjectURL(newSong.audioFile);
      player.value.src = oldUrl;
    }
  }
);

// handle when audio loads
function onLoadHandle() {
  if (!player.value) return;

  if (player.value.readyState >= 2) {
    // restore saved settings
    player.value.volume = volume.value / 100;
    player.value.playbackRate = playbackSpeed.value;
    // get song duration
    songDuration.value = player.value.duration;
    // account for autoplay
    isPlaying.value = true;
  }
}

function onVolumeChange(ratio: number) {
  if (!player.value) return;
  volume.value = ratio * 100;
  player.value.volume = ratio;
}

function onSeekChange(ratio: number) {
  if (!player.value) return;
  player.value.currentTime = ratio * songDuration.value;
  player.value.pause();
}

// handle when current play time changes / when song plays
function onTimeUpdateHandle() {
  if (!player.value) return;

  currentDuration.value = player.value.currentTime;
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
</script>

<template>
  <div class="player">
    <div class="info">
      <div class="title">
        {{ songTitle }}
      </div>
      <div class="artist">
        {{
          audioPlayerStore.currentSong
            ? audioPlayerStore.currentSong.artist
            : "Artist"
        }}
      </div>

      <div class="time">
        <span>{{ currentDuration ? humanCurrentTime : "00:00" }}</span>
        <span>/</span>
        <span>{{ songDuration ? humanTotalTime : "00:00" }}</span>
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

    <DragBar type="volume" :value="volume" :onChange="onVolumeChange" />

    <DragBar
      type="seek"
      :value="seekProgress"
      :onChange="onSeekChange"
      :onEnd="() => player?.play()"
    />

    <audio
      ref="player"
      @loadeddata="onLoadHandle"
      @timeupdate="onTimeUpdateHandle"
      autoplay
      loop
    />
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
</style>
