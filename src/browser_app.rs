use std::sync::mpsc::{self, Receiver, Sender};

use eframe::egui;
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::{PageLoadEvent, Rect as WebViewRect, WebView, WebViewBuilder};

use crate::icons::{NavigationIcon, navigation_button};

pub struct BrowserApp {
    address: String,
    status_message: String,
    webview: Option<WebView>,
    load_event_sender: Sender<(PageLoadEvent, String)>,
    load_event_receiver: Receiver<(PageLoadEvent, String)>,
}

impl Default for BrowserApp {
    fn default() -> Self {
        let (load_event_sender, load_event_receiver) = mpsc::channel();

        Self {
            address: "https://duckduckgo.com".to_owned(),
            status_message: "Opening DuckDuckGo…".to_owned(),
            webview: None,
            load_event_sender,
            load_event_receiver,
        }
    }
}

impl BrowserApp {
    fn normalize_address(address: &str) -> Result<String, String> {
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
            None => Ok(format!("https://{entered_address}")),
        }
    }

    fn can_go_back(&self) -> bool {
        self.webview
            .as_ref()
            .and_then(|webview| webview.can_go_back().ok())
            .unwrap_or(false)
    }

    fn can_go_forward(&self) -> bool {
        self.webview
            .as_ref()
            .and_then(|webview| webview.can_go_forward().ok())
            .unwrap_or(false)
    }

    fn go_back_in_history(&mut self) {
        if !self.can_go_back() {
            return;
        }

        if let Some(webview) = &self.webview {
            match webview.go_back() {
                Ok(()) => self.status_message = "Going back…".to_owned(),
                Err(error) => self.status_message = format!("Could not go back: {error}"),
            }
        }
    }

    fn go_forward_in_history(&mut self) {
        if !self.can_go_forward() {
            return;
        }

        if let Some(webview) = &self.webview {
            match webview.go_forward() {
                Ok(()) => self.status_message = "Going forward…".to_owned(),
                Err(error) => self.status_message = format!("Could not go forward: {error}"),
            }
        }
    }

    fn reload_page(&mut self) {
        let Some(webview) = &self.webview else {
            self.status_message = "The web page view is not available.".to_owned();
            return;
        };

        match webview.reload() {
            Ok(()) => self.status_message = format!("Reloading {}…", self.address),
            Err(error) => self.status_message = format!("Could not reload page: {error}"),
        }
    }

    fn start_navigation(&mut self) {
        let address = match Self::normalize_address(&self.address) {
            Ok(address) => address,
            Err(error) => {
                self.status_message = error;
                return;
            }
        };

        let Some(webview) = &self.webview else {
            self.status_message = "The web page view is not available.".to_owned();
            return;
        };

        match webview.load_url(&address) {
            Ok(()) => self.status_message = format!("Loading {address}…"),
            Err(error) => self.status_message = format!("Could not open address: {error}"),
        }
    }
}

fn to_webview_rect(rect: egui::Rect) -> WebViewRect {
    WebViewRect {
        position: LogicalPosition::new(f64::from(rect.min.x), f64::from(rect.min.y)).into(),
        size: LogicalSize::new(f64::from(rect.width()), f64::from(rect.height())).into(),
    }
}

impl eframe::App for BrowserApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        while let Ok((event, address)) = self.load_event_receiver.try_recv() {
            match event {
                PageLoadEvent::Started => {
                    if address != "about:blank" {
                        self.status_message = format!("Loading {address}…");
                    }
                }
                PageLoadEvent::Finished => {
                    if address.starts_with("http://") || address.starts_with("https://") {
                        self.address = address;
                        self.status_message = "Page loaded.".to_owned();
                    }
                }
            }
        }

        let mut navigate_requested = false;
        let mut page_area = None;
        let context = ui.ctx().clone();

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                let back = navigation_button(ui, NavigationIcon::Back, self.can_go_back());
                if back.clicked() {
                    self.go_back_in_history();
                }

                let forward = navigation_button(ui, NavigationIcon::Forward, self.can_go_forward());
                if forward.clicked() {
                    self.go_forward_in_history();
                }

                let reload = navigation_button(ui, NavigationIcon::Reload, self.webview.is_some());
                if reload.clicked() {
                    self.reload_page();
                }

                ui.add_space(8.0);
                ui.label("Address:");

                let address_field = ui.add(
                    egui::TextEdit::singleline(&mut self.address)
                        .hint_text("https://example.com")
                        .desired_width(f32::INFINITY),
                );

                navigate_requested = address_field.lost_focus()
                    && ui.input(|input| input.key_pressed(egui::Key::Enter));

                if ui.button("Go").clicked() {
                    navigate_requested = true;
                }
            });

            ui.separator();

            let available = ui.available_rect_before_wrap();
            ui.allocate_rect(available, egui::Sense::hover());
            page_area = Some(available);

            ui.separator();
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Status:").small());
                ui.label(egui::RichText::new(&self.status_message).italics());
            });
        });

        if let Some(page_area) = page_area {
            let bounds = to_webview_rect(page_area);

            if self.webview.is_none() {
                let load_event_sender = self.load_event_sender.clone();
                let context = context.clone();

                match WebViewBuilder::new()
                    .with_url("https://duckduckgo.com")
                    .with_incognito(true)
                    .with_on_page_load_handler(move |event, address| {
                        let _ = load_event_sender.send((event, address));
                        context.request_repaint();
                    })
                    .with_bounds(bounds.clone())
                    .build_as_child(frame)
                {
                    Ok(webview) => self.webview = Some(webview),
                    Err(error) => {
                        self.status_message = format!("Could not create web page view: {error}");
                    }
                }
            }

            if let Some(webview) = &self.webview {
                if let Err(error) = webview.set_bounds(bounds) {
                    self.status_message = format!("Could not resize web page view: {error}");
                }
            }
        }

        if navigate_requested {
            self.start_navigation();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
