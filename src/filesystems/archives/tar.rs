

#[cfg(feature = "inventory")]
inventory::submit! {
    crate::factory::FilesystemDescriptor {
        extension: "tar",
        asynchronous: false,
        construct: |_path| todo!("not yet implemented")
    }
}
