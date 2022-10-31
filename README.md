# Virtual Filesystem Implementation

This is an implementation of a *read-only* **virtual filesystem** with several backends.

The primary types to look at:
- [`file::FileId`]
- [`filesystem::Filesystem`]
- [`reader::ReadBlob`]

To construct a filesystem, look at the individual [`filesystems`].
