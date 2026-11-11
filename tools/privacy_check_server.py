from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
from pathlib import Path
import socket
import threading


PAGE = Path(__file__).with_name("privacy-check.html")
REQUEST_COUNTS = {}
REQUEST_COUNTS_LOCK = threading.Lock()
TEST_IMAGE = bytes.fromhex(
    "89504e470d0a1a0a0000000d49484452000000010000000108060000001f15c489"
    "0000000b49444154789c636000020000050001a5f645400000000049454e44ae426082"
)


class IPv6ThreadingHTTPServer(ThreadingHTTPServer):
    address_family = socket.AF_INET6


class PrivacyCheckHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        path = self.path.split("?", 1)[0]
        if path == "/request-counts":
            with REQUEST_COUNTS_LOCK:
                body = json.dumps(REQUEST_COUNTS).encode("utf-8")
            content_type = "application/json; charset=utf-8"
            cache_control = "no-store"
        elif path == "/" or path == "/privacy-check.html":
            body = PAGE.read_bytes()
            content_type = "text/html; charset=utf-8"
            cache_control = "no-store"
        elif path == "/cache-probe":
            body = b"Cache probe response: if you can read this, the local server is reachable."
            content_type = "text/plain; charset=utf-8"
            cache_control = "public, max-age=86400"
        elif path in {
            "/allowed-resource",
            "/blocked-resource",
            "/third-party-allowed-resource",
            "/third-party-blocked-resource",
        }:
            with REQUEST_COUNTS_LOCK:
                REQUEST_COUNTS[path] = REQUEST_COUNTS.get(path, 0) + 1
            body = f"Server received {path}.".encode("utf-8")
            content_type = "text/plain; charset=utf-8"
            cache_control = "no-store"
        elif path in {"/assets/first-party.js", "/assets/third-party.js"}:
            with REQUEST_COUNTS_LOCK:
                REQUEST_COUNTS[path] = REQUEST_COUNTS.get(path, 0) + 1
            key = "firstPartyScript" if "first-party" in path else "thirdPartyScript"
            body = f"window.filterTestAssets['{key}'] = true;".encode("utf-8")
            content_type = "application/javascript; charset=utf-8"
            cache_control = "no-store"
        elif path in {"/assets/first-party.png", "/assets/third-party.png"}:
            with REQUEST_COUNTS_LOCK:
                REQUEST_COUNTS[path] = REQUEST_COUNTS.get(path, 0) + 1
            body = TEST_IMAGE
            content_type = "image/png"
            cache_control = "no-store"
        else:
            self.send_error(404)
            return

        self.send_response(200)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", cache_control)
        self.send_header("Access-Control-Allow-Origin", "*")
        self.end_headers()
        self.wfile.write(body)


if __name__ == "__main__":
    port = 8765
    ipv4_server = ThreadingHTTPServer(("127.0.0.1", port), PrivacyCheckHandler)
    print(f"Privacy check server (IPv4): http://127.0.0.1:{port}/")

    try:
        ipv6_server = IPv6ThreadingHTTPServer(("::1", port), PrivacyCheckHandler)
        threading.Thread(target=ipv6_server.serve_forever, daemon=True).start()
        print(f"Privacy check server (IPv6): http://[::1]:{port}/")
    except OSError as error:
        print(f"IPv6 loopback is unavailable; localhost may only work over IPv4 ({error}).")

    ipv4_server.serve_forever()
