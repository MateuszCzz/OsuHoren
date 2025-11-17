import { defineStore } from "pinia";
import { ref } from "vue";
import LoadFilesWorker from "../workers/load_files_worker?worker";
import { useSongStore } from "./song-store";

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

    return { status, errors, uniqueAmount, doneAmount, onFileSelect };
});
