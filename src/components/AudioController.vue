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
  const title = audioPlayerStore.currentSong?.title_ascii || "Title";
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
// watch(
//   () => audioPlayerStore.currentSong,
//   (newSong) => {
//     if (oldUrl) URL.revokeObjectURL(oldUrl);

//     if (player.value && newSong) {
//       oldUrl = URL.createObjectURL(newSong.audioFile);
//       player.value.src = oldUrl;
//     }
//   }
// );

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
    <!-- main part of the music player -->
    <div class="player-main">
      <!-- TODO: -->
      <div class="cover"></div>

      <div class="meta">
        <div class="meta-title">
          {{ songTitle }}
        </div>
        <div class="meta-artist">
          {{
            audioPlayerStore.currentSong
              ? audioPlayerStore.currentSong.artist_ascii
              : "Artist"
          }}
        </div>
      </div>

      <div class="controls-side">
        <DragBar type="volume" :value="volume" :onChange="onVolumeChange" />
      </div>

      <div class="controls-primary">
        <button @click="togglePlaybackSpeed">
          {{ playbackSpeed == 1 ? "NM" : playbackSpeed == 1.5 ? "DT" : "HF" }}
        </button>
        <button @click="togglePlay">
          {{ isPlaying ? "Pause" : "Play" }}
        </button>
      </div>
    </div>
    <div class="seek">
      <span class="time">{{
        currentDuration ? humanCurrentTime : "00:00"
      }}</span>
      <DragBar
        class="seek-bar"
        type="seek"
        :value="seekProgress"
        :onChange="onSeekChange"
        :onEnd="() => player?.play()"
      />
      <span class="time"> {{ songDuration ? humanTotalTime : "00:00" }}</span>
    </div>

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
  border-top: 3px solid #333333;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  width: 100%;
  padding: 10px;
  box-sizing: border-box;
  background: #2a2a2a;
  font-family: sans-serif;
}

/* main top section */
.player-main {
  display: grid;
  grid-template-columns: auto 1fr auto;
  align-items: center;
  width: 100%;
}

.cover {
  grid-column: 1;
  width: 5rem;
  height: 5rem;
  background: #ddd;
}

.meta {
  display: flex;
  flex-direction: column;
  min-width: 25px;
  grid-column: 2;
  justify-self: start;
  box-sizing: border-box;
  padding: 5px;
}

.meta-title {
  font-weight: bold;
}

.meta-artist {
  opacity: 0.7;
}

/* buttons + side controls */
.controls-primary {
  display: flex;
  gap: 0.5rem;
  min-width: 40px;
  grid-column: 2;
  justify-self: center;
}

.controls-side {
  width: 5rem;
  height: 5rem;
  grid-column: 3;
  justify-self: end;
}

/* seek area */
.seek {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.seek-bar {
  flex: 1;
  height: 3px;
  padding-top: 8px;
}

.seek-bar :deep(.slider-level) {
  background: black;
}

.time {
  width: 50px;
  text-align: center;
  font-variant-numeric: tabular-nums;
}
</style>

<!-- Primary (Accent): #FF66CC – energetic pink

Secondary: #FFCC33 – bright yellow for highlights

Background: #1A1A1A – deep dark gray

Surface / Cards: #2A2A2A – slightly lighter dark

Text Primary: #E0E0E0 – off-white for readability

Text Secondary: #AAAAAA – muted gray for secondary info

Interactive / Hover: #FF3399 – vibrant hover effect

Borders / Dividers: #333333 – subtle separation -->
