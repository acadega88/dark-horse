# Request filtering research

This note records the investigation and implementation status for v0.0.9. macOS now converts a pinned EasyList snapshot to WebKit rules and attaches it to each tab; local first- and third-party fixture checks pass in a WebKit harness. EasyPrivacy and manual compatibility checks in the full Dark Horse app remain pending.

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

## Initial macOS list choice

Use EasyList first, pinned at version `202611110754` and commit `949819841ca1bafa028f2304a11b34103c50cb4d`. Its network rules are converted using Brave's `adblock` 0.13.5 crate and applied through WKWebView's native content-rule API. EasyPrivacy is deferred until the first list has been manually checked with the full app.

The application compiles the converted EasyList rules and appends four local first-/third-party fixture rules. It retains Wry's incognito WebView setup and attaches the compiled list before opening the requested page.

### Prototype check

- The privacy-check page's request test reported `Load failed`, and the server log showed no `GET /blocked-resource`, confirming that the test request was blocked on macOS.
- The initial prototype check exercised only the local test rule; list coverage was measured separately below.
- After writing the privacy markers and restarting Dark Horse, cookie, `localStorage`, and Cache API storage were all `absent`; `/cache-probe` was requested again from the server. This confirms the v0.0.6 private-browsing behavior still passes after the request-filter changes.

The converter only reports rules it can represent in Apple's format, so the measured coverage is for supported network rules only. Cosmetic hiding and scriptlet injection remain outside v0.0.9.

The bundled source retains its upstream header and attribution. EasyList's official licensing page says repository contents are generally dual-licensed under GPL-3.0-or-later or CC BY-SA 3.0-or-later. Review how these terms interact with the project's license before distributing the app or bundled data; the repository currently has no `LICENSE` file.

## Conversion measurement (2026-11-11, macOS)

Measured with a throwaway program outside this repository: `adblock` 0.13.5 (MPL-2.0, `content-blocking` feature), network rules only (`RuleTypes::NetworkOnly`), then compiled with WebKit's `WKContentRuleListStore` through a small Swift check.

| List | Version | Parsed network filters | Converted | Discarded | Content-blocking rules | JSON size |
| --- | --- | --- | --- | --- | --- | --- |
| EasyList | 202611110754 | 49,006 | 48,807 | 199 | 48,921 (48,374 block, 547 ignore-previous) | ~5.0 MB |
| EasyPrivacy | 202611110743 | 56,566 | 56,467 | 99 | 56,496 (55,638 block, 858 ignore-previous) | ~6.0 MB |

- About 99.6% of parsed network filters convert (EasyList ~99.6%, EasyPrivacy ~99.8%). Comments, cosmetic rules, and unparsed lines are not counted; cosmetic rules were excluded on purpose.
- WebKit compiled each list alone (1.9 s and 2.2 s) and a naive concatenation of both (105,417 rules, 4.1 s) without error. This shows that this size works on this macOS version, not that no rule-count limit exists.
- The converter reports only whether a filter converted, not why it was discarded. The crate's failure reasons include `$redirect`, `$csp`, `$removeparam`, `$badfilter`, `$generichide`, full-regex filters, `$from`/`$to` domain options, and non-ASCII rules. A per-reason breakdown was not measured.
- Ordering matters: `into_content_blocking()` places `ignore-previous-rules` exceptions after the blocking rules. Add both lists to one `FilterSet` instead of concatenating two output files, which was only a size test.
- Not yet measured: compatibility on everyday pages, site breakage, and repeated-launch compile cost. The local converter step took about 0.07 s; WebKit compiled EasyList in about 1.9 s. WebKit's store keeps compiled lists on disk under their identifier.
- Pinned inputs: the bundled EasyList snapshot has SHA-256 `34584e9bbdf09a99a6ad97469b7fe42465a739849e3db978e1c4a7e75c41459a`; the previously measured EasyPrivacy snapshot has SHA-256 `5bf871496066c7d589f49885971e4dc100868acb5579d41505917d95047e023f` and is not bundled.

## Local WebKit behavior check (2026-11-11, macOS)

Expanded the local privacy-check fixture with first-party and third-party fetches, scripts, and images. A temporary WKWebView harness compiled the `adblock` 0.13.5 output for the pinned EasyList, appended the four fixture rules, loaded the page from `127.0.0.1`, and ran the test button. All six result groups passed: both allowed fetches and all four script/image assets loaded; `/blocked-resource` and `/third-party-blocked-resource` each had server count 0. Allowed fetches each had count 1, and all four asset requests reached the server. This validates the generated rules in WKWebView, not the full Dark Horse/Wry application or compatibility with real sites.

## Next validation steps

1. [x] Rerun the v0.0.6 privacy restart check after writing markers, closing Dark Horse, and opening it again.
2. [x] Check how many lines from a pinned EasyList/EasyPrivacy snapshot convert to Apple's format (see the measurement above; per-reason breakdown still open).
3. [x] Build a local test page that requests first-party and third-party fetches, scripts, images, and tracking endpoints; EasyList plus local test rules blocked the expected URLs and allowed resources still loaded (macOS WebKit harness).
4. [ ] Manually verify the full Dark Horse app and commonly used websites with EasyList enabled.
5. [ ] Decide whether to add EasyPrivacy and review attribution/licensing before distribution.

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
