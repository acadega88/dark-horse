use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use eframe::egui;
use reqwest::blocking::Client;

struct NavigationResult {
    address: String,
    status: String,
}

struct BrowserApp {
    address: String,
    status_message: String,
    loading: bool,
    client: Option<Client>,
    client_error: Option<String>,
    result_sender: Sender<Result<NavigationResult, String>>,
    result_receiver: Receiver<Result<NavigationResult, String>>,
}

impl Default for BrowserApp {
    fn default() -> Self {
        let (result_sender, result_receiver) = mpsc::channel();
        let client_result = Client::builder()
            .timeout(Duration::from_secs(20))
            .build();

        let (client, client_error, status_message) = match client_result {
            Ok(client) => (Some(client), None, "Enter an HTTP or HTTPS address.".to_owned()),
            Err(error) => {
                let message = format!("Could not create HTTP client: {error}");
                (None, Some(message.clone()), message)
            }
        };

        Self {
            address: String::new(),
            status_message,
            loading: false,
            client,
            client_error,
            result_sender,
            result_receiver,
        }
    }
}

impl BrowserApp {
    fn start_navigation(&mut self, context: egui::Context) {
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

        let Some(client) = self.client.clone() else {
            self.status_message = self
                .client_error
                .clone()
                .unwrap_or_else(|| "The HTTP client is unavailable.".to_owned());
            return;
        };

        self.loading = true;
        self.status_message = format!("Loading {address}…");

        let result_sender = self.result_sender.clone();
        std::thread::spawn(move || {
            let result = client
                .get(&address)
                .send()
                .map(|response| NavigationResult {
                    address: response.url().to_string(),
                    status: response.status().to_string(),
                })
                .map_err(|error| error.to_string());

            let _ = result_sender.send(result);
            context.request_repaint();
        });
    }
}

impl eframe::App for BrowserApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Ok(result) = self.result_receiver.try_recv() {
            self.loading = false;
            self.status_message = match result {
                Ok(result) => format!("{} — HTTP {}", result.address, result.status),
                Err(error) => format!("Request failed: {error}"),
            };
        }

        let context = ui.ctx().clone();
        let mut navigate_requested = false;
        let loading = self.loading;

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Address:");

                let address_field = ui.add_enabled(
                    !loading,
                    egui::TextEdit::singleline(&mut self.address)
                        .hint_text("https://example.com")
                        .desired_width(f32::INFINITY),
                );

                navigate_requested = address_field.lost_focus()
                    && ui.input(|input| input.key_pressed(egui::Key::Enter));

                if ui
                    .add_enabled(!loading, egui::Button::new("Go"))
                    .clicked()
                {
                    navigate_requested = true;
                }
            });

            ui.separator();
            ui.label(&self.status_message);
            ui.separator();
            ui.label("Web page area (HTML rendering comes in a later milestone.)");
        });

        if navigate_requested {
            self.start_navigation(context);
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
