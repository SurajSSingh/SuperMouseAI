<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import Status from "$lib/components/ui/Status.svelte";

    interface PermissionButtonProps {
        name: string;
        showName?: boolean;
        showStatusAsEmoji?: boolean;
        status?: boolean | null;
        onclick: () => void;
        icon?: string;
    }

    let {
        name,
        status,
        onclick,
        icon,
        showName = true,
        showStatusAsEmoji = false,
    }: PermissionButtonProps = $props();

    const statusColor = $derived(
        status === true ? "success" : status === false ? "error" : "warning",
    );
    const statusValue = $derived(
        status === true
            ? "granted"
            : status === false
              ? "denied"
              : "user prompted",
    );
    const statusIcon = $derived(
        status === true ? "✅" : status === false ? "❌" : "❔",
    );
</script>

<div class="tooltip tooltip-bottom" data-tip="">
    <div class="tooltip-content">
        <div>
            <span
                >{`${name} permission is `}<span class={`text-${statusColor}`}
                    >{statusValue}</span
                ></span
            >
            {#if status === null}
                <br />
                <span class="text-secondary"
                    >Click button to ask permission.</span
                >
            {/if}
        </div>
    </div>
    <Button variant="outline" color={statusColor} {onclick}>
        {#if showStatusAsEmoji}
            <span>{statusIcon}</span>
        {:else}
            <Status color={statusColor} size="lg" class="mr-2" />
        {/if}
        <span class="text-accent">
            {icon ? `${icon}` : ""}{icon && showName ? ":" : ""}{showName
                ? name
                : ""}</span
        >
    </Button>
</div>
