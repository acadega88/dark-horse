use std::sync::mpsc::{self, Receiver, Sender};

use eframe::egui;
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::{PageLoadEvent, Rect as WebViewRect, WebView, WebViewBuilder};

struct BrowserApp {
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
            address: String::new(),
            status_message: "Enter an HTTP or HTTPS address.".to_owned(),
            webview: None,
            load_event_sender,
            load_event_receiver,
        }
    }
}

impl BrowserApp {
    fn start_navigation(&mut self) {
        let entered_address = self.address.trim();

        if entered_address.is_empty() {
            self.status_message = "Enter an HTTP or HTTPS address first.".to_owned();
            return;
        }

        let address = match entered_address.split_once("://") {
            Some((scheme, _))
                if scheme.eq_ignore_ascii_case("http")
                    || scheme.eq_ignore_ascii_case("https") =>
            {
                entered_address.to_owned()
            }
            Some(_) => {
                self.status_message = "Only HTTP and HTTPS addresses are supported.".to_owned();
                return;
            }
            None => format!("https://{entered_address}"),
        };

        self.address = address.clone();

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
            ui.label(&self.status_message);
            ui.separator();

            let available = ui.available_rect_before_wrap();
            ui.allocate_rect(available, egui::Sense::hover());
            page_area = Some(available);
        });

        if let Some(page_area) = page_area {
            let bounds = to_webview_rect(page_area);

            if self.webview.is_none() {
                let load_event_sender = self.load_event_sender.clone();
                let context = context.clone();

                match WebViewBuilder::new()
                    .with_url("about:blank")
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

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Dark Horse Browser",
        options,
        Box::new(|_creation_context| Ok(Box::<BrowserApp>::default())),
    )
}
