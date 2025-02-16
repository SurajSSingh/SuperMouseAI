<script lang="ts" module>
    import type { WithElementRef } from "bits-ui";
    import type {
        HTMLAnchorAttributes,
        HTMLButtonAttributes,
    } from "svelte/elements";
    import { type VariantProps, tv } from "tailwind-variants";

    // Daisy UI Variants
    const variant = {
        neutral: "btn-neutral",
        primary: "btn-primary",
        secondary: "btn-secondary",
        accent: "btn-accent",
        info: "btn-info",
        success: "btn-success",
        warning: "btn-warning",
        error: "btn-error",
        link: "btn-link",
        destructive: "btn-error",
    };

    const size = {
        xs: "btn-xs",
        sm: "btn-sm",
        md: "btn-md",
        lg: "btn-lg",
        xl: "btn-xl",
    };

    export const buttonVariants = tv({
        base: "ring-offset-background focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0",
        variants: {
            variant,
            size,
        },
        defaultVariants: {
            variant: "neutral",
            size: "md",
        },
    });

    export type ButtonVariant = VariantProps<typeof buttonVariants>["variant"];
    export type ButtonSize = VariantProps<typeof buttonVariants>["size"];

    export type ButtonProps = WithElementRef<HTMLButtonAttributes> &
        WithElementRef<HTMLAnchorAttributes> & {
            variant?: ButtonVariant;
            size?: ButtonSize;
        };
</script>

<script lang="ts">
    import { cn } from "$lib/utils.ts";

    let {
        class: className,
        variant = "neutral",
        size = "md",
        ref = $bindable(null),
        href = undefined,
        type = "button",
        children,
        ...restProps
    }: ButtonProps = $props();

    const isLink = $derived(Boolean(href));
</script>

<svelte:element
    this={isLink ? "a" : "button"}
    bind:this={ref}
    class={cn(
        "btn",
        buttonVariants({ size, variant: isLink ? "link" : variant }),
        className,
    )}
    href={isLink ? href : null}
    type={!isLink ? type : null}
    {...restProps}
>
    {@render children?.()}
</svelte:element>
