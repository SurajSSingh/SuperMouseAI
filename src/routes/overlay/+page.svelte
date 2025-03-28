<script lang="ts">
    import { events } from "$lib/bindings";
    import { type RecordingStates } from "$lib/types";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { debug } from "@tauri-apps/plugin-log";

    let follower: HTMLElement;

    let xOffset = $state(-20);
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
    #follower {
        position: absolute;
        height: 3rem;
        width: 3rem;
    }

    #follower div {
        position: absolute;
        border-radius: 50%;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 100%;
        height: 100%;
    }

    /* Stopped state - no animations */
    #follower.stopped div {
        background: transparent;
    }

    /* Recording state */
    #follower.recording #circle1 {
        background: transparent;
        outline-style: solid;
        outline-color: #fc5185;
        animation: pulse-record-1 1s infinite;
    }

    #follower.recording #circle2 {
        background: transparent;
        outline-style: solid;
        outline-color: #fc5185;
        animation: pulse-record-2 1s infinite;
    }

    /* Processing state */
    #follower.processing #circle1 {
        background: rgba(255, 255, 255, 0.2);
        border: 4px solid rgba(0, 0, 0, 0.1);
        border-left-color: #7983ff;
        animation: spin 1.25s linear infinite;
    }

    #follower.processing #circle2 {
        background: rgba(20, 104, 239, 0.1);
    }

    /* Keyframe animations */

    @keyframes spin {
        from {
            transform: translate(-50%, -50%) rotate(0deg);
        }
        to {
            transform: translate(-50%, -50%) rotate(360deg);
        }
    }

    @keyframes pulse-record-1 {
        0% {
            outline-width: 0px;
            transform: translate(-50%, -50%) rotate(0deg) scale(1);
        }
        20% {
            outline-width: 0px;
            transform: translate(-50%, -50%) rotate(0deg) scale(1);
        }
        60% {
            outline-width: 4px;
            transform: translate(-50%, -50%) rotate(180deg) scale(1.25);
        }
        100% {
            outline-width: 0px;
            transform: translate(-50%, -50%) rotate(360deg) scale(1);
        }
    }

    @keyframes pulse-record-2 {
        0% {
            outline-width: 0px;
            transform: translate(-50%, -50%) scale(0);
        }
        30% {
            outline-width: 2px;
            transform: translate(-50%, -50%) scale(1);
        }
        60% {
            outline-width: 4px;
            transform: translate(-50%, -50%) scale(1);
        }
        100% {
            outline-width: 0px;
            transform: translate(-50%, -50%) scale(0);
        }
    }
</style>
