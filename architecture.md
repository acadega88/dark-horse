# Architecture

This document records the architecture as it is understood today. It is intentionally small; future browser decisions should be made when the project reaches them.

## Current shape

```text
src/main.rs       Application entry point
Cargo.toml        Package metadata and Rust dependencies
Cargo.lock        Exact dependency versions selected by Cargo
```

The initial application is a normal Rust executable. The first GUI layer is `winit`, which creates the native window and delivers events such as resizing and keyboard input. It does not draw the contents of the window, so a drawing approach will be chosen when the prototype needs to display content.

## Platform goals

The application should run on macOS, Linux, and Windows. Shared code should be preferred where practical. Platform-specific behavior may be needed for operating-system APIs and packaging. Linux support must account for both X11 and Wayland environments where practical.

## Browser components

No browser engine or rendering architecture has been selected. A complete modern browser engine is far beyond the first prototype, so the project will evaluate which parts are educational and practical to implement and which parts should use established components.

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

The intended default is temporary browsing state: no history, persistent cookies, persistent cache, or persistent website storage; no account, sync, telemetry, or tracking. Local bookmarks are an explicit exception. These are product goals, not yet implemented behavior.

## Performance and compatibility

Performance claims will be measured. Planned measurements include startup time, page load time, rendering time, request count, transferred data, memory, and CPU. Filtering rules will be evaluated for both blocking effectiveness and site breakage.
