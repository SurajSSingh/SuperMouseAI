<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import { Toaster } from "$lib/components/ui/sonner";
    import ToggleSwitch from "$lib/components/ui/ToggleSwitch.svelte";
    import { NotificationSystem } from "$lib/notificationSystem.svelte";
    import { resolve } from "@tauri-apps/api/path";
    const notifier = new NotificationSystem(false, false, false);
    let message = $state("Message Test");
    let type: "info" | "warn" | "error" | "success" | "loading" | "default" =
        $state("default");
</script>

<main class="">
    <Toaster position="top-center" closeButton theme="system" />
    <div class="mt-30 mx-40">
        <input type="text" name="message" id="message" bind:value={message} />
        <select bind:value={type}>
            <option value="default">Default</option>
            <option value="info">Info</option>
            <option value="warn">Warning</option>
            <option value="success">Success</option>
            <option value="error">Error</option>
        </select>
        <Button onclick={() => notifier.showToast(message, type)}
            >Test Basic Toast</Button
        >
        <Button
            onclick={() =>
                notifier.showPromiseToast(
                    new Promise((resolve, reject) =>
                        setTimeout(
                            Math.random() > 0.5 ? resolve : reject,
                            2000,
                        ),
                    ),
                    "Loading Test...",
                    "It worked!",
                    "It failed!",
                )}>Test Promise Toast</Button
        >
    </div>
</main>
