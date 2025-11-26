import { defineStore } from "pinia";
import { ref } from "vue";
import type { SongType } from "../types/song-type";

export const useSongStore = defineStore('songStore', () => {
    const songs = ref<SongType[]>([]);

    function addSong(song: SongType) {
        songs.value.push(song);
    }

    function addSongs(songsArray: SongType[]) {
        songsArray.forEach(song => {
            songs.value.push(song);
        });
    }

    return { songs, addSong, addSongs }
});