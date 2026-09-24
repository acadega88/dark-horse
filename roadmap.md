# Roadmap

The roadmap is a learning guide, not a fixed contract. Each stage should remain small enough to understand and verify.

## Current milestone

- [x] v0.0.0 — Initialize the Rust project and run the starter program.
- [x] v0.0.1 — Open and run a basic application window (verified on macOS).
- [x] v0.0.2 — Add an editable address bar (verified on macOS).

## Browser prototype

- [x] v0.0.3 — Implement basic HTTP/HTTPS requests from the address bar; bare domains default to HTTPS (verified on macOS).
- [x] v0.0.4 — Display real web pages through Wry and the operating system's WebView engine (Google and YouTube verified on macOS).
- [x] v0.0.5 — Add back, forward, and reload controls; keep the address field in sync with page navigation (verified on macOS).
- [ ] v0.0.6 — Verify and enforce temporary-only cookies, cache, and site storage after the app closes.
- [ ] v0.0.7 — Support the WebView on macOS, Windows, and Linux, including Linux/Wayland.
- [ ] v0.0.8 — Add basic permission handling and safe download behavior.
- [ ] v0.0.9 — Add request filtering and basic ad/tracker blocking.
- [ ] v0.1.0 — Reach a minimal, genuinely usable browser prototype.

## Later possibilities

Local bookmarks, multiple windows, additional security hardening, developer tools, performance work, and release packaging for macOS, Linux, and Windows.

The sequence may change as we learn what is practical. Dark Horse currently relies on system WebView engines rather than building a complete modern browser engine from scratch; rendering and behavior can differ between platforms.
