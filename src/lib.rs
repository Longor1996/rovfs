//! Virtual Filesystem Implementation
//! 
//! This is an implementation of a *read-only* **virtual filesystem** with several backends.
//! 
//! The primary types to look at:
//! - [`file::FileId`]
//! - [`filesystem::Filesystem`]
//! - [`reader::ReadBlob`]
//! 
//! To construct a filesystem, look at the individual [`filesystems`].
#![deny(missing_docs)]

pub use miette;
pub use futures_lite;
pub use crossbeam::channel;

pub use file::FileId;
pub use filesystem::Filesystem;

use futures_lite::future::Boxed as BoxFuture;
use miette::{Result, IntoDiagnostic};

pub mod file;
pub mod fext;
pub mod memblob;
pub mod reader;
pub mod filesystem;

#[cfg(feature = "inventory")]
pub mod factory;

/// Various filesystem implementations.
pub mod filesystems {
    /// [`mnt::MountFs`] filesystem.
    /// 
    /// It is recommended to use this filesystem as 'root',
    /// when dealing with a multitude of filesystems.
    /// 
    /// TODO: mnt / mounting
    pub mod mnt;
    
    // ----- Direct Filesystems
    
    /// [`exe::EmbeddedFs`] filesystem.
    /// 
    /// TODO: exe / embedded
    pub mod exe;
    
    /// [`run::RuntimeFs`] filesystem.
    /// 
    /// TODO: run / runtime filesystem
    pub mod run;
    
    /// [`dir::DirectoryFs`] filesystem.
    /// 
    /// TODO: dir / directory
    pub mod dir;
    
    
    
    /// **Archive** Filesystems
    pub mod archives {
        /// Zip-Archive filesystem.
        /// 
        /// TODO: zip / zip archive
        pub mod zip;
        
        /// Multilayer-Archive filesystem.
        /// 
        /// TODO: mla / multilayer archive
        pub mod mla;
        
        /// Tape-Archive filesystem.
        /// 
        /// TODO: tar / tape archive
        pub mod tar;
        
        /// Multipart-Archive filesystem.
        /// 
        /// TODO: mpa / multipart archive
        pub mod mpa;
        
        /// FAT-Image filesystem.
        /// 
        /// TODO: fat / fat filesystem as archive
        pub mod fat;
    }
    
    /// **Database** Filesystems
    pub mod databases {
        /// SQL-Database filesystem.
        /// 
        /// TODO: sql / sqlite as archive
        pub mod sql;
        
        /// Git-Repository filesystem.
        /// 
        /// TODO: git / git as archive
        pub mod git;
    }
    
    /// **Remote** Filesystems
    pub mod remotes {
        /// Networked-Session filesystem.
        /// 
        /// TODO: net / remote session files
        pub mod net;
        
        /// Websocket-Download filesystem.
        /// 
        /// TODO: web / websocket download
        pub mod web;
        
        /// Internet-Download filesystem.
        /// 
        /// TODO: url / https download
        pub mod url;
    }
}
