use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use polodb_core::bson::oid::ObjectId;

pub trait GetIdString {
    fn get_id_string(&self) -> String;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub _id: Option<ObjectId>,
    pub name: String,
    pub notes: Option<String>,
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl GetIdString for System {
    fn get_id_string(&self) -> String {
        self.id().to_hex()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub _id: Option<ObjectId>,
    pub name: String,
    pub system_id: Option<ObjectId>,
    pub files: Vec<ObjectId>,
    // Release can be a single game or compilation of games
    pub games: Vec<ObjectId>,
}

impl Display for Release {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emulator {
    pub _id: Option<ObjectId>,
    pub name: String,
    pub executable: String,
    pub arguments: String,
    pub system_id: Option<ObjectId>,
    pub extract_files: bool,
    pub supported_file_type_extensions: Vec<String>,
    pub notes: Option<String>,
}

impl HasOid for Game {
    fn id(&self) -> ObjectId {
        self._id.clone().expect("Object id not set")
    }
}

impl HasOid for System {
    fn id(&self) -> ObjectId {
        self._id.clone().expect("Object id not set")
    }
}

pub trait HasOid {
    fn id(&self) -> ObjectId;
}

impl HasOid for Emulator {
    fn id(&self) -> ObjectId {
        self._id.clone().expect("Object id not set")
    }
}

impl HasOid for Release {
    fn id(&self) -> ObjectId {
        self._id.clone().expect("Object id not set")
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Game {
    pub _id: Option<ObjectId>,
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

impl Default for Emulator {
    fn default() -> Self {
        Emulator {
            _id: None,
            name: "".to_string(),
            executable: "".to_string(),
            arguments: "".to_string(),
            system_id: None,
            extract_files: false,
            supported_file_type_extensions: vec![],
            notes: None,
        }
    }
}

impl Default for System {
    fn default() -> Self {
        System {
            _id: None,
            name: "".to_string(),
            notes: None,
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            _id: None,
            name: "".to_string(),
        }
    }
}

impl Default for Release {
    fn default() -> Self {
        Release {
            _id: None,
            name: "".to_string(),
            system_id: None,
            files: vec![],
            games: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleasesByGame {
    pub _id: ObjectId, // game id
    pub release_ids: Vec<ObjectId>,
}

impl HasOid for ReleasesByGame {
    fn id(&self) -> ObjectId {
        self._id.clone()
    }
}
