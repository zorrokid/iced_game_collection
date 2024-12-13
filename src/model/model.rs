use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::collection_file::CollectionFile;

#[derive(Debug, Clone)]
pub struct GameListModel {
    pub id: String,
    pub name: String,
    pub can_delete: bool,
}

impl From<&Game> for GameListModel {
    fn from(game: &Game) -> Self {
        GameListModel {
            id: game.id.clone(),
            name: game.name.clone(),
            can_delete: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReleaseListModel {
    pub id: String,
    pub name: String,
}

impl From<&Release> for ReleaseListModel {
    fn from(release: &Release) -> Self {
        ReleaseListModel {
            id: release.id.clone(),
            name: release.name.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub id: String,
    pub name: String,
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemListModel {
    pub id: String,
    pub name: String,
    pub can_delete: bool,
}

impl From<&System> for SystemListModel {
    fn from(system: &System) -> Self {
        SystemListModel {
            id: system.id.clone(),
            name: system.name.clone(),
            can_delete: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub id: String,
    pub name: String,
    pub system_id: String,
    pub files: Vec<CollectionFile>,
    // Release can be a single game or compilation of games
    pub games: Vec<String>,
}

impl Display for Release {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emulator {
    pub id: String,
    pub name: String,
    pub executable: String,
    pub arguments: String,
    pub system_id: String,
    pub extract_files: bool,
    pub supported_file_type_extensions: Vec<String>,
}

pub trait HasId {
    fn id(&self) -> String;
}

impl HasId for ReleaseListModel {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl HasId for GameListModel {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl HasId for Game {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl HasId for System {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl HasId for Emulator {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl HasId for Release {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl HasId for SystemListModel {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Display for SystemListModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Game {
    pub id: String,
    pub name: String,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
    pub systems: Vec<System>,
    pub emulators: Vec<Emulator>,
    pub games: Vec<Game>,
    pub releases: Vec<Release>,
    pub settings: Settings,
}

#[derive(Debug, Clone)]
pub enum FolderType {
    Source,
    Destination,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub id: String,
    pub collection_root_dir: String,
}

impl HasId for Settings {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Emulator {
            id: get_new_id(),
            name: "".to_string(),
            executable: "".to_string(),
            arguments: "".to_string(),
            system_id: "".to_string(),
            extract_files: false,
            supported_file_type_extensions: vec![],
        }
    }
}

impl Default for System {
    fn default() -> Self {
        System {
            id: get_new_id(),
            name: "".to_string(),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            id: get_new_id(),
            name: "".to_string(),
        }
    }
}

impl Default for Release {
    fn default() -> Self {
        Release {
            id: get_new_id(),
            name: "".to_string(),
            system_id: "".to_string(),
            files: vec![],
            games: vec![],
        }
    }
}

pub fn get_new_id() -> String {
    Uuid::new_v4().to_string()
}
