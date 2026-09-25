use wry::WebView;

use super::BrowserApp;

pub(super) struct BrowserTab {
    pub(super) id: u64,
    pub(super) address: String,
    pub(super) status_message: String,
    pub(super) pending_url: Option<String>,
    pub(super) is_new_tab_chooser: bool,
    pub(super) webview: Option<WebView>,
    pub(super) webview_visible: bool,
    #[cfg(target_os = "macos")]
    pub(super) request_filter_attached: bool,
    #[cfg(target_os = "macos")]
    pub(super) request_filter_failed: bool,
}

impl BrowserTab {
    pub(super) fn with_url(id: u64, url: String, status_message: String) -> Self {
        Self {
            id,
            address: url.clone(),
            status_message,
            pending_url: Some(url),
            is_new_tab_chooser: false,
            webview: None,
            webview_visible: false,
            #[cfg(target_os = "macos")]
            request_filter_attached: false,
            #[cfg(target_os = "macos")]
            request_filter_failed: false,
        }
    }

    pub(super) fn new_tab(id: u64) -> Self {
        Self {
            id,
            address: String::new(),
            status_message: "Choose what to open in this tab.".to_owned(),
            pending_url: None,
            is_new_tab_chooser: true,
            webview: None,
            webview_visible: false,
            #[cfg(target_os = "macos")]
            request_filter_attached: false,
            #[cfg(target_os = "macos")]
            request_filter_failed: false,
        }
    }

    pub(super) fn label(&self) -> String {
        let address = self
            .address
            .strip_prefix("https://")
            .or_else(|| self.address.strip_prefix("http://"))
            .unwrap_or(&self.address);
        let host = address.split(['/', '?', '#']).next().unwrap_or_default();

        if host.is_empty() {
            "New tab".to_owned()
        } else {
            host.to_owned()
        }
    }
}

impl BrowserApp {
    pub(super) fn add_tab(&mut self) {
        let tab = BrowserTab::new_tab(self.next_tab_id);
        self.next_tab_id += 1;
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
    }

    pub(super) fn close_tab(&mut self, tab_index: usize) {
        if self.tabs.len() == 1 {
            self.tabs[0] = BrowserTab::new_tab(self.next_tab_id);
            self.next_tab_id += 1;
            self.active_tab = 0;
            return;
        }

        self.tabs.remove(tab_index);
        if self.active_tab > tab_index {
            self.active_tab -= 1;
        } else if self.active_tab == tab_index {
            self.active_tab = tab_index.min(self.tabs.len() - 1);
        }
    }
}
