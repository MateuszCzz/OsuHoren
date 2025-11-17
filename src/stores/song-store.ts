import { defineStore } from "pinia";
import { ref } from "vue";
import type { SongType } from "../types/song-type";

export const useSongStore = defineStore('songStore', () => {
    const songs = ref<SongType[]>([]);
    // id increment
    const nextId = ref<number>(0);

    function addSong(song: SongType) {
        song.id = nextId.value++;
        songs.value.push(song);
    }

    function addSongs(songsArray: SongType[]) {
        songsArray.forEach(song => {
            song.id = nextId.value++;
            songs.value.push(song);
        });
    }

    return { songs, addSong, addSongs }
});