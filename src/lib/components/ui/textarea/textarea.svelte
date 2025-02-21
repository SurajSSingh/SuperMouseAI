<script lang="ts" module>
    import type { WithElementRef, WithoutChildren } from "bits-ui";
    import type { HTMLTextareaAttributes } from "svelte/elements";
    import { cn } from "$lib/utils.js";
    import { type VariantProps, tv } from "tailwind-variants";

    const variant = {
        default: "",
        ghost: "textarea-ghost",
    };

    const color = {
        default: "",
        neutral: "textarea-neutral",
        primary: "textarea-primary",
        secondary: "textarea-secondary",
        accent: "textarea-accent",
        info: "textarea-info",
        success: "textarea-success",
        warning: "textarea-warning",
        error: "textarea-error",
        destructive: "textarea-error",
    };

    const size = {
        xs: "textarea-xs",
        sm: "textarea-sm",
        md: "textarea-md",
        lg: "textarea-lg",
        xl: "textarea-xl",
    };

    export const textareaVariants = tv({
        base: "border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex min-h-[80px] w-full rounded-md border px-3 py-2 text-base focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        variants: {
            variant,
            color,
            size,
        },
        defaultVariants: {
            variant: "default",
            color: "default",
            size: "md",
        },
    });

    export type TextAreaColor = VariantProps<typeof textareaVariants>["color"];
    export type TextAreaVariant = VariantProps<
        typeof textareaVariants
    >["variant"];
    export type TextAreaSize = VariantProps<typeof textareaVariants>["size"];
</script>

<script lang="ts">
    let {
        ref = $bindable(null),
        value = $bindable(),
        variant = "default",
        color = "neutral",
        size = "md",
        class: className,
        ...restProps
    }: WithoutChildren<WithElementRef<HTMLTextareaAttributes>> & {
        color?: TextAreaColor;
        variant?: TextAreaVariant;
        size?: TextAreaSize;
    } = $props();
</script>

<textarea
    bind:this={ref}
    class={cn(
        "textarea",
        textareaVariants({
            size,
            variant,
            color,
        }),
        className,
    )}
    bind:value
    {...restProps}
></textarea>
