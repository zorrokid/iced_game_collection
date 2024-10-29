use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

pub struct GameListModel {
    pub id: i32,
    pub name: String,
    pub can_delete: bool,
}

impl From<&Game> for GameListModel {
    fn from(game: &Game) -> Self {
        GameListModel {
            id: game.id,
            name: game.name.clone(),
            can_delete: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReleaseListModel {
    pub id: i32,
    pub name: String,
}

impl From<&Release> for ReleaseListModel {
    fn from(release: &Release) -> Self {
        ReleaseListModel {
            id: release.id,
            name: release.name.clone(),
        }
    }
}

impl HasId for ReleaseListModel {
    fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub id: i32,
    pub name: String,
    pub roms_source_path: String,
    pub roms_destination_path: String,
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemListModel {
    pub id: i32,
    pub name: String,
    pub can_delete: bool,
}

impl From<&System> for SystemListModel {
    fn from(system: &System) -> Self {
        SystemListModel {
            id: system.id,
            name: system.name.clone(),
            can_delete: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub id: i32,
    pub name: String,
    pub system_id: i32,
    pub files: Vec<String>,
    // Release can be a single game or compilation of games
    pub games: Vec<i32>,
}

impl Display for Release {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emulator {
    pub id: i32,
    pub name: String,
    pub executable: String,
    pub arguments: String,
    pub system_id: i32,
}

pub trait HasId {
    fn id(&self) -> i32;
}

impl HasId for Game {
    fn id(&self) -> i32 {
        self.id
    }
}

impl HasId for System {
    fn id(&self) -> i32 {
        self.id
    }
}

impl HasId for Emulator {
    fn id(&self) -> i32 {
        self.id
    }
}

impl HasId for Release {
    fn id(&self) -> i32 {
        self.id
    }
}

impl HasId for SystemListModel {
    fn id(&self) -> i32 {
        self.id
    }
}

impl Display for SystemListModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Game {
    pub id: i32,
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
}

#[derive(Debug, Clone)]
pub enum FolderType {
    Source,
    Destination,
}
