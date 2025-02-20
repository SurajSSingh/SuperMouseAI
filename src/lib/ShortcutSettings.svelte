<script lang="ts">
    import {
        isRegistered,
        register,
        unregister,
        unregisterAll,
        type ShortcutEvent,
    } from "@tauri-apps/plugin-global-shortcut";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import Button from "./components/ui/button/button.svelte";
    import type { NotificationSystem } from "./notificationSystem.svelte";

    interface ShortcutsProps {
        notifier?: NotificationSystem;
        onToggleShortcutEvent?: () => void;
    }

    let { notifier, onToggleShortcutEvent }: ShortcutsProps = $props();

    let clickEventUnlistener: UnlistenFn | null = null;

    // States
    let modKeyHeld = $state(false);
    let keyboardShortcut = $state("CommandOrControl+Shift+R");
    let leftMouseKeyShortcut = $state("Alt+Space");

    let previousKeyShortcut = $state("");
    let previousLMBShortcut = $state("");

    async function setupShortcuts() {
        // Setup Mouse press event
        console.log(clickEventUnlistener);
        if (clickEventUnlistener === null) {
            console.log("Regis click");
            clickEventUnlistener = await listen("mouse_press", (_e) => {
                console.log(_e);
                if (modKeyHeld) {
                    onToggleShortcutEvent?.();
                }
            });
        }
        // Register Keyboard only shortcut
        if (!(await isRegistered(keyboardShortcut))) {
            await registerNewShortcutWithMouse(false);
        }
        // Register Mouse+Keyboard shortcut
        if (!(await isRegistered(leftMouseKeyShortcut))) {
            await registerNewShortcutWithMouse(true);
        }
    }

    function hotkeyEvent(event: ShortcutEvent) {
        if (event.state === "Released") {
            onToggleShortcutEvent?.();
        }
    }

    function mouseKeyEvent(event: ShortcutEvent) {
        modKeyHeld = event.state === "Pressed";
    }

    async function cleanupShortcuts() {
        unregister([keyboardShortcut, leftMouseKeyShortcut]).catch((_err) =>
            notifier?.showAlert(`Could not unregister the shortcuts!`),
        );
    }

    async function registerNewShortcutWithMouse(useMouse: boolean) {
        // TODO: Reduce nesting and make more modular (too much repeating common code)
        if (useMouse) {
            leftMouseKeyShortcut = leftMouseKeyShortcut.trim();
            if (
                await isRegistered(leftMouseKeyShortcut).catch(
                    () =>
                        `Could not check if ${leftMouseKeyShortcut} is registered (check for spelling issues)`,
                )
            ) {
                previousLMBShortcut = leftMouseKeyShortcut;
                return;
            }
            // Register new before unregistering old in case of error
            await register(leftMouseKeyShortcut, mouseKeyEvent).catch((_err) =>
                notifier?.showAlert(
                    `Could not register the shortcut: ${leftMouseKeyShortcut} (may have spelling issues or already be registered)`,
                ),
            );
            if (previousLMBShortcut) {
                await unregister(previousLMBShortcut).then(
                    () => {
                        previousLMBShortcut = leftMouseKeyShortcut;
                    },
                    (_err) =>
                        notifier?.showAlert(
                            `Could not unregister the shortcut: ${previousLMBShortcut} (may have spelling issues or already be registered)`,
                        ),
                );
            } else {
                // Snapshot the current so if user changes, we can unregister
                previousLMBShortcut = leftMouseKeyShortcut;
            }
        } else {
            keyboardShortcut = keyboardShortcut.trim();
            if (
                await isRegistered(keyboardShortcut).catch(
                    () =>
                        `Could not check if ${keyboardShortcut} is registered (check for spelling issues)`,
                )
            ) {
                previousKeyShortcut = keyboardShortcut;
                return;
            }
            // Register new before unregistering old in case of error
            await register(keyboardShortcut, hotkeyEvent).catch((_err) =>
                notifier?.showAlert(
                    `Could not register the shortcut: ${keyboardShortcut} (check for spelling issues)`,
                ),
            );

            if (previousKeyShortcut) {
                await unregister(previousKeyShortcut).then(
                    () => {
                        previousKeyShortcut = keyboardShortcut;
                    },
                    (_err) =>
                        notifier?.showAlert(
                            `Could not unregister the shortcut: ${previousKeyShortcut} (check for spelling issues)`,
                        ),
                );
            } else {
                // Snapshot the current so if user changes, we can unregister
                previousKeyShortcut = keyboardShortcut;
            }
        }
    }

    // Effect
    $effect(() => {
        // On-Mount
        setupShortcuts();

        return async () => {
            // Clean-up code
            if (clickEventUnlistener) clickEventUnlistener();
            clickEventUnlistener = null;
            await cleanupShortcuts();
            await unregisterAll();
        };
    });
</script>

<h2 class="text-md text-center">Configure Shortcuts</h2>
<fieldset class="fieldset bg-base-200 border border-base-300 p-4 rounded-box">
    <legend class="fieldset-legend">Keyboard Only</legend>
    <input
        type="text"
        name="key-shortcut"
        id="key-shortcut"
        bind:value={keyboardShortcut}
        class={`p-1 rounded-sm border-1 input ${keyboardShortcut !== previousKeyShortcut ? "input-warning" : "input-success"}`}
    />
    <p class="fieldset-label">
        When keys are pressed anywhere, it will toggle recording.
    </p>
</fieldset>
<fieldset class="fieldset bg-base-200 border border-base-300 p-4 rounded-box">
    <legend class="fieldset-legend">Left Click + Keyboard</legend>
    <input
        type="text"
        name="mouse-shortcut"
        id="mouse-shortcut"
        bind:value={leftMouseKeyShortcut}
        class={`p-1 rounded-sm border-1 input ${leftMouseKeyShortcut !== previousLMBShortcut ? "input-warning" : "input-success"}`}
    />
    <p class="fieldset-label">
        When left mouse button is clicked with these keys held down, it will
        toggle the recording.
    </p>
</fieldset>

<Button
    onclick={() => {
        if (leftMouseKeyShortcut !== previousLMBShortcut)
            registerNewShortcutWithMouse(true);
        if (keyboardShortcut !== previousKeyShortcut)
            registerNewShortcutWithMouse(false);
    }}
    color="secondary">Register All</Button
>
