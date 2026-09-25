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

The code is organized into `src/main.rs` for startup, `src/browser_app.rs` for browser state and UI, and `src/icons.rs` for navigation icons.

## Run

From the project directory, use the terminal:

```sh
cargo run
```

## Project notes

- [Architecture](project-notes/architecture.md)
- [Roadmap](project-notes/roadmap.md)
- [Decisions](project-notes/decisions.md)
- [Learning notes](project-notes/LEARNING_NOTES.md)
- [Privacy check instructions](project-notes/privacy-check.md)
- [UI improvements](project-notes/ui-improvements.md)
