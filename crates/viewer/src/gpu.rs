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
//! cast is `crate::pickindex`, entirely headless and entirely tested, and
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
//! Ids are the values `crate::pickindex::IdMap` assigns, and the target is
//! CLEARED to `IdMap::NOTHING` — so "the cursor is over nothing" is a
//! value the pass produces rather than a case the reader infers.
//!
//! # Construction runs; drawing does not
//!
//! [`ViewportRenderer::new`] and everything it builds execute on a real
//! device under `--features app`, so a pipeline this module cannot
//! build is a red row. Nothing below that seam does: no surface, no
//! frame, no readback, no pixel. The questions only a drawn frame
//! answers — whether the depth attachment is really attached, and
//! whether the id pass and the ray path agree on the same cursor —
//! are open, and issue #1097 owns them.
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
//! **Depth is reversed** (`Camera::projection_matrix`): 1 is the near
//! plane, 0 is infinitely far, and every pipeline here compares
//! `Greater` (`GreaterEqual` for the marks). The painter's clear value
//! is not configurable and 1.0 is the wrong one for that, so the
//! callback's first draw is a viewport-covering triangle that writes 0
//! with `Always` and no colour (`ViewportRenderer::depth_reset`). It is
//! bounded by the pane's viewport and scissor like every other draw in
//! the callback, so it resets this pane's depth and nothing else. The
//! id pass owns its own depth texture and clears it to 0 directly.
//!
//! # Culling is off, on purpose
//!
//! The triangles are outward-wound (`mesh::FacePatch`'s contract) and
//! the shading uses that winding: the normal comes from the triangle's
//! own vertex order. Back-face *culling* was a second question — which
//! screen-space winding wgpu calls "front" — and it is **settled: `Ccw`**.
//! An outward-wound triangle presents counter-clockwise on screen, so
//! `front_face: FrontFace::Ccw` with `cull_mode: Some(Face::Back)` keeps a
//! closed solid whole. Read on hardware for issue #1097, on a D3D12 device
//! (Intel Iris Plus): the plate stayed complete through a full orbit, bore
//! included, with neither the total vanish that `Cw` would have produced
//! nor any partial loss of patches. The answer is backend-independent —
//! wgpu normalizes `FrontFace` — so it is stated here as a fact and not as
//! one driver's opinion.
//!
//! **The edge pass keeps `cull_mode: None`, deliberately.** An edge mark is
//! a camera-facing quad, not part of a closed solid, so back-face culling
//! is a question about billboards rather than about winding, and the
//! reading above does not answer it. It stays open on its own terms.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use eframe::wgpu;

use crate::camera::cursor_projection;
use crate::marks::{EdgeLane, EdgeOverlay, Highlight};
use crate::pickindex::IdMap;
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
/// which is what `crate::pickindex::IdMap` assigns. Not a colour format —
/// nothing blends, filters or gamma-corrects an identity.
const ID_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::R32Uint;

/// `copy_texture_to_buffer`'s row alignment. One `u32` is read back,
/// but the copy still pads its single row to this.
const COPY_ROW_ALIGNMENT: u64 = 256;

/// The uniform block both pipelines read.
///
/// `repr(C)` and a `Pod` derive rather than a hand-packed array: the
/// WGSL side declares the same fields in the same order, and a struct
/// that mirrors it field for field cannot lose a lane the way indexed writes into a
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
    ///
    /// **The two lanes may hold the SAME id**, when the hover is on
    /// the selection ([`crate::marks::Highlight`]); `fs_main` below is
    /// what rules between them, and it is the only thing that does.
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
    /// **Construction geometry's colour**, linear in `xyz`; `w` is
    /// padding and the shader does not read it.
    ///
    /// A whole lane for three numbers, and the padding is the honest
    /// cost of the block staying a struct whose fields the shader
    /// mirrors one for one. `Theme::datum` is a colour and not a
    /// [`Mark`], so there is no strength to put in `w`: a datum is
    /// drawn in this colour, not tinted toward it.
    datum: [f32; 4],
    /// **A committed profile's colour**, linear in `xyz`; `w` is
    /// padding. [`Uniforms::datum`]'s shape for its reason:
    /// `Theme::profile` is a line colour, drawn as stated.
    profile: [f32; 4],
    /// **What the edge pass needs to measure the screen**: the
    /// viewport's size in physical pixels (`xy`); `zw` are padding.
    ///
    /// A uniform lane rather than a shader constant for the reason
    /// the marks are lanes: the pane is resized by dragging a
    /// divider, and a pipeline rebuilt to repaint the same triangles
    /// at a different window size would be an odd way to spend a
    /// frame. The shaded and id passes read none of it.
    edge: [f32; 4],
    /// **Each edge lane's style**, indexed by the lane's code
    /// ([`lane_code`]): half the line's width in physical pixels in
    /// `x`, its opacity in `y`; `zw` are padding.
    ///
    /// Per lane because the lanes are not equally loud on purpose
    /// ([`lane_style`]), and in physical pixels because the width is
    /// stated in points and the device pixel ratio is a per-frame
    /// fact, exactly as the viewport size is.
    edge_lanes: [[f32; 4]; EdgeLane::DRAW_ORDER.len()],
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
    /// Writes depth 0 over the pane before anything is drawn: see the
    /// module docs' Depth section for why the pass's own clear is not
    /// the one this renderer needs.
    depth_reset: wgpu::RenderPipeline,
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
/// `crate::marks::edge_overlay`. The colour is not a new palette entry:
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
/// **The marks are QUADS, not lines.** wgpu's core specification has
/// no line width, so a `LineList` mark is one physical pixel wide —
/// which on a hidpi screen is half a point, and which lands on a
/// marked edge only where the rasterizer's diamond-exit rule says it
/// does. The result reads as a dotted mark rather than a thin one:
/// segments a few pixels long, seen nearly end-on, drop most of their
/// pixels. So each segment is expanded into a screen-space quad here
/// — the CPU emits six vertices carrying BOTH endpoints, and
/// `vs_edge` offsets each corner along the segment's screen normal by
/// its lane's half width ([`lane_style`]). The width is in POINTS, so
/// a mark is the same thickness to the eye at any device pixel ratio.
///
/// The expansion is per segment and deliberately does not join them:
/// a polyline's corners are left as two overlapping quads rather than
/// mitred. On an opaque lane the overlap is invisible, and a mitre
/// needs the neighbouring segment's direction — which is a different
/// vertex format and a real amount of arithmetic for a join nobody
/// can see. On the one translucent lane, the datum grid, the overlap
/// is blended twice and shows as a pixel or two of fuller colour
/// where two of its lines meet — which on a grid is where the eye
/// expects a crossing to be anyway.
///
/// **Lanes are drawn in [`EdgeLane::DRAW_ORDER`]**, one buffer in that
/// order ([`edge_vertices`]), and the pass writes no depth, so where
/// two lanes cover a pixel the later lane is on top. Within one draw
/// the output is merged in primitive order, which is what makes buffer
/// order a draw order at all.
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

/// **An edge vertex's lane, as the code the shader reads**: the lane's
/// discriminant, which is its position in [`EdgeLane::DRAW_ORDER`]
/// because `vocabulary!` projects that list from the declaration in
/// declaration order (`every_lane_code_is_its_draw_position` holds it).
///
/// One numbering, so the code a vertex carries, the row of
/// [`Uniforms::edge_lanes`] it reads its style from, the arm of the
/// shader's colour switch it takes ([`lane_colour_switch`]) and the
/// order the lanes are drawn in are one list.
fn lane_code(lane: EdgeLane) -> u32 {
    lane as u32
}

/// The mask over an edge vertex's word that holds its [`lane_code`]:
/// the smallest all-ones mask every lane's code fits, so a lane added
/// to `EdgeLane` widens it rather than overflowing into the flag above.
const EDGE_LANE_MASK: u32 = (EdgeLane::DRAW_ORDER.len() as u32).next_power_of_two() - 1;
/// Set when this vertex's edge belongs to a free-moved instance, so
/// its mark composites over the probe-tinted body exactly as the
/// shaded pass's marks do (`EdgePass`'s note on the shared base).
/// The first bit above [`EDGE_LANE_MASK`], because it says something
/// orthogonal to which lane the vertex is in.
const EDGE_FLAG_PROBE: u32 = EDGE_LANE_MASK + 1;

/// **The WGSL expression one lane's colour is**, over `base` — the body
/// colour, probe-tinted where the instance is free-moved.
///
/// An exhaustive match, so a lane added to `EdgeLane` does not compile
/// until it is given a colour; [`lane_colour_switch`] writes one arm
/// per lane from it.
fn lane_colour_wgsl(lane: EdgeLane) -> &'static str {
    match lane {
        // The picked marks: the theme's own mark, composited over the
        // base — `tint`, the same mix the shaded pass runs.
        EdgeLane::Selected => "tint(base, uniforms.selected)",
        EdgeLane::Hovered => "tint(base, uniforms.hovered)",
        // A preview is not in the document, and says so in the probe
        // mark — G3's "not committed" — over the same base.
        EdgeLane::Preview => "tint(base, uniforms.probe)",
        // NOT tints: a datum and a profile are not material, so there
        // is no body colour for either to be a state of. Each is drawn
        // in the theme's own colour for it, as stated.
        EdgeLane::Datum => "uniforms.datum.xyz",
        EdgeLane::Profile => "uniforms.profile.xyz",
    }
}

/// **`fs_edge`'s lane → colour switch, generated from the lanes**: one
/// `case` per lane in [`EdgeLane::DRAW_ORDER`], its code the lane's
/// [`lane_code`], its colour [`lane_colour_wgsl`].
///
/// WGSL requires a `default`, and no code reaches it — every code is
/// written by [`edge_vertices`] from a lane. It is drawn in
/// [`UNHANDLED_LANE_WGSL`], pure magenta, so a word that did reach it
/// would be the loudest thing on screen rather than a line quietly
/// wearing some other lane's colour.
fn lane_colour_switch() -> String {
    let mut out = String::from("switch lane {\n");
    for lane in EdgeLane::DRAW_ORDER {
        out.push_str(&format!(
            "        case {}u: {{ color = {}; }}\n",
            lane_code(lane),
            lane_colour_wgsl(lane)
        ));
    }
    out.push_str(&format!(
        "        default: {{ color = {UNHANDLED_LANE_WGSL}; }}\n    }}"
    ));
    out
}

/// The colour a lane code with no lane is drawn in; see
/// [`lane_colour_switch`].
const UNHANDLED_LANE_WGSL: &str = "vec3<f32>(1.0, 0.0, 1.0)";

/// **How loud one edge lane is drawn**: its width and its opacity.
#[derive(Clone, Copy, Debug, PartialEq)]
struct LaneStyle {
    /// Half the line's width, in POINTS — so a line is the same
    /// thickness to the eye on a hidpi screen as on a 1× one
    /// (`vs_edge` is handed the physical half-width, scaled by the
    /// frame's device pixel ratio).
    half_width_points: f32,
    /// How much of the line's colour covers what is under it, in
    /// `(0, 1]`: the pass blends `color · opacity + under · (1 −
    /// opacity)`.
    opacity: f32,
}

/// **Half the width of a mark, a profile or a preview, in POINTS** —
/// three points thick wherever it is drawn.
///
/// A judgement, and the range it sits in is narrow at both ends: much
/// above four points a mark's own width hides the short edges it is
/// marking, and a line has to be clearly heavier than the datum grid
/// ([`DATUM_HALF_WIDTH_POINTS`]) to read as the thing in front of it.
/// Three points is also roughly the weight of the chrome's own text,
/// which is what makes a mark findable at a glance without becoming the
/// loudest thing in the picture.
const EDGE_MARK_HALF_WIDTH_POINTS: f32 = 1.5;

/// **Half the width of a datum's lines, in POINTS**: one point thick,
/// a third of a mark.
///
/// A plane is ruled out to its horizon, so its grid is the one lane
/// that covers the whole picture, and at a mark's weight it read as
/// the loudest thing on screen. At one point it is the hairline a
/// grid is. On a display of at least one pixel per point that is at
/// least one physical pixel, and a quad at least a pixel wide covers a
/// pixel centre in every column (or row) it crosses, so the line stays
/// continuous rather than dotting the way a one-pixel `LineList` did.
/// Under a scale factor below one the quad is narrower than a pixel
/// and the grid CAN drop pixels along a shallow line; the marks, three
/// times as wide, stay continuous down to a third of that scale.
const DATUM_HALF_WIDTH_POINTS: f32 = 0.5;

/// **Each lane's style.** One match, so a lane cannot be added
/// without being given one.
///
/// Profiles and previews are drawn at a mark's width and fully opaque,
/// and the datum grid thin and half-transparent: a profile is what a
/// person is looking at on a plane, and the plane is what it is drawn
/// against. **Thickness is the channel that separates them**, with the
/// grid's opacity on top: a translucent profile would take on the
/// colour of whatever it crossed — the grid, a shaded face — and a
/// profile's colour is what says which of the two lanes it is in.
/// Drawn over the grid in any case ([`EdgeLane::DRAW_ORDER`]), so the
/// grid never covers one.
fn lane_style(lane: EdgeLane) -> LaneStyle {
    match lane {
        EdgeLane::Datum => LaneStyle {
            half_width_points: DATUM_HALF_WIDTH_POINTS,
            opacity: crate::theme::DATUM_OPACITY,
        },
        EdgeLane::Profile | EdgeLane::Preview | EdgeLane::Hovered | EdgeLane::Selected => {
            LaneStyle {
                half_width_points: EDGE_MARK_HALF_WIDTH_POINTS,
                opacity: 1.0,
            }
        }
    }
}

/// One vertex of an expanded edge mark: the segment it belongs to,
/// twice over, plus which corner of the quad this is.
///
/// **Both endpoints on every vertex** — the shader needs the
/// segment's screen DIRECTION to know which way to offset, and a
/// vertex that carried only its own position could not compute one.
/// The cost is the duplication (24 bytes of position per vertex
/// instead of 12, six vertices per segment instead of two); the marks
/// are a handful of edges, so it buys correctness for nothing that
/// matters.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct SegmentVertex {
    /// The segment's first endpoint, world space.
    a: [f32; 3],
    /// Its second endpoint, world space.
    b: [f32; 3],
    /// The corner code: bit 0 picks the endpoint (`b` when set), bit
    /// 1 picks the side of the segment (the negative normal when
    /// set). Decoded in `vs_edge`, which is why the two bits are
    /// documented here and not spelled twice.
    corner: u32,
}

/// The corner codes of one quad, as two triangles.
///
/// Order is `(a+, a−, b+)` then `(b+, a−, b−)` — consistent winding
/// is not load-bearing (the pipeline culls nothing, because a mark
/// seen from behind is still a mark) but a coherent order is what
/// makes the two triangles a quad rather than a bow tie.
const QUAD_CORNERS: [u32; 6] = [0, 2, 1, 1, 2, 3];

/// The edge pass's depth nudge: a dimensionless multiplicative lift
/// applied to clip-space z in `vs_edge`, before the perspective
/// divide. Depth is reversed (`Camera::projection_matrix`), so a
/// LARGER z is nearer the eye.
///
/// Applied in the vertex shader, not as pipeline `DepthBiasState`:
/// WebGPU validation forbids a depth bias on non-triangle topology,
/// so a line pipeline that asks for one refuses to build at all. On a
/// float depth buffer the lift is worth a handful of quanta at
/// every depth — an f32's ulp steps per binade, so `z * k * 2^-23`
/// lands between k/2 and k quanta depending on where `z` sits within
/// its binade — never exactly "k quanta", but enough either way to
/// clear the shared-position z-fight.
///
/// **Eyeballed, and there is no measurement behind it**: the lines lie
/// exactly on the surface (they share its positions), so a nudge
/// toward the eye large enough to clear a few depth quanta is enough,
/// and one large enough to lift a mark off a NEIGHBOURING surface
/// would be a bug. A handful of quanta rather than the fixed-function
/// era's two, because a vertex-shader nudge has no slope-scaled half —
/// the extra quanta stand in for what `slope_scale` gave a line lying
/// on a steeply-angled facet. The pass writes no depth, so an
/// over-large lift could only make a mark show through geometry it
/// should not — which is the reason to keep it minimal rather than to
/// tune it.
///
/// `crate::pickindex`'s `OCCLUSION_SLACK_REL` plays the same
/// coincident-edge-over-its-own-face role on the CPU pick lane, in a
/// different numeric domain (f64 world-depth comparison there, f32
/// clip z here) — a pointer each way, deliberately not one shared
/// constant.
///
/// **A quad, unlike a line, reaches ONTO the facets its edge
/// divides** — which is what sets the magnitude. On a concave edge
/// the neighbouring facet is nearer the eye than the shared chord, so
/// without enough nudge the outer half of a mark fails the depth test
/// and the mark thins on exactly the edges width was buying. The
/// nudge is still small enough that a mark cannot climb over
/// unrelated geometry: the pass writes no depth, so the worst an
/// over-large lift could do is show a mark through a surface in
/// front of it, and at 1e-5 relative that surface would have to be
/// within a thousandth of a percent of the edge's own depth.
const EDGE_CLIP_Z_LIFT: f32 = 1.0e-5;

struct Geometry {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    ids: wgpu::Buffer,
    /// Per-corner display flags (`SceneMesh::FLAG_PROBE`): the G3
    /// distinctness value, painted as the probe tint below.
    flags: wgpu::Buffer,
    /// How many vertices one pass over these buffers draws — the
    /// scene's corner count, which is [`corner_count`]'s answer for
    /// the [`SceneMesh`] they were built from.
    corners: u32,
    /// Which scene these buffers hold. The app bumps it whenever it
    /// rebuilds the mesh; a mismatch here is the upload trigger.
    revision: u64,
}

/// How many vertices one pass over `scene` draws.
///
/// **The scene is non-indexed geometry** — [`SceneMesh`]'s own
/// contract: every triangle emits its own three corners, so nothing
/// is shared and the draw range is the corner table's own length.
/// This is the only place that number is derived, so the two passes
/// over one scene cannot draw different ranges of it.
fn corner_count(scene: &SceneMesh) -> u32 {
    u32::try_from(scene.positions().len()).unwrap_or(u32::MAX)
}

impl ViewportRenderer {
    /// Build the pipeline. Called once, at application start, with the
    /// render state `eframe` hands the app.
    pub(crate) fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("viewer_scene_shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source(target_format).into()),
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
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Greater),
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
        let depth_reset = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("viewer_depth_reset_pipeline"),
            // Derived from the entry points, which bind nothing.
            layout: None,
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_depth_reset"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Always),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs_depth_reset"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                // The pass's colour target, written by nobody: this
                // draw is for the depth it leaves behind.
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::empty(),
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let id = IdPass::new(device, &module, &bind_group_layout, &layout);
        let edges = EdgePass::new(device, &module, &layout, target_format);
        Self {
            pipeline,
            depth_reset,
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
        self.geometry = Some(Geometry {
            positions,
            normals,
            ids,
            flags,
            corners: corner_count(scene),
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
        if geometry.corners == 0 {
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
                datum: [0.0; 4],
                profile: [0.0; 4],
                edge: [0.0; 4],
                edge_lanes: [[0.0; 4]; EdgeLane::DRAW_ORDER.len()],
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
                        // Reversed depth: 0 is infinitely far.
                        load: wgpu::LoadOp::Clear(0.0),
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
            pass.draw(0..geometry.corners, 0..1);
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
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Greater),
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
                        array_stride: core::mem::size_of::<SegmentVertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![
                            0 => Float32x3, 1 => Float32x3, 2 => Uint32,
                        ],
                    }),
                    Some(wgpu::VertexBufferLayout {
                        array_stride: 4,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![3 => Uint32],
                    }),
                ],
            },
            primitive: wgpu::PrimitiveState {
                // Triangles, because a mark is a quad now: see
                // `EdgePass`'s note on why a line cannot be widened.
                topology: wgpu::PrimitiveTopology::TriangleList,
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
                // triangles share, so a mark lands exactly on the
                // surface's own depth: `GreaterEqual` plus the vertex
                // shader's `EDGE_CLIP_Z_LIFT` nudge is what keeps it
                // from z-fighting with the facet it borders. The
                // nudge stays in the shader rather than moving to
                // `DepthBiasState` now that the topology would admit
                // one: the widened quad needs the SAME depth its
                // endpoints have (see `EDGE_CLIP_Z_LIFT`), and a
                // slope-scaled bias over a quad that is flat in
                // screen space is not that.
                depth_compare: Some(wgpu::CompareFunction::GreaterEqual),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module,
                entry_point: Some("fs_edge"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                // Blended, for the lanes that are drawn translucent
                // (`lane_style`): straight alpha, the fragment's alpha
                // being its lane's opacity. An opaque lane writes
                // alpha 1 and so replaces what is under it exactly as
                // an unblended pass would.
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
        let (positions, marks) = edge_vertices(overlay);
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

/// **One overlay as the edge pass's vertices**: the quads' corners, and
/// each corner's word (its [`lane_code`] and probe flag), lane by lane
/// in [`EdgeLane::DRAW_ORDER`] — so the buffer order IS the draw order,
/// and the lane drawn last is the one on top.
///
/// Six vertices per SEGMENT, not one per endpoint: the quad is
/// `QUAD_CORNERS` — two triangles over the four corners the shader
/// derives from the segment's own screen direction.
fn edge_vertices(overlay: &EdgeOverlay) -> (Vec<SegmentVertex>, Vec<u32>) {
    let segments = overlay.segments();
    let mut positions: Vec<SegmentVertex> = Vec::with_capacity(segments * QUAD_CORNERS.len());
    let mut marks: Vec<u32> = Vec::with_capacity(positions.capacity());
    for lane in EdgeLane::DRAW_ORDER {
        // Only the two marks can belong to a free-moved instance: a
        // preview, a profile and a datum are placed by nothing, so the
        // flag stays clear and the lane supplies its colour outright.
        let word = if overlay.probed(lane) {
            lane_code(lane) | EDGE_FLAG_PROBE
        } else {
            lane_code(lane)
        };
        // `chunks_exact(2)`: the overlay is a LINE LIST, so a trailing
        // odd position is not half a segment to draw — it is a
        // producer bug, and drawing nothing for it is the quiet half
        // of failing loud at the producer.
        for pair in overlay.lane(lane).chunks_exact(2) {
            for corner in QUAD_CORNERS {
                positions.push(SegmentVertex {
                    a: pair[0],
                    b: pair[1],
                    corner,
                });
                marks.push(word);
            }
        }
    }
    (positions, marks)
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
    /// The pane's size in physical pixels, and how many of those go
    /// to a point — what the edge pass measures its lines' widths
    /// against ([`lane_style`]).
    pub(crate) viewport_px: [f32; 2],
    /// The frame's device pixel ratio; see
    /// [`ViewportCallback::viewport_px`].
    pub(crate) pixels_per_point: f32,
    /// Which patch ids to mark, from `crate::marks::highlight` — a
    /// value computed from (index, selection, hover) and handed
    /// straight through. **No highlight decision is taken here**; this
    /// pass paints what the pure function said.
    pub(crate) highlight: Highlight,
    /// Which edges to mark, from `crate::marks::edge_overlay` — the
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
            datum: {
                let [r, g, b] = crate::theme::linear(self.theme.datum);
                [r, g, b, 0.0]
            },
            profile: {
                let [r, g, b] = crate::theme::linear(self.theme.profile);
                [r, g, b, 0.0]
            },
            edge: [self.viewport_px[0], self.viewport_px[1], 0.0, 0.0],
            edge_lanes: EdgeLane::DRAW_ORDER.map(|lane| {
                let style = lane_style(lane);
                [
                    style.half_width_points * self.pixels_per_point,
                    style.opacity,
                    0.0,
                    0.0,
                ]
            }),
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
        //
        // Depth first: the pass arrived cleared to 1.0, which under
        // reversed depth is the NEAREST value, so it is reset to 0
        // (infinitely far) over the pane before anything tests
        // against it.
        render_pass.set_pipeline(&renderer.depth_reset);
        render_pass.draw(0..3, 0..1);
        render_pass.set_pipeline(&renderer.pipeline);
        render_pass.set_bind_group(0, &renderer.bind_group, &[]);
        render_pass.set_vertex_buffer(0, geometry.positions.slice(..));
        render_pass.set_vertex_buffer(1, geometry.normals.slice(..));
        render_pass.set_vertex_buffer(2, geometry.ids.slice(..));
        render_pass.set_vertex_buffer(3, geometry.flags.slice(..));
        render_pass.draw(0..geometry.corners, 0..1);
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
fn shader_source(target_format: wgpu::TextureFormat) -> String {
    SHADER
        .replace(
            "{{ENCODE_SRGB}}",
            // **The one place the pipeline's output boundary is
            // decided.** `egui-wgpu` asks the surface for a NON-sRGB
            // framebuffer on purpose (`preferred_framebuffer_format`
            // takes `Rgba8Unorm`/`Bgra8Unorm` before anything else,
            // because egui blends in gamma space), so what this pass
            // writes is displayed as an sRGB code with no encode
            // applied. It shades in LINEAR — `theme::linear` is what
            // feeds the uniforms — so every colour reached the screen
            // one gamma step too dark: measured on the light neutral
            // palette, a fully lit face came out at sRGB 142 where the
            // palette says 197, and an unlit one landed exactly at the
            // raw ambient fraction.
            //
            // The encode therefore happens HERE, at the boundary, and
            // is skipped where the surface would do it — the fallback
            // arm of that same egui function can hand back an `*Srgb`
            // format when no gamma-space one is offered, and encoding
            // into that would be the same error in the other
            // direction.
            if target_format.is_srgb() {
                "false"
            } else {
                "true"
            },
        )
        .replace(
            "{{FLAG_PROBE}}",
            &crate::scene::SceneMesh::FLAG_PROBE.to_string(),
        )
        .replace(
            "{{FLAG_FOCUS}}",
            &crate::scene::SceneMesh::FLAG_FOCUS.to_string(),
        )
        .replace("{{EDGE_LANE_MASK}}", &EDGE_LANE_MASK.to_string())
        .replace("{{EDGE_FLAG_PROBE}}", &EDGE_FLAG_PROBE.to_string())
        .replace("{{EDGE_LANES}}", &EdgeLane::DRAW_ORDER.len().to_string())
        .replace("{{EDGE_LANE_COLOURS}}", &lane_colour_switch())
        .replace("{{EDGE_CLIP_Z_LIFT}}", &format!("{EDGE_CLIP_Z_LIFT:e}"))
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
    // The construction colour, in xyz; w is padding.
    datum: vec4<f32>,
    // A committed profile's colour, in xyz; w is padding.
    profile: vec4<f32>,
    // Viewport size in physical pixels (xy). See the Rust `Uniforms`.
    edge: vec4<f32>,
    // Per edge lane, by lane code: half width in physical pixels (x)
    // and opacity (y).
    edge_lanes: array<vec4<f32>, {{EDGE_LANES}}>,
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

// Whether this pass owes the sRGB encode — see `shader_source`, which
// substitutes it from the surface format eframe chose.
const ENCODE_SRGB: bool = {{ENCODE_SRGB}};

// **Linear light out to the display's own space.** The IEC 61966-2-1
// curve, the same one `theme::channel_to_srgb8` states on the Rust
// side and for the same reason: the toe below 0.0031308 is linear, and
// rounding it into the exponent is the difference that shows in
// near-black — which on a palette with a near-black probe mark is the
// half that has to be right.
//
// Two spellings of one curve is a thing to know about: this is a
// shader and that is a `u8` encoder, and neither can call the other.
// `every_shader_token_is_substituted`'s sibling row pins the
// constants against the Rust ones so a change to either is a failure
// rather than a divergence.
fn to_display(linear: vec3<f32>) -> vec3<f32> {
    if (!ENCODE_SRGB) {
        return linear;
    }
    let c = clamp(linear, vec3<f32>(0.0), vec3<f32>(1.0));
    let toe = c * 12.92;
    let curve = 1.055 * pow(c, vec3<f32>(1.0 / 2.4)) - 0.055;
    return select(curve, toe, c <= vec3<f32>(0.0031308));
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
    return vec4<f32>(to_display(shade), 1.0);
}

// An edge mark: the theme's own selected/hovered mark composited over
// the body colour — `tint`, the same mix the shaded pass runs — and
// drawn UNSHADED, which is what keeps a marked edge distinguishable
// from the marked face it borders without a second palette entry.
struct EdgeOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(flat) mark: u32,
};

// The screen-space expansion: both of the segment's endpoints arrive
// on every vertex, and `corner` says which one this vertex sits at
// and which side of the segment it is offset to. See `SegmentVertex`
// for the bit layout and `EdgePass` for why a mark is a quad.
@vertex
fn vs_edge(
    @location(0) a: vec3<f32>,
    @location(1) b: vec3<f32>,
    @location(2) corner: u32,
    @location(3) mark: u32,
) -> EdgeOut {
    var clip_a = uniforms.view_projection * vec4<f32>(a, 1.0);
    var clip_b = uniforms.view_projection * vec4<f32>(b, 1.0);
    // The mark pass's depth nudge, in the shader rather than as a
    // pipeline bias: a relative lift of clip z (toward the eye, depth
    // being reversed), worth a few float-depth quanta at any depth;
    // see EDGE_CLIP_Z_LIFT.
    clip_a.z *= 1.0 + {{EDGE_CLIP_Z_LIFT}};
    clip_b.z *= 1.0 + {{EDGE_CLIP_Z_LIFT}};

    let half_viewport = max(uniforms.edge.xy, vec2<f32>(1.0, 1.0)) * 0.5;
    // abs(w), not w: an endpoint behind the eye has a negative w, and
    // the projected point is then mirrored through the origin. The
    // magnitude keeps the two endpoints on the same side of that
    // mirror, so the DIRECTION between them — all this is for — stays
    // the segment's own. Such a segment is clipped by the rasterizer
    // in any case; what this avoids is a quad twisted into a bow tie
    // before the clip gets to it.
    let screen_a = clip_a.xy / max(abs(clip_a.w), 1.0e-6) * half_viewport;
    let screen_b = clip_b.xy / max(abs(clip_b.w), 1.0e-6) * half_viewport;
    let along = screen_b - screen_a;
    let span = length(along);
    // A segment with no screen extent (seen exactly end-on, or a
    // degenerate chord) has no direction to take a normal from. It is
    // widened along x, which draws a square dot at the point the
    // segment collapsed to — the honest picture of a mark with no
    // length, and never a NaN.
    var direction = vec2<f32>(1.0, 0.0);
    if (span > 1.0e-6) {
        direction = along / span;
    }
    let normal = vec2<f32>(-direction.y, direction.x);
    let side = select(1.0, -1.0, (corner & 2u) != 0u);
    let clip = select(clip_a, clip_b, (corner & 1u) != 0u);
    // Pixels back to clip space: an NDC offset is a pixel offset over
    // the half viewport, and multiplying by w undoes the perspective
    // divide the rasterizer is about to apply — which is what makes
    // the width constant on screen rather than in world units.
    let lane = mark & {{EDGE_LANE_MASK}}u;
    let offset = normal * side * uniforms.edge_lanes[lane].x / half_viewport;
    var out: EdgeOut;
    out.clip_position = vec4<f32>(clip.xy + offset * clip.w, clip.z, clip.w);
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
    let lane = in.mark & {{EDGE_LANE_MASK}}u;
    // One arm per lane, generated from `EdgeLane` (the Rust
    // `lane_colour_switch`).
    var color: vec3<f32>;
    {{EDGE_LANE_COLOURS}}
    return vec4<f32>(to_display(color), uniforms.edge_lanes[lane].y);
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

// One triangle covering the whole viewport at depth 0 — the reversed
// depth's "infinitely far" — for `ViewportRenderer::depth_reset`.
@vertex
fn vs_depth_reset(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    let x = f32((index << 1u) & 2u) * 2.0 - 1.0;
    let y = f32(index & 2u) * 2.0 - 1.0;
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_depth_reset() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}

@fragment
fn fs_id(in: IdOut) -> @location(0) u32 {
    return in.id;
}
"#;

#[cfg(test)]
// Panicking is a test's failure mechanism (workspace lint note).
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use eframe::egui_wgpu;
    use pncad::geom_core::Tol;

    use super::*;
    use crate::scene::{self, DisplayTolerance};

    /// Every `{{TOKEN}}` in [`SHADER`] must be substituted by
    /// [`shader_source`]: an unreplaced token would reach the WGSL
    /// compiler as a parse error at pipeline build — i.e. at app
    /// startup, not at test time. Both directions are pinned: the
    /// substituted source carries no `{{`, and the template still
    /// spells every token `shader_source` replaces (a token renamed on
    /// one side only would otherwise pass silently).
    ///
    /// Runs only under `--features app`, this module's own gate, so
    /// its seat is the hosted row `cargo nextest run -p viewer
    /// --features app` (`.github/workflows/ci.yml`) rather than the
    /// workspace archive — see `lib.rs`'s loud-skip marker for what a
    /// build without the feature is not checking.
    #[test]
    fn every_shader_token_is_substituted() {
        let source = shader_source(wgpu::TextureFormat::Bgra8Unorm);
        assert!(
            !source.contains("{{"),
            "an unreplaced template token survives in the shader source"
        );
        for token in [
            "{{FLAG_PROBE}}",
            "{{FLAG_FOCUS}}",
            "{{EDGE_LANE_MASK}}",
            "{{EDGE_FLAG_PROBE}}",
            "{{EDGE_LANES}}",
            "{{EDGE_LANE_COLOURS}}",
            "{{EDGE_CLIP_Z_LIFT}}",
            "{{ENCODE_SRGB}}",
        ] {
            assert!(
                SHADER.contains(token),
                "the template no longer spells {token}; keep this list \
                 and shader_source's replacements in step"
            );
        }
    }

    /// **Every lane's code is its draw position, fits the lane mask, and
    /// is not the probe flag.** The code is the discriminant; the style
    /// rows are indexed by draw position, so the two have to be one
    /// number — which holds only while the enum is declared in draw
    /// order, and this row is what says so if it stops.
    #[test]
    fn every_lane_code_is_its_draw_position() {
        for (position, lane) in EdgeLane::DRAW_ORDER.into_iter().enumerate() {
            let code = lane_code(lane);
            assert_eq!(
                code as usize, position,
                "{lane:?}'s code is not its position"
            );
            assert_eq!(
                code & EDGE_LANE_MASK,
                code,
                "{lane:?}'s code overflows its bits"
            );
            assert_eq!(
                code & EDGE_FLAG_PROBE,
                0,
                "{lane:?}'s code reads as the probe flag"
            );
        }
    }

    /// **The shader's colour switch has exactly one arm per lane**, at
    /// that lane's code — so no lane falls through to the unhandled
    /// colour, and the substituted shader carries the switch.
    #[test]
    fn the_colour_switch_has_one_arm_per_lane() {
        let switch = lane_colour_switch();
        for lane in EdgeLane::DRAW_ORDER {
            let arm = format!(
                "case {}u: {{ color = {}; }}",
                lane_code(lane),
                lane_colour_wgsl(lane)
            );
            assert_eq!(switch.matches(&arm).count(), 1, "{lane:?}: {switch}");
        }
        assert_eq!(
            switch.matches("case ").count(),
            EdgeLane::DRAW_ORDER.len(),
            "{switch}"
        );
        assert!(
            shader_source(wgpu::TextureFormat::Bgra8Unorm).contains(&switch),
            "the switch is not what the shader draws with"
        );
    }

    /// **The buffer order is the draw order**, and so the priority:
    /// the pass writes no depth, so the lane whose vertices come later
    /// is the one on top where two cover a pixel.
    ///
    /// One segment per lane, each at its own x so a vertex says which
    /// lane's segment it carries; the lanes are FILLED in an order
    /// that is not the draw order, so a builder that walked the fields
    /// rather than `DRAW_ORDER` reds this. The selected mark is on a
    /// free-moved instance, and only its vertices may carry the probe
    /// flag.
    #[test]
    fn the_lanes_are_emitted_in_draw_order_with_the_selection_last() {
        let segment = |x: f32| vec![[x, 0.0, 0.0], [x, 1.0, 0.0]];
        let overlay = EdgeOverlay {
            selected: segment(4.0),
            hovered: segment(3.0),
            selected_probed: true,
            hovered_probed: false,
            preview: segment(2.0),
            datums: segment(0.0),
            profiles: segment(1.0),
        };
        let (positions, marks) = edge_vertices(&overlay);
        assert_eq!(positions.len(), 5 * QUAD_CORNERS.len());
        assert_eq!(marks.len(), positions.len());
        let codes: Vec<u32> = marks.iter().map(|word| word & EDGE_LANE_MASK).collect();
        assert!(
            codes.windows(2).all(|pair| pair[0] <= pair[1]),
            "lanes interleave or run backwards: {codes:?}",
        );
        for (vertex, (word, code)) in positions.iter().zip(marks.iter().zip(&codes)) {
            let lane = EdgeLane::DRAW_ORDER[*code as usize];
            assert_eq!(
                overlay.lane(lane)[0][0],
                vertex.a[0],
                "a {lane:?} vertex carries another lane's segment",
            );
            assert_eq!(
                word & EDGE_FLAG_PROBE != 0,
                lane == EdgeLane::Selected,
                "the probe flag on a {lane:?} vertex",
            );
        }
        assert_eq!(codes.first(), Some(&lane_code(EdgeLane::Datum)));
        assert_eq!(codes.last(), Some(&lane_code(EdgeLane::Selected)));
    }

    /// **Nothing is drawn fainter or thinner than the datum grid.** The
    /// grid is the backdrop, so every lane drawn over it is at least as
    /// wide and as opaque; and the lanes that say what something IS —
    /// a profile, a preview, a mark — are strictly wider, so a profile
    /// on its own plane reads as the heavier line even where colour
    /// cannot tell them apart.
    #[test]
    fn the_datum_grid_is_the_faintest_and_thinnest_lane() {
        let grid = lane_style(EdgeLane::Datum);
        assert!(grid.opacity > 0.0 && grid.opacity <= 1.0, "{grid:?}");
        // An absolute bound as well as the relative ones below: the
        // grid is a hairline, at most one point across, and a grid
        // drawn at a mark's weight — Ev's complaint — fails here even
        // if every other lane were widened to stay ahead of it.
        assert!(
            grid.half_width_points > 0.0 && 2.0 * grid.half_width_points <= 1.0,
            "the grid is {} pt wide; a hairline is at most 1 pt",
            2.0 * grid.half_width_points
        );
        for lane in EdgeLane::DRAW_ORDER {
            if lane == EdgeLane::Datum {
                continue;
            }
            let style = lane_style(lane);
            assert!(
                style.half_width_points > grid.half_width_points,
                "{lane:?} is no wider than the grid"
            );
            assert!(
                style.opacity >= grid.opacity,
                "{lane:?} is fainter than the grid"
            );
        }
    }

    /// **The output encode follows the surface, both ways.**
    ///
    /// A gamma-space framebuffer — which is what `egui-wgpu` asks the
    /// surface for — needs this pass to encode, because it shades in
    /// linear. An `*Srgb` one does the encode itself and must not get
    /// a second. The bug this pins was the first case going
    /// unhandled: the whole viewport drew one gamma step dark under
    /// every palette.
    #[test]
    fn the_srgb_encode_is_on_exactly_when_the_surface_does_not_do_it() {
        for format in [
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::TextureFormat::Rgba8Unorm,
        ] {
            assert!(
                shader_source(format).contains("const ENCODE_SRGB: bool = true;"),
                "{format:?} is a gamma-space surface, so this pass owes the encode",
            );
        }
        for format in [
            wgpu::TextureFormat::Bgra8UnormSrgb,
            wgpu::TextureFormat::Rgba8UnormSrgb,
        ] {
            assert!(
                shader_source(format).contains("const ENCODE_SRGB: bool = false;"),
                "{format:?} encodes on write; a second encode here would wash it out",
            );
        }
    }

    /// **The shader's transfer curve is the palette's transfer
    /// curve.** They are two spellings — WGSL and Rust — of IEC
    /// 61966-2-1, and nothing but this row stops one from being edited
    /// without the other. The constants are what is compared, because
    /// they are what a divergence would be made of.
    #[test]
    fn the_shaders_srgb_curve_states_the_same_constants_the_palette_does() {
        for constant in ["12.92", "1.055", "0.055", "1.0 / 2.4", "0.0031308"] {
            assert!(
                SHADER.contains(constant),
                "the shader's sRGB encode no longer spells {constant}; \
                 `theme::channel_to_srgb8` is the other half of this curve",
            );
        }
    }

    /// **The draw range is every corner, and every buffer the passes
    /// bind is that long.**
    ///
    /// The scene is non-indexed, so [`corner_count`] is the whole
    /// contract between the buffers and the draw: the shaded pass
    /// binds positions, normals, ids and flags, the id pass binds
    /// positions and ids, and both draw `0..corners`. A table shorter
    /// than that range is a read past the end of a vertex buffer —
    /// wgpu validation on a device, and nothing at all here — and a
    /// table longer than it is geometry silently not drawn. `flags` is
    /// the one this pins that nothing else does: a walk arm that
    /// pushed a corner without its display word would draw the whole
    /// picture and mis-tint part of it.
    #[test]
    fn the_draw_range_is_the_length_of_every_buffer_the_passes_bind() {
        let tol = Tol::witness();
        let (doc, _root) = scene::plate_with_hole(tol).expect("the plate authors");
        let delta = DisplayTolerance::new(1.0e-4).expect("a positive display tolerance");
        let mesh = scene::scene_of(&doc, delta, tol).expect("the plate tessellates");

        let corners = corner_count(&mesh);
        assert_eq!(
            usize::try_from(corners).expect("the corner count fits a usize"),
            mesh.stats().triangles * 3,
            "the draw range is not every triangle's three corners"
        );
        for (what, held) in [
            ("positions", mesh.positions().len()),
            ("normals", mesh.normals().len()),
            ("ids", mesh.ids().len()),
            ("flags", mesh.flags().len()),
        ] {
            assert_eq!(
                held,
                usize::try_from(corners).expect("the corner count fits a usize"),
                "the passes draw {corners} vertices and bind {held} of {what}"
            );
        }

        // The empty picture draws nothing: `read_id_at` reads this as
        // "there is no answer" rather than submitting an empty pass.
        assert_eq!(corner_count(&SceneMesh::empty(mesh.bounds(), delta)), 0);
    }

    /// **Both scene passes hand [`corner_count`]'s answer to `draw`,
    /// and neither reaches an index buffer.**
    ///
    /// The row above pins the VALUE; nothing headless can pin the
    /// call, because a draw needs a device and no adapter here has
    /// one. What is left is the text of the two call sites, read
    /// through the shared lexer so comments and literals — this row's
    /// own needles included — are not counted.
    ///
    /// It asserts that the two passes are spelled this way, not that
    /// the GPU executes them: the hosted viewer-render rows are what
    /// judge the picture.
    #[test]
    fn both_scene_passes_draw_the_corner_count_with_no_index_buffer() {
        let code = test_utils::source::code_only(include_str!("gpu.rs"));
        assert_eq!(
            code.matches(".draw(0..geometry.corners, 0..1)").count(),
            2,
            "the shaded pass and the id pass are the two draws over the scene's \
             buffers; a third, or one spelled differently, is outside what this \
             row and `corner_count` together cover"
        );
        for absent in [".draw_indexed(", ".set_index_buffer("] {
            assert_eq!(
                code.matches(absent).count(),
                0,
                "`{absent}` is back: the scene is non-indexed geometry, so an index \
                 buffer here is a permutation nothing produces"
            );
        }
    }

    /// **Every pipeline in this module, built on a real device.**
    ///
    /// Device acquisition, shader-module compilation and each
    /// `create_render_pipeline` call are pure construction under
    /// wgpu's typed validation, and that validation refuses
    /// combinations the Rust types admit: a depth bias on a
    /// non-triangle topology, a vertex layout the WGSL does not
    /// declare, an attachment format the pipeline's blend state
    /// cannot take. Every one of those is raised when the
    /// application starts, on every adapter and every backend, and
    /// nothing above this seam can see them — G1 excuses pixel
    /// painting from a headless row, and pipeline construction is
    /// not pixel painting. This row is where the whole family gates.
    ///
    /// **Construction only.** No surface, no swapchain, no frame, no
    /// readback, and nothing is asserted about a pixel. What is
    /// asserted is that the device raised no validation error while
    /// [`ViewportRenderer::new`] built the shaded pipeline, the id
    /// pipeline and their 1x1 targets, and the edge pipeline.
    ///
    /// **Two formats, one axis.** [`shader_source`] branches on
    /// `is_srgb()`, not on the format, so `Bgra8Unorm` and
    /// `Bgra8UnormSrgb` sample both sides of the only branch it has —
    /// two shader modules and two sets of pipelines. The channel order
    /// is NOT sampled: `Rgba8Unorm`/`Rgba8UnormSrgb`, which a surface
    /// may well prefer, build no pipeline here.
    ///
    /// **No adapter is a FAILURE here, never a skip.** A row that
    /// goes green where no GPU exists reports the same thing whether
    /// the pipelines built or the driver was absent, and a reader
    /// cannot tell those apart — which is the exact defect this row
    /// exists to close. The environment owes this row an adapter:
    /// `mesa-vulkan-drivers` supplies lavapipe, it needs no display
    /// and no X server (crates/viewer/README.md, headless), and the
    /// panic below names it when it is missing.
    #[test]
    fn every_pass_builds_on_a_real_device() {
        // `_from_env` so `WGPU_BACKEND` can steer this row at an
        // operator's hand; with the variable unset it is every
        // backend the build has. No display handle: this row opens
        // no surface, so there is no window to hand one from.
        let instance =
            wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
                .expect(
                    "NO WGPU ADAPTER. This row builds every pipeline in this module on a real \
                     device and can report nothing without one, so it fails rather than passing \
                     emptily. Install a software ICD: `mesa-vulkan-drivers` supplies lavapipe, \
                     which needs no display (crates/viewer/README.md, headless).",
                );
        let info = adapter.get_info();
        println!(
            "viewer::gpu pipeline smoke: adapter {:?} / {} ({:?}, driver {:?})",
            info.backend, info.name, info.device_type, info.driver
        );

        // THE APP'S DEVICE, NOT A PERMISSIVE ONE. This crate never
        // builds a `DeviceDescriptor`: `NativeOptions`' default
        // `wgpu_options` carries `egui_wgpu`'s own closure, and that
        // is what the running app requests. So the row ASKS THAT
        // CLOSURE rather than restating its limits or handing itself
        // `adapter.limits()` — at the adapter's limits a pipeline that
        // fits the hardware and exceeds what egui asks for builds
        // green here and panics at startup, which is the one failure
        // this row exists to close.
        let egui_wgpu::WgpuSetup::CreateNew(setup) =
            egui_wgpu::WgpuConfiguration::default().wgpu_setup
        else {
            panic!(
                "egui_wgpu's default setup is no longer `CreateNew`, so this row can no longer \
                 ask it for the device descriptor the app requests"
            );
        };
        let (device, _queue) =
            pollster::block_on(adapter.request_device(&(setup.device_descriptor)(&adapter)))
                .expect("the adapter above must yield a device at the limits egui_wgpu asks for");

        // THE CENSUS, BOUND TO THE SOURCE. "Every pipeline" is a claim
        // only if something notices a new one: all four of this
        // file's `create_render_pipeline` calls are reached from
        // `ViewportRenderer::new`, and a fifth built lazily in a
        // frame path this row never enters would leave the row green
        // and its name unchanged.
        //
        // Read through the SHARED lexer, which is what
        // `crates/test-utils/tests/reader_census.rs` exists to make
        // the only way: a raw `matches` over the text counts the
        // needle in comments and in this row's own literals too, so it
        // would answer about prose rather than about calls. The code
        // view strips both, which is also why the needle needs no
        // splitting trick to avoid counting itself.
        assert_eq!(
            test_utils::source::code_only(include_str!("gpu.rs"))
                .matches(".create_render_pipeline(")
                .count(),
            4,
            "this module no longer builds exactly the four pipelines `ViewportRenderer::new` \
             builds. Route the new one through `new` so this row covers it, or narrow this \
             row's claim and its count together."
        );

        for target_format in [
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::TextureFormat::Bgra8UnormSrgb,
        ] {
            let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
            // Held until the scope is popped: the error is raised by
            // the calls inside `new`, and dropping the pipelines first
            // would not unsay it, but keeping the value is what makes
            // the construction the scope's subject rather than a
            // temporary's lifetime.
            let renderer = ViewportRenderer::new(&device, target_format);
            let error = pollster::block_on(scope.pop());
            assert!(
                error.is_none(),
                "{target_format:?}: {}",
                error.map_or_else(String::new, |e| e.to_string()),
            );
            drop(renderer);
            println!("  {target_format:?}: shaded, depth-reset, id and edge pipelines built");
        }
    }
}
