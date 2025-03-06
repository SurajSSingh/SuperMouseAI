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
}

export class ConfigStore {
    fileStore: Store | null = null;
    #theme: ThemeKind = $state("system");
    #transcriptions: string[] = $state([]);
    #cleanup: () => void;

    constructor() {
        this.#cleanup = $effect.root(() => {
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

    private async loadAll() {
        if (!this.fileStore) return;
        this.#theme = await this.fileStore.get(ConfigItem.THEME) ?? this.#theme;
        this.#transcriptions = await this.fileStore.get(ConfigItem.TRANSCRIPTS) ?? this.#transcriptions;
        console.log(await this.fileStore.entries());
    }

    async saveAll() {
        await this.fileStore?.save();
    }

    cleanup() {
        this.#cleanup();
    }

    get theme(): ThemeKind {
        return this.#theme;
    }

    set theme(newTheme: ThemeKind) {
        this.#theme = newTheme;
        this.fileStore?.set(ConfigItem.THEME, newTheme);
    }

    get transcriptions(): string[] {
        return this.#transcriptions;
    }

    addTranscription(transcript: string) {
        this.#transcriptions.push(transcript);
        this.fileStore?.set(ConfigItem.TRANSCRIPTS, this.#transcriptions);
        this.fileStore?.entries().then(x => console.log("TEST", x));
    }

    removeTranscription(index: number) {
        this.#transcriptions.splice(index, 1);
        this.fileStore?.set(ConfigItem.TRANSCRIPTS, this.#transcriptions);
        this.fileStore?.entries().then(x => console.log("TEST", x));
    }
}

export const configStore = new ConfigStore();