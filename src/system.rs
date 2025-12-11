use crate::component::{JustifyText, TextAnchor, TextMesh};
use crate::FontMesh;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use fontmesh::Font;
use tracing::warn;

#[derive(Component)]
pub struct TextMeshComputed;

pub fn update_text_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    font_assets: Res<Assets<FontMesh>>,
    mut query: Query<
        (Entity, &TextMesh, &mut Mesh3d),
        Or<(Changed<TextMesh>, Without<TextMeshComputed>)>,
    >,
) {
    for (entity, text_mesh, mut mesh_handle) in query.iter_mut() {
        // 1. Try to get the font data
        let font_asset = match font_assets.get(&text_mesh.font) {
            Some(f) => f,
            None => {
                // Font not loaded yet, skip this frame
                continue;
            }
        };

        // 2. Load fontmesh
        let font = match Font::from_bytes(&font_asset.data) {
            Ok(f) => f,
            Err(e) => {
                warn!("Failed to parse font for entity {:?}: {:?}", entity, e);
                continue;
            }
        };

        // 3. Generate combined mesh
        let mut all_vertices = Vec::new();
        let mut all_normals = Vec::new();
        let mut all_indices = Vec::new();

        let mut cursor = Vec3::ZERO;
        let mut index_offset = 0;

        let line_height = font.ascender() - font.descender() + font.line_gap();

        // Bounds tracking
        let mut min_bound = Vec3::splat(f32::MAX);
        let mut max_bound = Vec3::splat(f32::MIN);

        // Split text into lines for justification
        for line in text_mesh.text.split('\n') {
            // Calculate line width first
            let mut line_width = 0.0;
            for ch in line.chars() {
                if let Ok(glyph) = font.glyph_by_char(ch) {
                    line_width += glyph.advance();
                } else if ch.is_whitespace() {
                    line_width += 0.3; // Fallback space
                }
            }

            // Determine X start offset based on justification
            let x_offset = match text_mesh.style.justify {
                JustifyText::Left => 0.0,
                JustifyText::Center => -line_width * 0.5,
                JustifyText::Right => -line_width,
            };

            cursor.x = x_offset;

            // Generate mesh for line
            for ch in line.chars() {
                if ch.is_whitespace() {
                    if let Ok(glyph) = font.glyph_by_char(ch) {
                        cursor.x += glyph.advance();
                    } else {
                        cursor.x += 0.3;
                    }
                    continue;
                }

                let mesh_res = font.glyph_by_char(ch).and_then(|g| {
                    g.with_subdivisions(text_mesh.style.subdivision)
                        .to_mesh_3d(text_mesh.style.depth)
                });

                match mesh_res {
                    Ok(mesh) => {
                        for v in &mesh.vertices {
                            let pos = Vec3::new(v.x + cursor.x, v.y + cursor.y, v.z);
                            all_vertices.push([pos.x, pos.y, pos.z]);

                            min_bound = min_bound.min(pos);
                            max_bound = max_bound.max(pos);
                        }

                        for n in &mesh.normals {
                            all_normals.push([n.x, n.y, n.z]);
                        }

                        for i in &mesh.indices {
                            all_indices.push(i + index_offset);
                        }

                        index_offset += mesh.vertices.len() as u32;

                        if let Ok(glyph) = font.glyph_by_char(ch) {
                            cursor.x += glyph.advance();
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }

            // Move to next line
            cursor.y -= line_height;
        }

        // 4. Apply Anchor Offset
        if !all_vertices.is_empty() {
            let size = max_bound - min_bound;
            let center = min_bound + size * 0.5;

            let offset = match text_mesh.style.anchor {
                TextAnchor::TopLeft => Vec3::new(-min_bound.x, -max_bound.y, 0.0),
                TextAnchor::TopCenter => Vec3::new(-center.x, -max_bound.y, 0.0),
                TextAnchor::TopRight => Vec3::new(-max_bound.x, -max_bound.y, 0.0),

                TextAnchor::CenterLeft => Vec3::new(-min_bound.x, -center.y, 0.0),
                TextAnchor::Center => Vec3::new(-center.x, -center.y, 0.0),
                TextAnchor::CenterRight => Vec3::new(-max_bound.x, -center.y, 0.0),

                TextAnchor::BottomLeft => Vec3::new(-min_bound.x, -min_bound.y, 0.0),
                TextAnchor::BottomCenter => Vec3::new(-center.x, -min_bound.y, 0.0),
                TextAnchor::BottomRight => Vec3::new(-max_bound.x, -min_bound.y, 0.0),

                TextAnchor::Custom(pivot) => {
                    let pivot_pos = min_bound.truncate() + size.truncate() * pivot;
                    Vec3::new(-pivot_pos.x, -pivot_pos.y, 0.0)
                }
            };

            for v in &mut all_vertices {
                v[0] += offset.x;
                v[1] += offset.y;
                v[2] += offset.z;
            }
        }

        // 5. Create Bevy Mesh
        let mut new_mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        new_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, all_vertices);
        new_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, all_normals);

        new_mesh.insert_indices(Indices::U32(all_indices));

        // 6. Assign
        mesh_handle.0 = meshes.add(new_mesh);

        // 7. Mark as computed
        commands.entity(entity).insert(TextMeshComputed);
    }
}
