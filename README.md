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

Milestones v0.0.1 through v0.0.5 are complete on macOS. Dark Horse opens a larger, resizable window with DuckDuckGo as its start page, accepts an address, defaults bare domains to HTTPS, displays real pages through Wry, and provides back, forward, and reload controls. Google and YouTube have been tried successfully. The WebView's incognito option is enabled, but temporary-only storage behavior has not been verified. Linux and Windows have not been verified, and Linux/Wayland needs a different Wry integration.

## Run

From the project directory, use the terminal:

```sh
cargo run
```

## Project notes

- [Architecture](architecture.md)
- [Roadmap](roadmap.md)
- [Decisions](decisions.md)
- [Learning notes](LEARNING_NOTES.md)
