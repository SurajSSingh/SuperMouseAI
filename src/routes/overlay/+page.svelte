<script lang="ts">
    import { events } from "$lib/bindings";
    import type { UnlistenFn } from "@tauri-apps/api/event";
    import { debug } from "@tauri-apps/plugin-log";

    let follower: HTMLElement;

    $effect(() => {
        let unlisten: UnlistenFn | undefined;
        const run = async () => {
            unlisten = await events.mouseMoveEvent.listen((e) => {
                debug(`Mouse Move to ${JSON.stringify(e.payload)}`);
                follower.style.left = `${e.payload.x}px`;
                follower.style.top = `${e.payload.y}px`;
                // follower.style.setProperty("--mouse-y", e.payload.y.toString());
            });
        };
        run();
        return () => {
            unlisten?.();
        };
    });
</script>

<div id="follower" bind:this={follower}>
    <div id="circle1"></div>
    <div id="circle2"></div>
</div>

<style>
    /* :root {
        --mouse-x: 50;
        --mouse-y: 50;
    } */

    #follower {
        position: absolute;
        /* top: calc(var(--mouse-y, 0) * 100%); */
        /* left: calc(var(--mouse-x, 0) * 100%); */
    }
    #follower #circle1 {
        position: absolute;
        -webkit-animation: pulse 2s infinite; /* Chrome, Safari, Opera */
        animation: pulse 2s infinite;
        background: #fff;
        border-radius: 50%;
        height: 0em;
        width: 0em;
        margin-top: 0em;
        margin-left: 0em;
    }
    #follower #circle2 {
        position: absolute;
        -webkit-animation: pulse 4s infinite; /* Chrome, Safari, Opera */
        animation: pulse 4s infinite;
        background: rgba(200, 0, 0, 0.8);
        border-radius: 50%;
        height: 0em;
        width: 0em;
        margin-top: 0em;
        margin-left: 0em;
    }
    @-moz-keyframes pulse {
        0% {
            opacity: 0.2;
            height: 1em;
            width: 1em;
            margin-top: -0.5em;
            margin-left: -0.5em;
        }
        50% {
            opacity: 0.9;
            height: 3em;
            width: 3em;
            margin-top: -1.5em;
            margin-left: -1.5em;
        }
        100% {
            opacity: 0.2;
            height: 1em;
            width: 1em;
            margin-top: -0.5em;
            margin-left: -0.5em;
        }
    }
    @-webkit-keyframes pulse {
        0% {
            opacity: 0.2;
            height: 1em;
            width: 1em;
            margin-top: -0.5em;
            margin-left: -0.5em;
        }
        50% {
            opacity: 0.9;
            height: 3em;
            width: 3em;
            margin-top: -1.5em;
            margin-left: -1.5em;
        }
        100% {
            opacity: 0.2;
            height: 1em;
            width: 1em;
            margin-top: -0.5em;
            margin-left: -0.5em;
        }
    }
    @-o-keyframes pulse {
        0% {
            opacity: 0.2;
            height: 1em;
            width: 1em;
            margin-top: -0.5em;
            margin-left: -0.5em;
        }
        50% {
            opacity: 0.9;
            height: 3em;
            width: 3em;
            margin-top: -1.5em;
            margin-left: -1.5em;
        }
        100% {
            opacity: 0.2;
            height: 1em;
            width: 1em;
            margin-top: -0.5em;
            margin-left: -0.5em;
        }
    }
    @keyframes pulse {
        0% {
            opacity: 0.2;
            height: 1em;
            width: 1em;
            margin-top: -0.5em;
            margin-left: -0.5em;
        }
        50% {
            opacity: 0.9;
            height: 3em;
            width: 3em;
            margin-top: -1.5em;
            margin-left: -1.5em;
        }
        100% {
            opacity: 0.2;
            height: 1em;
            width: 1em;
            margin-top: -0.5em;
            margin-left: -0.5em;
        }
    }
</style>
