

#[cfg(feature = "inventory")]
inventory::submit! {
    crate::factory::FilesystemDescriptor {
        extension: "lnk",
        asynchronous: false,
        construct: |_path| todo!("not yet implemented")
    }
}
