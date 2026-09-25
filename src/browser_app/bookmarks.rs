use eframe::egui;

use crate::user_data::Bookmark;

use super::{BrowserApp, UiAction};

#[derive(Clone)]
pub(super) struct BookmarkEditDraft {
    pub(super) original_url: String,
    pub(super) title: String,
    pub(super) url: String,
}

pub(super) fn bookmark_button(
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
pub(super) fn show_bookmark_context_menu(frame: &eframe::Frame) -> Option<bool> {
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

impl BrowserApp {
    pub(super) fn toggle_bookmark(&mut self, tab_index: usize) {
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

    pub(super) fn begin_edit_bookmark(&mut self, url: &str) {
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

    pub(super) fn delete_bookmark(&mut self, url: &str) {
        self.user_data
            .bookmarks
            .retain(|bookmark| bookmark.url != url);
        let active_tab = self.active_tab;
        self.save_user_data(active_tab);
    }

    pub(super) fn save_bookmark_edit(&mut self, edit: BookmarkEditDraft) {
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
}
