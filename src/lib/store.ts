import { load, Store } from '@tauri-apps/plugin-store';

/** Auto-save every given millisecond, or never if set to `false` */
const AUTO_SAVE_FREQUENCY: false | number = 2000;

class ConfigStore {
    fileStore: Store | null = null;

    async loadStore(): Promise<ConfigStore> {
        this.fileStore = await load('super-mouse-ai.json', { autoSave: AUTO_SAVE_FREQUENCY });
        return this;
    }

    async getTheme(): Promise<"system" | "light" | "dark" | undefined> {
        if (!this.fileStore) await this.loadStore();
        return await this.fileStore!.get("theme");
    }

    async setTheme(theme: "system" | "light" | "dark") {
        if (!this.fileStore) await this.loadStore();
        return await this.fileStore!.set("theme", theme);
    }

    async getPrevTranscriptions(): Promise<string[] | undefined> {
        if (!this.fileStore) await this.loadStore();
        return await this.fileStore!.get("prev");
    }

    async setPrevTranscriptions(transcriptions: string[]) {
        if (!this.fileStore) await this.loadStore();
        return await this.fileStore!.set("prev", transcriptions);
    }

    async saveAll() {
        await this.fileStore?.save();
    }
}

export const configStore = new ConfigStore();