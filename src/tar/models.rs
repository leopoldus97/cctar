pub struct TarArchive {
    pub files: Vec<TarFile>,
}

// Name: The name of the file or directory. Occupies the first 100 bytes of the header.
// Mode: The file permissions. Occupies the next 8 bytes.
// UID: The user ID of the owner. Occupies the next 8 bytes.
// GID: The group ID of the owner. Occupies the next 8 bytes.
// Size: The size of the file in bytes. Occupies the next 12 bytes.
// MTime: The last modification time of the file. Occupies the next 12 bytes.
// Checksum: The checksum for the header block. Occupies the next 8 bytes.
// Typeflag: The type of the file. Occupies the next 1 byte.
// Linkname: The name of the linked file for hard or symbolic links. Occupies the next 100 bytes.
// Magic: The magic string. Occupies the next 6 bytes.
// Version: The version of the tar file format. Occupies the next 2 bytes.
// Uname: The user name of the owner. Occupies the next 32 bytes.
// Gname: The group name of the owner. Occupies the next 32 bytes.
// Devmajor: The major number of the device for character or block device files. Occupies the next 8 bytes.
// Devminor: The minor number of the device for character or block device files. Occupies the next 8 bytes.
// Prefix: The prefix of the file name. Occupies the next 155 bytes.

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
