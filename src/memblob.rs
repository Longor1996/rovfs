//! Abstraction for direct memory access.
use super::{Result, IntoDiagnostic};

/// A blob that is mapped somewhere in memory, permitting direct memory access (DMA).
/// 
/// **Important Note:** Some filesystems don't support DMA; they read the entire blob into memory!
pub trait MemoryMappedBlob {
    // TODO: Would it make sense to make this interface asynchronous?
    // TODO: Direct Storage? https://github.com/microsoft/DirectStorage
    
    /// Resolve (deref) the mapped memory as a byte-slice.
    fn resolve(&self) -> &[u8];
    
    /// Resolve (deref) the mapped memory as a string-slice; fails if the string is invalid UTF-8.
    /// 
    /// **Warning:** This will scan the *entire* mapped memory to validate the string.
    fn resolve_str(&self) -> Result<&str> {
        let mem = self.resolve();
        let str = std::str::from_utf8(mem).into_diagnostic()?;
        Ok(str)
    }
}

// Default impl for Slice-based blobs.
impl MemoryMappedBlob for &[u8] {
    fn resolve(&self) -> &[u8] {
        self
    }
}

// Default impl for Arc'd blobs.
impl MemoryMappedBlob for std::sync::Arc<[u8]> {
    fn resolve(&self) -> &[u8] {
        self
    }
}

// Default impl for Vec-based blobs.
impl MemoryMappedBlob for Vec<u8> {
    fn resolve(&self) -> &[u8] {
        self
    }
}
