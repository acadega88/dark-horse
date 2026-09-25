use std::sync::mpsc::{self, Receiver, Sender};

use eframe::egui;
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::{PageLoadEvent, Rect as WebViewRect, WebView, WebViewBuilder};

use crate::icons::{NavigationIcon, navigation_button};
use crate::user_data::{Bookmark, UserData};

const START_PAGE: &str = "https://duckduckgo.com";

struct BrowserTab {
    id: u64,
    address: String,
    status_message: String,
    pending_url: Option<String>,
    is_new_tab_chooser: bool,
    webview: Option<WebView>,
    webview_visible: bool,
}

impl BrowserTab {
    fn with_url(id: u64, url: String, status_message: String) -> Self {
        Self {
            id,
            address: url.clone(),
            status_message,
            pending_url: Some(url),
            is_new_tab_chooser: false,
            webview: None,
            webview_visible: false,
        }
    }

    fn new_tab(id: u64) -> Self {
        Self {
            id,
            address: String::new(),
            status_message: "Choose what to open in this tab.".to_owned(),
            pending_url: None,
            is_new_tab_chooser: true,
            webview: None,
            webview_visible: false,
        }
    }

    fn label(&self) -> String {
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

enum UiAction {
    SelectTab(usize),
    NewTab,
    CloseTab(usize),
    Navigate(usize, String),
    OpenBookmark(usize, String),
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    BeginEditBookmark(String),
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    DeleteBookmark(String),
    ToggleBookmark(usize),
    ChooseNewTabPage(usize, NewTabChoice),
}

#[derive(Clone)]
struct BookmarkEditDraft {
    original_url: String,
    title: String,
    url: String,
}

enum NewTabChoice {
    DuckDuckGo,
    Bookmark(String),
}

fn bookmark_button(
    ui: &mut egui::Ui,
    bookmark: &Bookmark,
    open_action: UiAction,
    actions: &mut Vec<UiAction>,
    bookmark_context_click: &mut Option<(String, egui::Pos2)>,
    size: Option<[f32; 2]>,
) {
    let button = egui::Button::new(bookmark.title.as_str());
    let response = match size {
        Some(size) => ui.add_sized(size, button),
        None => ui.add(button),
    }
    .on_hover_text(bookmark.url.as_str());

    if response.clicked() {
        actions.push(open_action);
    }

    if response.secondary_clicked() {
        if let Some(position) = response.interact_pointer_pos() {
            *bookmark_context_click = Some((bookmark.url.clone(), position));
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    response.context_menu(|ui| {
        if ui.button("Edit").clicked() {
            actions.push(UiAction::BeginEditBookmark(bookmark.url.clone()));
            ui.close();
        }
        if ui.button("Delete").clicked() {
            actions.push(UiAction::DeleteBookmark(bookmark.url.clone()));
            ui.close();
        }
    });
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn show_bookmark_context_menu(frame: &eframe::Frame) -> Option<bool> {
    use muda::{ContextMenu, MenuEvent, MenuItem, Submenu};
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    const EDIT_ID: &str = "dark-horse-bookmark-edit";
    const DELETE_ID: &str = "dark-horse-bookmark-delete";

    let edit_item = MenuItem::with_id(EDIT_ID, "Edit", true, None);
    let delete_item = MenuItem::with_id(DELETE_ID, "Delete", true, None);
    let menu = Submenu::new("Bookmark", true);
    menu.append_items(&[&edit_item, &delete_item]).ok()?;

    let window = frame.winit_window()?;
    let handle = window.window_handle().ok()?.as_raw();

    // Drain stale events so a previous menu selection cannot affect this click.
    let receiver = MenuEvent::receiver();
    while receiver.try_recv().is_ok() {}

    match handle {
        #[cfg(target_os = "macos")]
        RawWindowHandle::AppKit(handle) => unsafe {
            menu.show_context_menu_for_nsview(handle.ns_view.as_ptr(), None);
        },
        #[cfg(target_os = "windows")]
        RawWindowHandle::Win32(handle) => unsafe {
            menu.show_context_menu_for_hwnd(handle.hwnd.get(), None);
        },
        _ => return None,
    }

    while let Ok(event) = receiver.try_recv() {
        if &event.id == edit_item.id() {
            return Some(true);
        }
        if &event.id == delete_item.id() {
            return Some(false);
        }
    }

    None
}

pub struct BrowserApp {
    tabs: Vec<BrowserTab>,
    active_tab: usize,
    next_tab_id: u64,
    user_data: UserData,
    editing_bookmark: Option<BookmarkEditDraft>,
    editing_bookmark_position: Option<egui::Pos2>,
    load_event_sender: Sender<(u64, PageLoadEvent, String)>,
    load_event_receiver: Receiver<(u64, PageLoadEvent, String)>,
}

impl Default for BrowserApp {
    fn default() -> Self {
        let (load_event_sender, load_event_receiver) = mpsc::channel();
        let (user_data, load_error) = UserData::load();

        let mut first_tab =
            BrowserTab::with_url(1, START_PAGE.to_owned(), "Opening DuckDuckGo…".to_owned());
        if let Some(error) = load_error {
            first_tab.status_message = error;
        }

        Self {
            tabs: vec![first_tab],
            active_tab: 0,
            next_tab_id: 2,
            user_data,
            editing_bookmark: None,
            editing_bookmark_position: None,
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

    fn looks_like_url(input: &str) -> bool {
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

    fn can_go_back(&self) -> bool {
        self.active_webview()
            .and_then(|webview| webview.can_go_back().ok())
            .unwrap_or(false)
    }

    fn can_go_forward(&self) -> bool {
        self.active_webview()
            .and_then(|webview| webview.can_go_forward().ok())
            .unwrap_or(false)
    }

    fn active_webview(&self) -> Option<&WebView> {
        self.tabs
            .get(self.active_tab)
            .and_then(|tab| tab.webview.as_ref())
    }

    fn go_back_in_history(&mut self) {
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

    fn go_forward_in_history(&mut self) {
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

    fn reload_page(&mut self) {
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

    fn navigate(&mut self, tab_index: usize, input: &str) {
        let address = match Self::normalize_address(input) {
            Ok(address) => address,
            Err(error) => {
                self.tabs[tab_index].status_message = error;
                return;
            }
        };

        self.navigate_to_url(tab_index, address);
    }

    fn navigate_to_url(&mut self, tab_index: usize, address: String) {
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

    fn add_tab(&mut self) {
        let tab = BrowserTab::new_tab(self.next_tab_id);
        self.next_tab_id += 1;
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
    }

    fn close_tab(&mut self, tab_index: usize) {
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

    fn toggle_bookmark(&mut self, tab_index: usize) {
        let address = self.tabs[tab_index].address.clone();
        if !(address.starts_with("http://") || address.starts_with("https://")) {
            return;
        }

        if let Some(bookmark_index) = self
            .user_data
            .bookmarks
            .iter()
            .position(|bookmark| bookmark.url == address)
        {
            self.user_data.bookmarks.remove(bookmark_index);
            self.tabs[tab_index].status_message = "Bookmark removed.".to_owned();
        } else {
            let title = Self::bookmark_title(&address);
            self.user_data.bookmarks.push(Bookmark {
                title,
                url: address,
            });
            self.tabs[tab_index].status_message = "Bookmark saved on this device.".to_owned();
        }

        self.save_user_data(tab_index);
    }

    fn bookmark_title(address: &str) -> String {
        let without_scheme = address
            .strip_prefix("https://")
            .or_else(|| address.strip_prefix("http://"))
            .unwrap_or(address);
        without_scheme
            .split(['/', '?', '#'])
            .next()
            .filter(|host| !host.is_empty())
            .unwrap_or(address)
            .to_owned()
    }

    fn begin_edit_bookmark(&mut self, url: &str) {
        if let Some(bookmark) = self
            .user_data
            .bookmarks
            .iter()
            .find(|bookmark| bookmark.url == url)
        {
            self.editing_bookmark = Some(BookmarkEditDraft {
                original_url: bookmark.url.clone(),
                title: bookmark.title.clone(),
                url: bookmark.url.clone(),
            });
        }
    }

    fn delete_bookmark(&mut self, url: &str) {
        self.user_data
            .bookmarks
            .retain(|bookmark| bookmark.url != url);
        let active_tab = self.active_tab;
        self.save_user_data(active_tab);
    }

    fn save_bookmark_edit(&mut self, edit: BookmarkEditDraft) {
        let title = edit.title.trim().to_owned();
        if title.is_empty() {
            self.tabs[self.active_tab].status_message = "Bookmark name cannot be empty.".to_owned();
            self.editing_bookmark = Some(edit);
            return;
        }

        let entered_url = edit.url.trim();
        let normalized_url =
            if entered_url.split_once("://").is_some() || Self::looks_like_url(entered_url) {
                Self::normalize_address(entered_url)
            } else {
                Err("Enter a valid HTTP or HTTPS address.".to_owned())
            };

        let url = match normalized_url {
            Ok(url) if url.starts_with("http://") || url.starts_with("https://") => Ok(url),
            Ok(_) => Err("Only HTTP and HTTPS bookmark addresses are supported.".to_owned()),
            Err(error) => Err(error),
        };
        let url = match url {
            Ok(url) => url,
            Err(error) => {
                self.tabs[self.active_tab].status_message = error;
                self.editing_bookmark = Some(edit);
                return;
            }
        };

        if let Some(bookmark) = self
            .user_data
            .bookmarks
            .iter_mut()
            .find(|bookmark| bookmark.url == edit.original_url)
        {
            bookmark.title = title;
            bookmark.url = url;
        } else {
            self.tabs[self.active_tab].status_message = "Bookmark could not be found.".to_owned();
            self.editing_bookmark = None;
            return;
        }

        self.editing_bookmark = None;
        let active_tab = self.active_tab;
        match self.user_data.save() {
            Ok(()) => self.tabs[active_tab].status_message = "Bookmark updated.".to_owned(),
            Err(error) => self.tabs[active_tab].status_message = error,
        }
    }

    fn save_user_data(&mut self, status_tab: usize) {
        if let Err(error) = self.user_data.save() {
            self.tabs[status_tab].status_message = error;
        }
    }

    fn apply_action(&mut self, action: UiAction) {
        match action {
            UiAction::SelectTab(index) if index < self.tabs.len() => self.active_tab = index,
            UiAction::NewTab => self.add_tab(),
            UiAction::CloseTab(index) if index < self.tabs.len() => self.close_tab(index),
            UiAction::Navigate(index, input) if index < self.tabs.len() => {
                self.navigate(index, &input);
            }
            UiAction::OpenBookmark(index, url) if index < self.tabs.len() => {
                self.navigate_to_url(index, url);
            }
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            UiAction::BeginEditBookmark(url) => {
                self.begin_edit_bookmark(&url);
            }
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            UiAction::DeleteBookmark(url) => {
                self.delete_bookmark(&url);
            }
            UiAction::ToggleBookmark(index) if index < self.tabs.len() => {
                self.toggle_bookmark(index);
            }
            UiAction::ChooseNewTabPage(index, choice) if index < self.tabs.len() => match choice {
                NewTabChoice::DuckDuckGo => {
                    self.navigate_to_url(index, START_PAGE.to_owned());
                }
                NewTabChoice::Bookmark(url) => {
                    self.navigate_to_url(index, url);
                }
            },
            _ => {}
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
