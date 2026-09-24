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

The target platforms are macOS, Linux, and Windows. The main language is Rust. The first window prototype uses `winit` for cross-platform window creation and event handling. A web rendering engine has not yet been selected.

## Current status

The Rust project has been initialized, and `winit` has been added as a dependency. The starter program has run successfully and currently prints `Hello, world!`; opening a window is the next milestone.

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
