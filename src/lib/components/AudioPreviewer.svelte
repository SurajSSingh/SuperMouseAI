<!-- 
@component
Given a url to an auido source, render it to allow user to listen.
-->
<script lang="ts">
    interface AudioPreviewerProps {
        currentURL?: string;
    }
    // Props
    let { currentURL }: AudioPreviewerProps = $props();
</script>

<h2 class="text-lg text-center">Audio Preview</h2>
<p class="text-xs text-center">{currentURL ?? "No URL"}</p>
<audio class="w-full" controls src={currentURL}></audio>
