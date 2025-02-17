<script lang="ts" module>
    import type { WithElementRef } from "bits-ui";
    import type {
        HTMLAnchorAttributes,
        HTMLButtonAttributes,
    } from "svelte/elements";
    import { type VariantProps, tv } from "tailwind-variants";

    // Daisy UI Variants

    const variant = {
        default: "",
        outline: "btn-outline",
        soft: "btn-soft",
        ghost: "btn-ghost",
        link: "btn-link",
    };

    const color = {
        default: "",
        neutral: "btn-neutral",
        primary: "btn-primary",
        secondary: "btn-secondary",
        accent: "btn-accent",
        info: "btn-info",
        success: "btn-success",
        warning: "btn-warning",
        error: "btn-error",
        destructive: "btn-error",
    };

    const size = {
        xs: "btn-xs",
        sm: "btn-sm",
        md: "btn-md",
        lg: "btn-lg",
        xl: "btn-xl",
    };

    const shapeMod = {
        default: "",
        square: "btn-square",
        circle: "btn-circle",
    };

    const widthMod = {
        default: "",
        narrow: "btn-square",
        wide: "btn-wide",
        block: "btn-block",
    };

    export const buttonVariants = tv({
        base: "ring-offset-background focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0",
        variants: {
            variant,
            color,
            size,
            widthMod,
            shapeMod,
        },
        defaultVariants: {
            variant: "default",
            color: "neutral",
            size: "md",
            widthMod: "default",
            shapeMod: "default",
        },
    });

    export type ButtonColor = VariantProps<typeof buttonVariants>["color"];
    export type ButtonVariant = VariantProps<typeof buttonVariants>["variant"];
    export type ButtonSize = VariantProps<typeof buttonVariants>["size"];
    export type ButtonShape = VariantProps<typeof buttonVariants>["shapeMod"];
    export type ButtonWidth = VariantProps<typeof buttonVariants>["widthMod"];

    export type ButtonProps = WithElementRef<HTMLButtonAttributes> &
        WithElementRef<HTMLAnchorAttributes> & {
            color?: ButtonColor;
            variant?: ButtonVariant;
            size?: ButtonSize;
            shape?: ButtonShape;
            width?: ButtonWidth;
        };
</script>

<script lang="ts">
    import { cn } from "$lib/utils.ts";

    let {
        class: className,
        variant = "default",
        color = "neutral",
        size = "md",
        shape = "default",
        width = "default",
        ref = $bindable(null),
        href = undefined,
        type = "button",
        children,
        ...restProps
    }: ButtonProps = $props();

    const hasLink = $derived(Boolean(href));
</script>

<svelte:element
    this={hasLink ? "a" : "button"}
    bind:this={ref}
    class={cn(
        `btn${hasLink ? " btn-link" : ""}`,
        buttonVariants({
            size,
            variant,
            color,
            shapeMod: shape,
            widthMod: width,
        }),
        className,
    )}
    href={hasLink ? href : null}
    type={!hasLink ? type : null}
    {...restProps}
>
    {@render children?.()}
</svelte:element>
