//! Abstract file identity.

// We re-export this type and its crate for easier usage.
pub use compact_str::{CompactString, self as compact_str};

/// A abstract file identifier.
pub enum FileId<'s> {
    /// A reference to an existing path.
    RefPath(&'s str),
    
    /// An existing -inlined- path.
    DynPath(CompactString),
    
    /// A unique identifier.
    /// 
    /// Use to skip searching for a file in a filesystems layout, directly accessing it instead.
    /// 
    /// **Note:** Only some filesystems support this type.
    UniqueIdent(uuid::Uuid),
}

/// A abstract file identifier with owned lifetime.
pub type StaticFileId = FileId<'static>;

impl<'s> ToOwned for FileId<'s> {
    type Owned = FileId<'s>;
    
    fn to_owned(&self) -> Self::Owned {
        match self {
            FileId::RefPath(r) => FileId::DynPath((*r).into()),
            FileId::DynPath(d) => FileId::DynPath(d.clone()),
            FileId::UniqueIdent(i) => FileId::UniqueIdent(i.clone()),
        }
    }
}

#[test]
fn fileid_size() {
    let file_size = std::mem::size_of::<StaticFileId>();
    eprintln!("Size of a &str: {} bytes", std::mem::size_of::<&str>());
    eprintln!("Size of a µStr: {} bytes", std::mem::size_of::<CompactString>());
    eprintln!("Size of a UUID: {} bytes", std::mem::size_of::<uuid::Uuid>());
    eprintln!("Size of a FileId: {file_size} bytes");
    assert!(file_size == 32)
}
