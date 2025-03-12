import { load, Store } from '@tauri-apps/plugin-store';
import type { ThemeKind } from './types';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { warn } from '@tauri-apps/plugin-log';

/** Auto-save every given millisecond, or never if set to `false` */
const AUTO_SAVE_FREQUENCY: false | number = 2000;

/**
 * An "Enum" for config item to allow auto-completion
 * and decouple from string representation.
 */
export const ConfigItem = {
    VERSION: "version",
    THEME: "theme",
    TRANSCRIPTS: "transcripts",
    INDEX: "index",
    SHORTCUT: "shortcut",
    THREADS: "threads",
    PROMPT: "prompt",
    IGNORED_WORDS: "ignored",
    SOUND: "sound",
    TEST_NOTIFY: "notify_on_load",
    SYSTEM_NOTIFY: "use_system_notification",
    FLOAT_WINDOW: "window_always_on_top",

}

class StoreStateOption<T> {
    #value = $state() as T;
    #name;
    #config: Store | undefined;

    constructor(initial: T, name: string) {
        this.#value = initial;
        this.#name = name;
    }

    async loadFrom(store: Store) {
        this.#config = store;
        const value = await this.#config.get<T>(this.#name);
        if (value) this.#value = value
    }

    async saveToStore() {
        await this.#config?.set(this.#name, this.#value)
    }

    get value(): T {
        return this.#value;
    }

    set value(v: T) {
        this.#value = v;
        this.saveToStore()
    }

    get name(): string {
        return this.#name;
    }
}

export class ConfigStore {
    // Store items
    fileStore: Store | null = null;
    cleanup: () => void;
    keyChangeUnlistener: UnlistenFn | undefined;
    #version = 1;

    // Private Config data
    theme = new StoreStateOption<ThemeKind>("system", ConfigItem.THEME);
    transcriptions = new StoreStateOption<string[]>([], ConfigItem.TRANSCRIPTS);
    currentIndex = new StoreStateOption<number>(0, ConfigItem.INDEX);
    shortcut = new StoreStateOption<string>("Shift+Alt+KeyR", ConfigItem.SHORTCUT);
    threads = new StoreStateOption<number>(1, ConfigItem.THREADS);
    enabledSound = new StoreStateOption<boolean>(true, ConfigItem.SOUND);
    testNotify = new StoreStateOption<boolean>(true, ConfigItem.TEST_NOTIFY);
    useSystemNotification = new StoreStateOption<boolean>(true, ConfigItem.SYSTEM_NOTIFY);
    initialPrompt = new StoreStateOption<string>("", ConfigItem.PROMPT);
    ignoredWords = new StoreStateOption<string>("[BLANK_AUDIO]\n[NO_AUDIO]\n[SILENCE]", ConfigItem.IGNORED_WORDS);
    windowFloat = new StoreStateOption<boolean>(false, ConfigItem.FLOAT_WINDOW);

    #configFields = [
        this.theme,
        this.transcriptions,
        this.currentIndex,
        this.shortcut,
        this.threads,
        this.enabledSound,
        this.testNotify,
        this.initialPrompt,
        this.ignoredWords,
        this.windowFloat,
    ] as const;

    // Derived values
    transcriptLength = $derived(this.transcriptions.value.length);
    isTranscriptsEmpty = $derived(this.transcriptions.value.length === 0);
    currentTranscript = $derived(this.transcriptions.value[this.currentIndex.value]);
    ignoredWordsList = $derived(this.ignoredWords.value.split("\n"));
    // NOTE: A main key will alawy exist for a valid shortcut
    mainKey = $derived(this.shortcut.value.split("+").at(-1)!);
    modifierKeys = $derived({
        hasAlt: this.shortcut.value.includes("Alt"),
        hasControl: this.shortcut.value.includes("Control"),
        hasShift: this.shortcut.value.includes("Shift"),
        hasSuper: this.shortcut.value.includes("Super"),
    })

    #isReady: boolean = false;
    constructor() {
        this.cleanup = $effect.root(() => {
            $effect(() => {
                const loadFile = async () => {
                    this.fileStore = await load('super-mouse-ai.json', { autoSave: AUTO_SAVE_FREQUENCY });
                    await this.loadAll();
                    this.#isReady = true;
                }
                loadFile();
                return () => {
                    this.keyChangeUnlistener?.();
                    this.fileStore?.close()
                }
            })
        });
    }

    /** Wait until store is loaded */
    async waitForStoreLoaded() {
        // HACK: Works, but would be better to follow actual loading resolve
        while (!this.#isReady) {
            await new Promise(resolve => setTimeout(resolve, 100));
        }
    }

    /** Load all items from store for states */
    private async loadAll() {
        if (!this.fileStore) return;
        if (!this.fileStore.has(ConfigItem.VERSION)) {
            await this.fileStore.set(ConfigItem.VERSION, this.#version);
        }
        else {
            this.#version = await this.fileStore.get(ConfigItem.VERSION) ?? this.#version;
        }
        // Load all config from file store
        await Promise.allSettled(
            this.#configFields.map(field => field.loadFrom(this.fileStore!))
        ).catch((err) => {
            warn("Failed to load from store with given error: ", err)
        })
        this.keyChangeUnlistener = await this.fileStore.onChange((k, v) => console.log(`STORE CHANGE: ${k} => ${v}`));
        console.dir(await this.fileStore.entries());
    }

    async saveAll() {
        return Promise.allSettled(
            this.#configFields.map(field => field.saveToStore())
        )
    }

    clearTranscripts() {
        this.fileStore?.delete(ConfigItem.TRANSCRIPTS);
        this.currentIndex.value = 0;
        this.transcriptions.value = []
    }

    clearData() {
        this.fileStore?.reset()
    }

    // Convience Methods for config values

    get version(): number {
        return this.#version;
    }

    addTranscription(transcript: string) {
        // HACK: Issue with proxing array in classes, don't fully understand why yet
        this.transcriptions.value = [...this.transcriptions.value, transcript];
        this.currentIndex.value = this.transcriptLength - 1;
    }

    editTranscription(edited: string) {
        // HACK: Issue with proxing array in classes, don't fully understand why yet
        this.transcriptions.value = this.transcriptions.value.map((original, index) => index === this.currentIndex.value ? edited : original)
    }

    removeCurrentTranscription() {
        // HACK: Issue with proxing array in classes, don't fully understand why yet
        this.transcriptions.value = this.transcriptions.value.filter((_, i) => i !== this.currentIndex.value)
        if (this.currentIndex.value >= this.transcriptLength) this.currentIndex.value = this.transcriptLength - 1;
        if (this.isTranscriptsEmpty) this.currentIndex.value = 0;
    }

    prevIndex() {
        if (this.currentIndex.value <= 0) return;
        this.currentIndex.value--;
    }

    nextIndex() {
        if (this.currentIndex.value >= this.transcriptLength - 1) return;
        this.currentIndex.value++;
    }
}

export const configStore = new ConfigStore();