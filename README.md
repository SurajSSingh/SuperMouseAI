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

This project is licensed under either of

- Functional Source License, Version 1.1, ALv2 Future License
  ([FSL-1.1-APACHE-2.0](LICENSE-FSL-1.1-ALv2.md) or
  <https://github.com/getsentry/fsl.software/blob/main/FSL-1.1-ALv2.template.md>)
- Functional Source License, Version 1.1, MIT Future License
  ([FSL-1.1-MIT](LICENSE-FSL-1.1-MIT.md) or
  <https://github.com/getsentry/fsl.software/blob/main/FSL-1.1-MIT.template.md>)

at your option.

For each version of the software, after 2 years from that release, project
commits up to that point will be made available under `MIT OR Apache-2.0`
licenses. Prior to the 2 years, you may not sell the software without explicit
grant from the author. For any non-commerical/private uses, you may treat the
repository as if it were made available under the `MIT OR Apache-2.0` licenses.
