use bevy_core_pipeline::prepass::ViewPrepassTextures;
use bevy_render::render_resource::{
    binding_types::{
        texture_2d, texture_2d_multisampled, texture_depth_2d, texture_depth_2d_multisampled,
    },
    BindGroupLayoutEntryBuilder, TextureSampleType, TextureView,
};

use crate::MeshPipelineViewLayoutKey;

/// The bindings of the mesh view bind group that hold the prepass textures
/// a shader of that bind group reads: the depth, the normals and the
/// deferred texture, in this order. The motion vectors have no binding:
/// no shader of the mesh view bind group reads them, and each binding
/// counts against `Limits::max_sampled_textures_per_shader_stage`, of
/// which WebGPU guarantees 16 to a device.
pub const PREPASS_BINDINGS: [u32; 3] = [20, 21, 23];

pub fn get_bind_group_layout_entries(
    layout_key: MeshPipelineViewLayoutKey,
) -> [Option<BindGroupLayoutEntryBuilder>; 3] {
    let mut entries: [Option<BindGroupLayoutEntryBuilder>; 3] = [None; 3];

    let multisampled = layout_key.contains(MeshPipelineViewLayoutKey::MULTISAMPLED);

    if layout_key.contains(MeshPipelineViewLayoutKey::DEPTH_PREPASS) {
        // Depth texture
        entries[0] = if multisampled {
            Some(texture_depth_2d_multisampled())
        } else {
            Some(texture_depth_2d())
        };
    }

    if layout_key.contains(MeshPipelineViewLayoutKey::NORMAL_PREPASS) {
        // Normal texture
        entries[1] = if multisampled {
            Some(texture_2d_multisampled(TextureSampleType::Float {
                filterable: false,
            }))
        } else {
            Some(texture_2d(TextureSampleType::Float { filterable: false }))
        };
    }

    if layout_key.contains(MeshPipelineViewLayoutKey::DEFERRED_PREPASS) {
        // Deferred texture
        entries[2] = Some(texture_2d(TextureSampleType::Uint));
    }

    entries
}

pub fn get_bindings(prepass_textures: Option<&ViewPrepassTextures>) -> [Option<TextureView>; 3] {
    [
        prepass_textures.and_then(|pt| pt.depth_only_view().cloned()),
        prepass_textures.and_then(|pt| pt.normal_view().cloned()),
        prepass_textures.and_then(|pt| pt.deferred_view().cloned()),
    ]
}
