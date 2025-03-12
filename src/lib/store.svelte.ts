import { load, Store } from '@tauri-apps/plugin-store';
import type { ThemeKind } from './types';
import type { UnlistenFn } from '@tauri-apps/api/event';

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

}

class StoreOption<T> {
    #value;
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
    #theme: ThemeKind = $state("system");
    #transcriptions: string[] = $state([]);
    #currentIndex: number = $state(0);
    #shortcut = $state("Shift+Alt+KeyR")
    #threads = $state(1)
    #enableSound = $state(true);
    #testNotify = $state(true);
    #useSystemNotification = $state(true)
    #initialPrompt = $state("")
    #ignoredWords = $state("[BLANK_AUDIO]\n[NO_AUDIO]\n[SILENCE]");

    // Derived values
    transcriptLength = $derived(this.#transcriptions.length);
    isTranscriptsEmpty = $derived(this.#transcriptions.length === 0);
    currentTranscript = $derived(this.#transcriptions[this.#currentIndex]);
    ignoredWordsList = $derived(this.#ignoredWords.split("\n"));
    // NOTE: A main key will alawy exist for a valid shortcut
    mainKey = $derived(this.#shortcut.split("+").at(-1)!);

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
        this.#theme = await this.fileStore.get(ConfigItem.THEME) ?? this.#theme;
        this.#transcriptions = await this.fileStore.get(ConfigItem.TRANSCRIPTS) ?? this.#transcriptions;
        this.#currentIndex = this.transcriptLength > 0 ? await this.fileStore.get(ConfigItem.INDEX) ?? this.#currentIndex : 0;
        this.#shortcut = await this.fileStore.get(ConfigItem.SHORTCUT) || this.#shortcut;
        this.#threads = await this.fileStore.get(ConfigItem.THREADS) ?? this.#threads;
        this.#enableSound = await this.fileStore.get(ConfigItem.SOUND) ?? this.#enableSound;
        this.#testNotify = await this.fileStore.get(ConfigItem.TEST_NOTIFY) ?? this.#testNotify;
        this.#initialPrompt = await this.fileStore.get(ConfigItem.PROMPT) ?? this.#initialPrompt;
        this.#ignoredWords = await this.fileStore.get(ConfigItem.IGNORED_WORDS) ?? this.#ignoredWords;
        this.keyChangeUnlistener = await this.fileStore.onChange((k, v) => console.log(`STORE CHANGE: ${k} => ${v}`));
        console.dir(await this.fileStore.entries());
    }

    clearTranscripts() {
        this.fileStore?.delete(ConfigItem.TRANSCRIPTS);
        this.currentTranscriptionIndex = 0;
        this.#transcriptions = []
    }

    clearData() {
        this.fileStore?.reset()
    }

    // Properties and Functions for config values

    get version(): number {
        return this.#version;
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
        this.currentTranscriptionIndex = this.transcriptLength - 1;
    }

    editTranscription(edited: string) {
        this.#transcriptions[this.#currentIndex] = edited;
        this.fileStore?.set(ConfigItem.TRANSCRIPTS, this.#transcriptions);
    }

    removeCurrentTranscription() {
        this.#transcriptions.splice(this.#currentIndex, 1);
        this.fileStore?.set(ConfigItem.TRANSCRIPTS, this.#transcriptions);
        if (this.#currentIndex >= this.transcriptLength) this.currentTranscriptionIndex = this.transcriptLength - 1;
        if (this.isTranscriptsEmpty) this.currentTranscriptionIndex = 0;
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
    }

    nextIndex() {
        if (this.#currentIndex >= this.transcriptLength - 1) return;
        this.#currentIndex++;
        this.fileStore?.set(ConfigItem.INDEX, this.#currentIndex);
    }

    get shortcut(): string {
        return this.#shortcut
    }

    get modifierKeys(): {
        hasAlt: boolean,
        hasControl: boolean,
        hasShift: boolean,
        hasSuper: boolean,
    } {
        return {
            hasAlt: this.#shortcut.includes("Alt"),
            hasControl: this.#shortcut.includes("Control"),
            hasShift: this.#shortcut.includes("Shift"),
            hasSuper: this.#shortcut.includes("Super"),
        }
    }

    set shortcut(newShortcut: string) {
        this.#shortcut = newShortcut || this.#shortcut;
        this.fileStore?.set(ConfigItem.SHORTCUT, this.#shortcut);
    }

    get threads(): number {
        return this.#threads
    }

    set threads(newCount: number) {
        this.#threads = newCount;
        this.fileStore?.set(ConfigItem.THREADS, this.#threads);
    }

    get enabledSound(): boolean {
        return this.#enableSound
    }

    set enabledSound(enabled: boolean) {
        this.#enableSound = enabled;
        this.fileStore?.set(ConfigItem.SOUND, this.#enableSound);
    }

    get testNotify(): boolean {
        return this.#testNotify
    }

    set testNotify(willNotify: boolean) {
        this.#testNotify = willNotify;
        this.fileStore?.set(ConfigItem.TEST_NOTIFY, this.#testNotify);
    }

    get initialPrompt(): string {
        return this.#initialPrompt
    }

    set initialPrompt(newPrompt: string) {
        this.#initialPrompt = newPrompt;
        this.fileStore?.set(ConfigItem.PROMPT, this.#initialPrompt);
    }

    get ignoredWords(): string {
        return this.#ignoredWords;
    }

    set ignoredWords(newIgnored: string) {
        this.#ignoredWords = newIgnored;
        this.fileStore?.set(ConfigItem.IGNORED_WORDS, this.#ignoredWords);
    }

    get useSystemNotification(): boolean {
        return this.#useSystemNotification;
    }

    set useSystemNotification(newUse: boolean) {
        this.#useSystemNotification = newUse;
        this.fileStore?.set(ConfigItem.SYSTEM_NOTIFY, this.#useSystemNotification);
    }
}

export const configStore = new ConfigStore();