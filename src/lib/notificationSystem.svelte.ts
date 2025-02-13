import { invoke } from "@tauri-apps/api/core";
import {
    // isPermissionGranted,
    requestPermission,
    sendNotification,
} from "@tauri-apps/plugin-notification";


export class NotificationSystem {
    #permissionGranted = false;
    #enabledSound = true;

    get permissionGranted() {
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
                alert("Notification not enabled!");
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
            alert(`${subtitle ? subtitle + ": " : ""}${message}`);
        }
        if (this.#enabledSound && sound) {
            console.log(`Playing: ${sound}`)
            invoke("play_sound", { soundName: sound }).catch((err) =>
                console.error(err),
            );
        }
    }
}