# Decisions

Record decisions that shape the project, along with their reasons and any limits. Revisit them when new evidence changes the trade-offs.

## 2026-09-24 — Project name: Dark Horse

Use **Dark Horse** as the working name. The name has personal meaning because the project owner's son loves horses, and the expression appeals to the owner. This is a working project name, not a completed trademark clearance.

## 2026-09-24 — Main language: Rust

Use Rust as the main language. The project owner wants to learn systems programming, and Rust is a suitable language for a performance-conscious application. Do not change this direction without discussing it first.

## 2026-09-24 — Target desktop platforms

Target macOS, Linux, and Windows from the start. Check platform support when selecting each significant dependency; cross-platform intent does not replace testing on each operating system.

## 2026-09-24 — First window library: winit

Use `winit` for the first window prototype. It provides cross-platform window creation and event handling while keeping the prototype small. It does not draw window contents; choose a rendering approach later when needed.

## 2026-09-24 — UI prototype: eframe and egui

Use `eframe` with `egui` for the v0.0.2 browser UI prototype, beginning with an editable address field. It provides cross-platform widgets and handles the app window, input, and rendering, avoiding a custom text-input and drawing system at this stage. This changes the prototype's UI framework; it does not select the web page rendering engine. The existing direct `winit` window code still needs to be migrated.

## 2026-09-24 — HTTP client: reqwest on a worker thread

Use `reqwest`'s blocking client for the first HTTP/HTTPS requests, and run it on a standard-library worker thread so a slow response does not freeze the UI. This avoids introducing an async runtime for the first navigation step. Use Rustls for HTTPS and do not enable a cookie store. This is a prototype networking choice; it does not select the future browser engine or its full networking architecture.

## 2026-09-24 — Initial page display: Wry WebView

Use Wry to display real web pages with the operating system's WebView, initially embedding it in the existing `eframe` window while keeping the address controls in `egui`. Enable Wry's incognito option for the prototype. This avoids implementing an HTML/CSS/JavaScript engine from scratch. The platform engines differ, and ephemeral data behavior must be verified; Linux/Wayland needs a GTK-based integration beyond the initial child-view approach.

## 2026-09-24 — Browser navigation prototype: back, forward, and reload

Add compact icon controls to the `eframe` UI while keeping the existing child WebView. Back and forward availability comes from the WebView's native history, and reload uses the WebView reload method. The address field stays synchronized with the current page. The WebView owns navigation history; Dark Horse does not maintain a duplicate history list.

## 2026-09-24 — Bare domains default to HTTPS

When the address field contains a public domain without a URL scheme, prepend `https://`. Use `http://` for `localhost` and loopback IP addresses, which commonly serve local development pages without TLS. Keep explicitly entered `http://` and `https://` schemes. Non-URL text is searched with DuckDuckGo (added in v0.0.7).

## 2026-09-24 — Default start page: DuckDuckGo

Open DuckDuckGo when Dark Horse starts. This gives the prototype a useful initial page while Google repeatedly presents a traffic-verification challenge in the current WebView setup. The start page can be revisited after broader compatibility testing.

## 2026-09-24 — Split startup, browser app, and icons into modules

Keep `main.rs` focused on starting the application. Put browser state, navigation, and UI in `browser_app.rs`, and put navigation icon drawing in `icons.rs`. This makes each file easier to understand while keeping the prototype small; avoid creating more modules until they have a clear responsibility.

## 2026-09-24 — Development and learning approach

Keep implementation incremental. Explain important concepts and architectural choices before adding them. The project owner prefers to install and run tools through the terminal, with step-by-step instructions.

## 2026-09-24 — Git practice by milestone

Use a separate branch for each milestone (for example, `feature/v0.0.1-window`), then push it, open a pull request into `main`, merge it, and pull the updated `main` branch locally. This gives the project owner regular practice with Git while keeping milestone work reviewable.

## 2026-09-25 — Search queries from the address bar

If address-bar input is not recognized as a URL or domain, search for it with DuckDuckGo. Keep direct navigation for explicit HTTP/HTTPS URLs and recognizable bare domains. This makes the address bar useful for both navigation and search while keeping DuckDuckGo as the chosen search provider.

## 2026-09-25 — Local bookmarks and new-tab choices

Add local bookmarks as a deliberate persistence exception to the temporary browsing-data policy. When opening a new tab, offer a blank page, DuckDuckGo, or the bookmarks dashboard. Keep the initial browser startup page on DuckDuckGo.

## 2026-09-25 — Remove blank page from the new-tab chooser

Update the new-tab choices to DuckDuckGo and saved bookmarks only. A blank page is not useful as a choice for this browser. This supersedes the earlier new-tab choice decision; keep the initial browser startup page on DuckDuckGo.

## 2026-09-25 — Bookmark names and right-click actions

Store a display name and URL for each local bookmark, and let the user edit or delete bookmarks from a right-click menu. Do not put a delete `×` on every bookmark. The current native-menu/editor arrangement works around WebView layering and is temporary; the preferred UI is listed in [UI improvements](ui-improvements.md).

## 2026-09-25 — Platform testing order

Continue development and verification on macOS for now. Windows and Linux remain target platforms, but testing them is postponed until a suitable personal test environment is available; lack of those machines does not block macOS milestones. Do not claim support on a platform before testing it.

## 2026-09-25 — Password-manager direction

Treat password-manager support as a later feature. Evaluate established open-source password managers and their integration options before implementation. Do not create a custom password vault or cryptography as a first step; verify that any proposed browser integration works with Wry and meets the project's privacy goals.
