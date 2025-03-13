<script lang="ts">
    import {
        isRegistered,
        register,
        unregister,
        unregisterAll,
        type ShortcutEvent,
        type ShortcutHandler,
    } from "@tauri-apps/plugin-global-shortcut";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import Button from "$lib/components/ui/button/button.svelte";
    import {
        notifier,
        type NotificationSystem,
    } from "$lib/notificationSystem.svelte";

    interface ShortcutsProps {
        onToggleShortcutEvent?: () => void;
    }

    let { onToggleShortcutEvent }: ShortcutsProps = $props();

    type ShowShortcutFn = <T>(shortcut: string, returns: T) => T;

    const showShortcutFindingError: ShowShortcutFn = (
        shortcut: string,
        returns,
    ) => {
        notifier?.showError(
            `Could not check if ${shortcut} is registered (check for spelling issues)`,
            "Shortcut Finding Error",
        );
        return returns;
    };
    const showShortcutRegistrationError: ShowShortcutFn = (
        shortcut: string,
        returns,
    ) => {
        notifier?.showError(
            `Could not register the shortcut: ${shortcut} (may have spelling issues or already be registered)`,
            "Shortcut Registration Error",
        );
        return returns;
    };
    const showShortcutUnregistrationError: ShowShortcutFn = (
        shortcut: string,
        returns,
    ) => {
        notifier?.showError(
            `Could not unregister the shortcut: ${shortcut} (may have spelling issues or already be unregistered)`,
            "Shortcut Unregister Error",
        );
        return returns;
    };
    const showShortcutRegistrationSuccess: ShowShortcutFn = (
        shortcut: string,
        returns,
    ) => {
        notifier?.showSuccess(`Shortcut ${shortcut} has been registered`);
        return returns;
    };

    let clickEventUnlistener: UnlistenFn | null = null;
    let modKeyEventUnlistener: UnlistenFn | null = null;

    // States
    let modKeyHeld = $state(false);
    let keyboardShortcut = $state("CommandOrControl+Shift+R");
    let leftMouseKeyShortcut = $state("Alt+Space");

    let previousKeyShortcut = $state("");
    let previousLMBShortcut = $state("");

    // Semi-derived
    // TODO: Refactor this to be entirely derived somehow
    let hasHotKeyError = $state(false);
    let hasMouseKeyError = $state(false);

    // Derived
    const hotkeyShortcutChanged = $derived(
        keyboardShortcut !== previousKeyShortcut,
    );
    const mouseShortcutChanged = $derived(
        leftMouseKeyShortcut !== previousLMBShortcut,
    );

    async function setupShortcuts() {
        // Setup Mouse press event
        if (clickEventUnlistener === null) {
            clickEventUnlistener = await listen("mouse_press", (_e) => {
                console.log(_e);
                if (modKeyHeld) {
                    onToggleShortcutEvent?.();
                }
            });
        }
        // Mod Key Event
        if (modKeyEventUnlistener === null) {
            modKeyEventUnlistener = await listen("mod_key", (_p) => {
                console.log(_p);
            });
        }
        // Register Keyboard only shortcut
        if (hotkeyShortcutChanged || hasHotKeyError) {
            const prev = previousKeyShortcut;
            previousKeyShortcut = await registerNewShortcut(
                keyboardShortcut,
                hotkeyEvent,
                previousKeyShortcut,
            );
            hasHotKeyError =
                prev === previousKeyShortcut &&
                previousKeyShortcut !== keyboardShortcut;
        }
        // Register Mouse+Keyboard shortcut
        if (mouseShortcutChanged || hasMouseKeyError) {
            const prev = previousLMBShortcut;
            previousLMBShortcut = await registerNewShortcut(
                leftMouseKeyShortcut,
                mouseKeyEvent,
                previousLMBShortcut,
            );
            hasMouseKeyError =
                prev === previousLMBShortcut &&
                previousLMBShortcut !== leftMouseKeyShortcut;
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

    /**
     *
     * @param shortcut new shortcut to register
     * @param handler callback to run for shortcut
     * @param previousShortcut optional previous shortcut to unregister
     * @return previous shortcut
     */
    async function registerNewShortcut(
        shortcut: string,
        handler: ShortcutHandler,
        previousShortcut?: string,
    ): Promise<string> {
        shortcut = shortcut.trim();
        previousShortcut = previousShortcut ?? "";
        if (
            await isRegistered(shortcut).catch((_err) =>
                // Default to skip registration on error,
                // so it doesn't cause issues for shortcuts
                showShortcutFindingError(shortcut, true),
            )
        ) {
            return previousShortcut;
        }
        return await register(shortcut, handler).then(
            (_success) =>
                (previousShortcut
                    ? // Unregister when there is a previousShortcut
                      unregister(previousShortcut)
                    : // or just resolve immediately
                      Promise.resolve()
                ).then(
                    (_success) =>
                        showShortcutRegistrationSuccess(shortcut, shortcut),
                    (_failure) =>
                        // Show failure, but still return new shortcut,
                        // since it has been registered
                        showShortcutUnregistrationError(
                            previousShortcut,
                            shortcut,
                        ),
                ),
            (_failure) =>
                // Stop to show failure and return previous shortcut
                showShortcutRegistrationError(shortcut, previousShortcut),
        );
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
        class={`p-1 rounded-sm border-1 input ${hasHotKeyError ? "input-error" : hotkeyShortcutChanged ? "input-warning" : "input-success"}`}
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
        class={`p-1 rounded-sm border-1 input ${hasMouseKeyError ? "input-error" : mouseShortcutChanged ? "input-warning" : "input-success"}`}
    />
    <p class="fieldset-label">
        When left mouse button is clicked with these keys held down, it will
        toggle the recording.
    </p>
</fieldset>

<Button onclick={setupShortcuts} color="secondary">Register All</Button>
