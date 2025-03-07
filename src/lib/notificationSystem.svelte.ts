import { invoke } from "@tauri-apps/api/core";
import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
} from "@tauri-apps/plugin-notification";
import { toast } from "svelte-sonner";


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

    constructor(enableSound = true, testNotify = false) {
        this.#enabledSound = enableSound;
        this.getPermissionToNotify(testNotify);
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
            invoke("play_sound", { soundName: sound }).catch((err) =>
                console.error(err),
            );
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