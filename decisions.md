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

When the address field contains a domain without a URL scheme, prepend `https://`. Keep explicitly entered `http://` and `https://` schemes. This makes common address-bar input convenient while leaving a later opportunity to add search queries and more complete URL handling.

## 2026-09-24 — Default start page: DuckDuckGo

Open DuckDuckGo when Dark Horse starts. This gives the prototype a useful initial page while Google repeatedly presents a traffic-verification challenge in the current WebView setup. The start page can be revisited after broader compatibility testing.

## 2026-09-24 — Development and learning approach

Keep implementation incremental. Explain important concepts and architectural choices before adding them. The project owner prefers to install and run tools through the terminal, with step-by-step instructions.

## 2026-09-24 — Git practice by milestone

Use a separate branch for each milestone (for example, `feature/v0.0.1-window`), then push it, open a pull request into `main`, merge it, and pull the updated `main` branch locally. This gives the project owner regular practice with Git while keeping milestone work reviewable.
