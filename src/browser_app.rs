use std::sync::mpsc::{self, Receiver, Sender};

use eframe::egui;
use wry::PageLoadEvent;

use crate::user_data::UserData;

mod bookmarks;
mod navigation;
mod tabs;
mod ui;

use bookmarks::BookmarkEditDraft;
use navigation::START_PAGE;
use tabs::BrowserTab;

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

enum NewTabChoice {
    DuckDuckGo,
    Bookmark(String),
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
