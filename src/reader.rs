//! Read-objects returned by [`crate::Filesystem`]s.

use super::{
    Result,
    IntoDiagnostic,
    memblob::MemoryMappedBlob
};

use compact_str::CompactString;

pub use std::sync::Arc;

pub use futures_lite::AsyncRead;
pub use futures_lite::AsyncBufRead;
pub use futures_lite::AsyncSeek;
pub use futures_lite::stream::Stream;

/// Boxed [`ReadBlob`], as returned by [`crate::Filesystem::read_blob`].
pub type ReadBlobObj = smallbox::SmallBox<dyn ReadBlob, smallbox::space::S4>;

/// Boxed [`ReadList`], as returned by [`crate::Filesystem::read_list`].
pub type ReadListObj = smallbox::SmallBox<dyn ReadList, smallbox::space::S4>;

/// A seekable reader.
pub trait AsyncSeekableReader: AsyncRead + AsyncSeek {
    //
}

/// Trait that let's the filesystem return the proper blob reader type.
pub trait ReadBlob {
    /// The path of the blob within it's source filesystem.
    fn src(&self) -> CompactString;
    
    /// The unique identifier of this blob, if available.
    fn uid(&self) -> Option<uuid::Uuid>;
    
    /// The length (in bytes) of the blob, if available.
    fn len(&self) -> Option<usize>;
    
    /// Returns a memory-mapped, or memory-resident, buffer.
    fn into_memory(self)
        -> Result<Box<dyn MemoryMappedBlob>>
        where Self: Sized;
    
    /// Read the entire blob, where&whatever it may be, into an owned vector.
    fn into_buffer(self)
        -> Result<Vec<u8>>
        where Self: Sized;
    
    /// Read the entire blob, where&whatever it may be, into an owned string.
    fn into_string(self)
        -> Result<CompactString>
        where Self: Sized
    {
        let vec = self.into_buffer()?;
        let str = std::string::String::from_utf8(vec).into_diagnostic()?;
        Ok(str.into())
    }
    
    /// Return a buffered reader over the blob, where&whatever it may be.
    fn into_reader(self)
        -> Result<smallbox::SmallBox<dyn AsyncBufRead, smallbox::space::S2>>
        where Self: Sized;
    
    /// Return a seekable reader over the blob, where&whatever it may be.
    fn into_cursor(self)
        -> Result<smallbox::SmallBox<dyn AsyncSeekableReader, smallbox::space::S2>>
        where Self: Sized;
}

/// Trait that let's the filesystem return the proper list reader type.
pub trait ReadList {
    /// Read the entire directory into a buffer.
    fn into_buffer(self, recursive: bool)
        -> Result<Vec<ListEntry>>
        where Self: Sized;
    
    /// Return a stream of directory entires.
    fn into_stream(self, recursive: bool)
        -> Result<smallbox::SmallBox<dyn Stream<Item = ListEntry>, smallbox::space::S2>>
        where Self: Sized;
}

/// Enum returned by [`ReadList`] methods.
pub enum ListEntry {
    /// A file.
    File(crate::file::StaticFileId),
    
    /// A directory.
    Dir(crate::file::StaticFileId),
}
