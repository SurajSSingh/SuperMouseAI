<script lang="ts" module>
    import { tv, type VariantProps } from "tailwind-variants";

    export const variant = {
        spinner: "loading-spinner",
        dots: "loading-dots",
        balls: "loading-balls",
        bars: "loading-bars",
        infinity: "loading-infinity",
    };

    export const size = {
        xs: "loading-xs",
        sm: "loading-sm",
        md: "loading-md",
        lg: "loading-lg",
        xl: "loading-xl",
    };

    export const color = {
        default: "",
        neutral: "text-neutral",
        primary: "text-primary",
        secondary: "text-secondary",
        accent: "text-accent",
        info: "text-info",
        success: "text-success",
        warning: "text-warning",
        error: "text-error",
        destructive: "text-error",
    };
    export const loadingVariants = tv({
        base: "loading",
        variants: {
            variant,
            size,
            color,
        },
        defaultVariants: {
            variant: "spinner",
            size: "md",
            color: "default",
        },
    });

    export type LoadingColor = VariantProps<typeof loadingVariants>["color"];
    export type LoadingVariant = VariantProps<
        typeof loadingVariants
    >["variant"];
    export type LoadingSize = VariantProps<typeof loadingVariants>["size"];
</script>

<script lang="ts">
    interface LoadingProps {
        color?: LoadingColor;
        variant?: LoadingVariant;
        size?: LoadingSize;
    }

    let { color, variant, size }: LoadingProps = $props();
</script>

<span
    class={loadingVariants({
        size,
        variant,
        color,
    })}
></span>
