use bson::oid::ObjectId;
use iced::{
    widget::{button, pick_list, row},
    Element, Task,
};

use crate::{
    database_with_polo::DatabaseWithPolo,
    error::Error,
    files::{copy_file, delete_file, pick_file, PickedFile},
    model::{
        collection_file::{CollectionFile, CollectionFileType},
        model::HasOid,
    },
    util::file_path_builder::FilePathBuilder,
};

pub struct FileSelect {
    selected_file_type: Option<CollectionFileType>,
    system_id: Option<ObjectId>,
    file_path_builder: FilePathBuilder,
}

#[derive(Debug, Clone)]
pub enum Message {
    FileTypeSelected(CollectionFileType),
    StartFileSelection,
    FilePicked(Result<PickedFile, Error>),
    FileCopied(Result<ObjectId, Error>),
    SetSystemId(ObjectId),
    DeleteFile(CollectionFile),
    FileDeleted(Result<(), Error>, ObjectId),
    Reset,
}

pub enum Action {
    None,
    Run(Task<Message>),
    AddFile(ObjectId),
    //ViewImage(PathBuf),
    //DeleteFile(ObjectId),
}

impl FileSelect {
    pub fn new(files_root_dir: String) -> Self {
        let file_path_builder = FilePathBuilder::new(files_root_dir);

        Self {
            selected_file_type: None,
            system_id: None,
            file_path_builder,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::FileTypeSelected(file_type) => {
                self.selected_file_type = Some(file_type);
                Action::None
            }
            Message::StartFileSelection => {
                if self.system_id.is_none() || self.selected_file_type.is_none() {
                    return Action::None;
                }
                Action::Run(Task::perform(pick_file(), Message::FilePicked))
            }
            Message::FilePicked(result) => {
                if let (Some(system_id), Some(selected_file_type)) =
                    (self.system_id, self.selected_file_type.clone())
                {
                    match result {
                        Ok(picked_file) => {
                            let collection_file = CollectionFile {
                                _id: None,
                                original_file_name: picked_file.file_name.clone(),
                                collection_file_type: self.selected_file_type.clone().unwrap(),
                                files: picked_file.files.clone(),
                                is_zip: picked_file.is_zip,
                            };
                            let db = DatabaseWithPolo::get_instance();
                            // Add file info to database and copy file to collection directory
                            match db.add_collection_file(&collection_file) {
                                Ok(id) => Action::Run(Task::perform(
                                    copy_file(
                                        self.file_path_builder.build_target_directory(
                                            &system_id,
                                            &selected_file_type,
                                        ),
                                        id,
                                        picked_file,
                                    ),
                                    Message::FileCopied,
                                )),
                                Err(err) => {
                                    println!("Failed to add file {:?}", err);
                                    // TODO: show message to user
                                    Action::None
                                }
                            }
                        }
                        Err(_) => Action::None,
                    }
                } else {
                    Action::None
                }
            }
            Message::FileCopied(result) => match result {
                Ok(id) => Action::AddFile(id),
                // TODO: if copy fails, remove the file from the database
                Err(err) => {
                    println!("Failed to copy file {:?}", err);
                    // TODO: show message to user
                    Action::None
                }
            },
            Message::SetSystemId(system_id) => {
                self.system_id = Some(system_id);
                Action::None
            }
            Message::DeleteFile(file) => {
                // TODO: maybe file could be used in multiple releases
                // - add a reference list
                // - check if file has references to releases
                // - delete file only if it's not used in any release
                if let Some(system_id) = self.system_id {
                    if let Ok(file_path) = self.file_path_builder.build_file_path(&system_id, &file)
                    {
                        // TODO: remove also thumbnail if exists
                        return Action::Run(Task::perform(
                            delete_file(file_path.clone()),
                            move |result| Message::FileDeleted(result, file.id()),
                        ));
                    }
                }
                Action::None
            }
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
            Message::Reset => {
                self.selected_file_type = None;
                self.system_id = None;
                Action::None
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        row![self.create_file_picker(),].into()
    }

    fn create_file_picker(&self) -> Element<Message> {
        let collection_file_type_picker = pick_list(
            vec![
                CollectionFileType::Rom,
                CollectionFileType::DiskImage,
                CollectionFileType::CoverScan,
                CollectionFileType::Manual,
                CollectionFileType::Screenshot,
                CollectionFileType::TapeImage,
            ],
            self.selected_file_type.clone(),
            Message::FileTypeSelected,
        );
        let add_file_button = button("Add File").on_press_maybe(
            (self.system_id.is_some() && self.selected_file_type.is_some())
                .then_some(Message::StartFileSelection),
        );
        row![collection_file_type_picker, add_file_button].into()
    }
}
