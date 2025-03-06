import { load, Store } from '@tauri-apps/plugin-store';
import type { ThemeKind } from './types';

/** Auto-save every given millisecond, or never if set to `false` */
const AUTO_SAVE_FREQUENCY: false | number = 2000;

/**
 * An "Enum" for config item to allow auto-completion
 * and decouple from string representation.
 */
export const ConfigItem = {
    THEME: "theme",
    TRANSCRIPTS: "transcripts",
    INDEX: "index",
}

export class ConfigStore {
    // Store items
    fileStore: Store | null = null;
    cleanup: () => void;

    // Private Config data
    #theme: ThemeKind = $state("system");
    #transcriptions: string[] = $state([]);
    #currentIndex: number = $state(0);

    // Derived values
    transcriptLength = $derived(this.#transcriptions.length);
    isTranscriptsEmpty = $derived(this.#transcriptions.length === 0);
    currentTranscript = $derived(this.#transcriptions[this.#currentIndex]);

    constructor() {
        this.cleanup = $effect.root(() => {
            $effect(() => {
                const loadFile = async () => {
                    this.fileStore = await load('super-mouse-ai.json', { autoSave: AUTO_SAVE_FREQUENCY })
                    await this.loadAll();
                }
                loadFile();
                return () => {
                    this.fileStore?.close()
                }
            })
        });
    }

    /** Load all items from store for states */
    private async loadAll() {
        if (!this.fileStore) return;
        this.#theme = await this.fileStore.get(ConfigItem.THEME) ?? this.#theme;
        this.#transcriptions = await this.fileStore.get(ConfigItem.TRANSCRIPTS) ?? this.#transcriptions;
        this.#currentIndex = await this.fileStore.get(ConfigItem.INDEX) ?? this.#currentIndex;
        console.log(await this.fileStore.entries());
    }

    get theme(): ThemeKind {
        return this.#theme;
    }

    set theme(newTheme: ThemeKind) {
        this.#theme = newTheme;
        this.fileStore?.set(ConfigItem.THEME, this.#theme);
    }

    get transcriptions(): string[] {
        return this.#transcriptions;
    }

    addTranscription(transcript: string) {
        this.#transcriptions.push(transcript);
        this.fileStore?.set(ConfigItem.TRANSCRIPTS, this.#transcriptions);
    }

    editTranscription(edited: string) {
        this.#transcriptions[this.#currentIndex] = edited;
        this.fileStore?.set(ConfigItem.TRANSCRIPTS, this.#transcriptions);
        this.fileStore?.entries().then(x => console.log("edit: ", x));
    }

    removeCurrentTranscription() {
        this.#transcriptions.splice(this.#currentIndex, 1);
        this.fileStore?.set(ConfigItem.TRANSCRIPTS, this.#transcriptions);
    }

    get currentTranscriptionIndex() {
        return this.#currentIndex;
    }

    set currentTranscriptionIndex(idx) {
        this.#currentIndex = idx;
        this.fileStore?.set(ConfigItem.INDEX, this.#currentIndex);
    }

    prevIndex() {
        if (this.#currentIndex <= 0) return;
        this.#currentIndex--;
        this.fileStore?.set(ConfigItem.INDEX, this.#currentIndex);
        this.fileStore?.entries().then(x => console.log("prevI: ", x));
    }

    nextIndex() {
        if (this.#currentIndex >= this.transcriptLength - 1) return;
        this.#currentIndex++;
        this.fileStore?.set(ConfigItem.INDEX, this.#currentIndex);
        this.fileStore?.entries().then(x => console.log("nextI: ", x));
    }
}

export const configStore = new ConfigStore();