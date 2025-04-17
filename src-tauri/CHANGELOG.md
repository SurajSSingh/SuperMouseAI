# Changelog

All notable changes to this project will be documented in this file.

## [0.8.4] - 2025-04-17

### 🚀 Features

- *(backend)* :sparkles: Add option to use custom patience value
- *(backend)* :sparkles: Add callbacks for Whisper events
- *(backend)* :label: Add typed events for transcription callback
- :sparkles: Add transcription details
- *(installer)* :sparkles: Add auto download and install Vulkan

### 🐛 Bug Fixes

- *(installer)* :bug: Use inect plugin instead of NSISdl
- *(installer)* :bug: Update flags for silent install

### 🚜 Refactor

- *(backend)* :rotating_light: Add and fix more clippy lints
- *(backend)* :rotating_light: Fix remaining nursery lint warnings
- *(backend)* :recycle: Split apart builder code
- *(backend)* :rotating_light: Fix clippy lint on changes
- *(backend)* :recycle: Split apart setup code
- *(backend)* :rotating_light: Fix lint errors introduced by AI
- *(backend)* :loud_sound: Add trace log for abort and progress callbacks
- *(backend)* :recycle: Model transcribe function changes

### 📚 Documentation

- *(backend)* :memo: Add lint allow reason

### ⚙️ Miscellaneous Tasks

- *(backend)* :white_check_mark: Add new clippy lint subtypes
- *(backend)* :test_tube: Add pedantic clippy lint
- *(backend)* :rotating_light: Use clippy --fix for simple lint fixes
- *(backend)* :rotating_light: Fix remaining pedantic lints
- *(backend)* :rotating_light: Add nursery lints and apply auto fix
- *(backend)* :rotating_light: Fix lint issue in mutter
- :bookmark: Bump to v0.8.4
- *(backend)* :rotating_light: Fix Rust clippy lint issues

### ◀️ Revert

- :rewind: Bring back NSISdl download temporarily

## [0.8.3] - 2025-04-10

### ⚙️ Miscellaneous Tasks

- :bookmark: Bump to v0.8.3

## [0.8.2] - 2025-04-09

### 🚀 Features

- *(backend)* :heavy_plus_sign: Add hardware information query dependencies
- *(backend)* :sparkles: Get system information about user's machine

### ⚙️ Miscellaneous Tasks

- *(backend)* :arrow_up: Update backend dependencies
- :bookmark: Bump to v0.8.2

## [0.8.1] - 2025-04-04

### 🚀 Features

- *(frontend)* :building_construction: Move transcriptions to their own file

### ⚙️ Miscellaneous Tasks

- :bookmark: Bump to v0.8.1

## [0.8.0] - 2025-04-03

### 🚀 Features

- *(installer)* :wrench: Create test hook for NSIS installer
- *(installer)* :alembic: Try out a new template for NSIS installer
- *(backend)* :sparkles: Add functionality to swap models

### 🐛 Bug Fixes

- :wrench: Fix config for windows installer
- *(backend)* :heavy_plus_sign: Add missing Whisper-rs crate for Linux
- *(backend)* :rotating_light: Allow unused import for tauri sentry plugin

### 🚜 Refactor

- *(installer)* :recycle: Use hooks again for NSIS instead of installer template
- :wrench: Add another endpoint for pre-release updates
- *(backend)* :technologist: Add back Sentry code

### ⚙️ Miscellaneous Tasks

- :heavy_plus_sign: Add Tauri updater plugin
- :wrench: Update Tauri config to utilize updater plugin
- :construction: Add LLM auto-complete work for NSIS installer
- *(installer)* :wrench: Update updater configuration
- :bookmark: Bump to v0.7.4
- :adhesive_bandage: Apply version bump to Cargo.lock
- :heavy_plus_sign: Add Tauri HTTP client plugin
- :heavy_plus_sign: Add Tauri File System (fs) Plugin
- :heavy_plus_sign: Add Tauri Dialog Plugin
- :wrench: Update fs permissions for local app data file path
- :wrench: Update permission for http and fs
- :heavy_plus_sign: Add Tauri Upload plugin
- :wrench: Add permission to get value from redirect URL
- *(backend)* :rotating_light: Fix clippy lint for unneeded format
- :bookmark: Bump to v0.8.0

## [0.7.333] - 2025-03-31

### ⚙️ Miscellaneous Tasks

- *(backend)* :arrow_up: Update Rust dependneices in Cargo
- :bookmark: Bump to v0.7.3

## [0.7.2] - 2025-03-28

### 🐛 Bug Fixes

- *(overlay)* :wrench: Hide shadow for transparent overlay

### ⚙️ Miscellaneous Tasks

- :bookmark: Bump to v0.7.2

## [0.7.1] - 2025-03-28

### 🚀 Features

- *(backend)* :wrench: Add new overlay window
- *(backend)* :sparkles: Add mouse position listening event
- *(backend)* :sparkles: Have overlay ignore mouse events
- *(backend)* :sparkles: Close all windows when main window is closed
- *(backend)* :triangular_flag_on_post: Add no-overlay flag and use openmp by default
- *(backend)* :sparkles: Close overlay window when no-overlay is selected

### 🚜 Refactor

- :truck: Use overlay instead of empty for path
- *(backend)* :loud_sound: Add trace level logging for input events

### 🎨 Styling

- *(backend)* :rotating_light: Fix clippy lint
- *(backend)* :art: Run cargo fmt on Rust code
- *(backend)* :rotating_light: Fix small lint warning when running in release mode

### ⚙️ Miscellaneous Tasks

- *(backend)* Update Cargo config to use MacOS private API
- :bookmark: Bump to v0.7.0
- *(backend)* :heavy_plus_sign: Add direct Sentry dependency
- :bookmark: Bump to v0.7.1

### ◀️ Revert

- *(backend)* :triangular_flag_on_post: Remove Sentry feature flag

## [0.6.4] - 2025-03-21

### 🚀 Features

- *(backend)* :passport_control: Add default capability for sentry plugin
- *(backend)* :heavy_plus_sign: Add process Tauri plugin
- *(backend)* :technologist: Add sentry client for sending logging data
- *(backend)* :sparkles: Add system to run export bindings only without running the app
- *(logging)* :loud_sound: Add more logging to app

### 🐛 Bug Fixes

- *(backend)* :bug: Fix bug with mod key down

### 🚜 Refactor

- *(backend)* :art: Format command file using cargo fmt
- *(backend)* :triangular_flag_on_post: Add new logging feature flags
- *(backend)* :recycle: Move export bindings to separate function
- *(backend)* :recycle: Unqualify path for logging in lib.rs
- *(backend)* :recycle: Add collecting functions for commands and events

### ⚙️ Miscellaneous Tasks

- *(backend)* :heavy_plus_sign: Add Tauri Sentry Plugin
- :bookmark: Bump to v0.6.3
- *(backend)* :arrow_up: Update cargo dependencies
- :bookmark: Bump to v0.6.4

## [0.6.2] - 2025-03-15

### 🚀 Features

- *(backend)* :sparkles: Add proper paste from clipboard

### 🐛 Bug Fixes

- *(backend)* :rewind: Use loop to keep thread alive

### 🚜 Refactor

- *(backend)* :rotating_light: Fix`cargo check` warnings
- *(backend)* :rotating_light: Fix clippy lints in Rust

### ⚙️ Miscellaneous Tasks

- :bookmark: Bump to v0.6.2

## [0.6.1] - 2025-03-14

### 🚀 Features

- :label: Create Specta binding for TypeScript side
- :sparkles: Paste text functionality
- :label: Create type for Transcribing options
- :sparkles: Add processing text in backend
- :bug: Make all Process Options optional and trim string
- :recycle: Move inter-sentence newline remover to backend
- *(backend)* :sparkles: Add function to force window to be on top

### 🚜 Refactor

- Resource files now only mp3 or bin
- :recycle: Make the commands async
- :building_construction: Split modules
- :recycle: Use new Transcription Option type in function
- *(backend)* :pencil2: Fix casing for override param in set window top command
- *(backend)* :technologist: Minor command improvements

### ⚙️ Miscellaneous Tasks

- :package: Update Whisper Model
- :bookmark: Bump to v0.5.4
- :heavy_plus_sign: Add Tauri-Specta
- :heavy_plus_sign: Add Enigo crate
- :coffin: Remove unused import
- :heavy_plus_sign: Add Rust regex crate
- :bookmark: Bump to v0.5.5
- :bookmark: Bump to v0.6.0
- :bookmark: Bump to v0.6.1

### ◀️ Revert

- :rewind: Remove preload model feature
- :rewind: Remove model preloading

## [0.5.3] - 2025-03-07

### 🚀 Features

- :heavy_plus_sign: Add Tauri Store plugin
- :heavy_plus_sign: Add Tauri logging plugin
- :technologist: Add logging in-place of printlns

### 🐛 Bug Fixes

- :ambulance: Fix 0 CPU thread
- :bookmark: Bump to v0.5.3

### 🚜 Refactor

- :building_construction: Move commands and associated helper items
- Load model from the start
- :loud_sound: Add more logging points

### 📚 Documentation

- :bulb: Update comment in lib

### 🎨 Styling

- :art: Small Clippy fixes

### ⚙️ Miscellaneous Tasks

- Add new command module
- :arrow_up: Update Cargo deps and features
- :page_facing_up: Add Functional Source License (FSL) (dual MIT-Apache 2.0)
- :truck: Move license files to top folder of repo
- :bookmark: Bump to v0.4.3
- :memo: Add documentation about model
- :bento: Update Whisper model to Small-En-Q5
- :bookmark: Bump to v0.4.4
- :bookmark: Bump to v0.5.0
- :bookmark: Bump to v0.5.1
- :bookmark: Bump to v0.5.2

## [0.4.2] - 2025-02-26

### 🚀 Features

- :sparkles: Basic mod key listener
- :sparkles: Enforce Single Instance

### 🚜 Refactor

- :coffin: Remove unneeded device query import in lib.rs

### ⚙️ Miscellaneous Tasks

- :arrow_up: Update Backend dependencies
- :bookmark: Bump patch version
- :heavy_plus_sign: Add device_query as cargo dependency
- :triangular_flag_on_post: Add vulkan feature flag
- :bookmark: Bump to version v0.4.0
- :wrench: Add new profile options for dev and build
- :bookmark: Bump to v0.4.2

## [0.3.0] - 2025-02-21

### 🚀 Features

- :children_crossing: Add additional outside resource sound path to map
- :sparkles: Basic rebindable shortcut keys

### 🚜 Refactor

- :coffin: Remove old code in lib file

### 🎨 Styling

- :pencil2: Fix typo in client loading sound paths

### ⚙️ Miscellaneous Tasks

- :bookmark: Bump version to v0.3.0

## [0.2.0] - 2025-02-14

### 🚀 Features

- :sparkles: Add mouse press event from backend
- :sparkles: Mouse Shortcut working*
- :sparkles: Add sound effect play

### 🚜 Refactor

- :fire: Remove unused imports in lib.rs
- :sparkles: Add optional parameters to transcribe function

### ⚙️ Miscellaneous Tasks

- :heavy_plus_sign: Add mouce crate
- :bricks: Update settings to prevent Tauri recompile when no changes have occured
- Remove warnings for dead code in transcript
- Version bump
- :heavy_plus_sign: Add notifications plugin
- :bento: Add basic sound resources
- :bento: Replace ogg sound files with mp3
- :heavy_plus_sign: Add Tauri clipboard plugin
- :triangular_flag_on_post: Enable write to clipboard capability

## [0.1.0] - 2025-02-07

### 🚀 Features

- :construction: Working Checkpoint
- :sparkles: Full basic transcribe wokflow
- :truck: Embed Whisper model in app
- :sparkles: Add shortcut hotkey and copy button
- :triangular_flag_on_post: Update feature flag for OS

### 🐛 Bug Fixes

- :bug: Fix bug with permission not resetting
- :rewind: Reverse CUDA requirement

### 🚜 Refactor

- :rewind: Remove change for reset permission
- :construction: Add minimp3 as dep to see if fix
- :recycle: Small updates to frontend and model state
- :building_construction: Move Mutter code into project
- Update decode function
- Use WAV format instead of MP4
- :triangular_flag_on_post: Re-add cuda sub-feature dep when cuda flag active
- :truck: Rename resource model to general name

### ⚙️ Miscellaneous Tasks

- :sparkles: Initialize Tauri Svelte project
- :memo: Add some quick function docs
- :passport_control: Add permission files for MacOS bundle
- :heavy_plus_sign: Add global shortcuts Tauri plugin

<!-- generated by git-cliff -->
