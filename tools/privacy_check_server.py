from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import socket
import threading


PAGE = Path(__file__).with_name("privacy-check.html")


class IPv6ThreadingHTTPServer(ThreadingHTTPServer):
    address_family = socket.AF_INET6


class PrivacyCheckHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == "/" or self.path == "/privacy-check.html":
            body = PAGE.read_bytes()
            content_type = "text/html; charset=utf-8"
            cache_control = "no-store"
        elif self.path == "/cache-probe":
            body = b"Cache probe response: if you can read this, the local server is reachable."
            content_type = "text/plain; charset=utf-8"
            cache_control = "public, max-age=86400"
        elif self.path == "/blocked-resource":
            body = b"Request filtering did not block this test resource."
            content_type = "text/plain; charset=utf-8"
            cache_control = "no-store"
        else:
            self.send_error(404)
            return

        self.send_response(200)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", cache_control)
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
