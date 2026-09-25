use wry::WebView;

use super::BrowserApp;

pub(super) const START_PAGE: &str = "https://duckduckgo.com";

impl BrowserApp {
    pub(super) fn normalize_address(address: &str) -> Result<String, String> {
        let entered_address = address.trim();

        if entered_address.is_empty() {
            return Err("Enter an HTTP or HTTPS address first.".to_owned());
        }

        match entered_address.split_once("://") {
            Some((scheme, _))
                if scheme.eq_ignore_ascii_case("http") || scheme.eq_ignore_ascii_case("https") =>
            {
                Ok(entered_address.to_owned())
            }
            Some(_) => Err("Only HTTP and HTTPS addresses are supported.".to_owned()),
            None if Self::looks_like_url(entered_address) => {
                let scheme = if Self::is_loopback_address(entered_address) {
                    "http"
                } else {
                    "https"
                };
                Ok(format!("{scheme}://{entered_address}"))
            }
            None => Ok(format!(
                "https://duckduckgo.com/?q={}",
                Self::encode_search_query(entered_address)
            )),
        }
    }

    pub(super) fn looks_like_url(input: &str) -> bool {
        if input.chars().any(char::is_whitespace) {
            return false;
        }

        let authority = input.split(['/', '?', '#']).next().unwrap_or_default();
        if authority.is_empty() || authority.contains('@') {
            return false;
        }

        if let Some(bracketed_host) = authority.strip_prefix('[') {
            return bracketed_host
                .split_once(']')
                .is_some_and(|(host, suffix)| {
                    host.parse::<std::net::Ipv6Addr>().is_ok()
                        && (suffix.is_empty()
                            || suffix
                                .strip_prefix(':')
                                .is_some_and(|port| port.parse::<u16>().is_ok()))
                });
        }

        let host = match authority.rsplit_once(':') {
            Some((host, port)) if port.parse::<u16>().is_ok() => host,
            _ => authority,
        };

        if host.eq_ignore_ascii_case("localhost") || host.parse::<std::net::Ipv4Addr>().is_ok() {
            return true;
        }

        host.contains('.')
            && host.split('.').all(|label| {
                !label.is_empty()
                    && !label.starts_with('-')
                    && !label.ends_with('-')
                    && label
                        .chars()
                        .all(|character| character.is_alphanumeric() || character == '-')
            })
    }

    fn is_loopback_address(input: &str) -> bool {
        let authority = input.split(['/', '?', '#']).next().unwrap_or_default();

        if let Some(bracketed_host) = authority.strip_prefix('[') {
            return bracketed_host.split_once(']').is_some_and(|(host, _)| {
                host.parse::<std::net::Ipv6Addr>()
                    .is_ok_and(|address| address.is_loopback())
            });
        }

        let host = match authority.rsplit_once(':') {
            Some((host, port)) if port.parse::<u16>().is_ok() => host,
            _ => authority,
        };

        host.eq_ignore_ascii_case("localhost")
            || host
                .parse::<std::net::Ipv4Addr>()
                .is_ok_and(|address| address.is_loopback())
    }

    fn encode_search_query(query: &str) -> String {
        const HEX: &[u8; 16] = b"0123456789ABCDEF";
        let mut encoded = String::with_capacity(query.len());

        for byte in query.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'*' | b'-' | b'.' | b'_' => {
                    encoded.push(char::from(byte));
                }
                b' ' => encoded.push('+'),
                _ => {
                    encoded.push('%');
                    encoded.push(char::from(HEX[usize::from(byte >> 4)]));
                    encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
                }
            }
        }

        encoded
    }

    pub(super) fn can_go_back(&self) -> bool {
        self.active_webview()
            .and_then(|webview| webview.can_go_back().ok())
            .unwrap_or(false)
    }

    pub(super) fn can_go_forward(&self) -> bool {
        self.active_webview()
            .and_then(|webview| webview.can_go_forward().ok())
            .unwrap_or(false)
    }

    pub(super) fn active_webview(&self) -> Option<&WebView> {
        self.tabs
            .get(self.active_tab)
            .and_then(|tab| tab.webview.as_ref())
    }

    pub(super) fn go_back_in_history(&mut self) {
        let Some(webview) = self.active_webview() else {
            return;
        };

        match webview.go_back() {
            Ok(()) => self.tabs[self.active_tab].status_message = "Going back…".to_owned(),
            Err(error) => {
                self.tabs[self.active_tab].status_message = format!("Could not go back: {error}");
            }
        }
    }

    pub(super) fn go_forward_in_history(&mut self) {
        let Some(webview) = self.active_webview() else {
            return;
        };

        match webview.go_forward() {
            Ok(()) => self.tabs[self.active_tab].status_message = "Going forward…".to_owned(),
            Err(error) => {
                self.tabs[self.active_tab].status_message =
                    format!("Could not go forward: {error}");
            }
        }
    }

    pub(super) fn reload_page(&mut self) {
        let Some(webview) = self.active_webview() else {
            return;
        };

        match webview.reload() {
            Ok(()) => {
                let address = self.tabs[self.active_tab].address.clone();
                self.tabs[self.active_tab].status_message = format!("Reloading {address}…");
            }
            Err(error) => {
                self.tabs[self.active_tab].status_message =
                    format!("Could not reload page: {error}");
            }
        }
    }

    pub(super) fn navigate(&mut self, tab_index: usize, input: &str) {
        let address = match Self::normalize_address(input) {
            Ok(address) => address,
            Err(error) => {
                self.tabs[tab_index].status_message = error;
                return;
            }
        };

        self.navigate_to_url(tab_index, address);
    }

    pub(super) fn navigate_to_url(&mut self, tab_index: usize, address: String) {
        let tab = &mut self.tabs[tab_index];
        tab.address = address.clone();
        tab.is_new_tab_chooser = false;
        tab.status_message = format!("Loading {address}…");

        if let Some(webview) = &tab.webview {
            if let Err(error) = webview.load_url(&address) {
                tab.status_message = format!("Could not open address: {error}");
            }
        } else {
            tab.pending_url = Some(address);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BrowserApp;

    #[test]
    fn normalize_address_defaults_bare_domains_to_https() {
        assert_eq!(
            BrowserApp::normalize_address("example.com"),
            Ok("https://example.com".to_owned())
        );
    }

    #[test]
    fn normalize_address_rejects_non_http_schemes() {
        assert!(BrowserApp::normalize_address("ftp://example.com").is_err());
    }
}
