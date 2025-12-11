mod asset;
mod component;
mod system;

pub use asset::FontMesh;
pub use component::{TextMesh, TextMeshBundle, TextMeshStyle};

use bevy::prelude::*;
use asset::FontMeshLoader;
use system::{update_text_meshes, TextMeshComputed};

pub struct FontMeshPlugin;

impl Plugin for FontMeshPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<FontMesh>()
            .init_asset_loader::<FontMeshLoader>()
            .register_type::<TextMesh>()
            .add_systems(Update, update_text_meshes);
    }
}