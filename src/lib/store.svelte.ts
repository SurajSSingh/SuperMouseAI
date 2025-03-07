import { load, Store } from '@tauri-apps/plugin-store';
import type { ThemeKind } from './types';

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

}

export class ConfigStore {
    // Store items
    fileStore: Store | null = null;
    cleanup: () => void;
    #version = 1;

    // Private Config data
    #theme: ThemeKind = $state("system");
    #transcriptions: string[] = $state([]);
    #currentIndex: number = $state(0);
    #shortcut = $state("")
    #threads = $state(1)
    #enableSound = $state(true);
    #testNotify = $state(true);
    #initialPrompt = $state("")
    #ignoredWords = $state("[BLANK_AUDIO]\n[NO_AUDIO]\n[SILENCE]");

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
        this.#version = await this.fileStore.get(ConfigItem.VERSION) ?? this.#version;
        this.#theme = await this.fileStore.get(ConfigItem.THEME) ?? this.#theme;
        this.#transcriptions = await this.fileStore.get(ConfigItem.TRANSCRIPTS) ?? this.#transcriptions;
        this.#currentIndex = await this.fileStore.get(ConfigItem.INDEX) ?? this.#currentIndex;
        this.#enableSound = await this.fileStore.get(ConfigItem.SOUND) ?? this.#enableSound;
        this.#testNotify = await this.fileStore.get(ConfigItem.TEST_NOTIFY) ?? this.#testNotify;
        this.#initialPrompt = await this.fileStore.get(ConfigItem.PROMPT) ?? this.#initialPrompt;
        this.#ignoredWords = await this.fileStore.get(ConfigItem.IGNORED_WORDS) ?? this.#ignoredWords;
        console.log(await this.fileStore.entries());
    }

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

    get mainKey(): string {
        // NOTE: A main key will alawy exist for a valid shortcut
        return this.#shortcut.split("+").at(-1)!;
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
        this.#shortcut = newShortcut;
        this.fileStore?.set(ConfigItem.SHORTCUT, this.#shortcut);
    }

    get thread(): number {
        return this.#threads
    }

    set thread(newCount: number) {
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

    get ignoredWordsList(): string[] {
        return this.#ignoredWords.split("\n");
    }

    set ignoredWords(newIgnored: string) {
        this.#ignoredWords = newIgnored;
        this.fileStore?.set(ConfigItem.IGNORED_WORDS, this.#ignoredWords);
    }
}

export const configStore = new ConfigStore();