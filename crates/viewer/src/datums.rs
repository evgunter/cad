//! **Datums, drawn.** The wireframe a plane, an axis or a point is
//! shown as in the viewport — a pure function of the landed evaluation
//! and the picture's own extent, with no toolkit and no GPU in sight.
//!
//! # Why this is a value and not a paint routine
//!
//! G1's rule, the same one [`crate::pick::edge_overlay`] obeys: a test
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
//! - **The patch** is centred on the eye's own projection onto the
//!   plane and sized to overflow the window ([`PATCH_COVER`]), so
//!   panning cannot leave it and zooming out cannot shrink it away.
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
//! not from the patch's centre: the origin is a real point on the
//! plane and a grid line through it is a fact, where a grid indexed
//! off a moving window would slide as the eye moved.
//!
//! **Perspective makes one pitch a compromise**, stated rather than
//! hidden. World-per-pixel grows with depth, so a plane seen at a
//! grazing angle is denser at its far end than at its near one. The
//! scale is taken at the datum's ORIGIN, and the error is in the safe
//! direction: the far end is finer than asked for, never coarser, so
//! no part of a drawn plane can open into a hole the target pitch was
//! chosen to prevent.
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

use pncad::document::{
    DatumValue, Doc, Evaluation, Node, ProfileProgram, RecipeNodeId, ValuePayload,
};
use pncad::geom_core::{Point3, Vec3};

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
    /// distance from it); the target gives the CENTRE (where on an
    /// infinite plane to put the drawn patch). An earlier version
    /// centred on the eye's own perpendicular foot, which is a
    /// well-defined point and the wrong one: on a plane seen at a
    /// grazing angle that foot sits far from what the camera is
    /// pointed at, so the grid drifted into a corner of the window
    /// exactly when the view was most oblique.
    pub look_at: Point3<f64>,
    /// **World metres one pixel spans at one metre from the eye.** The
    /// scale at any other depth is this times that depth, which is the
    /// whole of the perspective arithmetic here.
    ///
    /// From the camera as `2 * tan(fov_y / 2) / viewport_height_px`.
    pub metres_per_pixel_at_one_metre: f64,
    /// The window's larger side, in pixels — what a patch has to
    /// overflow to be un-pannable-off.
    pub viewport_px: f64,
}

impl View {
    /// World metres one pixel spans at `point`.
    ///
    /// Floored at a hair above zero so a datum lying exactly at the
    /// eye — reachable by flying the camera into a plane — produces a
    /// degenerate drawing rather than a division by zero.
    fn metres_per_pixel_at(&self, point: Point3<f64>) -> f64 {
        let depth = ((point.x - self.eye.x).powi(2)
            + (point.y - self.eye.y).powi(2)
            + (point.z - self.eye.z).powi(2))
        .sqrt();
        (depth * self.metres_per_pixel_at_one_metre).max(f64::MIN_POSITIVE)
    }

    /// How much world the window spans at `point`'s depth.
    fn window_metres_at(&self, point: Point3<f64>) -> f64 {
        self.metres_per_pixel_at(point) * self.viewport_px.max(1.0)
    }
}

/// **The pitch one cell is drawn at**: the rung of the 1-2-5 ladder
/// whose on-screen span is nearest [`TARGET_PITCH_PX`].
///
/// Public because it is the module's one arithmetic claim worth
/// asserting on its own — that the realized pitch stays inside the
/// band the ladder's step size implies, at every scale.
pub fn grid_pitch(metres_per_pixel: f64) -> f64 {
    let wanted = metres_per_pixel * TARGET_PITCH_PX;
    if !wanted.is_finite() || wanted <= 0.0 {
        return f64::MIN_POSITIVE;
    }
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
    best
}

/// **How many windows across a drawn plane's patch spans.**
///
/// Above one, so the patch always overflows the window and a pan
/// cannot run off its edge; not far above one, because every extra
/// window of patch is grid lines drawn outside the frame.
///
/// **It cannot be made large enough for every view, and this is the
/// one limit worth stating.** A plane seen at a grazing angle recedes
/// to a horizon, so no finite patch covers what is on screen and the
/// far edge is visible as a straight line across the picture. Two and
/// a bit windows puts that edge well out of the way at any ordinary
/// angle and costs about 35 lines a direction; chasing the grazing
/// case properly means scaling by the view's inclination, which buys
/// a rarely-seen edge with arithmetic that blows up as the angle goes
/// to zero.
const PATCH_COVER: f64 = 2.2;

/// **What one grid cell aims to span on screen**/// **What one grid cell aims to span on screen**, in pixels.
///
/// The pitch ladder picks the rung nearest this. A judgement, and the
/// range around it is what the ladder's steps are worth: at a 1-2-5
/// ladder a rung is at most 2.5x the one below, so the realized pitch
/// stays inside roughly 40..160 px of this whatever the zoom.
const TARGET_PITCH_PX: f64 = 80.0;

/// The mantissas of the pitch ladder — a decade, halved and fifthed.
///
/// 1-2-5 rather than powers of two because a datum grid is read as a
/// RULER against a model authored in millimetres, and 2 mm and 5 mm
/// are lengths a person has a feel for where 1.6 mm is not.
const PITCH_STEPS: [f64; 3] = [1.0, 2.0, 5.0];

/// **The most grid lines one plane draws per direction.**
///
/// Not a budget the design expects to spend: a patch of
/// [`PATCH_COVER`] windows at [`TARGET_PITCH_PX`] per cell needs about
/// `PATCH_COVER * viewport_px / TARGET_PITCH_PX` lines, which is
/// around 26 on a 1280-pixel window. It is a backstop for the
/// arithmetic going wrong at an extreme — an eye inside the plane, a
/// pathological viewport — where an uncapped loop would spend the
/// frame drawing lines nobody asked for.
const MAX_GRID_LINES: usize = 96;

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

/// How long each arm of a drawn point's cross is, in pixels.
///
/// A point has no extent, so this is purely an annotation's size and
/// belongs in pixels outright — at any zoom, the mark is the same
/// mark.
const POINT_ARM_PX: f64 = 14.0;

/// Which kind of datum a drawing came from/// Which kind of datum a drawing came from — carried so a consumer can
/// say what it is pointing at without re-reading the document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatumKind {
    /// A plane: an outlined, gridded rectangle plus a normal tick.
    Plane,
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
            Self::Axis => "axis",
            Self::Point => "point",
        }
    }
}

/// One datum's wireframe: which node it came from, what kind it is,
/// and its segments as a LINE LIST — two positions per segment, the
/// shape [`crate::pick::EdgeOverlay`] carries and the renderer
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

/// **Every datum the landed evaluation holds a value for**, drawn for
/// `view`.
///
/// Walks the document's live nodes in order, so the answer is stable
/// and a reader gets datums in the order the tree lists them. A datum
/// node whose evaluation FAILED contributes nothing — there is no
/// value to draw and the tree's own badge already says why — and so
/// does a node this evaluation never reached.
pub fn draws(doc: &Doc<ProfileProgram>, eval: &Evaluation<f64>, view: View) -> Vec<DatumDraw> {
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
    out
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
fn plane_segments(origin: Point3<f64>, normal: Vec3<f64>, view: View) -> Vec<[f64; 3]> {
    let (u, v) = basis(normal);
    // What the camera is looking at, dropped onto the plane, in the
    // plane's own coordinates.
    let to_target = Vec3::new(
        view.look_at.x - origin.x,
        view.look_at.y - origin.y,
        view.look_at.z - origin.z,
    );
    let (cu, cv) = (dot(to_target, u), dot(to_target, v));
    let centre = Point3::new(
        origin.x + u.x * cu + v.x * cv,
        origin.y + u.y * cu + v.y * cv,
        origin.z + u.z * cu + v.z * cv,
    );
    // **The scale is taken at the CENTRE of the patch** — the point
    // of the plane the camera is pointed at, so the realized pitch is
    // the target pitch exactly where a reader is looking. Parts of the
    // plane nearer the eye than that are drawn coarser and parts
    // further are drawn finer, which is the compromise one pitch over
    // a perspective view cannot avoid.
    let per_pixel = view.metres_per_pixel_at(centre);
    let half = view.window_metres_at(centre) * PATCH_COVER * 0.5;
    let pitch = grid_pitch(per_pixel);
    let at = |a: f64, b: f64| {
        [
            origin.x + u.x * a + v.x * b,
            origin.y + u.y * a + v.y * b,
            origin.z + u.z * a + v.z * b,
        ]
    };
    let mut out = Vec::new();
    // The ruled range in each direction, as index bounds on multiples
    // of the pitch from the origin. `ceil`/`floor` outward, so the
    // patch is covered rather than nearly covered.
    let mut rule = |along_u: bool, from: f64, to: f64, lo: f64, hi: f64| {
        let first = (from / pitch).ceil();
        let last = (to / pitch).floor();
        let count = ((last - first) as usize).min(MAX_GRID_LINES);
        for i in 0..=count {
            let t = (first + i as f64) * pitch;
            if along_u {
                out.extend([at(t, lo), at(t, hi)]);
            } else {
                out.extend([at(lo, t), at(hi, t)]);
            }
        }
    };
    let (u_lo, u_hi) = (cu - half, cu + half);
    let (v_lo, v_hi) = (cv - half, cv + half);
    rule(true, u_lo, u_hi, v_lo, v_hi);
    rule(false, v_lo, v_hi, u_lo, u_hi);
    // Which way it faces, said once and quietly, AT THE ORIGIN — the
    // one part of the drawing that is about the datum rather than
    // about the window.
    let tick = view.metres_per_pixel_at(origin) * NORMAL_TICK_PX;
    out.extend([
        [origin.x, origin.y, origin.z],
        [
            origin.x + normal.x * tick,
            origin.y + normal.y * tick,
            origin.z + normal.z * tick,
        ],
    ]);
    out
}

/// **One segment along the axis, reaching past the window**, with a
/// screen-sized tick across each end so the length drawn reads as a
/// drawing decision rather than as the axis's own.
fn axis_segments(origin: Point3<f64>, dir: Vec3<f64>, view: View) -> Vec<[f64; 3]> {
    // Centred and sized at the point of the axis the camera is
    // pointed at, for `plane_segments`' reason.
    let to_target = Vec3::new(
        view.look_at.x - origin.x,
        view.look_at.y - origin.y,
        view.look_at.z - origin.z,
    );
    let along = dot(to_target, dir);
    let centre = Point3::new(
        origin.x + dir.x * along,
        origin.y + dir.y * along,
        origin.z + dir.z * along,
    );
    let half = view.window_metres_at(centre) * AXIS_COVER * 0.5;
    let (u, _) = basis(dir);
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
        let tick = view.metres_per_pixel_at(Point3::new(p[0], p[1], p[2])) * AXIS_TICK_PX * 0.5;
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
    let arm = view.metres_per_pixel_at(position) * POINT_ARM_PX * 0.5;
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

/// The dot product, spelled here for [`cross`]'s reason.
fn dot(a: Vec3<f64>, b: Vec3<f64>) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// **Two unit vectors spanning the plane `n` is normal to.**
///
/// `n` arrives normalized — `DatumValue` states that as its contract,
/// and the evaluator refuses a degenerate direction before one of
/// these values exists — so this only has to choose a direction, not
/// rescue one. The seed is whichever world axis `n` is least aligned
/// with, which is what keeps the cross product away from zero: a
/// vector cannot be nearly parallel to the axis it has its smallest
/// component along.
fn basis(n: Vec3<f64>) -> (Vec3<f64>, Vec3<f64>) {
    let seed = if n.x.abs() <= n.y.abs() && n.x.abs() <= n.z.abs() {
        Vec3::new(1.0, 0.0, 0.0)
    } else if n.y.abs() <= n.z.abs() {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(0.0, 0.0, 1.0)
    };
    let u = unit(cross(n, seed));
    (u, unit(cross(n, u)))
}

/// The cross product, spelled here because this module's vectors are
/// display scaffolding and never reach a predicate.
fn cross(a: Vec3<f64>, b: Vec3<f64>) -> Vec3<f64> {
    Vec3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

/// `v` normalized, or the x axis where it has no length.
///
/// The fallback is unreachable from [`basis`] — a unit normal crossed
/// with the world axis it is least aligned with has length at least
/// `1/√3` — and it is here rather than an assertion because a datum
/// nobody can see is a better failure than a panic in a paint path.
fn unit(v: Vec3<f64>) -> Vec3<f64> {
    let len = (v.x.powi(2) + v.y.powi(2) + v.z.powi(2)).sqrt();
    if len > 0.0 {
        Vec3::new(v.x / len, v.y / len, v.z / len)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
