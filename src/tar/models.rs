pub struct TarArchive {
    pub files: Vec<TarFile>,
}

pub struct TarFile {
    pub file_name: String,
    pub file_mode: u128,
    pub owner_uid: u128,
    pub group_uid: u128,
    pub file_size: u128,
    pub last_mod_time: u128,
    pub checksum: String,
    pub link_indicator: String,
    pub linked_file_nbame: String,
    pub ustar: String,
    pub ustar_version: String,
    pub owner_name: String,
    pub group_name: String,
    pub device_major: String,
    pub device_minor: String,
    pub prefix: String,
    pub body: Vec<u8>,
}
