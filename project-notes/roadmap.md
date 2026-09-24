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
- [x] v0.0.6 — Verify temporary-only cookies, cache, and site storage after the app closes (verified on macOS). The restart check found cookies, `localStorage`, and Cache API data absent; the cacheable HTTP resource was requested from the server again. Windows requires WebView2 Runtime 101.0.1210.39 or newer for Wry incognito mode and remains unverified; Linux remains unverified too.
- [x] v0.0.7 — Treat non-URL address-bar input as a DuckDuckGo search; continue opening recognized URLs and domains directly (verified on macOS, including search text, public domains, and `localhost:8765`). Public domains default to HTTPS; loopback addresses default to HTTP.
- [ ] v0.0.8 — Add local bookmarks and a new-tab preference: show bookmarks or open DuckDuckGo (macOS).
- [ ] v0.0.9 — Add request filtering and basic ad/tracker blocking; this is a core project feature (macOS).
- [ ] v0.1.0 — Add basic permission handling and safe downloads, then review the macOS prototype for everyday personal use.

## Platform support

macOS is the current development and verification platform. Windows and Linux remain project targets, but testing them is postponed until a personal or otherwise suitable test environment is available; this does not block the macOS roadmap. Linux/Wayland still needs a different Wry integration. Do not describe a platform as supported until it has been tested there.

## Later possibilities

Password-manager support, multiple windows, additional security hardening, developer tools, performance work, and release packaging for macOS, Linux, and Windows. Before password-manager work, evaluate established open-source managers and a safe integration path rather than implementing password storage or cryptography from scratch. Browser-extension compatibility with Wry needs investigation.

The sequence may change as we learn what is practical. Dark Horse currently relies on system WebView engines rather than building a complete modern browser engine from scratch; rendering and behavior can differ between platforms.

## Maintenance completed

- Split the application entry point, browser UI/state, and navigation icon drawing into separate Rust modules after v0.0.5. This is code organization work, not a new browser feature milestone.
