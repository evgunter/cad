//! The custom wgpu viewport: the shaded pass, the id pass, and one
//! paint callback.
//!
//! This module is the only place in the crate that knows what a GPU
//! is, and it is deliberately thin — the "rerun shape" GQ6-RESURVEY §2
//! recommends: egui owns the surface, the swapchain and the render
//! pass; we contribute a pipeline and a draw call inside the pane's
//! viewport rectangle through `egui_wgpu`'s paint callbacks.
//!
//! # The id pass, and what it is for
//!
//! GQ6-RESURVEY §3's picking strategy is a GPU id buffer for
//! hover/click exactness beside a CPU ray cast. Both are here: the ray
//! cast is `crate::pick`, entirely headless and entirely tested, and
//! the id pass is [`ViewportRenderer::read_id_at`] below.
//!
//! **The id pass renders into a 1×1 target, not into a pane-sized
//! one.** The cursor names one pixel and one pixel is what a pick
//! reads back, so the pass pre-multiplies the view-projection by the
//! transform that maps that pixel's clip square onto the whole target
//! ([`cursor_projection`]). What that buys: no offscreen texture to
//! resize as the pane changes, no full-pane rasterization for one
//! sample, and a readback of exactly four bytes. What it costs: the
//! vertex stage still runs over the whole scene, because a pick has to
//! consider every triangle that could be under the cursor.
//!
//! Ids are the values `crate::pick::IdMap` assigns, and the target is
//! CLEARED to `IdMap::NOTHING` — so "the cursor is over nothing" is a
//! value the pass produces rather than a case the reader infers.
//!
//! # Nothing here has ever run
//!
//! Every line in this module compiles and lints, and none of it has
//! executed: the lanes that wrote it had no GPU and no display. Issue
//! #1097 owns first light, and its checklist carries the two questions
//! only hardware answers about the code below — whether the depth
//! attachment is really attached, and whether the id pass and the ray
//! path agree on the same cursor.
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
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::wgpu;

use crate::pick::{EdgeOverlay, Highlight, IdMap, cursor_projection};
use crate::scene::SceneMesh;
use crate::theme::{Mark, Theme};

/// Bits of depth requested at startup. 32 maps to
/// `TextureFormat::Depth32Float` (`egui_wgpu::depth_format_from_bits`),
/// which needs no stencil aspect and is supported everywhere wgpu is.
pub(crate) const DEPTH_BITS: u8 = 32;

/// The depth format that pairs with [`DEPTH_BITS`]. Stated here so
/// the pipeline and the startup request cannot drift apart.
const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// The id buffer's texel format: one unsigned 32-bit id per pixel,
/// which is what `crate::pick::IdMap` assigns. Not a colour format —
/// nothing blends, filters or gamma-corrects an identity.
const ID_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::R32Uint;

/// `copy_texture_to_buffer`'s row alignment. One `u32` is read back,
/// but the copy still pads its single row to this.
const COPY_ROW_ALIGNMENT: u64 = 256;

/// The uniform block both pipelines read.
///
/// `repr(C)` and a `Pod` derive rather than a hand-packed array: the
/// WGSL side declares the same four rows, and a struct that mirrors it
/// field for field cannot lose a lane the way indexed writes into a
/// flat block could.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    /// `projection · view`, column-major.
    view_projection: [[f32; 4]; 4],
    /// Unit vector the light travels along; the fourth lane is padding.
    light_direction: [f32; 4],
    /// Base colour in `xyz`, the ambient term in `w`.
    base_color: [f32; 4],
    /// `[selected id, hovered id, 0, 0]` — `IdMap::NOTHING` for
    /// "nothing is marked", so the shader needs no absence case.
    highlight: [u32; 4],
    /// The four highlight marks: tint in `xyz`, mix strength in `w`.
    ///
    /// **Uniform lanes, not WGSL `const`s, and that is the whole
    /// reason this block grew.** A theme is a value the user picks at
    /// runtime (`crate::theme`), and a colour baked into the shader
    /// source could only be changed by rebuilding the pipeline —
    /// which is to say by dropping and recreating every GPU resource
    /// behind the viewport to repaint the same triangles a different
    /// colour. Four `vec4`s cost 64 bytes once and make a theme
    /// switch a buffer write.
    ///
    /// The strength rides in `w` rather than in a block of its own
    /// because a tint and its strength are one decision — see
    /// [`Mark`] — and packing them together also leaves the block
    /// with no padding to state.
    selected: [f32; 4],
    /// The hovered patch's mark; see [`Uniforms::selected`].
    hovered: [f32; 4],
    /// The free-move probe's mark; see [`Uniforms::selected`].
    probe: [f32; 4],
    /// The focused feature's mark; see [`Uniforms::selected`].
    focus: [f32; 4],
}

/// One [`Mark`] as the uniform lane the shader reads: linear tint in
/// `xyz`, strength in `w`.
fn mark_lane(mark: Mark) -> [f32; 4] {
    let [r, g, b] = crate::theme::linear(mark.tint);
    [r, g, b, mark.strength]
}

/// Uniform block size in bytes.
const UNIFORM_BYTES: u64 = core::mem::size_of::<Uniforms>() as u64;

/// The GPU-side state, held in `egui_wgpu`'s `callback_resources` for
/// the life of the render state.
pub(crate) struct ViewportRenderer {
    pipeline: wgpu::RenderPipeline,
    uniforms: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    geometry: Option<Geometry>,
    /// The id pass and everything only it needs.
    id: IdPass,
    /// The edge-mark pass and everything only it needs.
    edges: EdgePass,
}

/// The id-buffer pass: a second pipeline over the same geometry, a
/// 1×1 target, and the four bytes read back from it.
struct IdPass {
    pipeline: wgpu::RenderPipeline,
    uniforms: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    /// The 1×1 id target and its depth companion. Allocated once —
    /// they never resize, which is the point of rendering one pixel.
    target: wgpu::Texture,
    depth: wgpu::Texture,
    /// The staging buffer the id is copied into.
    readback: wgpu::Buffer,
}

/// The edge-mark pass: the same uniforms and the same camera, drawing
/// the selected and hovered edges' polylines as lines over the solid.
///
/// **Marks that cannot be a tint.** A face mark is a patch the shaded
/// pass recognises by id; an edge has no patch, so its mark is
/// geometry — the drawn polyline, handed over as a line list by
/// `crate::pick::edge_overlay`. The colour is not a new palette entry:
/// it is the theme's OWN selected/hovered mark composited over the
/// same base the shaded pass composites over (`Mark::over`'s mix, run
/// on the same probe/focus-tinted body), drawn UNSHADED. The line is
/// therefore that composited colour at full strength where the surface
/// is the same colour times its shading term, and the palette's
/// colourblind claim keeps covering exactly the colours it already
/// covers.
///
/// **Where that separation vanishes**, stated because an earlier form
/// of this note claimed it never did: the shading term is
/// `ambient + (1 − ambient) · lambert`, which reaches exactly 1 on a
/// facet facing the light head-on. A mark drawn over such a facet is
/// the same pixel value as the facet's own mark, and there the line is
/// legible by its position and its neighbours' shading rather than by
/// its own value. Every other orientation separates them, and the
/// primary distinction was always form — a one-pixel line over a
/// filled patch — rather than value.
///
/// Lines are one pixel: wgpu's core specification has no line width,
/// and widening a mark means expanding each segment into a quad, which
/// is a second geometry pass this unit does not need.
struct EdgePass {
    pipeline: wgpu::RenderPipeline,
    /// The uploaded overlay, and the value it was built from — the
    /// upload trigger, compared rather than versioned because the
    /// overlay is small and is rebuilt (identically) every frame.
    held: Option<EdgeGeometry>,
}

/// The buffers one [`EdgeOverlay`] became.
struct EdgeGeometry {
    positions: wgpu::Buffer,
    marks: wgpu::Buffer,
    vertices: u32,
    overlay: EdgeOverlay,
}

/// The edge vertex's word, as BITS: which mark it is drawn in and what
/// its base is. Spelled once here and substituted into the WGSL, so
/// the two cannot drift.
///
/// The selected mark is the absence of [`EDGE_MARK_HOVERED`] rather
/// than a bit of its own — a vertex is drawn in exactly one mark, and
/// two bits would admit a state meaning both.
const EDGE_MARK_SELECTED: u32 = 0;
/// Set when this vertex is drawn in the HOVERED mark.
const EDGE_MARK_HOVERED: u32 = 1;
/// Set when this vertex's edge belongs to a free-moved instance, so
/// its mark composites over the probe-tinted body exactly as the
/// shaded pass's marks do (`EdgePass`'s note on the shared base).
const EDGE_FLAG_PROBE: u32 = 2;

/// The constant half of the edge pass's depth bias, in units of the
/// smallest resolvable depth increment at the fragment.
///
/// **Eyeballed, and there is no measurement behind it**: the lines lie
/// exactly on the surface (they share its positions), so any bias
/// toward the eye large enough to clear one depth quantum is enough,
/// and one large enough to lift a mark off a NEIGHBOURING surface
/// would be a bug. Two quanta is the smallest value that is not one.
/// The pass writes no depth, so an over-large bias could only make a
/// mark show through geometry it should not — which is the reason to
/// keep it minimal rather than to tune it.
const EDGE_DEPTH_BIAS: i32 = -2;

/// The slope-scaled half of the same bias: one depth quantum per unit
/// of depth slope across the fragment, which is what a line lying on a
/// steeply-angled facet needs and a flat-on one does not. Eyeballed
/// beside [`EDGE_DEPTH_BIAS`], for the same reason.
const EDGE_DEPTH_BIAS_SLOPE: f32 = -1.0;

struct Geometry {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    ids: wgpu::Buffer,
    /// Per-corner display flags (`SceneMesh::FLAG_PROBE`): the G3
    /// distinctness value, painted as the probe tint below.
    flags: wgpu::Buffer,
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
            source: wgpu::ShaderSource::Wgsl(shader_source().into()),
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
                    Some(wgpu::VertexBufferLayout {
                        array_stride: 4,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![2 => Uint32],
                    }),
                    Some(wgpu::VertexBufferLayout {
                        array_stride: 4,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![3 => Uint32],
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
        let id = IdPass::new(device, &module, &bind_group_layout, &layout);
        let edges = EdgePass::new(device, &module, &layout, target_format);
        Self {
            pipeline,
            uniforms,
            bind_group,
            geometry: None,
            id,
            edges,
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
        let ids = create_init_buffer(
            device,
            "viewer_scene_ids",
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(scene.ids()),
        );
        let flags = create_init_buffer(
            device,
            "viewer_scene_flags",
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(scene.flags()),
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
            ids,
            flags,
            indices,
            index_count: u32::try_from(scene.indices().len()).unwrap_or(u32::MAX),
            revision,
        });
    }

    /// Render the id pass for one cursor and read the id back.
    ///
    /// `cursor_ndc` is the cursor's position in normalized device
    /// coordinates within the viewport pane, and `viewport_px` its
    /// size in physical pixels — together they say which source pixel
    /// [`cursor_projection`] blows up to fill the 1×1 target.
    ///
    /// **A blocking readback, deliberately.** A pick is a question the
    /// user just asked and the answer is four bytes; an asynchronous
    /// path would buy a frame of latency back at the cost of a second
    /// state machine spanning frames, for a query that only runs when
    /// the cursor moves inside the viewport.
    ///
    /// `None` when there is nothing to draw, when the device refuses
    /// the wait, or when the mapping fails — every one of which is
    /// "the GPU has no answer", never a wrong answer.
    fn read_id_at(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        cursor_ndc: [f32; 2],
        viewport_px: [f32; 2],
        view_projection: &[[f32; 4]; 4],
    ) -> Option<u32> {
        let geometry = self.geometry.as_ref()?;
        if geometry.index_count == 0 {
            return None;
        }
        queue.write_buffer(
            &self.id.uniforms,
            0,
            bytemuck::bytes_of(&Uniforms {
                view_projection: cursor_projection(view_projection, cursor_ndc, viewport_px),
                // Every shading lane zeroed: `fs_id` returns an
                // identity and reads none of them.
                light_direction: [0.0; 4],
                base_color: [0.0; 4],
                highlight: [0; 4],
                selected: [0.0; 4],
                hovered: [0.0; 4],
                probe: [0.0; 4],
                focus: [0.0; 4],
            }),
        );
        let color_view = self
            .id
            .target
            .create_view(&wgpu::TextureViewDescriptor::default());
        let depth_view = self
            .id
            .depth
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("viewer_id_pass"),
        });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("viewer_id_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &color_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // Cleared to `IdMap::NOTHING`: a miss is a
                        // value the pass writes, not an inference.
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.id.pipeline);
            pass.set_bind_group(0, &self.id.bind_group, &[]);
            pass.set_vertex_buffer(0, geometry.positions.slice(..));
            pass.set_vertex_buffer(1, geometry.ids.slice(..));
            pass.set_index_buffer(geometry.indices.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..geometry.index_count, 0, 0..1);
        }
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &self.id.target,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &self.id.readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(COPY_ROW_ALIGNMENT as u32),
                    rows_per_image: Some(1),
                },
            },
            wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        queue.submit(std::iter::once(encoder.finish()));
        self.id
            .readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::PollType::wait_indefinitely()).ok()?;
        let id = {
            let view = self.id.readback.slice(..).get_mapped_range().ok()?;
            let bytes: [u8; 4] = view.get(..4)?.try_into().ok()?;
            u32::from_le_bytes(bytes)
        };
        self.id.readback.unmap();
        Some(id)
    }
}

impl IdPass {
    /// Build the id pipeline and its 1×1 targets.
    ///
    /// Shares the shaded pass's shader module, bind-group layout and
    /// pipeline layout: the two passes read the same uniform block and
    /// differ only in their entry points and their attachments.
    fn new(
        device: &wgpu::Device,
        module: &wgpu::ShaderModule,
        bind_group_layout: &wgpu::BindGroupLayout,
        layout: &wgpu::PipelineLayout,
    ) -> Self {
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("viewer_id_uniforms"),
            size: UNIFORM_BYTES,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewer_id_uniforms_bind_group"),
            layout: bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms.as_entire_binding(),
            }],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("viewer_id_pipeline"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module,
                entry_point: Some("vs_id"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[
                    Some(wgpu::VertexBufferLayout {
                        array_stride: 12,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                    }),
                    Some(wgpu::VertexBufferLayout {
                        array_stride: 4,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![2 => Uint32],
                    }),
                ],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                // Both sides, for the same reason the shaded pass draws
                // both: which screen winding is "front" is the question
                // #1097 settles on hardware, and an id pass that culled
                // the wrong way would answer NOTHING over a face that
                // is plainly there.
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
                module,
                entry_point: Some("fs_id"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: ID_FORMAT,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let extent = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };
        let target = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("viewer_id_target"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: ID_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let depth = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("viewer_id_depth"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("viewer_id_readback"),
            size: COPY_ROW_ALIGNMENT,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        Self {
            pipeline,
            uniforms,
            bind_group,
            target,
            depth,
            readback,
        }
    }
}

impl EdgePass {
    /// Build the line pipeline. Shares the shaded pass's shader module
    /// and pipeline layout — same uniforms, same camera, different
    /// topology and entry points.
    fn new(
        device: &wgpu::Device,
        module: &wgpu::ShaderModule,
        layout: &wgpu::PipelineLayout,
        target_format: wgpu::TextureFormat,
    ) -> Self {
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("viewer_edge_pipeline"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module,
                entry_point: Some("vs_edge"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[
                    Some(wgpu::VertexBufferLayout {
                        array_stride: 12,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                    }),
                    Some(wgpu::VertexBufferLayout {
                        array_stride: 4,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![1 => Uint32],
                    }),
                ],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                // Tested against the solid so a mark on the far side
                // of a body stays hidden, but never written: a mark is
                // not geometry, and a line that occluded the surface
                // it lies on would change what the picture says is
                // there.
                depth_write_enabled: Some(false),
                // The polyline's chord points ARE mesh positions the
                // triangles share, so the line lands exactly on the
                // surface's own depth: `LessEqual` plus the bias below
                // is what keeps it from z-fighting with the facet it
                // borders.
                depth_compare: Some(wgpu::CompareFunction::LessEqual),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState {
                    constant: EDGE_DEPTH_BIAS,
                    slope_scale: EDGE_DEPTH_BIAS_SLOPE,
                    clamp: 0.0,
                },
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module,
                entry_point: Some("fs_edge"),
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
            held: None,
        }
    }

    /// Upload `overlay` if the held buffers do not already hold it.
    ///
    /// Compared rather than versioned: the overlay is a handful of
    /// segments, recomputed identically every frame from state that
    /// lives in one place, and a revision counter beside it would be a
    /// second thing to keep true.
    fn ensure_geometry(&mut self, device: &wgpu::Device, overlay: &EdgeOverlay) {
        if self
            .held
            .as_ref()
            .is_some_and(|held| held.overlay == *overlay)
        {
            return;
        }
        if overlay.is_empty() {
            self.held = None;
            return;
        }
        let mut positions: Vec<[f32; 3]> =
            Vec::with_capacity(overlay.selected.len() + overlay.hovered.len());
        let mut marks: Vec<u32> = Vec::with_capacity(positions.capacity());
        for (mark, probed, corners) in [
            (
                EDGE_MARK_SELECTED,
                overlay.selected_probed,
                &overlay.selected,
            ),
            (EDGE_MARK_HOVERED, overlay.hovered_probed, &overlay.hovered),
        ] {
            let word = if probed { mark | EDGE_FLAG_PROBE } else { mark };
            positions.extend_from_slice(corners);
            marks.extend(std::iter::repeat_n(word, corners.len()));
        }
        let vertices = u32::try_from(positions.len()).unwrap_or(u32::MAX);
        self.held = Some(EdgeGeometry {
            positions: create_init_buffer(
                device,
                "viewer_edge_positions",
                wgpu::BufferUsages::VERTEX,
                bytemuck::cast_slice(&positions),
            ),
            marks: create_init_buffer(
                device,
                "viewer_edge_marks",
                wgpu::BufferUsages::VERTEX,
                bytemuck::cast_slice(&marks),
            ),
            vertices,
            overlay: overlay.clone(),
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

/// One frame's worth of what the viewport needs: which scene, where
/// the camera is, what is marked, and — when the frame asks one — the
/// id query.
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
    /// The palette this frame draws with. The whole value, because
    /// the body colour, the ambient term and the four marks are one
    /// decision and a pass that took them separately could be handed
    /// halves of two different themes.
    pub(crate) theme: Theme,
    /// Which patch ids to mark, from `crate::pick::highlight` — a
    /// value computed from (index, selection, hover) and handed
    /// straight through. **No highlight decision is taken here**; this
    /// pass paints what the pure function said.
    pub(crate) highlight: Highlight,
    /// Which edges to mark, from `crate::pick::edge_overlay` — the
    /// same shape of value as `highlight` and handed through the same
    /// way: **no marking decision is taken here**.
    pub(crate) edges: EdgeOverlay,
    /// The cursor to run the id pass at, in normalized device
    /// coordinates within the pane, with the pane's size in physical
    /// pixels. `None` on a frame that asks no id question, which is
    /// most of them.
    pub(crate) id_query: Option<IdQuery>,
}

/// One id-buffer question, and where its answer goes.
///
/// The answer travels back through a shared atomic rather than through
/// `egui_wgpu`'s resources, because the asker is the application and
/// the answerer is a paint callback: an `Arc` both hold is the whole
/// channel, with no borrow of the render state in the frame loop.
///
/// `serial` is echoed into the high half of [`IdQuery::answer`] so the
/// application can tell this frame's answer from the previous one's —
/// an id of `NOTHING` is a real answer, so "unchanged" cannot stand in
/// for "not yet run".
pub(crate) struct IdQuery {
    /// The cursor in normalized device coordinates within the pane.
    pub(crate) cursor_ndc: [f32; 2],
    /// The pane's size in physical pixels.
    pub(crate) viewport_px: [f32; 2],
    /// This query's serial.
    pub(crate) serial: u32,
    /// `serial << 32 | id` once the pass has run.
    pub(crate) answer: Arc<AtomicU64>,
}

impl ViewportCallback {
    /// The uniform block: the matrix, the light direction, the base
    /// colour with the ambient term in its fourth lane, the two
    /// highlight ids, and the theme's four marks.
    ///
    /// **A struct that mirrors the WGSL declaration, not a flat block
    /// written by index.** The earlier shape wrote each scalar through
    /// `block.get_mut(i)` at indices that are statically in range — so
    /// an index error would have silently left a *zeroed* lane, and a
    /// zeroed matrix row or colour is an unlit or invisible viewport
    /// with no error anywhere. Named fields cannot miss a lane.
    fn block(&self) -> Uniforms {
        let [lx, ly, lz] = self.light_direction;
        let [r, g, b] = crate::theme::linear(self.theme.body);
        Uniforms {
            view_projection: self.view_projection,
            light_direction: [lx, ly, lz, 0.0],
            base_color: [r, g, b, self.theme.ambient],
            highlight: [self.highlight.selected, self.highlight.hovered, 0, 0],
            selected: mark_lane(self.theme.selected),
            hovered: mark_lane(self.theme.hovered),
            probe: mark_lane(self.theme.probe),
            focus: mark_lane(self.theme.focus),
        }
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
            renderer.edges.ensure_geometry(device, &self.edges);
            queue.write_buffer(&renderer.uniforms, 0, bytemuck::bytes_of(&self.block()));
            // The id pass runs BEFORE the shaded pass and outside
            // egui's own encoder: it submits, waits and reads back, so
            // it cannot ride in the command buffer egui submits after
            // this call returns.
            if let Some(query) = &self.id_query {
                let id = renderer
                    .read_id_at(
                        device,
                        queue,
                        query.cursor_ndc,
                        query.viewport_px,
                        &self.view_projection,
                    )
                    .unwrap_or(IdMap::NOTHING);
                query.answer.store(
                    u64::from(query.serial) << 32 | u64::from(id),
                    Ordering::Relaxed,
                );
            }
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
        render_pass.set_vertex_buffer(2, geometry.ids.slice(..));
        render_pass.set_vertex_buffer(3, geometry.flags.slice(..));
        render_pass.set_index_buffer(geometry.indices.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..geometry.index_count, 0, 0..1);
        // The marks last, over the solid they lie on: depth-tested
        // against it, biased toward the eye, writing no depth.
        if let Some(edges) = renderer.edges.held.as_ref() {
            render_pass.set_pipeline(&renderer.edges.pipeline);
            render_pass.set_bind_group(0, &renderer.bind_group, &[]);
            render_pass.set_vertex_buffer(0, edges.positions.slice(..));
            render_pass.set_vertex_buffer(1, edges.marks.slice(..));
            render_pass.draw(0..edges.vertices, 0..1);
        }
    }
}

/// Flat-shaded Lambert with an ambient floor. The normal arrives per
/// vertex and is constant across a triangle (see `scene`'s
/// flat-shading note), so no interpolation smooths the facets away.
/// The WGSL, with the Rust-side constants substituted in — the flag
/// value crosses the string boundary exactly once, here, so the
/// shader cannot hold a second spelling of `SceneMesh::FLAG_PROBE`
/// that drifts from the one the scene writes into the vertex buffer.
/// (`IdMap::NOTHING` is still mirrored as `0u`/`!= 0u` in the source
/// below — pre-existing, and pinned by the fact that the clear value
/// is hardcoded 0 on both sides.)
fn shader_source() -> String {
    SHADER
        .replace(
            "{{FLAG_PROBE}}",
            &crate::scene::SceneMesh::FLAG_PROBE.to_string(),
        )
        .replace(
            "{{FLAG_FOCUS}}",
            &crate::scene::SceneMesh::FLAG_FOCUS.to_string(),
        )
        .replace("{{EDGE_MARK_HOVERED}}", &EDGE_MARK_HOVERED.to_string())
        .replace("{{EDGE_FLAG_PROBE}}", &EDGE_FLAG_PROBE.to_string())
}

const SHADER: &str = r#"
struct Uniforms {
    view_projection: mat4x4<f32>,
    light_direction: vec4<f32>,
    base_color: vec4<f32>,
    highlight: vec4<u32>,
    // Each mark: tint in xyz, mix strength in w. See the Rust
    // `Uniforms` for why these are lanes rather than consts.
    selected: vec4<f32>,
    hovered: vec4<f32>,
    probe: vec4<f32>,
    focus: vec4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    // Flat: every corner of a triangle carries its patch's id, so
    // interpolating one would only introduce a way for them to differ.
    @location(1) @interpolate(flat) id: u32,
    @location(2) @interpolate(flat) flag: u32,
};

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) id: u32,
    @location(3) flag: u32,
) -> VertexOut {
    var out: VertexOut;
    out.clip_position = uniforms.view_projection * vec4<f32>(position, 1.0);
    out.normal = normal;
    out.id = id;
    out.flag = flag;
    return out;
}

// Every mark mixes rather than replaces: a highlight that discarded
// the shading would flatten the facets a display-delta reading is
// there to show. WHAT each mark looks like is the theme's answer
// (`crate::theme`), delivered in the uniform lanes above; WHICH mark
// applies is this shader's, and the order below is that ruling.
//
// Selection wins over hover on the same patch, because it is the
// state the user committed to. The probe's flag
// (`SceneMesh::FLAG_PROBE`) is asserted headlessly and G3 requires
// only that a probed placement be distinguishable from a mated one —
// the strength that makes it so lives with the colour, in the theme.
fn tint(base: vec3<f32>, mark: vec4<f32>) -> vec3<f32> {
    return mix(base, mark.xyz, mark.w);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let n = normalize(in.normal);
    let to_light = -uniforms.light_direction.xyz;
    let lambert = max(dot(n, to_light), 0.0);
    let ambient = uniforms.base_color.w;
    var base = uniforms.base_color.xyz;
    if ((in.flag & {{FLAG_PROBE}}u) != 0u) {
        base = tint(base, uniforms.probe);
    }
    // Applied BEFORE the selection and hover tints so that the picked
    // patch of a focused feature still reads as the picked one: the
    // stronger mark lands on top of the weaker.
    if ((in.flag & {{FLAG_FOCUS}}u) != 0u) {
        base = tint(base, uniforms.focus);
    }
    if (in.id != 0u && in.id == uniforms.highlight.x) {
        base = tint(base, uniforms.selected);
    } else if (in.id != 0u && in.id == uniforms.highlight.y) {
        base = tint(base, uniforms.hovered);
    }
    let shade = base * (ambient + (1.0 - ambient) * lambert);
    return vec4<f32>(shade, 1.0);
}

// An edge mark: the theme's own selected/hovered mark composited over
// the body colour — `tint`, the same mix the shaded pass runs — and
// drawn UNSHADED, which is what keeps a marked edge distinguishable
// from the marked face it borders without a second palette entry.
struct EdgeOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(flat) mark: u32,
};

@vertex
fn vs_edge(
    @location(0) position: vec3<f32>,
    @location(1) mark: u32,
) -> EdgeOut {
    var out: EdgeOut;
    out.clip_position = uniforms.view_projection * vec4<f32>(position, 1.0);
    out.mark = mark;
    return out;
}

@fragment
fn fs_edge(in: EdgeOut) -> @location(0) vec4<f32> {
    // The SAME base the shaded pass composites its marks over: the
    // body colour, probe-tinted where the instance is free-moved.
    // Focus is not applied here and cannot be — it is a per-PATCH
    // marking with no edge equivalent — which costs the mark on an
    // edge of a focused feature the focus tint under it; that edge is
    // marked by the selection above it in every case where the two
    // would coincide.
    var base = uniforms.base_color.xyz;
    if ((in.mark & {{EDGE_FLAG_PROBE}}u) != 0u) {
        base = tint(base, uniforms.probe);
    }
    var color = tint(base, uniforms.selected);
    if ((in.mark & {{EDGE_MARK_HOVERED}}u) != 0u) {
        color = tint(base, uniforms.hovered);
    }
    return vec4<f32>(color, 1.0);
}

struct IdOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(flat) id: u32,
};

@vertex
fn vs_id(
    @location(0) position: vec3<f32>,
    @location(2) id: u32,
) -> IdOut {
    var out: IdOut;
    out.clip_position = uniforms.view_projection * vec4<f32>(position, 1.0);
    out.id = id;
    return out;
}

@fragment
fn fs_id(in: IdOut) -> @location(0) u32 {
    return in.id;
}
"#;
