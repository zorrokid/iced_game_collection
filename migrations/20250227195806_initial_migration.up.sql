CREATE TABLE settings (
    id blob PRIMARY KEY,
    key TEXT NOT NULL,
    value TEXT NOT NULL
);

CREATE TABLE system (
    id blob PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE system_note (
    id blob PRIMARY KEY,
    system_id blob NOT NULL,
    note TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (system_id) REFERENCES system(id)
);

CREATE TABLE emulator (
    id blob PRIMARY KEY,
    name TEXT NOT NULL,
    system_id blob NOT NULL,
    executable TEXT NOT NULL,
    arguments TEXT,
    extract_files BOOLEAN NOT NULL,
    supported_extensions TEXT NOT NULL,
    FOREIGN KEY (system_id) REFERENCES system(id)
);

CREATE TABLE emulator_note (
    id blob PRIMARY KEY,
    emulator_id blob NOT NULL,
    note TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (emulator_id) REFERENCES emulator(id)
);

CREATE TABLE release (
    id blob PRIMARY KEY,
    name TEXT NOT NULL,
    system_id blob NOT NULL,
    notes TEXT,
    FOREIGN KEY (system_id) REFERENCES system(id)
);

CREATE TABLE collection_file (
    id blob PRIMARY KEY,
    original_file_name TEXT NOT NULL,
    is_archive BOOLEAN NOT NULL,
    archive_type INTEGER NOT NULL,
    file_type INTEGER NOT NULL
 );

CREATE TABLE file_info (
    id blob PRIMARY KEY,
    sha1_checksum TEXT NOT NULL,
    file_size INTEGER NOT NULL
);

CREATE TABLE collection_file_file_info (
    collection_file_id blob NOT NULL,
    file_info_id blob NOT NULL,
    -- same file can have different names in different collection_files
    file_name TEXT NOT NULL,
    PRIMARY KEY (collection_file_id, file_info_id),
    FOREIGN KEY (collection_file_id) REFERENCES collection_file(id),
    FOREIGN KEY (file_info_id) REFERENCES file_info(id)
);

CREATE TABLE franchise (
    id blob PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE software_title (
    id blob PRIMARY KEY,
    name TEXT NOT NULL,
    franchise_id blob
);

CREATE TABLE release_collection_file (
    release_id blob NOT NULL,
    collection_file_id blob NOT NULL,
    PRIMARY KEY (release_id, collection_file_id),
    FOREIGN KEY (release_id) REFERENCES release(id)
);

CREATE TABLE release_software_title (
    release_id blob NOT NULL,
    software_title_id blob NOT NULL,
    PRIMARY KEY (release_id, software_title_id),
    FOREIGN KEY (release_id) REFERENCES release(id)
);
