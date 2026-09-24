# v0.0.6 — Verify temporary browsing data

Dark Horse enables Wry's incognito mode in `src/browser_app.rs`. This check verifies that browser data does not survive a complete app restart on the platform being tested. It uses a local-only test page and does not contact an external website.

## Run the local check

From the project root, start the test server in a terminal:

```sh
python3 tools/privacy_check_server.py
```

Keep that terminal open. In Dark Horse, open:

```text
http://127.0.0.1:8765/
```

The page reports whether it found a cookie, a `localStorage` value, and a Cache API entry when it opened. Click **Store test data**, then click **Request cacheable resource** once. The server terminal should print a `GET /cache-probe` request.

Close Dark Horse completely, reopen it, and navigate to the same local address. The cookie, `localStorage`, and Cache API entry should each say **absent**. Click **Request cacheable resource** again; a new `GET /cache-probe` should appear in the still-running server terminal. If any marker says **FOUND**, or the second cache-probe request does not reach the server, the privacy check has failed for that platform.

Stop the local server with **Control+C** when finished.

## What this does and does not verify

- The cookie and `localStorage` checks verify that common site data is not retained after a full app exit.
- The Cache API check verifies site-managed cache storage. The cache-probe endpoint separately checks whether an HTTP response cached for one day is requested from the server again after restart.
- Run the check on each supported operating system before claiming the behavior is verified there. A macOS result does not verify Windows or Linux.
- Wry documents that incognito mode on Windows requires WebView2 Runtime 101.0.1210.39 or newer; on older runtimes the option does nothing. Windows therefore remains unverified until tested with a supported runtime.
- This is a practical regression check, not a security audit of every storage mechanism in the operating-system WebView.

## Results

- macOS, 2026-09-24: cookie, `localStorage`, and Cache API markers were absent after a full app restart. After reopening Dark Horse at 23:51, the cacheable resource was requested again at 23:52, as confirmed by a new `GET /cache-probe` in the server log. The check passed on this macOS setup; repeat it on Windows and Linux before claiming support there.
