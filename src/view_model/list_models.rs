use core::fmt;
use std::fmt::{Display, Formatter};

use bson::oid::ObjectId;

use crate::{
    error::Error,
    model::model::{Game, HasOid, System},
    repository::repository::{GamesReadRepository, SystemReadRepository},
};

#[derive(Debug, Clone)]
pub struct GameListModel {
    pub id: ObjectId,
    pub name: String,
    pub can_delete: bool,
}

impl From<&Game> for GameListModel {
    fn from(game: &Game) -> Self {
        GameListModel {
            id: game.id(),
            name: game.name.clone(),
            can_delete: false,
        }
    }
}

pub fn get_games_as_list_model<R>(repository: &R) -> Result<Vec<GameListModel>, Error>
where
    R: GamesReadRepository,
{
    let games = repository.get_all_games()?;
    let mut list_models: Vec<GameListModel> = games.iter().map(GameListModel::from).collect();
    for game in &mut list_models {
        game.can_delete = !repository.is_game_in_release(&game.id)?;
    }
    Ok(list_models)
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemListModel {
    pub id: ObjectId,
    pub name: String,
    pub can_delete: bool,
}

impl HasOid for SystemListModel {
    fn id(&self) -> ObjectId {
        self.id.clone()
    }
}

impl Display for SystemListModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&System> for SystemListModel {
    fn from(system: &System) -> Self {
        SystemListModel {
            id: system.id().clone(),
            name: system.name.clone(),
            can_delete: false,
        }
    }
}

pub fn get_systems_in_list_model<R>(repository: &R) -> Result<Vec<SystemListModel>, Error>
where
    R: SystemReadRepository,
{
    let systems = repository.get_all_systems()?;
    let mut list_models: Vec<SystemListModel> = systems.iter().map(SystemListModel::from).collect();
    for system in &mut list_models {
        system.can_delete = !repository.is_system_in_release(&system.id)?;
    }
    Ok(list_models)
}
