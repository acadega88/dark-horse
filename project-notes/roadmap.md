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
- [x] v0.0.8 — Add selectable/closable tabs, locally saved bookmarks with editable names and URLs, bookmark add/edit/delete actions, a toolbar on every tab, and a new-tab chooser for DuckDuckGo or saved bookmarks (manually verified on macOS).
- [x] v0.0.9 — Add request filtering and basic ad/tracker blocking (manually verified on macOS).
  - Research findings: [request-filtering-research.md](request-filtering-research.md)
  - [x] Integrate pinned EasyList network rules through `adblock` and WebKit; keep local first- and third-party test rules (macOS).
  - [x] Convert pinned EasyList and EasyPrivacy snapshots and measure rule coverage (~99.6% of parsed network filters convert). Results are in the research note.
  - [x] Test the combined rules with first- and third-party fetches, scripts, and images; blocked requests did not reach the server and allowed resources loaded (macOS WebKit harness).
  - [x] Manually verify the combined EasyList and EasyPrivacy integration in the full Dark Horse app on macOS (user reports it works).
  - [ ] Review EasyList and EasyPrivacy attribution/licensing before distributing the bundled lists or application.
- [ ] v0.1.0 — Make Dark Horse installable and launchable from the terminal on macOS, Windows, and Linux.
  - [ ] Build platform packages on native GitHub Actions runners.
  - [ ] macOS: package a `.app` in a `.dmg` and provide a terminal installation path into Applications.
  - [ ] Windows: provide an `.msi`/WinGet installation path that creates a Start menu shortcut; document the WebView2 Runtime requirement.
  - [ ] Linux: provide a Flatpak CLI installation that creates an application-menu launcher; support X11 first, with Wayland tracked separately.
  - [ ] Manually test installation, launching, and removal on each target OS before claiming support.
- [ ] v0.1.1 — Add basic permission handling and safe downloads, then review the browser for everyday personal use.

## Platform support

macOS is the current development and verification platform. The v0.1.0 packaging work will use native CI runners for Windows and Linux builds, but builds alone do not verify those platforms. Linux/Wayland still needs a different Wry integration; the initial Linux target is X11. Do not describe a platform as supported until it has been tested there.

## Later possibilities

Password-manager support, multiple windows, additional security hardening, developer tools, performance work, and publishing through package-manager catalogs. Before password-manager work, evaluate established open-source managers and a safe integration path rather than implementing password storage or cryptography from scratch. Browser-extension compatibility with Wry needs investigation.

The sequence may change as we learn what is practical. Dark Horse currently relies on system WebView engines rather than building a complete modern browser engine from scratch; rendering and behavior can differ between platforms.

## Maintenance completed

- Split the application entry point, browser UI/state, and navigation icon drawing into separate Rust modules after v0.0.5. This is code organization work, not a new browser feature milestone.
- Split `browser_app.rs` into `browser_app/ui.rs`, `navigation.rs`, `tabs.rs`, and `bookmarks.rs` after v0.0.8; `request_filter.rs` was added for v0.0.9.
