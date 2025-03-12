import { warn } from '@tauri-apps/plugin-log';
import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
} from "@tauri-apps/plugin-notification";
import { toast, type ExternalToast } from "svelte-sonner";
import { commands } from "./bindings";
import { configStore } from './store.svelte';


export const toastData: ExternalToast = {

    unstyled: true, classes: {
        toast: "alert",
        error: "alert-error",
        info: "alert-info",
        success: "alert-success",
        warning: "alert-warning",
        loading: "alert-info",
        closeButton: "btn btn-narrow",
        actionButton: "btn btn-primary hover:btn-success",
        cancelButton: "btn btn-accent hover:btn-error",
    }
} as const;

export class NotificationSystem {
    #permissionGranted = false;
    #enabledSound = true;

    get permissionGranted() {
        return this.#permissionGranted
    }

    async checkPermissionGranted() {
        this.#permissionGranted = await isPermissionGranted();
        return this.#permissionGranted
    }

    constructor(enableSound = true, testNotify = false, includeSystemNotification = true) {
        this.#enabledSound = enableSound;
        if (includeSystemNotification) this.getPermissionToNotify(testNotify);
    }

    getPermissionToNotify(testNotify = false) {
        requestPermission().then(permission => {
            this.#permissionGranted = permission === "granted";
            if (testNotify && this.#permissionGranted) {
                sendNotification("Notification are enabled!");
            } else if (testNotify) {
                window.alert("Notification not enabled!");
            }
        })
    }

    showNotification(
        message: string,
        subtitle = "",
        sound = "default_alert",
    ) {
        if (this.#permissionGranted) {
            sendNotification({
                title: `SuperMouse AI${subtitle ? ": " + subtitle : ""}`,
                body: message,
            });
        } else {
            this.showInfo(message, subtitle, sound);
        }
        this.playSound(sound);
    }

    showToast(message: string, subtitle = "", type?: "info" | "warn" | "error" | "success" | "loading" | "default", sound = "", duration = 5000): string | number {
        // This would be fixed it TS allowed switch/match-expression
        const toster =
            type === "info" ? toast.info
                : type === "warn" ? toast.warning
                    : type === "error" ? toast.error
                        : type === "success" ? toast.success
                            : type === "loading" ? toast.loading
                                : toast;
        if (sound) {
            this.playSound(sound)
        }
        return toster(subtitle || message, { ...toastData, duration, })
    }

    showPromiseToast<T>(
        promise: Promise<T>,
        loading: string = "Loading...",
        success: string | ((data: T) => string) = "Success!",
        error: string | ((error: unknown) => string) = "Error!",
        final?: () => void | Promise<void>
    ): string | number | undefined {
        return toast.promise(promise, { ...toastData, loading, success, error, finally: final })
    }

    // TEMP
    showSuccess(
        message: string,
        subtitle = "",
        sound = "",
        duration = 5000,
    ) {
        toast.success(subtitle || message, {
            description: subtitle ? message : "",
            duration
        });
        this.playSound(sound);
    }

    // TEMP
    showError(
        message: string,
        subtitle = "",
        sound = "default_alert",
        duration = 5000,
    ) {
        toast.error(subtitle || message, {
            description: subtitle ? message : "",
            duration
        });
        this.playSound(sound);
    }

    // TEMP
    showInfo(
        message: string,
        subtitle = "",
        sound = "default_alert",
        duration = 5000,
    ) {
        toast.info(subtitle || message, {
            description: subtitle ? message : "",
            duration
        });
        this.playSound(sound);
    }

    showAlert(
        message: string,
        subtitle = "",
        sound = "default_alert",
    ) {
        console.log(`Alert ${subtitle}: ${message}`)
        toast(`${subtitle ? subtitle + ": " : ""}${message}`);
        this.playSound(sound);
    }

    confirmAction(
        message: string,
        onConfirm: () => void,
        onCancel: () => void = () => { },
        subtitle = "",
        confirm: string = "Yes",
        cancel: string = "No",
        sound = "default_alert",
        confirmButtonStyle: string = "btn btn-success",
        cancelButtonStyle: string = "btn btn-error",
    ) {
        toast.warning(subtitle ? subtitle : message, {
            duration: Number.POSITIVE_INFINITY, action: {
                label: confirm,
                onClick: onConfirm,
            },
            actionButtonStyle: confirmButtonStyle,
            cancel: {
                label: cancel,
                onClick: onCancel,
            },
            cancelButtonStyle,
            important: true,
            description: subtitle ? message : undefined,
        });
        this.playSound(sound);
    }

    playSound(sound = "default_alert") {
        if (this.#enabledSound && sound) {
            commands.playSound(sound).catch(err => warn(err))
        }
    }

    setEnabledSound(value: boolean | "toggle") {
        if (value === "toggle") {
            this.#enabledSound = !this.#enabledSound;
        }
        else {
            this.#enabledSound = value;
        }
    }
}

export const notifier = new NotificationSystem(configStore.enabledSound.value, configStore.testNotify.value, configStore.useSystemNotification.value)