//! The viewport pane: the wgpu surface, the pointer, and the overlays
//! drawn over both.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use std::sync::Arc;

use eframe::egui;
use pncad::document::{Doc, Evaluation, ProfileProgram, RecipeNodeId};

use crate::app::{ViewerBehavior, chrome};
use crate::camera::{self, Camera, CameraOp};
use crate::display::DisplayView;
use crate::drafts::ProfileDoors;
use crate::frame;
use crate::gpu::{ID_PASS_ASKED, IdQuery, ViewportCallback};
use crate::idpass::{self, IdStep};
use crate::input::{self, PickAction, PointerButton, ViewportEvent, ViewportSize};
use crate::marks;
use crate::narrowing::Narrow;
use crate::pickcache;
use crate::pickindex::{PickIndex, PictureKey};
use crate::screen::{self, view_of};
use crate::session::SessionOp;
use crate::sketch::{self, PreviewLoop, TIP_MARK_PX, heading};

/// **One sketch-plane segment, placed and offered to an overlay lane**
/// as the line-list pair the edge pass draws.
///
/// This function PLACES and nothing else: whether a leg can be drawn
/// is [`marks::LegLane`]'s answer, which is where the crate keeps it
/// and where a test can ask it with no pane in existence. What is
/// particular here is that this lane is the one with an authored
/// producer: a path whose corner is `7e307` — a number the
/// add-profile form takes, because it is a number — replays,
/// flattens, and arrives on its plane with coordinates whose
/// narrowing is an infinity. A path with one such corner and the rest
/// ordinary is drawn as its ordinary legs and a gap, which is
/// `LegLane::undrawn`'s subject.
fn push_segment(
    lane: &mut marks::LegLane,
    plane: &pncad::profile::SketchPlane<f64>,
    a: [f64; 2],
    b: [f64; 2],
) {
    let [from, to] = [a, b].map(|[x, y]| plane.to_world(pncad::geom_core::Point2::new(x, y)));
    lane.leg(from, to);
}

/// **One drawn loop, placed on its plane and appended to a lane.**
///
/// A loop that closes ([`sketch::LoopEnd::closes`]) wraps — the last
/// point joins the first, which is the same thing `ProfileLoop` means
/// by being closed by construction. One that does not must not: the
/// leg back to the start is the provisional close `sketch::preview`
/// walked the chain under and nobody authored, so the wrap is dropped
/// and what is drawn is the authored legs exactly. A loop of one
/// vertex has no leg and is held as its point ([`marks::LegLane::point`]),
/// so a fit still frames it.
fn push_loop(
    lane: &mut marks::LegLane,
    plane: &pncad::profile::SketchPlane<f64>,
    polyline: &PreviewLoop,
) {
    let points = &polyline.points;
    let segments = if polyline.end.closes() {
        points.len()
    } else {
        points.len().saturating_sub(1)
    };
    for index in 0..segments {
        push_segment(
            lane,
            plane,
            points[index],
            points[(index + 1) % points.len()],
        );
    }
    if let (0, Some(&[x, y])) = (segments, points.first()) {
        lane.point(plane.to_world(pncad::geom_core::Point2::new(x, y)));
    }
}

/// **The world box around what the viewport hands the GPU as
/// geometry** ([`crate::narrowing::drawn_box`]): the scene's drawn
/// corners, and the loops of the committed profiles and the previews.
/// `None` when none of them draws anything.
///
/// A preview's step marks are not in either lane, and must not be:
/// they are sized against the view, so a fit that framed them would
/// depend on the camera it is choosing.
fn drawn_extent(
    scene: &[[f32; 3]],
    profiles: &marks::LegLane,
    preview: &marks::LegLane,
) -> Option<bvh::Aabb> {
    crate::narrowing::drawn_box(
        scene
            .iter()
            .chain(profiles.corners())
            .chain(preview.corners()),
    )
}

/// What a frame's camera half takes: the camera it starts from, what
/// it draws, whether a fit is owed, and the events it folds.
pub(crate) struct CameraFrame<'a> {
    /// The camera the frame starts from.
    pub(crate) camera: &'a Camera,
    /// The scene's drawn corners ([`crate::scene::SceneMesh::positions`]).
    pub(crate) scene: &'a [[f32; 3]],
    /// The committed profiles' lane ([`push_committed`]).
    pub(crate) profiles: marks::LegLane,
    /// The doors' previews.
    pub(crate) previews: &'a Previews,
    /// The open revolve tool's axis over its held profile, if any
    /// ([`push_revolve_axis`]).
    pub(crate) axis: Option<&'a RevolveAxisPreview>,
    /// The open split tool's plane under the copy it was picked
    /// through, if any ([`crate::splitplane::push_split_plane`]).
    pub(crate) split_plane: Option<&'a crate::splitplane::PlanePreview>,
    /// Whether a fit is owed this frame.
    pub(crate) fit: bool,
    /// The bindings the events fold through.
    pub(crate) input: &'a input::InputMap,
    /// The pane, in physical pixels.
    pub(crate) viewport: ViewportSize,
    /// This frame's pointer events.
    pub(crate) events: &'a [ViewportEvent],
}

/// What [`frame_camera`] settles: the owed fit's fold, the events'
/// fold over the camera the fit left, and the two overlay lanes drawn
/// from the camera both left.
pub(crate) struct FramedCamera {
    /// The fit's fold, when one was owed.
    pub(crate) fit: Option<camera::Folded>,
    /// The events' fold, started from the fit's camera.
    pub(crate) events: camera::Folded,
    /// The committed profiles' segments.
    pub(crate) profiles: Vec<[f32; 3]>,
    /// The previews' loops, then their step marks and the revolve
    /// tool's axis.
    pub(crate) preview: Vec<[f32; 3]>,
}

/// **The one order a frame's camera and its overlays are made in**:
/// the geometry lanes, then the extent they and the scene draw, then
/// the owed fit on that extent, then the events' fold, and only then
/// the previews' step marks and the revolve tool's axis, sized against
/// the camera all of that settled on. Pure, so the order is a row rather than a reading of
/// `viewport_ui`.
pub(crate) fn frame_camera(frame: CameraFrame<'_>) -> FramedCamera {
    let CameraFrame {
        camera,
        scene,
        profiles,
        previews,
        axis,
        split_plane,
        fit,
        input: map,
        viewport,
        events,
    } = frame;
    let mut preview = marks::LegLane::default();
    push_previews(&mut preview, previews);
    let fit = viewport.aspect().filter(|_| fit).map(|aspect| {
        let op = CameraOp::Frame {
            extent: drawn_extent(scene, &profiles, &preview),
            aspect,
        };
        camera::fold_recorded(camera, std::slice::from_ref(&op))
    });
    let start = fit.as_ref().map_or(*camera, |fit| fit.camera);
    let events = input::fold_events(map, &start, viewport, events);
    // In pixels, read at each vertex's own depth
    // ([`screen::View::screen_metres_at`]). A window this camera has no
    // view of draws the chains and no marks; the projection refusal is
    // what says why.
    let view = view_of(&events.camera, viewport).ok();
    let mut steps = marks::LegLane::default();
    push_previews_marks(&mut steps, previews, view);
    if let Some(axis) = axis {
        push_revolve_axis(&mut steps, axis, view);
    }
    if let Some(plane) = split_plane {
        crate::splitplane::push_split_plane(&mut steps, plane);
    }
    FramedCamera {
        fit,
        events,
        profiles: profiles.into_segments(),
        preview: [preview.into_segments(), steps.into_segments()].concat(),
    }
}

/// **What the revolve tool would commit, as the viewport draws it**:
/// the held profile's drawing and the axis line and angle the panel's
/// fields hold, canonical (metres, radians) and in the profile's own
/// coordinates — the numbers `RevolveTool::op` is handed.
#[derive(Clone, Debug)]
pub(crate) struct RevolveAxisPreview {
    /// The plane the held profile's landed value is on.
    pub(crate) plane: pncad::profile::SketchPlane<f64>,
    /// The held profile's loops, as the committed-profile pass draws
    /// them.
    pub(crate) loops: Vec<PreviewLoop>,
    /// The point the axis runs through.
    pub(crate) through: [f64; 2],
    /// The axis's heading from the profile's x axis, about its normal.
    pub(crate) heading: f64,
    /// The signed sweep angle.
    pub(crate) angle: f64,
}

/// How long the revolve axis's arrowheads are, in pixels — larger than
/// a step mark's ([`TIP_MARK_PX`]) so the axis's arrow does not read as
/// one more step of the profile.
const AXIS_ARROW_PX: f64 = 1.5 * TIP_MARK_PX;

/// The radius, relative to the profile's reach from the axis point,
/// below which the sweep arc is not drawn — far above the rounding of
/// a heading's sine and cosine, far below a radius anyone sketches.
const SWEEP_ARC_FLOOR: f64 = 1.0e-9;

/// How many legs a full turn of the sweep arc is drawn with.
const SWEEP_ARC_LEGS_PER_TURN: f64 = 64.0;

/// How far an arrowhead's wings spread to each side, as a fraction of
/// its length.
const ARROW_SPREAD: f64 = 0.45;

/// How many ticks mark the side of the axis a profile must lie on.
const SIDE_TICKS: u32 = 4;

/// **The revolve tool's axis and sweep, appended to a lane**, so the
/// direction the axis runs, the side of it the profile must lie on and
/// the way it turns are seen before anything is committed.
///
/// What is drawn follows `sweep::revolve`'s conventions: with `d` the
/// axis's unit heading, a profile must lie where
/// `(p − through).perp_dot(d) ≥ 0`, and a positive angle turns it
/// right-handed about `d`.
///
/// - The axis line, over the profile's axial extent and `through` with
///   a margin, with an arrowhead at its `+d` end and a comb of ticks
///   on the side the profile must lie on.
/// - The arc the profile's point farthest from the axis travels, with
///   an arrowhead where the sweep stops; none for a profile on its
///   axis or an angle that is not a number.
///
/// The arrowheads and ticks are sized against `view` and need one. It
/// all goes in the marks lane, outside what a fit frames, so a fit
/// does not chase the axis's margin.
fn push_revolve_axis(
    lane: &mut marks::LegLane,
    axis: &RevolveAxisPreview,
    view: Option<screen::View>,
) {
    use pncad::geom_core::{Affine3, Point2, Point3, Vec2};
    let placement = axis.plane.placement();
    let through = Point2::from_array(axis.through);
    let (sin, cos) = axis.heading.sin_cos();
    let along = Vec2::new(cos, sin);
    // `along` a quarter turn clockwise: the side a profile must lie on.
    let side = Vec2::new(sin, -cos);
    // The world point `z` along the axis and `r` toward its side.
    let world = |z: f64, r: f64| {
        let p = through + along * z + side * r;
        placement.transform_point(Point3::new(p.x, p.y, 0.0))
    };
    let size_at = |at: Point3<f64>| view.and_then(|view| view.screen_metres_at(at, AXIS_ARROW_PX));

    let (mut low, mut high, mut reach) = (0.0_f64, 0.0_f64, 0.0_f64);
    let mut farthest: Option<(f64, f64)> = None;
    for &p in axis.loops.iter().flat_map(|lp| &lp.points) {
        let offset = Point2::from_array(p) - through;
        let (z, r) = (offset.dot(along), offset.perp_dot(along));
        low = low.min(z);
        high = high.max(z);
        reach = reach.max(z.abs()).max(r.abs());
        if farthest.is_none_or(|(_, far)| r.abs() > far.abs()) {
            farthest = Some((z, r));
        }
    }
    let margin = 0.15 * (high - low);
    let (from, to) = (low - margin, high + margin);
    let tip = world(to, 0.0);
    lane.leg(world(from, 0.0), tip);
    if let Some(size) = size_at(tip) {
        push_arrowhead(
            lane,
            tip,
            world(to - size, 0.0) - tip,
            world(to, size) - tip,
        );
    }
    for tick in 0..SIDE_TICKS {
        let z = from + (to - from) * f64::from(tick) / f64::from(SIDE_TICKS);
        if let Some(size) = size_at(world(z, 0.0)) {
            lane.leg(world(z, 0.0), world(z, 0.5 * size));
        }
    }

    // A radius at the rounding of the heading's sine and cosine is a
    // profile ON the axis, which sweeps nothing.
    let Some((z, r)) = farthest.filter(|&(_, r)| r.abs() > SWEEP_ARC_FLOOR * reach) else {
        return;
    };
    if !axis.angle.is_finite() || axis.angle == 0.0 {
        return;
    }
    let sweep = axis
        .angle
        .clamp(-core::f64::consts::TAU, core::f64::consts::TAU);
    let (centre, start) = (world(z, 0.0), world(z, r));
    let direction = world(1.0, 0.0) - world(0.0, 0.0);
    let turned =
        |angle: f64, p: Point3<f64>| Affine3::rotate_point_about_axis(centre, direction, angle, p);
    let legs = (sweep.abs() / core::f64::consts::TAU * SWEEP_ARC_LEGS_PER_TURN)
        .ceil()
        .max(2.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let count = legs as u32;
    let mut last = start;
    for step in 1..=count {
        let here = turned(sweep * f64::from(step) / legs, start);
        lane.leg(last, here);
        last = here;
    }
    if let Some(size) = size_at(last) {
        // The arc a length `size` back from its end, and a step out
        // from the axis at the end.
        let back = turned(sweep - sweep.signum() * size / r.abs(), start) - last;
        let out = turned(sweep, world(z, r + r.signum() * size)) - last;
        push_arrowhead(lane, last, back, out);
    }
}

/// An arrowhead at `tip`: two wings, `back` along the way it came and
/// [`ARROW_SPREAD`] of `across` to either side.
fn push_arrowhead(
    lane: &mut marks::LegLane,
    tip: pncad::geom_core::Point3<f64>,
    back: pncad::geom_core::Vec3<f64>,
    across: pncad::geom_core::Vec3<f64>,
) {
    lane.leg(tip, tip + back + across * ARROW_SPREAD);
    lane.leg(tip, tip + back - across * ARROW_SPREAD);
}

/// **One drawn preview's loops, placed on its plane and appended to a
/// lane** ([`push_loop`]) — its geometry, which a fit frames.
fn push_preview(lane: &mut marks::LegLane, drawn: &sketch::ProfilePreview) {
    for polyline in &drawn.loops {
        push_loop(lane, &drawn.plane, polyline);
    }
}

/// **One drawn preview's step marks, appended to a lane**: the
/// directed point at each of its steps, sized against `view` — or no
/// marks at all without one. Kept out of [`push_preview`]'s lane
/// because a mark's size is the view's, not the profile's.
///
/// **The directed point at each step.** A tip is a position and, once
/// a verb has bound one, a direction — the pair the lattice calls a
/// directed point, and the thing a person composing a chain is
/// actually reasoning about. The polyline alone shows where the chain
/// went and not where its steps ARE: an arc's flattening puts a dozen
/// indistinguishable points along one leg, which is why
/// [`PreviewLoop::vertices`] says which of them the loop owns. Each is
/// a tick through the point, square to the path, with an arrowhead
/// just ahead of it: "here, going that way". The heading is taken from
/// the polyline itself rather than from bulge arithmetic: the next
/// flattened point IS the tangent to within the chord tolerance, and a
/// second derivation of a direction is a second thing to get wrong.
///
/// **A refused loop's tip is a cross instead** ([`sketch::LoopEnd::Refused`]),
/// on the diagonals of the heading there, with no arrowhead: the chain
/// goes nowhere from there, because the step that would have taken it
/// on is the one refused. The tip is the last vertex drawn, or the
/// start where the drawn steps close. An unfinished chain's tip keeps
/// its arrowhead — it goes on from there as soon as the next step is
/// written — so the two ends a reader must tell apart are drawn apart.
fn push_step_marks(
    lane: &mut marks::LegLane,
    drawn: &sketch::ProfilePreview,
    view: Option<screen::View>,
) {
    let plane = drawn.plane;
    for polyline in &drawn.loops {
        let points = &polyline.points;
        let refused_tip = match &polyline.end {
            sketch::LoopEnd::Refused(cut) if cut.closes => polyline.vertices.first().copied(),
            sketch::LoopEnd::Refused(_) => polyline.vertices.last().copied(),
            sketch::LoopEnd::Closed | sketch::LoopEnd::Unfinished(_) => None,
        };
        for &at in &polyline.vertices {
            let here = points[at];
            let Some([dx, dy]) = heading(points, at, polyline.end.closes()) else {
                continue;
            };
            let world = plane.to_world(pncad::geom_core::Point2::from_array(here));
            let Some(tick) = view.and_then(|view| view.screen_metres_at(world, TIP_MARK_PX)) else {
                continue;
            };
            let (nx, ny) = (-dy, dx);
            let at_offset = |along: f64, across: f64| {
                [
                    here[0] + dx * along * tick + nx * across * tick,
                    here[1] + dy * along * tick + ny * across * tick,
                ]
            };
            let mut segment = |a: [f64; 2], b: [f64; 2]| push_segment(lane, &plane, a, b);
            if refused_tip == Some(at) {
                segment(at_offset(-0.5, -0.5), at_offset(0.5, 0.5));
                segment(at_offset(-0.5, 0.5), at_offset(0.5, -0.5));
                continue;
            }
            // Across the heading, never along it: a tick along the
            // chain would lie on the leg already drawn there.
            segment(at_offset(0.0, -0.5), at_offset(0.0, 0.5));
            let tip = at_offset(1.0, 0.0);
            segment(tip, at_offset(0.2, 0.45));
            segment(tip, at_offset(0.2, -0.45));
        }
    }
}

/// **The committed profiles, placed and appended to a lane**: every
/// loop [`sketch::committed`] draws, leaving out `edited` — the node
/// the edit door is previewing in its place
/// ([`crate::drafts::Drafts::edited_in_place`]). Answers how many
/// profiles could not be drawn.
pub(crate) fn push_committed(
    lane: &mut marks::LegLane,
    doc: &Doc<ProfileProgram>,
    evaluation: &Evaluation<f64>,
    chord: f64,
    edited: Option<RecipeNodeId>,
) -> usize {
    let committed = sketch::committed(doc, evaluation, chord, edited);
    for profile in &committed.drawn {
        for polyline in &profile.loops {
            push_loop(lane, &profile.plane, polyline);
        }
    }
    committed.undrawn.len()
}

/// The doors' profile previews, as each door holds one.
type Previews = ProfileDoors<Option<Result<sketch::ProfilePreview, sketch::PreviewError>>>;

/// Each door's preview that drew. A door with no preview taken, or
/// whose preview failed to replay, draws nothing; the form says why.
fn drawn_previews(previews: &Previews) -> impl Iterator<Item = &sketch::ProfilePreview> {
    previews
        .as_ref()
        .into_array()
        .into_iter()
        .filter_map(|preview| preview.as_ref()?.as_ref().ok())
}

/// **Each drawn preview's loops, appended to a lane**
/// ([`push_preview`]): the profile being authored, drawn where it
/// would land.
///
/// The form's loops are on a sketch plane, so they HAVE a place: the
/// wireframe goes in the viewport, at that place, rather than into a
/// pane of its own — a preview beside the model cannot show whether
/// the shape is the right size and in the right spot relative to what
/// is already there. Both doors of the one profile editor draw the same
/// way, in the probe mark, never the selection mark, because a preview
/// is not in the document (`EdgeOverlay::preview`). A loop whose replay
/// refused a step draws the prefix before it, and one that replayed
/// but does not VALIDATE draws whole: both are cases where looking at
/// it is the whole point.
pub(crate) fn push_previews(lane: &mut marks::LegLane, previews: &Previews) {
    for drawn in drawn_previews(previews) {
        push_preview(lane, drawn);
    }
}

/// **Each drawn preview's step marks, appended to a lane**
/// ([`push_step_marks`]).
pub(crate) fn push_previews_marks(
    lane: &mut marks::LegLane,
    previews: &Previews,
    view: Option<screen::View>,
) {
    for drawn in drawn_previews(previews) {
        push_step_marks(lane, drawn, view);
    }
}

/// Land a fold: take the camera it reached, and show the refusal that
/// stopped it.
///
/// **The one place a camera event becomes application state** — the
/// camera a fold reached and the refusal that stopped it. Both the
/// toolbar's single operations and the viewport's event stream come
/// through here, so what a fold says has one implementation.
///
/// What it says is [`frame::fold_status`]'s to decide, and a clean fold
/// says NOTHING: a camera is the fastest-moving writer the status line
/// has, and one that assigned the line on every clean fold would erase
/// the news of whichever writer shares its frame — including, on the
/// frame a document lands, the landing's own.
pub(crate) fn land(
    camera: &mut Camera,
    notices: &mut Vec<frame::Message>,
    status: &mut Option<frame::Message>,
    folded: &camera::Folded,
) {
    *camera = folded.camera;
    // Both halves of the verdict, each by its own route
    // ([`frame::deliver`]): a refused fold is NEWS and joins this
    // frame's notices, where the ranking can weigh it against whatever
    // else the frame produced; a clean fold RETIRES the camera
    // sentence and reaches the field directly, because a notice cannot
    // un-say anything.
    frame::deliver(notices, status, frame::fold_status(folded));
}

pub(crate) use composed::{Composed, frame_marks};

/// **The marks a frame draws, and the one door that makes them.** A
/// module of its own so that its fields are private even to the rest
/// of this pane: [`Composed`] has no public constructor and no
/// `Default`, so a frame cannot hand the renderer marks it did not
/// gather held picks for.
mod composed {
    use crate::display::DisplayView;
    use crate::drafts::Drafts;
    use crate::marks::{self, EdgeOverlay, Highlight};
    use crate::pickindex::PickIndex;
    use crate::session::DocSession;
    use crate::tools::Tools;

    /// **What a frame marks, as the renderer takes it** — minted only by
    /// [`frame_marks`]. A value per frame, never kept.
    #[derive(Debug)]
    pub(crate) struct Composed {
        highlight: Highlight,
        edges: EdgeOverlay,
    }

    impl Composed {
        /// The patch marks.
        pub(crate) fn highlight(&self) -> &Highlight {
            &self.highlight
        }

        /// The edge marks and the other world-space lanes.
        pub(crate) fn edges(&self) -> &EdgeOverlay {
            &self.edges
        }

        /// Add the lanes a pick implies nothing about — the committed
        /// profiles and the preview — which the pane composes itself.
        pub(crate) fn with_lanes(
            mut self,
            profiles: Vec<[f32; 3]>,
            preview: Vec<[f32; 3]>,
        ) -> Self {
            self.edges.profiles = profiles;
            self.edges.preview = preview;
            self
        }

        /// Nothing marked and no lanes, for the renderer's own rows —
        /// a test door, so a frame cannot reach it.
        #[cfg(test)]
        pub(crate) fn nothing() -> Self {
            Self {
                highlight: Highlight::default(),
                edges: EdgeOverlay::default(),
            }
        }
    }

    /// **Everything this frame marks about picks**: [`marks::compose`]
    /// over the session's selection and hover, and every pick a form or
    /// a tool holds — gathered here and nowhere else.
    ///
    /// With no index for the picture on screen there is nothing to mark
    /// against, and nothing is lit.
    ///
    /// The seated tools hold NODES, and no mark draws a held node
    /// (`work/held/a-seated-tools-held-node-is-drawn-nowhere`).
    pub(crate) fn frame_marks(
        on_screen: Option<&PickIndex>,
        display: &DisplayView,
        session: &DocSession,
        tools: &Tools,
        drafts: &Drafts,
        create_drawn: bool,
    ) -> Composed {
        let Some(index) = on_screen else {
            return Composed {
                highlight: Highlight::default(),
                edges: EdgeOverlay::default(),
            };
        };
        // One tool is open at a time, so the mate tool's two held faces
        // and the mirror tool's plane share these two slots.
        // The mate tool's edge and vertex picks are drawn as geometry.
        let mate = tools.mate().map(|tool| tool.state().picks());
        let [a, b] = match (mate, tools.mirror()) {
            (Some(picks), _) => picks.map(|pick| pick.and_then(|held| held.pick().face())),
            (None, Some(tool)) => [tool.plane(), None],
            (None, None) => [None, None],
        };
        let entities = mate.map_or([None, None], |picks| {
            picks.map(|pick| match pick.map(crate::tree::HeldPick::pick) {
                Some(crate::matetool::MatePick::Edge(edge)) => Some(marks::HeldEntity::Edge(edge)),
                Some(crate::matetool::MatePick::Vertex(vertex)) => {
                    Some(marks::HeldEntity::Vertex(vertex))
                }
                Some(crate::matetool::MatePick::Face(_)) | None => None,
            })
        });
        let held = marks::Held {
            faces: [drafts.marked_face(create_drawn), a, b],
            edges: tools.blend().and_then(crate::blend::BlendTool::held_edges),
            entities,
        };
        let (highlight, edges) =
            marks::compose(index, display, session.selection(), session.hover(), &held);
        Composed { highlight, edges }
    }
}

/// The index the picture on screen was drawn FROM, or `None` when the
/// index in hand describes some other picture.
///
/// # Why a read of the index can need this and not the evaluation
///
/// `ViewerBehavior::index` is the index for the document the session
/// has landed. `ViewerBehavior::scene` is the mesh on screen, which is
/// that index's picture on almost every frame and is NOT whenever a
/// scene rebuild refused: `ViewerApp::sync_scene` then draws nothing
/// and keys the scene to no index, so a landed index over a refused
/// rebuild stands beside a picture no index minted, and nothing retries
/// while the display and the focus hold still. The startup mesh is the
/// same shape: no index minted its corners.
///
/// Three reads care. Two of them have a pick **id** for their
/// currency:
///
/// - resolving an id the id pass produced, which is a word of the id
///   map of whichever index minted the picture's corners; and
/// - minting ids or world-space segments for the picture to draw over
///   itself, where the shader compares them against those same
///   corners.
///
/// Both are false-by-construction across two pictures, and the first
/// writes its falsehood to the status line as *the two picking paths
/// disagree* — a sentence issue #1097 §4 tells an operator to read as
/// an `R32Uint` clear fault, so a wrong subsystem gets named.
///
/// The third is the **pick**, and it asks for a different reason.
///
/// **The whole key, not the generation.** An index is keyed by a
/// [`PictureKey`] and so is its id map: a δ typed while the picture
/// stands rebuilds the index at the same generation, over a different
/// tessellation, with a different alphabet. A generation-only check
/// reads as co-identity and is not it, so the question goes to
/// [`PickIndex::current_for`], the one door that answers *does this
/// index describe this picture* — and the key is one value, so that
/// door cannot be handed half of it.
///
/// **The pick path asks too, and what it does on `None` is
/// different.** A click asks what is under the cursor in the
/// DOCUMENT, and resolves it through the index and the evaluation
/// with no id and no mesh in sight — so nothing about it is false by
/// construction, and the reason it is gated is a product ruling
/// rather than a correctness one: an answer about geometry the screen
/// is not showing selects something the user cannot see, and Ev ruled
/// on 2026-09-15 that the click is refused instead. The picture-side
/// reads skip silently on `None`, because a mark nobody can draw is
/// nothing to say; the pick path refuses TYPED, because a click is an
/// act the user made and got nothing for
/// ([`crate::pickcache::NotIndexed::AnotherPicture`]).
pub(crate) fn drawn_index(
    index: Option<&PickIndex>,
    scene_key: Option<PictureKey>,
) -> Option<&PickIndex> {
    index.filter(|index| index.current_for(scene_key))
}

/// **Whether a pick action is skipped this frame**: a hover over an
/// unchanged picture at an unmoved cursor, whose answer the session is
/// taken to hold already. A click never skips: it is an ACTION, not an
/// observation.
fn skips_the_ray(action: PickAction, step: IdStep) -> bool {
    step == IdStep::Hold && matches!(action, PickAction::Hover(_))
}

/// **Whether this frame's pick actions ask the ray at `cursor`**, and
/// so say what it refuses there through [`frame::pick_refusal`].
///
/// The loop that performs the actions skips by [`skips_the_ray`] and
/// this reads the same rule, so the two cannot disagree about which
/// frames the pick path spoke on.
fn ray_asked_at(actions: &[PickAction], step: IdStep, cursor: [f64; 2]) -> bool {
    actions.iter().any(|&action| {
        !skips_the_ray(action, step)
            && match action {
                PickAction::Hover(at) | PickAction::Select(at) => at == cursor,
                // Clearing the hover asks the ray nothing.
                PickAction::ClearHover => false,
            }
    })
}

/// Everything the ray path is asked beside the index: one cursor over
/// one evaluation, through one camera, under one display view.
#[derive(Clone, Copy)]
struct RayQuestion<'a> {
    /// The landed document `eval` answers, which the news speaks from.
    doc: &'a Doc<ProfileProgram>,
    eval: &'a Evaluation<f64>,
    camera: &'a Camera,
    viewport: ViewportSize,
    cursor: [f64; 2],
    display: &'a DisplayView,
}

/// **What the cursor comparison says this frame**, in the order it
/// happened: the ray path's refusal when nobody else said it, then
/// what the id answer amounts to ([`idpass::IdNews`]) — the two
/// picking paths' disagreement, an id the picture does not draw said
/// beside a refusal, or a readback that answered nothing, said beside
/// whatever the ray answered. Empty when there is nothing to say.
///
/// The ray answer travels to [`idpass::compare`] typed, because a
/// refusal is not a miss: that function compares nothing against a
/// refused ray path rather than reading it as "the ray named nothing".
///
/// **A refusal no pick action said this frame is said here.** The
/// refusal is the ray path's news, and it has words already
/// ([`frame::pick_refusal`]); the pick loop says them whenever it asks
/// the ray at this cursor, because `hovered_for` seeds through the
/// same un-projection and hit test as `faces_under_cursor`. It does
/// not ask on a frame it skips ([`skips_the_ray`]), and the skip reads
/// only the cursor and the picture — so a camera that moved under a
/// still cursor gets a ray nobody else asked. `ray_asked` is the pick
/// loop's own record of that ([`ray_asked_at`]), which is what keeps
/// the refusal said exactly once a frame: by the pick path when it
/// asked, here when it did not.
///
/// **The refusal does not take the id's place.** Both are news, about
/// different paths, and the frame joins its notices into one line, so
/// an id the picture has no name for rides after the refusal whoever
/// said it.
fn cursor_news(
    index: &PickIndex,
    question: RayQuestion<'_>,
    answer: Option<&idpass::IdReply>,
    outstanding: Option<u32>,
    ray_asked: bool,
) -> Vec<frame::Message> {
    let RayQuestion {
        doc,
        eval,
        camera,
        viewport,
        cursor,
        display,
    } = question;
    let from_ray = index.faces_under_cursor(eval, camera, viewport, cursor, display);
    let refusal = match &from_ray {
        Err(refusal) if !ray_asked => Some(frame::pick_refusal(refusal, (doc, eval))),
        _ => None,
    };
    let id_news = idpass::compare(index, display, answer, outstanding, from_ray.as_deref())
        .map(|news| news.notice((doc, eval)));
    refusal.into_iter().chain(id_news).collect()
}

/// Direction the light travels, world space; a unit vector over the
/// viewer's left shoulder.
const LIGHT_DIRECTION: [f32; 3] = [0.408_248_3, 0.408_248_3, -0.816_496_6];

/// Which button of the viewer's vocabulary an `egui` button denotes,
/// or `None` for one the viewer binds nothing to.
///
/// **The one place the toolkit's button set meets the viewer's**, and
/// the two are deliberately not the same set. [`input::PointerButton`]
/// names the buttons this viewer BINDS; `egui::PointerButton` names
/// the buttons a mouse can report, side buttons included.
///
/// **A side button gets `None` because there is nothing for it to
/// be.** [`input::InputMap`]'s four bindings are filled by the three
/// main buttons; no preset, preferences key or API call can name a
/// fifth; and the bindings follow mainstream CAD (`input`'s module
/// docs), which has no side-button gesture. A variant for one would
/// be a word of the vocabulary that no sentence could use — and on
/// the web backend these two are the browser's back and forward.
/// Binding them is a product decision, and it arrives as a variant on
/// `input::PointerButton`, a binding field or preset that can name it,
/// and an arm here that stops saying `None`.
///
/// **The compiler holds the SET, and nothing more.** This match names
/// every `egui::PointerButton` and the enum is not `#[non_exhaustive]`,
/// so an egui that grows a sixth button makes it non-exhaustive;
/// [`egui_buttons`] is `NUM_POINTER_BUTTONS` long, so the same upgrade
/// fails its length. A version bump is the only moment the toolkit's
/// set can change, and it is the moment both of these fire. (Were egui
/// to become `#[non_exhaustive]`, the match half dies — `_ => None` is
/// then the only shape available — and the array's length is the whole
/// hold. Say so here on the day it happens.)
///
/// **Which button pairs with which is held by a ROW, not by the
/// compiler**, and it could not be otherwise: the pairing is a naming
/// decision with nothing to derive it from. Swapping two arms here
/// type-checks and changes what every mouse does.
/// `tests::the_pairing_is_the_one_this_module_intends` is the second
/// statement of the table that makes such an edit red, and the only
/// thing in the tree that can.
fn viewer_button(button: egui::PointerButton) -> Option<PointerButton> {
    match button {
        egui::PointerButton::Primary => Some(PointerButton::Primary),
        egui::PointerButton::Secondary => Some(PointerButton::Secondary),
        egui::PointerButton::Middle => Some(PointerButton::Middle),
        egui::PointerButton::Extra1 | egui::PointerButton::Extra2 => None,
    }
}

/// Every button `egui` can report.
///
/// A function rather than a `const` item, and its length is the
/// toolkit's own `NUM_POINTER_BUTTONS`: the compiler counts this list
/// against the declaration it mirrors, so it is not a hand-maintained
/// membership list and wants no row on
/// `crates/viewer/README.md`'s roster of those.
fn egui_buttons() -> [egui::PointerButton; egui::NUM_POINTER_BUTTONS] {
    [
        egui::PointerButton::Primary,
        egui::PointerButton::Secondary,
        egui::PointerButton::Middle,
        egui::PointerButton::Extra1,
        egui::PointerButton::Extra2,
    ]
}

/// Which modifier keys this viewport binds, in the order
/// [`ViewportEvent::Drag`] carries them: shift, then alt.
///
/// **The one place the toolkit's modifier state meets the viewer's.**
/// With [`scroll_event`] and [`pinch_event`] it says what this adapter
/// reads of a frame's modifier, wheel and magnification state and what
/// it drops, and [`pointer_events`] is where the three are read. The
/// toolkit offers five modifier fields and two scroll axes; this
/// viewport binds two of the five and one of the two, and the parts it
/// does not bind are named here and discarded rather than never
/// mentioned, because a part an adapter leaves out is
/// indistinguishable downstream from a part nobody touched.
///
/// **Two, because [`input::InputMap`] has two to bind**: `alt`, which
/// turns [`input::InputMap::alt_orbit_button`]'s drag into an orbit
/// (the trackpad binding), and `shift`, which turns the orbit binding
/// into a pan. `ctrl`, `command` and `mac_cmd` are not gestures in
/// this viewport — no binding field, preset or preferences key names
/// one, and [`ViewportEvent::Drag`] carries a bare `shift` and a bare
/// `alt` and nothing else, so a ctrl-drag is a plain drag at every
/// reader downstream. Binding one arrives the way a side button's
/// would ([`viewer_button`]): a field on [`ViewportEvent::Drag`] that
/// can carry it, a binding that can name it, and a pattern here that
/// stops discarding it.
///
/// **The compiler holds the SET.** `egui::Modifiers` is a plain struct
/// and not `#[non_exhaustive]`, so a pattern over it must mention
/// every field the toolkit declares: the day egui grows a sixth
/// modifier this stops compiling, and someone answers for it in
/// writing. **A stop rather than a wall** — the error names the
/// missing field and offers `..` among its repairs, so what it buys is
/// that the answer is given HERE, on the day the field appears,
/// instead of being given by omission. It is the struct's form of the
/// exhaustive match
/// [`viewer_button`] makes over the toolkit's button enum, and it
/// fires at the same moment — a version bump, the only moment either
/// set can change. **It holds the set and nothing else**: which field
/// lands in which half of the returned pair is a naming decision with
/// nothing to derive it from, and swapping the two type-checks while
/// inverting every modified drag in the viewer.
/// `tests::a_drag_carries_the_two_modifiers_this_viewport_binds` is
/// what makes that edit red.
fn viewer_modifiers(modifiers: egui::Modifiers) -> (bool, bool) {
    let egui::Modifiers {
        shift,
        alt,
        ctrl: _,
        mac_cmd: _,
        command: _,
    } = modifiers;
    (shift, alt)
}

/// The scroll event this frame's wheel denotes, or `None` when the
/// wheel turned in no direction this viewport binds.
///
/// The other half of [`viewer_modifiers`]' decision: of the toolkit's
/// two scroll axes this viewport reads `y`, which
/// [`input::InputMap::map`] reads as the one binding a scroll has —
/// zoom.
///
/// **`x` is dropped because the viewer has no horizontal gesture.** A
/// positive `x` is content moving right: a trackpad's sideways swipe,
/// a tilt wheel, and shift+wheel on an ordinary mouse, which the
/// toolkit folds onto that axis itself (`InputOptions`'
/// `horizontal_scroll_modifier`, SHIFT by default). Zoom being the
/// only binding a scroll has, an `x` passed on would have to zoom, and
/// a sideways swipe that zooms is a worse answer than one that does
/// nothing.
///
/// **A ctrl/⌘+wheel reaches this function as a plain wheel**, because
/// [`configure_input`] turns the toolkit's wheel-zoom routing off; it
/// zooms at the one wheel rate, [`input::InputMap::zoom_rate_per_notch`].
/// An alt+wheel arrives the same way by the toolkit's own folding
/// (`vertical_scroll_modifier` puts it on `y`).
///
/// **The compiler holds both axes** the way [`viewer_modifiers`] holds
/// the five fields — `egui::Vec2` is a plain struct, so the pattern
/// names `x` in order to drop it. That upgrade hold is nominal, a
/// two-axis vector being unlikely to grow a third; what the pattern
/// buys here is that the drop is written at the site rather than
/// implied by a field access.
fn scroll_event(delta: egui::Vec2) -> Option<ViewportEvent> {
    let egui::Vec2 { y: points, x: _ } = delta;
    (points != 0.0).then(|| ViewportEvent::Scroll {
        // egui reports scroll in points; a wheel notch is
        // conventionally 50 of them.
        units: f64::from(points) / 50.0,
    })
}

/// The pinch event this frame's magnification denotes, or `None` when
/// the factor is exactly 1.
///
/// `factor` is `InputState::zoom_delta()`. Under [`configure_input`] it
/// carries only real magnifications and no wheel: `Event::Zoom`, which
/// egui-winit emits for winit's `PinchGesture` (macOS and iOS only) and
/// the web backend emits for a browser pinch, and egui's own
/// touchscreen `multi_touch` pinch. A platform that synthesises a pinch
/// as a ctrl+wheel instead — which, by this mechanism, is what a
/// Windows precision touchpad's pinch would reach egui as — arrives as
/// a wheel and so as a [`ViewportEvent::Scroll`]. The factor is passed
/// on unfiltered: one that is not a positive number is
/// [`crate::camera::apply`]'s to refuse.
fn pinch_event(factor: f32) -> Option<ViewportEvent> {
    (factor != 1.0).then(|| ViewportEvent::Pinch {
        factor: f64::from(factor),
    })
}

/// The cursor position and the pointer events one frame of the pane
/// denotes — **the one composition the viewport runs and its test
/// probe reads**.
///
/// The cursor is in physical pixels from the pane's top-left corner
/// (`hover_pos` is in screen points, so both conversions happen here
/// and everything downstream sees one convention). The events are the
/// drags and clicks [`button_events`] reads under
/// [`viewer_modifiers`]' modifiers, then — only while the pointer is
/// over the pane — the wheel's [`scroll_event`] and the
/// magnification's [`pinch_event`].
fn pointer_events(
    response: &egui::Response,
    pixels_per_point: f64,
) -> (Option<[f64; 2]>, Vec<ViewportEvent>) {
    let rect = response.rect;
    let cursor_px = response.hover_pos().map(|pos| {
        [
            f64::from(pos.x - rect.min.x) * pixels_per_point,
            f64::from(pos.y - rect.min.y) * pixels_per_point,
        ]
    });
    let (shift, alt) = response.ctx.input(|i| viewer_modifiers(i.modifiers));
    let mut events = button_events(response, shift, alt, pixels_per_point, cursor_px);
    if response.hovered() {
        response.ctx.input(|i| {
            events.extend(scroll_event(i.smooth_scroll_delta));
            events.extend(pinch_event(i.zoom_delta()));
        });
    }
    (cursor_px, events)
}

/// The toolkit input options this viewer runs with: **a wheel is
/// always a scroll**, whatever modifier is held.
///
/// egui by default routes a wheel turned with ctrl, ⌘ or `command` into
/// its zoom accumulator instead of the scroll delta, so a ctrl+wheel
/// would reach [`scroll_event`] as nothing. The viewer owns how a wheel
/// is read — a scroll is zoom here, at its own rate — so the routing is
/// switched off. egui's advice not to change `zoom_modifier` concerns
/// its own zooming widgets (`egui::Scene`, plots), and this crate uses
/// none. The option is context-wide, so its one other visible effect is
/// that a ctrl+wheel over an egui `ScrollArea` (a panel) scrolls it.
/// The application and the viewport's test probe both call this, so
/// the shipped and the tested input options are one.
pub(crate) fn configure_input(ctx: &egui::Context) {
    ctx.options_mut(|options| options.input_options.zoom_modifier = egui::Modifiers::NONE);
}

/// The drag and click events this frame's pointer denotes.
///
/// **Every button the toolkit can report is asked**, and the ones the
/// viewer binds nothing to are dropped by [`viewer_button`] rather
/// than by omission: a button missing from this loop produces no
/// event at all, which every reader downstream cannot tell from a
/// button nobody pressed.
///
/// **Which click selects is [`input::InputMap::select_button`]'s to
/// decide**, so a click of any bound button is produced and `pick`
/// reads the binding. A click carries the cursor position, so one
/// with no position is not an event.
///
/// Drags come before clicks: [`input::fold_events`] applies the camera
/// in stream order and [`input::pick_stream`] reads the cursor in
/// stream order.
fn button_events(
    response: &egui::Response,
    shift: bool,
    alt: bool,
    pixels_per_point: f64,
    cursor_px: Option<[f64; 2]>,
) -> Vec<ViewportEvent> {
    let mut drags = Vec::new();
    let mut clicks = Vec::new();
    for egui_button in egui_buttons() {
        let Some(button) = viewer_button(egui_button) else {
            continue;
        };
        if response.dragged_by(egui_button) {
            let delta = response.drag_delta();
            drags.push(ViewportEvent::Drag {
                button,
                shift,
                alt,
                delta_px: [
                    f64::from(delta.x) * pixels_per_point,
                    f64::from(delta.y) * pixels_per_point,
                ],
            });
        }
        if let Some(pos_px) = cursor_px
            && response.clicked_by(egui_button)
        {
            clicks.push(ViewportEvent::Click { button, pos_px });
        }
    }
    drags.append(&mut clicks);
    drags
}

impl ViewerBehavior<'_> {
    /// The viewport pane: read the pointer, fold it into camera
    /// operations, then queue the paint callback.
    pub(crate) fn viewport_ui(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        let pixels_per_point = f64::from(ui.ctx().pixels_per_point());
        let viewport = ViewportSize {
            width_px: f64::from(rect.width()) * pixels_per_point,
            height_px: f64::from(rect.height()) * pixels_per_point,
        };
        let Some(aspect) = viewport.aspect() else {
            // **A pane with no extent projects nothing, so it holds no
            // projection refusal.** `view_projection` is not reached
            // below, so the fault would otherwise be a claim about a
            // camera nobody is asking to project — and unlike the
            // sentence this replaced, a badge has no `Clear` to sweep
            // it. Dragging a splitter to zero is an ordinary gesture.
            //
            // This closes that arm and NOT the one where the pane is
            // not drawn at all, which needs a "the viewport did not
            // draw this frame" latch and is
            // `work/view/projection-fault-has-no-sweeper.md`.
            *self.projection_fault = None;
            return;
        };

        let (cursor_px, mut events) = pointer_events(&response, pixels_per_point);
        match cursor_px {
            Some(pos_px) => {
                events.push(ViewportEvent::Hover { pos_px });
            }
            // Only when there is a hover to clear — the session's own
            // state answers that, so nothing here shadows it.
            None if self.session.hover().is_some() => events.push(ViewportEvent::Leave),
            None => {}
        }
        // The geometry, the owed fit on what it draws, the events' fold
        // and the step marks, in that order (`frame_camera`). An owed
        // fit is taken here and nowhere else: this is the only place
        // with a real aspect to fit against.
        let profiles = self.committed_lane();
        let axis = self.revolve_axis_preview();
        let split_plane = self.split_plane_preview();
        let framed = frame_camera(CameraFrame {
            camera: self.camera,
            scene: self.scene.positions(),
            profiles,
            previews: self.profile_previews,
            axis: axis.as_ref(),
            split_plane: split_plane.as_ref(),
            fit: std::mem::take(self.pending_fit),
            input: &self.input,
            viewport,
            events: &events,
        });
        if let Some(fit) = &framed.fit {
            land(self.camera, self.notices, self.status, fit);
        }
        // ONE fold, the same one `map_stream` gives the tests.
        //
        // Landed only when the fold is a camera event — it applied a
        // camera operation, refused one, or both: the stream carries
        // cursor events too, and a stream that denotes no camera
        // operation is not a camera event. `land` is where a camera
        // event becomes application state, so running it on a frame
        // with none would make that sentence false — see
        // `frame::folded_is_a_camera_event`, which owns the rule and
        // states what it does and does not buy now that a clean fold
        // clears nothing.
        if frame::folded_is_a_camera_event(&framed.events) {
            land(self.camera, self.notices, self.status, &framed.events);
        }

        // **One movement verdict for both picking paths.** The id
        // query's bookkeeping answers "has anything changed under this
        // cursor since the last question", and the CPU ray obeys the
        // same answer: a still cursor over an unchanged picture is a
        // ray cast whose result is already known. Without it an orbit
        // drag ran a full ray cast AND a blocking GPU readback on every
        // frame, because the app pushes a `Hover` whenever the pointer
        // is inside the pane — true of every frame of a drag.
        //
        // **The subject is the pair.** What is under this cursor
        // depends on the picture the id pass reads — which a hidden
        // part changes without moving the generation — and on the
        // index that resolves its ids, which a landing over a refused
        // rebuild changes without moving the picture. `idpass::IdSubject`
        // carries the argument for both.
        let subject = idpass::IdSubject {
            revision: self.revision,
            generation: self.index.map(PickIndex::generation),
        };
        let step = self.id_log.step(cursor_px, subject);
        // **A cursor event retires what the cursor last said.** The id
        // log has just judged whether the outstanding pick question
        // still describes this cursor and this picture; a message
        // about what was under the cursor is stale on exactly that
        // judgement, so `frame::cursor_status` reads it. It only ever
        // expires today — what the cursor has to SAY is raised below,
        // where the two picking paths are compared — and it goes
        // through the policies' door all the same, because that is the
        // only door a policy's verdict fits.
        frame::deliver(self.notices, self.status, frame::cursor_status(step));

        // The cursor path: actions in, session operations out. Every
        // step of it — the un-projection, the ray service, the miss
        // rule — lives in `pickindex::PickIndex::op_for`, so this is the
        // same path a headless test drives.
        let actions = input::pick_stream(&self.input, &events);
        // **An open tool narrows the priority rule, it does not
        // re-decide it** — which tool narrows what is
        // `ToolKind::pick_kinds`, an exhaustive match beside the tool
        // vocabulary, and the narrowing travels through `hovered_for`,
        // the one door that answers what a cursor means, so a tool
        // cannot end up on a different rule.
        let kinds = self.tools.pick_kinds_choosing(self.drafts.mate_kind());
        // **The index the PICTURE was drawn from**, which is the index
        // in hand on every frame but the ones `drawn_index` exists for.
        // Every read of an index below this line asks this one:
        // the marks composited against the drawn corners' ids, the id
        // pass's answer read back through an id map, and — since Ev's
        // 2026-09-15 ruling — the pick itself, which would otherwise
        // answer about geometry the screen is not showing.
        let on_screen = drawn_index(self.index, self.scene_key.clone());
        // Read before the loop spends `actions`: whether a pick action
        // below asks the ray at this frame's cursor, and so words its
        // refusal itself ([`cursor_news`] reads it).
        let ray_asked = cursor_px.is_some_and(|cursor| ray_asked_at(&actions, step, cursor));
        if let (Some(index), Some((doc, eval))) = (on_screen, self.session.landed_pair()) {
            for action in actions {
                if skips_the_ray(action, step) {
                    continue;
                }
                match index.op_under(eval, self.camera, viewport, action, self.display, kinds) {
                    // A hover that changes nothing is not queued: an
                    // operation per frame that performs no transition
                    // is churn in the one log a test reads.
                    Ok(SessionOp::Hover(face)) if face.as_ref() == self.session.hover() => {}
                    Ok(op) => {
                        // **A click aimed at an open tool that met
                        // nothing says so**: the tool takes no pick
                        // from it, and a panel still reading `no picks
                        // yet` would otherwise be the only answer.
                        // Said here, where the click is known to be a
                        // viewport pick; a deselect in the tree is
                        // no attempt to seat anything and stays quiet.
                        if matches!(op, SessionOp::Select(crate::session::Selection::None))
                            && let Some(kind) = self.tools.open_kind()
                        {
                            self.notices.push(frame::tool_notice(
                                &crate::tools::ToolNotice::Missed { tool: kind, kinds },
                                Some((doc, eval)),
                            ));
                        }
                        self.ops.push(op);
                    }
                    Err(error) => self.notices.push(frame::pick_refusal(&error, (doc, eval))),
                }
            }
        } else if let Some(refusal) = pickcache::unindexed(&actions, self.index, self.indexing) {
            // **Not indexed yet is not a miss.** There is nobody to
            // ask about the picture on screen — no index at all while
            // one is built on its own seam, or an index in hand that
            // describes a rebuild nobody has seen — and a click that
            // quietly did nothing here is the fail-quiet this window's
            // indexing indicator would otherwise be painted over.
            //
            // `self.index` rather than `on_screen`: this arm is the
            // `else` of the currency read above, so an index reaching
            // the door is by construction one for another picture,
            // which is the fact `pickcache::unindexed` reads the
            // sentence from.
            self.notices.push(frame::unindexed_refusal(&refusal));
        }

        // What to mark, as a pure function of what is drawn, what is
        // selected and what is held. Recomputed every frame; nothing
        // retains it.
        let composed = frame_marks(
            on_screen,
            self.display,
            self.session,
            self.tools,
            self.drafts,
            self.profile_was_drawn.create,
        );
        let landed = self.session.landed_pair().map(|(doc, _)| doc);
        self.viewport_reads.held_edges_refused = composed
            .edges()
            .held_refused
            .clone()
            .map(|refused| frame::Spelled::in_landed(refused, landed));
        let marks = composed.with_lanes(framed.profiles, framed.preview);

        // **Held, not said.** A view matrix that cannot be formed is
        // true of this camera on every frame until it moves somewhere
        // one can be, so it is a read the toolbar badges
        // (`frame::projection_badge`) rather than a sentence. As a
        // sentence it was written here, AFTER the toolbar had already
        // painted the line, and `perform_batch` then ran after this
        // pane — so on every frame whose batch acted cleanly the
        // `Clear` took it before any frame drew it.
        // **The matrix the GPU will actually hold**, not the algebra's
        // own: `Camera::view_projection_f32` is the camera's door at
        // the display seam, and a projection this module can form and
        // a GPU cannot hold refuses here by the same route and into
        // the same badge as one the camera could not form at all.
        let matrix = match self.camera.view_projection_f32(aspect) {
            Ok(matrix) => {
                *self.projection_fault = None;
                matrix
            }
            Err(error) => {
                *self.projection_fault = Some(error);
                return;
            }
        };

        // The two paths' agreement, compared BY NAME
        // (`idpass::compare` says why ids are the wrong currency,
        // and records the ray-authoritative role inversion against
        // GQ6-RESURVEY §3). Reported, never resolved.
        //
        // **The ray side of this comparison is the FACE under the
        // cursor, not the hover.** An id buffer can answer with a
        // patch and nothing else, so the question both sides must
        // answer is "which patch is here"; the hover answers a
        // different one as soon as the priority rule picks an edge,
        // and feeding it would report a disagreement between two
        // questions on every frame the cursor came within
        // `EDGE_PICK_RADIUS_PX` of an edge. So the faces are re-derived
        // through `faces_under_cursor`, and only where there is a fresh
        // answer waiting for them — `compare` still owns the
        // freshness rule, this only declines to do the work when no
        // question is outstanding at all. A ray path that could not be
        // asked — no evaluation to ask it of — is no comparison either.
        // A build that asks the id pass nothing has no answer coming
        // to compare, so it casts no ray for one ([`ID_PASS_ASKED`]).
        let outstanding = self.id_log.outstanding().filter(|_| ID_PASS_ASKED);
        let said = outstanding.and_then(|_| {
            let (doc, eval) = self.session.landed_pair()?;
            let question = RayQuestion {
                doc,
                eval,
                camera: self.camera,
                viewport,
                cursor: cursor_px?,
                display: self.display,
            };
            Some(cursor_news(
                on_screen?,
                question,
                self.id_answer.latest().as_ref(),
                outstanding,
                ray_asked,
            ))
        });
        self.notices.extend(said.into_iter().flatten());

        // **The pane's own numbers at the same seam the matrix just
        // crossed.** The size the renderer is told and the point
        // scale the edge pass sizes marks with are the last two `f64`
        // the GPU sees, and they cross the one narrowing rather than
        // a cast written twice here. What this arm buys is that the
        // seam has ONE disposition, not a door that refuses over
        // there and a cast that cannot fail over here.
        //
        // **It drops the frame and says nothing, and that is the
        // whole of what it should do.** The other two dispositions of
        // this seam reach a reader — a scene refuses as a whole and a
        // projection that will not narrow badges — and this one does
        // not, deliberately: no input can reach it. `aspect()` above
        // has already declined an extent that is not positive and
        // finite, and what is left is an extent in physical pixels
        // above `f32::MAX`, about `3.4e38`, which is not a window and
        // not a scale factor. A badge here would name a state no
        // person can put the pane into, on a frame no person can see,
        // so the refusal is spent on not drawing an infinity and on
        // nothing else. An overlay LEG past the seam is the opposite
        // case and is badged (`marks::LegLane::undrawn`, and the
        // VNEWS row above): that one is authorable, and it leaves a
        // picture a person is looking at.
        let (Some(viewport_px), Some(point_scale)) = (
            [viewport.width_px, viewport.height_px].narrow(),
            pixels_per_point.narrow(),
        ) else {
            return;
        };
        let id_query = match (step, cursor_px) {
            (IdStep::Ask { serial }, Some(cursor)) if ID_PASS_ASKED => viewport
                .ndc_of(cursor)
                .and_then(|ndc| ndc.narrow())
                .map(|cursor_ndc| IdQuery {
                    cursor_ndc,
                    viewport_px,
                    serial,
                    answer: self.id_answer.clone(),
                }),
            _ => None,
        };

        // **The ground first, then the picture on it.** The pane
        // allocates its rectangle and the paint callback fills only
        // what the model covers, so without this the pixels around a
        // part are whatever the window happened to be cleared to —
        // the toolkit's colour, not the palette's. The palette states
        // it (`Theme::ground`) and this is the one place it is drawn.
        ui.painter()
            .rect_filled(rect, 0.0, chrome(self.theme.ground));
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            ViewportCallback {
                scene: Arc::clone(self.scene),
                revision: self.revision,
                view_projection: matrix,
                light_direction: LIGHT_DIRECTION,
                theme: self.theme,
                viewport_px,
                pixels_per_point: point_scale,
                marks,
                id_query,
            },
        ));
    }

    /// **The committed profiles' lane** — the value that owns the
    /// display seam's rule ([`marks::LegLane`]) rather than a bare
    /// `Vec`. What it carries beyond the segments is `undrawn()` — how
    /// many legs the seam refused — and `extent()`, what a fit frames.
    /// Nothing here holds `undrawn()`: the badge that would say so is
    /// the VNEWS row `an-overlay-leg-past-the-display-seam-is-not-badged`.
    ///
    /// From the landed evaluation, every frame, because a profile node
    /// has no body until something extrudes it and so nothing else
    /// would draw it. Left out: the profile the edit door is
    /// previewing, if any ([`ViewerBehavior::profile_edited`]) — drawn
    /// by its live preview, or by nothing while that preview has
    /// nothing to draw, and never also as it was committed. The create
    /// door's profile is not a node while it is composed, and comes to
    /// rest when its add is accepted (`Drafts::accepted`), so it has
    /// nothing to leave out.
    fn committed_lane(&mut self) -> marks::LegLane {
        let mut profiles = marks::LegLane::default();
        if let Some((doc, evaluation)) = self.session.landed_pair() {
            self.viewport_reads.profiles_undrawn = push_committed(
                &mut profiles,
                doc,
                evaluation,
                self.delta.get(),
                self.profile_edited,
            );
        }
        profiles
    }

    /// **The open revolve tool's axis over its held profile**
    /// ([`push_revolve_axis`]), from the landed evaluation and the
    /// panel's fields as they stand. `None` with no revolve tool open,
    /// no profile held, or a held profile the landed evaluation draws
    /// nothing of — the axis is in that profile's plane, so without its
    /// value there is nowhere to draw one.
    /// **The open split tool's plane** ([`crate::splitplane::preview`]),
    /// drawn under the copy its target was picked through, from the
    /// landed evaluation and the panel's fields as they stand. `None`
    /// with no split tool open, no target held, a target picked in the
    /// tree or through a copy the picture does not draw, or a normal
    /// with no direction: the panel says which
    /// ([`crate::pane::create`]'s split panel).
    pub(crate) fn split_plane_preview(&self) -> Option<crate::splitplane::PlanePreview> {
        let tool = self.tools.split()?;
        crate::splitplane::drawn(
            &crate::splitplane::Drawing {
                seats: tool.seats(),
                landed: self.session.landed_pair()?,
                display: &self.session.display_view(),
                index: self.index?,
            },
            self.drafts.split_origin,
            self.drafts.split_normal,
        )
    }

    fn revolve_axis_preview(&self) -> Option<RevolveAxisPreview> {
        let profile = self.tools.revolve()?.profile()?;
        let (doc, evaluation) = self.session.landed_pair()?;
        let drawn = sketch::committed_profile(doc, evaluation, self.delta.get(), profile)?;
        Some(RevolveAxisPreview {
            plane: drawn.plane,
            loops: drawn.loops,
            through: self.drafts.revolve_axis_through.to_array(),
            heading: self.drafts.revolve_axis_heading,
            angle: self.drafts.revolve_angle,
        })
    }
}

/// **Where a real fold meets a real landing.**
///
/// The rules [`land`] obeys are values in [`crate::frame`] and are
/// asserted there over hand-built values, because `frame` is a
/// vocabulary and has to be testable with no session in existence.
/// This module is the driver, so the rows that need one are here: the
/// wiring — does `land` still ASK — and the composition the issue
/// reproduces, a document landing with a fault on the same frame it
/// books its own re-frame.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use eframe::egui;

    use super::{
        RayQuestion, configure_input, cursor_news, drawn_extent, drawn_index, egui_buttons,
        frame_camera, land, pointer_events, push_committed, push_loop, push_preview, push_previews,
        push_segment, push_step_marks, ray_asked_at, viewer_button,
    };
    use crate::camera::{self, Camera, CameraOp, fold_recorded};
    use crate::display::DisplayView;
    use crate::frame::{self, product_badge};
    use crate::idpass::{self, IdStep};
    use crate::input::{self, InputMap, PointerButton, ViewportEvent, ViewportSize};
    use crate::marks;
    use crate::pickcache::{self, NotIndexed};
    use crate::pickindex::{IdMap, PatchNameFault, PickError, PickIndex, PictureKey};
    use crate::props::SlotValue;
    use crate::scene::{self, DisplayTolerance};
    use crate::session::{DocSession, FaceSelection, SessionOp};
    use crate::sketch::{self, PreviewLoop, ProfilePreview, ProfileShape, TIP_MARK_PX};
    use pncad::document::{Doc, ProfileProgram, SlotId};
    use pncad::geom_core::Tol;
    use pncad::select::HitTestError;

    fn framed() -> Camera {
        Camera::framing(Some(scene::plate_bounds()), 16.0 / 9.0).expect("the plate frames")
    }

    /// The re-frame `DocumentDerived::fit_camera` books when a document
    /// lands — the operation the viewport folds and lands on that very
    /// frame.
    fn the_re_frame_an_open_books() -> CameraOp {
        CameraOp::Frame {
            extent: Some(scene::plate_bounds()),
            aspect: 16.0 / 9.0,
        }
    }

    #[test]
    fn landing_a_clean_fold_does_not_clear_a_message_it_did_not_write() {
        let fit = the_re_frame_an_open_books();
        let mut camera = framed();
        let folded = fold_recorded(&camera, std::slice::from_ref(&fit));
        assert!(folded.refused.is_none(), "the re-frame applies");

        // A message about the DOCUMENT: a clean fold retires what the
        // camera said and nothing else, so this row goes red if the
        // expiry reaches past its own subject.
        let landing = frame::Message::new(
            frame::Subject::Document,
            "product: the landing's own news",
            frame::Retold::Again,
        );
        let mut status = Some(landing.clone());
        let mut notices = Vec::new();
        land(&mut camera, &mut notices, &mut status, &folded);
        assert_eq!(camera, folded.camera, "the camera still lands");
        assert_eq!(
            status,
            Some(landing),
            "and the line is not the fold's to clear"
        );
    }

    /// **A refused fold is NEWS, so it joins the frame rather than
    /// writing the line.**
    ///
    /// It used to assign the field here, which is what this sweep
    /// removed: `perform_batch` runs after the panes have drawn, so a
    /// sentence written straight to the field was erased by the same
    /// frame's accepted batch before the toolbar painted it. Going
    /// through `notices` puts it in `frame::frame_status`'s rank 2,
    /// where the batch's verdict can no longer outrank it.
    ///
    /// The older sentence on the line is left ALONE — a notice adds to
    /// what the frame has to say and takes nothing away — and the
    /// ranking is what decides between them.
    #[test]
    fn landing_a_refused_fold_is_news_and_joins_the_frames_notices() {
        let mut camera = framed();
        let refuses = CameraOp::Dolly { factor: 0.0 };
        let folded = fold_recorded(&camera, std::slice::from_ref(&refuses));
        let older =
            frame::Message::new(frame::Subject::Document, "older news", frame::Retold::Again);
        let mut status = Some(older.clone());
        let mut notices = Vec::new();
        land(&mut camera, &mut notices, &mut status, &folded);

        assert_eq!(
            notices.len(),
            1,
            "a refused fold is one notice: {notices:?}"
        );
        let raised = notices.first().expect("the notice just asserted");
        assert!(
            raised.text().contains("camera:") && raised.text().contains("dolly by a factor"),
            "{raised}"
        );
        assert_eq!(raised.subject(), frame::Subject::Camera);
        assert_eq!(
            status,
            Some(older),
            "and it writes nothing: the ranking decides, not the writer"
        );
    }

    /// **The refusal is put on the line by the RANKING, not by hand.**
    ///
    /// The two frames are composed the way the frame loop composes
    /// them: `land` on the first, then `frame::frame_status` over the
    /// notices it produced and `frame::apply` for the verdict — which
    /// is `perform_batch`'s own pair, with an empty batch because
    /// navigating acts on nothing. Reaching into `notices` for the
    /// message would assert the retirement against a sentence this row
    /// placed rather than one the frame landed, and the subject is
    /// exactly what the ranking decides: `frame::joined_subject`
    /// answers `Document` for two notices that disagree, and the
    /// `Expire(Camera)` below would then retire nothing. One notice is
    /// the case where the two answers coincide, and that coincidence
    /// is the row's premise rather than a step it skips.
    #[test]
    fn landing_a_clean_fold_retires_the_camera_refusal_it_landed_before() {
        // The item's own reproduction, through the driver: refuse a
        // camera operation, then navigate. Nothing acts, so nothing
        // clears the line, and before the subject rule the refusal
        // stayed for as long as the user orbited.
        let mut camera = framed();
        let refuses = CameraOp::Dolly { factor: 0.0 };
        let mut status = None;
        let folded = fold_recorded(&camera, std::slice::from_ref(&refuses));
        let mut notices = Vec::new();
        land(&mut camera, &mut notices, &mut status, &folded);
        assert_eq!(notices.len(), 1, "the refusal is news the frame carries");

        // The end of that frame: the ranking weighs what the frame
        // said against a batch that did nothing, and the winner
        // becomes the line.
        frame::apply(&mut status, frame::frame_status(&notices, &[], None));
        let landed = status.clone().expect("the ranking put the refusal up");
        assert_eq!(
            landed.subject(),
            frame::Subject::Camera,
            "and it is the RANKING that says what the line is about: {landed}"
        );

        let orbit = CameraOp::Orbit {
            yaw: 0.2,
            pitch: 0.1,
        };
        let folded = fold_recorded(&camera, std::slice::from_ref(&orbit));
        assert!(folded.refused.is_none(), "the orbit applies");
        let mut notices = Vec::new();
        land(&mut camera, &mut notices, &mut status, &folded);
        assert!(
            notices.is_empty(),
            "a clean fold has nothing to say: {notices:?}"
        );
        assert_eq!(
            status, None,
            "and the next camera event retires it, whatever that event says"
        );
    }

    #[test]
    fn a_gather_fault_the_tree_cannot_badge_outlives_the_open_that_raised_it() {
        // The composition the issue reproduces, end to end and through
        // the real doors: a landed pair whose product does not gather,
        // and the re-frame that landing books, on one frame.
        //
        // The fault is built by hand rather than provoked, and that is
        // the honest way round. A fault a document can REACH by an
        // ordinary edit — a placed body driven to a zero distance — is
        // a failed placement, which `frame::badge_site` sends to the feature
        // tree. The ones it keeps for this channel are not authorable
        // from the panels.
        let tol = Tol::witness();
        let (doc, extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        assert!(
            session.product_fault().is_none(),
            "the plate's own product gathers: {:?}",
            session.product_fault()
        );

        // A landing that DOES fault, reached the way a user reaches it.
        let outcome = session.perform(SessionOp::SetSlot {
            node: extrude,
            slot: SlotId::Distance,
            value: SlotValue::Continuous(0.0),
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        let fault = session.product_fault().expect("the gather refuses");
        // …and it is one the tree carries, so the badge stays silent
        // and the tree row is the channel. Both halves asserted, since
        // silence is only correct while the other channel speaks.
        assert!(
            product_badge(Some(fault), session.committed_doc()).is_none(),
            "a failed placement is the tree's to badge: {fault}"
        );
        assert!(
            session
                .tree_rows()
                .iter()
                .any(|row| matches!(row.status, crate::tree::RowStatus::Failed { .. })),
            "and the tree does badge it"
        );

        // Now the frame the issue is about: the message the landing
        // raised, and the re-frame the same landing booked. Whatever
        // the line holds when the fit is landed, the fit is not what
        // takes it away.
        let mut camera = framed();
        let fit = the_re_frame_an_open_books();
        let folded = fold_recorded(&camera, std::slice::from_ref(&fit));
        let raised = frame::Message::new(
            frame::Subject::Document,
            "product: a declared contact has no image in the graft",
            frame::Retold::Again,
        );
        let mut status = Some(raised.clone());
        let mut notices = Vec::new();
        land(&mut camera, &mut notices, &mut status, &folded);
        assert_eq!(
            status,
            Some(raised),
            "the re-frame an Open books is not news and erases none"
        );
    }

    /// A pane that senses what the viewport's does, driven by raw
    /// `egui` events.
    ///
    /// The viewport pane itself needs a GPU, a session and a scene;
    /// what the rows below are about is one part of it — the
    /// translation from what the toolkit says the pointer did to the
    /// vocabulary `input` consumes — so the probe allocates the same
    /// [`egui::Sense`] over a bare `Ui`, under the same
    /// [`configure_input`], and calls the same [`pointer_events`]
    /// [`ViewerBehavior::viewport_ui`] does.
    struct Pane {
        ctx: egui::Context,
    }

    /// Where the probe's pointer aims: the middle of its 800x600
    /// screen, which is inside the pane the probe allocates.
    const AIM: egui::Pos2 = egui::pos2(400.0, 300.0);

    /// [`AIM`] in the pane's own physical pixels — the pane fills the
    /// screen from its origin and the probe runs at one pixel per
    /// point, so the two agree.
    const AIM_PX: [f64; 2] = [400.0, 300.0];

    impl Pane {
        fn new() -> Self {
            let ctx = egui::Context::default();
            configure_input(&ctx);
            Self { ctx }
        }

        /// Run one frame and hand back the pointer events the viewport
        /// would read from it.
        fn frame(&self, events: Vec<egui::Event>) -> Vec<ViewportEvent> {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                events,
                ..Default::default()
            };
            let mut read = Vec::new();
            let out = &mut read;
            let mut output = self.ctx.clone().run_ui(input, |ui| {
                let (_, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
                let (_, events) = pointer_events(&response, 1.0);
                *out = events;
            });
            // A frame's texture upload is the caller's to apply; this
            // probe paints nothing, and dropping it unapplied panics.
            output.textures_delta.clear();
            read
        }

        /// Lay the pane out and put the pointer on it. Two empty
        /// frames first: egui interacts against the PREVIOUS frame's
        /// widget rects, so nothing is hittable until one has been
        /// laid out.
        fn reach(&self) {
            self.frame(Vec::new());
            self.frame(Vec::new());
            self.frame(vec![egui::Event::PointerMoved(AIM)]);
        }
    }

    fn button(button: egui::PointerButton, pressed: bool, pos: egui::Pos2) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    /// Press and release without moving: what the toolkit calls a
    /// click. The release frame is the one that reports it.
    fn click(pane: &Pane, egui_button: egui::PointerButton) -> Vec<ViewportEvent> {
        pane.reach();
        pane.frame(vec![button(egui_button, true, AIM)]);
        pane.frame(vec![button(egui_button, false, AIM)])
    }

    /// Press and move: what the toolkit calls a drag. The moving frame
    /// is the one that reports it.
    fn drag(pane: &Pane, egui_button: egui::PointerButton) -> Vec<ViewportEvent> {
        drag_modified(pane, egui_button, egui::Modifiers::NONE)
    }

    /// The same drag with `modifiers` held.
    ///
    /// The modifier change is its own event because that is how the
    /// toolkit carries one: `InputState` keeps the modifier state
    /// across frames and only `ModifiersChanged` moves it, so a
    /// modifier named on a pointer event alone never reaches
    /// `i.modifiers`.
    fn drag_modified(
        pane: &Pane,
        egui_button: egui::PointerButton,
        modifiers: egui::Modifiers,
    ) -> Vec<ViewportEvent> {
        pane.reach();
        pane.frame(vec![
            egui::Event::ModifiersChanged(modifiers),
            egui::Event::PointerButton {
                pos: AIM,
                button: egui_button,
                pressed: true,
                modifiers,
            },
        ]);
        pane.frame(vec![egui::Event::PointerMoved(AIM + egui::vec2(40.0, 0.0))])
    }

    /// A wheel turn small enough that the toolkit hands it straight
    /// over: `InputState` smooths a wheel of eight points or more
    /// across several frames, and this probe reads one frame.
    const WHEEL_POINTS: f32 = 4.0;

    /// Turn the wheel over the pane with `modifiers` held, and hand
    /// back what the viewport read from that frame.
    fn turn_wheel(pane: &Pane, modifiers: egui::Modifiers) -> Vec<ViewportEvent> {
        pane.reach();
        pane.frame(vec![
            egui::Event::ModifiersChanged(modifiers),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, WHEEL_POINTS),
                phase: egui::TouchPhase::Move,
                modifiers,
            },
        ])
    }

    /// The scroll a plain [`turn_wheel`] denotes.
    fn a_wheels_scroll() -> ViewportEvent {
        ViewportEvent::Scroll {
            units: f64::from(WHEEL_POINTS) / 50.0,
        }
    }

    /// A δ, coarse enough to index the plate quickly.
    fn a_delta(mm: f64) -> DisplayTolerance {
        DisplayTolerance::new(mm * 1.0e-3).expect("a positive δ")
    }

    /// The plate, landed, plus the index of its landed evaluation at
    /// `delta`.
    fn plate_index(session: &DocSession, delta: DisplayTolerance) -> PickIndex {
        let (doc, eval) = session.landed_pair().expect("the inline seam lands");
        let generation = session
            .landed_generation()
            .expect("a landed evaluation has a generation");
        PickIndex::build(
            doc,
            eval,
            PictureKey::of(generation, delta, crate::world::Shown::none()),
            session.tol(),
        )
        .expect("the plate indexes")
    }

    /// **The picture's alphabet is its whole key — generation, δ and
    /// the shows — and the guard holds the δ part a generation-only
    /// check would drop.**
    ///
    /// The half a generation-only check would drop is δ: a δ typed
    /// while the document stands rebuilds the index at the SAME
    /// generation over a different tessellation, so the id map is a
    /// different alphabet under an identical generation. A guard that
    /// compared generations would pass the cross pairing below and read
    /// as co-identity while checking something else — which is the
    /// shape this unit was sent to remove, not to re-mint.
    #[test]
    fn the_drawn_index_is_the_one_whose_generation_and_delta_the_picture_carries() {
        let tol = Tol::witness();
        let (doc, extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        let mut session = DocSession::inline(doc, tol);
        session.pump();

        let coarse = plate_index(&session, a_delta(0.5));
        let fine = plate_index(&session, a_delta(0.05));
        let key = PickIndex::key;
        assert_eq!(
            coarse.generation(),
            fine.generation(),
            "both index the same landed evaluation, so only δ separates them"
        );
        assert_ne!(
            coarse.key().delta(),
            fine.key().delta(),
            "and δ does separate them"
        );

        assert!(
            drawn_index(Some(&coarse), Some(key(&coarse))).is_some(),
            "the index the picture was built from IS the drawn index"
        );
        assert!(
            drawn_index(Some(&fine), Some(key(&coarse))).is_none(),
            "an index at another δ did not mint this picture's ids"
        );

        // The other half, over the same predicate: an edit lands a new
        // generation, and the index of it is not the index of the
        // picture still on screen.
        let outcome = session.perform(SessionOp::SetSlot {
            node: extrude,
            slot: SlotId::Distance,
            value: SlotValue::Continuous(0.004),
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        let edited = plate_index(&session, a_delta(0.5));
        assert_ne!(
            edited.generation(),
            coarse.generation(),
            "the edit landed a new generation"
        );
        assert!(
            drawn_index(Some(&edited), Some(key(&coarse))).is_none(),
            "an index of the edited document did not mint the old picture's ids"
        );
    }

    /// **The false diagnosis this guard exists to stop, shown to be a
    /// real sentence, and shown to be unreachable through the guard.**
    ///
    /// A picture no index minted ids for — the startup mesh, whose
    /// corners all carry [`IdMap::NOTHING`] — makes the id pass answer
    /// *nothing* everywhere. Compared against a ray that names a face,
    /// that is a disagreement, and [`idpass::Disagreement`] writes it to
    /// the status line as *the two picking paths disagree*, which issue
    /// #1097 §4 tells an operator to read as an `R32Uint` clear fault.
    ///
    /// So the first assertion is that the sentence really is produced
    /// by the pairing, and the second is that `drawn_index` never hands
    /// the comparison that pairing.
    #[test]
    fn a_picture_no_index_minted_ids_for_is_not_compared_against_one() {
        let tol = Tol::witness();
        let (doc, _extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let index = plate_index(&session, a_delta(0.5));

        let id = index.ids().ids().next().expect("the plate draws patches");
        let named = index
            .face_of(id)
            .expect("an id of this index names its patch cleanly");

        let serial = 7u32;
        let nothing = idpass::IdReply {
            serial,
            read: Ok(IdMap::NOTHING),
        };
        assert_eq!(
            idpass::compare(
                &index,
                &DisplayView::none(),
                Some(&nothing),
                Some(serial),
                Ok(std::slice::from_ref(&named)),
            ),
            Some(idpass::IdNews::Disagreement(idpass::Disagreement {
                from_gpu: idpass::IdAnswer::Refused(PatchNameFault::Nothing),
                from_ray: vec![named],
            })),
            "nothing-under-the-cursor against a named face is a disagreement"
        );

        assert!(
            drawn_index(Some(&index), None).is_none(),
            "a picture with no index behind it is compared against no index"
        );
    }

    /// **A refused ray beside an id-pass face**: the plate's index, an
    /// evaluation of ANOTHER document its hit test refuses before
    /// reading a table (a distinct identity; a twin recipe derives the
    /// same one), a framed cursor, and an id-pass reply naming a face
    /// the plate really draws.
    struct RefusedRay {
        index: PickIndex,
        foreign: DocSession,
        camera: Camera,
        pane: ViewportSize,
        cursor: [f64; 2],
        named: FaceSelection,
        /// The id the named face is drawn under.
        id: u32,
        serial: u32,
        answer: idpass::IdReply,
    }

    fn refused_ray() -> RefusedRay {
        let tol = Tol::witness();
        let (doc, _extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let index = plate_index(&session, a_delta(0.5));
        let mut foreign =
            DocSession::inline(Doc::<ProfileProgram>::empty_derived("another", tol), tol);
        foreign.pump();
        let id = index.ids().ids().next().expect("the plate draws patches");
        let named = index
            .face_of(id)
            .expect("an id of this index names its patch cleanly");
        let serial = 7u32;
        RefusedRay {
            index,
            foreign,
            camera: framed(),
            pane: ViewportSize {
                width_px: 1600.0,
                height_px: 900.0,
            },
            cursor: [800.0, 450.0],
            named,
            id,
            serial,
            answer: idpass::IdReply {
                serial,
                read: Ok(id),
            },
        }
    }

    /// **A ray path that REFUSED is not a ray path that named
    /// nothing**, at the comparison itself: the same id-pass face is no
    /// verdict against the refusal and a disagreement against an empty
    /// answer, so the type tells the two apart.
    #[test]
    fn a_refused_ray_path_is_no_verdict_against_a_named_face() {
        let fixture = refused_ray();
        let foreign = fixture
            .foreign
            .evaluation()
            .expect("the other document lands");
        let refusal = fixture
            .index
            .faces_under_cursor(
                foreign,
                &fixture.camera,
                fixture.pane,
                fixture.cursor,
                &DisplayView::none(),
            )
            .expect_err("an evaluation of another document is refused");
        assert!(
            matches!(
                refusal,
                PickError::HitTest(HitTestError::EvaluationOfAnotherDocument { .. })
            ),
            "the planted refusal is the kernel declining: {refusal:?}"
        );
        let (index, answer, serial) = (&fixture.index, Some(&fixture.answer), Some(fixture.serial));
        let view = &DisplayView::none();
        assert_eq!(
            idpass::compare(index, view, answer, serial, Err(&refusal)),
            None,
            "a refused ray path is compared against nothing"
        );
        assert_eq!(
            idpass::compare(index, view, answer, serial, Ok(&[])),
            Some(idpass::IdNews::Disagreement(idpass::Disagreement {
                from_gpu: idpass::IdAnswer::Named(fixture.named),
                from_ray: Vec::new(),
            })),
            "while a ray path that answered nothing is contradicted by the face"
        );
    }

    /// **A failed readback is said as the device's fault, beside every
    /// ray answer, and never as the two paths disagreeing.** The rows
    /// the fault would have produced folded into [`IdMap::NOTHING`] sit
    /// beside it: a disagreement over a named face, silence over empty
    /// space, and no verdict beside a refusal.
    #[test]
    fn a_failed_readback_is_the_devices_fault_beside_any_ray_answer() {
        let fixture = refused_ray();
        let foreign = fixture
            .foreign
            .evaluation()
            .expect("the other document lands");
        let display = DisplayView::none();
        let refusal = fixture
            .index
            .faces_under_cursor(
                foreign,
                &fixture.camera,
                fixture.pane,
                fixture.cursor,
                &display,
            )
            .expect_err("an evaluation of another document is refused");
        let (index, serial) = (&fixture.index, fixture.serial);
        let named = std::slice::from_ref(&fixture.named);
        let fault = idpass::ReadbackFault::Wait("Timeout".to_owned());
        let failed = idpass::IdReply {
            serial,
            read: Err(fault.clone()),
        };
        let folded = idpass::IdReply {
            serial,
            read: Ok(IdMap::NOTHING),
        };
        let none: &[FaceSelection] = &[];
        let rays = [
            (
                "a ray that names a face",
                Ok(named),
                Some(idpass::IdNews::Disagreement(idpass::Disagreement {
                    from_gpu: idpass::IdAnswer::Refused(PatchNameFault::Nothing),
                    from_ray: named.to_vec(),
                })),
            ),
            ("a ray that names nothing", Ok(none), None),
            ("a ray that refused", Err(&refusal), None),
        ];
        for (ray, from_ray, as_nothing) in rays {
            assert_eq!(
                idpass::compare(index, &display, Some(&failed), Some(serial), from_ray),
                Some(idpass::IdNews::NoReadback(fault.clone())),
                "beside {ray}, a failed readback is the device's fault"
            );
            assert_eq!(
                idpass::compare(index, &display, Some(&folded), Some(serial), from_ray),
                as_nothing,
                "beside {ray}, the same fault read as the clear value says something else"
            );
        }
        for (ray, from_ray) in [
            ("a ray that names a face", Ok(named)),
            ("a refusal", Err(&refusal)),
        ] {
            assert_eq!(
                idpass::compare(index, &display, Some(&failed), Some(serial + 1), from_ray),
                None,
                "beside {ray}, a fault answering another query is stale like any reply"
            );
        }
        let said = idpass::IdNews::NoReadback(fault).to_string();
        assert!(
            said.contains("no answer") && said.contains("Timeout") && !said.contains("disagree"),
            "the sentence names the device's fault and no disagreement: {said}"
        );
    }

    /// **A ray refusal is said exactly once a frame — by the pick path
    /// when it asked, by the comparison when it did not — and never as
    /// a disagreement.** This drives the pane's own wiring
    /// (`ray_asked_at`, `cursor_news`) across the two frames the id
    /// log distinguishes.
    ///
    /// The second frame is the one the pick path skips: the camera
    /// orbits under a still cursor, which the id log reads as `Hold`,
    /// so no hover asks the ray, while the comparison asks it through
    /// the moved camera. The refusal a camera move ALONE can bring on
    /// is the hit test's unnamed-entity arm, which nothing in this
    /// crate can plant; the evaluation of another document stands in
    /// for it, and what this row pins is who says a refusal on a frame
    /// nobody else asked the ray, not which refusal it is.
    #[test]
    fn a_ray_refusal_the_pick_path_did_not_ask_for_is_said_by_the_comparison() {
        let fixture = refused_ray();
        let (foreign_doc, foreign) = fixture
            .foreign
            .landed_pair()
            .expect("the other document lands");
        let display = DisplayView::none();
        let at = fixture.cursor;
        let actions = [input::PickAction::Hover(at)];
        let subject = idpass::IdSubject {
            revision: 1,
            generation: Some(fixture.index.generation()),
        };
        let mut log = idpass::IdQueryLog::new();

        // The cursor arrives: a new question, and the pick path asks
        // the ray there — so it says the refusal, and the comparison
        // says nothing.
        let arrived = log.step(Some(at), subject);
        let serial = match arrived {
            IdStep::Ask { serial } => Some(serial),
            IdStep::Hold | IdStep::Void => None,
        }
        .expect("a cursor arriving is a new question");
        // The id pass's answer to THIS question, naming a face: fresh,
        // so a ray answer read as empty would be a disagreement.
        let answer = idpass::IdReply {
            serial,
            read: Ok(fixture.id),
        };
        assert!(
            ray_asked_at(&actions, arrived, at),
            "the hover asks the ray"
        );
        let asked = RayQuestion {
            doc: foreign_doc,
            eval: foreign,
            camera: &fixture.camera,
            viewport: fixture.pane,
            cursor: at,
            display: &display,
        };
        assert_eq!(
            cursor_news(
                &fixture.index,
                asked,
                Some(&answer),
                log.outstanding(),
                true
            ),
            Vec::new(),
            "the pick path said this refusal; the comparison adds nothing"
        );

        // The camera orbits; the cursor does not move.
        let moved = camera::apply(
            &fixture.camera,
            &CameraOp::Orbit {
                yaw: 0.3,
                pitch: 0.1,
            },
        )
        .expect("the orbit applies");
        let held = log.step(Some(at), subject);
        assert_eq!(held, IdStep::Hold, "the id log does not read the camera");
        assert!(
            !ray_asked_at(&actions, held, at),
            "so the hover skips the frame and nobody on the pick path asks the ray"
        );
        let unasked = RayQuestion {
            camera: &moved,
            ..asked
        };
        let refusal = fixture
            .index
            .faces_under_cursor(foreign, &moved, fixture.pane, at, &display)
            .expect_err("the moved ray is refused");
        assert_eq!(
            cursor_news(
                &fixture.index,
                unasked,
                Some(&answer),
                log.outstanding(),
                false
            ),
            vec![frame::pick_refusal(&refusal, (foreign_doc, foreign))],
            "the comparison says the refusal in the pick path's own words"
        );
    }

    /// **What the pane says about an id the drawn index never
    /// assigned**, through its own wiring (`cursor_news`) over a real
    /// plate: the id, the ray's answer at the first cursor whose ray
    /// answer `wanted` accepts, and the news.
    ///
    /// An unassigned id is what a corrupt readback looks like. Read as
    /// the clear value, it was misreported both ways: as *id buffer
    /// nothing* against a ray that named a face, and as silent
    /// agreement against a ray that named nothing. The two rows below
    /// each ask one of those cursors. The ray's faces come back in
    /// words, as the landed document speaks them within its evaluation.
    fn unassigned_id_news(
        wanted: impl Fn(&[FaceSelection]) -> bool,
    ) -> (u32, Vec<FaceSelection>, Vec<String>, Vec<frame::Message>) {
        let tol = Tol::witness();
        let (doc, _extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let index = plate_index(&session, a_delta(0.5));
        let (landed, eval) = session.landed_pair().expect("the plate lands");
        let unassigned = index.ids().ids().max().expect("the plate draws patches") + 1;
        assert!(
            index.name_of(unassigned) == Err(PatchNameFault::Unassigned { id: unassigned }),
            "an id past the last assigned one is no entry of this index"
        );
        let camera = framed();
        let pane = ViewportSize {
            width_px: 1600.0,
            height_px: 900.0,
        };
        let display = DisplayView::none();
        // The pane's corner, then out from the plate's centre (its
        // hole) along the horizontal.
        let (cursor, from_ray) = std::iter::once([1.0, 1.0])
            .chain((0..16).map(|step| [800.0 - 40.0 * f64::from(step), 450.0]))
            .map(|cursor| {
                let faces = index
                    .faces_under_cursor(eval, &camera, pane, cursor, &display)
                    .expect("the plate's ray path answers");
                (cursor, faces)
            })
            .find(|(_, faces)| wanted(faces))
            .expect("some cursor's ray answer is the one asked for");

        let mut log = idpass::IdQueryLog::new();
        let subject = idpass::IdSubject {
            revision: 1,
            generation: Some(index.generation()),
        };
        let serial = match log.step(Some(cursor), subject) {
            IdStep::Ask { serial } => Some(serial),
            IdStep::Hold | IdStep::Void => None,
        }
        .expect("a cursor arriving is a new question");
        let question = RayQuestion {
            doc: landed,
            eval,
            camera: &camera,
            viewport: pane,
            cursor,
            display: &display,
        };
        let answer = idpass::IdReply {
            serial,
            read: Ok(unassigned),
        };
        let news = cursor_news(&index, question, Some(&answer), log.outstanding(), true);
        let spoken = from_ray
            .iter()
            .map(|face| {
                let by = pncad::document::Speaker::of(landed).within(eval);
                pncad::document::Said(&crate::tree::PickedName::face(face), by).to_string()
            })
            .collect();
        (unassigned, from_ray, spoken, news)
    }

    /// Over what the ray calls empty space, an unassigned id is said,
    /// not agreed with.
    #[test]
    fn an_unassigned_id_over_empty_space_is_said_as_that_id() {
        let (id, _, _, news) = unassigned_id_news(<[FaceSelection]>::is_empty);
        assert_eq!(
            news,
            vec![frame::Message::new(
                frame::Subject::Cursor,
                format!(
                    "picking paths disagree at the cursor: id buffer id {id}, \
                     which this index never assigned, ray nothing"
                ),
                frame::Retold::Again,
            )],
        );
    }

    /// Against a ray that named a face, an unassigned id is said as the
    /// id, not as the id buffer naming nothing.
    #[test]
    fn an_unassigned_id_against_a_named_face_is_said_as_that_id() {
        let (id, from_ray, spoken, news) = unassigned_id_news(|faces| faces.len() == 1);
        let (face, said) = from_ray
            .first()
            .zip(spoken.first())
            .expect("the helper returns the one-face answer it was asked for");
        assert!(
            said.contains("Extrude") && !said.contains(&format!("{:?}", face.name.path)),
            "the face's minter is said as the landed document holds it, and no path: {said}"
        );
        let texts: Vec<&str> = news.iter().map(frame::Message::text).collect();
        assert_eq!(
            texts,
            [format!(
                "picking paths disagree at the cursor: id buffer id {id}, \
                 which this index never assigned, ray {said}"
            )],
            "an unassigned id against a named face is a disagreement, said once"
        );
    }

    /// **An id the index never assigned is said beside a refused ray**,
    /// through the pane's wiring, on both frames the refusal can be
    /// said on: after the pick path's own refusal when it asked the
    /// ray, and after the comparison's when it did not. A refused ray
    /// makes no claim for the id to contradict, and the id needs none:
    /// no patch of this picture is drawn under it.
    #[test]
    fn an_unassigned_id_beside_a_refused_ray_is_said_as_that_id() {
        let fixture = refused_ray();
        let (foreign_doc, foreign) = fixture
            .foreign
            .landed_pair()
            .expect("the other document lands");
        let display = DisplayView::none();
        let unassigned = fixture
            .index
            .ids()
            .ids()
            .max()
            .expect("the plate draws patches")
            + 1;
        assert!(
            fixture.index.name_of(unassigned) == Err(PatchNameFault::Unassigned { id: unassigned }),
            "an id past the last assigned one is no entry of this index"
        );
        let refusal = fixture
            .index
            .faces_under_cursor(
                foreign,
                &fixture.camera,
                fixture.pane,
                fixture.cursor,
                &display,
            )
            .expect_err("an evaluation of another document is refused");
        let mut log = idpass::IdQueryLog::new();
        let subject = idpass::IdSubject {
            revision: 1,
            generation: Some(fixture.index.generation()),
        };
        let serial = match log.step(Some(fixture.cursor), subject) {
            IdStep::Ask { serial } => Some(serial),
            IdStep::Hold | IdStep::Void => None,
        }
        .expect("a cursor arriving is a new question");
        let answer = idpass::IdReply {
            serial,
            read: Ok(unassigned),
        };
        let question = RayQuestion {
            doc: foreign_doc,
            eval: foreign,
            camera: &fixture.camera,
            viewport: fixture.pane,
            cursor: fixture.cursor,
            display: &display,
        };
        let said = frame::Message::new(
            frame::Subject::Cursor,
            format!(
                "id buffer at the cursor: id {unassigned}, which no patch of this picture draws"
            ),
            frame::Retold::Again,
        );
        assert_eq!(
            cursor_news(
                &fixture.index,
                question,
                Some(&answer),
                log.outstanding(),
                true
            ),
            vec![said.clone()],
            "the pick path said the refusal; the comparison says the id"
        );
        assert_eq!(
            cursor_news(
                &fixture.index,
                question,
                Some(&answer),
                log.outstanding(),
                false
            ),
            vec![frame::pick_refusal(&refusal, (foreign_doc, foreign)), said],
            "nobody else said the refusal, so the comparison says it, and the id after it"
        );
    }

    /// **The id buffer's clear value beside a refused ray is no
    /// verdict**: *nothing* is a claim only a ray answer could
    /// contradict, so on the frame the pick path asked the comparison
    /// adds nothing, and on the frame it did not it says the refusal
    /// alone.
    #[test]
    fn nothing_beside_a_refused_ray_is_no_verdict() {
        let fixture = refused_ray();
        let (foreign_doc, foreign) = fixture
            .foreign
            .landed_pair()
            .expect("the other document lands");
        let display = DisplayView::none();
        let refusal = fixture
            .index
            .faces_under_cursor(
                foreign,
                &fixture.camera,
                fixture.pane,
                fixture.cursor,
                &display,
            )
            .expect_err("an evaluation of another document is refused");
        let answer = idpass::IdReply {
            serial: fixture.serial,
            read: Ok(IdMap::NOTHING),
        };
        let question = RayQuestion {
            doc: foreign_doc,
            eval: foreign,
            camera: &fixture.camera,
            viewport: fixture.pane,
            cursor: fixture.cursor,
            display: &display,
        };
        let serial = Some(fixture.serial);
        assert_eq!(
            cursor_news(&fixture.index, question, Some(&answer), serial, true),
            Vec::new(),
            "the pick path said the refusal, and nothing beside it is no news"
        );
        assert_eq!(
            cursor_news(&fixture.index, question, Some(&answer), serial, false),
            vec![frame::pick_refusal(&refusal, (foreign_doc, foreign))],
            "the comparison says the refusal alone"
        );
    }

    /// **An id-buffer word that lands on a hidden placement's patch is an
    /// id this picture does not draw**, not the face the index names it:
    /// the index keeps a hidden part's ids and names them, and the
    /// picture draws none of them, so reading the word as that name
    /// would say the id buffer saw a face that is not on screen. Driven
    /// through the pane's wiring beside a ray that answered, and at the
    /// comparison beside one that refused.
    #[test]
    fn a_hidden_roots_patch_id_is_said_as_an_id_this_picture_does_not_draw() {
        let tol = Tol::witness();
        let (doc, _extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        assert_eq!(doc.placements().len(), 1, "the plate is placed once");
        let copy = *doc.placements().first().expect("the plate is placed");
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let index = plate_index(&session, a_delta(0.5));
        let (landed, eval) = session.landed_pair().expect("the plate lands");
        let id = *index
            .ids_of_node(copy)
            .first()
            .expect("the plate's placement draws patches");
        let named = index
            .face_of(id)
            .expect("an id of this index names its patch cleanly");
        let hidden = DisplayView {
            hidden_drawn: vec![crate::world::Drawn::whole(copy)],
            ..DisplayView::none()
        };
        let camera = framed();
        let pane = ViewportSize {
            width_px: 1600.0,
            height_px: 900.0,
        };
        let cursor = [800.0, 450.0];
        assert!(
            index
                .faces_under_cursor(eval, &camera, pane, cursor, &hidden)
                .expect("the plate's ray path answers")
                .is_empty(),
            "with its only placement hidden the plate is out of the ray path's answer"
        );

        let serial = 7u32;
        let answer = idpass::IdReply {
            serial,
            read: Ok(id),
        };
        assert_eq!(
            idpass::compare(
                &index,
                &DisplayView::none(),
                Some(&answer),
                Some(serial),
                Ok(&[])
            ),
            Some(idpass::IdNews::Disagreement(idpass::Disagreement {
                from_gpu: idpass::IdAnswer::Named(named),
                from_ray: Vec::new(),
            })),
            "shown, the same word is the face the index names"
        );

        let question = RayQuestion {
            doc: landed,
            eval,
            camera: &camera,
            viewport: pane,
            cursor,
            display: &hidden,
        };
        assert_eq!(
            cursor_news(&index, question, Some(&answer), Some(serial), true),
            vec![frame::Message::new(
                frame::Subject::Cursor,
                format!(
                    "picking paths disagree at the cursor: id buffer id {id}, \
                     which no patch of this picture draws, ray nothing"
                ),
                frame::Retold::Again,
            )],
            "hidden, it is an id this picture does not draw"
        );

        let fixture = refused_ray();
        let foreign = fixture
            .foreign
            .evaluation()
            .expect("the other document lands");
        // The refusal is planted where a ray is still asked of a drawn
        // part: `compare` reads only that the ray refused.
        let refusal = fixture
            .index
            .faces_under_cursor(foreign, &camera, pane, cursor, &DisplayView::none())
            .expect_err("an evaluation of another document is refused");
        assert_eq!(
            idpass::compare(&index, &hidden, Some(&answer), Some(serial), Err(&refusal)),
            Some(idpass::IdNews::BesideRefusal { id }),
            "and beside a refused ray it is still said"
        );
    }

    /// **A click over a picture the index in hand did not draw is
    /// refused, typed** — Ev's ruling, 2026-09-15.
    ///
    /// This row is the PAIR of predicates the pick path composes, over
    /// a real session: `drawn_index` answers `None` for an index that
    /// did not mint the picture's corners, and the index it declined
    /// is what `pickcache::unindexed` reads
    /// [`NotIndexed::AnotherPicture`] from. Each assertion can fail for
    /// the reason the rule exists: a generation-only guard passes the
    /// cross pairing, and a door that ignored the held index says *the
    /// picture has no pick index* over one that plainly has.
    ///
    /// **What it does not assert is the wiring**, and no row in this
    /// crate can. `ViewerBehavior::viewport_ui` is a private method
    /// over an `egui::Ui` that paints through a wgpu callback and
    /// borrows twenty-odd fields of the application, so it is driven by
    /// nothing headless — the probe below reaches only the event
    /// translation, for the same reason. That the pick path asks
    /// `drawn_index` rather than the index in hand is held by its one
    /// call site being one line, the way
    /// `crates/viewer/tests/panel_display.rs` records for the
    /// parameter field's widget.
    #[test]
    fn a_click_over_a_picture_the_index_did_not_draw_is_refused_typed() {
        let tol = Tol::witness();
        let (doc, _extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        let mut session = DocSession::inline(doc, tol);
        session.pump();

        let drawn = plate_index(&session, a_delta(0.5));
        let landed = plate_index(&session, a_delta(0.05));
        let picture = drawn.key();
        assert!(
            drawn_index(Some(&landed), Some(picture.clone())).is_none(),
            "the index in hand describes a rebuild the picture is not"
        );

        let click = [input::PickAction::Select([10.0, 10.0])];
        assert_eq!(
            pickcache::unindexed(&click, Some(&landed), pickcache::IndexSeam::Idle),
            Some(NotIndexed::AnotherPicture),
            "so the click gets the picture's answer, which is a refusal",
        );
        assert_eq!(
            pickcache::unindexed(
                &[
                    input::PickAction::Hover([10.0, 10.0]),
                    input::PickAction::ClearHover,
                ],
                Some(&landed),
                pickcache::IndexSeam::Idle,
            ),
            None,
            "and a hover, pushed every frame the pointer is inside the \
             pane, is not news",
        );

        // The other side of the same door: the index that DID draw the
        // picture answers, so nothing is refused over it.
        assert!(
            drawn_index(Some(&drawn), Some(picture)).is_some(),
            "the index that minted the picture's corners still answers",
        );
    }

    /// **[`egui_buttons`] is every toolkit button exactly once, and
    /// that is a THEOREM rather than a reading of the list.**
    ///
    /// Three facts compose to it. The array's length is
    /// `egui::NUM_POINTER_BUTTONS`, which the compiler checks against
    /// the declaration. [`viewer_button`]'s exhaustive match means the
    /// enum has exactly that many variants — an egui that adds one
    /// without raising the constant reds there. And this row says the
    /// entries are pairwise distinct. `n` distinct members of an
    /// `n`-member set are all of them, so the array is a permutation
    /// of the enum: nothing missing, nothing doubled.
    ///
    /// **Without this row the length is the only hold, and length
    /// alone is not membership.** `[Primary; NUM_POINTER_BUTTONS]`
    /// compiles, and the rows below derive their expectations from
    /// [`egui_buttons`] itself, so a doubled entry asks one button
    /// twice and another never — silently, for any pair the viewer
    /// binds nothing to. A complete list held only by its length is
    /// the class this fix was sent to close, and it would have been
    /// re-minted here.
    #[test]
    fn the_toolkits_buttons_are_each_asked_exactly_once() {
        let buttons = egui_buttons();
        for (index, button) in buttons.iter().enumerate() {
            for other in &buttons[index + 1..] {
                assert_ne!(button, other, "{buttons:?} asks a button twice");
            }
        }
    }

    /// **Every button the toolkit can report reaches the pane, and
    /// what [`viewer_button`] says of it is what comes out.**
    ///
    /// This row is over the PLUMBING, and its reach is exactly that.
    /// Buttons come from [`egui_buttons`] and the expectation from
    /// [`viewer_button`], so a button the loop stopped polling fails
    /// here — the defect this row was written for, where a button
    /// produced no event and no reader could tell that from a button
    /// nobody pressed.
    ///
    /// **What it cannot catch is a change to `viewer_button` itself**,
    /// because both sides of the assertion move with it: give `Extra1`
    /// an arm and this row stays green, having asked for the new
    /// answer and got it. The decision that function encodes is held
    /// by [`the_pairing_is_the_one_this_module_intends`] instead, and
    /// the two rows are complementary rather than overlapping.
    #[test]
    fn every_toolkit_button_the_adapter_binds_produces_its_click() {
        for egui_button in egui_buttons() {
            let events = click(&Pane::new(), egui_button);
            let expected: Vec<ViewportEvent> = viewer_button(egui_button)
                .map(|button| ViewportEvent::Click {
                    button,
                    pos_px: AIM_PX,
                })
                .into_iter()
                .collect();
            assert_eq!(events, expected, "clicking {egui_button:?}");
        }
    }

    /// The same plumbing over drags, with the same reach and the same
    /// blind spot: the three main buttons already dragged before this
    /// unit, so what it adds is the side buttons, whose events stop at
    /// [`viewer_button`]'s `None` rather than at a loop that never
    /// asked. Whether `None` is the right answer for them is
    /// [`the_pairing_is_the_one_this_module_intends`]'s to say.
    #[test]
    fn every_toolkit_button_the_adapter_binds_produces_its_drag() {
        for egui_button in egui_buttons() {
            let events = drag(&Pane::new(), egui_button);
            let expected: Vec<ViewportEvent> = viewer_button(egui_button)
                .map(|button| ViewportEvent::Drag {
                    button,
                    shift: false,
                    alt: false,
                    delta_px: [40.0, 0.0],
                })
                .into_iter()
                .collect();
            assert_eq!(events, expected, "dragging {egui_button:?}");
        }
    }

    /// **`select_button` decides which click selects, and the pane
    /// produces the click it names.**
    ///
    /// [`InputMap::select_button`] is a binding: any button of the
    /// vocabulary may hold it, and `InputMap::pick` reads the field
    /// rather than a fixed button. `InputMap` is `pub` with `pub`
    /// fields and re-exported from the crate root, so an embedder can
    /// already write `select_button: Middle` — and before this unit
    /// that setting selected nothing, silently, because the pane
    /// produced a click for `Primary` only.
    ///
    /// **The bound buttons are DERIVED, not listed.** Filtering
    /// [`egui_buttons`] through [`viewer_button`] is every button the
    /// viewer binds, by construction: a hand-written
    /// `[Primary, Secondary, Middle]` here would be a complete list of
    /// [`input::PointerButton`] that nothing forces — the shape
    /// `work/view/viewer-suites-hold-hand-written-complete-variant-lists.md`
    /// catalogues, and one `viewer-vocab-declared-once.sh` names as a
    /// blind spot it cannot see. Derived, the row also widens itself
    /// on the day a side button gains a binding.
    #[test]
    fn a_click_selects_through_whichever_button_the_map_binds() {
        for egui_button in egui_buttons() {
            let Some(select_button) = viewer_button(egui_button) else {
                continue;
            };
            let map = InputMap {
                select_button,
                ..InputMap::DEFAULT
            };
            assert_eq!(
                input::pick_stream(&map, &click(&Pane::new(), egui_button)),
                vec![input::PickAction::Select(AIM_PX)],
                "a click of {select_button:?}, the button this map binds, selects"
            );
        }
    }

    /// **The pairing itself, written a second time so that changing it
    /// by accident is red.**
    ///
    /// Everything else about the adapter is derivable and so is
    /// derived. This is not: which toolkit button denotes which of the
    /// viewer's is a naming decision, with nothing in the tree to
    /// check it against. Swapping two arms of [`viewer_button`]
    /// type-checks, keeps the set complete and the list a permutation,
    /// and leaves every other row here green — because they all ask
    /// that function what to expect. The mouse would simply behave
    /// wrongly.
    ///
    /// So the table is stated twice on purpose, and the second copy
    /// costs an edit that has to be made deliberately in two places.
    /// **That cost is the guard, not a defect in it.** Both copies are
    /// exhaustive matches over a closed enum, so neither can fall
    /// behind the toolkit while the other moves: a sixth
    /// `egui::PointerButton` reds them together.
    /// **A drag carries the two modifiers this viewport binds, and the
    /// other three leave no trace on it.**
    ///
    /// Two claims, and the second is the one nothing else can make.
    /// [`viewer_modifiers`]'s pattern holds the SET of fields it reads;
    /// the PAIRING — shift into the first half of the returned pair,
    /// alt into the second — is a naming decision with nothing to
    /// derive it from, and swapping the two type-checks while turning
    /// every constrain into an orbit and back. That is the same gap
    /// [`viewer_button`]'s doc names for the button table, answered the
    /// same way: a second statement of the table, here at the far end
    /// of a real toolkit frame.
    #[test]
    fn a_drag_carries_the_two_modifiers_this_viewport_binds() {
        for (modifiers, expected) in [
            (egui::Modifiers::NONE, (false, false)),
            (egui::Modifiers::SHIFT, (true, false)),
            (egui::Modifiers::ALT, (false, true)),
            (egui::Modifiers::CTRL, (false, false)),
            (egui::Modifiers::COMMAND, (false, false)),
            (egui::Modifiers::MAC_CMD, (false, false)),
        ] {
            let pane = Pane::new();
            let events = drag_modified(&pane, egui::PointerButton::Middle, modifiers);
            let (shift, alt) = expected;
            assert_eq!(
                events,
                vec![ViewportEvent::Drag {
                    button: PointerButton::Middle,
                    shift,
                    alt,
                    delta_px: [40.0, 0.0],
                }],
                "what a middle drag carries with {modifiers:?} held"
            );
        }
    }

    /// **A plain wheel is the one scroll this viewport binds.**
    ///
    /// The control the shift row below needs: it asserts that a
    /// modified wheel produces NO scroll, and a probe that never
    /// delivered a wheel event at all would satisfy it.
    #[test]
    fn a_plain_wheel_is_the_one_scroll_this_viewport_binds() {
        let pane = Pane::new();
        assert_eq!(
            turn_wheel(&pane, egui::Modifiers::NONE),
            vec![a_wheels_scroll()]
        );
    }

    /// **A ctrl/⌘+wheel is a wheel**: the same scroll a plain wheel
    /// produces, and nothing in the toolkit's zoom accumulator. Both
    /// halves fail if the probe's [`configure_input`] is dropped — the
    /// toolkit then spends the wheel as a zoom and the viewport reads
    /// nothing. That the application calls it too is
    /// `app::tests::startup_reads_every_wheel_as_a_scroll`'s claim.
    #[test]
    fn a_ctrl_wheel_is_the_same_scroll_as_a_plain_wheel() {
        for modifiers in [
            egui::Modifiers::CTRL,
            egui::Modifiers::MAC_CMD,
            egui::Modifiers::COMMAND,
        ] {
            let pane = Pane::new();
            assert_eq!(
                turn_wheel(&pane, modifiers),
                vec![a_wheels_scroll()],
                "a wheel with {modifiers:?} held zooms as a plain wheel does"
            );
            assert_eq!(
                pane.ctx.input(|i| i.zoom_delta()),
                1.0,
                "a wheel with {modifiers:?} held is no magnification"
            );
        }
    }

    /// **A pinch is read as the ratio the toolkit reports**: a
    /// trackpad's `Event::Zoom` arrives as [`ViewportEvent::Pinch`]
    /// carrying the same factor, and nothing else.
    #[test]
    fn a_trackpad_pinch_is_a_pinch_event_with_its_factor() {
        let pane = Pane::new();
        pane.reach();
        assert_eq!(
            pane.frame(vec![egui::Event::Zoom(1.1)]),
            vec![ViewportEvent::Pinch {
                factor: f64::from(1.1_f32)
            }]
        );
    }

    /// **A shift+wheel is folded onto the axis this viewport drops.**
    ///
    /// `horizontal_scroll_modifier` is SHIFT, so the toolkit moves the
    /// whole delta onto `x` before the adapter sees it — the axis
    /// [`scroll_event`] names in order to drop. Shift is a modifier
    /// this viewport DOES bind on a drag, which is why the row is worth
    /// having: the two halves of the decision are independent.
    #[test]
    fn a_shift_wheel_is_folded_onto_the_axis_this_viewport_drops() {
        let pane = Pane::new();
        assert_eq!(
            turn_wheel(&pane, egui::Modifiers::SHIFT),
            Vec::<ViewportEvent>::new()
        );
        assert_ne!(
            pane.ctx.input(|i| i.smooth_scroll_delta.x),
            0.0,
            "the wheel did arrive, on the axis the viewer has no gesture for"
        );
    }

    /// **An alt+wheel is folded onto the axis this viewport binds**, so
    /// it zooms exactly as a plain wheel does. The same toolkit
    /// mechanism as the shift row above, pointing the other way:
    /// `vertical_scroll_modifier` is ALT.
    #[test]
    fn an_alt_wheel_is_folded_onto_the_axis_this_viewport_binds() {
        let pane = Pane::new();
        assert_eq!(
            turn_wheel(&pane, egui::Modifiers::ALT),
            vec![a_wheels_scroll()]
        );
    }

    #[test]
    fn the_pairing_is_the_one_this_module_intends() {
        for egui_button in egui_buttons() {
            let intended = match egui_button {
                egui::PointerButton::Primary => Some(PointerButton::Primary),
                egui::PointerButton::Secondary => Some(PointerButton::Secondary),
                egui::PointerButton::Middle => Some(PointerButton::Middle),
                egui::PointerButton::Extra1 | egui::PointerButton::Extra2 => None,
            };
            assert_eq!(
                viewer_button(egui_button),
                intended,
                "{egui_button:?} denotes the wrong button of the viewer's vocabulary"
            );
        }
    }
    /// **The pane's own producer reaches the display seam's rule**,
    /// and a leg it refuses does not reach the lane.
    ///
    /// `push_segment` PLACES a sketch-plane pair and offers it; what
    /// happens to a leg the seam cannot carry is
    /// [`marks::LegLane`]'s, asserted there over the arithmetic. What
    /// is asserted here is the wiring: that this producer goes
    /// through that value at all, so a corner past `f32::MAX` reaches
    /// the vertex buffer as an absence rather than as an infinity.
    #[test]
    fn a_placed_leg_past_the_display_seam_is_offered_and_refused() {
        let plane = pncad::profile::SketchPlane::xy();
        let mut lane = marks::LegLane::default();
        push_segment(&mut lane, &plane, [0.0, 0.0], [1.0, 0.0]);
        assert_eq!(lane.undrawn(), 0);
        assert_eq!(lane.segments().len(), 2, "one leg is two positions");
        push_segment(&mut lane, &plane, [1.0, 0.0], [7.0e307, 0.0]);
        assert_eq!(
            lane.segments().len(),
            2,
            "the refused leg added no position to the lane"
        );
        assert_eq!(lane.undrawn(), 1);
        assert!(
            lane.segments().iter().flatten().all(|c| c.is_finite()),
            "no infinity reaches the vertex buffer"
        );
    }

    /// **An authored outline with one far corner draws as a gap**, not
    /// as nothing and not as a smear.
    ///
    /// The loop a person could compose in the add-profile form:
    /// ordinary corners and one at `7e307`, a number the form takes
    /// because it is a number. Four legs are authored, two of them
    /// reach the far corner, and what the lane holds afterwards is the
    /// other two plus a count of what it lost — which is the evidence
    /// that the drop is visible in a picture a person is looking at
    /// rather than only in one that is nowhere.
    #[test]
    fn an_authored_loop_with_one_far_corner_draws_its_other_legs() {
        let plane = pncad::profile::SketchPlane::xy();
        let polyline = PreviewLoop {
            points: vec![[0.0, 0.0], [1.0, 0.0], [7.0e307, 0.0], [0.0, 1.0]],
            vertices: vec![0, 1, 2, 3],
            end: sketch::LoopEnd::Closed,
        };
        let mut lane = marks::LegLane::default();
        push_loop(&mut lane, &plane, &polyline);
        assert_eq!(lane.undrawn(), 2, "the two legs that reach the far corner");
        assert_eq!(
            lane.segments().len(),
            4,
            "the two legs between ordinary corners are drawn"
        );
        let extent = lane.extent().expect("the drawn legs frame");
        assert_eq!(
            (extent.max_x, extent.max_y),
            (1.0, 1.0),
            "a corner no drawn leg reaches frames nothing"
        );
    }

    /// **The fit frames the loops, the scene and nothing sized by the
    /// view.** A preview's step marks are drawn beside its loops and
    /// reach past them; the extent is the loops' alone, joined to the
    /// scene's box, and `None` when nothing draws. A loop of one vertex
    /// has no leg and still frames its point.
    #[test]
    fn the_drawn_extent_is_the_loops_and_the_scene_and_not_the_step_marks() {
        let nothing = marks::LegLane::default();
        assert_eq!(drawn_extent(&[], &nothing, &nothing), None);

        let drawn = chain(Vec::new());
        let mut loops = marks::LegLane::default();
        push_preview(&mut loops, &drawn);
        let mut steps = marks::LegLane::default();
        push_step_marks(&mut steps, &drawn, Some(a_view()));
        let extent = drawn_extent(&[], &nothing, &loops).expect("the chain draws");
        let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
        for [x, y] in drawn.loops.iter().flat_map(|l| l.points.iter().copied()) {
            lo = [lo[0].min(x), lo[1].min(y)];
            hi = [hi[0].max(x), hi[1].max(y)];
        }
        let near = |a: f64, b: f64| (a - b).abs() < 1.0e-9;
        assert!(
            near(extent.min_x, lo[0])
                && near(extent.min_y, lo[1])
                && near(extent.max_x, hi[0])
                && near(extent.max_y, hi[1]),
            "the extent {extent:?} is the loops' box {lo:?}..{hi:?}"
        );
        let marked = steps.extent().expect("the fixture has marks");
        assert!(
            marked.max_x > extent.max_x + 1.0e-6 || marked.max_y > extent.max_y + 1.0e-6,
            "the marks reach past the loops ({marked:?} vs {extent:?}), so leaving them \
             out is what keeps the fit off the view"
        );

        let scene = [[0.5, 0.5, 0.5], [2.0, 2.0, 2.0], [1.0, 1.0, 1.0]];
        let joined = drawn_extent(&scene, &nothing, &loops).expect("both draw");
        assert!(
            near(joined.min_x, lo[0]) && joined.max_x == 2.0 && joined.max_z == 2.0,
            "the scene's box joins the loops': {joined:?}"
        );

        let lone = PreviewLoop {
            points: vec![[0.02, -0.03]],
            vertices: vec![0],
            end: sketch::LoopEnd::Unfinished(None),
        };
        let mut point = marks::LegLane::default();
        push_loop(&mut point, &pncad::profile::SketchPlane::xy(), &lone);
        assert!(point.segments().is_empty(), "one vertex is no leg");
        let at = drawn_extent(&[], &point, &nothing).expect("the vertex frames");
        assert!(
            near(at.min_x, 0.02) && near(at.max_y, -0.03) && at.min_x == at.max_x,
            "a lone vertex frames its point: {at:?}"
        );
    }

    /// **The frame's order, held as a row**: geometry, extent, fit,
    /// marks. From a camera far from a centimetre preview, an owed fit
    /// must frame the preview's loops alone — red if its marks join the
    /// geometry lane or the extent, since marks sized from out here are
    /// metres long — must centre on them — red if the geometry is built
    /// after the fit, which would then frame nothing and centre on the
    /// origin — and the marks drawn must be the ones the FITTED camera
    /// sizes — red if they are sized before the fit.
    #[test]
    fn a_fit_frames_the_loops_and_the_marks_are_sized_after_it() {
        let drawn = chain(Vec::new());
        let previews = crate::drafts::ProfileDoors {
            create: Some(Ok(drawn.clone())),
            edit: None,
        };
        let far = Camera::new(
            pncad::geom_core::Point3::new(5.0, -4.0, 3.0),
            40.0,
            0.3,
            0.4,
            0.8,
            10.0,
        )
        .expect("a camera");
        let viewport = ViewportSize {
            width_px: 800.0,
            height_px: 600.0,
        };
        let map = crate::input::InputMap::default();
        let framed = frame_camera(super::CameraFrame {
            camera: &far,
            scene: &[],
            profiles: marks::LegLane::default(),
            previews: &previews,
            axis: None,
            split_plane: None,
            fit: true,
            input: &map,
            viewport,
            events: &[],
        });
        let fit = framed.fit.expect("the owed fit was taken");
        let extent = fit
            .applied
            .first()
            .and_then(|op| match op {
                CameraOp::Frame { extent, .. } => *extent,
                _ => None,
            })
            .expect("the fit framed an extent");
        let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
        for [x, y] in drawn.loops.iter().flat_map(|l| l.points.iter().copied()) {
            lo = [lo[0].min(x), lo[1].min(y)];
            hi = [hi[0].max(x), hi[1].max(y)];
        }
        let near = |a: f64, b: f64| (a - b).abs() < 1.0e-9;
        assert!(
            near(extent.min_x, lo[0])
                && near(extent.min_y, lo[1])
                && near(extent.max_x, hi[0])
                && near(extent.max_y, hi[1]),
            "the fit framed {extent:?}, not the loops' box {lo:?}..{hi:?}"
        );
        let target = fit.camera.target();
        assert!(
            near(target.x, 0.5 * (lo[0] + hi[0])) && near(target.y, 0.5 * (lo[1] + hi[1])),
            "the fit centred on {target:?}, not on the loops"
        );
        let mut expected = marks::LegLane::default();
        push_preview(&mut expected, &drawn);
        push_step_marks(
            &mut expected,
            &drawn,
            Some(
                crate::screen::view_of(&fit.camera, viewport)
                    .expect("the fitted camera has a view"),
            ),
        );
        assert_eq!(
            framed.preview,
            expected.segments(),
            "the preview lane is its loops, then the marks the fitted camera sizes"
        );
    }

    /// A unit square at `x ∈ [1, 2]`, `y ∈ [0, 1]` on the xy plane,
    /// revolved about the y axis by `angle` — the revolve tool's
    /// default axis — with `heading` free so a row can turn it.
    fn square_about_y(heading: f64, angle: f64) -> super::RevolveAxisPreview {
        super::RevolveAxisPreview {
            plane: pncad::profile::SketchPlane::xy(),
            loops: vec![PreviewLoop {
                points: vec![[1.0, 0.0], [2.0, 0.0], [2.0, 1.0], [1.0, 1.0]],
                vertices: vec![0, 1, 2, 3],
                end: sketch::LoopEnd::Closed,
            }],
            through: [0.0, 0.0],
            heading,
            angle,
        }
    }

    /// The lane's legs as world pairs.
    fn legs(lane: &marks::LegLane) -> Vec<[[f32; 3]; 2]> {
        lane.segments()
            .chunks_exact(2)
            .map(|pair| [pair[0], pair[1]])
            .collect()
    }

    fn near3(a: [f32; 3], b: [f64; 3]) -> bool {
        a.iter()
            .zip(b)
            .all(|(&a, b)| (f64::from(a) - b).abs() < 1.0e-5)
    }

    /// **The axis runs the way its heading says, across the profile**:
    /// heading `turn/4` draws a line up the y axis from below the
    /// square to above it, a margin past each end, and its arrowhead's
    /// two legs both start at the top end — the `+d` end.
    #[test]
    fn the_revolve_axis_is_drawn_along_its_heading_with_its_arrow_at_plus_d() {
        let axis = square_about_y(core::f64::consts::FRAC_PI_2, core::f64::consts::TAU);
        let mut lane = marks::LegLane::default();
        super::push_revolve_axis(&mut lane, &axis, Some(a_view()));
        let drawn = legs(&lane);
        let [from, to] = drawn[0];
        assert!(
            near3(from, [0.0, -0.15, 0.0]) && near3(to, [0.0, 1.15, 0.0]),
            "the axis line spans the square's height and a margin: {from:?}..{to:?}"
        );
        assert!(
            near3(drawn[1][0], [0.0, 1.15, 0.0]) && near3(drawn[2][0], [0.0, 1.15, 0.0]),
            "both arrowhead legs leave the +d end: {:?}",
            &drawn[1..3]
        );
        assert!(
            drawn[1][1][1] < to[1] && drawn[2][1][1] < to[1],
            "the arrowhead points up the heading: {:?}",
            &drawn[1..3]
        );
        assert!(
            drawn[3..7]
                .iter()
                .all(|[at, end]| at[0].abs() < 1.0e-6 && end[0] > 0.0),
            "the side ticks point to +x, the right of an arrow up y: {:?}",
            &drawn[3..7]
        );
        // Turned half a turn, the same axis points the other way.
        let flipped = square_about_y(-core::f64::consts::FRAC_PI_2, core::f64::consts::TAU);
        let mut lane = marks::LegLane::default();
        super::push_revolve_axis(&mut lane, &flipped, Some(a_view()));
        let flipped_legs = legs(&lane);
        assert!(
            near3(flipped_legs[1][0], [0.0, -0.15, 0.0]),
            "a heading of -turn/4 puts the arrow at the bottom: {:?}",
            flipped_legs[1]
        );
        assert!(
            flipped_legs[3..7].iter().all(|[_, end]| end[0] < 0.0),
            "and its side ticks to -x: {:?}",
            &flipped_legs[3..7]
        );
    }

    /// **The sweep arc turns right-handed about the arrow**: a quarter
    /// turn about +y carries the square's farthest point, `(2, 0, 0)`,
    /// to `(0, 0, −2)` — x toward −z, the revolve's own convention
    /// (`sweep::revolve`, "Winding") — and a negative angle the other
    /// way. Without a view the axis line and the arc are drawn and no
    /// screen-sized mark is: one leg, then 64/4 arc legs.
    #[test]
    fn the_sweep_arc_turns_right_handed_about_the_axis_by_the_angle() {
        let quarter = core::f64::consts::FRAC_PI_2;
        for (angle, end) in [(quarter, [0.0, 0.0, -2.0]), (-quarter, [0.0, 0.0, 2.0])] {
            let axis = square_about_y(quarter, angle);
            let mut lane = marks::LegLane::default();
            super::push_revolve_axis(&mut lane, &axis, None);
            let drawn = legs(&lane);
            assert_eq!(drawn.len(), 1 + 16, "the axis, then the arc: {drawn:?}");
            assert!(
                near3(drawn[1][0], [2.0, 0.0, 0.0]),
                "the arc starts at the farthest point: {:?}",
                drawn[1]
            );
            assert!(
                near3(drawn[16][1], end),
                "a turn of {angle} ends the arc at {end:?}, not {:?}",
                drawn[16][1]
            );
        }
    }

    /// **The sweep's arrowhead points the way the profile turns**: its
    /// two wings start at the arc's end and lie behind it along the
    /// travel. A quarter turn either way about +y ends at `∓2 z`, still
    /// moving toward −x, so both wings sit at `x > 0`.
    #[test]
    fn the_sweep_arcs_arrowhead_points_along_the_turn() {
        let quarter = core::f64::consts::FRAC_PI_2;
        for (angle, end) in [(quarter, [0.0, 0.0, -2.0]), (-quarter, [0.0, 0.0, 2.0])] {
            let axis = square_about_y(quarter, angle);
            let mut lane = marks::LegLane::default();
            super::push_revolve_axis(&mut lane, &axis, Some(a_view()));
            let drawn = legs(&lane);
            assert_eq!(
                drawn.len(),
                1 + 2 + super::SIDE_TICKS as usize + 16 + 2,
                "axis, arrowhead, ticks, arc, arrowhead: {drawn:?}"
            );
            let wings = &drawn[drawn.len() - 2..];
            assert!(
                wings
                    .iter()
                    .all(|[at, wing]| near3(*at, end) && wing[0] > 0.0),
                "a turn of {angle} draws its arrowhead behind {end:?}: {wings:?}"
            );
        }
    }

    /// **The axis is no part of what a fit frames**: a fit owed with the
    /// revolve tool's axis drawn frames the same box as one without it.
    #[test]
    fn a_fit_frames_the_same_box_with_the_revolve_axis_drawn() {
        let previews: super::Previews = crate::drafts::ProfileDoors {
            create: None,
            edit: None,
        };
        let scene: Vec<[f32; 3]> = vec![[1.0, 0.0, 0.0], [2.0, 1.0, 0.0]];
        let axis = square_about_y(core::f64::consts::FRAC_PI_2, core::f64::consts::TAU);
        let camera = Camera::new(
            pncad::geom_core::Point3::new(5.0, -4.0, 3.0),
            40.0,
            0.3,
            0.4,
            0.8,
            10.0,
        )
        .expect("a camera");
        let map = crate::input::InputMap::default();
        let extent = |axis: Option<&super::RevolveAxisPreview>| {
            let framed = frame_camera(super::CameraFrame {
                camera: &camera,
                scene: &scene,
                profiles: marks::LegLane::default(),
                previews: &previews,
                axis,
                split_plane: None,
                fit: true,
                input: &map,
                viewport: ViewportSize {
                    width_px: 800.0,
                    height_px: 600.0,
                },
                events: &[],
            });
            let fit = framed.fit.expect("the owed fit was taken");
            let framed_box = fit.applied.first().and_then(|op| match op {
                CameraOp::Frame { extent, .. } => *extent,
                _ => None,
            });
            format!("{:?}", framed_box.expect("the fit framed an extent"))
        };
        assert_eq!(extent(Some(&axis)), extent(None));
    }

    /// A profile lying ON its axis has no radius to sweep, and an angle
    /// that is no number has no arc: the axis alone is drawn.
    #[test]
    fn no_sweep_arc_is_drawn_without_a_radius_or_an_angle() {
        let mut on_axis = square_about_y(core::f64::consts::FRAC_PI_2, 1.0);
        on_axis.loops[0].points = vec![[0.0, 0.0], [0.0, 1.0]];
        let not_a_number = square_about_y(core::f64::consts::FRAC_PI_2, f64::NAN);
        for axis in [on_axis, not_a_number] {
            let mut lane = marks::LegLane::default();
            super::push_revolve_axis(&mut lane, &axis, None);
            assert_eq!(lane.segments().len(), 2, "the axis line alone");
        }
    }

    /// A view of the xy plane from a tenth of a metre, looking at the
    /// centimetre square the preview rows below draw in.
    fn a_view() -> crate::screen::View {
        let camera = Camera::new(
            pncad::geom_core::Point3::new(0.005, 0.005, 0.0),
            0.1,
            0.0,
            1.0,
            0.8,
            0.05,
        )
        .expect("a camera");
        crate::screen::view_of(
            &camera,
            ViewportSize {
                width_px: 800.0,
                height_px: 600.0,
            },
        )
        .expect("a view")
    }

    /// Every segment [`push_preview`] and [`push_step_marks`] put in a
    /// lane for `drawn`, as sketch-plane pairs (the plane is xy, so
    /// world x and y).
    fn painted_preview(drawn: &ProfilePreview) -> Vec<[[f64; 2]; 2]> {
        let mut lane = marks::LegLane::default();
        push_preview(&mut lane, drawn);
        push_step_marks(&mut lane, drawn, Some(a_view()));
        lane.segments()
            .chunks_exact(2)
            .map(|pair| pair_2d([pair[0], pair[1]]))
            .collect()
    }

    fn pair_2d(pair: [[f32; 3]; 2]) -> [[f64; 2]; 2] {
        pair.map(|[x, y, _]| [f64::from(x), f64::from(y)])
    }

    /// Two plane points within what an `f32` vertex buffer keeps of
    /// centimetre-scale coordinates.
    fn near(a: [f64; 2], b: [f64; 2]) -> bool {
        (a[0] - b[0]).hypot(a[1] - b[1]) < 1.0e-7
    }

    /// **What is marked at the tip `at`, arriving along `heading`**:
    /// the strokes centred on it, each as its |cosine| to the heading
    /// (a tick across the path is 0; a cross on its diagonals is
    /// ~0.707), and how many segments end at the arrowhead point a
    /// tick-length ahead of it.
    fn tip_marks(painted: &[[[f64; 2]; 2]], at: [f64; 2], heading: [f64; 2]) -> (Vec<f64>, usize) {
        let world = pncad::geom_core::Point3::new(at[0], at[1], 0.0);
        let tick = a_view()
            .screen_metres_at(world, TIP_MARK_PX)
            .expect("a mark size at the tip");
        let ahead = [at[0] + heading[0] * tick, at[1] + heading[1] * tick];
        let centred = painted
            .iter()
            .filter(|[a, b]| near([(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0], at))
            .map(|[a, b]| {
                let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
                ((dx * heading[0] + dy * heading[1]) / dx.hypot(dy)).abs()
            })
            .collect();
        let arrow = painted
            .iter()
            .filter(|[a, b]| near(*a, ahead) || near(*b, ahead))
            .count();
        (centred, arrow)
    }

    /// Whether a leg joins `a` and `b`, either way round.
    fn joins(painted: &[[[f64; 2]; 2]], a: [f64; 2], b: [f64; 2]) -> bool {
        painted
            .iter()
            .any(|[p, q]| near(*p, a) && near(*q, b) || near(*p, b) && near(*q, a))
    }

    /// `test_support::two_legs` from the origin, to [`TIP`] arriving
    /// along `+y`, then `tail`.
    fn chain(tail: Vec<pncad::profile::Step<f64>>) -> ProfilePreview {
        let mut steps = crate::test_support::two_legs(0.0, 0.0);
        steps.extend(tail);
        path(steps)
    }

    /// The preview of one path loop on the xy plane, which draws.
    fn path(steps: Vec<pncad::profile::Step<f64>>) -> ProfilePreview {
        sketch::preview(
            pncad::profile::SketchPlane::xy(),
            &[ProfileShape::Path { steps }],
            Tol::witness(),
            1.0e-4,
        )
        .expect("the chain draws")
    }

    const TIP: [f64; 2] = [0.01, 0.01];
    const ARRIVING: [f64; 2] = [0.0, 1.0];

    /// **A step the author picked that refuses still leaves the chain
    /// before it on screen, with a cross where it stops.** The
    /// `arc_fillet_arc` the form hands an author who picks that verb
    /// at this tip is refused on arrival; what is painted is the two
    /// legs before it, not the provisional close back to the start,
    /// and at the tip a cross on the heading's diagonals with no
    /// arrowhead.
    ///
    /// Red if the refused loop draws nothing (the `expect` panics), if
    /// it draws its provisional close, or if its tip is marked like any
    /// other vertex.
    #[test]
    fn a_refused_step_paints_the_chain_before_it_and_a_cross_at_its_tip() {
        use pncad::profile::{Step, Target, TipState, Verb};
        let picked = sketch::fresh_step_at(Verb::ArcFilletArc, Some(TipState::DirectedPoint));
        let drawn = chain(vec![picked, Step::LineTo(Target::Start)]);
        assert!(
            drawn.loops[0].end.refusal().is_some(),
            "a fixture whose picked step refuses: {drawn:?}"
        );
        let painted = painted_preview(&drawn);
        assert!(joins(&painted, [0.0, 0.0], [0.01, 0.0]), "{painted:?}");
        assert!(joins(&painted, [0.01, 0.0], TIP), "{painted:?}");
        assert!(
            !joins(&painted, TIP, [0.0, 0.0]),
            "the provisional close is not painted: {painted:?}"
        );
        let (centred, arrow) = tip_marks(&painted, TIP, ARRIVING);
        assert_eq!(
            centred.len(),
            2,
            "two strokes cross at the tip: {centred:?}"
        );
        assert!(
            centred
                .iter()
                .all(|cos| (cos - core::f64::consts::FRAC_1_SQRT_2).abs() < 1.0e-3),
            "both on the heading's diagonals: {centred:?}"
        );
        assert_eq!(arrow, 0, "no arrowhead: the chain goes nowhere from here");
    }

    /// **An unfinished chain's tip keeps its arrowhead and no cross** —
    /// the same two legs as the row above with nothing after them, so
    /// the only difference painted is the tip's.
    ///
    /// Red if the cross is drawn at every open tip rather than only a
    /// refused one.
    #[test]
    fn an_unfinished_chains_tip_is_painted_going_on() {
        let drawn = chain(Vec::new());
        assert!(
            drawn.loops[0].end.is_unfinished(),
            "a fixture that is merely unfinished: {drawn:?}"
        );
        let painted = painted_preview(&drawn);
        let (centred, arrow) = tip_marks(&painted, TIP, ARRIVING);
        assert_eq!(centred.len(), 1, "one tick through the tip: {centred:?}");
        assert!(centred[0] < 1.0e-3, "square to the path: {centred:?}");
        assert_eq!(arrow, 2, "an arrowhead ahead of it");
    }

    /// **A chain with an unclosable tip is painted as the legs before
    /// it, going on** — at every state no `line_to` leaves, as the
    /// lattice table lists them. The steps that put the tip there are
    /// not a leg anybody can draw yet, so what is painted is the two
    /// legs, not the provisional close, and the tip keeps its
    /// arrowhead: the chain is unfinished, not refused.
    ///
    /// Red if such a chain draws nothing (the `expect` panics), if its
    /// provisional close is painted, or if its tip is crossed.
    #[test]
    fn an_unclosable_tip_is_painted_as_the_legs_before_it_going_on() {
        let mut walked = Vec::new();
        for state in crate::test_support::unclosable_tips() {
            let Some(way_in) = ::profile::test_support::way_in(state) else {
                assert_eq!(
                    state,
                    pncad::profile::TipState::Entry,
                    "only the entry has no way in"
                );
                continue;
            };
            let drawn = chain(way_in);
            assert!(
                matches!(
                    drawn.loops[0].end.unfinished_refusal(),
                    Some(sketch::PreviewError::Transition { state: at, verb: None, .. }) if *at == state
                ),
                "{state:?}: a fixture whose tip is unclosable: {drawn:?}"
            );
            let painted = painted_preview(&drawn);
            assert!(joins(&painted, [0.0, 0.0], [0.01, 0.0]), "{state:?}");
            assert!(joins(&painted, [0.01, 0.0], TIP), "{state:?}");
            assert!(
                !joins(&painted, TIP, [0.0, 0.0]),
                "{state:?}: the provisional close is not painted"
            );
            let (centred, arrow) = tip_marks(&painted, TIP, ARRIVING);
            assert_eq!(centred.len(), 1, "{state:?}: one tick: {centred:?}");
            assert_eq!(arrow, 2, "{state:?}: an arrowhead ahead of it");
            walked.push(state);
        }
        assert!(
            walked.contains(&pncad::profile::TipState::RadiusArrival),
            "the census reads the table: {walked:?}"
        );
    }

    /// **A chain cut at an unclosable tip whose last leg lands on its
    /// start is painted closed.** The walked-back prefix is the square
    /// the author's own legs close; its last leg is authored, not the
    /// provisional close, so it is painted.
    ///
    /// Red if an unfinished loop is read as never closing.
    #[test]
    fn a_prefix_cut_at_an_unclosable_tip_that_lands_on_its_start_is_painted_closed() {
        use pncad::geom_core::Point2;
        use pncad::profile::{Step, Target};
        let drawn = chain(vec![
            Step::LineTo(Target::Point(Point2::new(0.0, 0.01))),
            Step::LineTo(Target::Point(Point2::new(0.0, 0.0))),
            Step::Turn(0.5),
        ]);
        assert!(
            matches!(&drawn.loops[0].end, sketch::LoopEnd::Unfinished(Some(cut)) if cut.closes),
            "a fixture whose drawn prefix closes: {drawn:?}"
        );
        let painted = painted_preview(&drawn);
        assert!(
            joins(&painted, [0.0, 0.01], [0.0, 0.0]),
            "the authored last leg is painted: {painted:?}"
        );
    }

    /// **A chain that closed and then went on is painted closed, with
    /// the cross at its start.** A square closed by `line_to Start`,
    /// then one more `line_to`, which no closed loop takes: the four
    /// authored legs are painted, the closing one included, and the
    /// start — where the drawn steps end — carries the cross and no
    /// arrowhead.
    ///
    /// Red if the drawn prefix is replayed only under the provisional
    /// close (the closing leg goes unpainted and the cross lands on
    /// `(0, 0.01)`).
    #[test]
    fn a_chain_that_closed_and_went_on_is_painted_closed() {
        use pncad::geom_core::Point2;
        use pncad::profile::{Step, Target};
        let drawn = chain(vec![
            Step::LineTo(Target::Point(Point2::new(0.0, 0.01))),
            Step::LineTo(Target::Start),
            Step::LineTo(Target::Point(Point2::new(0.005, 0.005))),
        ]);
        assert!(
            drawn.loops[0].end.refusal().is_some(),
            "a fixture whose last step refuses: {drawn:?}"
        );
        let painted = painted_preview(&drawn);
        assert!(
            joins(&painted, [0.0, 0.01], [0.0, 0.0]),
            "the authored close is painted: {painted:?}"
        );
        let (centred, arrow) = tip_marks(&painted, [0.0, 0.0], [1.0, 0.0]);
        assert_eq!(centred.len(), 2, "the cross at the start: {centred:?}");
        assert_eq!(arrow, 0, "no arrowhead at the start");
    }

    /// **A chain whose close is refused on its geometry is painted as
    /// the legs written and no more** — at every shape of such a close
    /// ([`crate::test_support::geometry_refused_closes`]). Every painted
    /// stroke longer than a tip mark is one of those legs; the leg back
    /// to the start is painted where the author's last leg lands there,
    /// and nowhere else; and the tip keeps its arrowhead, since the
    /// chain is unfinished: at the last leg's end, or at the start
    /// where the loop closes.
    ///
    /// Red if such a chain paints nothing (the `expect` panics), if it
    /// paints the close nobody wrote or a fillet resolved against it,
    /// or if its tip is crossed.
    #[test]
    fn a_close_refused_on_its_geometry_is_painted_as_the_legs_written() {
        for fixture in crate::test_support::geometry_refused_closes() {
            let drawn = path(fixture.steps.clone());
            let painted = painted_preview(&drawn);
            let vertices = &fixture.vertices;
            let mut written: Vec<[[f64; 2]; 2]> =
                vertices.windows(2).map(|w| [w[0], w[1]]).collect();
            let back = [vertices[vertices.len() - 1], vertices[0]];
            if fixture.closes {
                written.push(back);
            } else {
                assert!(!joins(&painted, back[0], back[1]), "{:?}", fixture.steps);
            }
            for [a, b] in &written {
                assert!(joins(&painted, *a, *b), "{:?}: {a:?}-{b:?}", fixture.steps);
            }
            let tick = a_view()
                .screen_metres_at(pncad::geom_core::Point3::new(0.0, 0.0, 0.0), TIP_MARK_PX)
                .expect("a mark size");
            for [p, q] in &painted {
                let long = (q[0] - p[0]).hypot(q[1] - p[1]) > 2.0 * tick;
                assert!(
                    !long || written.iter().any(|[a, b]| joins(&[[*p, *q]], *a, *b)),
                    "{:?}: a stroke nobody wrote, {p:?}-{q:?}",
                    fixture.steps
                );
            }
            let (tip, heading) = if fixture.closes {
                ([0.0, 0.0], [1.0, 0.0])
            } else {
                (TIP, ARRIVING)
            };
            let (centred, arrow) = tip_marks(&painted, tip, heading);
            assert_eq!(centred.len(), 1, "{:?}: {centred:?}", fixture.steps);
            assert_eq!(arrow, 2, "{:?}: an arrowhead", fixture.steps);
        }
    }

    /// **The cross sits on the step that refused**, after a last leg a
    /// close would run tangent to, after a last leg onto the start by
    /// an entry that opens with a direction, and after a fused entry's
    /// last leg. Each is painted to the last leg's end and crossed
    /// there — at the start where the leg lands on it.
    ///
    /// Red if any last leg is dropped (the cross then lands one vertex
    /// early, on a step that is fine), or if a close nobody wrote is
    /// painted closed.
    #[test]
    fn the_cross_sits_on_the_step_that_refused() {
        use crate::test_support::{self, ill_typed};
        let mut cases = test_support::tangent_closes();
        let angle_first = test_support::geometry_refused_closes()
            .into_iter()
            .map(|fixture| fixture.steps)
            .find(|steps| !matches!(steps[0], pncad::profile::Step::At(_)))
            .expect("an entry that opens with a direction");
        cases.push((angle_first, [0.0, 0.0]));
        let fused = test_support::fused_entry_then(test_support::line_to(0.01, 0.003));
        cases.push((fused, [0.01, 0.003]));
        for (mut steps, tip) in cases {
            steps.push(ill_typed());
            let drawn = path(steps.clone());
            let only = &drawn.loops[0];
            assert!(only.end.refusal().is_some(), "{steps:?}");
            let at = only
                .vertices
                .iter()
                .copied()
                .find(|&at| near(only.points[at], tip))
                .expect("the tip is a vertex drawn");
            let heading = sketch::heading(&only.points, at, only.end.closes()).expect("a heading");
            let (centred, arrow) = tip_marks(&painted_preview(&drawn), tip, heading);
            assert_eq!(centred.len(), 2, "{steps:?}: the cross: {centred:?}");
            assert_eq!(arrow, 0, "{steps:?}: no arrowhead");
        }
    }

    /// **An edit of a committed profile that a step refuses hides the
    /// committed shape, whether or not anything before the refusal
    /// draws** — Ev's ruling of 2026-09-30. A square on the xy frame,
    /// held by the edit door; the drive is the frame's own, in order:
    /// the edit door's preview, [`crate::drafts::Drafts::edited_in_place`]
    /// on it, then the two lanes the viewport paints
    /// ([`push_committed`], [`push_previews`]) and the form's sentence
    /// ([`crate::pane::profile::preview_verdict`], headless).
    ///
    /// - Refused at step 1, nothing before it draws: no committed leg
    ///   and no preview leg is painted, and the form says the refusal.
    ///   Red if a preview with nothing to draw leaves the committed
    ///   drawing up (the flip the ruling fixed).
    /// - Refused at step 3, after two legs: no committed leg; the two
    ///   legs are painted. Red if a refused preview stops standing in
    ///   for the node.
    #[test]
    fn a_refused_edit_hides_the_committed_profile_with_or_without_a_prefix() {
        use pncad::document::{CancelToken, Doc, EvalOptions, evaluate};
        use pncad::geom_core::Point2;
        use pncad::profile::{Step, TipState, Verb};

        use crate::drafts::{Drafts, ProfileDoors, RowEdit};

        let tol = Tol::witness();
        let chord = 1.0e-4;
        let side = 0.02;
        let (doc, profile) = crate::test_support::square_profile(
            &Doc::empty_derived("c5-refused-edit", tol),
            side,
            tol,
        );
        let evaluation = evaluate(
            &doc,
            None,
            &CancelToken::default(),
            &EvalOptions::default(),
            tol,
        );
        let mut drafts = Drafts::default();
        drafts
            .profile_edit(&doc, profile)
            .expect("the square is held");
        assert!(
            drafts.door_loops(&doc, tol).edit.is_some(),
            "the edit door holds the square"
        );
        let placement = sketch::profile_plane();
        let segments = |lane: &marks::LegLane| -> Vec<[[f64; 2]; 2]> {
            lane.segments()
                .chunks_exact(2)
                .map(|pair| pair_2d([pair[0], pair[1]]))
                .collect()
        };
        // One frame with `loops` held by the edit door: its preview,
        // the committed lane, the preview lane.
        let mut frame_with = |loops: Vec<Step<f64>>| {
            let edit = drafts.profile_edit.as_mut().expect("held");
            let notation = crate::props::Notation::CANONICAL;
            edit.edit_row(0, RowEdit::Clear, notation)
                .expect("a clear makes no row");
            for (at, step) in loops.into_iter().enumerate() {
                edit.edit_row(0, RowEdit::Insert { at, step }, notation)
                    .expect("finite");
            }
            let held = drafts.door_loops(&doc, tol).edit.expect("held");
            let preview = sketch::preview(placement, &held.loops, tol, chord);
            let previews = ProfileDoors {
                create: None,
                edit: Some(preview.clone()),
            };
            let edited = drafts.edited_in_place(previews.edit.as_ref());
            let mut committed = marks::LegLane::default();
            let undrawn = push_committed(&mut committed, &doc, &evaluation, chord, edited);
            assert_eq!(undrawn, 0, "the square draws when it is drawn at all");
            let mut drawn = marks::LegLane::default();
            push_previews(&mut drawn, &previews);
            (preview, segments(&committed), segments(&drawn))
        };

        let mut control = marks::LegLane::default();
        push_committed(&mut control, &doc, &evaluation, chord, None);
        let control = segments(&control);
        assert!(
            joins(&control, [0.0, 0.0], [side, 0.0]),
            "the control: the square is painted when nothing edits it: {control:?}"
        );

        let fused = |state| sketch::fresh_step_at(Verb::ArcFilletArc, state);
        let (preview, committed, drawn) = frame_with(vec![
            Step::At(Point2::origin()),
            fused(Some(TipState::PlainPoint)),
        ]);
        let refusal = preview
            .as_ref()
            .expect_err("a fixture refused at step 1 with nothing to draw");
        assert!(
            committed.is_empty(),
            "refused at step 1: the committed square is hidden: {committed:?}"
        );
        assert!(
            drawn.is_empty(),
            "refused at step 1: nothing is drawn: {drawn:?}"
        );
        let said = crate::pane::headless::painted_text(|ui| {
            crate::pane::profile::preview_verdict(ui, crate::theme::Theme::DEFAULT, Some(&preview));
        });
        assert!(
            said.contains(&refusal.to_string()),
            "refused at step 1: the form says the refusal: {said:?}"
        );

        let mut steps = crate::test_support::two_legs(0.0, 0.0);
        steps.push(fused(Some(TipState::DirectedPoint)));
        let (preview, committed, drawn) = frame_with(steps);
        assert!(
            matches!(&preview, Ok(p) if p.loops[0].end.refusal().is_some()),
            "a fixture refused at step 3 with two legs before it: {preview:?}"
        );
        assert!(
            committed.is_empty(),
            "refused at step 3: the committed square is hidden: {committed:?}"
        );
        assert!(
            joins(&drawn, [0.0, 0.0], [0.01, 0.0]) && joins(&drawn, [0.01, 0.0], [0.01, 0.01]),
            "refused at step 3: the legs before it are painted: {drawn:?}"
        );
    }
}
