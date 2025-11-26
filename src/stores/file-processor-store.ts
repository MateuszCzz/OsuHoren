import { defineStore } from "pinia";
import { ref } from "vue";
import { useSongStore } from "./song-store";
import { SongType } from "../types/song-type";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

const EVENT_DATA_SETUP_RESULT: string = "data-setup-result";
export type DataProcessingStatus = "Warning" | "Error" | "Started" | "Processing" | "Done";

interface DataSetupResult {
    status: DataProcessingStatus;
    message: string | null;
    songs: SongType[];
}

export const useFileProcessorStore = defineStore("fileProcessor", () => {
    const songStore = useSongStore();
    const totalAmount = ref<number>(0);
    const doneAmount = ref<number>(0);
    const status = ref<DataProcessingStatus | null>();
    const error = ref<string>("");

    let listener: UnlistenFn | null = null;

    function initProcessing() {
        // clean up previous state
        stopProcessing();

        // start listing function
        handleProcessing();

        // trigger backend file picker
        try {
            invoke("setup_osu_data");
        } catch (err) {
            console.error("Backend invocation failed:", err);
            status.value = "Error";
        }
    }

    async function handleProcessing() {
        listener = await listen<DataSetupResult>(EVENT_DATA_SETUP_RESULT, (event) => {
            const data = event.payload;
            console.log(data);
            switch (data.status) {
                case "Started":
                    totalAmount.value = Number(data.message) || 0;
                    status.value = "Started";
                    break;
                case "Processing":
                    doneAmount.value = Number(data.message) || 0;
                    status.value = "Processing";
                    break;
                case "Done":
                    stopProcessing();
                    status.value = "Done";
                    if (data.songs?.length) songStore.addSongs(data.songs);
                    console.log("Processing finished");
                    break;
                case "Warning":
                    error.value = data.message || "";
                    console.warn(data.message);
                    break;
                case "Error":
                    stopProcessing();
                    status.value = "Error";
                    error.value = data.message || "";
                    console.error(data.message);
                    break;
                default:
                    stopProcessing();
                    status.value = "Error";
                    console.error("Unknown status:", event.payload);
            }
        });
    }

    function stopProcessing() {
        if (listener) {
            listener();
            listener = null;
        }
        status.value = null;
        // songStore.removeSongs();
    }

    return {
        status, error, doneAmount, totalAmount,
        initProcessing
    };
});
