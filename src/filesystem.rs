//! Definition of an abstract filesystem.
use super::*;

/// A virtual/abstract filesystem.
pub trait Filesystem: std::fmt::Debug {
    /// Returns a [`reader::ReadBlobObj`] that can resolve into the bytes of the given file.
    fn read_blob(&self, id: file::FileIdArg<'_>, ext: fext::FileExtCow<'_>) -> BoxFuture<Result<reader::ReadBlobObj>>;
    
    /// Returns a [`reader::ReadListObj`] that can resolve into the entries of the given directory.
    fn read_list(&self, id: file::FileIdArg<'_>) -> BoxFuture<Result<reader::ReadListObj>>;
    
    /// Returns the parent of the given [`file::FileIdArg`], if possible.
    fn parent(&self, id: file::FileIdArg<'_>) -> BoxFuture<Result<file::FileId>>;
    
    /// Returns `true` if the given path (might be a file OR a directory!) exists.
    fn exists(&self, id: file::FileIdArg<'_>) -> BoxFuture<bool>;
    
    /// Returns `true` if the given path exists and is a file.
    fn is_file(&self, id: file::FileIdArg<'_>) -> BoxFuture<bool>;
    
    /// Returns `true` if the given path exists and is a directory.
    fn is_dir(&self, id: file::FileIdArg<'_>) -> BoxFuture<bool>;
    
    /// Does this filesystem have an index, thus allowing use of [`file::FileId::UniqueIdent`]?
    fn indexed(&self) -> bool;
    
    /// Is this filesystem mounted directly into the local computer?
    fn mounted(&self) -> bool;
    
    /// Does this filesystem support watching files for hot-reloading?
    fn watched(&self) -> bool;
    
    /// Returns a receiver for changes to files.
    fn watch(&self, id: file::FileIdArg<'_>) -> Result<channel::Receiver<file::FileId>>;
    
    // fn is_interactive(&self) -> Option<&dyn interactive::InteractiveFilesystem> {
    //     None
    // }
}
