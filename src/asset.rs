use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::prelude::*;
use thiserror::Error;

/// Asset that holds TrueType font data for use with fontmesh
#[derive(Asset, TypePath, Debug)]
pub struct FontMesh {
    pub data: Vec<u8>,
}

#[derive(Default)]
pub struct FontMeshLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum FontMeshLoaderError {
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
