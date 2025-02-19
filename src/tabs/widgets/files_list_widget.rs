use std::{collections::HashMap, path::PathBuf};

use bson::oid::ObjectId;
use iced::{
    widget::{button, column, image, pick_list, row, text, Column},
    Element, Task,
};

use crate::{
    database_with_polo::DatabaseWithPolo,
    error::Error,
    files::delete_file,
    model::{
        collection_file::{CollectionFile, CollectionFileType},
        model::HasOid as _,
    },
    repository::repository::CollectionFilesReadRepository,
    util::{file_path_builder::FilePathBuilder, image::get_thumbnail_path},
};

use super::emulator_files_list_widget::{self, EmulatorFilesList};

pub struct FilesList {
    files: Vec<CollectionFile>,
    system_id: Option<ObjectId>,
    file_path_builder: FilePathBuilder,
    emulator_files_list: EmulatorFilesList,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartRemoveFile(ObjectId),
    FileReferenceRemoved(ObjectId, ObjectId),
    SetFiles(Vec<ObjectId>, ObjectId),
    ViewImage(PathBuf),
    FileDeleted(Result<(), Error>, ObjectId),
    EmulatorFilesList(super::emulator_files_list_widget::Message),
}

pub enum Action {
    ViewImage(PathBuf),
    RemoveFileReference(ObjectId),
    Run(Task<Message>),
    None,
}

impl FilesList {
    pub fn new(
        collection_root_dir: String,
        file_ids: Vec<ObjectId>,
        system_id: Option<ObjectId>,
    ) -> Self {
        let db = DatabaseWithPolo::get_instance();
        let files = db.get_collection_files(&file_ids).unwrap_or_else(|err| {
            println!("Failed to get files {:?}", err);
            vec![]
        });
        Self {
            files: files.clone(),
            file_path_builder: FilePathBuilder::new(collection_root_dir),
            system_id,
            emulator_files_list: EmulatorFilesList::new(system_id, files),
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::StartRemoveFile(id) => {
                println!("Remove file: {:?}", id);
                Action::RemoveFileReference(id)
            }
            Message::SetFiles(ids, system_id) => {
                let db = DatabaseWithPolo::get_instance();
                let files = db.get_collection_files(&ids);
                self.files = files.unwrap_or_else(|err| {
                    println!("Failed to get files {:?}", err);
                    vec![]
                });
                self.system_id = Some(system_id);
                println!("Set file ids: {:?}", ids);
                Action::None
            }
            Message::ViewImage(path) => Action::ViewImage(path),
            // TODO: this could be somewhere else:
            Message::FileReferenceRemoved(file_id, system_id) => {
                let file = self.files.iter().find(|f| f.id() == file_id);
                if let Some(file) = file {
                    let file_path = self.file_path_builder.build_file_path(&system_id, file);
                    let db = DatabaseWithPolo::get_instance();
                    match db.is_collection_file_in_release(&file_id) {
                        Ok(is_in_release) => {
                            if !is_in_release {
                                if let Ok(file_path) = file_path {
                                    // TODO: remove also thumbnail if exists
                                    return Action::Run(Task::perform(
                                        delete_file(file_path.clone()),
                                        move |result| Message::FileDeleted(result, file_id),
                                    ));
                                }
                            }
                        }
                        Err(err) => {
                            println!("Failed to get file references {:?}", err);
                            return Action::None;
                        }
                    }
                }
                Action::None
            }
            // TODO: this could be somewhere else:
            Message::FileDeleted(result, id) => match result {
                Ok(_) => {
                    // File was deleted from disk, now delete from database
                    let db = DatabaseWithPolo::get_instance();
                    match db.delete_collection_file(&id) {
                        Ok(_) => {
                            println!("File deleted {:?}", id);
                        }
                        Err(err) => {
                            // TODO: show message to user
                            println!("Failed to delete file {:?}", err);
                        }
                    };
                    Action::None
                }
                Err(err) => {
                    println!("Failed to delete file {:?}", err);
                    // TODO: show message to user
                    Action::None
                }
            },
            Message::EmulatorFilesList(message) => {
                match self.emulator_files_list.update(message) {
                    emulator_files_list_widget::Action::Run(task) => {
                        Action::Run(task.map(Message::EmulatorFilesList))
                    }
                    emulator_files_list_widget::Action::RemoveFileReference(_id) => {
                        // TODO
                        Action::None
                    }
                    emulator_files_list_widget::Action::None => Action::None,
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let emulator_files_list = self
            .emulator_files_list
            .view()
            .map(Message::EmulatorFilesList);
        let scan_files_list = self.create_files_list(CollectionFileType::CoverScan);
        let screenshot_files_list = self.create_files_list(CollectionFileType::Screenshot);

        column![emulator_files_list, scan_files_list, screenshot_files_list,].into()
    }

    fn create_files_list(&self, file_type: CollectionFileType) -> Element<Message> {
        println!("Create files list: {:?}", file_type);
        let files_list = self
            .files
            .iter()
            .filter(|f| f.collection_file_type == file_type)
            .filter_map(|file| {
                if let Some(system_id) = self.system_id {
                    if let Ok(thumb_path) = get_thumbnail_path(
                        file,
                        self.file_path_builder.get_collection_root_dir(),
                        &system_id,
                    ) {
                        if let Ok(file_path) =
                            self.file_path_builder.build_file_path(&system_id, file)
                        {
                            let image = image(thumb_path);
                            let view_image_button =
                                button(image).on_press(Message::ViewImage(file_path));
                            let delete_button =
                                button("Delete").on_press(Message::StartRemoveFile(file.id()));
                            return Some(row![view_image_button, delete_button].into());
                        }
                    }
                }

                None
            })
            .collect::<Vec<iced::Element<Message>>>();
        Column::with_children(files_list).into()
    }
}
