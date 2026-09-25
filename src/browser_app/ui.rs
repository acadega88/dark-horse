use eframe::egui;
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::{PageLoadEvent, Rect as WebViewRect, WebViewBuilder};

use super::bookmarks::bookmark_button;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use super::bookmarks::show_bookmark_context_menu;
use super::tabs::BrowserTab;
use super::{BrowserApp, NewTabChoice, UiAction};
use crate::icons::{NavigationIcon, navigation_button};

fn to_webview_rect(rect: egui::Rect) -> WebViewRect {
    WebViewRect {
        position: LogicalPosition::new(f64::from(rect.min.x), f64::from(rect.min.y)).into(),
        size: LogicalSize::new(f64::from(rect.width()), f64::from(rect.height())).into(),
    }
}

impl eframe::App for BrowserApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        while let Ok((tab_id, event, address)) = self.load_event_receiver.try_recv() {
            if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) {
                match event {
                    PageLoadEvent::Started => {
                        if address != "about:blank" {
                            tab.status_message = format!("Loading {address}…");
                        }
                    }
                    PageLoadEvent::Finished => {
                        if address.starts_with("http://") || address.starts_with("https://") {
                            tab.address = address;
                            tab.status_message = "Page loaded.".to_owned();
                        }
                    }
                }
            }
        }

        let active_index = self.active_tab;
        let can_go_back = self.can_go_back();
        let can_go_forward = self.can_go_forward();
        let can_reload = self.active_webview().is_some();
        let current_address = self.tabs[active_index].address.clone();
        let bookmark_saved = self
            .user_data
            .bookmarks
            .iter()
            .any(|bookmark| bookmark.url == current_address);
        let can_toggle_bookmark = can_reload
            && (current_address.starts_with("http://") || current_address.starts_with("https://"));
        let show_new_tab_chooser = self.tabs[active_index].is_new_tab_chooser;
        let status_message = self.tabs[active_index].status_message.clone();
        let bookmarks = self.user_data.bookmarks.clone();
        let tab_labels: Vec<String> = self.tabs.iter().map(BrowserTab::label).collect();
        let mut actions = Vec::new();
        let mut bookmark_context_click = None;
        let mut page_area = None;
        let context = ui.ctx().clone();

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for (index, label) in tab_labels.iter().enumerate() {
                    if ui
                        .selectable_label(index == active_index, label.as_str())
                        .clicked()
                    {
                        actions.push(UiAction::SelectTab(index));
                    }
                    if ui.small_button("×").clicked() {
                        actions.push(UiAction::CloseTab(index));
                    }
                }

                if ui.button("+").on_hover_text("New tab").clicked() {
                    actions.push(UiAction::NewTab);
                }
            });

            ui.horizontal(|ui| {
                let back = navigation_button(ui, NavigationIcon::Back, can_go_back);
                if back.clicked() {
                    self.go_back_in_history();
                }

                let forward = navigation_button(ui, NavigationIcon::Forward, can_go_forward);
                if forward.clicked() {
                    self.go_forward_in_history();
                }

                let reload = navigation_button(ui, NavigationIcon::Reload, can_reload);
                if reload.clicked() {
                    self.reload_page();
                }

                ui.add_space(8.0);
                ui.label("Address:");
                let address_width = (ui.available_width() - 170.0).max(120.0);
                let address_field = ui.add(
                    egui::TextEdit::singleline(&mut self.tabs[active_index].address)
                        .hint_text("https://example.com or search")
                        .desired_width(address_width),
                );

                if address_field.lost_focus()
                    && ui.input(|input| input.key_pressed(egui::Key::Enter))
                {
                    actions.push(UiAction::Navigate(
                        active_index,
                        self.tabs[active_index].address.clone(),
                    ));
                }

                if ui.button("Go").clicked() {
                    actions.push(UiAction::Navigate(
                        active_index,
                        self.tabs[active_index].address.clone(),
                    ));
                }

                let bookmark_label = if bookmark_saved {
                    "★ Saved"
                } else {
                    "☆ Bookmark"
                };
                if ui
                    .add_enabled(can_toggle_bookmark, egui::Button::new(bookmark_label))
                    .clicked()
                {
                    actions.push(UiAction::ToggleBookmark(active_index));
                }
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Bookmarks").strong());
                ui.separator();

                if bookmarks.is_empty() {
                    ui.label(
                        egui::RichText::new("Save a page with ☆ Bookmark to see it here.").weak(),
                    );
                } else {
                    egui::ScrollArea::horizontal()
                        .id_salt("bookmarks-toolbar")
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                for bookmark in &bookmarks {
                                    bookmark_button(
                                        ui,
                                        bookmark,
                                        UiAction::OpenBookmark(active_index, bookmark.url.clone()),
                                        &mut actions,
                                        &mut bookmark_context_click,
                                        None,
                                    );
                                }
                            });
                        });
                }
            });

            ui.separator();

            if show_new_tab_chooser {
                ui.add_space(36.0);
                ui.vertical_centered(|ui| {
                    ui.heading("Where would you like to go?");
                    ui.add_space(18.0);
                    ui.horizontal_wrapped(|ui| {
                        if ui
                            .add_sized([190.0, 54.0], egui::Button::new("DuckDuckGo"))
                            .on_hover_text("Open DuckDuckGo search")
                            .clicked()
                        {
                            actions.push(UiAction::ChooseNewTabPage(
                                active_index,
                                NewTabChoice::DuckDuckGo,
                            ));
                        }

                        for bookmark in &bookmarks {
                            bookmark_button(
                                ui,
                                bookmark,
                                UiAction::ChooseNewTabPage(
                                    active_index,
                                    NewTabChoice::Bookmark(bookmark.url.clone()),
                                ),
                                &mut actions,
                                &mut bookmark_context_click,
                                Some([190.0, 54.0]),
                            );
                        }

                        if bookmarks.is_empty() {
                            ui.label("Save a page with ☆ Bookmark and it will appear here.");
                        }
                    });
                });
            } else {
                let available = ui.available_rect_before_wrap();
                ui.allocate_rect(available, egui::Sense::hover());
                page_area = Some(available);
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Status:").small());
                ui.label(egui::RichText::new(&status_message).italics());
            });
        });

        let needs_repaint = !actions.is_empty();
        for action in actions {
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            if let UiAction::BeginEditBookmark(url) = &action {
                if let Some((context_url, click_position)) = &bookmark_context_click {
                    if context_url == url {
                        self.editing_bookmark_position = Some(
                            context
                                .input(|input| input.viewport().inner_rect)
                                .map_or(*click_position, |rect| {
                                    rect.min + click_position.to_vec2()
                                }),
                        );
                    }
                }
            }
            self.apply_action(action);
        }

        #[cfg(any(target_os = "macos", target_os = "windows"))]
        if let Some((url, click_position)) = bookmark_context_click {
            if let Some(edit) = show_bookmark_context_menu(frame) {
                if edit {
                    self.editing_bookmark_position = Some(
                        context
                            .input(|input| input.viewport().inner_rect)
                            .map_or(click_position, |rect| rect.min + click_position.to_vec2()),
                    );
                    self.begin_edit_bookmark(&url);
                } else {
                    self.delete_bookmark(&url);
                }
                context.request_repaint();
            }
        }

        let edit_viewport_id = egui::ViewportId::from_hash_of("bookmark-edit-dialog");
        let mut save_edit = None;
        let mut cancel_edit = false;
        if self.editing_bookmark.is_some() {
            let window_position = self
                .editing_bookmark_position
                .take()
                .unwrap_or(egui::pos2(100.0, 100.0));
            let viewport_builder = egui::ViewportBuilder::default()
                .with_title("Edit bookmark")
                .with_inner_size([380.0, 210.0])
                .with_position(window_position)
                .with_resizable(false)
                .with_decorations(false)
                .with_active(true)
                .with_always_on_top();

            let close_requested = context.show_viewport_immediate(
                edit_viewport_id,
                viewport_builder,
                |ui, _class| {
                    let close_requested = ui.ctx().input(|input| {
                        input.viewport().close_requested() || input.key_pressed(egui::Key::Escape)
                    });
                    egui::CentralPanel::default().show(ui, |ui| {
                        egui::Frame::window(ui.style()).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.strong("Edit bookmark");
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui.small_button("×").clicked() {
                                            cancel_edit = true;
                                        }
                                    },
                                );
                            });
                            ui.separator();
                            if let Some(edit) = self.editing_bookmark.as_mut() {
                                ui.label("Name");
                                ui.text_edit_singleline(&mut edit.title);
                                ui.add_space(8.0);
                                ui.label("URL");
                                ui.text_edit_singleline(&mut edit.url);
                                ui.add_space(12.0);
                                ui.horizontal(|ui| {
                                    if ui.button("Save").clicked() {
                                        save_edit = Some(edit.clone());
                                    }
                                    if ui.button("Cancel").clicked() {
                                        cancel_edit = true;
                                    }
                                });
                            }
                        });
                    });
                    close_requested
                },
            );

            if close_requested || cancel_edit {
                self.editing_bookmark = None;
                context.send_viewport_cmd_to(edit_viewport_id, egui::ViewportCommand::Close);
            } else if let Some(edit) = save_edit {
                self.save_bookmark_edit(edit);
                if self.editing_bookmark.is_none() {
                    context.send_viewport_cmd_to(edit_viewport_id, egui::ViewportCommand::Close);
                }
                context.request_repaint();
            }
        }

        if let Some(page_area) = page_area {
            let bounds = to_webview_rect(page_area);
            let active_tab = self.active_tab;
            let tab = &mut self.tabs[active_tab];

            if tab.webview.is_none() {
                if let Some(address) = tab.pending_url.take() {
                    let tab_id = tab.id;
                    let load_event_sender = self.load_event_sender.clone();
                    let context = context.clone();

                    match WebViewBuilder::new()
                        .with_url(&address)
                        .with_incognito(true)
                        .with_visible(true)
                        .with_on_page_load_handler(move |event, address| {
                            let _ = load_event_sender.send((tab_id, event, address));
                            context.request_repaint();
                        })
                        .with_bounds(bounds.clone())
                        .build_as_child(frame)
                    {
                        Ok(webview) => {
                            tab.webview = Some(webview);
                            tab.webview_visible = true;
                        }
                        Err(error) => {
                            tab.status_message = format!("Could not create web page view: {error}");
                        }
                    }
                }
            }
        }

        let active_tab = self.active_tab;
        for (index, tab) in self.tabs.iter_mut().enumerate() {
            if let Some(webview) = &tab.webview {
                let is_active = index == active_tab;
                if tab.webview_visible != is_active {
                    match webview.set_visible(is_active) {
                        Ok(()) => tab.webview_visible = is_active,
                        Err(error) => {
                            tab.status_message =
                                format!("Could not change tab visibility: {error}");
                        }
                    }
                }

                if is_active {
                    if let Some(page_area) = page_area {
                        if let Err(error) = webview.set_bounds(to_webview_rect(page_area)) {
                            tab.status_message = format!("Could not resize web page view: {error}");
                        }
                    }
                }
            }
        }

        if needs_repaint {
            context.request_repaint();
        }
    }
}
