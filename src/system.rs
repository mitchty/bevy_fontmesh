use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::asset::render_asset::RenderAssetUsages;
use fontmesh::Font;
use tracing::warn; // Add this

use tracing::warn;
use crate::component::TextMesh;

#[derive(Component)]
pub struct TextMeshComputed;

pub fn update_text_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    font_assets: Res<Assets<FontMesh>>,
    mut query: Query<
        (Entity, &TextMesh, &mut Handle<Mesh>),
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
        // This is cheap (parsing tables), but ideally we'd cache this `Font` instance?
        // `fontmesh::Font` borrows the data. We have the data in `font_asset`.
        // So we can create it on the fly.
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
        let mut current_offset = 0.0;
        let mut index_offset = 0;

        for ch in text_mesh.text.chars() {
             // Handle whitespace roughly
            if ch.is_whitespace() {
                // Approximate space width (e.g., width of 'a' or just a fixed value)
                // fontmesh glyphs have advance width even for whitespace usually?
                if let Ok(glyph) = font.glyph_by_char(ch) {
                    current_offset += glyph.advance();
                } else {
                     // Fallback for space if not in font
                     current_offset += 0.3; 
                }
                continue;
            }

            // Generate glyph mesh
            // Use the configured quality and depth
            let mesh_res = font.glyph_by_char(ch)
                .and_then(|g| g.with_subdivisions(text_mesh.style.quality).to_mesh_3d(text_mesh.style.depth));

            match mesh_res {
                Ok(mesh) => {
                    // Append vertices with offset
                    for v in &mesh.vertices {
                        all_vertices.push([v.x + current_offset, v.y, v.z]);
                    }
                    
                    // Append normals
                    for n in &mesh.normals {
                        all_normals.push([n.x, n.y, n.z]);
                    }

                    // Append indices
                    for i in &mesh.indices {
                        all_indices.push(i + index_offset);
                    }

                    index_offset += mesh.vertices.len() as u32;
                    
                    // Advance cursor
                    // We need the advance width from the glyph
                    if let Ok(glyph) = font.glyph_by_char(ch) {
                         current_offset += glyph.advance();
                    }
                }
                Err(_) => {
                    // Skip missing chars
                    continue;
                }
            }
        }

        // 4. Create Bevy Mesh
        let mut new_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
        new_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, all_vertices);
        new_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, all_normals);
        // Add dummy UVs if needed by StandardMaterial (often required)
        // new_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; index_offset as usize]);
        
        new_mesh.insert_indices(Indices::U32(all_indices));

        // 5. Assign
        *mesh_handle = meshes.add(new_mesh);
        
        // 6. Mark as computed
        commands.entity(entity).insert(TextMeshComputed);
    }
}
