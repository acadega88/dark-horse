# Architecture

This document records the architecture as it is understood today. It is intentionally small; future browser decisions should be made when the project reaches them.

## Current shape

```text
src/main.rs        Application entry point and startup window options
src/browser_app.rs BrowserApp state, address handling, navigation, and UI
src/icons.rs       Drawing for the back, forward, and reload controls
Cargo.toml         Package metadata and Rust dependencies
Cargo.lock         Exact dependency versions selected by Cargo
```

The initial application is a normal Rust executable. The v0.0.1 prototype uses `winit` directly to create a native window and handle events; it opens and closes on macOS. Linux and Windows have not been verified yet. `winit` does not provide widgets or draw window contents.

For v0.0.2, the app migrated to `eframe`/`egui` for browser UI controls. `eframe` supplies the app framework and renderer; `egui` supplies widgets. The prototype shows an editable address field on macOS.

For v0.0.3, the selected networking approach is `reqwest` with its blocking HTTP client running on a standard-library worker thread. This avoids adding an async runtime while keeping network waits off the UI thread. The app uses a 20-second request timeout, defaults bare domains to HTTPS, and displays the final response address and HTTP status. HTTPS uses Rustls. No cookie-store feature is enabled; website rendering is a later stage.

For v0.0.4, page display uses Wry, a cross-platform wrapper around each operating system's WebView. The existing `eframe`/`egui` address controls remain in the app window, with the WebView embedded below them as a child view. Google and YouTube have been opened successfully on macOS. The WebView is configured for incognito mode, but temporary-only storage behavior has not yet been verified. Wry documents that Windows requires WebView2 Runtime 101.0.1210.39 or newer for incognito mode; older runtimes ignore the setting. Wry's child-view approach works on macOS, Windows, and Linux/X11; Linux/Wayland needs a GTK-based integration and remains to be addressed. Page rendering and engine behavior come from the OS and may differ between platforms.

For v0.0.5, the app keeps the same `eframe` window and address bar while adding back, forward, and reload controls. The controls query and use the WebView's own navigation history so link clicks and page navigations stay in sync. The address field follows the current page, and a bare domain becomes `https://...`. DuckDuckGo is the start page, and the initial window size is 1800×1100 logical pixels. This remains a small prototype and relies on the platform WebView for page rendering.

For v0.0.7, the address bar keeps explicit HTTP/HTTPS URLs and recognized domains as direct navigation. Public domains default to HTTPS; `localhost` and loopback IP addresses default to HTTP so local development servers work without TLS. Other text becomes a DuckDuckGo search query, with query characters percent-encoded for the URL. Search text, public domains, and `localhost:8765` were manually verified on macOS.

## Platform goals

The application should run on macOS, Linux, and Windows. Shared code should be preferred where practical. Platform-specific behavior may be needed for operating-system APIs and packaging. Linux support must account for both X11 and Wayland environments where practical.

## Browser components

The project uses Wry to host the operating system's WebView engine. A complete modern browser engine is far beyond the first prototype, so Dark Horse uses this established component for page layout and rendering while its own browser controls and privacy behavior remain project responsibilities.

Longer-term areas include:

- User interface and window management
- Navigation and HTTP/HTTPS networking
- HTML, CSS, layout, and rendering
- JavaScript execution
- Request filtering and compatibility
- Temporary cookies, cache, and website storage
- Security boundaries, permissions, and sandboxing
- Bookmarks stored locally

## Privacy model

The intended default is temporary browsing state: no history, persistent cookies, persistent cache, or persistent website storage; no account, sync, telemetry, or tracking. Local bookmarks are an explicit exception. Wry incognito mode is enabled. The restart check on macOS found no cookie, `localStorage`, or Cache API marker, and the cacheable HTTP resource was requested from the server again after restart. Windows/Linux remain unverified. See [the privacy check](privacy-check.md).

## Planned browser behavior

The address bar now navigates directly for recognized URLs and domains, and sends other text to DuckDuckGo as a search; this v0.0.7 behavior is verified on macOS. Bookmarks will be stored locally as an intentional exception to temporary browsing data. New tabs will offer a user choice between a bookmarks page and DuckDuckGo. The bookmark and new-tab features are not implemented yet.

Password-manager support is a later investigation. Wry hosts an operating-system WebView rather than a standard Chrome or Firefox browser, so compatibility with existing password-manager extensions cannot be assumed.

## Performance and compatibility

Performance claims will be measured. Planned measurements include startup time, page load time, rendering time, request count, transferred data, memory, and CPU. Filtering rules will be evaluated for both blocking effectiveness and site breakage.
