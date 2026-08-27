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

use crate::pick::{Highlight, cursor_projection};
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

struct Geometry {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    ids: wgpu::Buffer,
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
        Self {
            pipeline,
            uniforms,
            bind_group,
            geometry: None,
            id,
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
                light_direction: [0.0; 4],
                base_color: [0.0; 4],
                highlight: [0; 4],
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
    /// The body's base colour, linear RGB.
    pub(crate) base_color: [f32; 3],
    /// Which patch ids to mark, from `crate::pick::highlight` — a
    /// value computed from (index, selection, hover) and handed
    /// straight through. **No highlight decision is taken here**; this
    /// pass paints what the pure function said.
    pub(crate) highlight: Highlight,
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
    /// colour with the ambient term in its fourth lane, and the two
    /// highlight ids.
    ///
    /// **A struct that mirrors the WGSL declaration, not a flat block
    /// written by index.** The earlier shape wrote each scalar through
    /// `block.get_mut(i)` at indices that are statically in range — so
    /// an index error would have silently left a *zeroed* lane, and a
    /// zeroed matrix row or colour is an unlit or invisible viewport
    /// with no error anywhere. Named fields cannot miss a lane.
    fn block(&self) -> Uniforms {
        let [lx, ly, lz] = self.light_direction;
        let [r, g, b] = self.base_color;
        Uniforms {
            view_projection: self.view_projection,
            light_direction: [lx, ly, lz, 0.0],
            base_color: [r, g, b, AMBIENT],
            highlight: [self.highlight.selected, self.highlight.hovered, 0, 0],
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
                    .unwrap_or(0);
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
    highlight: vec4<u32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    // Flat: every corner of a triangle carries its patch's id, so
    // interpolating one would only introduce a way for them to differ.
    @location(1) @interpolate(flat) id: u32,
};

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) id: u32,
) -> VertexOut {
    var out: VertexOut;
    out.clip_position = uniforms.view_projection * vec4<f32>(position, 1.0);
    out.normal = normal;
    out.id = id;
    return out;
}

// The selected patch is tinted toward one colour and the hovered patch
// toward another, both by mixing rather than replacing: a highlight
// that discarded the shading would flatten the facets a display-delta
// reading is there to show. Selection wins over hover on the same
// patch, because it is the state the user committed to.
const SELECTED_TINT: vec3<f32> = vec3<f32>(1.0, 0.62, 0.16);
const HOVERED_TINT: vec3<f32> = vec3<f32>(0.45, 0.72, 1.0);
const TINT_STRENGTH: f32 = 0.55;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let n = normalize(in.normal);
    let to_light = -uniforms.light_direction.xyz;
    let lambert = max(dot(n, to_light), 0.0);
    let ambient = uniforms.base_color.w;
    var base = uniforms.base_color.xyz;
    if (in.id != 0u && in.id == uniforms.highlight.x) {
        base = mix(base, SELECTED_TINT, TINT_STRENGTH);
    } else if (in.id != 0u && in.id == uniforms.highlight.y) {
        base = mix(base, HOVERED_TINT, TINT_STRENGTH);
    }
    let shade = base * (ambient + (1.0 - ambient) * lambert);
    return vec4<f32>(shade, 1.0);
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
