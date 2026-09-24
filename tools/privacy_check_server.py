from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


PAGE = Path(__file__).with_name("privacy-check.html")


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
    address = ("127.0.0.1", 8765)
    print(f"Privacy check server: http://{address[0]}:{address[1]}/")
    ThreadingHTTPServer(address, PrivacyCheckHandler).serve_forever()
