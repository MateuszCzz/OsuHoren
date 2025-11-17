import { defineStore } from "pinia";
import { ref } from "vue";
import type { SongType } from "../types/song-type";
import { useSongStore } from "./song-store";

export const useAudioPlayerStore = defineStore('audioPlayerStore', () => {
    const songStore = useSongStore();
    const currentSong = ref<SongType | null>(null);

    function chooseSong(song: SongType) {
        currentSong.value = song;
    }
    return { songStore, currentSong, chooseSong }
});