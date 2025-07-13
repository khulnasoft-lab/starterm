//! Provides access to assets embedded in the binary using the `embed_assets!` macro.
// In a real project, you would add `starterm-macros` to dependencies.

// use starterm_macros::embed_assets;

/// A static reference to all embedded assets.
///
/// The `embed_assets!` macro would be called here to embed a directory
/// (e.g., "assets/") into the binary at compile time.
// static ALL_ASSETS: &include_dir::Dir<'_> = embed_assets!("path/to/assets");

pub struct Assets;

impl Assets {
    /// Retrieves a file from the embedded assets by its path.
    pub fn get(path: &str) -> Option<Vec<u8>> {
        // TODO: Replace this placeholder with the actual asset lookup.
        // `ALL_ASSETS.get_file(path).map(|f| f.contents().to_vec())`
        if path == "default_config.toml" {
            Some(b"[window]\ntitle = \"Starterm\"\n".to_vec())
        } else {
            None
        }
    }
} 