use crate::asset::FontMesh;
use bevy::prelude::*;

/// Alignment of the text mesh relative to its transform origin
#[derive(Reflect, Clone, Copy, Debug, Default, PartialEq)]
pub enum TextAnchor {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    /// Custom anchor point (0.0-1.0), where (0,0) is BottomLeft and (1,1) is TopRight
    Custom(Vec2),
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TextMesh {
    pub text: String,
    pub font: Handle<FontMesh>,
    pub style: TextMeshStyle,
}

/// Text justification (alignment of lines relative to each other)
#[derive(Reflect, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum JustifyText {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Reflect, Clone, Debug)]
pub struct TextMeshStyle {
    /// Depth of the extrusion in font units (usually relative to 1.0 em height)
    pub depth: f32,
    /// Curve subdivision quality
    pub subdivision: u8,
    /// Alignment of the mesh relative to the Transform and text justification
    pub anchor: TextAnchor,
    /// Text justification (multiline alignment)
    pub justify: JustifyText,
}

impl Default for TextMeshStyle {
    fn default() -> Self {
        Self {
            depth: 0.1,
            subdivision: 20, // Default low poly-ish but smooth enough
            anchor: TextAnchor::TopLeft,
            justify: JustifyText::Left,
        }
    }
}

/// Bundle for easy spawning
#[derive(Bundle, Default)]
pub struct TextMeshBundle {
    pub text_mesh: TextMesh,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}
