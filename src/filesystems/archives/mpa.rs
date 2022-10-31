

#[cfg(feature = "inventory")]
inventory::submit! {
    crate::factory::FilesystemDescriptor {
        extension: "mpa",
        asynchronous: false,
        construct: |_path| todo!("not yet implemented")
    }
}
