

#[cfg(feature = "inventory")]
inventory::submit! {
    crate::factory::FilesystemDescriptor {
        extension: "git",
        asynchronous: false,
        construct: |_path| todo!("not yet implemented")
    }
}
