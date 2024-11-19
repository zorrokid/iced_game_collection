use crate::{
    error::Error,
    model::{Collection, Emulator, Game, GameListModel, HasId, Release, ReleaseListModel, System},
};
use async_std::{fs as fs_async, task};
use lazy_static::lazy_static;
use std::fs;
use std::sync::{Arc, RwLock};

const COLLECTION_FILE_NAME: &str = "games.json";

pub struct Database {
    collection: Collection,
}

impl Database {
    pub async fn load() -> Result<Collection, Error> {
        let data = fs_async::read_to_string(COLLECTION_FILE_NAME)
            .await
            .map_err(|e| Error::IoError(format!("Error reading collection {}", e)))?;
        let collection: Collection = serde_json::from_str(&data)
            .map_err(|e| Error::IoError(format!("Error deserializing collection {}", e)))?;
        Ok(collection)
    }

    pub fn save(&self) -> Result<(), Error> {
        let data = serde_json::to_string(&self.collection)
            .map_err(|e| Error::IoError(format!("Error serializing collection {}", e)))?;
        fs::write(COLLECTION_FILE_NAME, data)
            .map_err(|e| Error::IoError(format!("Error writing collection {}", e)))?;
        Ok(())
    }

    pub fn get_instance() -> Arc<RwLock<Database>> {
        lazy_static! {
            static ref INSTANCE: Arc<RwLock<Database>> = Arc::new(RwLock::new(Database {
                collection: task::block_on(Database::load()).unwrap_or_else(|_| Collection {
                    systems: vec![],
                    emulators: vec![],
                    games: vec![],
                    releases: vec![],
                    settings: Default::default(),
                })
            }));
        }
        INSTANCE.clone()
    }

    // get all items
    pub fn get_systems(&self) -> Vec<System> {
        self.collection.systems.clone()
    }

    pub fn get_games(&self) -> Vec<Game> {
        self.collection.games.clone()
    }

    pub fn get_emulators(&self) -> Vec<Emulator> {
        self.collection.emulators.clone()
    }

    // get single item

    pub fn get_system(&self, id: i32) -> Option<System> {
        get_cloned(&self.collection.systems, id)
    }

    pub fn get_emulator(&self, id: i32) -> Option<Emulator> {
        get_cloned(&self.collection.emulators, id)
    }

    pub fn get_game(&self, id: i32) -> Option<Game> {
        get_cloned(&self.collection.games, id)
    }

    pub fn get_release(&self, id: i32) -> Option<Release> {
        get_cloned(&self.collection.releases, id)
    }

    pub fn get_settings(&self) -> crate::model::Settings {
        self.collection.settings.clone()
    }

    // add_or_update

    pub fn add_or_update_system(&mut self, system: System) {
        add_or_update(&mut self.collection.systems, system);
    }

    pub fn add_or_update_release(&mut self, release: Release) {
        add_or_update(&mut self.collection.releases, release);
    }

    pub fn add_or_update_game_new(&mut self, game: Game) {
        add_or_update(&mut self.collection.games, game);
    }

    pub fn add_or_update_emulator(&mut self, emulator: Emulator) {
        add_or_update(&mut self.collection.emulators, emulator);
    }

    // delete

    pub fn delete_system(&mut self, id: i32) {
        // TODO: check if system is used in a release (delete is disabled in UI in this case but should be checked anyway)
        self.collection.systems.retain(|s| s.id != id);
    }

    pub fn delete_game(&mut self, game_id: i32) {
        // TODO: check if game is used in a release (delete is disabled in UI in this case but should be checked anyway)
        self.collection.games.retain(|game| game.id != game_id);
    }

    pub fn delete_emulator(&mut self, emulator_id: i32) {
        self.collection
            .emulators
            .retain(|emulator| emulator.id != emulator_id);
    }

    // to list model

    pub fn to_game_list_model(&self) -> Vec<GameListModel> {
        let mut list_models: Vec<GameListModel> = self
            .collection
            .games
            .iter()
            .map(GameListModel::from)
            .collect();
        for game in &mut list_models {
            let has_release = self
                .collection
                .releases
                .iter()
                .find(|r| r.games.contains(&game.id))
                .is_some();
            game.can_delete = !has_release;
        }
        list_models
    }

    pub fn to_release_list_model(&self) -> Vec<ReleaseListModel> {
        self.collection
            .releases
            .iter()
            .map(ReleaseListModel::from)
            .collect()
    }

    /*pub fn to_system_list_model(&self) -> Vec<SystemListModel> {
        let mut list_models: Vec<SystemListModel> = self
            .collection
            .systems
            .iter()
            .map(SystemListModel::from)
            .collect();
        for system in &mut list_models {
            let has_release = self
                .collection
                .releases
                .iter()
                .find(|r| r.system_id == system.id)
                .is_some();
            system.can_delete = !has_release;
        }
        list_models
    }*/

    // special

    pub fn get_releases_with_game(&self, id: i32) -> Vec<Release> {
        let releases_with_game = self
            .collection
            .releases
            .iter()
            .filter(|r| r.games.contains(&id))
            .cloned()
            .collect();
        releases_with_game
    }
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
