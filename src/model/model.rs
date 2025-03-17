use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::impl_has_oid;

pub trait HasIdField {
    fn get_id(&self) -> &Option<i64>;
    fn set_id(&mut self, id: i64);
}

pub trait HasOid {
    fn id(&self) -> i64;
    fn has_id(&self) -> bool;
    fn with_id(self, id: i64) -> Self;
    fn get_id_string(&self) -> String;
}

impl<T> HasOid for T
where
    T: HasIdField,
{
    fn id(&self) -> i64 {
        self.get_id().expect("id not set")
    }

    fn has_id(&self) -> bool {
        self.get_id().is_some()
    }

    fn with_id(mut self, id: i64) -> Self {
        self.set_id(id);
        self
    }
    fn get_id_string(&self) -> String {
        self.id().to_string()
    }
}

impl_has_oid!(Emulator System SoftwareTitle Release Franchise);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub id: i64,
    pub name: String,
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Release {
    pub id: i64,
    pub name: String,
}

impl Display for Release {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emulator {
    pub id: i64,
    pub name: String,
    pub executable: String,
    pub arguments: String,
    pub system_id: i64,
    pub extract_files: bool,
    pub supported_extensions: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Franchise {
    pub id: Option<i64>,
    pub name: String,
}

impl Display for Franchise {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct SoftwareTitle {
    pub id: i64,
    pub name: String,
    pub franchise_id: Option<i64>,
}

impl Display for SoftwareTitle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// #[derive(Default, Serialize, Deserialize, Debug, Clone)]
//pub struct Collection {
//    pub systems: Vec<System>,
//    pub emulators: Vec<Emulator>,
//    pub games: Vec<SoftwareTitle>,
//    pub releases: Vec<Release>,
//    pub settings: Settings,
//}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    pub key: String,
    pub value: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
struct Note {
    pub id: i64,
    pub note: String,
    pub release_id: Option<i64>,
    pub emulator_id: Option<i64>,
    pub system_id: Option<i64>,
}

impl Default for Emulator {
    fn default() -> Self {
        Emulator {
            id: None,
            name: "".to_string(),
            executable: "".to_string(),
            arguments: "".to_string(),
            system_id: None,
            extract_files: false,
            supported_extensions: "".to_string(),
        }
    }
}

impl Default for System {
    fn default() -> Self {
        System {
            id: None,
            name: "".to_string(),
        }
    }
}

impl Default for SoftwareTitle {
    fn default() -> Self {
        SoftwareTitle {
            id: None,
            name: "".to_string(),
            franchise_id: None,
        }
    }
}

impl Default for Release {
    fn default() -> Self {
        Release {
            id: None,
            name: "".to_string(),
        }
    }
}

pub trait CanBeLinkedToReleases {
    fn release_ids(&self) -> Vec<i64>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleasesByGame {
    pub id: i64, // game id
    pub release_ids: Vec<i64>,
}

impl CanBeLinkedToReleases for ReleasesByGame {
    fn release_ids(&self) -> Vec<i64> {
        self.release_ids.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleasesByFile {
    pub id: i64, // file id
    pub release_ids: Vec<i64>,
}

impl CanBeLinkedToReleases for ReleasesByFile {
    fn release_ids(&self) -> Vec<i64> {
        self.release_ids.clone()
    }
}
