use bson::oid::ObjectId;

use crate::{
    error::Error,
    model::{
        collection_file::CollectionFile,
        model::{Game, HasOid, System},
    },
    repository::repository::{
        CollectionFilesReadRepository, GamesReadRepository, ReleaseReadRepository,
        SystemReadRepository,
    },
};

#[derive(Debug, Clone)]
pub struct ReleaseViewModel {
    pub id: ObjectId,
    pub name: String,
    pub system: System,
    pub files: Vec<CollectionFile>,
    // Release can be a single game or compilation of games
    pub games: Vec<Game>,
}

pub fn get_release_view_model<R>(
    release_id: &ObjectId,
    repository: &R,
) -> Result<Option<ReleaseViewModel>, Error>
where
    R: ReleaseReadRepository
        + GamesReadRepository
        + SystemReadRepository
        + CollectionFilesReadRepository,
{
    let release = repository.get_release(release_id)?;

    if let Some(release) = release {
        let games = repository.get_games(&release.games)?;
        let files = repository.get_collection_files(&release.files)?;

        let system = match release.system_id {
            Some(system_id) => repository.get_system(&system_id)?,
            None => None,
        };

        match system {
            Some(system) => Ok(Some(ReleaseViewModel {
                id: release.id(),
                name: release.name.clone(),
                system,
                files,
                games,
            })),
            // TODO: there probably should be a db model with obligatory system_id and save model with optional system_id
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        model::{
            collection_file::{CollectionFile, CollectionFileType},
            model::{Game, Release, System},
        },
        repository::mock_repository::MockRepository,
    };
    use bson::oid::ObjectId;
    use std::collections::HashMap;

    #[test]
    fn test_get_release_view_model() {
        let release_id = ObjectId::new();
        let game_id = ObjectId::new();
        let file_id = ObjectId::new();
        let system_id = ObjectId::new();

        let release = Release {
            _id: Some(release_id.clone()),
            name: "Test Release".to_string(),
            games: vec![game_id.clone()],
            files: vec![file_id.clone()],
            system_id: Some(system_id.clone()),
        };

        let game = Game {
            _id: Some(game_id.clone()),
            name: "Test Game".to_string(),
        };

        let collection_file = CollectionFile {
            _id: Some(file_id.clone()),
            original_file_name: "test_file.zip".to_string(),
            is_zip: true,
            files: None,
            collection_file_type: CollectionFileType::DiskImage,
        };

        let system = System {
            _id: Some(system_id.clone()),
            name: "Test System".to_string(),
        };

        let mut releases = HashMap::new();
        releases.insert(release_id.clone(), release);

        let mut games = HashMap::new();
        games.insert(game_id.clone(), game);

        let mut collection_files = HashMap::new();
        collection_files.insert(file_id.clone(), collection_file);

        let mut systems = HashMap::new();
        systems.insert(system_id.clone(), system);

        let repository = MockRepository {
            releases,
            games,
            collection_files,
            systems,
        };

        let result = get_release_view_model(&release_id, &repository).unwrap();
        assert!(result.is_some());

        let release_view_model = result.unwrap();
        assert_eq!(release_view_model.id, release_id);
        assert_eq!(release_view_model.name, "Test Release");
        assert_eq!(release_view_model.system.id(), system_id);
        assert_eq!(release_view_model.games.len(), 1);
        assert_eq!(release_view_model.games[0].id(), game_id);
        assert_eq!(release_view_model.files.len(), 1);
        assert_eq!(release_view_model.files[0].id(), file_id);
    }
}
