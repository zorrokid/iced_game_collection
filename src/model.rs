use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
    pub games: Vec<Game>,
    pub systems: Vec<System>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub releases: Vec<Release>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub id: i32,
    pub name: String,
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub id: i32,
    pub name: String,
    pub system: System,
}

impl Display for Release {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
