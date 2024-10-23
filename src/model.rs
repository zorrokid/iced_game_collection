use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub releases: Vec<Release>,
}

impl Game {
    pub fn add_or_update_release(&mut self, release: Release) {
        add_or_update(&mut self.releases, release);
    }
    pub fn delete_release(&mut self, release_id: i32) {
        self.releases.retain(|release| release.id != release_id);
    }
    pub fn get_release(&self, id: i32) -> Option<Release> {
        get_cloned(&self.releases, id)
    }
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
    pub roms_source_path: String,
    pub roms_destination_path: String,
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

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
    pub games: Vec<Game>,
    pub systems: Vec<System>,
    pub emulators: Vec<Emulator>,
}

impl Collection {
    pub fn delete_system(&mut self, system_id: i32) {
        self.games.iter_mut().for_each(|game| {
            game.releases
                .retain(|release| release.system_id != system_id)
        });
        self.emulators
            .retain(|emulator| emulator.system_id != system_id);
        self.systems.retain(|system| system.id != system_id);
    }

    pub fn delete_game(&mut self, game_id: i32) {
        self.games.retain(|game| game.id != game_id);
    }
    pub fn delete_emulator(&mut self, emulator_id: i32) {
        self.emulators.retain(|emulator| emulator.id != emulator_id);
    }
    pub fn add_or_update_game(&mut self, game: Game) {
        add_or_update(&mut self.games, game);
    }

    pub fn add_or_update_system(&mut self, system: System) {
        add_or_update(&mut self.systems, system);
    }

    pub fn add_or_update_emulator(&mut self, emulator: Emulator) {
        add_or_update(&mut self.emulators, emulator);
    }
    pub fn to_game_list_model(&self) -> Vec<GameListModel> {
        self.games.iter().map(GameListModel::from).collect()
    }
    pub fn get_system(&self, id: i32) -> Option<System> {
        get_cloned(&self.systems, id)
    }
    pub fn get_emulator(&self, id: i32) -> Option<Emulator> {
        get_cloned(&self.emulators, id)
    }
}

pub fn get_new_id<T: HasId>(items: &Vec<T>) -> i32 {
    items
        .iter()
        .max_by_key(|item| item.id())
        .map(|item| item.id() + 1)
        .unwrap_or(1)
}

fn add_or_update<T: HasId>(items: &mut Vec<T>, item: T) {
    if let Some(existing_item) = items.iter_mut().find(|i| i.id() == item.id()) {
        *existing_item = item;
    } else {
        items.push(item);
    }
}

fn get_cloned<T: HasId + Clone>(items: &Vec<T>, id: i32) -> Option<T> {
    items.iter().find(|item| item.id() == id).cloned()
}

#[derive(Debug, Clone)]
pub enum FolderType {
    Source,
    Destination,
}
