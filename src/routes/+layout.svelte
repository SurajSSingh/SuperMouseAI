<script>
    import { Toaster } from "$lib/components/ui/sonner";
    let { children } = $props();
    import "../app.css";
</script>

<Toaster />

{@render children()}
