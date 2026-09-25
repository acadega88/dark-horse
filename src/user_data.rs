use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct UserData {
    pub bookmarks: Vec<Bookmark>,
}

impl UserData {
    pub fn load() -> (Self, Option<String>) {
        let path = match data_file_path() {
            Ok(path) => path,
            Err(error) => return (Self::default(), Some(error)),
        };

        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return (Self::default(), None);
            }
            Err(error) => {
                return (
                    Self::default(),
                    Some(format!("Could not read local browser settings: {error}")),
                );
            }
        };

        match serde_json::from_str(&contents) {
            Ok(user_data) => (user_data, None),
            Err(error) => (
                Self::default(),
                Some(format!("Could not parse local browser settings: {error}")),
            ),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = data_file_path()?;
        let parent = path
            .parent()
            .ok_or_else(|| "Could not find the local settings folder.".to_owned())?;

        fs::create_dir_all(parent)
            .map_err(|error| format!("Could not create the local settings folder: {error}"))?;

        let contents = serde_json::to_string_pretty(self)
            .map_err(|error| format!("Could not encode local browser settings: {error}"))?;

        fs::write(path, contents)
            .map_err(|error| format!("Could not save local browser settings: {error}"))
    }
}

fn data_file_path() -> Result<PathBuf, String> {
    ProjectDirs::from("com", "acadega88", "dark-horse")
        .map(|project_dirs| project_dirs.data_dir().join("browser-data.json"))
        .ok_or_else(|| "Could not find the local application-data folder.".to_owned())
}
