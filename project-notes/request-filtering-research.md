# Request filtering research

This note records the initial investigation and implementation status for v0.0.9. The macOS prototype compiles and attaches local test rules; the manual endpoint check passed on macOS.

## Project context

- The current dependency is Wry 0.57.0. The roadmap scopes v0.0.9 to macOS; Windows and Linux remain later platform targets.
- Wry's shared `with_navigation_handler` callback decides whether a URL may navigate. The documented shared builder API does not provide a general callback for every network resource requested by a page.
- As a result, request blocking through Wry appears to need platform-specific adapters. This is an inference from the public APIs reviewed, not proof that no other integration is possible.

## Platform mechanisms found

### macOS / WKWebView

- Wry 0.57.0 allows a custom `WKWebViewConfiguration` through `WebViewBuilderExtMacos::with_webview_configuration`.
- Apple's `WKContentRuleListStore` compiles declarative rules and `WKUserContentController` applies them to a web view. This is a native content-blocking path, rather than a per-request Rust callback.
- Brave's `adblock` Rust crate can convert supported ABP-style rules into Apple's content-blocking format with its `content-blocking` feature.
- **Important privacy detail:** Wry 0.57.0's local source shows that when an existing WKWebView configuration is supplied, Wry reads that configuration's website data store and skips the branch that selects a non-persistent store from `with_incognito(true)`. A custom configuration must therefore set `WKWebsiteDataStore::nonPersistentDataStore` itself. Keep the current privacy behavior and rerun the v0.0.6 restart check.

### Windows / WebView2

- WebView2 exposes `WebResourceRequested` for filtering and handling selected requests, including by URL and resource type.
- Wry's Windows extension exposes the underlying `ICoreWebView2` through `WebViewExtWindows::webview()`. A Windows implementation therefore looks possible, but would use Windows-specific APIs and needs a Windows machine for verification.

### Linux / WebKitGTK

- WebKitGTK has `WebKitUserContentFilterStore` and `WebKitUserContentManager::add_filter` for declarative content filters.
- Wry 0.57.0's Linux `WebViewExtUnix::webview()` exposes the underlying WebKitGTK view. The current project has not been built or tested on Linux, so the timing for attaching a filter to the existing view and whether it catches the first page load must be checked there.

## Candidate for the macOS milestone

Investigate `adblock` (Brave's `adblock-rust`) as the rule parser/converter, then apply its Apple-format output through WKWebView's native content-rule API. This could let the project use established ABP-style lists without writing a filter parser or hooking every request itself.

The current code is only a plumbing prototype: it blocks the local `/blocked-resource` endpoint on the privacy-check server. It does not yet block general ads or trackers. The prototype keeps Wry's existing incognito webview setup and adds the compiled rule to each webview's content controller before opening its requested page.

### Prototype check

- The privacy-check page's request test reported `Load failed`, and the server log showed no `GET /blocked-resource`, confirming that the test request was blocked on macOS.
- This verifies the local test rule only; it does not yet measure ad/tracker list coverage.
- After writing the privacy markers and restarting Dark Horse, cookie, `localStorage`, and Cache API storage were all `absent`; `/cache-probe` was requested again from the server. This confirms the v0.0.6 private-browsing behavior still passes after the request-filter changes.

The converter only reports rules it can represent in Apple's format, so measure supported-rule coverage before selecting a list. Start with network request blocking; leave cosmetic hiding and scriptlet injection outside v0.0.9 unless a tested need changes the scope.

Before bundling a list or publishing it with Dark Horse, review its current redistribution terms and attribution requirements. EasyList's official licensing page says its repository contents are generally dual-licensed under GPL-3.0-or-later or CC BY-SA 3.0-or-later, with possible exceptions for externally hosted lists. This repository currently has no `LICENSE` file, so the project's own license should also be decided before distributing bundled filter data.

## Next validation steps

1. [x] Rerun the v0.0.6 privacy restart check after writing markers, closing Dark Horse, and opening it again.
2. Check how many lines from a pinned EasyList/EasyPrivacy snapshot convert to Apple's format, and identify discarded rule types.
3. Build a local test page that requests first-party and third-party scripts, images, and tracking endpoints; verify blocked requests never reach the local server and allowed requests still load.
4. Record the selected list source, version/update policy, attribution, and filter behavior before distributing bundled filter data.

## Primary references

- [Wry 0.57.0 WebViewBuilder](https://docs.rs/wry/0.57.0/wry/struct.WebViewBuilder.html)
- [Wry 0.57.0 WebViewBuilderExtMacos](https://docs.rs/wry/0.57.0/wry/trait.WebViewBuilderExtMacos.html)
- [Wry 0.57.0 WebViewExtWindows](https://docs.rs/wry/0.57.0/wry/trait.WebViewExtWindows.html)
- [Wry 0.57.0 WebViewExtUnix](https://docs.rs/wry/0.57.0/wry/trait.WebViewExtUnix.html)
- [Wry 0.57.0 WKWebView setup source](https://docs.rs/crate/wry/0.57.0/source/src/wkwebview/mod.rs)
- [Apple WKContentRuleListStore](https://developer.apple.com/documentation/webkit/wkcontentruleliststore)
- [Apple WKUserContentController.addContentRuleList](https://developer.apple.com/documentation/webkit/wkusercontentcontroller/add%28_%3A%29)
- [Microsoft WebView2 custom management of network requests](https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/webresourcerequested)
- [WebKitGTK UserContentFilterStore](https://webkitgtk.org/reference/webkit2gtk/stable/class.UserContentFilterStore.html)
- [WebKitGTK UserContentManager](https://webkitgtk.org/reference/webkit2gtk/stable/class.UserContentManager.html)
- [Brave adblock-rust](https://github.com/brave/adblock-rust)
- [EasyList licensing](https://easylist.to/pages/licence.html)
