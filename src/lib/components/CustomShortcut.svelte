<script lang="ts">
    import {
        isRegistered,
        register,
        unregister,
        unregisterAll,
        type ShortcutHandler,
    } from "@tauri-apps/plugin-global-shortcut";
    import Button from "./ui/button/button.svelte";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import type { NotificationSystem } from "$lib/notificationSystem.svelte";
    import { SvelteSet } from "svelte/reactivity";

    interface CustomShortcutProps {
        notifier?: NotificationSystem;
        onToggleShortcutEvent: ShortcutHandler;
    }

    let { notifier, onToggleShortcutEvent }: CustomShortcutProps = $props();

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

    // https://github.com/tauri-apps/global-hotkey/blob/c9913a97667b3e44cb000de384cd8937d5a0050a/src/hotkey.rs#L199C20-L218C22
    let modAlt: boolean = $state(true);
    let modCtrl: boolean = $state(false);
    let modSuper: boolean = $state(false);
    let modShift: boolean = $state(true);

    let modCurrentSet: SvelteSet<string> = new SvelteSet();
    let mainKey: string = $state("KeyR");
    let isListening: boolean = $state(false);
    let tauriRegistered: boolean = $state(false);
    const LISTENER_BUTTON_ID = "shortcut-listener";
    let previousShortcut = $state("");

    const shortcut = $derived(
        `${modCtrl ? "Control+" : ""}${modShift ? "Shift+" : ""}${modAlt ? "Alt+" : ""}${modSuper ? "Super+" : ""}${mainKey}`,
    );
    const numberOfModKyes = $derived(
        +modCtrl + +modShift + +modAlt + +modSuper,
    );

    // Looking at https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values
    // for code values
    const EXCLUDED_MAIN_KEYS: string[] = [
        "Control",
        "Shift",
        "Alt",
        "Meta",
        "ControlLeft",
        "ShiftLeft",
        "AltLeft",
        "MetaLeft",
        "MetaRight",
        "ShiftRight",
        "AltRight",
    ];

    function onKeyDown(event: KeyboardEvent) {
        if (!isListening) return;
        modAlt = event.altKey;
        modCtrl = event.ctrlKey;
        modShift = event.shiftKey;
        modSuper = event.metaKey;
        const keycode = event.code;
        if (!keycode || EXCLUDED_MAIN_KEYS.includes(keycode)) return;
        mainKey = keycode;
    }

    function mouseNumberToText(button: number): string {
        if (button === 0) {
            return "LeftClick";
        }
        if (button === 1) {
            return "MiddleClick";
        }
        if (button === 2) {
            return "RightClick";
        }
        return `MouseButton${button + 1}Click`;
    }

    function onMouseDown(event: MouseEvent) {
        if (
            !isListening ||
            // Don't change if clicks to stop listening
            // @ts-ignore .id on target exists
            event.target?.id === LISTENER_BUTTON_ID ||
            // Don't allow unmodified clicks
            !(event.altKey || event.ctrlKey || event.shiftKey || event.metaKey)
        )
            return;
        modAlt = event.altKey;
        modCtrl = event.ctrlKey;
        modShift = event.shiftKey;
        modSuper = event.metaKey;
        mainKey = mouseNumberToText(event.button);
    }

    function toggleListen() {
        if (!isListening) {
            previousShortcut = shortcut;
        }
        isListening = !isListening;
        if (!isListening) {
            setupShortcut(previousShortcut !== shortcut);
        }
    }

    async function setupShortcut(unregisterOld: boolean = false) {
        if (
            unregisterOld &&
            tauriRegistered &&
            (await isRegistered(previousShortcut).catch((e) =>
                showShortcutFindingError(previousShortcut, false),
            ))
        ) {
            await unregister(previousShortcut).catch((e) =>
                showShortcutUnregistrationError(previousShortcut, null),
            );
        }
        // For mouse click, don't use Tauri's shortcut system
        if (shortcut.includes("Click")) {
            tauriRegistered = false;
        } else {
            await register(shortcut, onToggleShortcutEvent).then(
                (_success) => {
                    tauriRegistered = true;
                },
                (_failure) => showShortcutRegistrationError(shortcut, null),
            );
        }
        showShortcutRegistrationSuccess(shortcut, null);
    }

    let clickEventUnlistener: UnlistenFn | null = null;
    let modKeyEventUnlistener: UnlistenFn | null = null;

    let hasShortcutError = $state(false);

    function checkAllModPressed(): boolean {
        return (
            modCurrentSet.has("Super") === modSuper &&
            modCurrentSet.has("Shift") === modShift &&
            modCurrentSet.has("Control") === modCtrl &&
            modCurrentSet.has("Alt") === modAlt
        );
    }

    $effect(() => {
        addEventListener("keydown", onKeyDown);
        addEventListener("mousedown", onMouseDown);
        const asyncSetup = async () => {
            clickEventUnlistener = await listen("mouse_press", (p) => {
                if (isListening || tauriRegistered) return;
                const payload = p.payload as string;
                if (checkAllModPressed() && mainKey.includes(payload)) {
                    onToggleShortcutEvent({
                        shortcut: "MouseClick",
                        id: 0,
                        state: "Pressed",
                    });
                }
            });
            modKeyEventUnlistener = await listen("mod_key", (p) => {
                if (isListening || tauriRegistered) return;
                const payload = p.payload as {
                    key: string;
                    is_pressed: boolean;
                };
                const key = payload.key
                    .replace(/(?:L(?:eft)?|R(?:ight))?(.*)/g, "$1")
                    .replace("Option", "Alt")
                    .replace("Command", "Super");
                if (payload.is_pressed) {
                    modCurrentSet.add(key);
                } else {
                    modCurrentSet.delete(key);
                }
            });
            setupShortcut(previousShortcut === "");
        };
        asyncSetup();

        return () => {
            removeEventListener("keydown", onKeyDown);
            removeEventListener("mousedown", onMouseDown);
            try {
                clickEventUnlistener?.();
                modKeyEventUnlistener?.();
                unregisterAll();
            } catch (error) {
                console.error(error);
            }
        };
    });
</script>

{#snippet keyboardItem(
    name: string,
    active?: boolean,
    enabled?: boolean,
    onclick?: () => void,
    usePlus: boolean = false,
)}
    {#if enabled ?? true}
        <button {onclick}>
            <kbd
                class={`kbd ${active ? "outline-2 outline-primary" : ""} ${numberOfModKyes > 1 && onclick ? "hover:bg-error" : ""}`}
                >{name}</kbd
            >
        </button>
        {#if usePlus}
            <span class="px-1 m-0">+</span>
        {/if}
    {/if}
{/snippet}

<section>
    <fieldset
        class={`fieldset bg-base-200 border border-base-300 p-4 rounded-box ${hasShortcutError ? "border-error" : "border-success"}`}
    >
        <legend class="fieldset-legend">Custom Shortcut</legend>
        <p class="">
            {@render keyboardItem(
                "Ctrl",
                false,
                modCtrl,
                () => {
                    if (numberOfModKyes > 1) modCtrl = false;
                    else {
                        notifier?.showInfo(
                            "Must have at least one modifer key!",
                        );
                    }
                },
                true,
            )}
            {@render keyboardItem(
                "Shift",
                false,
                modShift,
                () => {
                    if (numberOfModKyes > 1) modShift = false;
                    else {
                        notifier?.showInfo(
                            "Must have at least one modifer key!",
                        );
                    }
                },
                true,
            )}
            {@render keyboardItem(
                "Alt",
                false,
                modAlt,
                () => {
                    if (numberOfModKyes > 1) modAlt = false;
                    else {
                        notifier?.showInfo(
                            "Must have at least one modifer key!",
                        );
                    }
                },
                true,
            )}
            {@render keyboardItem(
                "Super",
                false,
                modSuper,
                () => {
                    if (numberOfModKyes > 1) modSuper = false;
                    else {
                        notifier?.showInfo(
                            "Must have at least one modifer key!",
                        );
                    }
                },
                true,
            )}
            {@render keyboardItem(
                mainKey.replace(/([a-z0-9])([A-Z])/g, "$1 $2"),
                false,
                true,
            )}
        </p>
        <Button
            id={LISTENER_BUTTON_ID}
            color={isListening ? "primary" : "neutral"}
            onclick={toggleListen}>Listen for shortcut</Button
        >
        <p class="fieldset-label">
            You can click on modifier keys to remove them. There must always be
            one main trigger key or mouse click in order to be a valid shortcut.
        </p>
        <p class="fieldset-label text-warning">
            NOTE: Key shown may not map one-to-one if you are using a non-QWERTY
            keyboard layout.
        </p>
    </fieldset>
</section>
