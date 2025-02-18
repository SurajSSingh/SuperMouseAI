<script lang="ts">
    import type { Snippet } from "svelte";
    import { Content, Item, Root, Trigger } from "./ui/accordion";

    export interface SingleCollapsableProps {
        itemValue?: string;
        summary?: string;
        class?: string;
        summaryClass?: string;
        children?: Snippet;
    }

    let {
        itemValue,
        class: className,
        summary = "Open",
        summaryClass = "text-center",
        children,
    }: SingleCollapsableProps = $props();
</script>

<Root type="single" class={className}>
    <Item value={itemValue}>
        <Trigger>
            {summary}
        </Trigger>
        <Content>
            {@render children?.()}
        </Content>
    </Item>
</Root>
