import { defineStore } from "pinia";
import { ref } from "vue";
import LoadFilesWorker from "../workers/load_files_worker?worker";
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
    const worker = ref<Worker | null>(null);
    // pending->notStarted error->fatalError started->prepingToProcess 
    const status = ref<"pending" | "error" | "started" | "processing" | "done">("pending");
    const errors = ref<string>("");
    const uniqueAmount = ref<number>(0);
    const doneAmount = ref<number>(0);

    // user uploaded files
    function onFileSelect(e: Event) {
        const input = e.target as HTMLInputElement;
        const files = Array.from(input?.files || []);

        // no files found
        if (!files.length) {
            status.value = "error";
            errors.value = "No Files selected.";
            return;
        }

        // check if the main folder is called "songs"
        const folder = files[0].webkitRelativePath.split("/")[0].toLowerCase();
        if (folder !== "songs") {
            status.value = "error";
            errors.value = "Please select songs folder from your game directory.";
            return;
        }

        // start worker
        status.value = "started";
        if (!worker.value) initWorker();
        worker.value?.postMessage(files);
    }

    // uploaded files were ok start processing
    function initWorker() {
        worker.value = new LoadFilesWorker();

        worker.value.onmessage = (e) => {
            const data = e.data;
            console.log("Worker message received:", data);

            // worker hit error
            if (data.error) {
                console.log("Worker error:", data.message, "Fatal?", data.fatal || false);
                if (data.fatal) {
                    terminateWorker();
                    status.value = "error";
                }
                errors.value += (errors.value ? "\n" : "") + data.message;
                return;
            }

            // worker started processing
            if (data.processing) {
                console.log("Worker started processing. Unique amount:", data.unique_amount);
                status.value = "processing";
                uniqueAmount.value = data.unique_amount;
            }

            // partial data update
            if (data.partial) {
                console.log("Worker partial batch received. Batch size:", data.data.length);
                songStore.addSongs(data.data);
                doneAmount.value += data.data.length; // running total
            }

            // worker finished processing
            if (data.done) {
                console.log("Worker finished processing all songs.");
                status.value = "done";
                terminateWorker();
            }
        };
    }

    function terminateWorker() {
        worker.value?.terminate();
        worker.value = null;
    }
















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
    return { status, errors, uniqueAmount, doneAmount, onFileSelect, initProcessing };
});
