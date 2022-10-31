

#[cfg(feature = "inventory")]
inventory::submit! {
    crate::factory::FilesystemDescriptor {
        extension: "url",
        asynchronous: false,
        construct: |_path| todo!("not yet implemented")
    }
}
