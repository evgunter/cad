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
/// images, its vertices.
#[derive(Clone, Debug, Default)]
pub(crate) struct Boundary {
    pub(crate) sense: bool,
    pub(crate) edges: Vec<Rim>,
    pub(crate) vertices: BTreeMap<VertexKey, Point3<Interval>>,
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
}

/// **The analytic carrier's own injectivity**: every analytic carrier
/// the engine admits is one-to-one on its chart's fundamental domain,
/// and a face is a region of it. A torus is, where `R > r`.
///
/// # Errors
///
/// The torus that is not decided a ring.
pub(crate) fn by_kind(face: FaceKey, surface: &Surface<Interval>, band: Band) -> Result<(), EngineRefusal> {
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
        Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } => {
            axis == Axis::U
        }
        Surface::Torus { .. } => true,
        _ => false,
    }
}

const TURN: f64 = core::f64::consts::TAU;

/// `x` moved by whole turns to sit nearest `near`.
fn wrap(x: Interval, near: f64) -> Interval {
    let k = ((near - mid(x)) / TURN).round();
    if k == 0.0 {
        x
    } else {
        x + at(k * TURN)
    }
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
}

/// What presenting a face in a cell came to.
enum Presentation<'a> {
    /// The face has no point in the cell.
    Vacuous,
    Of(Presented<'a>),
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
        self.image(self.x, self.y)
    }

    /// `(Φ_x, Φ_y, Φ_xy)` over the rectangle.
    fn jet(&self) -> (Vec3<Interval>, Vec3<Interval>, Vec3<Interval>) {
        let (u, v) = self.chart(iv(self.x), iv(self.y));
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

    /// The image of one side.
    fn side(&self, side: Side) -> Point3<Interval> {
        let b = self.bound(side);
        match side.axis() {
            Axis::U => self.image((b, b), self.y),
            Axis::V => self.image(self.x, (b, b)),
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
    Some(Interval::from_bounds(a.lo().min(b.lo()), a.hi().max(b.hi())))
}

/// Whether `want` lies in the union of `ranges`, each moved by whole
/// turns where the axis is periodic.
fn covered(want: (f64, f64), ranges: &[Interval], periodic: bool) -> bool {
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

/// The chart presentation: the cell's own rectangle, clipped by the iso
/// lines of the boundary edges near it. An iso-bounded NURBS face's
/// cells lie in its domain, whose boundary lines are its edges.
fn present_chart<'a>(
    piece: &Piece<'a>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> Result<Presentation<'a>, Unpresented> {
    let b = piece.boundary;
    let (mut x, mut y) = (piece.u, piece.v);
    let mut clips: Vec<Clip> = Vec::new();
    let mut outside: Vec<(Side, Interval, Vec<usize>)> = Vec::new();
    let range = |axis: Axis, x: (f64, f64), y: (f64, f64)| match axis {
        Axis::U => x,
        Axis::V => y,
    };
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
            let (lo, hi) = range(iso.held, x, y);
            if piece.whole {
                // The line is a bound of the exact knot domain.
                let exact = |b: f64| iso.at.lo() == b && iso.at.hi() == b;
                if exact(lo) {
                    clips.push(Clip {
                        side: Side::of(iso.held, true),
                        at: iso.at,
                        edges: vec![ei],
                        collapsed: None,
                    });
                } else if exact(hi) {
                    clips.push(Clip {
                        side: Side::of(iso.held, false),
                        at: iso.at,
                        edges: vec![ei],
                        collapsed: None,
                    });
                }
                continue;
            }
            let Some(sigma) = face_side(u, iso, b.sense) else {
                return Err(unread(ei, near_edges));
            };
            let line = if periodic(piece.surface, iso.held) {
                wrap(iso.at, geom::mid_param(lo, hi))
            } else {
                iso.at
            };
            let side = Side::of(iso.held, sigma > 0.0);
            if (sigma > 0.0 && line.lo() >= hi) || (sigma < 0.0 && line.hi() <= lo) {
                outside.push((side, line, vec![ei]));
                continue;
            }
            let r = if sigma > 0.0 {
                (line.lo(), hi)
            } else {
                (lo, line.hi())
            };
            match iso.held {
                Axis::U => x = r,
                Axis::V => y = r,
            }
            clips.push(Clip {
                side,
                at: line,
                edges: vec![ei],
                collapsed: None,
            });
        }
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
            // The face's nappe: the sign of the slant coordinate at its
            // vertices off the apex.
            let nappe = b
                .vertices
                .values()
                .map(|&q| (q - *apex).dot(*axis))
                .find_map(|h| sign_of(h, APEX, band));
            let Some(nappe) = nappe else {
                return Err(Unpresented::Local);
            };
            let lo = nappe == Sign::Positive;
            if (lo && y.1 <= 0.0) || (!lo && y.0 >= 0.0) {
                return Ok(Presentation::Vacuous);
            }
            y = if lo { (y.0.max(0.0), y.1) } else { (y.0, y.1.min(0.0)) };
            clips.push(Clip {
                side: Side::of(Axis::V, lo),
                at: at(0.0),
                edges: Vec::new(),
                collapsed: Some(v),
            });
        }
    }
    // Lines on one side merge; two different lines on one side do not
    // bound a face the clip reads.
    let mut merged: Vec<Clip> = Vec::new();
    for c in clips {
        match merged.iter_mut().find(|m| m.side == c.side) {
            Some(m) if m.at.lo() <= c.at.hi() && c.at.lo() <= m.at.hi() => {
                m.edges.extend(c.edges);
                m.collapsed = m.collapsed.or(c.collapsed);
            }
            Some(_) => return Err(Unpresented::Local),
            None => merged.push(c),
        }
    }
    if x.0 > x.1 || y.0 > y.1 {
        return Ok(Presentation::Vacuous);
    }
    let edge_ranges = |edges: &[usize], held: Axis| -> Option<Vec<Interval>> {
        let mut out = Vec::new();
        for &ei in edges {
            let e = b.edges.get(ei)?;
            for u in &e.uses {
                let iso = u.iso?;
                if iso.held != held {
                    continue;
                }
                if iso.full {
                    out.push(Interval::from_bounds(f64::NEG_INFINITY, f64::INFINITY));
                } else {
                    out.push(along(u, iso)?);
                }
            }
        }
        Some(out)
    };
    // Every clip's side must be the edges' own: an edge that ends inside
    // the cell leaves the face free to wrap round its end.
    let other = |axis: Axis| match axis {
        Axis::U => (y, periodic(piece.surface, Axis::V)),
        Axis::V => (x, periodic(piece.surface, Axis::U)),
    };
    for c in &merged {
        if c.collapsed.is_some() || piece.whole {
            continue;
        }
        let (want, per) = other(c.side.axis());
        let Some(ranges) = edge_ranges(&c.edges, c.side.axis()) else {
            return Err(Unpresented::Local);
        };
        if !covered(want, &ranges, per) {
            return Err(Unpresented::Local);
        }
    }
    // A cell wholly past an edge's line holds none of the face, when the
    // line across it is the edge's.
    for (side, _, edges) in &outside {
        let (want, per) = other(side.axis());
        let Some(ranges) = edge_ranges(edges, side.axis()) else {
            return Err(Unpresented::Local);
        };
        if covered(want, &ranges, per) {
            return Ok(Presentation::Vacuous);
        }
        return Err(Unpresented::Local);
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
    Ok(Presentation::Of(Presented {
        surface: piece.surface,
        map: Map::Chart,
        x,
        y,
        zero,
    }))
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
    let on_shared = |c: &Clip| {
        if let Some(v) = c.collapsed {
            near_vertices.contains(&v)
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
        return Ok(Presentation::Of(Presented {
            surface: piece.surface,
            map: Map::Chart,
            x: piece.u,
            y: piece.v,
            zero,
        }));
    }
    // A line edge's chart image, in traversal order.
    let segment = |ei: usize| -> Result<(Point2<Interval>, Point2<Interval>), Unpresented> {
        let e = b.edges.get(ei).ok_or(Unpresented::Local)?;
        let line = matches!(e.arc, Some((Curve3::Line { .. }, _)));
        let u = e.uses.first().ok_or(Unpresented::Local)?;
        let [p, q] = u.image.ok_or_else(|| unread(ei, near_edges))?;
        if !line || e.uses.len() != 1 {
            return Err(unread(ei, near_edges));
        }
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
            Ok(Presentation::Of(Presented {
                surface: piece.surface,
                map: Map::Affine {
                    o: [p.x, p.y],
                    a,
                    b: bv,
                },
                x: (x.lo(), x.hi()),
                y: (0.0, y.hi()),
                zero,
            }))
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
            let start1 = if r1.uses[0].plus { r1.ends[0] } else { r1.ends[1] };
            let start2 = if r2.uses[0].plus { r2.ends[0] } else { r2.ends[1] };
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
            Ok(Presentation::Of(Presented {
                surface: piece.surface,
                map: Map::Affine {
                    o: [o.x, o.y],
                    a,
                    b: bv,
                },
                x: (0.0, x.hi()),
                y: (0.0, y.hi()),
                zero,
            }))
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
    Linear { n: Vec3<Interval>, q: Point3<Interval> },
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
    let zero_on = |b: Point3<Interval>| !checked || psi.value(b).is_some_and(|v| is_zero(v, band));
    let (fx, fy, fxy) = p.jet();
    let image = p.whole();
    let grad = psi.gradient(image);
    let across = |axis: Axis| match axis {
        Axis::U => grad.dot(fx) * at(p.span(Axis::U)),
        Axis::V => grad.dot(fy) * at(p.span(Axis::V)),
    };
    match p.zero {
        Zero::None => sign_of(psi.value(image)?, DEPARTURE, band),
        Zero::Side(s) => {
            if !zero_on(p.side(s)) {
                return None;
            }
            sign_of(across(s.axis()) * at(s.into()), DEPARTURE, band)
        }
        Zero::Corner(s, t) => {
            let n = psi.linear()?;
            if !(zero_on(p.side(s)) && zero_on(p.side(t))) {
                return None;
            }
            let g = n.dot(fxy) * at(p.span(Axis::U) * p.span(Axis::V) * s.into() * t.into());
            sign_of(g, DEPARTURE, band)
        }
        Zero::Point(s, t) => {
            if !zero_on(p.corner(s, t)) {
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
            let nappe = piece
                .boundary
                .vertices
                .values()
                .map(|&q| (q - apex).dot(axis))
                .find_map(|h| sign_of(h, APEX, band))?;
            Implicit::Cone {
                apex,
                axis,
                cos,
                sin,
                nappe: at(if nappe == Sign::Positive { 1.0 } else { -1.0 }),
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
    if let Some(refusal) = cusp(x, y, near, qx, qy, band) {
        return Outcome::Refuse(refusal);
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
fn plane_through(x: &Piece<'_>, near: &Near, px: &Presented<'_>, py: &Presented<'_>) -> Option<Psi> {
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
/// presented images: the hull of the parameter pieces whose enclosure
/// meets either image, by bisection. A closed conic's piece may run
/// through its seam.
fn edge_piece(
    carrier: &Curve3<Interval>,
    span: (Interval, Interval),
    closed: bool,
    boxes: &[Point3<Interval>],
) -> Option<(f64, f64)> {
    const DEPTH: u32 = 24;
    let (t0, t1) = (span.0.lo(), span.1.hi());
    let mut kept: Vec<(f64, f64)> = Vec::new();
    let mut stack = vec![(t0, t1, 0u32)];
    while let Some((a, b, depth)) = stack.pop() {
        let enc = carrier.eval(Interval::from_bounds(a, b));
        if !boxes.iter().any(|&bx| overlap(enc, bx)) {
            continue;
        }
        let m = geom::mid_param(a, b);
        if depth >= DEPTH || !(a < m && m < b) {
            kept.push((a, b));
        } else {
            stack.push((m, b, depth + 1));
            stack.push((a, m, depth + 1));
        }
    }
    kept.sort_by(|p, q| p.0.total_cmp(&q.0));
    let (first, last) = (*kept.first()?, *kept.last()?);
    let conic = matches!(carrier, Curve3::Circle { .. } | Curve3::Ellipse { .. });
    if closed && conic && first.0 <= t0 && last.1 >= t1 {
        // Through the seam: the complement of the widest gap.
        let mut widest = (0.0, None);
        for w in kept.windows(2) {
            if let [p, q] = w {
                let gap = q.0 - p.1;
                if gap > widest.0 {
                    widest = (gap, Some((p.1, q.0)));
                }
            }
        }
        if let (_, Some((gap_lo, gap_hi))) = widest {
            return Some((gap_hi, gap_lo + (t1 - t0)));
        }
    }
    Some((first.0, last.1))
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
    let d = carrier.deriv(iv(piece));
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
/// would read it is not built, so it refuses typed.
fn cusp(
    x: &Piece<'_>,
    y: &Piece<'_>,
    near: &Near,
    px: &Presented<'_>,
    py: &Presented<'_>,
    band: Band,
) -> Option<EngineRefusal> {
    let [(i, _)] = near.edges.as_slice() else {
        return None;
    };
    let (Zero::Side(sx), Zero::Side(sy)) = (px.zero, py.zero) else {
        return None;
    };
    let nx = px.normal_at(sx, x.boundary.sense)?;
    let ny = py.normal_at(sy, y.boundary.sense)?;
    let gap = lift(nx + ny).norm() * x.ball.reach();
    if !matches!(decide(CUSP, Margin::of(gap), band), Ok(Sign::Zero)) {
        return None;
    }
    let edge = x.boundary.edges.get(*i)?.key;
    Some(EngineRefusal::Uncertified {
        a: x.face,
        b: y.face,
        shared: SharedCell::Edge(edge),
        arm: CertificateArm::SecondOrder,
    })
}

// ----------------------------------------------------- the monotone arm

/// **The monotone arm** over the chart rectangle `u × v` of `surface`:
/// `L` is the polar factor of the centre's Jacobian (columns scaled by
/// the spans), so `L∘S` is near the identity there, and the arm holds
/// where the least eigenvalue of the symmetric part of `J(L∘S)` is
/// decided positive over the whole rectangle.
pub(crate) fn monotone(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64), band: Band) -> bool {
    let (du, dv) = (u.1 - u.0, v.1 - v.0);
    let centre = surface.jet(
        at(geom::mid_param(u.0, u.1)),
        at(geom::mid_param(v.0, v.1)),
    );
    let (a, b) = (vmid(centre.du) * du, vmid(centre.dv) * dv);
    // G = JᵀJ and its square root R; L = R⁻¹Jᵀ has orthonormal rows and
    // L·J = R.
    let (g11, g12, g22) = (a.dot(a), a.dot(b), b.dot(b));
    let det = g11 * g22 - g12 * g12;
    if !(det.is_finite() && det > 0.0) {
        return false;
    }
    let sd = det.sqrt();
    let k = (g11 + g22 + 2.0 * sd).sqrt();
    let (r11, r12, r22) = ((g11 + sd) / k, g12 / k, (g22 + sd) / k);
    let rdet = r11 * r22 - r12 * r12;
    if !(rdet.is_finite() && rdet > 0.0) {
        return false;
    }
    let (i11, i12, i22) = (r22 / rdet, -r12 / rdet, r11 / rdet);
    let l1 = lift(a * i11 + b * i12);
    let l2 = lift(a * i12 + b * i22);
    let j = surface.jet(iv(u), iv(v));
    let (su, sv) = (j.du * at(du), j.dv * at(dv));
    let (m11, m12, m21, m22) = (l1.dot(su), l1.dot(sv), l2.dot(su), l2.dot(sv));
    let half = at(0.5);
    let p12 = (m12 + m21) * half;
    let least = (m11 + m22) * half - (((m11 - m22) * half).powi(2) + p12.powi(2)).sqrt();
    matches!(decide(MONOTONE, Margin::of(least), band), Ok(Sign::Positive))
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
        assert!(covered((0.2, 0.8), &[r(0.0, 0.5), r(0.5, 1.0)], false), "a joined union");
        assert!(!covered((0.2, 0.8), &[r(0.0, 0.4), r(0.5, 1.0)], false), "a gap");
        let turn = core::f64::consts::TAU;
        assert!(
            covered((turn - 0.1, turn + 0.1), &[r(0.0, 1.0), r(turn - 1.0, turn)], true),
            "a periodic range through the seam"
        );
    }
}
