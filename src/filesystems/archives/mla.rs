

#[cfg(feature = "inventory")]
inventory::submit! {
    crate::factory::FilesystemDescriptor {
        extension: "mla",
        asynchronous: false,
        construct: |_path| todo!("not yet implemented")
    }
}
