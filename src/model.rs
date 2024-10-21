use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub releases: Vec<Release>,
}

pub struct GameListModel {
    pub id: i32,
    pub name: String,
}

impl From<&Game> for GameListModel {
    fn from(game: &Game) -> Self {
        GameListModel {
            id: game.id,
            name: game.name.clone(),
        }
    }
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
    pub system_id: i32,
    pub files: Vec<String>,
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

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
    pub games: Vec<Game>,
    pub systems: Vec<System>,
    pub emulators: Vec<Emulator>,
}

pub trait ToGameListModel {
    fn to_game_list_model(&self) -> Vec<GameListModel>;
}

pub trait DeleteSystem {
    fn delete_system(&mut self, system_id: i32);
}

impl ToGameListModel for Collection {
    fn to_game_list_model(&self) -> Vec<GameListModel> {
        self.games.iter().map(GameListModel::from).collect()
    }
}

impl DeleteSystem for Collection {
    fn delete_system(&mut self, system_id: i32) {
        self.games.iter_mut().for_each(|game| {
            game.releases
                .retain(|release| release.system_id != system_id)
        });
        self.emulators
            .retain(|emulator| emulator.system_id != system_id);
        self.systems.retain(|system| system.id != system_id);
    }
}

pub fn get_new_id<T: HasId>(items: &Vec<T>) -> i32 {
    items
        .iter()
        .max_by_key(|item| item.id())
        .map(|item| item.id() + 1)
        .unwrap_or(1)
}
