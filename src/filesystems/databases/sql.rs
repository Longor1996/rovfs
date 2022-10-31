

#[cfg(feature = "inventory")]
inventory::submit! {
    crate::factory::FilesystemDescriptor {
        extension: "db",
        asynchronous: false,
        construct: |_path| todo!("not yet implemented")
    }
}
