CREATE TABLE setting (
    key TEXT NOT NULL PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE system (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);


CREATE TABLE emulator (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    system_id INTEGER NOT NULL,
    executable TEXT NOT NULL,
    arguments TEXT,
    extract_files INTEGER NOT NULL,
    supported_extensions TEXT NOT NULL,
    FOREIGN KEY (system_id) REFERENCES system(id)
);


CREATE TABLE release (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE release_system (
    release_id INTEGER NOT NULL,
    system_id INTEGER NOT NULL,
    PRIMARY KEY (release_id, system_id),
    FOREIGN KEY (release_id) REFERENCES release(id),
    FOREIGN KEY (system_id) REFERENCES system(id)
);

CREATE TABLE collection_file (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    original_file_name TEXT NOT NULL,
    is_archive INTEGER NOT NULL,
    archive_type INTEGER NOT NULL,
    file_type INTEGER NOT NULL
 );

CREATE TABLE file_info (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sha1_checksum TEXT NOT NULL,
    file_size INTEGER NOT NULL
);

CREATE TABLE collection_file_file_info (
    collection_file_id INTEGER NOT NULL,
    file_info_id INTEGER NOT NULL,
    -- same file can have different names in different collection_files
    file_name TEXT NOT NULL,
    PRIMARY KEY (collection_file_id, file_info_id),
    FOREIGN KEY (collection_file_id) REFERENCES collection_file(id),
    FOREIGN KEY (file_info_id) REFERENCES file_info(id)
);

CREATE TABLE franchise (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE software_title (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    franchise_id INTEGER,
    FOREIGN KEY (franchise_id) REFERENCES franchise(id)
);

CREATE TABLE release_collection_file (
    release_id INTEGER NOT NULL,
    collection_file_id INTEGER NOT NULL,
    PRIMARY KEY (release_id, collection_file_id),
    FOREIGN KEY (release_id) REFERENCES release(id)
);

CREATE TABLE release_software_title (
    release_id INTEGER NOT NULL,
    software_title_id INTEGER NOT NULL,
    PRIMARY KEY (release_id, software_title_id),
    FOREIGN KEY (release_id) REFERENCES release(id),
    FOREIGN KEY (software_title_id) REFERENCES software_title(id)
);

CREATE INDEX idx_release_software_title_software_title_id ON release_software_title(software_title_id);

CREATE TABLE system_note (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    system_id INTEGER DEFAULT NULL,
    note TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (system_id) REFERENCES system(id)
);

CREATE TABLE emulator_note (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    emulator_id INTEGER DEFAULT NULL,
    note TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (emulator_id) REFERENCES system(id)
);

CREATE TABLE release_note (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    release_id INTEGER DEFAULT NULL,
    note TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (release_id) REFERENCES release(id)
);

