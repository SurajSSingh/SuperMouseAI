<script lang="ts">
    import { events } from "$lib/bindings";
    import { type RecordingStates } from "$lib/types";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { debug } from "@tauri-apps/plugin-log";

    let follower: HTMLElement;

    let xOffset = $state(0);
    let yOffset = $state(-20);

    let currentState: RecordingStates | undefined = $state();

    $effect(() => {
        let unlistenMoveEvent: UnlistenFn | undefined;
        const run = async () => {
            unlistenMoveEvent = await events.mouseMoveEvent.listen((e) => {
                follower.style.left = `${e.payload.x + xOffset}px`;
                follower.style.top = `${e.payload.y + yOffset}px`;
            });
            listen("stateUpdate", (e) => {
                debug(`State of recording updated: ${JSON.stringify(e)}`);
                currentState = (e.payload as { state: RecordingStates }).state;
            });
        };
        run();
        return () => {
            debug("Close overlay");
            unlistenMoveEvent?.();
        };
    });
</script>

<div id="follower" bind:this={follower} class={currentState}>
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
        background: #fff;
        border-radius: 50%;
        height: 0em;
        width: 0em;
        margin-top: 0em;
        margin-left: 0em;
        opacity: 0.2;
        height: 1em;
        width: 1em;
        margin-top: -0.5em;
        margin-left: -0.5em;
    }
    #follower #circle2 {
        position: absolute;
        background: rgba(247, 231, 52, 0.8);
        border-radius: 50%;
        height: 0em;
        width: 0em;
        margin-top: 0em;
        margin-left: 0em;
        opacity: 0.2;
        height: 1em;
        width: 1em;
        margin-top: -0.5em;
        margin-left: -0.5em;
    }
    #follower.processing #circle1 {
        position: absolute;
        background: #fff;
        border-radius: 50%;
        height: 0em;
        width: 0em;
        margin-top: 0em;
        margin-left: 0em;
        opacity: 0.2;
        height: 1em;
        width: 1em;
        margin-top: -0.5em;
        margin-left: -0.5em;
    }
    #follower.processing #circle2 {
        position: absolute;
        background: rgba(20, 104, 239, 0.8);
        border-radius: 50%;
        height: 0em;
        width: 0em;
        margin-top: 0em;
        margin-left: 0em;
        opacity: 0.2;
        height: 1em;
        width: 1em;
        margin-top: -0.5em;
        margin-left: -0.5em;
        -webkit-animation: pulse 1s infinite; /* Chrome, Safari, Opera */
        animation: pulse 1s infinite;
    }
    #follower.recording #circle1 {
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
    #follower.recording #circle2 {
        position: absolute;
        -webkit-animation: pulse 4s infinite; /* Chrome, Safari, Opera */
        animation: pulse 2s infinite;
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
