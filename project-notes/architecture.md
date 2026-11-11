# Architecture

This document records the architecture as it is understood today. It is intentionally small; future browser decisions should be made when the project reaches them.

## Current shape

```text
src/main.rs                          Application entry point and startup window options
src/browser_app.rs                   BrowserApp state and action routing
src/browser_app/ui.rs                egui layout and WebView lifecycle
src/browser_app/navigation.rs        Address recognition, search, and WebView navigation
src/browser_app/tabs.rs              Tab state and operations
src/browser_app/bookmarks.rs         Bookmark controls, context menu, and editing
src/browser_app/request_filter.rs    macOS-only WebKit content rule list (v0.0.9 prototype)
assets/easylist-202611110754.txt     Pinned EasyList snapshot
src/icons.rs                         Drawing for the back, forward, and reload controls
src/user_data.rs                     Local bookmark storage
tools/                               Local privacy-check server and test page
Cargo.toml                           Package metadata and Rust dependencies
Cargo.lock                           Exact dependency versions selected by Cargo
```

The application is a normal Rust executable. The v0.0.1 prototype used `winit` directly to create a native window; it opened and closed on macOS. Since v0.0.2 the window is created by `eframe`, which uses `winit` internally; `winit` is no longer a direct dependency, though `bookmarks.rs` still reads the window handle through `eframe::Frame`. Linux and Windows have not been verified yet. `winit` does not provide widgets or draw window contents.

For v0.0.2, the app migrated to `eframe`/`egui` for browser UI controls. `eframe` supplies the app framework and renderer; `egui` supplies widgets. The prototype shows an editable address field on macOS.

For v0.0.3, the app used `reqwest`'s blocking HTTP client on a standard-library worker thread to fetch addresses. This was superseded in v0.0.4: Wry now loads pages through the platform WebView, and `reqwest` is no longer a dependency. Dark Horse itself makes no HTTP requests; the WebView does.

For v0.0.4, page display uses Wry, a cross-platform wrapper around each operating system's WebView. The existing `eframe`/`egui` address controls remain in the app window, with the WebView embedded below them as a child view. Google and YouTube have been opened successfully on macOS. The WebView is configured for incognito mode, but temporary-only storage behavior has not yet been verified. Wry documents that Windows requires WebView2 Runtime 101.0.1210.39 or newer for incognito mode; older runtimes ignore the setting. Wry's child-view approach works on macOS, Windows, and Linux/X11; Linux/Wayland needs a GTK-based integration and remains to be addressed. Page rendering and engine behavior come from the OS and may differ between platforms.

For v0.0.5, the app keeps the same `eframe` window and address bar while adding back, forward, and reload controls. The controls query and use the WebView's own navigation history so link clicks and page navigations stay in sync. The address field follows the current page, and a bare domain becomes `https://...`. DuckDuckGo is the start page, and the initial window size is 1800×1100 logical pixels. This remains a small prototype and relies on the platform WebView for page rendering.

For v0.0.7, the address bar keeps explicit HTTP/HTTPS URLs and recognized domains as direct navigation. Public domains default to HTTPS; `localhost` and loopback IP addresses default to HTTP so local development servers work without TLS. Other text becomes a DuckDuckGo search query, with query characters percent-encoded for the URL. Search text, public domains, and `localhost:8765` were manually verified on macOS.

For v0.0.8, each tab owns a separate incognito Wry WebView. Inactive WebViews are hidden and the active one is sized to the page area. Opening a new tab presents DuckDuckGo and the saved bookmarks as choices; there is no blank-page choice. Saved bookmarks are also shown in a horizontally scrolling toolbar below the address bar on every tab; selecting one navigates the active tab. Each bookmark has a name and URL and can be edited or deleted from its right-click menu. Bookmarks are written to a small JSON file in the operating system's application-data directory as an intentional persistence exception; website cookies, history, and site data are not stored there. Closing the app does not restore tabs. This implementation has been manually verified on macOS.

The WebView is a native child view layered above the main egui canvas. To keep bookmark context actions visible on macOS and Windows, the current implementation uses Muda's native context menu there; Linux uses egui's context menu. The bookmark editor currently uses a borderless, always-on-top egui viewport so the page remains visible. These are implementation workarounds; the final bookmark menu/editor design is recorded in [UI improvements](ui-improvements.md).

For v0.0.9 (in progress, macOS only), `request_filter.rs` uses `adblock` 0.13.5 to convert the pinned EasyList and EasyPrivacy network rules into one WebKit rule list, then adds it to each tab's `WKUserContentController`. Four local test rules are appended for the privacy-check page. To make sure the filter is in place before the first page loads, each macOS WebView is created on `about:blank`; once the rule list is attached, the pending URL is loaded. Compilation and attachment complete asynchronously and report back to the UI through `mpsc` channels. Other platforms are not included yet. See [request filtering research](request-filtering-research.md).

Browser application code is split by responsibility: `browser_app/ui.rs` owns the egui and WebView presentation lifecycle, `navigation.rs` handles address recognition and WebView navigation, `tabs.rs` owns tab state and tab operations, `bookmarks.rs` owns bookmark controls and persistence actions, and `request_filter.rs` owns the macOS content-filter integration. `browser_app.rs` holds shared application state and routes UI actions to those modules.

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

The address bar now navigates directly for recognized URLs and domains, and sends other text to DuckDuckGo as a search; this v0.0.7 behavior is verified on macOS. The v0.0.8 implementation adds tabs, locally persisted bookmarks with add/edit/delete actions, a new-tab chooser, and a bookmark toolbar. This is manually verified on macOS.

Password-manager support is a later investigation. Wry hosts an operating-system WebView rather than a standard Chrome or Firefox browser, so compatibility with existing password-manager extensions cannot be assumed.

## Performance and compatibility

Performance claims will be measured. Planned measurements include startup time, page load time, rendering time, request count, transferred data, memory, and CPU. Filtering rules will be evaluated for both blocking effectiveness and site breakage.
