//! The custom wgpu viewport: one pipeline, one paint callback.
//!
//! This module is the only place in the crate that knows what a GPU
//! is, and it is deliberately thin — the "rerun shape" GQ6-RESURVEY §2
//! recommends: egui owns the surface, the swapchain and the render
//! pass; we contribute a pipeline and a draw call inside the pane's
//! viewport rectangle through `egui_wgpu`'s paint callbacks.
//!
//! # Depth
//!
//! egui's own pass carries the depth attachment. `eframe`'s
//! `NativeOptions::depth_buffer` reaches `egui_wgpu::RendererOptions`
//! as a `depth_stencil_format`, the winit painter then allocates the
//! texture and clears it to 1.0 at the start of the pass, and egui's
//! own pipeline is `depth_compare: Always, depth_write: false` — so UI
//! still paints over everything while a callback that declares the
//! same format gets real depth testing. Requesting the depth buffer at
//! startup is therefore load-bearing, not a preference; [`DEPTH_BITS`]
//! is the one place it is spelled.
//!
//! # Culling is off, on purpose
//!
//! The triangles are outward-wound (`mesh::FacePatch`'s contract) and
//! the shading uses that winding: the normal comes from the triangle's
//! own vertex order. Back-face *culling* is a second question — which
//! screen-space winding wgpu calls "front" — that this lane had no GPU
//! to settle, and getting it backwards makes a closed solid vanish
//! entirely. With a depth buffer and an opaque closed body, drawing
//! both sides is visually identical and cannot fail that way.
//!
//! **Scheduled, not merely noted: issue #1097** (viewer first light) owns
//! turning it on — set `cull_mode: Some(Face::Back)`, run it, and if the
//! solid vanishes the answer is `FrontFace::Cw` rather than `Ccw`. Whoever
//! does it replaces this section with which one it was; the reason it is
//! off today is ignorance, and ignorance recorded is a debt with an
//! owner.

use std::sync::Arc;

use eframe::wgpu;

use crate::scene::SceneMesh;

/// Bits of depth requested at startup. 32 maps to
/// `TextureFormat::Depth32Float` (`egui_wgpu::depth_format_from_bits`),
/// which needs no stencil aspect and is supported everywhere wgpu is.
pub(crate) const DEPTH_BITS: u8 = 32;

/// The depth format that pairs with [`DEPTH_BITS`]. Stated here so
/// the pipeline and the startup request cannot drift apart.
const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// Ambient term: how lit a surface facing away from the light is.
/// Enough that the unlit side reads as geometry rather than a hole.
const AMBIENT: f32 = 0.25;

/// Uniform block size in bytes: a 4×4 matrix, a light direction and a
/// base colour, each 16-byte aligned.
const UNIFORM_BYTES: u64 = 64 + 16 + 16;

/// The GPU-side state, held in `egui_wgpu`'s `callback_resources` for
/// the life of the render state.
pub(crate) struct ViewportRenderer {
    pipeline: wgpu::RenderPipeline,
    uniforms: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    geometry: Option<Geometry>,
}

struct Geometry {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_count: u32,
    /// Which scene these buffers hold. The app bumps it whenever it
    /// rebuilds the mesh; a mismatch here is the upload trigger.
    revision: u64,
}

impl ViewportRenderer {
    /// Build the pipeline. Called once, at application start, with the
    /// render state `eframe` hands the app.
    pub(crate) fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("viewer_scene_shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("viewer_scene_uniforms_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(UNIFORM_BYTES),
                },
                count: None,
            }],
        });
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("viewer_scene_uniforms"),
            size: UNIFORM_BYTES,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewer_scene_uniforms_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms.as_entire_binding(),
            }],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("viewer_scene_pipeline_layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("viewer_scene_pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[
                    Some(wgpu::VertexBufferLayout {
                        array_stride: 12,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                    }),
                    Some(wgpu::VertexBufferLayout {
                        array_stride: 12,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![1 => Float32x3],
                    }),
                ],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                // See the module docs: both sides are drawn.
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::COLOR,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        Self {
            pipeline,
            uniforms,
            bind_group,
            geometry: None,
        }
    }

    /// Upload `scene` if the buffers do not already hold `revision`.
    fn ensure_geometry(&mut self, device: &wgpu::Device, scene: &SceneMesh, revision: u64) {
        if self
            .geometry
            .as_ref()
            .is_some_and(|held| held.revision == revision)
        {
            return;
        }
        let positions = create_init_buffer(
            device,
            "viewer_scene_positions",
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(scene.positions()),
        );
        let normals = create_init_buffer(
            device,
            "viewer_scene_normals",
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(scene.normals()),
        );
        let indices = create_init_buffer(
            device,
            "viewer_scene_indices",
            wgpu::BufferUsages::INDEX,
            bytemuck::cast_slice(scene.indices()),
        );
        self.geometry = Some(Geometry {
            positions,
            normals,
            indices,
            index_count: u32::try_from(scene.indices().len()).unwrap_or(u32::MAX),
            revision,
        });
    }
}

/// Create a buffer and fill it, without `wgpu::util` (which would be
/// a second crate for one function).
fn create_init_buffer(
    device: &wgpu::Device,
    label: &str,
    usage: wgpu::BufferUsages,
    contents: &[u8],
) -> wgpu::Buffer {
    // `mapped_at_creation` requires a non-zero size that is a
    // multiple of wgpu's 4-byte copy alignment; an empty scene gets a
    // pad buffer and a zero-length draw rather than a device error.
    let size = ((contents.len() as u64).div_ceil(4) * 4).max(4);
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage,
        mapped_at_creation: true,
    });
    // A mapping failure on a buffer created `mapped_at_creation` is a
    // wgpu-side bug rather than an input-reachable state; the zeroed
    // buffer that results draws nothing, never garbage.
    if !contents.is_empty()
        && let Ok(mut mapped) = buffer.slice(..).get_mapped_range_mut()
    {
        mapped.slice(..contents.len()).copy_from_slice(contents);
    }
    buffer.unmap();
    buffer
}

/// One frame's worth of what the viewport needs: which scene, and
/// where the camera is.
pub(crate) struct ViewportCallback {
    /// The scene to draw. `Arc` because the callback outlives the
    /// `update` call that built it.
    pub(crate) scene: Arc<SceneMesh>,
    /// Which scene this is; the upload trigger.
    pub(crate) revision: u64,
    /// `projection · view`, column-major, already `f32`.
    pub(crate) view_projection: [[f32; 4]; 4],
    /// Unit vector the light travels along, world space.
    pub(crate) light_direction: [f32; 3],
    /// The body's base colour, linear RGB.
    pub(crate) base_color: [f32; 3],
}

impl ViewportCallback {
    /// The uniform block, laid out as three 16-byte rows: the matrix's
    /// four columns, the light direction, and the base colour with the
    /// ambient term in its fourth lane.
    ///
    /// **Built by concatenation, not by indexed writes.** The earlier
    /// shape wrote each scalar through `block.get_mut(i)` at indices
    /// that are statically in range — so an index error would have
    /// silently left a *zeroed* lane in the block, and a zeroed matrix
    /// row or colour is an unlit or invisible viewport with no error
    /// anywhere. Concatenation of fixed-size arrays cannot miss a
    /// lane, and the row structure is visible in the source instead of
    /// living in arithmetic.
    fn block(&self) -> [f32; 24] {
        let [c0, c1, c2, c3] = self.view_projection;
        let [lx, ly, lz] = self.light_direction;
        let [r, g, b] = self.base_color;
        let mut block = [0.0f32; 24];
        let (matrix, rest) = block.split_at_mut(16);
        matrix.copy_from_slice(&[c0, c1, c2, c3].concat());
        rest.copy_from_slice(&[lx, ly, lz, 0.0, r, g, b, AMBIENT]);
        block
    }
}

impl egui_wgpu::CallbackTrait for ViewportCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some(renderer) = resources.get_mut::<ViewportRenderer>() {
            renderer.ensure_geometry(device, &self.scene, self.revision);
            queue.write_buffer(&renderer.uniforms, 0, bytemuck::cast_slice(&self.block()));
        }
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        let Some(renderer) = resources.get::<ViewportRenderer>() else {
            return;
        };
        let Some(geometry) = renderer.geometry.as_ref() else {
            return;
        };
        // The pane's viewport rectangle is already set by egui's own
        // renderer before a callback is invoked, so the clip-space
        // mapping here is the pane's, not the window's.
        render_pass.set_pipeline(&renderer.pipeline);
        render_pass.set_bind_group(0, &renderer.bind_group, &[]);
        render_pass.set_vertex_buffer(0, geometry.positions.slice(..));
        render_pass.set_vertex_buffer(1, geometry.normals.slice(..));
        render_pass.set_index_buffer(geometry.indices.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..geometry.index_count, 0, 0..1);
    }
}

/// Flat-shaded Lambert with an ambient floor. The normal arrives per
/// vertex and is constant across a triangle (see `scene`'s
/// flat-shading note), so no interpolation smooths the facets away.
const SHADER: &str = r#"
struct Uniforms {
    view_projection: mat4x4<f32>,
    light_direction: vec4<f32>,
    base_color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
) -> VertexOut {
    var out: VertexOut;
    out.clip_position = uniforms.view_projection * vec4<f32>(position, 1.0);
    out.normal = normal;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let n = normalize(in.normal);
    let to_light = -uniforms.light_direction.xyz;
    let lambert = max(dot(n, to_light), 0.0);
    let ambient = uniforms.base_color.w;
    let shade = uniforms.base_color.xyz * (ambient + (1.0 - ambient) * lambert);
    return vec4<f32>(shade, 1.0);
}
"#;
