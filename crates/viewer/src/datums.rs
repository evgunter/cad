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
//! # A datum has no size, so the picture lends it one
//!
//! The document says where a plane is and which way it faces; it does
//! not say how big to draw it, because a plane is not big. Every
//! dimension here is therefore a multiple of an EXTENT the caller
//! supplies — the scene's own bounding diagonal — so a datum on a
//! 2 mm boss and one on a 2 m plate are drawn at the same size
//! relative to what they are beside. A document with no geometry at
//! all has no extent to scale against and gets a fallback, stated
//! once, in [`FALLBACK_EXTENT`].
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

/// **What a datum is drawn at when the document has no geometry to
/// scale against**, in metres.
///
/// A fresh document whose first act is a datum has no extent — there
/// is nothing on screen for a plane to be sized relative to — and a
/// datum drawn at zero would be a datum nobody can see. 20 mm is the
/// order of the shipped startup document, so the first datum in an
/// empty document lands at a size the camera can frame.
pub const FALLBACK_EXTENT: f64 = 0.02;

/// How much of the scene's diagonal a drawn plane spans.
const PLANE_SPAN: f64 = 0.55;

/// How much of the scene's diagonal a drawn axis spans, end to end.
/// Longer than a plane is wide, because an axis's whole content is a
/// direction and a short one states it weakly.
const AXIS_SPAN: f64 = 0.9;

/// How much of the scene's diagonal a drawn point's arms span.
const POINT_SPAN: f64 = 0.06;

/// How long a plane's normal tick is, as a share of the diagonal — the
/// one mark that says which way the plane faces, and deliberately much
/// shorter than the plane is wide so it reads as an annotation on the
/// plane rather than as an axis through it.
const NORMAL_TICK: f64 = 0.12;

/// How many interior grid lines a drawn plane carries per axis.
///
/// Three, which is what makes a rectangle read as a SURFACE rather
/// than as four segments that happen to meet: an empty outline seen
/// edge-on is indistinguishable from a single line. More would be a
/// texture competing with the model behind it.
const PLANE_GRID: usize = 3;

/// Which kind of datum a drawing came from — carried so a consumer can
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

/// **Every datum the landed evaluation holds a value for**, drawn at
/// `extent`.
///
/// Walks the document's live nodes in order, so the answer is stable
/// and a reader gets datums in the order the tree lists them. A datum
/// node whose evaluation FAILED contributes nothing — there is no
/// value to draw and the tree's own badge already says why — and so
/// does a node this evaluation never reached.
///
/// `extent` is the scene's bounding diagonal in metres; a
/// non-positive or non-finite one falls back to [`FALLBACK_EXTENT`]
/// rather than drawing a datum of zero size that nobody could see.
pub fn draws(doc: &Doc<ProfileProgram>, eval: &Evaluation<f64>, extent: f64) -> Vec<DatumDraw> {
    let extent = if extent.is_finite() && extent > 0.0 {
        extent
    } else {
        FALLBACK_EXTENT
    };
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
        out.push(draw_one(node, datum, extent));
    }
    out
}

/// One datum value's wireframe.
fn draw_one(node: RecipeNodeId, datum: &DatumValue<f64>, extent: f64) -> DatumDraw {
    match datum {
        DatumValue::Plane { origin, normal } => DatumDraw {
            node,
            kind: DatumKind::Plane,
            segments: plane_segments(*origin, normal.get(), extent),
        },
        DatumValue::Axis { origin, dir } => DatumDraw {
            node,
            kind: DatumKind::Axis,
            segments: axis_segments(*origin, dir.get(), extent),
        },
        DatumValue::Point { position } => DatumDraw {
            node,
            kind: DatumKind::Point,
            segments: point_segments(*position, extent),
        },
    }
}

/// A gridded rectangle in the plane, plus a tick along the normal.
fn plane_segments(origin: Point3<f64>, normal: Vec3<f64>, extent: f64) -> Vec<[f64; 3]> {
    let (u, v) = basis(normal);
    let half = extent * PLANE_SPAN * 0.5;
    let at = |a: f64, b: f64| {
        [
            origin.x + u.x * a + v.x * b,
            origin.y + u.y * a + v.y * b,
            origin.z + u.z * a + v.z * b,
        ]
    };
    let mut out = Vec::new();
    // The outline and the interior grid are one loop: the border is
    // just the first and last line of each family, so an off-by-one
    // that dropped an edge would drop a grid line too and be visible
    // rather than subtle.
    let lines = PLANE_GRID + 2;
    for i in 0..lines {
        // `lines - 1` is at least 1 because PLANE_GRID is at least 0
        // and two borders always exist.
        let t = -half + 2.0 * half * (i as f64) / ((lines - 1) as f64);
        out.extend([at(t, -half), at(t, half)]);
        out.extend([at(-half, t), at(half, t)]);
    }
    // Which way it faces, said once and quietly.
    let tick = extent * NORMAL_TICK;
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

/// One segment along the axis, with a short cross tick at each end so
/// the extent drawn reads as a drawing decision rather than as the
/// axis's own length.
fn axis_segments(origin: Point3<f64>, dir: Vec3<f64>, extent: f64) -> Vec<[f64; 3]> {
    let half = extent * AXIS_SPAN * 0.5;
    let (u, _) = basis(dir);
    let tick = extent * POINT_SPAN * 0.5;
    let at = |t: f64| {
        [
            origin.x + dir.x * t,
            origin.y + dir.y * t,
            origin.z + dir.z * t,
        ]
    };
    let mut out = vec![at(-half), at(half)];
    for end in [-half, half] {
        let p = at(end);
        out.extend([
            [p[0] - u.x * tick, p[1] - u.y * tick, p[2] - u.z * tick],
            [p[0] + u.x * tick, p[1] + u.y * tick, p[2] + u.z * tick],
        ]);
    }
    out
}

/// Three arms crossing at the position — a point has no extent, so
/// what is drawn is a mark AT it rather than a picture OF it.
fn point_segments(position: Point3<f64>, extent: f64) -> Vec<[f64; 3]> {
    let arm = extent * POINT_SPAN * 0.5;
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
/// `n` arrives normalized — it comes out of a `UnitVec3`, which has no
/// unnormalized spelling — so this only has to choose a direction, not
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
