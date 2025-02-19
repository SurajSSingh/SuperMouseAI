<script lang="ts" module>
    import { tv, type VariantProps } from "tailwind-variants";

    export const size = {
        default: "",
        xs: "status-xs",
        sm: "status-sm",
        md: "status-md",
        lg: "status-lg",
        xl: "status-xl",
    };
    export const animation = {
        none: "",
        ping: "animate-ping",
        bounce: "animate-bounce",
    };
    export const color = {
        default: "",
        neutral: "status-neutral",
        primary: "status-primary",
        secondary: "status-secondary",
        accent: "status-accent",
        info: "status-info",
        success: "status-success",
        warning: "status-warning",
        error: "status-error",
        destructive: "status-error",
    };
    export const statusVariants = tv({
        base: "status",
        variants: {
            size,
            animation,
            color,
        },
        defaultVariants: {
            size: "default",
            animation: "none",
            color: "default",
        },
    });

    export type StatusColor = VariantProps<typeof statusVariants>["color"];
    export type StatusAnimation = VariantProps<
        typeof statusVariants
    >["animation"];
    export type StatusSize = VariantProps<typeof statusVariants>["size"];
</script>

<script lang="ts">
    interface StatusProps {
        color?: StatusColor;
        animation?: StatusAnimation;
        size?: StatusSize;
        class?: string;
    }

    let {
        color,
        animation = "none",
        size = "default",
        class: className = "",
    }: StatusProps = $props();
</script>

<span
    class={statusVariants({
        size,
        animation,
        color,
    }) +
        " " +
        className}
></span>
