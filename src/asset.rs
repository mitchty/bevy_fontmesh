use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::prelude::*;
use thiserror::Error;

/// Asset containing font data for 3D text mesh generation.
///
/// This asset type holds the raw bytes of a TrueType font file.
/// Fonts are automatically loaded from the asset server and used by the [`TextMesh`](crate::TextMesh)
/// component to generate 3D mesh geometry.
///
/// # Loading Fonts
///
/// Fonts are loaded like any other Bevy asset:
///
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_fontmesh::FontMesh;
/// # fn example(asset_server: Res<AssetServer>) {
/// let font: Handle<FontMesh> = asset_server.load("fonts/MyFont.ttf");
/// # }
/// ```
///
/// Place your font files in the `assets/fonts/` directory (or any subdirectory of `assets/`).
///
/// # Supported Formats
///
/// - TrueType (`.ttf`)
///
/// **Note**: Some OpenType fonts (`.otf`) with TrueType outlines are supported,
/// but OpenType fonts with CFF/PostScript outlines are not (limitation of ttf-parser).
#[derive(Asset, TypePath, Debug)]
pub struct FontMesh {
    /// Raw font file data in TTF or OTF format.
    pub data: Vec<u8>,
}

/// Asset loader for TrueType and OpenType font files.
///
/// This loader is registered automatically by [`FontMeshPlugin`](crate::FontMeshPlugin)
/// and handles `.ttf` and `.otf` file extensions.
#[derive(Default)]
pub struct FontMeshLoader;

/// Errors that can occur when loading font assets.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum FontMeshLoaderError {
    /// Failed to read the font file from disk.
    #[error("Could not load font file: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for FontMeshLoader {
    type Asset = FontMesh;
    type Settings = ();
    type Error = FontMeshLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data).await?;
        Ok(FontMesh { data })
    }

    fn extensions(&self) -> &[&str] {
        &["ttf", "otf"]
    }
}
