import { load, type Store } from "@tauri-apps/plugin-store";
import type { ThemeKind } from "./types.ts";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { debug, error, info, trace, warn } from "@tauri-apps/plugin-log";

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
  INTER_SENTENCE_NEWLINE_REMOVE: "remove_newline_inside_sentence",
  AUTO_PASTE: "paste_after_transcribe",
  PASTE_VIA_KEYBOARD: "use_keys_to_paste",
  UPDATES_NOTIFY: "notify_of_updates",
  AUTO_CHECK_UPDATES: "check_for_updates",
  AUTO_APPLY_UPDATES: "auto_download_and_install_update",
};

class StoreStateOption<T> {
  #value = $state() as T;
  #name;
  #config: Store | undefined;

  constructor(initial: T, name: string) {
    this.#value = initial;
    this.#name = name;
    trace(
      `Construct new runic store named ${this.#name} with value = ${this.#value}`,
    );
  }

  async loadFrom(store: Store): Promise<void> {
    debug(`Load value from new store`);
    this.#config = store;
    const value = await this.#config.get<T>(this.#name);
    if (value !== undefined) this.#value = value;
  }

  async saveToStore(): Promise<void> {
    await this.#config?.set(this.#name, this.#value);
  }

  get value(): T {
    return this.#value;
  }

  set value(v: T) {
    this.#value = v;
    this.saveToStore();
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
  shortcut = new StoreStateOption<string>(
    "Shift+Alt+KeyR",
    ConfigItem.SHORTCUT,
  );
  threads = new StoreStateOption<number>(1, ConfigItem.THREADS);
  enabledSound = new StoreStateOption<boolean>(true, ConfigItem.SOUND);
  testNotify = new StoreStateOption<boolean>(true, ConfigItem.TEST_NOTIFY);
  useSystemNotification = new StoreStateOption<boolean>(
    true,
    ConfigItem.SYSTEM_NOTIFY,
  );
  initialPrompt = new StoreStateOption<string>("", ConfigItem.PROMPT);
  ignoredWords = new StoreStateOption<string>(
    "[BLANK_AUDIO]\n[NO_AUDIO]\n[SILENCE]",
    ConfigItem.IGNORED_WORDS,
  );
  windowFloat = new StoreStateOption<boolean>(false, ConfigItem.FLOAT_WINDOW);
  interNLRemove = new StoreStateOption<boolean>(
    true,
    ConfigItem.INTER_SENTENCE_NEWLINE_REMOVE,
  );
  autoPaste = new StoreStateOption<boolean>(true, ConfigItem.AUTO_PASTE);
  pasteViaKeys = new StoreStateOption<boolean>(
    true,
    ConfigItem.PASTE_VIA_KEYBOARD,
  );
  notifyOfUpdates = new StoreStateOption<boolean>(
    true,
    ConfigItem.UPDATES_NOTIFY,
  );
  autoCheckForUpdates = new StoreStateOption<boolean>(
    true,
    ConfigItem.AUTO_CHECK_UPDATES,
  );
  autoApplyUpdates = new StoreStateOption<boolean>(
    false,
    ConfigItem.AUTO_APPLY_UPDATES,
  );

  /** Array of all fields in class that are configuration optiosn */
  #configFields = [
    this.theme,
    this.transcriptions,
    this.currentIndex,
    this.shortcut,
    this.threads,
    this.enabledSound,
    this.testNotify,
    this.useSystemNotification,
    this.initialPrompt,
    this.ignoredWords,
    this.windowFloat,
    this.interNLRemove,
    this.autoPaste,
    this.notifyOfUpdates,
    this.autoCheckForUpdates,
    this.autoApplyUpdates,
  ] as const;

  // Derived values
  transcriptLength = $derived(this.transcriptions.value.length);
  isTranscriptsEmpty = $derived(this.transcriptions.value.length === 0);
  currentTranscript = $derived(
    this.transcriptions.value[this.currentIndex.value],
  );
  ignoredWordsList = $derived(this.ignoredWords.value.split("\n"));
  // NOTE: A main key will alway exist for a valid shortcut
  // deno-lint-ignore no-non-null-assertion
  mainKey = $derived(this.shortcut.value.split("+").at(-1)!);
  modifierKeys = $derived({
    hasAlt: this.shortcut.value.includes("Alt"),
    hasControl: this.shortcut.value.includes("Control"),
    hasShift: this.shortcut.value.includes("Shift"),
    hasSuper: this.shortcut.value.includes("Super"),
  });

  #isReady = false;
  constructor() {
    this.cleanup = $effect.root(() => {
      $effect(() => {
        const loadFile = async () => {
          debug(`Load store from file`);
          this.fileStore = await load("super-mouse-ai.json", {
            autoSave: AUTO_SAVE_FREQUENCY,
          });
          await this.loadAll();
          this.#isReady = true;
        };
        loadFile();
        return () => {
          debug(`Clean up store to file`);
          this.keyChangeUnlistener?.();
          this.fileStore?.close();
        };
      });
    });
  }

  /** Wait until store is loaded */
  async waitForStoreLoaded(): Promise<void> {
    // HACK: Works, but would be better to follow actual loading resolve
    while (!this.#isReady) {
      // deno-lint-ignore no-await-in-loop
      await new Promise((resolve) => setTimeout(resolve, 100));
      trace(`Waiting for store to load`);
    }
  }

  /** Load all items from store for states */
  private async loadAll(): Promise<void> {
    if (!this.fileStore) {
      error(`No store provided`);
      return;
    }
    if (!this.fileStore.has(ConfigItem.VERSION)) {
      warn(
        `Store is unversioned, this may not be valid, setting to current version`,
      );
      await this.fileStore.set(ConfigItem.VERSION, this.#version);
    } else {
      this.#version = await this.fileStore.get(ConfigItem.VERSION) ??
        this.#version;
      info(`Loading store with version = ${this.#version}`);
    }
    const store = this.fileStore;

    debug(`Load values in config into each field in store`);
    // Load all config from file store
    await Promise.allSettled(
      this.#configFields.map((field) => field.loadFrom(store)),
    ).catch((err) => {
      error("Failed to load from store with given error: ", err);
    });
    this.keyChangeUnlistener = await this.fileStore.onChange((k, v) =>
      trace(`STORE CHANGE: ${k} => ${v}`)
    );
    trace(
      `Current store data: ${
        JSON.stringify(await this.fileStore.entries(), null, 2)
      }`,
    );
  }

  saveAll(): Promise<PromiseSettledResult<void>[]> {
    debug(`Save all configured options to file`);
    return Promise.allSettled(
      this.#configFields.map((field) => field.saveToStore()),
    );
  }

  clearTranscripts(): void {
    debug(`Clear all transcripts`);
    this.fileStore?.delete(ConfigItem.TRANSCRIPTS);
    this.currentIndex.value = 0;
    this.transcriptions.value = [];
  }

  clearData(): void {
    debug(`Clear all app data`);
    this.fileStore?.reset();
  }

  // Convience Methods for config values

  get version(): number {
    return this.#version;
  }

  addTranscription(transcript: string): void {
    debug(`Add new transcript with size: ${transcript.length}`);
    // HACK: Issue with proxing array in classes, don't fully understand why yet
    this.transcriptions.value = [...this.transcriptions.value, transcript];
    this.currentIndex.value = this.transcriptLength - 1;
  }

  editTranscription(edited: string): void {
    debug(`Update transcript at ${this.currentIndex.value}`);
    // HACK: Issue with proxing array in classes, don't fully understand why yet
    this.transcriptions.value = this.transcriptions.value.map((
      original,
      index,
    ) => index === this.currentIndex.value ? edited : original);
  }

  removeCurrentTranscription(): void {
    debug(`Remove transcript at ${this.currentIndex.value}`);
    // HACK: Issue with proxing array in classes, don't fully understand why yet
    this.transcriptions.value = this.transcriptions.value.filter((_, i) =>
      i !== this.currentIndex.value
    );
    if (this.currentIndex.value >= this.transcriptLength) {
      this.currentIndex.value = this.transcriptLength - 1;
    }
    if (this.isTranscriptsEmpty) this.currentIndex.value = 0;
  }

  prevIndex(): void {
    if (this.currentIndex.value <= 0) {
      warn(`Cannot go to a previous index before the first index`);
      return;
    }
    debug(
      `Set current index from ${this.currentIndex.value} to ${
        this.currentIndex.value - 1
      }`,
    );
    this.currentIndex.value--;
  }

  nextIndex(): void {
    if (this.currentIndex.value >= this.transcriptLength - 1) {
      warn(`Cannot go to a next index after the last index`);
      return;
    }
    debug(
      `Set current index from ${this.currentIndex.value} to ${
        this.currentIndex.value + 1
      }`,
    );
    this.currentIndex.value++;
  }
}

export const configStore = new ConfigStore();
