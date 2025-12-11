use bevy::prelude::*;
use crate::asset::FontMesh;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TextMesh {
    pub text: String,
    pub font: Handle<FontMesh>,
    pub style: TextMeshStyle,
}

#[derive(Reflect, Clone, Debug)]
pub struct TextMeshStyle {
    pub depth: f32,
    pub quality: u8, // Subdivisions
    pub color: Color,
    // Add more alignment/spacing options here later if needed
}

impl Default for TextMeshStyle {
    fn default() -> Self {
        Self {
            depth: 0.1,
            quality: 20,
            color: Color::WHITE,
        }
    }
}

/// Bundle for easy spawning
#[derive(Bundle, Default)]
pub struct TextMeshBundle {
    pub text_mesh: TextMesh,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}
