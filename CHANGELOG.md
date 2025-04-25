# Changelog

All notable changes to this project will be documented in this file.

## [current]

### 🚀 Features

- *(backend)* :sparkles: Add new decode and denoise function
- *(backend)* :sparkles: Add command setup and pre-normalize step
- *(frontend)* :sparkles: Add decode option to transcription function
- *(frontend)* :sparkles: Add new audio option toggles

### 🐛 Bug Fixes

- *(backend)* :bug: Fix denoising not being applying

### 🚜 Refactor

- *(backend)* :rotating_light: Fix clippy lint issues with new function

### 🎨 Styling

- *(frontend)* :coffin: Remove unneeded imports from AudioTranscriber

### ⚙️ Miscellaneous Tasks

- V0.8.4 update files
- *(backend)* :heavy_plus_sign: Add NNNoiseless crate

## [0.8.4] - 2025-04-17

### 🚀 Features

- *(frontend)* :label: Add recommendations for (V)RAM
- *(backend)* :sparkles: Add option to use custom patience value
- *(frontend)* :sparkles: Add input to change patience value
- *(backend)* :sparkles: Add callbacks for Whisper events
- *(backend)* :label: Add typed events for transcription callback
- *(frontend)* :construction: Add frontend side for callback feature
- :sparkles: Add transcription details
- *(installer)* :sparkles: Add auto download and install Vulkan

### 🐛 Bug Fixes

- *(frontend)* :bug: Fix missing clear all transcript function
- *(frontend)* :speech_balloon: Add missing patience field information
- *(installer)* :bug: Use inect plugin instead of NSISdl
- *(installer)* :bug: Update flags for silent install

### 🚜 Refactor

- *(backend)* :rotating_light: Add and fix more clippy lints
- *(backend)* :rotating_light: Fix remaining nursery lint warnings
- *(backend)* :recycle: Split apart builder code
- *(backend)* :rotating_light: Fix clippy lint on changes
- *(backend)* :recycle: Split apart setup code
- *(backend)* :rotating_light: Fix lint errors introduced by AI
- *(frontend)* :construction: Add alternative option for mouse position
- *(frontend)* :recycle: Revamp toast notification
- *(backend)* :loud_sound: Add trace log for abort and progress callbacks
- *(frontend)* :recycle: Improve Transcription Info Type
- *(frontend)* Change default patience to 2
- *(backend)* :recycle: Model transcribe function changes
- *(frontend)* :recycle: Require recommended VRAM

### 📚 Documentation

- *(backend)* :memo: Add lint allow reason

### ⚙️ Miscellaneous Tasks

- *(backend)* :white_check_mark: Add new clippy lint subtypes
- *(backend)* :test_tube: Add pedantic clippy lint
- *(backend)* :rotating_light: Use clippy --fix for simple lint fixes
- *(backend)* :rotating_light: Fix remaining pedantic lints
- *(backend)* :rotating_light: Add nursery lints and apply auto fix
- :label: Update bindings.ts docs changes
- :rotating_light: Fix lint error for vite config
- *(backend)* :rotating_light: Fix lint issue in mutter
- :wrench: Add full check Deno task
- :bookmark: Bump to v0.8.4
- :green_heart: Install dependencies before running check
- *(backend)* :rotating_light: Fix Rust clippy lint issues

### ◀️ Revert

- :rewind: Bring back NSISdl download temporarily

## [0.8.3] - 2025-04-10

### 🐛 Bug Fixes

- *(frontend)* :goal_net: Add catch for JS async errors

### 🚜 Refactor

- *(frontend)* :fire: Remove deprecated update available

### ⚙️ Miscellaneous Tasks

- Update version to v0.8.2
- :bookmark: Bump to v0.8.3

## [0.8.2] - 2025-04-09

### 🚀 Features

- *(backend)* :heavy_plus_sign: Add hardware information query dependencies
- *(backend)* :sparkles: Get system information about user's machine
- *(frontend)* :sparkles: Download model when no model is found
- *(frontend)* :sparkles: Add actual find best model

### 🚜 Refactor

- *(frontend)* :recycle: Move all pre setup functions into single function
- *(frontend)* :recycle: Model object update
- *(frontend)* :recycle: Save user preference for telemetry

### 📚 Documentation

- *(frontend)* :memo: Add documentation to functions in myUtils

### ⚙️ Miscellaneous Tasks

- :bookmark: Update latest version to v0.8.1
- *(frontend)* :arrow_up: Update front-end dependencies
- *(backend)* :arrow_up: Update backend dependencies
- :green_heart: Remove CPU only build for simple windows CI
- :bookmark: Bump to v0.8.2

## [0.8.1] - 2025-04-04

### 🚀 Features

- *(frontend)* :building_construction: Move transcriptions to their own file

### 🐛 Bug Fixes

- *(frontend)* :bug: Fix save transcript bug
- *(ui)* :lipstick: Fix z-index glitch in Model Dropdown
- *(frontend)* :bug: Fix window float bug

### 🚜 Refactor

- *(frontend)* :pencil2: Use better name for save checking
- *(frontend)* :recycle: Improve download progress
- *(frontend)* :label: Improve WhisperModelInfo Type

### ⚙️ Miscellaneous Tasks

- Add more models
- :hammer: Add changelog generation to version bump task
- :bookmark: Bump to v0.8.1

## [0.8.0] - 2025-04-03

### 🚀 Features

- *(installer)* :wrench: Create test hook for NSIS installer
- *(frontend)* :sparkles: Basic Update checker component with download button
- *(installer)* :alembic: Try out a new template for NSIS installer
- *(ui)* :lipstick: Create new collapsable fieldset UI component
- *(frontend)* :sparkles: Add toggle config for updater
- *(frontend)* :sparkles: Use config value for updater
- *(frontend)* :sparkles: Add native dialog to notifier
- :label: Add whisper model info type and constant
- *(frontend)* :sparkles: Add UI for Model management
- *(frontend)* :sparkles: Add model config items
- *(frontend)* :sparkles: Implement proper add/remove model
- *(frontend)* :sparkles: Add Model selecting dropdown
- *(frontend)* :sparkles: Add function to convert bytes info into human readable format
- *(frontend)* :sparkles: Add filter system for models
- *(frontend)* :sparkles: Add toggle for GPU use
- *(backend)* :sparkles: Add functionality to swap models
- *(frontend)* :sparkles: Connect model dropdown to model switching function

### 🐛 Bug Fixes

- :wrench: Fix config for windows installer
- *(frontend)* Run update check after telemetry is accepted
- *(frontend)* :goal_net: Catch update error when target is not found
- *(backend)* :heavy_plus_sign: Add missing Whisper-rs crate for Linux
- *(backend)* :rotating_light: Allow unused import for tauri sentry plugin
- *(frontend)* :bug: Fix model not removed bug

### 🚜 Refactor

- *(installer)* :recycle: Use hooks again for NSIS instead of installer template
- :wrench: Add another endpoint for pre-release updates
- *(frontend)* :recycle: Unify App configuration
- *(frontend)* :recycle: Use native dialog for telemetry request
- *(frontend)* :recycle: Move initial prompt to model config
- *(frontend)* :building_construction: Add new consts and util function
- *(ui)* :recycle: Add dropdown caret for theme dropdown
- *(frontend)* :recycle: Use new download function in model manager
- *(frontend)* :label: Add additonal information for whisper model
- *(frontend)* :recycle: Improve model management component
- *(frontend)* :coffin: Remove commented code from WhisperOptions
- *(backend)* :technologist: Add back Sentry code

### ⚙️ Miscellaneous Tasks

- :heavy_plus_sign: Add Tauri updater plugin
- :wrench: Update Tauri config to utilize updater plugin
- :wrench: Create git-cliff config for changelog
- :construction: Add LLM auto-complete work for NSIS installer
- :memo: Add a changelog
- *(frontend)* :construction: Add toggle items for update config
- :wrench: Create a new latest.json within repo
- *(installer)* :wrench: Update updater configuration
- :bookmark: Bump to v0.7.4
- :adhesive_bandage: Apply version bump to Cargo.lock
- :green_heart: Attempt fix of Rust check by installing Ubuntu dependencies
- :green_heart: Add missing ALSA lib dependency
- :green_heart: Add empty whisper-model file
- :construction_worker: Allow clippy for all targets again
- :heavy_plus_sign: Add Tauri HTTP client plugin
- :heavy_plus_sign: Add Tauri File System (fs) Plugin
- :heavy_plus_sign: Add Tauri Dialog Plugin
- :wrench: Update fs permissions for local app data file path
- :wrench: Update permission for http and fs
- :heavy_plus_sign: Add Tauri Upload plugin
- :wrench: Add permission to get value from redirect URL
- Add small model for download testing
- *(backend)* :rotating_light: Fix clippy lint for unneeded format
- :bookmark: Bump to v0.8.0

## [0.7.333] - 2025-03-31

### ⚙️ Miscellaneous Tasks

- :construction_worker: Make OpenMP feature explicit for build
- *(backend)* :arrow_up: Update Rust dependneices in Cargo
- *(frontend)* :arrow_up: Update JS Dependnecies
- :bookmark: Bump to v0.7.3

## [0.7.2] - 2025-03-28

### 🚀 Features

- *(frontend)* :wrench: Use inline bundleStrategy
- *(frontend)* :sparkles: Add a new homepage to link to app and overlay

### 🐛 Bug Fixes

- *(overlay)* :wrench: Hide shadow for transparent overlay

### 🚜 Refactor

- *(frontend)* :pencil2: Use name of app in title
- *(frontend)* :wrench: Undo inlining and use absolute paths

### ⚙️ Miscellaneous Tasks

- :construction_worker: Remove no-overlay variant in matrix
- :bookmark: Bump to v0.7.2

## [0.7.1] - 2025-03-28

### 🚀 Features

- *(backend)* :wrench: Add new overlay window
- *(backend)* :sparkles: Add mouse position listening event
- *(backend)* :sparkles: Have overlay ignore mouse events
- *(frontend)* :building_construction: Split pages for overlay and app
- *(frontend)* :sparkles: Add event for recording state change for overlay
- *(frontend)* :sparkles: Add retry mechanism for confirmation
- *(backend)* :sparkles: Close all windows when main window is closed
- *(backend)* :triangular_flag_on_post: Add no-overlay flag and use openmp by default
- *(backend)* :sparkles: Close overlay window when no-overlay is selected

### 🚜 Refactor

- :truck: Use overlay instead of empty for path
- *(ui)* :lipstick: Small stylistic changes to overlay
- *(backend)* :loud_sound: Add trace level logging for input events
- *(frontend)* :recycle: Move Menu items directly into MenuScreen
- *(overlay)* :bug: Fix cursor centering on Windows
- *(frontend)* :art: Format frontend code to pass CI check
- *(frontend)* :rotating_light: Fix lint location for showToast function in notification system

### 🎨 Styling

- *(overlay)* :lipstick: Use new animation for overlay
- *(backend)* :rotating_light: Fix clippy lint
- *(frontend)* :rotating_light: Ignore Deno lint issue in vite config
- *(backend)* :art: Run cargo fmt on Rust code
- *(backend)* :rotating_light: Fix small lint warning when running in release mode
- *(frontend)* :art: Fix notification system file formatting

### ⚙️ Miscellaneous Tasks

- *(backend)* Update Cargo config to use MacOS private API
- :bookmark: Bump to v0.7.0
- *(backend)* :heavy_plus_sign: Add direct Sentry dependency
- :bookmark: Bump to v0.7.1
- :construction_worker: Adapt old win-simple-build GitHub Action for new simple workflow
- :wrench: Update Deno config's fmt option
- :construction_worker: Add GitHub Action for format and lint checks
- :construction_worker: Update windows-simple-build with release configuration to test
- :green_heart: Add missing v for install Vulkan SDK action
- :construction_worker: Use main branch for simple windows builds
- :green_heart: Change to src-tauri directory before running Rust checks
- :construction_worker: Rename job names
- :construction_worker: Remove clippy lint for all targets

### ◀️ Revert

- *(backend)* :triangular_flag_on_post: Remove Sentry feature flag

## [0.6.4] - 2025-03-21

### 🚀 Features

- *(backend)* :passport_control: Add default capability for sentry plugin
- *(backend)* :heavy_plus_sign: Add process Tauri plugin
- *(backend)* :technologist: Add sentry client for sending logging data
- *(backend)* :sparkles: Add system to run export bindings only without running the app
- *(logging)* :loud_sound: Add more logging to app
- *(frontend)* :sparkles: Show app version string to user

### 🐛 Bug Fixes

- *(backend)* :bug: Fix bug with mod key down

### 🚜 Refactor

- *(backend)* :art: Format command file using cargo fmt
- *(backend)* :triangular_flag_on_post: Add new logging feature flags
- *(backend)* :recycle: Move export bindings to separate function
- *(backend)* :recycle: Unqualify path for logging in lib.rs
- *(backend)* :recycle: Add collecting functions for commands and events
- *(frontend)* :fire: Remove old components

### 📚 Documentation

- *(github)* :memo: Create pull request template
- *(github)* :memo: Create issues templates

### ⚙️ Miscellaneous Tasks

- *(backend)* :heavy_plus_sign: Add Tauri Sentry Plugin
- *(frontend)* :hammer: Create Deno task for refreshing the App TS bindings
- :bookmark: Bump to v0.6.3
- *(frontend)* :arrow_up: Update JS dependencies
- *(backend)* :arrow_up: Update cargo dependencies
- :bookmark: Bump to v0.6.4

## [0.6.2] - 2025-03-15

### 🚀 Features

- *(backend)* :sparkles: Add proper paste from clipboard
- *(frontend)* :sparkles: Add paste via keyboard option

### 🐛 Bug Fixes

- *(frontend)* :bug: Close dialog before showing confirmation toast
- *(backend)* :rewind: Use loop to keep thread alive

### 🚜 Refactor

- *(frontend)* :rotating_light: Fix lint rules from Deno
- *(backend)* :rotating_light: Fix`cargo check` warnings
- *(backend)* :rotating_light: Fix clippy lints in Rust

### ⚙️ Miscellaneous Tasks

- *(frontend)* :wrench: Add Deno.json (for linting rules)
- :label: Binding update with writeText function
- :bookmark: Bump to v0.6.2

## [0.6.1] - 2025-03-14

### 🚀 Features

- :label: Create Specta binding for TypeScript side
- :sparkles: Paste text functionality
- :label: Create type for Transcribing options
- :sparkles: Add processing text in backend
- :bug: Make all Process Options optional and trim string
- :recycle: Move inter-sentence newline remover to backend
- *(ui)* :lipstick: Create basic Switch
- *(backend)* :sparkles: Add function to force window to be on top
- *(ui)* :sparkles: Create Toggle Switch component
- *(frontend)* :building_construction: Improve notification system
- *(frontend)* :wrench: Update store config
- *(frontend)* :sparkles: Add option to enforce window on top
- *(frontend)* :sparkles: Add option to toggle new features

### 🐛 Bug Fixes

- *(frontend)* :bug: Fix reactivity bug with StoreOption
- *(frontend)* :bug: Fix shortcut not updating on mod key change
- *(frontend)* :bug: Fix transcription not updating issue
- *(frontend)* :bug: Fix value not updating on load
- *(frontend)* :adhesive_bandage: Temporary fix for tab in light mode

### 🚜 Refactor

- Resource files now only mp3 or bin
- :recycle: Make the commands async
- :building_construction: Split modules
- :recycle: Refactor frontend to use new bindings
- :recycle: Use new Transcription Option type in function
- :recycle: Use transcribe with post processing in front-end
- *(frontend)* :building_construction: Move general notifier to notification system
- *(backend)* :pencil2: Fix casing for override param in set window top command
- *(frontend)* :recycle: Move Danger Zone items in its own component
- *(frontend)* :label: Add type for auto-syncing store option
- *(frontend)* :recycle: Small refactor for config theme
- *(frontend)* :recycle: Store theme as public variable
- *(frontend)* :construction: Update to new config system
- *(backend)* :technologist: Minor command improvements
- *(frontend)* :coffin: Remove unneeded inspect in ToggleSwitch

### 🧪 Testing

- *(frontend)* :white_check_mark: Create Toast testing routing

### ⚙️ Miscellaneous Tasks

- :package: Update Whisper Model
- :bookmark: Bump to v0.5.4
- :heavy_plus_sign: Add Tauri-Specta
- :heavy_plus_sign: Add Enigo crate
- :coffin: Remove unused import
- :heavy_plus_sign: Add Rust regex crate
- :bookmark: Bump to v0.5.5
- *(vscode)* Add new scopes
- :label: Update TS Bindings
- :bookmark: Bump to v0.6.0
- :bookmark: Bump to v0.6.1

### ◀️ Revert

- :rewind: Remove preload model feature
- :rewind: Remove model preloading

## [0.5.3] - 2025-03-07

### 🚀 Features

- :lipstick: Add Dialog UI component
- Add directionality and class override to theme switcher
- :lipstick: Create Menu Screen using dialog component
- :lipstick: Create new permission components
- :iphone: Improve homepage design
- :heavy_plus_sign: Add Tauri Store plugin
- :sparkles: Create basic store
- :sparkles: Add loading config info for theme and transcripts
- :sparkles: Add confirmation toast
- :wrench: Update config store with new values
- :sparkles: Remove newline after words for transcriptions
- :sparkles: Add Danger Zone Tab
- :heavy_plus_sign: Add Tauri logging plugin
- Add button and function to clear only the transcripts
- :technologist: Add logging in-place of printlns

### 🐛 Bug Fixes

- :adhesive_bandage: Fix small bug when registering new shortcut, forgot to check if registered in the first place
- :bug: Fix shortcut modifier key not loaded on start
- :bug: Fix notification permission issue
- :ambulance: Fix 0 CPU thread
- :bookmark: Bump to v0.5.3

### 🚜 Refactor

- :building_construction: Move commands and associated helper items
- :lipstick: Have audio transcription preview show text left aligned
- :coffin: Remove dead code/imports
- :building_construction: Separate my utils from Shadcn Svelte utils
- Move component files to appropriate folders
- :coffin: Remove comment code for keycode
- Reduce Custom Shortcut component info
- :lipstick: Give Shortcut listening button more space for text
- Remove HR from MicRecorder
- :lipstick: Small improvements for permission button and bar
- :adhesive_bandage: Disable re-transcribe button if no recording
- :building_construction: Modify config class to be a svelte reactive class
- :building_construction: Move theme constant into theme switcher
- :building_construction: Move most state value of AudioTranscriber to ConfigStore
- Update store
- Minor changes to notification system
- Improve delete transcription message
- :building_construction: Improve store structure
- :building_construction: Use config store values to replace inner component variables
- Load model from the start
- :loud_sound: Add more logging points

### 📚 Documentation

- :bulb: Update comment in lib
- :bulb: Improve Svelte component comment

### 🎨 Styling

- :art: Small Clippy fixes

### ⚙️ Miscellaneous Tasks

- Add new command module
- :arrow_up: Update Cargo deps and features
- :page_facing_up: Add Functional Source License (FSL) (dual MIT-Apache 2.0)
- :memo: Update README credits
- :memo: Fix license name in README
- :truck: Move license files to top folder of repo
- :pencil2: Fix typo in local license links
- :arrow_up: Update (most) outdated package.json dependencies
- :arrow_up: Update BitsUI
- :arrow_up: Update Svelte
- :construction_worker: Add a system to go to next patch or minor version with Deno task
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
- Working Customizable Shortcut with listener
- :sparkles: Add ignored words option
- Main page applies margin and use Custom Shortcut component
- :sparkles: Add show info toast and set sound for notification system
- Show toast when copied to clipboard
- :sparkles: Allow multiple transcriptions for AudioTranscriber component
- :sparkles: Enforce Single Instance

### 🐛 Bug Fixes

- :lipstick: Fix UI glitch with theme switcher and tab
- :bug: Fix issue with microphone permission not being accurate
- :bug: Small fix with transcribed output not following current transcript index
- :bug: Fix bug for unmodified shortcuts

### 🚜 Refactor

- :coffin: Remove unneeded device query import in lib.rs
- :pencil2: Remove debug info from custom shortcut title

### ⚙️ Miscellaneous Tasks

- :arrow_up: Update Frontend depedencies
- :arrow_up: Update Backend dependencies
- :bookmark: Bump patch version
- :heavy_plus_sign: Add device_query as cargo dependency
- :lipstick: Remove x-margin for components to allow parent to control
- :triangular_flag_on_post: Add vulkan feature flag
- :bookmark: Bump to version v0.4.0
- :wrench: Add new profile options for dev and build
- :bookmark: Bump to v0.4.2

## [0.3.0] - 2025-02-21

### 🚀 Features

- :lipstick: Create test button from Shadcn-Svelte
- :lipstick: Enable DaisyUI theme for light and add dark theme
- :lipstick: Create DaisyUI textarea from Shadcn-Svelte template
- :lipstick: Add default Shadcn-Svelte Accordion UI component
- :lipstick: Create Single Collapsable component for Whisper option
- :sparkles: Add Theme Switcher dropdown
- :lipstick: Add Tab component
- :building_construction: Create and update pages for tab
- :sparkles: Add Loading Component
- :lipstick: Create Status component using DaisyUI
- :sparkles: Update permission page to show granted permissions
- :children_crossing: Add additional outside resource sound path to map
- :sparkles: Basic rebindable shortcut keys
- :sparkles: Add in app toast pop-up

### 🐛 Bug Fixes

- :bug: Fix Permission page error when setting up mic
- :bug: Fix reactive issue with notification permission
- :lipstick: Fix CSS issue with toast
- :bug: Fix sound not playing for notifications

### 🚜 Refactor

- :lipstick: Adapt button component to use DaisyUI
- Add new variations on button
- :lipstick: Update components to use common button and textarea
- :lipstick: Update main app page to use button component
- :lipstick: Modify WhisperOptions component to use new SingleCollapable component
- :rotating_light: Fix warning about using state value for notification constructor
- :lipstick: Unify radius style for light-dark themes
- Update homepage to use tab for settings
- Use Loading in MicRecorder during processing
- Add X button to close tabs and remove old tab code
- :recycle: Add in-between state for permissions
- :building_construction: Improve shortcut registration
- :coffin: Remove old code in lib file
- Update notification system toasts
- :recycle: Refine theme switcher system to work with toast notification
- Remove theme from title

### 🎨 Styling

- :pencil2: Name prop type in Loading as LoadingProp
- :pencil2: Fix typo in client loading sound paths

### ⚙️ Miscellaneous Tasks

- :package: Move bits and daisy UI libraries to dev-dep
- :heavy_plus_sign: Update dependencies from manual Shadcn-Svelte install
- :bookmark: Bump version to v0.3.0

## [0.2.0] - 2025-02-14

### 🚀 Features

- :sparkles: Add mouse press event from backend
- :sparkles: Add listener for frontend
- :sparkles: Mouse Shortcut working*
- :sparkles: Transcription area editable
- :sparkles: Working notifications
- :sparkles: Add sound effect play
- :sparkles: Add supporting lib TS files
- :sparkles: Basic Audio previewer component
- :sparkles: Add Recorder and Transcriber components
- :sparkles: Add notification system class
- :sparkles: Add Shortcut Setting component
- Add recorder permission function
- :sparkles: Create Whisper Option panel component
- :lipstick: Make recording button more prominent
- :sparkles: Save working chunk for transcriber
- :sparkles: Update homepage design

### 🐛 Bug Fixes

- :rewind: Fix shortcut registration
- :bug: Fix bind for transcribed text area value

### 🚜 Refactor

- :fire: Remove unused imports in lib.rs
- :coffin: Remove unneeded imports and unused code
- :building_construction: Replace monolith with components for homepage
- :sparkles: Add optional parameters to transcribe function

### 🎨 Styling

- :pencil2: Fix typo on page
- Replace relative path lib to $lib for imports
- :pencil2: Use plural for threads variable in WhisperOptions

### ⚙️ Miscellaneous Tasks

- :memo: Update README to current project
- :heavy_plus_sign: Add mouce crate
- :bricks: Update settings to prevent Tauri recompile when no changes have occured
- Remove warnings for dead code in transcript
- Version bump
- :heavy_plus_sign: Add notifications plugin
- :bento: Add basic sound resources
- :bento: Replace ogg sound files with mp3
- :heavy_plus_sign: Add Tauri clipboard plugin
- :triangular_flag_on_post: Enable write to clipboard capability
- :lipstick: Add Daisy UI (without styles applied)
- :heavy_plus_sign: Add Bits-UI as headless library

## [0.1.0] - 2025-02-07

### 🚀 Features

- :heavy_plus_sign: Add TailwindCSS v4
- :construction: Working Checkpoint
- :sparkles: Audio record and preview checkpoint
- :sparkles: Full basic transcribe wokflow
- :truck: Embed Whisper model in app
- :sparkles: Add shortcut hotkey and copy button
- :triangular_flag_on_post: Update feature flag for OS

### 🐛 Bug Fixes

- :goal_net: Catch error when gettingUserMedia
- :bug: Fix bug with permission not resetting
- :bug: Blob bytes not in Chrome
- :construction: Check if this works for Windows version
- :rewind: Reverse CUDA requirement

### 🚜 Refactor

- :rewind: Remove change for reset permission
- :construction: Attempt fix for blob to bytes with stream
- :construction: Add minimp3 as dep to see if fix
- :construction: Use MPEG instead of mp3
- :recycle: Small updates to frontend and model state
- :building_construction: Move Mutter code into project
- :goal_net: Catch error from front-end when transcription fails
- Update decode function
- Use WAV format instead of MP4
- Add microphone to beginning of recording text
- :triangular_flag_on_post: Re-add cuda sub-feature dep when cuda flag active
- :truck: Rename resource model to general name

### ⚙️ Miscellaneous Tasks

- :sparkles: Initialize Tauri Svelte project
- :memo: Add some quick function docs
- :passport_control: Add permission files for MacOS bundle
- :heavy_plus_sign: Add extendable-media-recorder + wav-encoder dep
- :heavy_plus_sign: Add global shortcuts Tauri plugin
- :fire: Remove pnpm_lock

<!-- generated by git-cliff -->
