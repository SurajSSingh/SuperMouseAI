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
                sendNotification("Notification Test!");
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
            window.alert(`${subtitle ? subtitle + ": " : ""}${message}`);
        }
        if (this.#enabledSound && sound) {
            console.log(`Playing: ${sound}`)
            invoke("play_sound", { soundName: sound }).catch((err) =>
                console.error(err),
            );
        }
    }

    showAlert(
        message: string,
        subtitle = "",
        sound = "default_alert",
    ) {
        console.log(`Alert ${subtitle}: ${message}`)
        toast(`${subtitle ? subtitle + ": " : ""}${message}`);
        if (this.#enabledSound && sound) {
            invoke("play_sound", { soundName: sound }).catch((err) =>
                console.error(err),
            );
        }
    }
}