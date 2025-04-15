import { debug, info, warn } from "@tauri-apps/plugin-log";
import {
  ask,
  confirm,
  type ConfirmDialogOptions,
  message as dialogMessage,
} from "@tauri-apps/plugin-dialog";
import type { ConfirmActionType } from "./types.ts";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";
import { type ExternalToast, toast } from "svelte-sonner";
import { commands } from "./bindings.ts";
import { configStore } from "./store.svelte.ts";

export const toastData: ExternalToast = {
  unstyled: true,
  classes: {
    toast: "alert",
    error: "alert-error",
    info: "alert-info",
    success: "alert-success",
    warning: "alert-warning",
    loading: "alert-info",
    closeButton: "btn btn-narrow",
    actionButton: "btn btn-primary hover:btn-success",
    cancelButton: "btn btn-accent hover:btn-error",
  },
} as const;

export class NotificationSystem {
  #permissionGranted = false;
  #enabledSound = true;

  get permissionGranted(): boolean {
    return this.#permissionGranted;
  }

  async checkPermissionGranted(): Promise<boolean> {
    this.#permissionGranted = await isPermissionGranted();
    return this.#permissionGranted;
  }

  constructor(
    enableSound = true,
    testNotify = false,
    includeSystemNotification = true,
  ) {
    this.#enabledSound = enableSound;
    if (includeSystemNotification) this.getPermissionToNotify(testNotify);
  }

  getPermissionToNotify(testNotify = false): void {
    requestPermission().then((permission) => {
      this.#permissionGranted = permission === "granted";
      if (testNotify && this.#permissionGranted) {
        sendNotification("Notification are enabled!");
      } else if (testNotify) {
        globalThis.alert("Notification not enabled!");
      }
    });
  }

  showNotification(
    message: string,
    subtitle = "",
    sound = "default_alert",
  ): void {
    if (this.#permissionGranted) {
      sendNotification({
        title: `SuperMouse AI${subtitle ? ": " + subtitle : ""}`,
        body: message,
      });
      this.playSound(sound);
    } else {
      warn("Could not show notification, falling back to toast message.");
      this.showToast(message, "default", { subtitle, sound });
    }
  }

  showToast(
    message: string,
    type?: "info" | "warn" | "error" | "success" | "loading" | "default",
    toastConfig?: ExternalToast & { sound?: string; subtitle?: string },
  ): string | number {
    toastConfig ??= toastData;
    if (toastConfig.subtitle && !toastConfig.description) {
      toastConfig.description = message;
    }
    // This would be fixed it TS allowed switch/match-expression
    const toster = type === "info"
      ? toast.info
      : type === "warn"
      ? toast.warning
      : type === "error"
      ? toast.error
      : type === "success"
      ? toast.success
      : type === "loading"
      ? toast.loading
      : toast;
    if (toastConfig.sound) {
      this.playSound(toastConfig.sound);
    }
    return toster(toastConfig.subtitle || message, toastConfig);
  }

  showPromiseToast<T>(
    promise: Promise<T>,
    loading = "Loading...",
    success: string | ((data: T) => string) = "Success!",
    error: string | ((error: unknown) => string) = "Error!",
    final: (() => void | Promise<void>) | undefined = undefined,
  ): string | number | undefined {
    return toast.promise(promise, {
      ...toastData,
      loading,
      success,
      error,
      finally: final,
    });
  }

  /**
   * Show native dialog box
   * @param type: `message` = "Simple message", `ask` = "Question to answer", `confirm` = "Confirm an action"
   * @param message: The message/question for the user
   * @param options: Options to modify the dialog box
   * @returns Promise of user's response (`true` = OK/Yes, `false`=Cancel/No)
   */
  async showDialog(
    type: "message" | "ask" | "confirm",
    message: string,
    options?: ConfirmDialogOptions,
  ): Promise<boolean> {
    switch (type) {
      case "message":
        await dialogMessage(message, options);
        return true;
      case "ask":
        return await ask(message, options);
      case "confirm":
        return await confirm(message, options);
      default:
        warn("Reached unreachable case in showDialog, skipping action.");
        break;
    }
    return false;
  }

  confirmAction(
    message: string,
    onConfirm: () => void,
    onCancel: () => void = () => {},
    subtitle = "",
    confirm = "Yes",
    cancel = "No",
    options: ConfirmActionType = {},
  ): void {
    options = {
      sound: "default_alert",
      confirmButtonStyle: "btn btn-success",
      cancelButtonStyle: "btn btn-error",
      mustRetry: false,
      ...options,
    };
    debug(`Confirmation Action options given: ${JSON.stringify(options)}`);
    toast.warning(subtitle ? subtitle : message, {
      duration: Number.POSITIVE_INFINITY,
      action: {
        label: confirm,
        onClick: onConfirm,
      },
      actionButtonStyle: options.confirmButtonStyle,
      cancel: {
        label: cancel,
        onClick: onCancel,
      },
      cancelButtonStyle: options.cancelButtonStyle,
      important: true,
      description: subtitle ? message : undefined,
      onDismiss: (t) =>
        options.mustRetry &&
          (options.mustRetry === true || +options.mustRetry > 0)
          ? this.confirmAction(
            message,
            onConfirm,
            onCancel,
            subtitle,
            confirm,
            cancel,
            {
              ...options,
              mustRetry: typeof options.mustRetry === "boolean"
                ? true
                : options.mustRetry - 1,
            },
          )
          : info(`Toast with id ${t.id} has been dismissed`),
    });
    this.playSound(options.sound);
  }

  playSound(sound = "default_alert"): void {
    if (this.#enabledSound && sound) {
      commands.playSound(sound).catch((err) => warn(err));
    }
  }

  setEnabledSound(value: boolean | "toggle"): void {
    if (value === "toggle") {
      this.#enabledSound = !this.#enabledSound;
    } else {
      this.#enabledSound = value;
    }
  }
}

export const notifier = new NotificationSystem(
  configStore.enabledSound.value,
  configStore.testNotify.value,
  configStore.useSystemNotification.value,
);
