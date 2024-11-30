use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub directory: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CollectionFileType {
    Rom,
    DiskImage,
    TapeImage,
    ScreenShot,
    Manual,
    CoverScan,
}

impl CollectionFileType {
    pub fn directory(&self) -> &str {
        match self {
            CollectionFileType::Rom => "roms",
            CollectionFileType::DiskImage => "disk_images",
            CollectionFileType::TapeImage => "tape_images",
            CollectionFileType::ScreenShot => "screenshots",
            CollectionFileType::Manual => "manuals",
            CollectionFileType::CoverScan => "cover_scans",
        }
    }
}

impl ToString for CollectionFileType {
    fn to_string(&self) -> String {
        match self {
            CollectionFileType::Rom => "Rom".to_string(),
            CollectionFileType::DiskImage => "Disk Image".to_string(),
            CollectionFileType::TapeImage => "Tape Image".to_string(),
            CollectionFileType::ScreenShot => "Screen Shot".to_string(),
            CollectionFileType::Manual => "Manual".to_string(),
            CollectionFileType::CoverScan => "Cover scan".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollectionFile {
    pub id: String,
    pub file_name: String,
    pub is_zip: bool,
    pub files: Option<Vec<FileInfo>>,
    pub collection_file_type: CollectionFileType,
}

impl Display for CollectionFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.file_name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileInfo {
    pub name: String,
    pub checksum: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub collection_root_dir: String,
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
        }
    }
}

impl Default for System {
    fn default() -> Self {
        System {
            id: get_new_id(),
            name: "".to_string(),
            directory: "".to_string(),
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

fn get_new_id() -> String {
    Uuid::new_v4().to_string()
}
