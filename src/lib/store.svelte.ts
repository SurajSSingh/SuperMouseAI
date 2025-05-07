import { load, type Store } from "@tauri-apps/plugin-store";
import type {
  ThemeKind,
  TranscriptionInfo,
  WhisperModelInfo,
} from "./types.ts";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { debug, error, info, trace, warn } from "@tauri-apps/plugin-log";
import { open } from "@tauri-apps/plugin-fs";
import { BASE_LOCAL_APP_DIR } from "./constants.ts";

/** Auto-save every given millisecond, or never if set to `false` */
const AUTO_SAVE_FREQUENCY: false | number = 2000;

/**
 * An "Enum" for config item to allow auto-completion
 * and decouple from string representation.
 */
export const ConfigItem = {
  VERSION: "version",
  THEME: "theme",
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
  CURRENT_MODEL: "current_model",
  DOWNLOADED_MODELS: "downloaded_models",
  USE_GPU: "use_gpu",
  PATIENCE: "beam_search_patience",
  AUDIO_STREAM_USE_AUTO_GAIN: "audio_auto_gain",
  AUDIO_STREAM_NOISE_SUPRESS: "audio_noise_suppression",
  AUDIO_STREAM_USE_ECHO_CANCEL: "audio_echo_cancellation",
  AUDIO_PROCESS_DENOISE: "audio_denoise",
  AUDIO_PROCESS_NORMALIZE: "audio_normalize",
  SEND_CRASH_REPORT: "send_crash_report",

  // THIS HAS BEEN DEPRECATED FROM FIELD
  TRANSCRIPTS: "transcripts",
  ENABLE_TELEMETRY: "telemetry_enabled",
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

export class TranscriptStore {
  #fileName;
  #interval;

  value: TranscriptionInfo[] = $state([]);

  constructor(
    fileName: string = "super_mouse_ai_transcriptions",
    autoSave = AUTO_SAVE_FREQUENCY,
  ) {
    this.#fileName = fileName;
    if (autoSave !== false) {
      this.#interval = setInterval(() => this.save(), Math.max(autoSave, 100));
    }
  }

  async load(): Promise<void> {
    trace(`Loading transcriptions from ${this.#fileName}.json`);
    let file;
    try {
      trace(`START transcript load`);
      file = await open(`${this.#fileName}.json`, {
        ...BASE_LOCAL_APP_DIR,
        read: true,
        write: true,
        create: true,
      });
      // FIXME(@): This is wasteful if many loads or large file, currently not a problem,
      // but may want to stream data in
      const buffer = new Uint8Array((await file.stat()).size);
      trace(`LOAD TRANSCRIPTS: Got to BUFFER`);
      await file.read(buffer);
      trace(`LOAD TRANSCRIPTS: Got to READ`);
      const decoder = new TextDecoder("utf-8");
      const text = decoder.decode(buffer.buffer);
      trace(`LOAD TRANSCRIPTS: Got to DECODE`);
      this.value = text.trim() ? JSON.parse(text) : [];
      trace(`LOAD TRANSCRIPTS: Got to PARSE`);
      trace(`TRANSCRIPT VALUE: ${JSON.stringify(this.value)}`);
    } catch (err) {
      error(`Error on loading transcripts: ${err}`);
    } finally {
      file?.close();
    }
  }

  async save(): Promise<boolean> {
    trace(`Saving transcriptions to ${this.#fileName}.json`);
    let file;
    let hasError = false;
    try {
      file = await open(`${this.#fileName}.json`, {
        ...BASE_LOCAL_APP_DIR,
        write: true,
        create: true,
        truncate: true,
      });
      const encoder = new TextEncoder();
      trace(JSON.stringify(this.value));
      //    Write file <- UTF-8 Encode <- JSON string of value
      await file.write(encoder.encode(JSON.stringify(this.value)));
    } // deno-lint-ignore no-explicit-any
    catch (err: any) {
      error(err.toString());
      hasError = true;
    } finally {
      await file?.close();
    }
    return !hasError;
  }

  cleanUp(): void {
    if (this.#interval) {
      debug(`Removing transcript store auto-save interval`);
      clearInterval(this.#interval);
    }
  }
}

export class ConfigStore {
  // Store items
  fileStore: Store | null = null;
  cleanup: () => void;
  keyChangeUnlistener: UnlistenFn | undefined;
  #version = 3;

  // Private Config data
  theme = new StoreStateOption<ThemeKind>("system", ConfigItem.THEME);
  transcriptions = new TranscriptStore();
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
  currentModel = new StoreStateOption<string>(
    "default",
    ConfigItem.CURRENT_MODEL,
  );
  downloadedModels = new StoreStateOption<string[]>(
    [],
    ConfigItem.DOWNLOADED_MODELS,
  );
  useGPU = new StoreStateOption<boolean>(
    true,
    ConfigItem.USE_GPU,
  );
  enableCrashReport = new StoreStateOption<boolean | null>(
    null,
    ConfigItem.SEND_CRASH_REPORT,
  );
  patience = new StoreStateOption<number>(
    2.0,
    ConfigItem.PATIENCE,
  );
  autoGainControl = new StoreStateOption<boolean>(
    false,
    ConfigItem.AUDIO_STREAM_USE_AUTO_GAIN,
  );
  noiseSuppression = new StoreStateOption<boolean>(
    true,
    ConfigItem.AUDIO_STREAM_NOISE_SUPRESS,
  );
  echoCancellation = new StoreStateOption<boolean>(
    true,
    ConfigItem.AUDIO_STREAM_USE_ECHO_CANCEL,
  );
  denoise_audio = new StoreStateOption<boolean>(
    true,
    ConfigItem.AUDIO_PROCESS_DENOISE,
  );
  normalize_result = new StoreStateOption<boolean>(
    true,
    ConfigItem.AUDIO_PROCESS_NORMALIZE,
  );

  // Private config data not backed by file
  includeEnglishOnlyModels = $state({ value: false });
  includeQuantizedModels = $state({ value: false });
  includeObsoleteModels = $state({ value: false });

  /** Array of all fields in class that are configuration optiosn */
  #configFields = [
    this.theme,
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
    this.currentModel,
    this.downloadedModels,
    this.useGPU,
    this.enableCrashReport,
    this.patience,
    this.autoGainControl,
    this.noiseSuppression,
    this.echoCancellation,
    this.denoise_audio,
    this.normalize_result,
  ] as const;

  // Derived values
  transcriptLength = $derived(this.transcriptions.value.length);
  isTranscriptsEmpty = $derived(this.transcriptions.value.length === 0);
  currentTranscript = $derived(
    !this.isTranscriptsEmpty
      ? this.transcriptions.value[this.currentIndex.value].text
      : "",
  );
  currentTranscriptObject = $derived(
    !this.isTranscriptsEmpty
      ? this.transcriptions.value[this.currentIndex.value]
      : null,
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
          this.transcriptions.cleanUp();
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
    let oldVersion = -1;
    const fileVersion = await this.fileStore.get<number>(ConfigItem.VERSION);
    if (!fileVersion || fileVersion < this.#version) {
      if (!fileVersion) {
        warn(
          `Store is unversioned, this may not be valid, setting to current version, and running migration`,
        );
        oldVersion = 0;
      } else {
        info("Will run migration on old config file");
        oldVersion = fileVersion;
      }
    }
    info(`Loading store with version = ${this.#version}`);
    const store = this.fileStore;

    debug(`Load values in config into each field in store`);
    // Load all config from file store
    await Promise.allSettled(
      this.#configFields.map((field) => field.loadFrom(store)),
    ).catch((err) => {
      error("Failed to load from store with given error: ", err);
    });
    if (oldVersion >= 0) await this.transitionDeprecatedOptions(oldVersion);
    this.keyChangeUnlistener = await this.fileStore.onChange((k, v) =>
      trace(`STORE CHANGE: ${k} => ${v}`)
    );
    trace(
      `Current store data: ${
        JSON.stringify(await this.fileStore.entries(), null, 2)
      }`,
    );
  }

  private async transitionDeprecatedOptions(
    previousVersion: number,
  ): Promise<void> {
    if (!this.fileStore) {
      error(`No store provided`);
      return;
    }
    // Old Transcript list ---move to--> New Transcript file
    if (await this.fileStore.has(ConfigItem.TRANSCRIPTS)) {
      info(`Move transcriptions from regular config to dedicated config`);
      trace(
        `OLD TRANSCRIPT: ${
          JSON.stringify(await this.fileStore.get(ConfigItem.TRANSCRIPTS))
        }`,
      );
      this.transcriptions.value =
        (await this.fileStore.get(ConfigItem.TRANSCRIPTS) as string[]).map(
          (text) => {
            return { text, provider: "whisper-cpp" };
          },
        );
      // Save before deleting
      debug("Save migrated transcription before deleting");
      const success = await this.transcriptions.save();
      if (success) {
        debug("Delete old transcript data");
        await this.fileStore.delete(ConfigItem.TRANSCRIPTS);
      }
    }
    await this.transcriptions.load();

    // Old Telemetry -> delete to allow app to ask again
    if (await this.fileStore.has(ConfigItem.ENABLE_TELEMETRY)) {
      this.fileStore.delete(ConfigItem.ENABLE_TELEMETRY);
    }

    // Successfully updated
    if (previousVersion === 0) {
      info(`Successfully checked to v${this.#version}`);
    } else {
      info(
        `Successfully transistioned versions: v${previousVersion} -> v${this.#version}`,
      );
    }
    await this.fileStore.set(ConfigItem.VERSION, this.#version);
  }

  saveAll(): Promise<PromiseSettledResult<void>[]> {
    debug(`Save all configured options to file`);
    this.transcriptions.save();
    return Promise.allSettled(
      this.#configFields.map((field) => field.saveToStore()),
    );
  }

  clearData(): void {
    debug(`Clear all app data`);
    this.fileStore?.reset();
  }

  // Convience Methods for config values

  get version(): number {
    return this.#version;
  }

  addTranscription(transcript: string, processingTime?: number): void {
    debug(`Add new transcript with size: ${transcript.length}`);
    this.transcriptions.value.push({
      text: transcript,
      model: this.currentModel.value,
      provider: "whisper-cpp",
      // TODO(eventually): This may not be accurate if user changes value mid-transcription
      //                   IDEA: Disable while transcribing
      onGPU: this.useGPU.value,
      processingTime,
      // TODO(eventually): React to changes from user
      strategy: {
        type: "beam",
        // TODO(eventually): Get decoder value from store
        beamSize: 5,
        patience: this.patience.value,
      },
    });
    this.currentIndex.value = this.transcriptLength - 1;
  }

  editTranscription(edited: string): void {
    debug(`Update transcript at ${this.currentIndex.value}`);
    this.transcriptions.value[this.currentIndex.value].text = edited;
  }

  removeCurrentTranscription(): void {
    debug(`Remove transcript at ${this.currentIndex.value}`);
    this.transcriptions.value = this.transcriptions.value.filter((_, i) =>
      i !== this.currentIndex.value
    );
    if (this.currentIndex.value >= this.transcriptLength) {
      this.currentIndex.value = this.transcriptLength - 1;
    }
    if (this.isTranscriptsEmpty) this.currentIndex.value = 0;
  }

  clearTranscripts(): void {
    this.transcriptions.value = [];
    this.currentIndex.value = 0;
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

  addModel(model: WhisperModelInfo): void {
    debug(`Add model ${model.relativePath} to config`);
    // HACK: Issue with proxing array in classes, don't fully understand why yet
    this.downloadedModels.value = [
      ...this.downloadedModels.value,
      model.relativePath,
    ];
  }

  removeModel(model: WhisperModelInfo): void {
    debug(`Remove model ${model.relativePath} from config`);
    // HACK: Issue with proxing array in classes, don't fully understand why yet
    this.downloadedModels.value = this.downloadedModels.value.filter(
      (downloaded) => downloaded !== model.relativePath,
    );
  }
}

export const configStore = new ConfigStore();
