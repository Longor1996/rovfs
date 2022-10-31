//! File extension handling.

use std::borrow::Cow;

/// A [`FileExt`] in a [`Cow`].
pub type FileExtCow<'f> = Cow<'f, FileExt>;

/// File extension handler.
#[derive(Debug, Clone)]
pub struct FileExt {
    /// Valid extensions.
    pub extensions: smallvec::SmallVec<[compact_str::CompactString; 1]>
}

impl FileExt {
    /// Creates a new [`FileExt`] from a [`&str`]; if possible at compile-time.
    pub const fn new(ext: &str) -> Self {
        Self {
            extensions: smallvec::smallvec_inline![
                compact_str::CompactString::new_inline(ext)
            ]
        }
    }
    
    /// Creates a [`FileExt`] from a set of strings.
    pub fn new_set(exts: &[&str]) -> Self {
        let mut extensions = smallvec::SmallVec::with_capacity(exts.len());
        extensions.extend(exts.iter().map(|ext| compact_str::CompactString::new(ext)));
        Self {
            extensions
        }
    }
}

#[test]
fn fileext_size() {
    let fileext = FileExt::new("txt");
    eprintln!("Size of a FileExt: {} bytes", std::mem::size_of_val(&fileext));
}
