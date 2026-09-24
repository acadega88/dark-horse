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

The target platforms are macOS, Linux, and Windows. The main language is Rust. The v0.0.2 browser UI uses `eframe`/`egui` to provide cross-platform widgets. A web rendering engine has not yet been selected.

## Current status

Milestones v0.0.1 through v0.0.3 are complete on macOS: the application opens a window, provides an address field, and makes basic HTTP/HTTPS requests. Entering a domain such as `google.com` defaults to HTTPS. The app shows the final response address and HTTP status, but does not render the page yet. Linux and Windows have not been verified yet.

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
