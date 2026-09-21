//! **Datums, drawn.** The wireframe a plane, an axis or a point is
//! shown as in the viewport — a pure function of the landed evaluation
//! and the picture's own extent, with no toolkit and no GPU in sight.
//!
//! # Why this is a value and not a paint routine
//!
//! G1's rule, the same one [`crate::marks::edge_overlay`] obeys: a test
//! asserts which segments a datum draws and where they are, and what
//! colour they come out is the theme's answer and the shader's. That
//! matters more here than for an edge mark, because a datum's geometry
//! is INVENTED — a plane is infinite and a point has no extent, so
//! every number below is a display decision this module is answerable
//! for rather than a fact read out of the document.
//!
//! # A datum has no size, so the VIEW lends it one
//!
//! The document says where a plane is and which way it faces; it does
//! not say how big to draw it, because a plane is not big. Every
//! dimension here therefore comes from a [`View`] — where the eye is
//! and how much world one pixel spans — and NOT from the scene's
//! extent, which is what the first version of this module used.
//!
//! **Sizing against the scene has a hole in it, and the hole is
//! literal.** A world-fixed grid keeps its pitch while the view zooms
//! in, so past some distance one cell fills the window and the plane
//! vanishes — not dimmed, not clipped, absent, with nothing on screen
//! to say a plane is there at all. On the shipped 60x40x8 plate the
//! grid was 10 mm and one cell filled the view at 12 mm of camera
//! distance, inside a zoom band that reaches 1.8 mm. The same failure
//! runs the other way at the far end: zoom out and a world-fixed patch
//! shrinks to a speck.
//!
//! So both halves follow the view, and they are two decisions:
//!
//! - **The patch** is the part of the plane the window sees, out to
//!   where a cell shrinks to [`MIN_CELL_PX`] (`seen_region`) — so it
//!   reaches the window's edges at every zoom and every tilt, and a
//!   plane seen at a slant is ruled toward its horizon rather than in
//!   a square that ends partway up the window.
//! - **The pitch** snaps to a 1-2-5 ladder chosen so one cell spans
//!   about [`TARGET_PITCH_PX`] pixels. Snapping is the half that is
//!   easy to miss: a pitch varying CONTINUOUSLY with distance would
//!   keep the on-screen spacing perfect and make every line swim under
//!   the cursor as you zoom, which is useless as a ruler. On a ladder
//!   the lines hold still and the grid subdivides in steps — at the
//!   cost of a visible pop at each rung, which this pass cannot
//!   cross-fade away because it has no alpha and which is the honest
//!   price of lines that stay put.
//!
//! Lines are laid at multiples of the pitch FROM THE DATUM'S ORIGIN,
//! not from the ruled region's edge: the origin is a real point on the
//! plane and a grid line through it is a fact, where a grid indexed
//! off a moving window would slide as the eye moved.
//!
//! **Perspective makes one pitch a compromise**, stated rather than
//! hidden, and the compromise is not one-sided. World-per-pixel grows
//! with depth, so one pitch over a plane seen at an angle is finer
//! than asked for beyond the scale point and COARSER than asked for
//! nearer than it. The scale is taken where the camera is pointed,
//! which puts the target pitch exactly where a reader is looking and
//! spends the error on the parts of the plane they are not.
//!
//! So the claim this module makes is the bounded one and not the
//! total one: no cell opens wide enough to swallow the window where
//! the view is aimed. A plane raking away under the camera still has
//! a near corner drawn coarser than [`TARGET_PITCH_PX`], and pointing
//! at that corner is what re-scales it.
//!
//! # Lines, not a translucent quad
//!
//! A plane is drawn as an outline with a few interior grid lines and a
//! short normal tick, and NOT as a filled surface at partial opacity.
//! Both read as a plane; the difference is what they cost the picture.
//! A filled quad hides whatever is behind it — which on a datum
//! cutting through a part is exactly the part — so it buys legibility
//! of the datum with legibility of the model, and it needs an
//! alpha-blended pass of its own, drawn in an order that has to be got
//! right against the depth buffer. The grid reads as a surface from
//! the interior lines and occludes nothing. If a fill is wanted later
//! the geometry here is what it would be built from; this module's
//! answer would gain a triangle list beside its segments rather than
//! changing shape.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    DatumValue, Doc, Evaluation, Node, ProfileProgram, RecipeNodeId, ValuePayload,
};
use pncad::geom_core::{Point3, UnitVec3, Vec3};

use crate::camera::{Camera, CameraError};
use crate::input::ViewportSize;

/// **Are all of these numbers?** — the module's one finiteness
/// question, asked wherever a value that is not a number would be
/// carried into a drawing rather than stopping one.
///
/// Every quantity here is a plain `f64` a caller wrote or this module
/// derived from one: a height off a plane, a crossing's plane
/// coordinate, a region bound, a lattice index. None of them is a
/// decided length with a norm witness, so **`geom_core`'s
/// `is_finite_length` is not the home for them** even though it
/// decides the same fact for `f64`. That predicate is generic over
/// `Real` and asks through the scalar's poison channel because its
/// callers may be carrying an interval or a dual; its rustdoc's roster
/// is a hand-kept claim about doors that DECIDE a length, and the
/// recourse it names — rescale the geometry — is not a recourse a
/// display module can offer. Routing display code through it would
/// widen that roster with sites it is not about and would still leave
/// the positivity half below hand-spelled.
///
/// Taken by array rather than by slice so a call site names its
/// quantities and nothing can be appended to the list by accident.
fn all_finite<const N: usize>(values: [f64; N]) -> bool {
    values.iter().all(|value| value.is_finite())
}

/// **The module's other finiteness question: is this a POSITIVE
/// length?** — a scale, a span or a pitch, handed back when it is one
/// and refused when it is not.
///
/// The two questions are separate because their subjects are: a
/// coordinate and a lattice bound are numbers that may legitimately be
/// negative or zero, and asking them for a sign would refuse half the
/// plane. This one is asked of quantities every consumer MULTIPLIES a
/// pixel count by, where zero and negative are as undrawable as `NaN`
/// — so it answers with the value, and a caller that gets one is
/// holding a length rather than a permission to re-read the number it
/// just asked about.
///
/// Built ON [`all_finite`] rather than beside it, so the finiteness
/// half has exactly one spelling in this module.
fn positive_length(value: f64) -> Option<f64> {
    (all_finite([value]) && value > 0.0).then_some(value)
}

/// **Where the eye is and how much world a pixel spans** — everything
/// this module needs to size a drawing against the window rather than
/// against the model.
///
/// A value rather than a borrow of the camera, for the reason the rest
/// of this module is a value: the suite drives these numbers directly
/// and asserts what comes out, with no camera, no viewport and no
/// renderer in the room.
#[derive(Debug, Clone, Copy)]
pub struct View {
    /// The eye's position, world metres.
    pub eye: Point3<f64>,
    /// **What the camera is looking AT** — the orbit target, world
    /// metres.
    ///
    /// Both points are needed and they do different jobs. The eye
    /// gives the SCALE (how much world a pixel spans is a function of
    /// distance from it); the target gives the DIRECTION the window
    /// looks in, and the point a plane's pitch and an axis's reach are
    /// read at — the point a reader is looking at, where the eye's own
    /// perpendicular foot on a plane seen at a grazing angle sits far
    /// from it.
    pub look_at: Point3<f64>,
    /// **World metres one pixel spans at one metre from the eye.** The
    /// scale at any other depth is this times that depth, which is the
    /// whole of the perspective arithmetic here.
    ///
    /// From the camera as `2 * tan(fov_y / 2) / viewport_height_px`.
    ///
    /// **Not promised to be a length**, because this struct's fields
    /// are the caller's: the suite builds a [`View`] directly, and a
    /// number written here is whatever was written. [`datum_view`]
    /// refuses the window that would produce a non-length rather than
    /// carrying one, and [`View::metres_per_pixel_at`] is where a
    /// value that is not a scale is refused anyway, once, for every
    /// mark — the field's promise and the door's check are separate
    /// claims and the door owes its own.
    pub metres_per_pixel_at_one_metre: f64,
    /// **Which way is up on screen** — a world direction that, with
    /// the line from the eye to [`View::look_at`], turns the window.
    /// Need not be unit or square to the view: only the plane it
    /// spans with the view direction is read.
    ///
    /// Read only by the plane's ruling, which rules the part of the
    /// plane the window sees ([`View::frustum_corners`]).
    pub up: Vec3<f64>,
    /// The window's width and height, in pixels.
    ///
    /// **Not promised to be pixel counts**, for the reason above and
    /// with the same disposal: a number that is not a count of pixels
    /// reaches the marks sized in windows — an axis's reach
    /// ([`View::half_patch_at`]) and a plane's ruling
    /// ([`View::frustum_corners`]) — and neither draws for it.
    pub window_px: [f64; 2],
}

impl View {
    /// **World metres one pixel spans at `point`**, or `None` when
    /// this view lends `point` no scale at all.
    ///
    /// **This is the module's refusal at its source**: every length
    /// drawn below is this number times a pixel count, so a scale
    /// that is not a positive finite length is a scale there is no
    /// drawing for. Two depths reach that, and the same answer is
    /// right for both because the same drawing comes out of both.
    ///
    /// - **A depth that is not a number** — an eye or a point with a
    ///   `NaN` coordinate. Nothing downstream can tell a length
    ///   derived from one from a length that means something.
    /// - **A depth of exactly zero** — the eye on the point,
    ///   reachable by flying the camera into a plane. A floor at a
    ///   hair above zero would keep this total and buy a patch about
    ///   `1e-305 m` across; no consumer needs it, because every one
    ///   of them takes this `Option` and draws nothing, so the
    ///   division a floor guards against is not the alternative.
    fn metres_per_pixel_at(&self, point: Point3<f64>) -> Option<f64> {
        let depth = ((point.x - self.eye.x).powi(2)
            + (point.y - self.eye.y).powi(2)
            + (point.z - self.eye.z).powi(2))
        .sqrt();
        let scale = depth * self.metres_per_pixel_at_one_metre;
        positive_length(scale)
    }

    /// **What a span of `px` PIXELS measures at `point`**, in world
    /// metres, or `None` when this view lends `point` no such length.
    ///
    /// **Every mark this module draws is a pixel count read through
    /// here**, so this is the module's one door for the refusal
    /// [`grid_pitch`] makes at the other end of the same arithmetic —
    /// and each mark asks for its own point. A plane's ruling is
    /// scaled at the looked-at point and its normal tick at the
    /// origin; a frame's arms at the origin; an axis's tick at each of
    /// its two ends. Those are DIFFERENT depths, so they refuse
    /// separately: a datum whose patch has no scale still says which
    /// way it faces if its origin has one.
    ///
    /// The check is on the PRODUCT and not on the metres-per-pixel,
    /// because a scale that is a length does not make every multiple
    /// of it one — and BOTH halves of it are live. `px` is a positive
    /// constant at every mark, but [`View::half_patch_at`] passes a
    /// span read off [`View::viewport_px`], which is the caller's
    /// number and need not be one.
    ///
    /// Public because it is the one door from a pixel count to a
    /// world length, and a mark drawn outside this module — a profile
    /// preview's tip marks (`pane::viewport`) — is sized the same way
    /// or it is sized against the model.
    pub fn screen_metres_at(&self, point: Point3<f64>, px: f64) -> Option<f64> {
        let span = self.metres_per_pixel_at(point)? * px;
        positive_length(span)
    }

    /// What HALF a patch of `cover` windows measures at `point`.
    ///
    /// [`View::viewport_px`] reaches [`View::screen_metres_at`]'s
    /// product as it stands, so a window whose larger side is zero or
    /// is not a number is refused there and has no patch. A floor at
    /// one pixel would instead answer with the patch a one-pixel
    /// window has — a length this view did not lend, through a door
    /// whose whole job is to refuse exactly those.
    fn half_patch_at(&self, point: Point3<f64>, cover: f64) -> Option<f64> {
        self.screen_metres_at(point, self.viewport_px() * cover * 0.5)
    }

    /// The window's larger side, in pixels, or `NaN` when either side
    /// is not a number (`f64::max` would quietly answer the other).
    fn viewport_px(&self) -> f64 {
        let [width, height] = self.window_px;
        if width.is_nan() || height.is_nan() {
            f64::NAN
        } else {
            width.max(height)
        }
    }

    /// **The four far corners of the window's view, `depth` in front
    /// of the eye**, in world metres, in order around the window.
    ///
    /// With the eye they are the apex and base of the pyramid the
    /// window sees out to `depth`. A corner direction is the view
    /// direction plus the corner's pixel offset from the window's
    /// centre times [`View::metres_per_pixel_at_one_metre`] along
    /// screen right and up — one metre of view depth, so scaling by
    /// `depth` puts every corner at that view depth. Nothing here is
    /// checked: an input that is not a number comes out as corners
    /// that are not, which is the refusal [`grid`] reads.
    fn frustum_corners(&self, depth: f64) -> [Point3<f64>; 4] {
        let forward = Vec3::new(
            self.look_at.x - self.eye.x,
            self.look_at.y - self.eye.y,
            self.look_at.z - self.eye.z,
        );
        let forward = forward / forward.norm();
        let right = forward.cross(self.up);
        let right = right / right.norm();
        let up = right.cross(forward);
        let [width, height] = self.window_px;
        let half_x = width * 0.5 * self.metres_per_pixel_at_one_metre;
        let half_y = height * 0.5 * self.metres_per_pixel_at_one_metre;
        [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)].map(|(sx, sy): (f64, f64)| {
            let dir = forward + right * (sx * half_x) + up * (sy * half_y);
            Point3::new(
                self.eye.x + dir.x * depth,
                self.eye.y + dir.y * depth,
                self.eye.z + dir.z * depth,
            )
        })
    }
}

/// **The pitch one cell is drawn at**: the rung of the 1-2-5 ladder
/// whose on-screen span is nearest [`TARGET_PITCH_PX`], or `None`
/// when there is no rung to read.
///
/// Public because it is the module's one arithmetic claim worth
/// asserting on its own — that the realized pitch stays inside the
/// band the ladder's step size implies, at every scale, and that a
/// scale which is not one gets no rung at all.
///
/// **The refusal is the whole reason this returns an `Option`.** The
/// ladder is a reading of `metres_per_pixel * TARGET_PITCH_PX`, and
/// when that span is not a positive finite length there is nothing to
/// read: the logarithm below has no floor and the ratio has no
/// minimum. Every rung this could hand back instead is a number the
/// function did not compute, arriving at the caller in the shape of
/// one that it did — and the caller rules a lattice at multiples of
/// it, so the substitution does not stay small. `f64::MIN_POSITIVE`
/// is the worst of them and was what this returned: the index bounds
/// come out `-inf..=inf`, so the patch is ruled at the cap
/// ([`MAX_GRID_LINES`]) with every coordinate `NaN`. Refusing hands
/// the caller the one fact it can act on — this view has no scale at
/// that point — and it rules nothing.
///
/// **The scale reaching here from this module is already one**,
/// which is what makes this refusal readable:
/// [`View::metres_per_pixel_at`] answers `None` for a depth that is
/// not a positive finite number, so a rung request refused here is a
/// scale whose LADDER has no rung rather than a non-scale wearing a
/// legitimate value. This function is public and takes a bare `f64`,
/// though, so it owes the check on its own account.
pub fn grid_pitch(metres_per_pixel: f64) -> Option<f64> {
    let wanted = positive_length(metres_per_pixel * TARGET_PITCH_PX)?;
    // The decade below `wanted`, then the mantissa on the ladder that
    // lands closest to it in RATIO — a grid is read logarithmically,
    // so 1.0 and 2.0 are equally far from 1.41 and the linear midpoint
    // would prefer the coarser rung at every crossing.
    let decade = 10.0_f64.powf(wanted.log10().floor());
    let mut best = decade;
    let mut best_ratio = f64::INFINITY;
    for scale in [0.1, 1.0, 10.0] {
        for mantissa in PITCH_STEPS {
            let rung = decade * scale * mantissa;
            let ratio = (rung / wanted).ln().abs();
            if ratio < best_ratio {
                best_ratio = ratio;
                best = rung;
            }
        }
    }
    Some(best)
}

/// **How small a cell may get on screen before the ruling stops**, in
/// pixels.
///
/// A plane is ruled over the part of it the window sees, and a plane
/// seen at a slant runs out to the horizon, where cells shrink to
/// nothing — so the ruling stops at the view depth where one cell of
/// the pitch spans this many pixels across the view. Past that the
/// lines would crowd into a smear that says nothing a reader can use,
/// and every one of them is a line drawn. A cell's on-screen span
/// falls as one over depth, so with the looked-at point's cell inside
/// the band [`LADDER_STEP`] sets (about 51..126 px on 1-2-5), the
/// ruling reaches between about four and eleven times that point's
/// depth.
const MIN_CELL_PX: f64 = 12.0;

/// **What one grid cell aims to span on screen**, in pixels.
///
/// The pitch ladder picks the rung nearest this. A judgement; how far
/// the realized cell strays from it is what the ladder's steps are
/// worth, and [`LADDER_STEP`] is where that band is stated.
const TARGET_PITCH_PX: f64 = 80.0;

/// The mantissas of the pitch ladder — a decade, halved and fifthed.
///
/// 1-2-5 rather than powers of two because a datum grid is read as a
/// RULER against a model authored in millimetres, and 2 mm and 5 mm
/// are lengths a person has a feel for where 1.6 mm is not.
const PITCH_STEPS: [f64; 3] = [1.0, 2.0, 5.0];

/// **The ladder's widest step**: the largest ratio between adjacent
/// rungs of [`PITCH_STEPS`], counting the wrap from the last mantissa
/// to the first one a decade up. 2.5 on 1-2-5 (2 to 5).
///
/// **This is the home of the realized-cell band.** [`grid_pitch`]
/// snaps to the rung nearest in RATIO, so across a step of ratio `r`
/// the switch comes at its geometric midpoint and the cell at the
/// looked-at point is never further than √`r` from
/// [`TARGET_PITCH_PX`] either way: it spans
/// `TARGET_PITCH_PX / √r ..= TARGET_PITCH_PX * √r` pixels, about
/// 51..126 px on 1-2-5.
const LADDER_STEP: f64 = ladder_step();

const fn ladder_step() -> f64 {
    let last = PITCH_STEPS.len() - 1;
    let mut widest = PITCH_STEPS[0] * 10.0 / PITCH_STEPS[last];
    let mut i = 1;
    while i <= last {
        let step = PITCH_STEPS[i] / PITCH_STEPS[i - 1];
        if step > widest {
            widest = step;
        }
        i += 1;
    }
    widest
}

/// **The most grid lines one plane draws per direction.**
///
/// Not a budget the design expects to spend: the ruled region is the
/// window's view cut off where a cell spans [`MIN_CELL_PX`], so across
/// the view it holds at most about `viewport_px / MIN_CELL_PX` cells,
/// and up to √2 of that along a plane axis at 45° to the window —
/// around 150 on a 1280-pixel window. It is a backstop for the
/// arithmetic going wrong at an extreme — an eye inside the plane, a
/// pathological viewport — where an uncapped loop would spend the
/// frame drawing lines nobody asked for.
///
/// **It is a bound on the PATCH, not on the line list** — see
/// [`capped_span`], which is what makes a capped drawing a true
/// statement rather than a truncated one.
const MAX_GRID_LINES: usize = 512;

/// **The patch shrunk to what [`MAX_GRID_LINES`] rules**, as index
/// bounds on multiples of the pitch.
///
/// A region needing more lines than the cap allows is not ruled as
/// far as the cap reaches and then abandoned: that hands the caller a
/// ruling of part of the region wearing the shape of a ruling of all
/// of it, with no count, no flag and the loss all at one end. What
/// comes back instead is a SMALLER region, ruled completely — a true
/// statement about a patch the caller can see the edges of, and still
/// a grid, which is what the reader the ruling is for came for.
///
/// Centred on the region, because the region is centred on what the
/// camera is pointed at ([`seen_region`] boxes the plane against the
/// window, and [`grid`] takes the pitch at the looked-at point): the
/// lines that survive are the ones under the reader's eye.
///
/// **Half each rather than half the difference**: a region spanning
/// the exponent range has a difference that overflows to infinity and
/// a midpoint that does not, and an infinite midpoint would put the
/// shrunk patch nowhere.
fn capped_span(first: f64, last: f64) -> [f64; 2] {
    let cap = MAX_GRID_LINES as f64;
    // `last - first` is one less than the line count, so this is the
    // count fitting — and it reads false for a difference that
    // overflowed, which is a region that needs shrinking most.
    if last - first < cap {
        return [first, last];
    }
    let centre = first * 0.5 + last * 0.5;
    let shrunk = (centre - (cap - 1.0) * 0.5).floor();
    [shrunk, shrunk + cap - 1.0]
}

/// How long a plane's normal tick is, in PIXELS — the one mark that
/// says which way the plane faces, screen-sized because it is an
/// annotation on the plane rather than a part of it.
const NORMAL_TICK_PX: f64 = 46.0;

/// How far a drawn axis reaches from its origin, in windows.
///
/// Longer than a plane's patch is wide: an axis's whole content is a
/// direction, and a segment that ends inside the frame states one
/// weakly.
const AXIS_COVER: f64 = 1.4;

/// How long the tick across each end of a drawn axis is, in pixels.
const AXIS_TICK_PX: f64 = 18.0;

/// How long each of a frame's two arrows is, in PIXELS — the marks
/// that say which way the frame is turned, screen-sized for the reason
/// the plane's normal tick is. The +x and +y arrows are this one
/// length from the origin; which is x is said by the head
/// ([`FRAME_HEADS`]), not by the arm.
///
/// **Shorter than the narrowest cell on purpose.** The arrow's shaft
/// lies on a grid line (both run along the axis, and both start at the
/// origin), so the head is the whole of what a reader sees, and the
/// barbs are what keep it off the ruling: each points away from both
/// axes. What would crowd a head is a CROSSING — the next line across
/// the axis landing on the tip or between the barbs. At the looked-at
/// point no cell is narrower than the floor of the band
/// [`LADDER_STEP`] states (about 51 px), so an arm under that puts
/// every head inside the first cell, in open ground between the
/// origin's crossing and the next.
/// The claim is bounded the way the pitch is: an origin much further
/// from the eye than the looked-at point sees cells finer than that.
const FRAME_ARM_PX: f64 = 44.0;

// The arm has to fit inside the narrowest cell the ladder realizes,
// `FRAME_ARM_PX < TARGET_PITCH_PX / √LADDER_STEP`, squared to stay in
// const.
const _: () =
    assert!(FRAME_ARM_PX * FRAME_ARM_PX * LADDER_STEP < TARGET_PITCH_PX * TARGET_PITCH_PX);

/// How far each barb runs back from an arrow's tip, as a fraction of
/// that arrow's length. Its half-width across the axis is half again
/// of this, which is the ordinary look of an arrowhead.
const FRAME_BARB_FRACTION: f64 = 0.3;

/// **How many heads each arrow wears**, sketch +x then +y.
///
/// A grid is symmetric under a quarter turn, so two identical arrows
/// would say which pair of directions the axes are without saying
/// which of them is x. The arrows are told apart at the HEAD, and by
/// count rather than by size: two arms of one length keep the mark
/// balanced about the origin, and a doubled head (the `>>` of a
/// fast-forward) reads as a different kind of arrow where a slightly
/// larger one reads as a drawing error. The heads sit nose to tail,
/// each tip at the root of the head in front of it, which keeps them
/// two at a slant where a nested pair runs together into one.
const FRAME_HEADS: [usize; 2] = [2, 1];

/// How long each arm of a drawn point's cross is, in pixels.
///
/// A point has no extent, so this is purely an annotation's size and
/// belongs in pixels outright — at any zoom, the mark is the same
/// mark.
const POINT_ARM_PX: f64 = 14.0;

/// Which kind of datum a drawing came from — carried so a consumer can
/// say what it is pointing at without re-reading the document.
///
/// **A partition of the datum VALUES by how they are drawn**, which is
/// why four members cover `DatumValue`'s five arms: `AxisInPlane` is a
/// line in space and is drawn as the axis it is, so it carries this
/// same tag as `Axis`. Growth is held at `draw_one`'s exhaustive match
/// — a sixth datum value cannot reach a drawing without an arm
/// saying which tag it draws under.
///
/// **Not the add-datum form's `forms::DatumKindChoice`**, which names
/// what that form OFFERS rather than what a drawing IS, and which owns
/// the radio row's words and its `ALL`. The two differ by
/// `AxisInPlane` and by `FaceFrame`, and for one reason twice over:
/// each is its own choice in the form because authoring it takes a
/// PICK, and each is drawn here as the thing it evaluates to — the
/// axis, and the frame. Neither side is required to move when the
/// other does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatumKind {
    /// A plane: an outlined, gridded rectangle plus a normal tick.
    Plane,
    /// A frame: a plane's grid ruled on the frame's own axes, plus an
    /// arm along each of them.
    Frame,
    /// An axis: one segment, with a tick across each end.
    Axis,
    /// A point: three short arms crossing at the position.
    Point,
}

impl DatumKind {
    /// The word this kind is called in a sentence to a reader.
    pub fn label(self) -> &'static str {
        match self {
            Self::Plane => "plane",
            Self::Frame => "frame",
            Self::Axis => "axis",
            Self::Point => "point",
        }
    }
}

/// One datum's wireframe: which node it came from, what kind it is,
/// and its segments as a LINE LIST — two positions per segment, the
/// shape [`crate::marks::EdgeOverlay`] carries and the renderer
/// consumes.
#[derive(Debug, Clone, PartialEq)]
pub struct DatumDraw {
    /// The node whose value this draws. Carried so a caller can mark
    /// the selected datum without matching geometry back to a node.
    pub node: RecipeNodeId,
    /// Which kind of datum it is.
    pub kind: DatumKind,
    /// The segments, two positions per segment, in world metres.
    pub segments: Vec<[f64; 3]>,
}

/// **What a view made of the document's datums**: the wireframes, and
/// how many datums it drew nothing of.
///
/// A value rather than a bare `Vec` so that the second fact travels
/// with the first. Every mark in this module refuses on its own scale
/// and a datum whose every mark refuses contributes an EMPTY segment
/// list — a correct answer that reaches a caller looking exactly like
/// a document with no datums in it. A caller holding this cannot show
/// the drawings without having been handed the count as well.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DatumDraws {
    /// One per datum node the evaluation holds a value for, in
    /// document order.
    pub drawn: Vec<DatumDraw>,
}

impl DatumDraws {
    /// **How many of these datums came out with nothing drawn at
    /// all** — the fact a reader cannot get from the picture, because
    /// what it looks like is a document that has no datums.
    ///
    /// A method rather than a field: it is a reading of [`Self::drawn`]
    /// and a second copy of a count is a copy that can disagree with
    /// the thing it counts.
    ///
    /// **The property is "drew nothing", not "has no scale"**, and
    /// the two are not the same set, because THREE different refusals
    /// empty a drawing and only the first is about a scale:
    ///
    /// - a mark's own point lends it no length, so
    ///   [`View::screen_metres_at`] or [`grid_pitch`] declines it;
    /// - a ruling's index bounds overflow, so `rule_patch`'s
    ///   finiteness guard declines a direction whose `coordinate /
    ///   pitch` is no longer a number;
    /// - a ruling keeps its scale and loses its EXTENT, so
    ///   `rule_patch` declines a direction whose two endpoints round
    ///   onto the same point.
    ///
    /// The three are independent and they fire in different bands of
    /// the datum's own magnitude. A plane out at the end of the
    /// number line is emptied by all three at once — and its patch
    /// centre still HAS a scale there, because the centre is what the
    /// camera is aimed at. So a count named for the scale would be a
    /// count of the first mechanism wearing the name of the set. What
    /// every member has in common is only this: the datum is in the
    /// document and none of it is on the screen, which is the fact a
    /// reader is owed and the one this counts.
    ///
    /// **What it is NOT.** A datum that drew SOME of itself is not
    /// counted — a plane whose ruling lost a direction while its
    /// normal tick drew is on the screen, and a count of partial
    /// drawings would be a different fact.
    pub fn vanished(&self) -> usize {
        self.drawn
            .iter()
            .filter(|draw| draw.segments.is_empty())
            .count()
    }
}

/// **Every datum the landed evaluation holds a value for**, drawn for
/// `view`.
///
/// Walks the document's live nodes in order, so the answer is stable
/// and a reader gets datums in the order the tree lists them. A datum
/// node whose evaluation FAILED contributes nothing — there is no
/// value to draw and the tree's own badge already says why — and so
/// does a node this evaluation never reached.
pub fn draws(doc: &Doc<ProfileProgram>, eval: &Evaluation<f64>, view: View) -> DatumDraws {
    let mut out = Vec::new();
    for &node in doc.order() {
        // The NODE says it is a datum and the EVALUATION says what it
        // came to. Reading only the value would draw a datum for
        // anything that happens to evaluate to one; reading only the
        // node would have to re-derive geometry the evaluator already
        // produced, which is the second opinion `wire`'s own doc
        // refuses.
        if !matches!(doc.node(node), Some(Node::Datum(_))) {
            continue;
        }
        let Some(value) = eval.value(node) else {
            continue;
        };
        let ValuePayload::Datum(datum) = &value.payload else {
            continue;
        };
        out.push(draw_one(node, datum, view));
    }
    DatumDraws { drawn: out }
}

/// One datum value's wireframe.
fn draw_one(node: RecipeNodeId, datum: &DatumValue<f64>, view: View) -> DatumDraw {
    match datum {
        DatumValue::Plane { origin, normal } => DatumDraw {
            node,
            kind: DatumKind::Plane,
            segments: plane_segments(*origin, *normal, view),
        },
        DatumValue::Axis { origin, dir } => DatumDraw {
            node,
            kind: DatumKind::Axis,
            segments: axis_segments(*origin, *dir, view),
        },
        DatumValue::Point { position } => DatumDraw {
            node,
            kind: DatumKind::Point,
            segments: point_segments(*position, view),
        },
        DatumValue::Frame(f) => DatumDraw {
            node,
            kind: DatumKind::Frame,
            segments: frame_segments(f.origin(), f.u().get(), f.v().get(), view),
        },
        // Drawn from the WORLD lift, and drawn as the axis it is: the
        // sketch coordinates it was authored in are what a revolve
        // reads, not what a viewport shows, and a line in space looks
        // the same however it was written down.
        DatumValue::AxisInPlane { origin, dir, .. } => DatumDraw {
            node,
            kind: DatumKind::Axis,
            segments: axis_segments(*origin, *dir, view),
        },
    }
}

/// **A grid over the part of the plane the window is looking at**,
/// plus a tick along the normal.
///
/// Centred on the EYE's projection onto the plane, so the patch
/// follows the view instead of sitting where the datum's origin
/// happens to be; ruled at multiples of the pitch FROM THE ORIGIN, so
/// a line passes through the origin and no line moves when the eye
/// does. The two together are what makes this a grid over an infinite
/// plane rather than a rectangle somebody placed.
fn plane_segments(origin: Point3<f64>, normal: UnitVec3<f64>, view: View) -> Vec<[f64; 3]> {
    let (u, v) = basis(normal);
    grid(origin, u, v, normal.get(), view)
}

/// **A frame's grid, ruled along the frame's OWN axes**, plus an
/// ARROW along each of them.
///
/// A frame drawn exactly like a plane would be a drawing that lies: the
/// two differ by precisely the spin about the normal, so a grid ruled
/// on a display convention (which is what [`basis`] is) would show the
/// same picture for every rotation of the same frame. Here the ruling
/// IS the datum — a reader can see which way sketch +x points by
/// looking at the lines — and the two arrows name which of the two
/// directions is which, since a grid alone is symmetric under a quarter
/// turn.
///
/// **The barbs are the half that is visible, and the reason is the
/// grid.** The ruling passes through the origin along both axes (that
/// is what anchoring it there means), so a bare arm drawn along an
/// axis lies exactly on top of a grid line: in the grid's colour and
/// at the grid's width, it reads as nothing more than the two blended
/// strokes' slightly fuller line. A barb points AWAY from both axes, so
/// it is the one part of the mark that cannot coincide with the ruling.
/// The arms stay
/// because an arrowhead floating at a distance reads as debris.
fn frame_segments(origin: Point3<f64>, u: Vec3<f64>, v: Vec3<f64>, view: View) -> Vec<[f64; 3]> {
    let mut out = grid(origin, u, v, u.cross(v), view);
    // **The arms refuse on their own scale, not on the patch's.** The
    // ruling is read at the point the camera is aimed at and the arms
    // at the frame's ORIGIN, which are two different depths — so a
    // frame the view cannot rule still says which way it is turned,
    // and the two marks are never traded for one another.
    let Some(arm) = view.screen_metres_at(origin, FRAME_ARM_PX) else {
        return out;
    };
    let o = [origin.x, origin.y, origin.z];
    let (back, wide) = (arm * FRAME_BARB_FRACTION, arm * FRAME_BARB_FRACTION * 0.5);
    for ((along, across), heads) in [(u, v), (v, u)].into_iter().zip(FRAME_HEADS) {
        let at = |d: f64| {
            [
                origin.x + along.x * d,
                origin.y + along.y * d,
                origin.z + along.z * d,
            ]
        };
        out.extend([o, at(arm)]);
        for head in 0..heads {
            let tip_at = arm - back * head as f64;
            let tip = at(tip_at);
            let root = at(tip_at - back);
            for side in [1.0_f64, -1.0] {
                out.extend([
                    tip,
                    [
                        root[0] + across.x * wide * side,
                        root[1] + across.y * wide * side,
                        root[2] + across.z * wide * side,
                    ],
                ]);
            }
        }
    }
    out
}

/// The gridded patch the two plane-like datums share, ruled along
/// `u`/`v` and ticked along `normal`.
///
/// **The two marks refuse separately**, because they are scaled at
/// two different points: the ruling at the looked-at point, where the
/// pitch is read, and the tick at the origin.
/// A view that scales neither draws nothing at all
/// ([`View::screen_metres_at`], [`grid_pitch`]).
fn grid(
    origin: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    normal: Vec3<f64>,
    view: View,
) -> Vec<[f64; 3]> {
    // What the camera is looking at, dropped onto the plane, in the
    // plane's own coordinates.
    let to_target = Vec3::new(
        view.look_at.x - origin.x,
        view.look_at.y - origin.y,
        view.look_at.z - origin.z,
    );
    let (cu, cv) = (to_target.dot(u), to_target.dot(v));
    let centre = Point3::new(
        origin.x + u.x * cu + v.x * cv,
        origin.y + u.y * cu + v.y * cv,
        origin.z + u.z * cu + v.z * cv,
    );
    let mut out = Vec::new();
    // **The pitch is taken where the camera is pointed** — the
    // looked-at point dropped onto the plane — so the realized pitch
    // is the target pitch exactly where a reader is looking. Parts of
    // the plane nearer the eye than that are drawn coarser and parts
    // further are drawn finer, which is the compromise one pitch over
    // a perspective view cannot avoid.
    if let Some(pitch) = view.metres_per_pixel_at(centre).and_then(grid_pitch)
        && let Some(bounds) = seen_region(origin, u, v, view, pitch)
    {
        rule_patch(&mut out, origin, u, v, bounds, pitch);
    }
    // Which way it faces, said once and quietly, AT THE ORIGIN — the
    // one part of the drawing that is about the datum rather than
    // about the window, and scaled at its own point for that reason.
    //
    // **Dropping this costs more than dropping a ruled line, and it
    // is still the right answer.** The ruling is decoration a reader
    // can do without; the tick is the drawing's only statement of
    // which side of the plane is which, so a patch ruled without one
    // says less than a plane usually does. It is dropped anyway,
    // because a tick drawn at a length the view did not give it does
    // not say which way the plane faces either — it says whatever the
    // substituted number happened to point at.
    if let Some(tick) = view.screen_metres_at(origin, NORMAL_TICK_PX) {
        out.extend([
            [origin.x, origin.y, origin.z],
            [
                origin.x + normal.x * tick,
                origin.y + normal.y * tick,
                origin.z + normal.z * tick,
            ],
        ]);
    }
    out
}

/// **The part of the plane the window sees**, as bounds on the plane's
/// own coordinates from `origin` — `[u_lo, u_hi, v_lo, v_hi]` — or
/// `None` when the window sees none of it.
///
/// The window sees a pyramid from the eye; it is cut off at the view
/// depth where one cell of `pitch` spans [`MIN_CELL_PX`], since past
/// that a ruling is a smear. The plane meets that solid in a convex
/// polygon whose corners are where the solid's eight edges — four
/// from the eye, four around the far rectangle — cross the plane, and
/// the bounds are that polygon's box. So the ruling covers the window
/// at every tilt: to the edges of the window looking straight down,
/// and up to the cut-off looking along the plane toward its horizon.
///
/// A crossing that is not a number — a view built from values that
/// are not ([`View`]'s fields are the caller's) — refuses the whole
/// region rather than being skipped: a box drawn round the corners
/// that happened to be numbers is a region this view did not lend.
fn seen_region(
    origin: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    view: View,
    pitch: f64,
) -> Option<[f64; 4]> {
    let reach = pitch / (MIN_CELL_PX * view.metres_per_pixel_at_one_metre);
    let far = view.frustum_corners(reach);
    let normal = u.cross(v);
    let offset = |p: Point3<f64>| Vec3::new(p.x - origin.x, p.y - origin.y, p.z - origin.z);
    let height = |p: Point3<f64>| offset(p).dot(normal);
    let mut edges: Vec<(Point3<f64>, Point3<f64>)> = Vec::with_capacity(8);
    for (index, &corner) in far.iter().enumerate() {
        edges.push((view.eye, corner));
        edges.push((corner, far[(index + 1) % far.len()]));
    }
    let mut crossings: Vec<Vec3<f64>> = Vec::new();
    for (a, b) in edges {
        let (ha, hb) = (height(a), height(b));
        if !all_finite([ha, hb]) {
            return None;
        }
        // An endpoint ON the plane is a crossing of its own, counted
        // from every edge that has it; a repeat moves no bound.
        if ha == 0.0 {
            crossings.push(offset(a));
        }
        if hb == 0.0 {
            crossings.push(offset(b));
        }
        if (ha < 0.0) != (hb < 0.0) && ha != 0.0 && hb != 0.0 {
            let t = ha / (ha - hb);
            crossings.push(offset(a) + (offset(b) - offset(a)) * t);
        }
    }
    let mut bounds = [
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];
    for at in crossings {
        let (a, b) = (at.dot(u), at.dot(v));
        if !all_finite([a, b]) {
            return None;
        }
        bounds = [
            bounds[0].min(a),
            bounds[1].max(a),
            bounds[2].min(b),
            bounds[3].max(b),
        ];
    }
    // No crossing leaves the infinities standing: the plane misses
    // what the window sees.
    all_finite(bounds).then_some(bounds)
}

/// The ruled lines of one patch, appended to `out`.
///
/// Split out of [`grid`] so the refusal above it reads as one
/// condition rather than as a wrapper around forty lines.
fn rule_patch(
    out: &mut Vec<[f64; 3]>,
    origin: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    [u_lo, u_hi, v_lo, v_hi]: [f64; 4],
    pitch: f64,
) {
    let at = |a: f64, b: f64| {
        [
            origin.x + u.x * a + v.x * b,
            origin.y + u.y * a + v.y * b,
            origin.z + u.z * a + v.z * b,
        ]
    };
    // The ruled range in each direction, as index bounds on multiples
    // of the pitch from the origin, rounded OUTWARD — so the region
    // is covered by whole cells. Rounded inward, the last line of
    // each family could fall a cell short of the window's edge, with
    // the other family's lines running on past it as stubs.
    //
    // **Both directions are bounded here, once, and each family is
    // ruled between the OTHER's bounds** — so the two families close
    // into one rectangle, and a patch shrunk to the cap
    // ([`capped_span`]) is shrunk for both.
    //
    // **The bounds are asked whether they are bounds**, because the
    // cast below cannot ask: a float→int cast saturates, so `NaN`, a
    // negative difference and a span holding one line all arrive as
    // the integer zero and only the third of them means a line. The
    // region's bounds are finite by [`seen_region`]'s refusal, but
    // not so once divided by a pitch near the bottom of the exponent
    // range — and a bound that is not one stops the WHOLE patch
    // rather than leaving one family ruled between `NaN` endpoints.
    let span = |lo: f64, hi: f64| {
        let (first, last) = ((lo / pitch).floor(), (hi / pitch).ceil());
        // Rounded outward, `last < first` is a region whose bounds are
        // the wrong way round, which rules NONE; `last == first` is a
        // region that is one lattice line exactly and rules it; wider
        // rules the lines between.
        (all_finite([first, last]) && last >= first).then(|| capped_span(first, last))
    };
    let (Some(along_u), Some(along_v)) = (span(u_lo, u_hi), span(v_lo, v_hi)) else {
        return;
    };
    let mut rule = |is_u: bool, [first, last]: [f64; 2], [lo, hi]: [f64; 2]| {
        // Bounded by [`MAX_GRID_LINES`] at [`capped_span`], so the
        // cast is a cast only here, over a difference known finite,
        // non-negative and small.
        let count = ((last - first) as usize).saturating_add(1);
        let (lo, hi) = (lo * pitch, hi * pitch);
        // **A ruled line has to come out a line**, and being finite
        // is not enough to make one. The region's bounds are offsets
        // from the datum's origin; at an origin near the end of the
        // number line, the region's width is below the spacing of the
        // representable numbers around those offsets, so both bounds
        // round to one value and every segment's two endpoints land
        // on the same point. Nothing is non-finite and nothing is out of place —
        // the patch's EXTENT is simply gone, and a list of
        // zero-length segments is a ruling this function did not
        // compute wearing the shape of one it did.
        //
        // Built and then committed, so the answer is the whole
        // ruling or none of it: a direction that loses its extent
        // loses it for every line (the loss is in `hi - lo`, which
        // does not vary with `t`), and a partial ruling would be the
        // same substitution one line smaller.
        let mut lines = Vec::with_capacity(count * 2);
        for i in 0..count {
            let t = (first + i as f64) * pitch;
            let (a, b) = if is_u {
                (at(t, lo), at(t, hi))
            } else {
                (at(lo, t), at(hi, t))
            };
            let span = (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2);
            if span > 0.0 {
                lines.extend([a, b]);
            } else {
                // Zero, or not a number: the endpoints coincide, or
                // the subtraction of two overflowed coordinates left
                // no separation to measure. Neither is a line.
                return;
            }
        }
        out.append(&mut lines);
    };
    rule(true, along_u, along_v);
    rule(false, along_v, along_u);
}

/// **One segment along the axis, reaching past the window**, with a
/// screen-sized tick across each end so the length drawn reads as a
/// drawing decision rather than as the axis's own.
fn axis_segments(origin: Point3<f64>, axis: UnitVec3<f64>, view: View) -> Vec<[f64; 3]> {
    let dir = axis.get();
    // Centred and sized at the point of the axis the camera is
    // pointed at, for `plane_segments`' reason.
    let to_target = Vec3::new(
        view.look_at.x - origin.x,
        view.look_at.y - origin.y,
        view.look_at.z - origin.z,
    );
    let along = to_target.dot(dir);
    let centre = Point3::new(
        origin.x + dir.x * along,
        origin.y + dir.y * along,
        origin.z + dir.z * along,
    );
    // The segment IS the drawing here — there is no second mark to
    // fall back to — so a centre the view cannot scale draws nothing.
    let Some(half) = view.half_patch_at(centre, AXIS_COVER) else {
        return Vec::new();
    };
    let (u, _) = basis(axis);
    let at = |t: f64| {
        [
            origin.x + dir.x * t,
            origin.y + dir.y * t,
            origin.z + dir.z * t,
        ]
    };
    // Centred on the looked-at point, not on the origin, so the
    // segment covers the window wherever along it the view is.
    let (lo, hi) = (along - half, along + half);
    let mut out = vec![at(lo), at(hi)];
    for end in [lo, hi] {
        let p = at(end);
        // Each tick is scaled at its own end of the segment, so each
        // refuses for itself the way a plane's tick does.
        let Some(tick) = view.screen_metres_at(Point3::new(p[0], p[1], p[2]), AXIS_TICK_PX * 0.5)
        else {
            continue;
        };
        out.extend([
            [p[0] - u.x * tick, p[1] - u.y * tick, p[2] - u.z * tick],
            [p[0] + u.x * tick, p[1] + u.y * tick, p[2] + u.z * tick],
        ]);
    }
    out
}

/// Three arms crossing at the position — a point has no extent, so
/// what is drawn is a mark AT it rather than a picture OF it, and the
/// mark is the same size at every zoom.
fn point_segments(position: Point3<f64>, view: View) -> Vec<[f64; 3]> {
    // A point's whole drawing is one screen-sized mark, so a position
    // the view cannot scale draws no mark rather than an invented one.
    let Some(arm) = view.screen_metres_at(position, POINT_ARM_PX * 0.5) else {
        return Vec::new();
    };
    let p = [position.x, position.y, position.z];
    let mut out = Vec::with_capacity(6);
    for axis in 0..3 {
        let (mut lo, mut hi) = (p, p);
        lo[axis] -= arm;
        hi[axis] += arm;
        out.extend([lo, hi]);
    }
    out
}

/// **Two unit vectors spanning the plane `n` is normal to.**
///
/// The kernel's door, named here because two marks share it: a plane's
/// ruling and an axis's end ticks are both drawn along a pair the
/// datum does not carry, and where that pair comes from is a display
/// decision this module is answerable for. The answer is that it is
/// not a second one — the viewer does not decide how a normal is
/// completed to a frame, so it asks the door that does.
///
/// `n` is unit as a property of its type, and
/// [`UnitVec3::orthonormal_basis`] completes it to a right-handed
/// frame with no length to divide by, so there is no direction to
/// rescue here and no conditioning argument to keep true. The frame it
/// picks is discontinuous across the equator `n.z == 0`, which that
/// door states and no construction can avoid: there is no continuous
/// global frame on the sphere, so the seam is somewhere, and a datum
/// whose normal crosses it redraws its ruling turned.
fn basis(n: UnitVec3<f64>) -> (Vec3<f64>, Vec3<f64>) {
    n.orthonormal_basis()
}

/// **What a datum is drawn against**: where the eye is, and how much
/// world one pixel of this window spans.
///
/// The one place the camera and the pane's pixel size become
/// [`crate::datums::View`], so the module below stays a value over two numbers
/// rather than a borrow of the renderer. The scale is the vertical
/// field of view over the vertical pixel count — one pixel's angular
/// share — which at one metre from the eye is that many metres.
///
/// **A window that is not a number of pixels is REFUSED, by name.**
/// Neither side is floored at a pixel and neither is carried: the
/// height divides the field of view, so a zero height would lend an
/// infinite metres-per-pixel and a `NaN` height a `NaN` one, and a
/// [`View`] carrying either is a value a caller cannot tell from a
/// working one without reading its fields. A floor would be worse
/// still — it hands back the scale of a one-pixel window, a number
/// this camera and this pane did not produce.
///
/// **The refusal is the camera's own**, on the same two quantities
/// and in the same words: [`Camera::ray_through`] answers
/// [`CameraError::NotFinite`] for a viewport dimension that is not
/// finite, naming the dimension and carrying the value, and
/// [`CameraError::UnusableBounds`] for a viewport with no area. This
/// is the sibling door on the same inputs, so it reads alike rather
/// than answering in a second vocabulary of its own.
///
/// # Errors
///
/// [`CameraError::NotFinite`] for a width or a height that is not
/// finite, and [`CameraError::UnusableBounds`] for a viewport with no
/// area.
///
/// **The app's own caller reaches only one of those**, and not the
/// one the guard above it looks like it covers: `pane::viewport`
/// returns before this when [`ViewportSize::aspect`] refuses a pane
/// with no area — which is the frame a pane is first laid out and
/// every frame a splitter is dragged shut, and which also catches a
/// dimension that is `NaN`. It does NOT catch an INFINITE one:
/// `aspect` asks whether both sides are above zero, and `inf` is,
/// so a pane of infinite extent has an aspect and reaches here. That
/// arm is this door's alone.
pub fn datum_view(camera: &Camera, viewport: ViewportSize) -> Result<View, CameraError> {
    let (width, height) = (viewport.width_px, viewport.height_px);
    // Named one at a time, so the message says WHICH side was not a
    // number of pixels — the fact a caller needs and the one a single
    // "the viewport is unusable" would spend. That is also why this
    // does not route through `all_finite`: that door answers yes or
    // no ABOUT A SET, and the answer owed here names the member.
    for (what, value) in [("viewport width", width), ("viewport height", height)] {
        if !value.is_finite() {
            return Err(CameraError::NotFinite { what, value });
        }
    }
    if viewport.aspect().is_none() {
        return Err(CameraError::UnusableBounds);
    }
    Ok(View {
        eye: camera.eye(),
        look_at: camera.target(),
        metres_per_pixel_at_one_metre: 2.0 * (camera.fov_y() * 0.5).tan() / height,
        up: camera.up(),
        window_px: [width, height],
    })
}

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use super::*;

    /// **The realized cell at the looked-at point stays inside the band
    /// [`LADDER_STEP`] states, and above the frame arm, at every
    /// scale.** Swept over twelve decades of metres-per-pixel — which
    /// is zoom and depth both, since the scale at the looked-at point
    /// is their product — finely enough to land on both sides of every
    /// rung switch.
    ///
    /// **The value that makes this false** is a snapping rule that is
    /// not nearest-in-ratio (the linear midpoint would let the cell
    /// fall below the floor just past each switch), or a ladder whose
    /// widest step is not the one [`ladder_step`] reads.
    #[test]
    fn the_realized_cell_stays_in_the_band_the_arm_relies_on() {
        let floor_sq = TARGET_PITCH_PX * TARGET_PITCH_PX / LADDER_STEP;
        let ceiling_sq = TARGET_PITCH_PX * TARGET_PITCH_PX * LADDER_STEP;
        let (mut lowest, mut highest) = (f64::INFINITY, 0.0_f64);
        let mut per_pixel = 1.0e-9_f64;
        while per_pixel < 1.0e3 {
            let pitch = grid_pitch(per_pixel).expect("a positive finite scale has a rung");
            let cell = pitch / per_pixel;
            assert!(
                cell * cell >= floor_sq * (1.0 - 1.0e-9),
                "at {per_pixel:e} m/px the cell is {cell} px, under the band's floor",
            );
            assert!(
                cell * cell <= ceiling_sq * (1.0 + 1.0e-9),
                "at {per_pixel:e} m/px the cell is {cell} px, over the band's ceiling",
            );
            assert!(
                cell > FRAME_ARM_PX,
                "at {per_pixel:e} m/px the cell is {cell} px, no wider than a frame arm",
            );
            lowest = lowest.min(cell);
            highest = highest.max(cell);
            per_pixel *= 1.001;
        }
        // The sweep reached the band's edges rather than sitting inside
        // it, so the bounds above are the band and not a looser one.
        assert!(lowest * lowest < floor_sq * 1.01, "lowest cell {lowest} px");
        assert!(
            highest * highest > ceiling_sq * 0.99,
            "highest cell {highest} px"
        );
    }
}
