# Dark Horse Browser

Dark Horse is a long-term personal project to build a small, fast, privacy-focused web browser. It is also a learning project: the code should stay understandable, and each new idea should be explained before it is added.

## Project principles

- Do as little as possible, and do it well.
- Do not keep browsing history, cookies, cache, or website storage after the browser closes by default.
- Do not add accounts, sync, telemetry, or tracking.
- Keep local bookmarks as an intentional exception.
- Measure performance and compatibility instead of assuming that a privacy choice is faster or harmless.
- Prefer small, incremental changes that the project owner can explain.

## Platforms and technology

The target platforms are macOS, Linux, and Windows. The main language is Rust. Browser controls use `eframe`/`egui`; Wry displays pages through the operating system's WebView engine. Dark Horse uses an existing engine rather than building a complete rendering engine itself.

## Current status

Milestones v0.0.1 through v0.0.8 are implemented and manually verified on macOS. The privacy restart check confirmed that cookies, `localStorage`, Cache API data, and the cacheable HTTP response do not persist after Dark Horse closes. Dark Horse opens a larger, resizable window with DuckDuckGo as its start page, sends non-URL address-bar text to DuckDuckGo, opens domains directly, and provides back, forward, reload, tabs, and local bookmarks. Bare public domains default to HTTPS; `localhost` and loopback IP addresses default to HTTP. Google and YouTube have been tried successfully. Linux and Windows have not been verified, and Linux/Wayland needs a different Wry integration.

The v0.0.8 work adds selectable and closable tabs, locally saved bookmarks with add, edit, and delete actions, editable bookmark names and URLs, and a bookmark toolbar beneath the address bar on every tab. Opening a new tab offers DuckDuckGo and saved bookmarks. This work has been manually verified on macOS.

The v0.0.9 request-filtering work is in progress on macOS. Dark Horse converts pinned EasyList and EasyPrivacy snapshots into WebKit rules and attaches them to each tab. A combined-list WebKit check passes; the user manually verified the full app with EasyList alone, and the combined lists still need manual verification. Linux and Windows have no request filtering yet.

The code is organized into `src/main.rs` for startup, `src/browser_app.rs` for shared browser state and action routing, `src/browser_app/` for the UI, navigation, tabs, bookmarks, and request-filter modules, `src/user_data.rs` for bookmark storage, and `src/icons.rs` for navigation icons.

## Run

From the project directory, use the terminal:

```sh
cargo run
```

## Installable package previews

The **Build desktop packages** GitHub Actions workflow builds macOS, Windows, and Linux artifacts on their native runners. It runs manually or on pull requests that touch packaging files, uploads test artifacts for 14 days, and does not publish a release or install Dark Horse on the build machine.

Download and extract the artifact for your operating system, then run its installer helper:

- macOS: `./install-macos.sh` copies the app from the DMG into `~/Applications`.
- Windows: `powershell -ExecutionPolicy Bypass -File .\install-windows.ps1` installs the MSI and creates Start menu and desktop shortcuts; Windows will request administrator approval.
- Linux: `./install-linux.sh` installs the Flatpak for your user and adds its launcher to the application menu. The initial package targets X11; Wayland remains unsupported by the current Wry integration.

These preview packages are unsigned and may trigger macOS Gatekeeper or Windows SmartScreen warnings. Code signing, notarization, and public release publishing are not configured yet.

## Project notes

- [Architecture](project-notes/architecture.md)
- [Roadmap](project-notes/roadmap.md)
- [Decisions](project-notes/decisions.md)
- [Learning notes](project-notes/LEARNING_NOTES.md)
- [Privacy check instructions](project-notes/privacy-check.md)
- [UI improvements](project-notes/ui-improvements.md)
