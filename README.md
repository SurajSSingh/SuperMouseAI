# Super Mouse AI

A way to trascribe your voice using Whisper from the click of your mouse.

## Credits

- [Rust](https://www.rust-lang.org): Backend
  - [Tauri](https://tauri.app): Application framework
  - [whisper-rs](https://github.com/tazz4843/whisper-rs): Wrapper for
    [whisper.cpp](https://github.com/ggerganov/whisper.cpp) project, allowing
    local use of [Whisper](https://github.com/openai/whisper) model from
    [OpenAI](https://openai.com/index/whisper/)
  - [mutter](https://github.com/sigaloid/mutter): Project that wraps
    `whisper-rs`, directly imported into the project and used under the
    `MIT OR Apache-2.0` license.
  - [nnnoiseless](https://github.com/jneem/nnnoiseless): Local Audio Denoise
- [JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript)/[TypeScript](https://www.typescriptlang.org):
  Frontend language
  - [Deno](https://deno.com): Runtime
  - [Svelte/SvelteKit](https://svelte.dev): Framework/Meta-framework
  - [BitsUI](https://www.bits-ui.com/docs/introduction) and
    [Shadcn-Svelte](https://www.shadcn-svelte.com): Functional components
  - [TailwindCSS](https://tailwindcss.com) and [DaisyUI](https://daisyui.com):
    App styling
  - [extendable-media-recorder](https://github.com/chrisguttandin/extendable-media-recorder):
    An extendable drop-in replacement for the native MediaRecorder, used to help
    record user voice to WAV for transcription.
- Misc.
  - [Sentry](https://sentry.io/welcome/): Error/Crash Tracking
  - [Git-Cliff](https://git-cliff.org): Changelog Generator

## License

`MIT OR Apache-2.0`:

This project is licensed under either of

- [Apache 2.0](LICENSE-ALv2.md)
- [MIT](LICENSE-MIT.md)
