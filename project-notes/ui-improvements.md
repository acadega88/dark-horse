# UI improvements

This is a parking list for UI work we have discussed. The current UI is usable for now; revisit these items when we return to UI polish. Do not treat the current bookmark edit popup as the agreed final design.

## Bookmark right-click menu and editor

- Restore the original egui context-menu appearance with **Edit** and **Delete** when right-clicking a bookmark.
- Open the menu at the pointer and keep it in front of the web page, including when a Wry webview is active.
- Keep the page visible behind the menu and editor. Do not hide the webview while editing.
- Keep editing a bookmark's name and URL in the original in-app form. Avoid a separate decorated OS window or a form that appears centered on the page.
- Keep **Edit** and **Delete** in the right-click menu; do not add an `×` delete button to each bookmark.
- Preserve bookmark access in the toolbar and the bookmark choices on the new-tab page.

The current implementation uses an OS-native right-click menu on macOS and Windows, an egui context menu on other platforms, and a separate borderless editor viewport. This is a temporary workaround, not the desired UI. Revisit the WebView/native-view stacking approach before replacing it; previous egui popups were covered by the Wry webview, while hiding the webview made the page disappear during editing.

## New-tab chooser

- Keep the choices clear: DuckDuckGo and each saved bookmark.
- Do not offer a blank page in the new-tab chooser.

## Scope

Do not start a broad visual redesign as part of this list. First resolve the bookmark menu/editor behavior; other UI polish can be collected here when specific issues come up.
