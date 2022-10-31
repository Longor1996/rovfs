

#[cfg(feature = "inventory")]
inventory::submit! {
    crate::factory::FilesystemDescriptor {
        extension: "fat",
        asynchronous: false,
        construct: |_path| todo!("not yet implemented")
    }
}
