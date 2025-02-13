<script lang="ts">
    import {
        register as registerShortcut,
        unregisterAll as unregisterAllShortcuts,
    } from "@tauri-apps/plugin-global-shortcut";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";

    interface ShortcutsProps {
        onToggleShortcutEvent?: () => void;
    }

    let { onToggleShortcutEvent }: ShortcutsProps = $props();

    let modKeyHeld = false;
    let clickEventUnlistener: UnlistenFn;
    let modEventUnlistener: UnlistenFn;

    // Effect
    $effect(() => {
        // On-Mount
        const setupShortcuts = async () => {
            await registerShortcut("CommandOrControl+Shift+R", (event) => {
                if (event.state === "Released") {
                    onToggleShortcutEvent?.();
                }
            });
            clickEventUnlistener = await listen("mouse_press", (_e) => {
                if (modKeyHeld) {
                    onToggleShortcutEvent?.();
                }
            });
            modEventUnlistener = await listen("mod_key_event", (e) => {
                modKeyHeld = e.payload === "Pressed";
            });
        };
        setupShortcuts();

        return async () => {
            // Clean-up code
            clickEventUnlistener();
            modEventUnlistener();
            unregisterAllShortcuts();
        };
    });
</script>
