//! **The local certificates the injectivity question rests on**
//! (ERROR-DESIGN's *Self-intersection scope*): what
//! [`crate::clearance`]'s sweep asks of a cell pair that TOUCHES — two
//! cells that may share a point of the abstract boundary M — where a
//! distance test has nothing to say, because the distance there is 0 by
//! construction.
//!
//! Everything here reads geometry already lifted to [`Interval`]
//! ([`Boundary`], built by the engine's window door), and every
//! decision goes through the funnel at a named site.
//!
//! # Touching, in M
//!
//! Two cells of different faces share a point of M only on an edge or a
//! vertex the two faces share. A cell whose ball ([`Ball`]) is decided
//! clear of a shared feature holds no point of it, so a pair touches
//! only through the shared features neither ball is clear of
//! ([`Near`]). Two cells of one face touch when their chart rectangles
//! meet, or through the face's own identifications: a seam edge both of
//! whose halves bound the face, a vertex the face's loops pass more than
//! once, the vertex of a null edge.
//!
//! # The arms
//!
//! - **Monotone** ([`monotone`]), within one face: on a convex chart
//!   rectangle `H`, a linear `L : ℝ³ → ℝ²` with `J(L∘S) + J(L∘S)ᵀ`
//!   positive definite at every point of `H` makes `L∘S` strongly
//!   monotone, `⟨L∘S(p) − L∘S(q), p − q⟩ > 0` for `p ≠ q`, so `S` is
//!   injective on `H`.
//! - **Departure** ([`certify_pair`]), across a shared edge or vertex:
//!   a separator `ψ`, vanishing on the shared cell, takes one strict sign
//!   on one face's part of the pair and the other strict sign on the
//!   other's, so a coincidence can only happen on the shared cell, where
//!   `f` is injective. `ψ` is a face's own analytic carrier (the other
//!   face departs it to one side), a plane through straight shared edges,
//!   or the graph of the shared edge under a projection.
//! - **Second order**, at a wedge-0/2π edge, is not built: such an edge
//!   refuses typed, naming it.
//!
//! Each arm's sign is read off a [`Presented`] face: the face's part of
//! the cell written as the image of a rectangle on whose sides the shared
//! cell lies, and a derivative across those sides.

use std::collections::{BTreeMap, BTreeSet};

use geom::{Curve3, Surface};
use geom_core::interval::Interval;
use geom_core::{Band, Bounds, Margin, Point2, Point3, Real, Sign, Vec3};

use crate::cell_region::Ball;
use crate::clearance::{CertificateArm, EngineRefusal, SharedCell};
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::validate::decide;

/// The funnel site of a presented face's side that lies on the shared
/// cell: the separator over that side, metres. `Zero` is "the side lies
/// on the separator at the run's tolerance"; anything else fails the
/// arm.
pub(crate) const ON_SHARED: &str = "self_intersection_on_shared";
/// The funnel site of a separator's sign over a presented face: its
/// derivative across the face's shared side times the rectangle's extent
/// across it, or its value where no side is shared, metres.
pub(crate) const DEPARTURE: &str = "self_intersection_departure";
/// The funnel site of the graph arm's monotone edge: the edge's advance
/// along the projection's first axis over its piece, metres.
pub(crate) const GRAPH: &str = "self_intersection_graph";
/// The funnel site of a planar presentation's reach along its edge:
/// how far inside the edge's two ends the cell's extent stays, metres.
pub(crate) const SPAN: &str = "self_intersection_span";
/// The funnel site of a planar corner's convexity: how far the second
/// edge's far end stands on the first edge's face side, metres.
pub(crate) const CORNER: &str = "self_intersection_corner";
/// The funnel site of a vertex at a cone's apex: its distance from the
/// apex, metres. `Zero` is the apex.
pub(crate) const APEX: &str = "self_intersection_apex";
/// The funnel site of a cone face's nappe: a vertex's signed height
/// along the axis from the apex, metres. Its sign is the nappe.
pub(crate) const NAPPE: &str = "self_intersection_nappe";
/// The funnel site of a presentation clipped to a shared line: the
/// rectangle's extent across the line times the chart's speed across it
/// (an upper bound, `rect_jet`'s), metres. `Zero` reads the strip as
/// holding no point of the face off the line — the arm's one ε
/// concession ([`present_chart`]).
pub(crate) const SLIVER: &str = "self_intersection_sliver";
/// The funnel site of a wedge-0/2π shared edge: the length of the sum of
/// the two faces' unit outward normals at the shared side's middle,
/// times the ball's reach, metres. `Zero` is opposed normals.
pub(crate) const CUSP: &str = "self_intersection_cusp";
/// The funnel site of the monotone arm: the least eigenvalue of the
/// symmetric part of the projected chart's Jacobian, each column scaled
/// by the rectangle's span on its axis, metres.
pub(crate) const MONOTONE: &str = "self_intersection_monotone";
/// The funnel site of a torus's ring condition: major radius less minor
/// radius, metres.
pub(crate) const RING_TORUS: &str = "self_intersection_ring_torus";

// ------------------------------------------------- the lifted boundary

/// A chart axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Axis {
    /// The first chart coordinate.
    U,
    /// The second.
    V,
}

impl Axis {
    /// This axis's range of a rectangle `(u, v)`.
    fn of<R: Copy>(self, (u, v): (R, R)) -> R {
        match self {
            Self::U => u,
            Self::V => v,
        }
    }

    /// The other axis.
    fn other(self) -> Self {
        match self {
            Self::U => Self::V,
            Self::V => Self::U,
        }
    }

    /// Its index in a `[u, v]` pair.
    fn index(self) -> usize {
        match self {
            Self::U => 0,
            Self::V => 1,
        }
    }
}

/// A use of an edge whose chart image is a coordinate line.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Iso {
    /// The coordinate the image holds.
    pub(crate) held: Axis,
    /// Its value.
    pub(crate) at: Interval,
    /// The image runs once round a periodic moving coordinate: every
    /// chart point of the line is a point of the edge.
    pub(crate) full: bool,
}

/// One half-edge of a face on an edge.
#[derive(Clone, Debug)]
pub(crate) struct Use {
    /// The loop runs it forward in the curve's parameter.
    pub(crate) plus: bool,
    /// Its chart image's points at the curve's start and end parameters.
    pub(crate) image: Option<[Point2<Interval>; 2]>,
    /// The image, where it is a coordinate line.
    pub(crate) iso: Option<Iso>,
}

/// One edge of a face's boundary, lifted.
#[derive(Clone, Debug)]
pub(crate) struct Rim {
    pub(crate) key: EdgeKey,
    /// The carrier and its parameter span; `None` for null scaffolding,
    /// whose locus is its vertex.
    pub(crate) arc: Option<(Curve3<Interval>, (Interval, Interval))>,
    /// The vertices at the curve's start and end.
    pub(crate) ends: [VertexKey; 2],
    /// The face's half-edges on it: two on a seam.
    pub(crate) uses: Vec<Use>,
}

/// **A face's boundary, lifted**: its sense, its edges with their chart
/// images, its vertices, and its own extent in its chart.
#[derive(Clone, Debug, Default)]
pub(crate) struct Boundary {
    pub(crate) sense: bool,
    pub(crate) edges: Vec<Rim>,
    pub(crate) vertices: BTreeMap<VertexKey, Point3<Interval>>,
    /// **The face's own extent on each chart axis**, `[u, v]`: the hull
    /// of its outer loop's lift ([`crate::pcurves::loop_lift`]), one
    /// continuous chain, so on a periodic axis it is the face's span on
    /// ONE sheet and not wrapped. Exact as a hull: it is read only where
    /// every row of the loop is a coordinate line whose moving coordinate
    /// is monotone, so no row bulges past its ends. A span a whole turn
    /// wide (a wrapping loop) is every value of the axis. `None` where
    /// the loop has no lift or a row is not such a line.
    pub(crate) extent: Option<[(f64, f64); 2]>,
}

impl Boundary {
    fn index(&self, key: EdgeKey) -> Option<usize> {
        self.edges.iter().position(|e| e.key == key)
    }

    /// The vertices the face's own chart identifies: one its loops start
    /// two half-edges at, and a null edge's.
    fn self_vertices(&self) -> BTreeSet<VertexKey> {
        let mut starts: BTreeMap<VertexKey, usize> = BTreeMap::new();
        let mut out = BTreeSet::new();
        for e in &self.edges {
            if e.arc.is_none() {
                out.extend(e.ends);
            }
            for u in &e.uses {
                *starts
                    .entry(if u.plus { e.ends[0] } else { e.ends[1] })
                    .or_default() += 1;
            }
        }
        out.extend(starts.into_iter().filter(|&(_, n)| n > 1).map(|(v, _)| v));
        out
    }

    /// The vertices of null edges.
    fn collapsed(&self) -> BTreeSet<VertexKey> {
        self.edges
            .iter()
            .filter(|e| e.arc.is_none())
            .flat_map(|e| e.ends)
            .collect()
    }
}

// ------------------------------------------------------ touching, in M

/// **What two faces share**: the features a point of M common to both
/// lies on. For one face against itself, the features its chart
/// identifies.
#[derive(Clone, Debug, Default)]
pub(crate) struct Shared {
    /// `(index in the first face's boundary, index in the second's)`.
    edges: Vec<(usize, usize)>,
    vertices: Vec<VertexKey>,
}

impl Shared {
    /// What `x` and `y` share; `same` says they are one face.
    pub(crate) fn of(x: &Boundary, y: &Boundary, same: bool) -> Self {
        if same {
            return Self {
                edges: (0..x.edges.len())
                    .filter(|&i| x.edges.get(i).is_some_and(|e| e.uses.len() > 1))
                    .map(|i| (i, i))
                    .collect(),
                vertices: x.self_vertices().into_iter().collect(),
            };
        }
        Self {
            edges: x
                .edges
                .iter()
                .enumerate()
                .filter_map(|(i, e)| Some((i, y.index(e.key)?)))
                .collect(),
            vertices: x
                .vertices
                .keys()
                .filter(|v| y.vertices.contains_key(v))
                .copied()
                .collect(),
        }
    }

    /// Whether the two faces share nothing.
    pub(crate) fn is_empty(&self) -> bool {
        self.edges.is_empty() && self.vertices.is_empty()
    }

    /// The cell the two scopes meet on: an edge where they share one,
    /// else a vertex.
    pub(crate) fn cell(&self, x: &Boundary) -> Option<SharedCell> {
        self.edges
            .first()
            .and_then(|&(i, _)| x.edges.get(i))
            .map(|e| SharedCell::Edge(e.key))
            .or_else(|| self.vertices.first().map(|&v| SharedCell::Vertex(v)))
    }

    /// A vertex the two faces share, if any: an end of a shared edge
    /// first.
    pub(crate) fn vertex(&self, x: &Boundary) -> Option<VertexKey> {
        self.edges
            .first()
            .and_then(|&(i, _)| x.edges.get(i))
            .map(|e| e.ends[0])
            .or_else(|| self.vertices.first().copied())
    }
}

/// **A cell's local boundary**: the face's edges and vertices its ball
/// is not decided clear of, as indices into the face's [`Boundary`].
#[derive(Clone, Debug, Default)]
pub(crate) struct Local {
    edges: Vec<usize>,
    vertices: Vec<VertexKey>,
}

impl Local {
    /// Every feature of `b` the ball is not decided clear of.
    pub(crate) fn of(b: &Boundary, ball: Ball, band: Band) -> Self {
        let vertices: Vec<VertexKey> = b
            .vertices
            .iter()
            .filter(|&(_, &p)| !ball.clear_of_point(p, band))
            .map(|(&k, _)| k)
            .collect();
        let edges = b
            .edges
            .iter()
            .enumerate()
            .filter(|(_, e)| match &e.arc {
                Some((carrier, span)) => !ball.clear_of_arc(carrier, *span, band),
                None => e.ends.iter().any(|v| vertices.contains(v)),
            })
            .map(|(i, _)| i)
            .collect();
        Self { edges, vertices }
    }
}

/// **The shared features a cell pair touches through**: those neither
/// cell's ball is clear of. Empty means the two cells hold no common
/// point of M.
#[derive(Clone, Debug, Default)]
pub(crate) struct Near {
    /// `(index in the first face's boundary, index in the second's)`.
    edges: Vec<(usize, usize)>,
    vertices: Vec<VertexKey>,
}

impl Near {
    pub(crate) fn of(shared: &Shared, la: &Local, lb: &Local) -> Self {
        Self {
            edges: shared
                .edges
                .iter()
                .filter(|(i, j)| la.edges.contains(i) && lb.edges.contains(j))
                .copied()
                .collect(),
            vertices: shared
                .vertices
                .iter()
                .filter(|v| la.vertices.contains(v) && lb.vertices.contains(v))
                .copied()
                .collect(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.edges.is_empty() && self.vertices.is_empty()
    }

    /// The cell a refusal names.
    fn cell(&self, x: &Boundary) -> SharedCell {
        match (self.edges.first(), self.vertices.first()) {
            (Some(&(i, _)), _) => x
                .edges
                .get(i)
                .map_or(SharedCell::Face(FaceKey::default()), |e| {
                    SharedCell::Edge(e.key)
                }),
            (None, Some(&v)) => SharedCell::Vertex(v),
            (None, None) => SharedCell::Face(FaceKey::default()),
        }
    }

    /// The near edges on one side, as indices into that face's boundary.
    fn side(&self, second: bool) -> Vec<usize> {
        self.edges
            .iter()
            .map(|&(i, j)| if second { j } else { i })
            .collect()
    }
}

// ------------------------------------------------------------ a piece

/// **One face's part of a cell pair**, as the arms read it.
pub(crate) struct Piece<'a> {
    pub(crate) face: FaceKey,
    pub(crate) surface: &'a Surface<Interval>,
    /// The window is the whole patch (an iso-bounded NURBS face).
    pub(crate) whole: bool,
    pub(crate) boundary: &'a Boundary,
    pub(crate) u: (f64, f64),
    pub(crate) v: (f64, f64),
    pub(crate) local: &'a Local,
    /// The cell's ball: its reach levers a decision into metres.
    pub(crate) ball: Ball,
}

/// What a touching cell pair came to.
#[derive(Debug)]
pub(crate) enum Outcome {
    /// A local certificate holds: the pair's union is injective.
    Certified,
    /// No arm held here; a smaller pair may.
    Split {
        arm: CertificateArm,
        shared: SharedCell,
    },
    /// No arm can hold at any depth: refused now, typed.
    Refuse(EngineRefusal),
    /// The two faces meet at wedge 0 or 2π on the shared edge, which
    /// only the second-order arm reads.
    Cusp { shared: SharedCell },
}

/// **The analytic carrier's own injectivity**: every analytic carrier
/// the engine admits is one-to-one on its chart's fundamental domain,
/// and a face is a region of it. A torus is, where `R > r`.
///
/// # Errors
///
/// The torus that is not decided a ring.
pub(crate) fn by_kind(
    face: FaceKey,
    surface: &Surface<Interval>,
    band: Band,
) -> Result<(), EngineRefusal> {
    match surface {
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => match decide(RING_TORUS, Margin::of(*major_radius - *minor_radius), band) {
            Ok(Sign::Positive) => Ok(()),
            _ => Err(EngineRefusal::TorusNotRing { face }),
        },
        _ => Ok(()),
    }
}

/// Whether two closed chart rectangles of one face meet. The cells are
/// the split rule's own bisections of one window, so this compares
/// structure, not geometry.
pub(crate) fn rects_meet(a: ((f64, f64), (f64, f64)), b: ((f64, f64), (f64, f64))) -> bool {
    a.0.0 <= b.0.1 && b.0.0 <= a.0.1 && a.1.0 <= b.1.1 && b.1.0 <= a.1.1
}

/// **A touching pair of one face**, whose rectangles do not meet: it
/// touches through the face's own identifications.
pub(crate) fn certify_self(x: &Piece<'_>, y: &Piece<'_>, near: &Near, band: Band) -> Outcome {
    let collapsed = x.boundary.collapsed();
    if let Some(&v) = near.vertices.iter().find(|v| collapsed.contains(v)) {
        return Outcome::Refuse(EngineRefusal::CollapsedChart {
            face: x.face,
            vertex: v,
        });
    }
    match near.edges.as_slice() {
        [_] => departure(x, y, near, band, false),
        _ => Outcome::Split {
            arm: CertificateArm::Departure,
            shared: near.cell(x.boundary),
        },
    }
}

/// **A touching pair of two faces**: the departure arm, its separators
/// in a fixed order.
pub(crate) fn certify_pair(x: &Piece<'_>, y: &Piece<'_>, near: &Near, band: Band) -> Outcome {
    departure(x, y, near, band, true)
}

// ------------------------------------------------------------ helpers

fn iv((lo, hi): (f64, f64)) -> Interval {
    Interval::from_bounds(lo, hi)
}

fn at(x: f64) -> Interval {
    Interval::from_bounds(x, x)
}

fn mid(x: Interval) -> f64 {
    geom::mid_param(x.lo(), x.hi())
}

fn vmid(v: Vec3<Interval>) -> Vec3<f64> {
    v.map(mid)
}

fn pmid(p: Point3<Interval>) -> Point3<f64> {
    p.map(mid)
}

fn lift(v: Vec3<f64>) -> Vec3<Interval> {
    v.map(at)
}

/// **`C` over the parameter piece `t`**, enclosed: a spline edge's by
/// its restricted net ([`geom::NurbsCurve3::span_enclosure`]), any other
/// carrier's by its closed form at the interval. A piece the net door
/// does not read is the whole space.
fn arc_enclosure(c: &Curve3<Interval>, t: (f64, f64)) -> Point3<Interval> {
    match c {
        Curve3::Nurbs(n) => n.span_enclosure(t.0, t.1).unwrap_or_else(|| {
            let line = whole_line();
            Point3::new(line, line, line)
        }),
        _ => c.eval(iv(t)),
    }
}

/// **`C′` over the parameter piece `t`**, enclosed: a spline edge's off
/// its restricted hodograph ([`geom::NurbsCurve3::span_deriv`]), any
/// other carrier's by its closed form at the interval. A piece the
/// hodograph door does not read is the whole line.
fn arc_deriv(c: &Curve3<Interval>, t: (f64, f64)) -> Vec3<Interval> {
    match c {
        Curve3::Nurbs(n) => n.span_deriv(t.0, t.1).unwrap_or_else(|| whole_space()),
        _ => c.deriv(iv(t)),
    }
}

/// **`(S_u, S_v, S_uv)` over the chart rectangle `u × v`**, enclosed: a
/// spline chart's off its restricted net
/// ([`geom::NurbsSurface::rect_jet`]), an analytic carrier's by its
/// closed forms at the two intervals. A spline rectangle with no extent
/// on an axis, or one the net door does not read, is the whole line,
/// which decides nothing.
fn rect_jet(
    s: &Surface<Interval>,
    u: (f64, f64),
    v: (f64, f64),
) -> (Vec3<Interval>, Vec3<Interval>, Vec3<Interval>) {
    match s {
        Surface::Nurbs(net) => net.rect_jet(u, v).unwrap_or_else(|| {
            let whole = whole_space();
            (whole, whole, whole)
        }),
        _ => {
            let j = s.jet(iv(u), iv(v));
            (j.du, j.dv, j.duv)
        }
    }
}

fn unit(v: Vec3<f64>) -> Option<Vec3<f64>> {
    let n = v.norm();
    (n.is_finite() && n > 0.0).then(|| v * (1.0 / n))
}

fn perp(v: Vec3<f64>, t: Vec3<f64>) -> Vec3<f64> {
    v - t * v.dot(t)
}

fn sign_of(x: Interval, row: &'static str, band: Band) -> Option<Sign> {
    match decide(row, Margin::of(x), band) {
        Ok(s @ (Sign::Positive | Sign::Negative)) => Some(s),
        _ => None,
    }
}

fn is_zero(x: Interval, band: Band) -> bool {
    matches!(decide(ON_SHARED, Margin::of(x), band), Ok(Sign::Zero))
}

fn flip(s: Sign) -> Sign {
    match s {
        Sign::Positive => Sign::Negative,
        Sign::Negative => Sign::Positive,
        Sign::Zero => Sign::Zero,
    }
}

/// Whether two boxes overlap. Both are rigorous enclosures, so a pair
/// this calls disjoint is disjoint: it selects what is examined and
/// decides nothing.
fn overlap(a: Point3<Interval>, b: Point3<Interval>) -> bool {
    let o = |p: Interval, q: Interval| p.lo() <= q.hi() && q.lo() <= p.hi();
    o(a.x, b.x) && o(a.y, b.y) && o(a.z, b.z)
}

fn periodic(surface: &Surface<Interval>, axis: Axis) -> bool {
    match surface {
        Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } => axis == Axis::U,
        Surface::Torus { .. } => true,
        _ => false,
    }
}

const TURN: f64 = core::f64::consts::TAU;

/// `x` moved by whole turns to sit nearest `near`.
fn wrap(x: Interval, near: f64) -> Interval {
    let k = ((near - mid(x)) / TURN).round();
    if k == 0.0 { x } else { x + at(k * TURN) }
}

// ------------------------------------------------------- presentations

/// A side of a presentation rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    XLo,
    XHi,
    YLo,
    YHi,
}

impl Side {
    fn of(axis: Axis, lo: bool) -> Self {
        match (axis, lo) {
            (Axis::U, true) => Self::XLo,
            (Axis::U, false) => Self::XHi,
            (Axis::V, true) => Self::YLo,
            (Axis::V, false) => Self::YHi,
        }
    }

    fn axis(self) -> Axis {
        match self {
            Self::XLo | Self::XHi => Axis::U,
            Self::YLo | Self::YHi => Axis::V,
        }
    }

    /// `+1` where the rectangle lies above the side on its axis.
    fn into(self) -> f64 {
        match self {
            Self::XLo | Self::YLo => 1.0,
            Self::XHi | Self::YHi => -1.0,
        }
    }
}

/// Which of a presentation's sides the separator vanishes on.
#[derive(Clone, Copy, Debug)]
enum Zero {
    /// None: the separator's sign is read off its value.
    None,
    /// One side.
    Side(Side),
    /// Two adjacent sides.
    Corner(Side, Side),
    /// Only the corner point of two adjacent sides.
    Point(Side, Side),
}

/// How presentation coordinates map into the face's chart.
#[derive(Clone, Copy, Debug)]
enum Map {
    /// They are the chart's.
    Chart,
    /// `(x, y) ↦ o + x·a + y·b`.
    Affine {
        o: [Interval; 2],
        a: [Interval; 2],
        b: [Interval; 2],
    },
}

/// **A face's part of a cell, presented**: a rectangle `R` whose image
/// holds every point of the face in the cell, and the sides of `R` that
/// map into the shared cell.
struct Presented<'a> {
    surface: &'a Surface<Interval>,
    map: Map,
    x: (f64, f64),
    y: (f64, f64),
    zero: Zero,
    /// The rectangle's jet and image, read once.
    cache: Cache,
}

/// What a presentation reads of its whole rectangle, once.
#[derive(Default)]
struct Cache {
    jet: std::cell::OnceCell<(Vec3<Interval>, Vec3<Interval>, Vec3<Interval>)>,
    image: std::cell::OnceCell<Point3<Interval>>,
}

/// What presenting a face in a cell came to.
enum Presentation<'a> {
    /// The face has no point in the cell.
    Vacuous,
    Of(Box<Presented<'a>>),
}

/// Why a face is not presented in a cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Unpresented {
    /// A shared edge near the cell whose image no presentation reads:
    /// no smaller cell changes that.
    Structural,
    /// A null edge's vertex: the chart collapses there.
    Collapsed(VertexKey),
    /// The cell holds more boundary than a presentation reads; a smaller
    /// one may not.
    Local,
}

impl<'a> Presented<'a> {
    fn chart(&self, x: Interval, y: Interval) -> (Interval, Interval) {
        match self.map {
            Map::Chart => (x, y),
            Map::Affine { o, a, b } => (o[0] + x * a[0] + y * b[0], o[1] + x * a[1] + y * b[1]),
        }
    }

    /// The image of `x × y`, enclosed.
    fn image(&self, x: (f64, f64), y: (f64, f64)) -> Point3<Interval> {
        match self.map {
            Map::Chart => self.surface.rect_enclosure(x, y),
            Map::Affine { .. } => {
                let (u, v) = self.chart(iv(x), iv(y));
                self.surface.eval(u, v)
            }
        }
    }

    fn whole(&self) -> Point3<Interval> {
        *self.cache.image.get_or_init(|| self.image(self.x, self.y))
    }

    /// `(Φ_x, Φ_y, Φ_xy)` over the rectangle.
    fn jet(&self) -> (Vec3<Interval>, Vec3<Interval>, Vec3<Interval>) {
        *self.cache.jet.get_or_init(|| self.jet_on(self.x, self.y))
    }

    /// `(Φ_x, Φ_y, Φ_xy)` over `x × y`. On a spline chart a side, with
    /// no extent across, reads the whole rectangle's, which holds it.
    fn jet_on(
        &self,
        x: (f64, f64),
        y: (f64, f64),
    ) -> (Vec3<Interval>, Vec3<Interval>, Vec3<Interval>) {
        if let Map::Chart = self.map {
            let flat = !(x.0 < x.1 && y.0 < y.1);
            if flat && matches!(self.surface, Surface::Nurbs(_)) {
                return self.jet();
            }
            return rect_jet(self.surface, x, y);
        }
        let (u, v) = self.chart(iv(x), iv(y));
        let j = self.surface.jet(u, v);
        match self.map {
            Map::Chart => (j.du, j.dv, j.duv),
            Map::Affine { a, b, .. } => (
                j.du * a[0] + j.dv * a[1],
                j.du * b[0] + j.dv * b[1],
                j.duu * (a[0] * b[0]) + j.duv * (a[0] * b[1] + a[1] * b[0]) + j.dvv * (a[1] * b[1]),
            ),
        }
    }

    fn bound(&self, side: Side) -> f64 {
        match side {
            Side::XLo => self.x.0,
            Side::XHi => self.x.1,
            Side::YLo => self.y.0,
            Side::YHi => self.y.1,
        }
    }

    fn span(&self, axis: Axis) -> f64 {
        match axis {
            Axis::U => self.x.1 - self.x.0,
            Axis::V => self.y.1 - self.y.0,
        }
    }

    /// The image of the corner of two sides.
    fn corner(&self, s: Side, t: Side) -> Point3<Interval> {
        let (xs, ys) = if s.axis() == Axis::U { (s, t) } else { (t, s) };
        let (x, y) = (self.bound(xs), self.bound(ys));
        self.image((x, x), (y, y))
    }

    /// The outward unit normal at the middle of `side`, at `f64`, of a
    /// face of sense `sense` (the cusp test's reading).
    fn normal_at(&self, side: Side, sense: bool) -> Option<Vec3<f64>> {
        let b = self.bound(side);
        let (x, y) = match side.axis() {
            Axis::U => (at(b), at(geom::mid_param(self.y.0, self.y.1))),
            Axis::V => (at(geom::mid_param(self.x.0, self.x.1)), at(b)),
        };
        let (u, v) = self.chart(x, y);
        let j = self.surface.jet(u, v);
        let n = unit(vmid(j.du.cross(j.dv)))?;
        Some(if sense { n } else { -n })
    }
}

/// Which side of an iso edge's line its face lies on, along the held
/// axis: the face is to the left of its boundary about the outward
/// normal, and the chart is positively oriented about `+(S_u × S_v)`.
fn face_side(u: &Use, iso: Iso, sense: bool) -> Option<f64> {
    let [p, q] = u.image?;
    let moving = |p: Point2<Interval>| match iso.held {
        Axis::U => p.y,
        Axis::V => p.x,
    };
    let delta = moving(q) - moving(p);
    let forward = match (delta.lo() > 0.0, delta.hi() < 0.0) {
        (true, _) => 1.0,
        (_, true) => -1.0,
        _ => return None,
    };
    let traversal = if u.plus { forward } else { -forward };
    let side = match iso.held {
        Axis::V => traversal,
        Axis::U => -traversal,
    };
    Some(if sense { side } else { -side })
}

/// The moving coordinate's range along a use, outer hull of its ends.
fn along(u: &Use, iso: Iso) -> Option<Interval> {
    let [p, q] = u.image?;
    let m = |p: Point2<Interval>| match iso.held {
        Axis::U => p.y,
        Axis::V => p.x,
    };
    let (a, b) = (m(p), m(q));
    Some(Interval::from_bounds(
        a.lo().min(b.lo()),
        a.hi().max(b.hi()),
    ))
}

/// Whether `want` lies in the union of `ranges`, each moved by whole
/// turns where the axis is periodic. Its ends are taken a few ulps in:
/// a line placed by whole turns ([`wrap`]) is rounded outward, which is
/// no gap in the edge.
fn covered(want: (f64, f64), ranges: &[Interval], periodic: bool) -> bool {
    let slack = |x: f64| 4.0 * f64::EPSILON * x.abs().max(1.0);
    let want = (want.0 + slack(want.0), want.1 - slack(want.1));
    if want.0 > want.1 {
        return true;
    }
    if ranges.iter().any(|r| r.lo() <= want.0 && want.1 <= r.hi()) {
        return true;
    }
    let near = geom::mid_param(want.0, want.1);
    let mut rs: Vec<(f64, f64)> = ranges
        .iter()
        .map(|&r| if periodic { wrap(r, near) } else { r })
        .map(|r| (r.lo(), r.hi()))
        .collect();
    if periodic {
        // A range a turn either side may cover the rest.
        let more: Vec<(f64, f64)> = rs
            .iter()
            .flat_map(|&(a, b)| [(a - TURN, b - TURN), (a + TURN, b + TURN)])
            .collect();
        rs.extend(more);
    }
    rs.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut reach = want.0;
    for (a, b) in rs {
        if a > reach {
            break;
        }
        reach = reach.max(b);
        if reach >= want.1 {
            return true;
        }
    }
    reach >= want.1
}

/// One line a presentation clips its rectangle by.
struct Clip {
    side: Side,
    at: Interval,
    /// The edges whose images lie on it; empty for a collapsed side.
    edges: Vec<usize>,
    /// The vertex a collapsed side is.
    collapsed: Option<VertexKey>,
}

/// **Present `piece`'s face in its cell**, the shared cell being the
/// edges `near_edges` (indices into the face's boundary) and the vertices
/// `near_vertices`.
fn present<'a>(
    piece: &Piece<'a>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> Result<Presentation<'a>, Unpresented> {
    match piece.surface {
        Surface::Plane { .. } => present_plane(piece, near_edges, near_vertices, band),
        Surface::Nurbs(_) => present_chart(piece, near_edges, near_vertices, band),
        Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Sphere { .. }
        | Surface::Torus { .. } => present_chart(piece, near_edges, near_vertices, band),
        Surface::Approx(_) => Err(Unpresented::Structural),
    }
}

/// The failure for an edge no presentation reads: structural when the
/// edge is a shared one.
fn unread(edge: usize, near_edges: &[usize]) -> Unpresented {
    if near_edges.contains(&edge) {
        Unpresented::Structural
    } else {
        Unpresented::Local
    }
}

/// **Which sheet of each periodic axis the face lies on in a cell.**
enum Sheets {
    /// The cell meets no copy of the face's extent: it holds none of
    /// the face.
    Missed,
    /// The cell meets two or more copies: the face enters it more than
    /// once, which one rectangle does not present.
    Several,
    /// Per axis, `[u, v]`.
    On([Sheet; 2]),
}

/// The face's sheet on one axis of a cell.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Sheet {
    /// No sheet to choose: the axis is not periodic, or the face spans a
    /// whole turn of it.
    Free,
    /// The one copy of the face's extent the cell meets.
    Copy(f64, f64),
    /// The face's extent is unknown: nothing on this axis is placed.
    Unknown,
    /// An axis that is not periodic, where the face's extent is known:
    /// lines are where they are, and the face lies within it.
    Within(f64, f64),
}

/// **The face's sheet in `cell`, per periodic axis**: the copy of its
/// own extent ([`Boundary::extent`]) moved by whole turns that the cell
/// meets. Read once from the face, so every line of it is placed on the
/// same sheet ([`placed`]) whatever order its edges are read in. A face
/// whose extent is unknown leaves the sheet unknown: a line on that
/// axis then has no placement and the face is not presented there. On
/// an axis that is not periodic the extent still says what the cell
/// holds.
///
/// A cell MEETS a copy where their overlap has width. One that only
/// touches it, along the copy's end line, holds the face's points
/// there on its own boundary — and the closed cell beside it holds
/// them too, with width, and is paired as this one is. So a cell
/// meeting no copy with width holds none of the face this pair must
/// read.
fn sheets(
    surface: &Surface<Interval>,
    extent: Option<[(f64, f64); 2]>,
    cell: ((f64, f64), (f64, f64)),
) -> Sheets {
    let mut on = [Sheet::Free; 2];
    let Some(extent) = extent else {
        for axis in [Axis::U, Axis::V] {
            if periodic(surface, axis) {
                on[axis.index()] = Sheet::Unknown;
            }
        }
        return Sheets::On(on);
    };
    for axis in [Axis::U, Axis::V] {
        let (lo, hi) = extent[axis.index()];
        let (clo, chi) = axis.of(cell);
        // A few ulps of the turn: the shifted copies are rounded.
        let slack = 8.0 * f64::EPSILON * TURN.max(lo.abs()).max(hi.abs());
        let meets = |(a, b): (f64, f64)| b.min(chi) - a.max(clo) > slack;
        if !periodic(surface, axis) {
            if !meets((lo, hi)) {
                return Sheets::Missed;
            }
            on[axis.index()] = Sheet::Within(lo, hi);
            continue;
        }
        if hi - lo >= TURN - slack {
            continue;
        }
        #[allow(clippy::cast_precision_loss)]
        let copies: Vec<(f64, f64)> = (-2i32..=2)
            .map(|k| (lo + f64::from(k) * TURN, hi + f64::from(k) * TURN))
            .filter(|&c| meets(c))
            .collect();
        match copies.as_slice() {
            [] => return Sheets::Missed,
            [one] => on[axis.index()] = Sheet::Copy(one.0, one.1),
            _ => return Sheets::Several,
        }
    }
    Sheets::On(on)
}

/// **Where an edge's line lies in a cell's chart**: on a periodic axis,
/// on the face's sheet ([`sheets`]), or, for a face spanning a whole
/// turn, the copy nearest the cell — the only one in it, as no cell is
/// wider than a turn. `None` on a periodic axis whose sheet is unknown.
fn placed(
    surface: &Surface<Interval>,
    iso: Iso,
    sheet: [Sheet; 2],
    (lo, hi): (f64, f64),
) -> Option<Interval> {
    if !periodic(surface, iso.held) {
        return Some(iso.at);
    }
    match sheet[iso.held.index()] {
        Sheet::Copy(a, b) => Some(wrap(iso.at, geom::mid_param(a, b))),
        Sheet::Free | Sheet::Within(..) => Some(wrap(iso.at, geom::mid_param(lo, hi))),
        Sheet::Unknown => None,
    }
}

/// **Whether a cell every near line puts on its non-face side holds
/// none of the face.** No near line crosses the cell and every other
/// edge is clear of its ball, so the cell holds no boundary: it is
/// wholly in the face or wholly out. It is out where one of the lines
/// TOUCHES it — the cell's points beside the line are then on its
/// non-face side — along an edge that covers the cell's whole side, so
/// the edge, not its end, is what the cell is beside. A line apart from
/// the cell says nothing: the face may come back between them.
fn vacuous(
    b: &Boundary,
    surface: &Surface<Interval>,
    cell: ((f64, f64), (f64, f64)),
    outside: &[(Side, Interval, usize)],
) -> bool {
    outside.iter().any(|&(side, line, ei)| {
        let (lo, hi) = side.axis().of(cell);
        let slack = |x: f64| 4.0 * f64::EPSILON * x.abs().max(1.0);
        let touches = match side {
            Side::XLo | Side::YLo => line.lo() <= hi + slack(hi),
            Side::XHi | Side::YHi => line.hi() >= lo - slack(lo),
        };
        let axis = side.axis().other();
        touches
            && edge_ranges(b, &[ei], side.axis())
                .is_some_and(|r| covered(axis.of(cell), &r, periodic(surface, axis)))
    })
}

/// The ranges the uses of `edges` holding `held` run over on the other
/// axis: a whole turn's image covers everything.
fn edge_ranges(b: &Boundary, edges: &[usize], held: Axis) -> Option<Vec<Interval>> {
    let mut out = Vec::new();
    for &ei in edges {
        let e = b.edges.get(ei)?;
        for u in &e.uses {
            let iso = u.iso?;
            if iso.held != held {
                continue;
            }
            if iso.full {
                out.push(whole_line());
            } else {
                out.push(along(u, iso)?);
            }
        }
    }
    Some(out)
}

/// **A cone face's nappe**: `true` where its vertices off the apex stand
/// on the axis's positive side. Every vertex whose signed height is
/// decided must agree — a face across the apex is on both nappes and has
/// no one implicit — and one must be decided.
fn nappe(b: &Boundary, apex: Point3<Interval>, axis: Vec3<Interval>, band: Band) -> Option<bool> {
    let mut signs = b
        .vertices
        .values()
        .filter_map(|&q| sign_of((q - apex).dot(axis), NAPPE, band));
    let first = signs.next()?;
    signs.all(|s| s == first).then_some(first == Sign::Positive)
}

/// The whole line, as an interval: what a reading no door bounds is.
pub(crate) fn whole_line() -> Interval {
    Interval::from_bounds(f64::NEG_INFINITY, f64::INFINITY)
}

/// Every vector: [`whole_line`] on each axis.
fn whole_space() -> Vec3<Interval> {
    let line = whole_line();
    Vec3::new(line, line, line)
}

/// The chart presentation: the cell's own rectangle, clipped by the iso
/// lines of the boundary edges near it. An iso-bounded NURBS face's
/// cells lie in its domain, whose boundary lines are its edges.
///
/// **What a presentation must be, and why it is read in this order.**
/// The rectangle must hold every point of the face in the cell: a
/// SUPERSET is sound (every claim departure makes over it covers more
/// points than the face has), a subset is not. So:
///
/// - Every line is read against the CELL, never against a rectangle an
///   earlier line clipped, and on a periodic axis every line is placed
///   on the one sheet of the face's own extent the cell meets
///   ([`sheets`]). No line's reading depends on another's, so the
///   result does not depend on the order of [`Boundary::edges`].
/// - A line the cell lies on the face side of bounds it there; two
///   bounds on one side must be one line, and a low bound above a high
///   one is a face that re-enters the cell (a slot), which a rectangle
///   does not present.
/// - A cell holds none of the face ([`Presentation::Vacuous`]) only
///   when it misses the face's extent, or every near line puts it on its
///   non-face side and one of them touches it along an edge that covers
///   its side ([`vacuous`]).
fn present_chart<'a>(
    piece: &Piece<'a>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> Result<Presentation<'a>, Unpresented> {
    let b = piece.boundary;
    let cell = (piece.u, piece.v);
    let sheet = if piece.whole {
        [Sheet::Free; 2]
    } else {
        match sheets(piece.surface, b.extent, cell) {
            Sheets::Missed => return Ok(Presentation::Vacuous),
            Sheets::Several => return Err(Unpresented::Local),
            Sheets::On(s) => s,
        }
    };
    // The face lies within its extent's copy: the rectangle starts as the
    // cell clipped to it, a superset of the face in the cell still.
    let within = |axis: Axis| {
        let (lo, hi) = axis.of(cell);
        match sheet[axis.index()] {
            Sheet::Copy(a, b) | Sheet::Within(a, b) => (lo.max(a), hi.min(b)),
            Sheet::Free | Sheet::Unknown => (lo, hi),
        }
    };
    let (mut x, mut y) = (within(Axis::U), within(Axis::V));
    let mut clips: Vec<Clip> = Vec::new();
    let mut outside: Vec<(Side, Interval, usize)> = Vec::new();
    // Whether some near line has the cell on its face side: then no
    // line it lies outside of can say the cell holds none of the face.
    let mut inside = false;
    for &ei in &piece.local.edges {
        let Some(e) = b.edges.get(ei) else {
            return Err(Unpresented::Local);
        };
        if e.arc.is_none() {
            if piece.whole {
                return Err(Unpresented::Collapsed(e.ends[0]));
            }
            continue;
        }
        if !piece.whole && e.uses.len() > 1 {
            // An analytic seam is no boundary: the face lies on both
            // sides of it.
            continue;
        }
        for u in &e.uses {
            let Some(iso) = u.iso else {
                return Err(unread(ei, near_edges));
            };
            let (lo, hi) = iso.held.of(cell);
            if piece.whole {
                // The line is a bound of the exact knot domain.
                let exact = |b: f64| iso.at.lo() == b && iso.at.hi() == b;
                for (bound, low) in [(lo, true), (hi, false)] {
                    if exact(bound) {
                        clips.push(Clip {
                            side: Side::of(iso.held, low),
                            at: iso.at,
                            edges: vec![ei],
                            collapsed: None,
                        });
                    }
                }
                continue;
            }
            let Some(sigma) = face_side(u, iso, b.sense) else {
                return Err(unread(ei, near_edges));
            };
            // A line on a periodic axis whose sheet is unknown has no
            // place, at any depth.
            let Some(line) = placed(piece.surface, iso, sheet, (lo, hi)) else {
                return Err(Unpresented::Structural);
            };
            let side = Side::of(iso.held, sigma > 0.0);
            if (sigma > 0.0 && line.lo() >= hi) || (sigma < 0.0 && line.hi() <= lo) {
                outside.push((side, line, ei));
                continue;
            }
            inside = true;
            // A line the cell stops short of bounds nothing in it, unless
            // the gap is no wider than the cell: then the rectangle is
            // widened to it, a superset still, so a side lies on it.
            let reach = hi - lo;
            if (sigma > 0.0 && line.hi() < lo - reach) || (sigma < 0.0 && line.lo() > hi + reach) {
                continue;
            }
            clips.push(Clip {
                side,
                at: line,
                edges: vec![ei],
                collapsed: None,
            });
        }
    }
    if !inside && !outside.is_empty() {
        return if vacuous(b, piece.surface, cell, &outside) {
            Ok(Presentation::Vacuous)
        } else {
            Err(Unpresented::Local)
        };
    }
    // Lines on one side merge when they hold one common value; two
    // different lines on one side do not bound a face the clip reads.
    let mut merged: Vec<Clip> = Vec::new();
    for side in [Side::XLo, Side::XHi, Side::YLo, Side::YHi] {
        let on: Vec<&Clip> = clips.iter().filter(|c| c.side == side).collect();
        let Some(first) = on.first() else {
            continue;
        };
        let common_lo = on
            .iter()
            .map(|c| c.at.lo())
            .fold(f64::NEG_INFINITY, f64::max);
        let common_hi = on.iter().map(|c| c.at.hi()).fold(f64::INFINITY, f64::min);
        if common_lo > common_hi {
            return Err(Unpresented::Local);
        }
        let hull = Interval::from_bounds(
            on.iter().map(|c| c.at.lo()).fold(first.at.lo(), f64::min),
            on.iter().map(|c| c.at.hi()).fold(first.at.hi(), f64::max),
        );
        merged.push(Clip {
            side,
            at: hull,
            edges: on.iter().flat_map(|c| c.edges.iter().copied()).collect(),
            collapsed: None,
        });
    }
    // The rectangle: the cell, its sides moved to the lines, outward.
    for c in &merged {
        if piece.whole {
            continue;
        }
        match c.side {
            Side::XLo => x.0 = c.at.lo(),
            Side::XHi => x.1 = c.at.hi(),
            Side::YLo => y.0 = c.at.lo(),
            Side::YHi => y.1 = c.at.hi(),
        }
    }
    if x.0 > x.1 || y.0 > y.1 {
        // A low line above a high one: the face leaves the cell and
        // comes back into it, which no one rectangle presents.
        return Err(Unpresented::Local);
    }
    // A cone's apex is a collapsed side of its chart.
    if let Surface::Cone { apex, axis, .. } = piece.surface {
        for &v in &piece.local.vertices {
            let Some(&p) = b.vertices.get(&v) else {
                continue;
            };
            if !matches!(
                decide(APEX, Margin::of((p - *apex).norm()), band),
                Ok(Sign::Zero)
            ) {
                continue;
            }
            let Some(lo) = nappe(b, *apex, *axis, band) else {
                return Err(Unpresented::Local);
            };
            if (lo && y.1 <= 0.0) || (!lo && y.0 >= 0.0) {
                return Ok(Presentation::Vacuous);
            }
            if (lo && y.0 > 0.0) || (!lo && y.1 < 0.0) {
                continue;
            }
            y = if lo {
                (y.0.max(0.0), y.1)
            } else {
                (y.0, y.1.min(0.0))
            };
            let side = Side::of(Axis::V, lo);
            match merged.iter_mut().find(|m| m.side == side) {
                Some(m) if m.at.lo() <= 0.0 && 0.0 <= m.at.hi() => m.collapsed = Some(v),
                Some(_) => return Err(Unpresented::Local),
                None => merged.push(Clip {
                    side,
                    at: at(0.0),
                    edges: Vec::new(),
                    collapsed: Some(v),
                }),
            }
        }
    }
    // A rectangle clipped to within the band of a shared line is read as
    // holding no point of the face off that line: its extent across, in
    // metres, is decided zero (`self_intersection_sliver`).
    //
    // **That is an ε concession, and it is the only one this arm
    // makes.** Any coincidence it misses lies within the strip, so
    // within ε of the shared edge's image in ℝ³. It is asymmetric with
    // the distance test: two cells that do NOT touch and come as close
    // read `Violated`, or refuse, while here, beside a shared edge, the
    // same approach reads certified.
    for c in &merged {
        let on_shared = c.edges.iter().all(|e| near_edges.contains(e)) && !c.edges.is_empty();
        if !on_shared {
            continue;
        }
        let (across, lever) = match c.side.axis() {
            Axis::U => (x.1 - x.0, rect_jet(piece.surface, x, y).0),
            Axis::V => (y.1 - y.0, rect_jet(piece.surface, x, y).1),
        };
        let width = at(across) * at(lever.norm().hi());
        if matches!(decide(SLIVER, Margin::of(width), band), Ok(Sign::Zero)) {
            return Ok(Presentation::Vacuous);
        }
    }
    // Every clip's side must be the edges' own: an edge that ends inside
    // the cell leaves the face free to wrap round its end.
    for c in &merged {
        if c.collapsed.is_some() || piece.whole {
            continue;
        }
        let axis = c.side.axis().other();
        let want = axis.of((x, y));
        let Some(ranges) = edge_ranges(b, &c.edges, c.side.axis()) else {
            return Err(Unpresented::Local);
        };
        if !covered(want, &ranges, periodic(piece.surface, axis)) {
            return Err(Unpresented::Local);
        }
    }
    // Two lines on different axes meet at a corner of the face; the
    // clipped quadrant is the face there only where the corner is
    // convex. A NURBS face's corners are its domain's.
    if !piece.whole {
        for (i, c) in merged.iter().enumerate() {
            for d in merged.iter().skip(i + 1) {
                if c.side.axis() == d.side.axis() || c.collapsed.is_some() || d.collapsed.is_some()
                {
                    continue;
                }
                if !convex(b, c, d) {
                    return Err(Unpresented::Local);
                }
            }
        }
    }
    let zero = zero_of(b, &merged, near_edges, near_vertices)?;
    Ok(Presentation::Of(Box::new(Presented {
        surface: piece.surface,
        map: Map::Chart,
        x,
        y,
        zero,
        cache: Cache::default(),
    })))
}

/// Whether the face is the quadrant two clips cut at the vertex their
/// edges share: each edge leaves the vertex into the other's face side.
fn convex(b: &Boundary, c: &Clip, d: &Clip) -> bool {
    let away = |ei: usize, v: VertexKey| -> Option<f64> {
        let e = b.edges.get(ei)?;
        let u = e.uses.first()?;
        let iso = u.iso?;
        let [p, q] = u.image?;
        let m = |p: Point2<Interval>| match iso.held {
            Axis::U => p.y,
            Axis::V => p.x,
        };
        let delta = m(q) - m(p);
        let forward = if delta.lo() > 0.0 {
            1.0
        } else if delta.hi() < 0.0 {
            -1.0
        } else {
            return None;
        };
        // Away from `v` along the curve: forward from its start.
        if e.ends[0] == v {
            Some(forward)
        } else if e.ends[1] == v {
            Some(-forward)
        } else {
            None
        }
    };
    let common = |c: &Clip, d: &Clip| -> Option<VertexKey> {
        c.edges.iter().find_map(|&i| {
            let e = b.edges.get(i)?;
            d.edges.iter().find_map(|&j| {
                let f = b.edges.get(j)?;
                e.ends.iter().find(|v| f.ends.contains(v)).copied()
            })
        })
    };
    let Some(v) = common(c, d) else {
        // No corner: the two lines cross where neither edge reaches,
        // which the span check has already refused.
        return true;
    };
    let ok = |c: &Clip, d: &Clip| -> bool {
        // An edge of `d` leaves `v` along `c`'s held axis, toward `c`'s
        // face side.
        d.edges
            .iter()
            .filter_map(|&j| away(j, v))
            .all(|s| s * c.side.into() > 0.0)
    };
    ok(c, d) && ok(d, c)
}

/// The sides of a presentation the separator vanishes on: the clips
/// whose every edge is shared, a collapsed side at a shared vertex, or
/// the corner of two clips at a shared vertex.
fn zero_of(
    b: &Boundary,
    clips: &[Clip],
    near_edges: &[usize],
    near_vertices: &[VertexKey],
) -> Result<Zero, Unpresented> {
    // The shared cell holds its edges' ends.
    let shared_vertex = |v: &VertexKey| {
        near_vertices.contains(v)
            || near_edges
                .iter()
                .any(|&i| b.edges.get(i).is_some_and(|e| e.ends.contains(v)))
    };
    let on_shared = |c: &Clip| {
        if let Some(v) = c.collapsed {
            shared_vertex(&v)
        } else {
            !c.edges.is_empty() && c.edges.iter().all(|e| near_edges.contains(e))
        }
    };
    let zeros: Vec<&Clip> = clips.iter().filter(|c| on_shared(c)).collect();
    let ends_of = |c: &Clip| -> Vec<VertexKey> {
        let mut v: Vec<VertexKey> = c
            .edges
            .iter()
            .filter_map(|&i| b.edges.get(i))
            .flat_map(|e| e.ends)
            .collect();
        v.extend(c.collapsed);
        v
    };
    // A shared vertex the zero sides do not hold must be a corner of two
    // clips, read as a point.
    let loose: Vec<VertexKey> = near_vertices
        .iter()
        .filter(|v| !zeros.iter().any(|c| ends_of(c).contains(v)))
        .copied()
        .collect();
    match (zeros.as_slice(), loose.as_slice()) {
        ([], []) => Ok(Zero::None),
        ([c], []) => Ok(Zero::Side(c.side)),
        ([c, d], []) if c.side.axis() != d.side.axis() => Ok(Zero::Corner(c.side, d.side)),
        ([], [v]) => {
            for (i, c) in clips.iter().enumerate() {
                for d in clips.iter().skip(i + 1) {
                    if c.side.axis() != d.side.axis()
                        && ends_of(c).contains(v)
                        && ends_of(d).contains(v)
                    {
                        return Ok(Zero::Point(c.side, d.side));
                    }
                }
            }
            Err(Unpresented::Local)
        }
        _ => Err(Unpresented::Local),
    }
}

/// The planar presentation: the plane's chart through an affine map in
/// which one line edge is `y = 0` and the face `y ≥ 0`, or two line edges
/// at a convex corner are the positive axes.
fn present_plane<'a>(
    piece: &Piece<'a>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> Result<Presentation<'a>, Unpresented> {
    let b = piece.boundary;
    let s = if b.sense { 1.0 } else { -1.0 };
    let edges: Vec<usize> = piece
        .local
        .edges
        .iter()
        .copied()
        .filter(|&i| b.edges.get(i).is_some_and(|e| e.arc.is_some()))
        .collect();
    let cell_u = iv(piece.u);
    let cell_v = iv(piece.v);
    if edges.is_empty() {
        let zero = if near_vertices.is_empty() {
            Zero::None
        } else {
            return Err(Unpresented::Local);
        };
        return Ok(Presentation::Of(Box::new(Presented {
            surface: piece.surface,
            map: Map::Chart,
            x: piece.u,
            y: piece.v,
            zero,
            cache: Cache::default(),
        })));
    }
    // A line edge's chart image, in traversal order: its two ends read
    // into the plane's affine chart.
    let Surface::Plane {
        origin,
        normal,
        u_ref,
    } = *piece.surface
    else {
        return Err(Unpresented::Structural);
    };
    let v_ref = normal.cross(u_ref);
    let chart = |p: Point3<Interval>| {
        let d = p - origin;
        Point2::new(d.dot(u_ref), d.dot(v_ref))
    };
    let segment = |ei: usize| -> Result<(Point2<Interval>, Point2<Interval>), Unpresented> {
        let e = b.edges.get(ei).ok_or(Unpresented::Local)?;
        let Some((carrier @ Curve3::Line { .. }, (t0, t1))) = &e.arc else {
            return Err(unread(ei, near_edges));
        };
        let [u] = e.uses.as_slice() else {
            return Err(unread(ei, near_edges));
        };
        let (p, q) = (chart(carrier.eval(*t0)), chart(carrier.eval(*t1)));
        Ok(if u.plus { (p, q) } else { (q, p) })
    };
    let rot = |a: [Interval; 2]| [-a[1] * at(s), a[0] * at(s)];
    match edges.as_slice() {
        [e] => {
            if !piece.local.vertices.is_empty() {
                return Err(Unpresented::Local);
            }
            let (p, q) = segment(*e)?;
            let a = [q.x - p.x, q.y - p.y];
            let bv = rot(a);
            let len2 = a[0].powi(2) + a[1].powi(2);
            let d = [cell_u - p.x, cell_v - p.y];
            let x = (d[0] * a[0] + d[1] * a[1]) / len2;
            let y = (d[0] * bv[0] + d[1] * bv[1]) / len2;
            // Wholly on the edge's non-face side holds none of the face:
            // every vertex and every other edge is clear of the cell's
            // ball, so the plane's disk inside the ball meets the
            // boundary only in a whole chord of this edge, and the
            // disk's half off the face side is all outside the face. The
            // cell lies in that half.
            if y.hi() <= 0.0 {
                return Ok(Presentation::Vacuous);
            }
            let len = len2.sqrt();
            let inside = (x * len).min((at(1.0) - x) * len);
            if sign_of(inside, SPAN, band) != Some(Sign::Positive) {
                return Err(Unpresented::Local);
            }
            let zero = if near_edges.contains(e) {
                Zero::Side(Side::YLo)
            } else {
                Zero::None
            };
            Ok(Presentation::Of(Box::new(Presented {
                surface: piece.surface,
                map: Map::Affine {
                    o: [p.x, p.y],
                    a,
                    b: bv,
                },
                x: (x.lo(), x.hi()),
                y: (0.0, y.hi()),
                zero,
                cache: Cache::default(),
            })))
        }
        [e1, e2] => {
            let (Some(r1), Some(r2)) = (b.edges.get(*e1), b.edges.get(*e2)) else {
                return Err(Unpresented::Local);
            };
            let Some(&vtx) = r1.ends.iter().find(|v| r2.ends.contains(v)) else {
                return Err(Unpresented::Local);
            };
            if piece.local.vertices.iter().any(|&w| w != vtx) {
                return Err(Unpresented::Local);
            }
            let (p1, q1) = segment(*e1)?;
            let (p2, q2) = segment(*e2)?;
            // Each edge's chart point at the vertex, its far point, and
            // its traversal direction.
            let start1 = if r1.uses[0].plus {
                r1.ends[0]
            } else {
                r1.ends[1]
            };
            let start2 = if r2.uses[0].plus {
                r2.ends[0]
            } else {
                r2.ends[1]
            };
            let (o, far1) = if start1 == vtx { (p1, q1) } else { (q1, p1) };
            let far2 = if start2 == vtx { q2 } else { p2 };
            let t1 = [q1.x - p1.x, q1.y - p1.y];
            let t2 = [q2.x - p2.x, q2.y - p2.y];
            let a = [far1.x - o.x, far1.y - o.y];
            let bv = [far2.x - o.x, far2.y - o.y];
            let n1 = rot(t1);
            let n2 = rot(t2);
            let lean = |n: [Interval; 2], w: [Interval; 2]| {
                (n[0] * w[0] + n[1] * w[1]) / (n[0].powi(2) + n[1].powi(2)).sqrt()
            };
            if sign_of(lean(n1, bv), CORNER, band) != Some(Sign::Positive)
                || sign_of(lean(n2, a), CORNER, band) != Some(Sign::Positive)
            {
                return Err(Unpresented::Local);
            }
            let det = a[0] * bv[1] - a[1] * bv[0];
            let d = [cell_u - o.x, cell_v - o.y];
            let x = (d[0] * bv[1] - d[1] * bv[0]) / det;
            let y = (a[0] * d[1] - a[1] * d[0]) / det;
            if x.hi() <= 0.0 || y.hi() <= 0.0 {
                return Ok(Presentation::Vacuous);
            }
            let la = (a[0].powi(2) + a[1].powi(2)).sqrt();
            let lb = (bv[0].powi(2) + bv[1].powi(2)).sqrt();
            if sign_of((at(1.0) - x) * la, SPAN, band) != Some(Sign::Positive)
                || sign_of((at(1.0) - y) * lb, SPAN, band) != Some(Sign::Positive)
            {
                return Err(Unpresented::Local);
            }
            let z1 = near_edges.contains(e1);
            let z2 = near_edges.contains(e2);
            let zero = match (z1, z2) {
                (true, true) => Zero::Corner(Side::YLo, Side::XLo),
                (true, false) => Zero::Side(Side::YLo),
                (false, true) => Zero::Side(Side::XLo),
                (false, false) if near_vertices.contains(&vtx) => Zero::Point(Side::XLo, Side::YLo),
                (false, false) => Zero::None,
            };
            Ok(Presentation::Of(Box::new(Presented {
                surface: piece.surface,
                map: Map::Affine {
                    o: [o.x, o.y],
                    a,
                    b: bv,
                },
                x: (0.0, x.hi()),
                y: (0.0, y.hi()),
                zero,
                cache: Cache::default(),
            })))
        }
        _ => Err(Unpresented::Local),
    }
}

// --------------------------------------------------------- separators

/// A face's analytic carrier as a function of space, in metres, zero on
/// the carrier (the face's nappe, on a cone).
#[derive(Clone, Copy, Debug)]
enum Implicit {
    Plane {
        o: Point3<Interval>,
        n: Vec3<Interval>,
    },
    Cylinder {
        o: Point3<Interval>,
        axis: Vec3<Interval>,
        r: Interval,
    },
    Sphere {
        c: Point3<Interval>,
        r: Interval,
    },
    Cone {
        apex: Point3<Interval>,
        axis: Vec3<Interval>,
        cos: Interval,
        sin: Interval,
        nappe: Interval,
    },
}

/// **The separator** `ψ`, metres.
#[derive(Clone, Copy, Debug)]
enum Psi {
    /// `n·(p − q)`.
    Linear {
        n: Vec3<Interval>,
        q: Point3<Interval>,
    },
    /// A face's carrier.
    Carrier(Implicit),
    /// `h(p) − γ(s(p))`: `h = m·p`, `s = t·p`, `γ` the shared edge's
    /// graph, extended past its piece with slopes in `slope`. Its
    /// gradient is `m − γ′(s)·t`, `γ′ ∈ slope`, everywhere.
    Graph {
        m: Vec3<Interval>,
        t: Vec3<Interval>,
        slope: Interval,
    },
}

impl Psi {
    /// `ψ` over a box, where it has a closed form.
    fn value(&self, p: Point3<Interval>) -> Option<Interval> {
        Some(match *self {
            Self::Linear { n, q } => n.dot(p - q),
            Self::Carrier(Implicit::Plane { o, n }) => n.dot(p - o),
            Self::Carrier(Implicit::Cylinder { o, axis, r }) => {
                let w = p - o;
                (w - axis * w.dot(axis)).norm() - r
            }
            Self::Carrier(Implicit::Sphere { c, r }) => (p - c).norm() - r,
            Self::Carrier(Implicit::Cone {
                apex,
                axis,
                cos,
                sin,
                nappe,
            }) => {
                let w = p - apex;
                let h = w.dot(axis);
                (w - axis * h).norm() * cos - nappe * h * sin
            }
            Self::Graph { .. } => return None,
        })
    }

    /// `∇ψ` over a box.
    fn gradient(&self, p: Point3<Interval>) -> Vec3<Interval> {
        match *self {
            Self::Linear { n, .. } | Self::Carrier(Implicit::Plane { n, .. }) => n,
            Self::Carrier(Implicit::Cylinder { o, axis, .. }) => {
                let w = p - o;
                let radial = w - axis * w.dot(axis);
                radial * (at(1.0) / radial.norm())
            }
            Self::Carrier(Implicit::Sphere { c, .. }) => {
                let w = p - c;
                w * (at(1.0) / w.norm())
            }
            Self::Carrier(Implicit::Cone {
                apex,
                axis,
                cos,
                sin,
                nappe,
            }) => {
                let w = p - apex;
                let radial = w - axis * w.dot(axis);
                radial * (cos / radial.norm()) - axis * (nappe * sin)
            }
            Self::Graph { m, t, slope } => m - t * slope,
        }
    }

    /// The constant gradient of a linear separator.
    fn linear(&self) -> Option<Vec3<Interval>> {
        match *self {
            Self::Linear { n, .. } | Self::Carrier(Implicit::Plane { n, .. }) => Some(n),
            _ => None,
        }
    }
}

/// `ψ∘Φ` over `x × y` in its centred form: its value at the middle, and
/// the gradient's enclosure over the image times the half-spans.
/// Tight where the plain enclosure of a slanted image's box is not.
fn centred(psi: &Psi, p: &Presented<'_>, x: (f64, f64), y: (f64, f64)) -> Option<Interval> {
    let (xm, ym) = (geom::mid_param(x.0, x.1), geom::mid_param(y.0, y.1));
    let middle = psi.value(p.image((xm, xm), (ym, ym)))?;
    let whole = x == p.x && y == p.y;
    let (fx, fy, _) = if whole { p.jet() } else { p.jet_on(x, y) };
    let grad = psi.gradient(if whole { p.whole() } else { p.image(x, y) });
    Some(middle + grad.dot(fx) * iv((x.0 - xm, x.1 - xm)) + grad.dot(fy) * iv((y.0 - ym, y.1 - ym)))
}

/// **The sign `ψ∘Φ` takes on the presentation's rectangle less its zero
/// sides**, decided, or `None`.
///
/// - No zero side: `ψ` over the image.
/// - One side: `ψ∘Φ` is zero along it, so its sign inside is the sign of
///   the derivative across it, over the whole rectangle.
/// - Two adjacent sides, `ψ` linear: zero along both, so its sign is the
///   mixed derivative's, oriented by the corner.
/// - A corner point: zero there, so its sign is the common sign of the
///   two derivatives along the rectangle's axes away from it.
///
/// A side is zero by decision (`ON_SHARED`), except under the graph
/// separator, which vanishes on the shared edge by construction.
fn sign_on(psi: &Psi, p: &Presented<'_>, band: Band) -> Option<Sign> {
    let checked = !matches!(psi, Psi::Graph { .. });
    let side_of = |s: Side| {
        let b = p.bound(s);
        match s.axis() {
            Axis::U => ((b, b), p.y),
            Axis::V => (p.x, (b, b)),
        }
    };
    let zero_on = |s: Side| {
        let (x, y) = side_of(s);
        !checked || centred(psi, p, x, y).is_some_and(|v| is_zero(v, band))
    };
    let (fx, fy, fxy) = p.jet();
    let image = p.whole();
    let grad = psi.gradient(image);
    let across = |axis: Axis| match axis {
        Axis::U => grad.dot(fx) * at(p.span(Axis::U)),
        Axis::V => grad.dot(fy) * at(p.span(Axis::V)),
    };
    match p.zero {
        Zero::None => sign_of(centred(psi, p, p.x, p.y)?, DEPARTURE, band),
        Zero::Side(s) => {
            if !zero_on(s) {
                return None;
            }
            sign_of(across(s.axis()) * at(s.into()), DEPARTURE, band)
        }
        Zero::Corner(s, t) => {
            let n = psi.linear()?;
            if !(zero_on(s) && zero_on(t)) {
                return None;
            }
            let g = n.dot(fxy) * at(p.span(Axis::U) * p.span(Axis::V) * s.into() * t.into());
            sign_of(g, DEPARTURE, band)
        }
        Zero::Point(s, t) => {
            if checked && !psi.value(p.corner(s, t)).is_some_and(|v| is_zero(v, band)) {
                return None;
            }
            let a = sign_of(across(s.axis()) * at(s.into()), DEPARTURE, band)?;
            let b = sign_of(across(t.axis()) * at(t.into()), DEPARTURE, band)?;
            (a == b).then_some(a)
        }
    }
}

/// The carrier of a face as a separator.
fn implicit_of(piece: &Piece<'_>, band: Band) -> Option<Implicit> {
    Some(match *piece.surface {
        Surface::Plane { origin, normal, .. } => Implicit::Plane {
            o: origin,
            n: normal,
        },
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => Implicit::Cylinder {
            o: origin,
            axis,
            r: radius,
        },
        Surface::Sphere { center, radius, .. } => Implicit::Sphere {
            c: center,
            r: radius,
        },
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (sin, cos) = half_angle.sin_cos();
            let positive = nappe(piece.boundary, apex, axis, band)?;
            Implicit::Cone {
                apex,
                axis,
                cos,
                sin,
                nappe: at(if positive { 1.0 } else { -1.0 }),
            }
        }
        _ => return None,
    })
}

/// **The departure arm.** Each separator in turn: one face's carrier
/// with the other presented, a plane through straight shared edges, the
/// shared edge's graph.
fn departure(x: &Piece<'_>, y: &Piece<'_>, near: &Near, band: Band, two: bool) -> Outcome {
    let shared = near.cell(x.boundary);
    let split = Outcome::Split {
        arm: CertificateArm::Departure,
        shared,
    };
    let (ex, ey) = (near.side(false), near.side(true));
    let px = present(x, &ex, &near.vertices, band);
    let py = present(y, &ey, &near.vertices, band);
    for p in [&px, &py] {
        if let Err(Unpresented::Collapsed(v)) = p {
            return Outcome::Refuse(EngineRefusal::CollapsedChart {
                face: x.face,
                vertex: *v,
            });
        }
    }
    if matches!(px, Ok(Presentation::Vacuous)) || matches!(py, Ok(Presentation::Vacuous)) {
        return Outcome::Certified;
    }
    // One face's carrier, the other departing it. Not between two faces
    // of one face: the face is its own carrier.
    if two {
        for (sep, other) in [(x, &py), (y, &px)] {
            if let (Some(imp), Ok(Presentation::Of(p))) = (implicit_of(sep, band), other)
                && sign_on(&Psi::Carrier(imp), p, band).is_some()
            {
                return Outcome::Certified;
            }
        }
    }
    let (Ok(Presentation::Of(qx)), Ok(Presentation::Of(qy))) = (&px, &py) else {
        if matches!(px, Err(Unpresented::Structural)) && matches!(py, Err(Unpresented::Structural))
        {
            return Outcome::Refuse(EngineRefusal::Uncertified {
                a: x.face,
                b: y.face,
                shared,
                arm: CertificateArm::Departure,
            });
        }
        return split;
    };
    let opposite = |psi: &Psi| match (sign_on(psi, qx, band), sign_on(psi, qy, band)) {
        (Some(a), Some(b)) => a == flip(b),
        _ => false,
    };
    if let Some(psi) = plane_through(x, near, qx, qy)
        && opposite(&psi)
    {
        return Outcome::Certified;
    }
    if let Some(psi) = graph_of(x, near, qx, qy, band)
        && opposite(&psi)
    {
        return Outcome::Certified;
    }
    if let Some(shared) = cusp(x, near, qx, qy, y.boundary.sense, band) {
        return Outcome::Cusp { shared };
    }
    split
}

/// A point of the shared cell: a shared vertex, or the shared edge's
/// start.
fn anchor(x: &Piece<'_>, near: &Near) -> Option<Point3<Interval>> {
    if let Some(v) = near.vertices.first() {
        return x.boundary.vertices.get(v).copied();
    }
    let &(i, _) = near.edges.first()?;
    let (carrier, span) = x.boundary.edges.get(i)?.arc.as_ref()?;
    Some(carrier.eval(span.0))
}

/// The heuristic direction a presented face leaves `q` in: toward its
/// rectangle's middle. Chooses a separator; decides nothing.
fn heading(p: &Presented<'_>, q: Point3<f64>) -> Option<Vec3<f64>> {
    let x = geom::mid_param(p.x.0, p.x.1);
    let y = geom::mid_param(p.y.0, p.y.1);
    unit(pmid(p.image((x, x), (y, y))) - q)
}

/// **A plane through the shared cell**, where its edges are straight:
/// through one line edge, bisecting the faces' headings about it;
/// through two line edges at a shared vertex; or, at a shared vertex
/// alone, across the faces' two headings.
fn plane_through(
    x: &Piece<'_>,
    near: &Near,
    px: &Presented<'_>,
    py: &Presented<'_>,
) -> Option<Psi> {
    let q = anchor(x, near)?;
    let q0 = pmid(q);
    let (hx, hy) = (heading(px, q0)?, heading(py, q0)?);
    let lines: Vec<Vec3<f64>> = near
        .edges
        .iter()
        .map(|&(i, _)| match x.boundary.edges.get(i)?.arc.as_ref()? {
            (Curve3::Line { dir, .. }, _) => unit(vmid(*dir)),
            _ => None,
        })
        .collect::<Option<_>>()?;
    let n = match lines.as_slice() {
        [] => unit(hx - hy)?,
        [d] => unit(perp(hx, *d) - perp(hy, *d))?,
        [d, e] if !near.vertices.is_empty() => unit(d.cross(*e))?,
        _ => return None,
    };
    Some(Psi::Linear { n: lift(n), q })
}

/// The piece of the shared edge whose points can lie in the two
/// presented images: a hull of parameters holding every one whose point
/// the images' boxes may hold. A coarse pass finds the pieces that meet
/// either box — through its seam, on a closed conic, the complement of
/// the widest run that meets neither — and the hull's two ends are then
/// narrowed by bisection toward the first and last parameter that still
/// meets. Each end only moves inward past pieces decided to meet
/// neither box, so the hull keeps every parameter it must.
fn edge_piece(
    carrier: &Curve3<Interval>,
    span: (Interval, Interval),
    closed: bool,
    boxes: &[Point3<Interval>],
) -> Option<(f64, f64)> {
    const COARSE: usize = 64;
    const DEPTH: u32 = 20;
    let (t0, t1) = (span.0.lo(), span.1.hi());
    let width = t1 - t0;
    if !(width.is_finite() && width > 0.0) {
        return None;
    }
    let meets = |a: f64, b: f64| {
        let enc = arc_enclosure(carrier, (a.min(b), a.max(b)));
        boxes.iter().any(|&bx| overlap(enc, bx))
    };
    #[allow(clippy::cast_precision_loss)]
    let at_k = |k: usize| t0 + width * (k as f64) / (COARSE as f64);
    let hit: Vec<bool> = (0..COARSE).map(|k| meets(at_k(k), at_k(k + 1))).collect();
    let first = hit.iter().position(|&h| h)?;
    let last = hit.iter().rposition(|&h| h)?;
    let conic = matches!(carrier, Curve3::Circle { .. } | Curve3::Ellipse { .. });
    // The run of pieces, in order, the hull spans: `(start, count)`,
    // cyclic through the seam on a closed conic.
    let (start, count) = if closed && conic && first == 0 && last == COARSE - 1 {
        // The widest cyclic run of misses; the hull is its complement.
        let mut best = (0usize, 0usize);
        for s in 0..COARSE {
            if hit[s] {
                continue;
            }
            let run = (0..COARSE).take_while(|&k| !hit[(s + k) % COARSE]).count();
            if run > best.1 {
                best = (s, run);
            }
        }
        if best.1 == 0 {
            return Some((t0, t1));
        }
        ((best.0 + best.1) % COARSE, COARSE - best.1)
    } else {
        (first, last - first + 1)
    };
    // Piece `k` of the run, unwrapped past the seam by one period.
    let piece = |k: usize| {
        let i = start + k;
        let shift = if i >= COARSE { width } else { 0.0 };
        (at_k(i % COARSE) + shift, at_k(i % COARSE + 1) + shift)
    };
    let narrow = |(mut a, mut b): (f64, f64), from_left: bool| -> f64 {
        for _ in 0..DEPTH {
            let m = geom::mid_param(a, b);
            if !(a < m && m < b) {
                break;
            }
            let (near, far) = if from_left {
                ((a, m), (m, b))
            } else {
                ((m, b), (a, m))
            };
            (a, b) = if meets(near.0, near.1) { near } else { far };
        }
        if from_left { a } else { b }
    };
    let lo = narrow(piece(0), true);
    let hi = narrow(piece(count - 1), false);
    (lo < hi).then_some((lo, hi))
}

/// **The shared edge's graph**: one shared edge, projected onto the
/// plane of `t`, its direction over the piece both images can reach,
/// and `m`, across the faces' headings. Where `t·c′` is decided positive
/// over the piece, the projected piece is the graph of `γ` over `s`.
fn graph_of(
    x: &Piece<'_>,
    near: &Near,
    px: &Presented<'_>,
    py: &Presented<'_>,
    band: Band,
) -> Option<Psi> {
    let [(i, _)] = near.edges.as_slice() else {
        return None;
    };
    let e = x.boundary.edges.get(*i)?;
    let (carrier, span) = e.arc.as_ref()?;
    let piece = edge_piece(
        carrier,
        *span,
        e.ends[0] == e.ends[1],
        &[px.whole(), py.whole()],
    )?;
    let middle = at(geom::mid_param(piece.0, piece.1));
    let t = unit(vmid(carrier.deriv(middle)))?;
    let q0 = pmid(carrier.eval(middle));
    let (hx, hy) = (heading(px, q0)?, heading(py, q0)?);
    let m = unit(perp(hx, t) - perp(hy, t))?;
    let (t, m) = (lift(t), lift(m));
    let d = arc_deriv(carrier, piece);
    let advance = t.dot(d);
    if sign_of(advance * at(piece.1 - piece.0), GRAPH, band) != Some(Sign::Positive) {
        return None;
    }
    Some(Psi::Graph {
        m,
        t,
        slope: m.dot(d) / advance,
    })
}

/// **A wedge-0/2π shared edge**: the faces' outward normals opposed at
/// the middle of the side each presents on it. The second-order arm that
/// would read it is not built.
fn cusp(
    x: &Piece<'_>,
    near: &Near,
    px: &Presented<'_>,
    py: &Presented<'_>,
    y_sense: bool,
    band: Band,
) -> Option<SharedCell> {
    let [(i, _)] = near.edges.as_slice() else {
        return None;
    };
    let (Zero::Side(sx), Zero::Side(sy)) = (px.zero, py.zero) else {
        return None;
    };
    let nx = px.normal_at(sx, x.boundary.sense)?;
    let ny = py.normal_at(sy, y_sense)?;
    let gap = lift(nx + ny).norm() * x.ball.reach();
    if !matches!(decide(CUSP, Margin::of(gap), band), Ok(Sign::Zero)) {
        return None;
    }
    Some(SharedCell::Edge(x.boundary.edges.get(*i)?.key))
}

// ----------------------------------------------------- the monotone arm

/// **The monotone arm** over the chart rectangle `u × v` of `surface`:
/// `L` is the polar factor of the centre's Jacobian (columns scaled by
/// the spans), so `L∘S` is near the identity there, and the arm holds
/// where the least eigenvalue of the symmetric part of `J(L∘S)` is
/// decided positive over the whole rectangle.
pub(crate) fn monotone(
    surface: &Surface<Interval>,
    u: (f64, f64),
    v: (f64, f64),
    band: Band,
) -> bool {
    let (du, dv) = (u.1 - u.0, v.1 - v.0);
    let centre = surface.jet(at(geom::mid_param(u.0, u.1)), at(geom::mid_param(v.0, v.1)));
    let (a, b) = (vmid(centre.du) * du, vmid(centre.dv) * dv);
    // G = JᵀJ and its square root R; L = R⁻¹Jᵀ has orthonormal rows and
    // L·J = R.
    let (g11, g12, g22) = (a.dot(a), a.dot(b), b.dot(b));
    let det = g11 * g22 - g12.powi(2);
    if !(det.is_finite() && det > 0.0) {
        return false;
    }
    let sd = det.sqrt();
    let k = (g11 + g22 + 2.0 * sd).sqrt();
    let (r11, r12, r22) = ((g11 + sd) / k, g12 / k, (g22 + sd) / k);
    let rdet = r11 * r22 - r12.powi(2);
    if !(rdet.is_finite() && rdet > 0.0) {
        return false;
    }
    let (i11, i12, i22) = (r22 / rdet, -r12 / rdet, r11 / rdet);
    let l1 = lift(a * i11 + b * i12);
    let l2 = lift(a * i12 + b * i22);
    let (ju, jv, _) = rect_jet(surface, u, v);
    let (su, sv) = (ju * at(du), jv * at(dv));
    let (m11, m12, m21, m22) = (l1.dot(su), l1.dot(sv), l2.dot(su), l2.dot(sv));
    let half = at(0.5);
    let p12 = (m12 + m21) * half;
    let least = (m11 + m22) * half - (((m11 - m22) * half).powi(2) + p12.powi(2)).sqrt();
    matches!(
        decide(MONOTONE, Margin::of(least), band),
        Ok(Sign::Positive)
    )
}

#[cfg(test)]
mod rows {
    use super::covered;

    /// Coverage reads a union of ranges, and a periodic range a turn
    /// away.
    #[test]
    fn coverage_reads_unions_and_turns() {
        use geom_core::interval::Interval;
        let r = |a: f64, b: f64| Interval::from_bounds(a, b);
        assert!(
            covered((0.2, 0.8), &[r(0.0, 0.5), r(0.5, 1.0)], false),
            "a joined union"
        );
        assert!(
            !covered((0.2, 0.8), &[r(0.0, 0.4), r(0.5, 1.0)], false),
            "a gap"
        );
        let turn = core::f64::consts::TAU;
        assert!(
            covered(
                (turn - 0.1, turn + 0.1),
                &[r(0.0, 1.0), r(turn - 1.0, turn)],
                true
            ),
            "a periodic range through the seam"
        );
    }
}

/// **Presentations, read directly**: a face built by hand, its local
/// edges named, and what [`present_chart`] / [`present_plane`] make of a
/// cell. Each row names the failure it catches.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod presentation_rows {
    use super::*;
    use geom_core::Tol;

    const ALPHA: f64 = 0.2;
    const BETA: f64 = 4.0;

    fn iv0(x: f64) -> Interval {
        Interval::from_bounds(x, x)
    }

    fn p2((u, v): (f64, f64)) -> Point2<Interval> {
        Point2::new(iv0(u), iv0(v))
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn vertex(n: u64) -> VertexKey {
        slotmap::KeyData::from_ffi((1u64 << 32) | n).into()
    }

    fn edge(n: u64) -> EdgeKey {
        slotmap::KeyData::from_ffi((1u64 << 32) | n).into()
    }

    fn origin() -> Point3<Interval> {
        Point3::new(iv0(0.0), iv0(0.0), iv0(0.0))
    }

    fn cylinder() -> Surface<Interval> {
        Surface::Cylinder {
            origin: origin(),
            axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            radius: iv0(1.0),
            u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        }
    }

    /// An edge of one use, from `a` to `b` in its loop, whose chart image
    /// holds `held` at `at`.
    fn rim(n: u64, ends: [u64; 2], held: Axis, at: f64, a: (f64, f64), b: (f64, f64)) -> Rim {
        Rim {
            key: edge(n),
            arc: Some((
                Curve3::Line {
                    origin: origin(),
                    dir: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                },
                (iv0(0.0), iv0(1.0)),
            )),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus: true,
                image: Some([p2(a), p2(b)]),
                iso: Some(Iso {
                    held,
                    at: iv0(at),
                    full: false,
                }),
            }],
        }
    }

    /// **The face across its chart's seam**: a cylinder strip over
    /// `u ∈ [β, α + 2π]`, `v ∈ [0, 1]`, its loop counter-clockwise about
    /// the outward normal. The `α` edge's image is stored on the stored
    /// sheet (`u = α`), the rest on the lifted one — so a reading that
    /// wraps a line toward whatever it last clipped lands it on the
    /// wrong sheet. `alpha_first` puts the `α` edge first in
    /// [`Boundary::edges`].
    fn seam_strip(alpha_first: bool) -> Boundary {
        let top = ALPHA + TURN;
        let mut edges = vec![
            rim(1, [1, 2], Axis::U, ALPHA, (ALPHA, 0.0), (ALPHA, 1.0)),
            rim(2, [2, 3], Axis::V, 1.0, (top, 1.0), (BETA, 1.0)),
            rim(3, [3, 4], Axis::U, BETA, (BETA, 1.0), (BETA, 0.0)),
            rim(4, [4, 1], Axis::V, 0.0, (BETA, 0.0), (top, 0.0)),
        ];
        if !alpha_first {
            edges.reverse();
        }
        Boundary {
            sense: true,
            edges,
            vertices: BTreeMap::new(),
            extent: Some([(BETA, top), (0.0, 1.0)]),
        }
    }

    /// What a presentation came to, without its borrow.
    #[derive(Debug, PartialEq)]
    enum Read {
        Vacuous,
        Of((f64, f64), (f64, f64)),
        Local,
        Structural,
        Collapsed,
    }

    fn read(p: Result<Presentation<'_>, Unpresented>) -> Read {
        match p {
            Ok(Presentation::Vacuous) => Read::Vacuous,
            Ok(Presentation::Of(q)) => Read::Of(q.x, q.y),
            Err(Unpresented::Local) => Read::Local,
            Err(Unpresented::Structural) => Read::Structural,
            Err(Unpresented::Collapsed(_)) => Read::Collapsed,
        }
    }

    fn chart(
        surface: &Surface<Interval>,
        b: &Boundary,
        local: &[usize],
        u: (f64, f64),
        v: (f64, f64),
    ) -> Read {
        let local = Local {
            edges: local.to_vec(),
            vertices: Vec::new(),
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface,
            whole: false,
            boundary: b,
            u,
            v,
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        read(present_chart(&piece, &[], &[], band()))
    }

    /// Every local edge of [`seam_strip`] in the given edge order.
    fn all(n: usize) -> Vec<usize> {
        (0..n).collect()
    }

    /// **MAJOR-A's case, both orders.** A cell spanning the whole turn
    /// meets the strip on both sides of the seam: no one rectangle
    /// presents it, in either order of the edges — never the sliver
    /// `(0, α)` that left `[β, 2π]` unread.
    #[test]
    fn a_cell_across_the_seam_is_not_presented_in_either_order() {
        let s = cylinder();
        for alpha_first in [true, false] {
            let b = seam_strip(alpha_first);
            assert_eq!(
                chart(&s, &b, &all(4), (0.0, TURN), (0.0, 1.0)),
                Read::Local,
                "alpha first: {alpha_first}"
            );
        }
    }

    /// The half turn the strip's long part lies in is presented whole,
    /// `[β, α + 2π] ⊇ [β, 2π]`, the same in both orders; the slice by
    /// the seam reads the short part; the gap between them holds none
    /// of the face.
    #[test]
    fn each_side_of_the_seam_presents_its_own_part_in_both_orders() {
        let s = cylinder();
        let half = core::f64::consts::PI;
        for alpha_first in [true, false] {
            let b = seam_strip(alpha_first);
            match chart(&s, &b, &all(4), (half, TURN), (0.0, 1.0)) {
                Read::Of(x, _) => assert!(
                    x.0 <= BETA && TURN <= x.1,
                    "the long part [β, 2π] is held: {x:?} (alpha first: {alpha_first})"
                ),
                other => panic!("the long part is presented, got {other:?}"),
            }
            match chart(
                &s,
                &b,
                &[usize::from(!alpha_first) * 3],
                (0.0, 0.1),
                (0.3, 0.7),
            ) {
                Read::Of(x, _) => {
                    assert!(x.0 <= 0.0 && 0.1 <= x.1 && x.1 <= ALPHA + 1e-12, "{x:?}")
                }
                other => panic!("the short part is presented, got {other:?}"),
            }
            assert_eq!(
                chart(&s, &b, &all(4), (1.0, 3.0), (0.0, 1.0)),
                Read::Vacuous,
                "the gap (α, β) holds none of the strip"
            );
        }
    }

    /// A face whose extent is unknown has no sheet: a line on its
    /// periodic axis is not placed, so the face is not presented there
    /// rather than read on a guessed sheet.
    #[test]
    fn a_seam_line_without_the_faces_extent_is_not_placed() {
        let s = cylinder();
        let mut b = seam_strip(true);
        b.extent = None;
        assert_eq!(
            chart(&s, &b, &[0], (0.0, 0.1), (0.3, 0.7)),
            Read::Structural
        );
    }

    /// **A slot**: a face over `u ∈ [0, 3]` with a hole across
    /// `u ∈ [1, 1.5]`, `v ∈ [0.2, 0.8]`. A cell past the hole's left
    /// line and beside its right one is on the face: it must not read
    /// vacuous off the left line (the defect the old `outside` branch
    /// had). Inside the hole, touching both lines, it is vacuous; past
    /// the left line with a gap between, it is not decided.
    fn slot() -> Boundary {
        // The hole's loop runs clockwise about the outward normal.
        Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 2], Axis::U, 1.0, (1.0, 0.2), (1.0, 0.8)),
                rim(2, [3, 4], Axis::U, 1.5, (1.5, 0.8), (1.5, 0.2)),
            ],
            vertices: BTreeMap::new(),
            extent: Some([(0.0, 3.0), (0.0, 1.0)]),
        }
    }

    #[test]
    fn a_cell_past_one_side_of_a_slot_is_read_by_the_other() {
        let s = cylinder();
        let b = slot();
        assert_eq!(
            chart(&s, &b, &[0, 1], (1.6, 2.0), (0.3, 0.7)),
            Read::Of((1.5, 2.0), (0.3, 0.7)),
            "beside the right line, the face side: presented from it"
        );
        assert_eq!(
            chart(&s, &b, &[0, 1], (1.0, 1.4), (0.3, 0.7)),
            Read::Vacuous,
            "inside the hole, touching its left line"
        );
        assert_eq!(
            chart(&s, &b, &[0], (1.2, 1.4), (0.3, 0.7)),
            Read::Local,
            "apart from the only line it lies past: not decided"
        );
        assert_eq!(
            chart(&s, &b, &[0, 1], (0.5, 2.5), (0.3, 0.7)),
            Read::Local,
            "the face leaves the cell and comes back: no one rectangle"
        );
    }

    /// **`face_side`**: the face lies left of its boundary about the
    /// outward normal; the sense flips it.
    #[test]
    fn face_side_reads_the_loop_and_the_sense() {
        let iso = |held, at| Iso {
            held,
            at: iv0(at),
            full: false,
        };
        let up = Use {
            plus: true,
            image: Some([p2((1.0, 0.0)), p2((1.0, 1.0))]),
            iso: None,
        };
        assert_eq!(
            face_side(&up, iso(Axis::U, 1.0), true),
            Some(-1.0),
            "v rising at u = 1: face below u"
        );
        assert_eq!(
            face_side(&up, iso(Axis::U, 1.0), false),
            Some(1.0),
            "reversed sense"
        );
        let east = Use {
            plus: true,
            image: Some([p2((0.0, 0.0)), p2((1.0, 0.0))]),
            iso: None,
        };
        assert_eq!(
            face_side(&east, iso(Axis::V, 0.0), true),
            Some(1.0),
            "u rising at v = 0: face above v"
        );
        let back = Use {
            plus: false,
            ..east
        };
        assert_eq!(
            face_side(&back, iso(Axis::V, 0.0), true),
            Some(-1.0),
            "run backward"
        );
        let flat = Use {
            plus: true,
            image: Some([p2((0.0, 0.0)), p2((0.0, 0.0))]),
            iso: None,
        };
        assert_eq!(
            face_side(&flat, iso(Axis::V, 0.0), true),
            None,
            "no direction, no side"
        );
    }

    /// **`convex`**: the strip's corner where its right line meets its
    /// bottom one is convex; the same two lines read with the right one
    /// as a LOW clip are not.
    #[test]
    fn convex_reads_a_corner_the_edges_leave_into() {
        let b = seam_strip(true);
        let clip = |side, i: usize| Clip {
            side,
            at: iv0(0.0),
            edges: vec![i],
            collapsed: None,
        };
        assert!(convex(&b, &clip(Side::XHi, 0), &clip(Side::YLo, 3)));
        assert!(!convex(&b, &clip(Side::XLo, 0), &clip(Side::YLo, 3)));
    }

    /// **`zero_of`**: the sides on the shared cell. One shared line is a
    /// side, two on different axes a corner, a shared vertex at the
    /// corner of two unshared lines a point, and two shared lines on one
    /// axis are not one zero set.
    #[test]
    fn zero_of_reads_sides_corners_and_points() {
        let b = seam_strip(true);
        let clip = |side, i: usize| Clip {
            side,
            at: iv0(0.0),
            edges: vec![i],
            collapsed: None,
        };
        let right = clip(Side::XHi, 0);
        let bottom = clip(Side::YLo, 3);
        let clips = [right, bottom];
        assert!(matches!(
            zero_of(&b, &clips, &[0], &[]),
            Ok(Zero::Side(Side::XHi))
        ));
        assert!(matches!(
            zero_of(&b, &clips, &[0, 3], &[]),
            Ok(Zero::Corner(Side::XHi, Side::YLo))
        ));
        assert!(matches!(
            zero_of(&b, &clips, &[], &[vertex(1)]),
            Ok(Zero::Point(Side::XHi, Side::YLo))
        ));
        assert!(matches!(zero_of(&b, &clips, &[], &[]), Ok(Zero::None)));
        let left = clip(Side::XLo, 2);
        assert!(zero_of(&b, &[clip(Side::XHi, 0), left], &[0, 2], &[]).is_err());
    }

    /// **`present_plane`**: one line edge presents the half plane on its
    /// face side, its side zero where the edge is shared; a cell wholly
    /// past the line holds none of the face.
    #[test]
    fn a_plane_presents_the_face_side_of_its_one_edge() {
        let plane = Surface::Plane {
            origin: origin(),
            normal: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        };
        // The edge from (0, 0) to (4, 0), run forward: the face above it.
        let b = Boundary {
            sense: true,
            edges: vec![Rim {
                key: edge(1),
                arc: Some((
                    Curve3::Line {
                        origin: origin(),
                        dir: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                    },
                    (iv0(0.0), iv0(4.0)),
                )),
                ends: [vertex(1), vertex(2)],
                uses: vec![Use {
                    plus: true,
                    image: None,
                    iso: None,
                }],
            }],
            vertices: BTreeMap::new(),
            extent: None,
        };
        let local = Local {
            edges: vec![0],
            vertices: Vec::new(),
        };
        let piece = |u, v| Piece {
            face: FaceKey::default(),
            surface: &plane,
            whole: false,
            boundary: &b,
            u,
            v,
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        match present_plane(&piece((1.0, 2.0), (-0.5, 0.5)), &[0], &[], band()) {
            Ok(Presentation::Of(q)) => {
                assert!(
                    matches!(q.zero, Zero::Side(Side::YLo)),
                    "the shared edge is the zero side"
                );
                assert_eq!(q.y.0, 0.0, "clipped to the edge's line");
            }
            _ => panic!("a cell across the edge is presented"),
        }
        assert!(matches!(
            present_plane(&piece((1.0, 2.0), (-0.5, -0.1)), &[0], &[], band()),
            Ok(Presentation::Vacuous)
        ));
    }

    /// **`edge_piece`**: the hull of an edge's parameters whose points the
    /// boxes may hold, narrowed to them from both ends.
    #[test]
    fn edge_piece_narrows_to_the_parameters_the_boxes_hold() {
        let line = Curve3::Line {
            origin: origin(),
            dir: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        };
        let bx = |a: f64, b: f64| {
            Point3::new(
                Interval::from_bounds(a, b),
                Interval::from_bounds(-0.1, 0.1),
                Interval::from_bounds(-0.1, 0.1),
            )
        };
        let (a, b) = edge_piece(&line, (iv0(0.0), iv0(10.0)), false, &[bx(2.0, 3.0)]).unwrap();
        assert!(
            a <= 2.0 && 3.0 <= b,
            "every held parameter kept: ({a}, {b})"
        );
        assert!(1.5 < a && b < 3.5, "and narrowed to them: ({a}, {b})");
        assert!(edge_piece(&line, (iv0(0.0), iv0(10.0)), false, &[bx(20.0, 30.0)]).is_none());
    }
}
