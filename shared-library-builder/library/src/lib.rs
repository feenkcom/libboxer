use shared_library_builder::{GitLocation, LibraryLocation, RustLibrary};

pub fn libboxer(version: Option<impl Into<String>>) -> RustLibrary {
    RustLibrary::new(
        "Boxer",
        LibraryLocation::Git(GitLocation::github("feenkcom", "libboxer").tag_or_latest(version)),
    )
    .package("libboxer")
    .feature("phlow")
}

pub fn latest_libboxer() -> RustLibrary {
    let version: Option<String> = None;
    libboxer(version)
}
