

Limitations:
1. Whisper not real time -> instead, break into chunks of audio and pad with silence, see https://github.com/ggerganov/whisper.cpp/issues/10#top
    - Use circular buffer to make sure that context is preserved.
    - Whisper reads in 30 second chunks -> 1 for totality should be enough

Inspiration:
https://stackoverflow.com/questions/78530532/how-to-process-audio-with-whisper-in-rust