//! Factory for filesystems.
//! 
//! When enabled, [`crate::filesystems::dir`] can open other fileysystems.

use inventory;
use std::path::Path;

/// A function that, given a [`Path`], creates a filesystem.
pub type FilesystemConstructor = fn(&Path) -> miette::Result<Box<dyn crate::Filesystem>>;

/// A [`inventory::submit`]-d struct holding a [`FilesystemConstructor`]
pub struct FilesystemDescriptor {
    /// The extension of the filesystem.
    pub extension: &'static str,
    
    /// A function that, given a `file`, creates a filesystem.
    pub construct: FilesystemConstructor,
    
    /// Should the filesystem be mounted asynchronously?
    /// 
    /// This should be `true` for any filesystem that needs to do noticable work before being usable.
    pub asynchronous: bool,
}

inventory::collect!(FilesystemDescriptor);

/// Loads available filesystems from the inventory.
/// 
/// When enabled, [`crate::filesystems::dir`] will call this once when created.
pub fn filesystems() -> Vec<&'static FilesystemDescriptor> {
    inventory::iter::<FilesystemDescriptor>.into_iter().collect()
}
