

#[cfg(feature = "inventory")]
inventory::submit! {
    crate::factory::FilesystemDescriptor {
        extension: "zip",
        asynchronous: false,
        construct: |_path| todo!("not yet implemented")
    }
}
