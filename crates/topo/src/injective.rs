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
//!   face departs it to one side), a plane through straight shared edges
//!   or the plane of a conic one, or the graph of the shared edge under
//!   a projection.
//! - **Second order**, at a wedge-0/2π edge, is not built: such an edge
//!   refuses typed, naming it.
//!
//! Each arm's sign is read off a [`Presented`] face: the face's part of
//! the cell written as the image of a rectangle on whose sides the shared
//! cell lies, or which a curved shared edge's chart curve crosses as a
//! graph over one chart axis ([`present_curved`]), and a derivative
//! across those sides or that curve. At a corner where a curved edge
//! meets another, the shared one is read alone, through the vertex
//! ([`present_corner`]).
//!
//! # The ε concessions
//!
//! A presentation drops points of the face in six places, each stated
//! where it is decided. Every point dropped lies within a few ulps, in
//! the chart, of a line the face's boundary or extent is placed on, or
//! within a few ulps, in the curve's parameter, of a closed edge's
//! vertex, or within the band, in metres, of a shared edge's image or
//! of a corner's other edge:
//!
//! - **the sliver** ([`SLIVER`], [`present_chart`]): a rectangle within
//!   the band of a shared line reads as holding nothing off it;
//! - **the sheet's overlap** ([`sheets`]): a cell overlapping a copy of
//!   the face's extent by no more than [`few_ulps`] meets none of it,
//!   and an extent within [`few_ulps`] of a turn reads as wrapping;
//! - **the touch** ([`vacuous`], [`present_chart`]): a line within
//!   [`few_ulps`] of the cell's side touches it, and a cell reaching no
//!   more than [`few_ulps`] past a line onto its face side lies outside
//!   it;
//! - **the cover** ([`covered`]): an edge whose range stops within
//!   [`few_ulps`] of the cell's side covers it;
//! - **the join** ([`present_curved`]): a closed edge's periodic chart
//!   curve is read through its vertex by its closed form, past a span
//!   that stops short of the period, in the curve's parameter, by the
//!   rounding of `τ`;
//! - **the vertex** ([`on_vertex`]): at a corner the two edges' points at
//!   the vertex, decided within the band of each other, are read as one,
//!   so a face point between the two edges within the band of the corner's
//!   other edge goes unread.

use std::collections::{BTreeMap, BTreeSet};

use geom::{Curve3, Surface};
use geom_core::interval::Interval;
use geom_core::{Band, Bounds, Margin, Point2, Point3, Real, Sign, Vec3};

use crate::cell_region::Ball;
use crate::clearance::{CertificateArm, EngineRefusal, SharedCell, Which};
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
/// holding no point of the face off the line — one of the presentation's
/// ε concessions (the module docs list them).
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

impl Iso {
    /// The coordinate the line moves along, read off a chart point: the
    /// one it does not hold.
    fn moving(self, p: Point2<Interval>) -> Interval {
        match self.held {
            Axis::U => p.y,
            Axis::V => p.x,
        }
    }
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
    /// The image, where it is harmonic.
    pub(crate) harmonic: Option<Harmonic>,
}

/// **A harmonic chart image**, per chart axis `[u, v]`:
/// `p0 + pa·cos t + pb·sin t + pl·t` on the curve's own parameter.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Harmonic {
    pub(crate) p0: [Interval; 2],
    pub(crate) pa: [Interval; 2],
    pub(crate) pb: [Interval; 2],
    pub(crate) pl: [Interval; 2],
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
    /// **The hull of the face's outer loop on each chart axis**, `[u, v]`:
    /// its lift ([`crate::pcurves::loop_lift`]) is one continuous chain,
    /// so on a periodic axis the hull is on ONE sheet, not wrapped. A
    /// row that is a coordinate line whose moving coordinate is monotone
    /// gives its ends, which bound it; a harmonic row its enclosure over
    /// its span, so the hull may be a superset, which every reader takes
    /// soundly: it holds the face, and a superset a whole turn wide is
    /// not taken. It is the FACE's extent only where the loop does not
    /// wrap a periodic axis, and a reader that finds it a whole turn wide
    /// reads nothing of it ([`sheets`]). `None` where the loop has no
    /// lift or a row has no closed form.
    ///
    /// **Why a loop that does not wrap holds the face**: that rests on
    /// the B-rep's region invariant, not on a check made here. The face
    /// lies left of its outer loop about its outward normal (on the
    /// analytic curved kinds, validate's check 6 curved arm reads the
    /// loops' material side against the sense bit), so it is the loop's
    /// bounded side, and its rings lie inside
    /// that loop: every split that leaves a face owning rings re-homes
    /// them by containment (`chord_join`'s `laringmv` step). At rest
    /// that nesting is checked on planar faces only (check 9); on a
    /// curved face it holds by construction (validate's not-yet-checked
    /// list, #638).
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
        named_cell(&self.edges, &self.vertices, x)
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

/// **The one rule a shared cell is named by**: its first edge, by its key
/// in the first face's boundary, else its first vertex; `None` where
/// there is neither, or the edge index does not resolve. The choice of
/// FIRST names the refusal; it decides nothing.
fn named_cell(
    edges: &[(usize, usize)],
    vertices: &[VertexKey],
    x: &Boundary,
) -> Option<SharedCell> {
    match edges.first() {
        Some(&(i, _)) => x.edges.get(i).map(|e| SharedCell::Edge(e.key)),
        None => vertices.first().map(|&v| SharedCell::Vertex(v)),
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

    /// The cell a refusal names: [`Shared::cell`]'s rule, over the cells
    /// both pieces are near.
    fn cell(&self, x: &Boundary) -> Option<SharedCell> {
        named_cell(&self.edges, &self.vertices, x)
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
    /// `corner`: it read a face at a corner ([`present_corner`]).
    Certified { corner: bool },
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
/// and a face is a region of it — the premise is that the face's
/// parameter domain covers its carrier at most once, which the face's
/// validation owns. A torus is, where `R > r`.
///
/// # Errors
///
/// The torus that is not decided a ring, and a free-form carrier, which
/// no kind makes injective: its own pairs go to the monotone arm.
pub(crate) fn by_kind(
    face: FaceKey,
    surface: &Surface<Interval>,
    band: Band,
) -> Result<(), EngineRefusal> {
    match surface {
        Surface::Plane { .. }
        | Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Sphere { .. } => Ok(()),
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => match decide(RING_TORUS, Margin::of(*major_radius - *minor_radius), band) {
            Ok(Sign::Positive) => Ok(()),
            _ => Err(EngineRefusal::TorusNotRing {
                face,
                side: Which::First,
            }),
        },
        Surface::Nurbs(_) | Surface::Approx(_) => Err(EngineRefusal::Unsupported {
            carrier: "a free-form face, which no carrier kind makes injective",
            face,
            side: Which::First,
        }),
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
            side: Which::First,
            vertex: v,
        });
    }
    match near.edges.as_slice() {
        [_] => departure(x, y, near, band, false),
        _ => Outcome::Split {
            arm: CertificateArm::Departure,
            shared: near.cell(x.boundary).unwrap_or(SharedCell::Face(x.face)),
        },
    }
}

/// **A touching pair of two faces**: the departure arm, its separators
/// in a fixed order.
pub(crate) fn certify_pair(x: &Piece<'_>, y: &Piece<'_>, near: &Near, band: Band) -> Outcome {
    departure(x, y, near, band, true)
}

// ------------------------------------------------------------ helpers

/// Whether `x` is exactly the point `k`: structure, not a decision.
pub(crate) fn exactly(x: Interval, k: f64) -> bool {
    x.is_certified() && x.lo() == k && x.hi() == k
}

/// The hull of two enclosures, through the evaluation scalar's hull
/// door: a poison operand poisons the hull. An uncertified operand is
/// not poison: it passes, the hull takes the weaker decoration, and
/// every caller here reads the hull's `lo`/`hi` alone, which drop it.
/// Its bounds still bracket the values its expression was defined on,
/// and the charts read here (a plane's affine map, a cylinder's
/// harmonic one) have no domain to leave, so none arises from them.
fn hull(a: Interval, b: Interval) -> Interval {
    geom_core::spline::SpanLocate::enclosure_hull(a, b)
}

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
        Curve3::Nurbs(n) => n.span_deriv(t.0, t.1).unwrap_or_else(whole_space),
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

// --------------------------------------------- the periodic vocabulary
//
// ONE home for which chart axes are periodic and what a turn is: the
// windows ([`crate::clearance`]'s `window_of`, whose periodic axes are
// these) and the presentations read the same table and the same turn.

/// **Whether a chart axis is periodic**: an analytic carrier's azimuth,
/// and a torus's both angles.
pub(crate) fn periodic<T: Real>(surface: &Surface<T>, axis: Axis) -> bool {
    match surface {
        Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } => axis == Axis::U,
        Surface::Torus { .. } => true,
        _ => false,
    }
}

/// One turn of a periodic axis.
pub(crate) const TURN: f64 = core::f64::consts::TAU;

/// **The widest extent a periodic window may carry**: one whole turn,
/// one ulp wider, so the rounding of `TAU` itself can never make a band
/// that really is a turn read as more than one.
pub(crate) const TURN_CEILING: f64 = {
    // `next_up` is not const on this toolchain; the bit pattern above
    // `TAU` is, and it is the same number `f64::next_up` returns for a
    // positive finite value.
    f64::from_bits(TURN.to_bits() + 1)
};

/// The canonical full turn, one ulp wider at each end so the rounding
/// of `TAU` itself can never shave a sliver of azimuth off a periodic
/// window. An ulp guard is not a tolerance — the same idiom
/// [`bvh::Aabb::padded`] uses, for the same reason.
pub(crate) fn full_turn() -> (f64, f64) {
    (0.0f64.next_down(), TURN_CEILING)
}

/// `k` whole turns, enclosed: the door every enclosed reading that
/// moves a chart value by turns goes through. [`sheets`] alone moves
/// bare `f64` bounds by `k·TURN`; its rounding, and `TURN`'s own, is
/// within the [`few_ulps`] slack it grants and says so.
fn turns(k: f64) -> Interval {
    Interval::tau() * at(k)
}

/// The whole turns that move `x` to sit nearest `near`.
fn turns_toward(x: Interval, near: f64) -> f64 {
    ((near - mid(x)) / TURN).round()
}

/// `x` moved by whole turns to sit nearest `near`.
fn wrap(x: Interval, near: f64) -> Interval {
    let k = turns_toward(x, near);
    if k == 0.0 { x } else { x + turns(k) }
}

/// **Every whole turn a path whose reach on a periodic axis is `reach`
/// can meet the range `a` at**: from `a`'s low end against the reach's
/// high one to its high end against the reach's low one, a turn more
/// either side. `None` where that is not finite or more than 16 turns,
/// a reach no cell reads.
fn turn_ladder((lo, hi): (f64, f64), (a0, a1): (f64, f64)) -> Option<Vec<f64>> {
    let (k0, k1) = (
        ((a0 - hi) / TURN).floor() - 1.0,
        ((a1 - lo) / TURN).ceil() + 1.0,
    );
    (k0.is_finite() && k1.is_finite() && k1 - k0 < 16.0).then(|| {
        core::iter::successors(Some(k0), |k| Some(k + 1.0))
            .take_while(|&k| k <= k1)
            .collect()
    })
}

/// Whether the enclosure `x` meets the closed range `(lo, hi)`.
fn meets_range(x: Interval, (lo, hi): (f64, f64)) -> bool {
    x.lo() <= hi && lo <= x.hi()
}

/// **Whether `[lo, hi]` spans a whole turn**, to within [`few_ulps`] of
/// the values involved — the rounding of `τ` itself.
pub(crate) fn whole_turn(lo: f64, hi: f64) -> bool {
    hi - lo >= TURN - few_ulps(TURN.max(lo.abs()).max(hi.abs()))
}

/// **How many pieces a curve's span is first cut into**, to find the
/// pieces whose image can reach a box ([`edge_piece`]) or a cell's
/// columns ([`present_curved`]): one count, which only coarsens or
/// refines the search and decides nothing.
const PIECES: usize = 64;

/// **A few ulps of `x`**, never less than a few of 1: the one slack for
/// a chart value that rounding alone can move — a line placed by whole
/// turns, a copy of an extent, a pcurve's end evaluated at a parameter.
/// Every reading that grants it is an ε concession and says so where it
/// does (the module docs list them).
pub(crate) fn few_ulps(x: f64) -> f64 {
    8.0 * f64::EPSILON * x.abs().max(1.0)
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
#[derive(Clone, Debug)]
enum Zero {
    /// None: the separator's sign is read off its value.
    None,
    /// One side.
    Side(Side),
    /// One side that runs on past the shared cell, over an edge the
    /// other face does not share ([`partial_zero`]); read only where
    /// [`sign_on`] admits it.
    Past(Side),
    /// Two adjacent sides.
    Corner(Side, Side),
    /// Only the corner point of two adjacent sides.
    Point(Side, Side),
    /// A curve across the rectangle, the face on this side's side of it
    /// ([`present_curved`]).
    Curve(Side, Box<Curved>),
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
/// holds every point of the face in the cell, and where in `R` the
/// separator vanishes ([`Zero`]): sides of `R` on the shared cell, or a
/// curve across it.
struct Presented<'a> {
    surface: &'a Surface<Interval>,
    map: Map,
    x: (f64, f64),
    y: (f64, f64),
    zero: Zero,
    /// Read through a corner ([`present_corner`]).
    corner: bool,
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

    /// The outward unit normal at the middle of the side on the shared
    /// edge, at `f64`, of a face of sense `sense` (the cusp test's
    /// reading); `None` where no one side or curve is on it.
    fn shared_normal(&self, sense: bool) -> Option<Vec3<f64>> {
        let (u, v) = match &self.zero {
            &(Zero::Side(side) | Zero::Past(side)) => {
                let b = self.bound(side);
                let (x, y) = match side.axis() {
                    Axis::U => (at(b), at(geom::mid_param(self.y.0, self.y.1))),
                    Axis::V => (at(geom::mid_param(self.x.0, self.x.1)), at(b)),
                };
                self.chart(x, y)
            }
            Zero::Curve(_, c) => c.middle(),
            _ => return None,
        };
        let j = self.surface.jet(u, v);
        let n = unit(vmid(j.du.cross(j.dv)))?;
        Some(crate::face_normal::chart_outward_normal(n, sense).vec())
    }
}

/// **Which way an iso use's image runs** along its moving coordinate,
/// from the curve's start parameter to its end: `+1` or `-1`, decided by
/// the image's ends; `None` where they do not decide it.
fn forward(u: &Use, iso: Iso) -> Option<f64> {
    let [p, q] = u.image?;
    let delta = iso.moving(q) - iso.moving(p);
    match (delta.lo() > 0.0, delta.hi() < 0.0) {
        (true, _) => Some(1.0),
        (_, true) => Some(-1.0),
        _ => None,
    }
}

/// Which side of an iso edge's line its face lies on, along the held
/// axis: the face is to the left of its boundary about the outward
/// normal, and the chart is positively oriented about `+(S_u × S_v)`.
fn face_side(u: &Use, iso: Iso, sense: bool) -> Option<f64> {
    let forward = forward(u, iso)?;
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
    let (a, b) = (iso.moving(p), iso.moving(q));
    Some(Interval::from_bounds(
        a.lo().min(b.lo()),
        a.hi().max(b.hi()),
    ))
}

/// Whether `want` lies in the union of `ranges`, each moved by whole
/// turns where the axis is periodic. Its ends are taken [`few_ulps`]
/// in, as a line placed by whole turns ([`wrap`]) is rounded outward:
/// an ε concession, since an edge that truly stops that short of the
/// side reads as covering it.
fn covered(want: (f64, f64), ranges: &[Interval], periodic: bool) -> bool {
    let want = (want.0 + few_ulps(want.0), want.1 - few_ulps(want.1));
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
    /// The cell's bounds are not finite: no turn count places it.
    Unplaced,
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
/// A cell MEETS a copy where their overlap is wider than [`few_ulps`]
/// of the values involved, the rounding of a copy moved by whole turns.
/// **That is an ε concession**: a cell overlapping a copy by less reads
/// [`Sheets::Missed`], and the face's points in the overlap — within a
/// few ulps, in the chart, of the copy's end line — go unread by this
/// pair. Likewise an extent within [`few_ulps`] of a whole turn reads
/// as wrapping, its gap unread.
///
/// Every copy the cell can meet is tried: the turn counts from the
/// cell's low end against the extent's high one to its high end against
/// the extent's low one.
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
    // **An outer loop that wraps a periodic axis bounds nothing.** The
    // face is not the inside of it: a band lies between it and a ring, a
    // polar cap between it and the pole. Its hull says nothing of the
    // face on either axis, and the face spans every value of the axis
    // it wraps, so no axis has a sheet to choose.
    let wraps = [Axis::U, Axis::V].into_iter().any(|axis| {
        let (lo, hi) = extent[axis.index()];
        periodic(surface, axis) && whole_turn(lo, hi)
    });
    if wraps {
        return Sheets::On(on);
    }
    for axis in [Axis::U, Axis::V] {
        let (lo, hi) = extent[axis.index()];
        let (clo, chi) = axis.of(cell);
        let slack = few_ulps(
            TURN.max(lo.abs())
                .max(hi.abs())
                .max(clo.abs())
                .max(chi.abs()),
        );
        let meets = |(a, b): (f64, f64)| b.min(chi) - a.max(clo) > slack;
        if !periodic(surface, axis) {
            if !meets((lo, hi)) {
                return Sheets::Missed;
            }
            on[axis.index()] = Sheet::Within(lo, hi);
            continue;
        }
        let (k0, k1) = (((clo - hi) / TURN).floor(), ((chi - lo) / TURN).ceil());
        if !(k0.is_finite() && k1.is_finite()) {
            return Sheets::Unplaced;
        }
        let copies: Vec<(f64, f64)> = core::iter::successors(Some(k0), |k| Some(k + 1.0))
            .take_while(|&k| k <= k1)
            .map(|k| (lo + k * TURN, hi + k * TURN))
            .filter(|&c| meets(c))
            .take(2)
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
///
/// A line within [`few_ulps`] of the cell's side touches it, as a line
/// placed by whole turns is rounded. **That is an ε concession**: a
/// face that truly comes back in a gap that narrow goes unread.
fn vacuous(
    b: &Boundary,
    surface: &Surface<Interval>,
    cell: ((f64, f64), (f64, f64)),
    outside: &[(Side, Interval, usize)],
) -> bool {
    outside.iter().any(|&(side, line, ei)| {
        let (lo, hi) = side.axis().of(cell);
        let touches = match side {
            Side::XLo | Side::YLo => line.lo() <= hi + few_ulps(hi),
            Side::XHi | Side::YHi => line.hi() >= lo - few_ulps(lo),
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
///   its side ([`vacuous`]), or its rectangle — read only once every
///   side and corner is proven the face's own — is a sliver beside a
///   shared line.
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
            Sheets::Unplaced => return Err(Unpresented::Structural),
            Sheets::On(s) => s,
        }
    };
    // A boundary edge whose image is no coordinate line: the curved
    // presentation, where it is the only one.
    if !piece.whole {
        let walls: Vec<usize> = piece
            .local
            .edges
            .iter()
            .copied()
            .filter(|&i| {
                b.edges
                    .get(i)
                    .is_some_and(|e| e.arc.is_some() && e.uses.len() <= 1)
            })
            .collect();
        if let Some(read) = curved_walls(piece, &walls, near_edges, near_vertices, band) {
            return read;
        }
    }
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
                for (bound, low) in [(lo, true), (hi, false)] {
                    if exactly(iso.at, bound) {
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
            // A cell past the line on its face side by no more than
            // [`few_ulps`] lies outside it: the touch's ε concession.
            if (sigma > 0.0 && line.lo() >= hi - few_ulps(hi))
                || (sigma < 0.0 && line.hi() <= lo + few_ulps(lo))
            {
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
    // A rectangle clipped to within the band of a shared line is read as
    // holding no point of the face off that line: its extent across, in
    // metres, is decided zero (`self_intersection_sliver`).
    //
    // **That is an ε concession** (the module docs list them all). Any
    // coincidence it misses lies within the strip, so within ε of the
    // shared edge's image in ℝ³. It is asymmetric with the distance
    // test: two cells that do NOT touch and come as close read
    // `Violated`, or refuse, while here, beside a shared edge, the same
    // approach reads certified.
    //
    // **It reads only a rectangle the coverage and corner tests above
    // have passed.** The strip is the face's part of the cell only once
    // every clip's side is its edges' own and every corner convex;
    // before that the rectangle may be a subset — a finger narrower than
    // the band, rising through the cell from a shared wall, clips it to
    // the finger and leaves the face's points beside the finger out —
    // and reading it vacuous drops them. The order is required for
    // soundness.
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
    let zero = match zero_of(b, &merged, near_edges, near_vertices) {
        Ok(zero) => zero,
        Err(e) => match partial_zero(b, &merged, near_edges, near_vertices) {
            Some(side) => Zero::Past(side),
            None => return Err(e),
        },
    };
    Ok(Presentation::Of(Box::new(Presented {
        surface: piece.surface,
        map: Map::Chart,
        x,
        y,
        zero,
        corner: false,
        cache: Cache::default(),
    })))
}

/// Whether the face is the quadrant two clips cut at the vertex their
/// edges share: each edge leaves the vertex into the other's face side.
fn convex(b: &Boundary, c: &Clip, d: &Clip) -> bool {
    let away = |ei: usize, v: VertexKey| -> Option<f64> {
        let e = b.edges.get(ei)?;
        let u = e.uses.first()?;
        let forward = forward(u, u.iso?)?;
        // Away from `v` along the curve: forward from its start.
        if e.ends[0] == v {
            Some(forward)
        } else if e.ends[1] == v {
            Some(-forward)
        } else {
            None
        }
    };
    // EVERY vertex the two clips' edges share is a corner of the face:
    // two edges that meet twice (round a periodic chart) make two, and
    // each must be convex.
    let ends = |c: &Clip| -> BTreeSet<VertexKey> {
        c.edges
            .iter()
            .filter_map(|&i| b.edges.get(i))
            .flat_map(|e| e.ends)
            .collect()
    };
    let corners: Vec<VertexKey> = ends(c).intersection(&ends(d)).copied().collect();
    // No corner: the two lines cross where neither edge reaches, which
    // the span check has already refused.
    let ok = |c: &Clip, d: &Clip, v: VertexKey| -> bool {
        // Every edge of `d` at `v` leaves it along `c`'s held axis, toward
        // `c`'s face side; one whose direction is not decided fails.
        let at_v: Vec<usize> = d
            .edges
            .iter()
            .copied()
            .filter(|&j| b.edges.get(j).is_some_and(|e| e.ends.contains(&v)))
            .collect();
        !at_v.is_empty()
            && at_v
                .iter()
                .all(|&j| away(j, v).is_some_and(|s| s * c.side.into() > 0.0))
    };
    corners.iter().all(|&v| ok(c, d, v) && ok(d, c, v))
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
            // The ONE corner of two clips the vertex is: a second pair
            // holding it too would leave the point ambiguous.
            let mut corners = Vec::new();
            for (i, c) in clips.iter().enumerate() {
                for d in clips.iter().skip(i + 1) {
                    if c.side.axis() != d.side.axis()
                        && ends_of(c).contains(v)
                        && ends_of(d).contains(v)
                    {
                        corners.push(Zero::Point(c.side, d.side));
                    }
                }
            }
            match corners.as_slice() {
                [one] => Ok(one.clone()),
                _ => Err(Unpresented::Local),
            }
        }
        _ => Err(Unpresented::Local),
    }
}

/// **A zero side that runs on past the shared cell**: the one clip
/// holding a shared edge, which also holds an edge the other face does
/// not share — a line of the face's chart through a vertex where the
/// shared edge gives way to the next, as a rim circle split into arcs
/// is. Every shared vertex is an end of its edges.
///
/// The separator must vanish along the whole side (it is read, so the
/// graph separator, which is not, answers nothing on it), and only the
/// other face's zero set is on the shared cell. A coincidence between
/// a point of this side off the shared cell and the other face's zero
/// set is then two points of THIS face with one image — one on the side,
/// one the same point of the shared edge — which its injectivity by kind
/// rules out. So [`sign_on`] reads it only on an analytic carrier,
/// against a presentation whose zero set is on the shared cell, and not
/// in the carrier arm, where the separating face's every point is on
/// its zero set.
///
/// **That leans on another reading of the same run**: the face's
/// injectivity by kind is [`by_kind`]'s, decided for the face's own
/// pairs. Its premise — the face's domain covers its carrier at most
/// once — is the face validation's, and is not yet checked
/// (`work/cleartail/an-analytic-face-may-cover-its-carrier-more-than-once.md`).
fn partial_zero(
    b: &Boundary,
    clips: &[Clip],
    near_edges: &[usize],
    near_vertices: &[VertexKey],
) -> Option<Side> {
    let touches = |c: &Clip| c.edges.iter().any(|e| near_edges.contains(e));
    let mut held = clips.iter().filter(|c| c.collapsed.is_none() && touches(c));
    let c = held.next()?;
    if held.next().is_some() || clips.iter().any(|d| d.collapsed.is_some()) {
        return None;
    }
    let ends: BTreeSet<VertexKey> = c
        .edges
        .iter()
        .filter_map(|&i| b.edges.get(i))
        .flat_map(|e| e.ends)
        .collect();
    near_vertices
        .iter()
        .all(|v| ends.contains(v))
        .then_some(c.side)
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
    if let Some(read) = curved_walls(piece, &edges, near_edges, near_vertices, band) {
        return read;
    }
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
            corner: false,
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
                corner: false,
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
                corner: false,
                cache: Cache::default(),
            })))
        }
        _ => Err(Unpresented::Local),
    }
}

// ------------------------------------------------ the curved presentation

/// The funnel site of a curved presentation's columns: the chart
/// curve's advance along the column axis over one of its pieces, times
/// the chart's speed along that axis (an upper bound, `rect_jet`'s),
/// metres. Only a definite sign, one sign over every piece, reads the
/// curve as a graph over that axis.
pub(crate) const COLUMN: &str = "self_intersection_column";

/// How many halvings place a curved presentation's run on its columns'
/// two ends.
const NARROW: u32 = 48;

impl Harmonic {
    /// The image on `axis` over the parameters `t`, enclosed.
    fn on(&self, axis: Axis, t: Interval) -> Interval {
        let i = axis.index();
        let (s, c) = t.sin_cos();
        self.p0[i] + self.pa[i] * c + self.pb[i] * s + self.pl[i] * t
    }

    /// Its derivative on `axis` over `t`, enclosed.
    fn rate(&self, axis: Axis, t: Interval) -> Interval {
        let i = axis.index();
        let (s, c) = t.sin_cos();
        self.pb[i] * c - self.pa[i] * s + self.pl[i]
    }

    /// The chart point at `t`, or the image over the parameters `t`,
    /// enclosed.
    pub(crate) fn at(&self, t: Interval) -> (Interval, Interval) {
        (self.on(Axis::U, t), self.on(Axis::V, t))
    }
}

/// **A curve `c + a·cos t + b·sin t` in space**: a face's boundary image
/// over a harmonic chart curve, where the carrier makes it one.
#[derive(Clone, Copy, Debug)]
struct Trig {
    c: Point3<Interval>,
    a: Vec3<Interval>,
    b: Vec3<Interval>,
}

/// `k0 + k1·cos t + k2·sin t + k3·cos 2t + k4·sin 2t` over `t`.
fn trig(k: [Interval; 5], t: Interval) -> Interval {
    let (s, c) = t.sin_cos();
    let (s2, c2) = (t * at(2.0)).sin_cos();
    k[0] + k[1] * c + k[2] * s + k[3] * c2 + k[4] * s2
}

/// `(c + a·cos t + b·sin t)²` as [`trig`]'s coefficients, each formed
/// before any `t` is read: where the curve keeps a constant value the
/// oscillating terms are rounding.
fn square(c: Interval, a: Interval, b: Interval) -> [Interval; 5] {
    let half = at(0.5);
    let (aa, bb) = (a.powi(2), b.powi(2));
    [
        c.powi(2) + (aa + bb) * half,
        c * a * at(2.0),
        c * b * at(2.0),
        (aa - bb) * half,
        a * b,
    ]
}

/// `|w0 + w1·cos t + w2·sin t|²`: [`square`] summed over the axes.
fn square3(w0: Vec3<Interval>, w1: Vec3<Interval>, w2: Vec3<Interval>) -> [Interval; 5] {
    let [x, y, z] = [
        square(w0.x, w1.x, w2.x),
        square(w0.y, w1.y, w2.y),
        square(w0.z, w1.z, w2.z),
    ];
    core::array::from_fn(|i| x[i] + y[i] + z[i])
}

impl Trig {
    /// **The image of a harmonic chart curve on `surface`**, where it is
    /// a curve of this form: on a plane, any image with no linear term;
    /// on a cylinder, an image winding the azimuth once (`u = φ ± t`)
    /// whose height has no linear term. `None` elsewhere.
    fn of(surface: &Surface<Interval>, h: &Harmonic) -> Option<Self> {
        match *surface {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => {
                if !(exactly(h.pl[0], 0.0) && exactly(h.pl[1], 0.0)) {
                    return None;
                }
                let v_ref = normal.cross(u_ref);
                let map = |p: [Interval; 2]| u_ref * p[0] + v_ref * p[1];
                Some(Self {
                    c: origin + map(h.p0),
                    a: map(h.pa),
                    b: map(h.pb),
                })
            }
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => {
                let w = [1.0, -1.0].into_iter().find(|&w| exactly(h.pl[0], w))?;
                if !(exactly(h.pa[0], 0.0) && exactly(h.pb[0], 0.0) && exactly(h.pl[1], 0.0)) {
                    return None;
                }
                let v_ref = axis.cross(u_ref);
                let (s, c) = h.p0[0].sin_cos();
                // cos(φ + w·t) = cos φ·cos t − w·sin φ·sin t, and
                // sin(φ + w·t) = sin φ·cos t + w·cos φ·sin t.
                let at_cos = (u_ref * c + v_ref * s) * radius;
                let at_sin = (v_ref * c - u_ref * s) * (radius * at(w));
                Some(Self {
                    c: origin + axis * h.p0[1],
                    a: axis * h.pa[1] + at_cos,
                    b: axis * h.pb[1] + at_sin,
                })
            }
            _ => None,
        }
    }

    /// **`ψ` along the curve over `t`**, bounded: for a separator that
    /// is a signed distance, its value is formed as [`trig`]'s
    /// coefficients (a squared distance's where the separator is not
    /// linear, divided by a lower bound of the factor it carries), so a
    /// curve lying on the separator's zero set reads rounding, at any
    /// width of `t`. `None` for the graph separator, which vanishes on
    /// its edge's own carrier, not on a face's image of it.
    fn psi(&self, psi: &Psi, t: Interval) -> Option<Interval> {
        let zero = at(0.0);
        let linear = |n: Vec3<Interval>, q: Point3<Interval>| {
            trig(
                [n.dot(self.c - q), n.dot(self.a), n.dot(self.b), zero, zero],
                t,
            )
        };
        let radial = |o: Point3<Interval>, axis: Vec3<Interval>| {
            let perp = |w: Vec3<Interval>| w - axis * w.dot(axis);
            (perp(self.c - o), perp(self.a), perp(self.b))
        };
        // `|ψ| ≤ |Q| / d` for `Q = ψ·D`, `D ≥ d > 0`.
        let over = |q: Interval, d: Interval| {
            (d.lo() > 0.0).then(|| {
                let m = q.abs() / d;
                Interval::from_bounds(-m.hi(), m.hi())
            })
        };
        match *psi {
            Psi::Linear { n, q } => Some(linear(n, q)),
            Psi::Carrier(Implicit::Plane { o, n }) => Some(linear(n, o)),
            Psi::Carrier(Implicit::Cylinder { o, axis, r }) => {
                let (w0, w1, w2) = radial(o, axis);
                let mut k = square3(w0, w1, w2);
                k[0] = k[0] - r.powi(2);
                // Q = ρ² − r² = ψ·(ρ + r), and ρ + r ≥ r.
                over(trig(k, t), r)
            }
            Psi::Carrier(Implicit::Sphere { c, r }) => {
                let mut k = square3(self.c - c, self.a, self.b);
                k[0] = k[0] - r.powi(2);
                over(trig(k, t), r)
            }
            Psi::Carrier(Implicit::Cone {
                apex,
                axis,
                cos,
                sin,
                nappe,
            }) => {
                let (w0, w1, w2) = radial(apex, axis);
                let h = |w: Vec3<Interval>| w.dot(axis);
                let (h0, h1, h2) = (h(self.c - apex), h(self.a), h(self.b));
                let (cc, ss) = (cos.powi(2), sin.powi(2));
                let (rho, height) = (square3(w0, w1, w2), square(h0, h1, h2));
                let k: [Interval; 5] = core::array::from_fn(|i| rho[i] * cc - height[i] * ss);
                // Q = ρ²cos² − h²sin² = ψ·(ρ cos α + ν h sin α), and the
                // second factor is at least ν h sin α where that is
                // positive.
                let lever = trig([h0, h1, h2, zero, zero], t) * nappe * sin;
                over(trig(k, t), lever)
            }
            Psi::Graph { .. } => None,
        }
    }
}

/// **A presentation's curved side**: the face's image of the shared
/// edge's chart curve over the parameters `t` its columns hold, and the
/// edge's own carrier there where it is a conic.
#[derive(Clone, Copy, Debug)]
struct Curved {
    chart: Harmonic,
    image: Trig,
    edge: Option<Trig>,
    t: (f64, f64),
}

impl Curved {
    /// `ψ` along the side, bounded ([`Trig::psi`]). The graph separator
    /// vanishes on its edge's carrier over its piece, so along the side
    /// it is within its Lipschitz bound times the side's distance from
    /// the carrier, read where the side's parameters lie in that piece.
    fn psi(&self, psi: &Psi) -> Option<Interval> {
        let t = iv(self.t);
        if let Psi::Graph {
            m,
            t: axis,
            slope,
            piece,
        } = *psi
        {
            let edge = self.edge?;
            // A conic is periodic: the side's parameters may lie in the
            // piece a whole period on, where the piece runs past the
            // carrier's seam. Past the piece `γ` runs on at slopes in
            // `slope`, so there `ψ` on the carrier is at most its
            // Lipschitz bound times the carrier's speed (`|a| + |b|`
            // bounds it) times the overhang.
            let overhang = [-1.0, 0.0, 1.0]
                .iter()
                .map(|&k| {
                    let shifted = t + turns(k);
                    (piece.0 - shifted.lo())
                        .max(shifted.hi() - piece.1)
                        .max(0.0)
                })
                .fold(f64::INFINITY, f64::min);
            if !overhang.is_finite() {
                return None;
            }
            let d = trig(
                square3(
                    self.image.c - edge.c,
                    self.image.a - edge.a,
                    self.image.b - edge.b,
                ),
                t,
            );
            let lip = m.norm() + slope.abs() * axis.norm();
            let speed = edge.a.norm() + edge.b.norm();
            let bound = (lip * (at(d.hi().max(0.0)).sqrt() + speed * at(overhang))).hi();
            return Some(Interval::from_bounds(-bound, bound));
        }
        self.image.psi(psi, t)
    }

    /// The chart point at the middle of the side's parameters.
    fn middle(&self) -> (Interval, Interval) {
        self.chart.at(at(geom::mid_param(self.t.0, self.t.1)))
    }
}

/// A conic carrier as a [`Trig`]: `center + u_ref·a·cos t + v_ref·b·sin t`.
fn conic(c: &Curve3<Interval>) -> Option<Trig> {
    let (center, axis, a, b, x) = match *c {
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => (center, axis, radius, radius, u_ref),
        Curve3::Ellipse {
            center,
            axis,
            semi_u,
            semi_v,
            u_ref,
        } => (center, axis, semi_u, semi_v, u_ref),
        _ => return None,
    };
    Some(Trig {
        c: center,
        a: x * a,
        b: axis.cross(x) * b,
    })
}

/// **A face's chart curve on an edge it is not presented by lines
/// on**: a plane's image of a conic carrier, through its affine chart;
/// any other face's harmonic row.
fn chart_curve(surface: &Surface<Interval>, rim: &Rim, u: &Use) -> Option<Harmonic> {
    let Surface::Plane {
        origin,
        normal,
        u_ref,
    } = *surface
    else {
        return u.harmonic;
    };
    let edge = conic(&rim.arc.as_ref()?.0)?;
    let v_ref = normal.cross(u_ref);
    let chart = |w: Vec3<Interval>| [w.dot(u_ref), w.dot(v_ref)];
    let zero = at(0.0);
    Some(Harmonic {
        p0: chart(edge.c - origin),
        pa: chart(edge.a),
        pb: chart(edge.b),
        pl: [zero, zero],
    })
}

/// Whether an edge's chart curve is one [`present_curved`] reads.
fn reads_curve(surface: &Surface<Interval>, rim: &Rim) -> bool {
    matches!(rim.uses.as_slice(), [u] if chart_curve(surface, rim, u)
        .is_some_and(|h| Trig::of(surface, &h).is_some()))
}

/// Whether a face reads an edge as curved: on a plane, any carrier but
/// a line; on a chart, an image that is no coordinate line.
fn is_curved(surface: &Surface<Interval>, rim: &Rim) -> bool {
    match surface {
        Surface::Plane { .. } => !matches!(rim.arc, Some((Curve3::Line { .. }, _))),
        _ => rim.uses.iter().any(|u| u.iso.is_none()),
    }
}

/// **The curved dispatch** of [`present_chart`] and [`present_plane`],
/// over the cell's boundary edges `walls`: where one is curved, the
/// curved presentation where it is the only one, else why the cell is
/// not presented ([`beside_curve`]); `None` where none is curved.
fn curved_walls<'a>(
    piece: &Piece<'a>,
    walls: &[usize],
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> Option<Result<Presentation<'a>, Unpresented>> {
    let b = piece.boundary;
    let curved = |i: &usize| b.edges.get(*i).is_some_and(|e| is_curved(piece.surface, e));
    let &first = walls.iter().find(|i| curved(i))?;
    Some(match walls {
        [e] => present_curved(piece, *e, None, near_edges, near_vertices, band),
        _ => corner_of(b, walls, near_edges)
            .and_then(|c| present_corner(piece, c, near_edges, near_vertices, band))
            .unwrap_or_else(|| Err(beside_curve(piece, walls, first, near_edges, near_vertices))),
    })
}

/// **Why a cell is not presented beside a curved edge that is not its
/// one boundary edge**: a corner where a curved edge meets another at
/// a vertex of the shared cell, which no corner reading reads
/// ([`reading`]), is read by no presentation, at any depth; a curved
/// edge no presentation reads, when shared, is not either; otherwise a
/// smaller cell may hold the curve alone, or the corner alone.
fn beside_curve(
    piece: &Piece<'_>,
    walls: &[usize],
    first: usize,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
) -> Unpresented {
    let b = piece.boundary;
    let ends = |i: usize| b.edges.get(i).map_or(Vec::new(), |e| e.ends.to_vec());
    let curved: Vec<usize> = walls
        .iter()
        .copied()
        .filter(|&i| b.edges.get(i).is_some_and(|e| is_curved(piece.surface, e)))
        .collect();
    let on_shared = |v: &VertexKey| {
        near_vertices.contains(v) || near_edges.iter().any(|&i| ends(i).contains(v))
    };
    // A corner the corner presentation reads may be read smaller, alone.
    let unread_corner = piece.local.vertices.iter().any(|v| {
        let at_v: Vec<usize> = walls
            .iter()
            .copied()
            .filter(|&w| ends(w).contains(v))
            .collect();
        on_shared(v)
            && curved.iter().any(|&c| at_v.contains(&c) && at_v.len() > 1)
            && corner_of(b, &at_v, near_edges).is_none_or(|c| reading(piece, c).is_none())
    });
    if unread_corner {
        return Unpresented::Structural;
    }
    match curved.iter().find(|&&i| {
        b.edges
            .get(i)
            .is_none_or(|e| !reads_curve(piece.surface, e))
    }) {
        Some(&i) => unread(i, near_edges),
        None if curved.is_empty() => unread(first, near_edges),
        None => Unpresented::Local,
    }
}

/// **Present `piece`'s face by the region on one side of the curved
/// edge `ei`**, the one boundary edge near the cell: the face's part of
/// the cell lies in `{(a, b) : a ∈ A, b on the face's side of g(a)}`,
/// `A` the cell's range on a chart axis (its columns), `g` the edge's
/// chart curve read as a graph over that axis.
///
/// **Why that region holds every point of the face in the cell.** Let
/// `R` be the columns times the hull, on the other axis, of the cell
/// and of the curve over them. Read on the cover of the chart (every
/// lift of every boundary curve by whole turns), so no sheet is chosen:
///
/// 1. Every lift of the curve whose column coordinate can meet `A` is
///    cut into runs of consecutive pieces ([`runs`]); exactly one is in
///    reach of the cell, its column coordinate strictly monotone over
///    it and running past both ends of `A` ([`monotone_run`]). So each
///    column holds exactly one point of that run, `(a, g(a))`.
/// 2. No other run, nor any other copy of this one across the other
///    axis, meets `R` ([`apart`]), and every other edge, every vertex
///    off the curve too, is decided clear of `R`'s ball ([`alone`]), so
///    the curve is the only boundary in `R`.
/// 3. A point `p` of the cell on the wrong side of `g(a)` in its column
///    joins `(a, g(a))` by a segment in `R` that meets no boundary until
///    it reaches the curve, crossing it — the curve is a graph, so
///    transverse to the column — from the side the face is not on
///    ([`left_is_above`]). So the segment, and `p`, lie outside the
///    face.
///
/// `R` is presented; a separator vanishing along the curve and strictly
/// monotone across it over `R` then takes one sign on the face's part of
/// the cell off the curve ([`sign_on`]).
///
/// A closed edge whose chart curve is periodic is read through its
/// join by the curve's closed form continued past its span. **That is
/// an ε concession**: a span stored as the rounded `τ` stops short of
/// the period by the rounding of `τ` itself, a gap of ulps in the
/// curve's parameter, which reads as the edge.
///
/// Through a `corner` the curve is continued past the vertex by its
/// closed form in the same way, `R` is widened to hold the vertex, and
/// the corner's other edge is admitted in reach where it leaves the
/// vertex onto the face's side ([`present_corner`] argues the region).
fn present_curved<'a>(
    piece: &Piece<'a>,
    ei: usize,
    corner: Option<Corner>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> Result<Presentation<'a>, Unpresented> {
    let e = piece.boundary.edges.get(ei).ok_or(Unpresented::Local)?;
    let [u] = e.uses.as_slice() else {
        return Err(unread(ei, near_edges));
    };
    let Some(chart) = chart_curve(piece.surface, e, u) else {
        return Err(unread(ei, near_edges));
    };
    let Some(image) = Trig::of(piece.surface, &chart) else {
        return Err(unread(ei, near_edges));
    };
    let Some((carrier, span)) = e.arc.as_ref() else {
        return Err(Unpresented::Local);
    };
    let (t0, t1) = (span.0.lo(), span.1.hi());
    if !(t1 - t0).is_finite() || t1 <= t0 {
        return Err(Unpresented::Structural);
    }
    // The curve continues through the join where the edge is closed, its
    // span a whole period, and the period moves the image by whole
    // turns: none on an axis that is not periodic.
    let lattice = [Axis::U, Axis::V].into_iter().all(|axis| {
        let l = chart.pl[axis.index()];
        exactly(l, 0.0) || (periodic(piece.surface, axis) && (exactly(l, 1.0) || exactly(l, -1.0)))
    });
    let continued = e.ends[0] == e.ends[1] && whole_turn(t0, t1) && lattice;
    let shared = near_edges.contains(&ei);
    if !shared && !near_vertices.is_empty() {
        return Err(Unpresented::Local);
    }
    // Through a corner the curve is read past its vertex by its closed
    // form, over a span still short of a turn: its own pieces and as
    // much again, or half what the turn leaves, past that end.
    let through = match corner {
        Some(c) => {
            if continued || e.ends[0] == e.ends[1] {
                return Err(Unpresented::Local);
            }
            let ext = (t1 - t0).min((TURN - (t1 - t0)) * 0.5);
            if ext.is_nan() || ext <= 0.0 {
                return Err(Unpresented::Local);
            }
            let start = e.ends[0] == c.vertex;
            Some(Through {
                corner: c,
                t: if start { t0 } else { t1 },
                span: if start {
                    (t0 - ext, t1)
                } else {
                    (t0, t1 + ext)
                },
            })
        }
        None => None,
    };
    for column in [Axis::U, Axis::V] {
        let span = through.as_ref().map_or((t0, t1), |th| th.span);
        let pieces = Pieces::of(&chart, column, span, continued);
        let Graph { x, y, above, t } =
            match graph(piece, ei, u.plus, &pieces, through.as_ref(), band) {
                Ok(g) => g,
                Err(Unpresented::Local) => continue,
                Err(e) => return Err(e),
            };
        let zero = if shared {
            Zero::Curve(
                Side::of(column.other(), above),
                Box::new(Curved {
                    chart,
                    image,
                    edge: conic(carrier),
                    t,
                }),
            )
        } else {
            Zero::None
        };
        return Ok(Presentation::Of(Box::new(Presented {
            surface: piece.surface,
            map: Map::Chart,
            x,
            y,
            zero,
            corner: through.is_some(),
            cache: Cache::default(),
        })));
    }
    Err(Unpresented::Local)
}

// -------------------------------------------------- the curved corner

/// The funnel site of a curved corner's convexity: how fast the
/// corner's other edge leaves the edge read through the vertex toward
/// that edge's face side — off the line, or off the curve's graph —
/// times the extent it is read over and the chart's speed (an upper
/// bound, `rect_jet`'s), metres. Only a definite `Positive` reads the
/// corner convex; a definite `Negative` is a reflex corner, which this
/// presentation never reads.
pub(crate) const CORNER_DEPARTS: &str = "self_intersection_corner_departs";

/// The funnel site of a corner's vertex: the chart distance between the
/// other edge's end and the read edge's point at the vertex, each axis
/// times the chart's speed along it over the rectangle (an upper bound,
/// `rect_jet`'s), metres. Only a definite `Zero` reads them as one
/// point — one of the presentation's ε concessions (the module docs
/// list them).
pub(crate) const CORNER_VERTEX: &str = "self_intersection_corner_vertex";

/// **The two edges' points at a corner's vertex are one**, decided at
/// [`CORNER_VERTEX`]: `a` and `b` chart points `[u, v]` on one copy,
/// `speed` the chart's speeds along `[u, v]` over the rectangle.
///
/// **That is an ε concession.** Each edge's step-3 reading takes the
/// other edge's height over the read one to be zero at the vertex. Two
/// edges of a body built to a tolerance meet within it, not exactly, so
/// the other edge may start that far on the read edge's other side, and
/// stand there over a stretch before its height, strictly growing, turns
/// positive. Every face point the presentation drops there lies between
/// the two edges, within that gap of the other edge — within the band,
/// in metres, of the face's own boundary. A coincidence farther from the
/// boundary than the band cannot pass through it.
fn on_vertex(a: [Interval; 2], b: [Interval; 2], speed: [f64; 2], band: Band) -> bool {
    let off = |i: usize| ((a[i] - b[i]) * at(speed[i])).powi(2);
    let gap = (off(0) + off(1)).sqrt();
    matches!(decide(CORNER_VERTEX, Margin::of(gap), band), Ok(Sign::Zero))
}

/// How many halvings [`leaves_line`] gives a piece of the other edge it
/// cannot yet tell.
const LEAVES: u32 = 24;

/// **A corner of a cell's boundary**: its two walls meeting at `vertex`,
/// `read` the one on the shared cell, read through the vertex, and
/// `other`, which leaves it.
#[derive(Clone, Copy, Debug)]
struct Corner {
    vertex: VertexKey,
    read: usize,
    other: usize,
}

/// A curve read through a corner: the corner, the curve's parameter at
/// the vertex, and the span continued past it.
struct Through {
    corner: Corner,
    t: f64,
    span: (f64, f64),
}

/// **The corner a cell's two walls make**: two open edges with exactly
/// one end in common, one of them on the shared cell. `None` for
/// anything else — two edges meeting at both ends among them. Neither
/// restriction is needed for soundness: a lens's other end in reach is a
/// vertex its reading refuses, and a shared `other` would only be left
/// off the zero, which costs a certificate, never a wrong one. They keep
/// such cells to the readings they had.
fn corner_of(b: &Boundary, walls: &[usize], near_edges: &[usize]) -> Option<Corner> {
    let &[e, f] = walls else {
        return None;
    };
    let (re, rf) = (b.edges.get(e)?, b.edges.get(f)?);
    if re.ends[0] == re.ends[1] || rf.ends[0] == rf.ends[1] {
        return None;
    }
    let common: Vec<VertexKey> = re
        .ends
        .iter()
        .copied()
        .filter(|v| rf.ends.contains(v))
        .collect();
    let &[vertex] = common.as_slice() else {
        return None;
    };
    match (near_edges.contains(&e), near_edges.contains(&f)) {
        (true, false) => Some(Corner {
            vertex,
            read: e,
            other: f,
        }),
        (false, true) => Some(Corner {
            vertex,
            read: f,
            other: e,
        }),
        _ => None,
    }
}

/// **Present a cell at a corner** ([`corner_of`]) by its shared edge
/// alone, read through the vertex: a curve by its closed form continued
/// ([`present_curved`]), a coordinate line of a chart by the line
/// ([`present_line_corner`]). `None` where neither reads it.
///
/// **Why the face's part of the cell lies on the read edge's face
/// side**, for `R` the presentation's rectangle, which holds the vertex
/// `V`, and `D` the points of `R` across the read edge `E` — its line,
/// or its curve's graph, continued past `V` — from the face:
///
/// 1. Nothing in reach of `R` bounds the face but `E` and the other
///    edge `O` ([`alone`], or [`present_line_corner`]'s own reading),
///    and `O` leaves `V` strictly into `E`'s face side and stays there
///    within `R` ([`leaves_graph`], [`leaves_line`]). So `D`, and the
///    continuation of `E` past `V`, hold no point of the boundary.
/// 2. `D` is connected: across a line it is a convex set; across a
///    graph, every column of it reaches the side of `R` the curve is
///    not on.
/// 3. Near `V` the two edges cut a small disc into two parts: the face
///    is the one between them on `E`'s face side, which `D` does not
///    meet. `D` meets the other part, so `D` — and the continuation,
///    which `D` meets on every side — lies outside the face.
///
/// So every point of the face in the cell is in `R` off `D`, and its
/// points on `E`'s line or graph lie on `E` itself, the shared cell:
/// `R` is presented with `E`'s side or curve as its zero, as the edge's
/// own presentations are. A corner `O` leaves into `E`'s other side
/// (reflex) is read by neither, at any depth: `Structural`.
fn present_corner<'a>(
    piece: &Piece<'a>,
    c: Corner,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> Option<Result<Presentation<'a>, Unpresented>> {
    Some(match reading(piece, c)? {
        Reading::Curve => present_curved(piece, c.read, Some(c), near_edges, near_vertices, band),
        Reading::Line => present_line_corner(piece, c, band),
    })
}

/// How [`present_corner`] reads a corner.
enum Reading {
    /// The shared edge is a curve [`present_curved`] reads, the other
    /// edge straight in the chart.
    Curve,
    /// The shared edge is a coordinate line of a cylinder's chart, the
    /// other edge a harmonic chart curve.
    Line,
}

/// **Which reading a corner has, if any**: one fixed by the two edges'
/// kinds, so a corner with none has none at any depth.
fn reading(piece: &Piece<'_>, c: Corner) -> Option<Reading> {
    let b = piece.boundary;
    let (e, o) = (b.edges.get(c.read)?, b.edges.get(c.other)?);
    let (s, curved) = (piece.surface, |r: &Rim| is_curved(piece.surface, r));
    if curved(e) {
        (reads_curve(s, e) && straight(s, o, c.vertex).is_some()).then_some(Reading::Curve)
    } else {
        // The argument is made on the cylinder's chart, which never
        // collapses. A sphere's images are its parallels and meridians,
        // coordinate lines, so none reaches this; a cone's apex collapses a
        // line of its chart, and is not read past here, conservatively: an
        // apex in reach is a vertex the reading refuses. A torus is not
        // read until its curves, which turn on both axes, are argued.
        let chart = matches!(s, Surface::Cylinder { .. }) && !piece.whole;
        let line = matches!(e.uses.as_slice(), [u] if u.iso.is_some_and(|i| !i.full));
        let harmonic = matches!(o.uses.as_slice(), [u] if u.harmonic.is_some());
        (chart && line && curved(o) && harmonic).then_some(Reading::Line)
    }
}

/// **An edge straight in the face's chart, leaving `v`**: its chart
/// direction away from `v`, and its chart points at `v` and at its
/// other end — a chart's coordinate line, or a plane's line edge.
fn straight(
    surface: &Surface<Interval>,
    rim: &Rim,
    v: VertexKey,
) -> Option<([Interval; 2], [Point2<Interval>; 2])> {
    let from_start = rim.ends[0] == v;
    if !from_start && rim.ends[1] != v {
        return None;
    }
    let (p, q, d) = if let Surface::Plane {
        origin,
        normal,
        u_ref,
    } = *surface
    {
        let Some((carrier @ Curve3::Line { .. }, (t0, t1))) = &rim.arc else {
            return None;
        };
        let v_ref = normal.cross(u_ref);
        let chart = |p: Point3<Interval>| {
            let d = p - origin;
            Point2::new(d.dot(u_ref), d.dot(v_ref))
        };
        let (p, q) = (chart(carrier.eval(*t0)), chart(carrier.eval(*t1)));
        (p, q, [q.x - p.x, q.y - p.y])
    } else {
        let [u] = rim.uses.as_slice() else {
            return None;
        };
        let iso = u.iso?;
        if iso.full {
            return None;
        }
        let [p, q] = u.image?;
        let s = at(forward(u, iso)?);
        let zero = at(0.0);
        (
            p,
            q,
            match iso.held {
                Axis::U => [zero, s],
                Axis::V => [s, zero],
            },
        )
    };
    Some(if from_start {
        (d, [p, q])
    } else {
        (d.map(|x| -x), [q, p])
    })
}

/// The whole turns, per chart axis, that move `x` nearest `to`: zero on
/// an axis that is not periodic.
fn turns_to(surface: &Surface<Interval>, x: [Interval; 2], to: [Interval; 2]) -> [f64; 2] {
    core::array::from_fn(|i| {
        let axis = if i == 0 { Axis::U } else { Axis::V };
        if periodic(surface, axis) {
            turns_toward(x[i], mid(to[i]))
        } else {
            0.0
        }
    })
}

/// Whether the chart box `b`, `[u, v]`, meets the closed rectangle `r`.
fn meets_rect(b: [Interval; 2], r: ((f64, f64), (f64, f64))) -> bool {
    meets_range(b[0], r.0) && meets_range(b[1], r.1)
}

/// **The copies by whole turns of a chart path that can meet `r`**,
/// `[u, v]` turn counts, the path's own reach being `reach`: on each
/// periodic axis every count [`turn_ladder`] gives, as [`runs`] tries
/// them; only `0` on an axis that is not periodic.
fn copies(
    surface: &Surface<Interval>,
    reach: [(f64, f64); 2],
    r: ((f64, f64), (f64, f64)),
) -> Option<Vec<[f64; 2]>> {
    let counts = |axis: Axis| -> Option<Vec<f64>> {
        if !periodic(surface, axis) {
            return Some(vec![0.0]);
        }
        turn_ladder(reach[axis.index()], axis.of(r))
    };
    let (us, vs) = (counts(Axis::U)?, counts(Axis::V)?);
    Some(
        us.iter()
            .flat_map(|&a| vs.iter().map(move |&b| [a, b]))
            .collect(),
    )
}

/// A chart point `(u, v)` as `[u, v]`.
fn pair((u, v): (Interval, Interval)) -> [Interval; 2] {
    [u, v]
}

/// The hull of chart boxes `[u, v]`, per axis.
fn reach_of(boxes: impl Iterator<Item = [Interval; 2]>) -> [(f64, f64); 2] {
    boxes.fold([(f64::INFINITY, f64::NEG_INFINITY); 2], |acc, b| {
        core::array::from_fn(|i| (acc[i].0.min(b[i].lo()), acc[i].1.max(b[i].hi())))
    })
}

/// **The corner's other edge leaves the curve read through it toward
/// the face's side** ([`present_corner`]'s step 1, for a curve): it is
/// straight in the chart, every copy of it but the one through the
/// vertex misses `R`, and over the parameters `ts` the columns hold, the
/// curve's chart tangent `τ` and the edge's direction `d` make
/// `(d_b·τ_a − d_a·τ_b)·sign τ_a` one strict sign, the face's side's.
/// That is the derivative of the edge's height over the graph along it,
/// which is zero at the vertex, so the edge stands strictly on the
/// face's side wherever it is in `R`. `None` where this is not decided;
/// a reflex corner, decided, is `Structural`.
#[allow(clippy::too_many_arguments)]
fn leaves_graph(
    piece: &Piece<'_>,
    th: &Through,
    p: &Pieces<'_>,
    ts: Interval,
    rising: bool,
    above: bool,
    vertex: [Interval; 2],
    r: ((f64, f64), (f64, f64)),
    band: Band,
) -> Option<Result<(), Unpresented>> {
    let surface = piece.surface;
    let o = piece.boundary.edges.get(th.corner.other)?;
    let (d, [near, far]) = straight(surface, o, th.corner.vertex)?;
    let k = turns_to(surface, [near.x, near.y], vertex);
    let reach = reach_of([[near.x, near.y], [far.x, far.y]].into_iter());
    // Another copy meets `R` only where the edge, a segment from the
    // vertex, comes round by a turn to a range of `R` that holds the
    // vertex's: its far end then lies in `R`, a vertex [`alone`] refuses.
    // Defence for the reading; not load-bearing on it.
    for c in copies(surface, reach, r)? {
        if c == k {
            continue;
        }
        let shift = |x: Interval, i: usize| x + turns(c[i]);
        let segment = [
            hull(shift(near.x, 0), shift(far.x, 0)),
            hull(shift(near.y, 1), shift(far.y, 1)),
        ];
        if meets_rect(segment, r) {
            return None;
        }
    }
    let (ju, jv, _) = rect_jet(surface, r.0, r.1);
    let speed = [ju.norm().hi(), jv.norm().hi()];
    let start = [near.x + turns(k[0]), near.y + turns(k[1])];
    if !on_vertex(start, vertex, speed, band) {
        return None;
    }
    let (a, b) = (p.column, p.column.other());
    let (tau_a, tau_b) = (p.chart.rate(a, ts), p.chart.rate(b, ts));
    let (da, db) = (a.of((d[0], d[1])), b.of((d[0], d[1])));
    let length = mid((d[0].powi(2) + d[1].powi(2)).sqrt());
    let lever = speed[0].max(speed[1]);
    let scale = (ts.hi() - ts.lo()) * lever / length;
    if !(scale.is_finite() && scale > 0.0) {
        return None;
    }
    let side = if rising == above { 1.0 } else { -1.0 };
    let rate = (db * tau_a - da * tau_b) * at(side * scale);
    match decide(CORNER_DEPARTS, Margin::of(rate), band) {
        Ok(Sign::Positive) => Some(Ok(())),
        Ok(Sign::Negative) => Some(Err(Unpresented::Structural)),
        _ => None,
    }
}

/// **Present a cell at a corner by its coordinate line** ([`present_corner`]):
/// `R` the cell and the vertex's chart point, clipped to the line's face
/// side, its side on the line the zero. Nothing in reach of `R` may bound
/// the face but the line's edge and the other one, whose chart curve
/// leaves the vertex into the face's side ([`leaves_line`]); every
/// vertex in reach is the corner's.
fn present_line_corner<'a>(
    piece: &Piece<'a>,
    c: Corner,
    band: Band,
) -> Result<Presentation<'a>, Unpresented> {
    let b = piece.boundary;
    let surface = piece.surface;
    let (Some(e), Some(o)) = (b.edges.get(c.read), b.edges.get(c.other)) else {
        return Err(Unpresented::Local);
    };
    let ([ue], [uo]) = (e.uses.as_slice(), o.uses.as_slice()) else {
        return Err(Unpresented::Local);
    };
    let (Some(iso), Some(h), Some((_, span))) = (ue.iso, uo.harmonic, o.arc.as_ref()) else {
        return Err(Unpresented::Local);
    };
    if iso.full {
        return Err(Unpresented::Local);
    }
    let cell = (piece.u, piece.v);
    let Sheets::On(sheet) = sheets(surface, b.extent, cell) else {
        return Err(Unpresented::Local);
    };
    let (held, moving) = (iso.held, iso.held.other());
    let line = placed(surface, iso, sheet, held.of(cell)).ok_or(Unpresented::Structural)?;
    let sigma = face_side(ue, iso, b.sense).ok_or(Unpresented::Local)?;
    let [i0, i1] = ue.image.ok_or(Unpresented::Local)?;
    let (mlo, mhi) = moving.of(cell);
    let vm = iso.moving(if e.ends[0] == c.vertex { i0 } else { i1 });
    let vm = if periodic(surface, moving) {
        wrap(vm, geom::mid_param(mlo, mhi))
    } else {
        vm
    };
    let (lo, hi) = held.of(cell);
    let across = if sigma > 0.0 {
        (line.lo(), hi.max(line.hi()))
    } else {
        (lo.min(line.lo()), line.hi())
    };
    let along = (mlo.min(vm.lo()), mhi.max(vm.hi()));
    let ((x, y), vertex) = match held {
        Axis::U => ((across, along), [line, vm]),
        Axis::V => ((along, across), [vm, line]),
    };
    // Defence. On the moving axis a copy of the line's edge a turn on lies
    // in `R` only where its far end does, a vertex refused below; on the
    // held axis `R` lies within the cell and its line, and a cell meeting
    // two copies of the face's extent reads `Several` in `sheets`, which
    // is not presented.
    if [Axis::U, Axis::V].into_iter().any(|axis| {
        let (lo, hi) = axis.of((x, y));
        periodic(surface, axis) && whole_turn(lo, hi)
    }) {
        return Err(Unpresented::Local);
    }
    let m = |(l, h): (f64, f64)| at(geom::mid_param(l, h));
    let ball = Ball::holding(surface.eval(m(x), m(y)), surface.rect_enclosure(x, y));
    let local = Local::of(b, ball, band);
    // The line admits only the vertex: its far end in `R` would leave the
    // face free to come round it.
    if !only(piece, &local, &[c.read, c.other], &[c.vertex]) {
        return Err(Unpresented::Local);
    }
    let from_start = o.ends[0] == c.vertex;
    let (ju, jv, _) = rect_jet(surface, x, y);
    leaves_line(
        surface,
        &h,
        (span.0.lo(), span.1.hi()),
        from_start,
        (held, sigma),
        vertex,
        (x, y),
        [ju.norm().hi(), jv.norm().hi()],
        band,
    )?;
    Ok(Presentation::Of(Box::new(Presented {
        surface,
        map: Map::Chart,
        x,
        y,
        zero: Zero::Side(Side::of(held, sigma > 0.0)),
        corner: true,
        cache: Cache::default(),
    })))
}

/// **The corner's other edge leaves the line toward the face's side and
/// does not come back within `R`** ([`present_corner`]'s step 1, for a
/// line): every piece of its chart curve `h`, on every copy by whole
/// turns, that meets `R` is one of a run of consecutive pieces from the
/// vertex, on the copy through it, over each of which the curve's
/// height above the line — zero at the vertex — strictly grows, decided
/// at [`CORNER_DEPARTS`] off its rate on the held axis times the piece's
/// width and `lever`. A piece that is not told is halved, [`LEAVES`]
/// times at most. A reflex corner, decided at the vertex, is
/// `Structural`; anything else not read, `Local`.
#[allow(clippy::too_many_arguments)]
fn leaves_line(
    surface: &Surface<Interval>,
    h: &Harmonic,
    (t0, t1): (f64, f64),
    from_start: bool,
    (held, sigma): (Axis, f64),
    vertex: [Interval; 2],
    r: ((f64, f64), (f64, f64)),
    speed: [f64; 2],
    band: Band,
) -> Result<(), Unpresented> {
    let t_v = if from_start { t0 } else { t1 };
    let start = pair(h.at(at(t_v)));
    let k = turns_to(surface, start, vertex);
    if !on_vertex(
        [start[0] + turns(k[0]), start[1] + turns(k[1])],
        vertex,
        speed,
        band,
    ) {
        return Err(Unpresented::Local);
    }
    let lever = speed[held.index()];
    let away = if from_start { 1.0 } else { -1.0 };
    let width = t1 - t0;
    if !(width.is_finite() && width > 0.0) {
        return Err(Unpresented::Local);
    }
    let pieces = Pieces::of(h, held, (t0, t1), false).t;
    let reach = reach_of(pieces.iter().map(|&t| pair(h.at(t))));
    // Two guards, each load-bearing. The rate keeps the run from the
    // vertex on the face's side while it is in `R`; `gone` keeps a piece
    // that comes back into `R` after the run has left — from any side, at
    // any rate — from being read as the run.
    for c in copies(surface, reach, r).ok_or(Unpresented::Local)? {
        let through = c == k;
        // The pieces in order from the vertex, each with its depth.
        let mut todo: Vec<((f64, f64), u32)> =
            pieces.iter().map(|t| ((t.lo(), t.hi()), 0)).collect();
        if from_start {
            todo.reverse();
        }
        let mut gone = false;
        while let Some(((a, z), depth)) = todo.pop() {
            let t = Interval::from_bounds(a, z);
            let (u, v) = h.at(t);
            if !meets_rect([u + turns(c[0]), v + turns(c[1])], r) {
                gone |= through;
                continue;
            }
            let split = |todo: &mut Vec<((f64, f64), u32)>| {
                let m = geom::mid_param(a, z);
                if depth >= LEAVES || !(a < m && m < z) {
                    return false;
                }
                // The half nearer the vertex is read first.
                let (near, far) = if from_start {
                    ((a, m), (m, z))
                } else {
                    ((m, z), (a, m))
                };
                todo.push((far, depth + 1));
                todo.push((near, depth + 1));
                true
            };
            if !through || gone {
                if split(&mut todo) {
                    continue;
                }
                return Err(Unpresented::Local);
            }
            let rise = h.rate(held, t) * at(sigma * away * (z - a) * lever);
            match decide(CORNER_DEPARTS, Margin::of(rise), band) {
                Ok(Sign::Positive) => {}
                Ok(Sign::Negative) if (if from_start { a } else { z }) == t_v => {
                    return Err(Unpresented::Structural);
                }
                // Off the vertex a decided `Negative` falls to the arm
                // below: the curve's rate is continuous, so the run meets a
                // piece the funnel cannot sign before any it signs negative,
                // and that piece refuses first.
                _ => {
                    if !split(&mut todo) {
                        return Err(Unpresented::Local);
                    }
                }
            }
        }
    }
    Ok(())
}

/// One lift of an edge's chart curve the cell's columns may meet: the
/// curve moved by `k` turns along the column axis, over piece `i`.
type Node = (i64, usize);

/// A run of consecutive lifts, each with the parameter offset that
/// continues the curve's closed form through a closed edge's join.
type Run = Vec<(Node, f64)>;

/// **A curved edge's chart curve over its span, cut into [`PIECES`]
/// pieces**, read along one column axis, and how its pieces follow one
/// another: through a closed edge's join, its period moves the curve by
/// its linear part's whole turns on the column axis.
struct Pieces<'c> {
    chart: &'c Harmonic,
    column: Axis,
    t: Vec<Interval>,
    width: f64,
    continued: bool,
    wind: i64,
}

impl<'c> Pieces<'c> {
    fn of(chart: &'c Harmonic, column: Axis, (t0, t1): (f64, f64), continued: bool) -> Self {
        let width = t1 - t0;
        #[allow(clippy::cast_precision_loss)]
        let bound = |i: usize| {
            if i == PIECES {
                t1
            } else {
                t0 + width * (i as f64) / (PIECES as f64)
            }
        };
        #[allow(clippy::cast_possible_truncation)]
        let wind = chart.pl[column.index()].lo() as i64;
        Self {
            chart,
            column,
            t: (0..PIECES)
                .map(|i| Interval::from_bounds(bound(i), bound(i + 1)))
                .collect(),
            width,
            continued,
            wind,
        }
    }

    /// The lift after `n`, with the change of parameter offset into it.
    fn next(&self, (k, i): Node) -> Option<(Node, f64)> {
        if i + 1 < PIECES {
            Some(((k, i + 1), 0.0))
        } else {
            self.continued.then_some(((k + self.wind, 0), self.width))
        }
    }

    /// The lift before `n`, with the change of parameter offset into it.
    fn prev(&self, (k, i): Node) -> Option<(Node, f64)> {
        if i > 0 {
            Some(((k, i - 1), 0.0))
        } else {
            self.continued
                .then_some(((k - self.wind, PIECES - 1), -self.width))
        }
    }

    /// A run member's parameters, continued.
    fn params(&self, &((_, i), offset): &(Node, f64)) -> Interval {
        self.t[i] + at(offset)
    }
}

/// **A curve read as a graph** ([`graph`]): the rectangle `R` as
/// `x × y`, whether the face lies above the curve on the other axis, and
/// the parameters the columns hold.
struct Graph {
    x: (f64, f64),
    y: (f64, f64),
    above: bool,
    t: (f64, f64),
}

/// **The curve as a graph over the column axis** in `piece`'s cell
/// ([`present_curved`]'s three steps); `None` where it is not read as
/// one.
fn graph(
    piece: &Piece<'_>,
    ei: usize,
    plus: bool,
    p: &Pieces<'_>,
    through: Option<&Through>,
    band: Band,
) -> Result<Graph, Unpresented> {
    graph_read(piece, ei, plus, p, through, band).ok_or(Unpresented::Local)?
}

/// [`graph`], `None` where the curve is not read as one; a corner it
/// reads and decides reflex is `Some(Err(Structural))`.
fn graph_read(
    piece: &Piece<'_>,
    ei: usize,
    plus: bool,
    p: &Pieces<'_>,
    through: Option<&Through>,
    band: Band,
) -> Option<Result<Graph, Unpresented>> {
    let (column, other) = (p.column, p.column.other());
    let surface = piece.surface;
    let own = (piece.u, piece.v);
    // Through a corner `R` holds the vertex: the cell is widened to its
    // chart point, on the copy nearest the cell.
    let vertex = through.map(|th| {
        let place = |axis: Axis| {
            let x = p.chart.on(axis, at(th.t));
            let (lo, hi) = axis.of(own);
            if periodic(surface, axis) {
                wrap(x, geom::mid_param(lo, hi))
            } else {
                x
            }
        };
        (place(column), place(other))
    });
    let cell = match vertex {
        Some((vc, vo)) => {
            let widen = |(lo, hi): (f64, f64), x: Interval| (lo.min(x.lo()), hi.max(x.hi()));
            match column {
                Axis::U => (widen(own.0, vc), widen(own.1, vo)),
                Axis::V => (widen(own.0, vo), widen(own.1, vc)),
            }
        }
        None => own,
    };
    let (a0, a1) = column.of(cell);
    let (b0, b1) = other.of(cell);
    // Step 1: the ONE run in reach of the cell, monotone across the
    // columns, narrowed to them.
    let mut all = runs(p, surface, (a0, a1))?;
    let nearest = |g: Interval| {
        if periodic(surface, other) {
            wrap(g, geom::mid_param(b0, b1))
        } else {
            g
        }
    };
    let reaches: Vec<Interval> = all
        .iter()
        .map(|run| {
            run.iter()
                .map(|n| p.chart.on(other, p.params(n)))
                .reduce(hull)
                .map(nearest)
        })
        .collect::<Option<_>>()?;
    let inside = |g: &Interval| g.lo() <= b1 && b0 <= g.hi();
    // A second run held by the cell is refused by `apart` below, which is
    // the load-bearing guard: `R`'s range holds the cell's, so a run that
    // meets the cell meets `R`. The first held run is the one read.
    let (chosen, _) = reaches.iter().enumerate().find(|(_, g)| inside(g))?;
    let run = all.swap_remove(chosen);
    let mut others = reaches;
    others.swap_remove(chosen);
    let (ju, jv, _) = rect_jet(surface, cell.0, cell.1);
    let lever = column.of((ju, jv)).norm().hi();
    let (run, rising) = monotone_run(p, run, (a0, a1), lever, band)?;
    let (ta, tb) = narrowed(p, &run, rising, (a0, a1))?;
    // Step 2: `R`, and nothing in it but the curve.
    const REACH: u32 = 8;
    let slice = |j: u32| ta + (tb - ta) * f64::from(j) / f64::from(REACH);
    let g = (0..REACH)
        .map(|k| {
            p.chart
                .on(other, Interval::from_bounds(slice(k), slice(k + 1)))
        })
        .reduce(hull)
        .map(nearest)?;
    // `f64::min` passes over a NaN, so a poisoned `g` is refused here,
    // before it can read as the cell's own range.
    if !Real::is_bounded(g) {
        return None;
    }
    let rb = (b0.min(g.lo()), b1.max(g.hi()));
    if whole_turn(rb.0, rb.1) {
        return None;
    }
    if !apart(rb, &others, periodic(surface, other)) {
        return None;
    }
    let (x, y) = match column {
        Axis::U => ((a0, a1), rb),
        Axis::V => (rb, (a0, a1)),
    };
    let local = if rb == (b0, b1) && cell == own {
        piece.local.clone()
    } else {
        let m = |(l, h): (f64, f64)| at(geom::mid_param(l, h));
        let ball = Ball::holding(surface.eval(m(x), m(y)), surface.rect_enclosure(x, y));
        Local::of(piece.boundary, ball, band)
    };
    if !alone(piece, ei, through.map(|th| th.corner.other), &local) {
        return None;
    }
    // Step 3: the face's side.
    let above = left_is_above(rising == plus, column) == piece.boundary.sense;
    if let Some(th) = through {
        // The vertex is in `R`, on the run read: its parameter in the
        // run's, and its point on the run's copy. The cell was widened to
        // the vertex, so the vertex's own run meets it; a run read that is
        // not that one has it among `others`, which `apart` refused above.
        // So this is not load-bearing; it states step 3's premise.
        let ends = Ends::of(p, &run, rising, (a0, a1));
        let (ts, te) = (p.params(run.first()?).lo(), p.params(run.last()?).hi());
        let within = |x: Interval, (lo, hi): (f64, f64)| lo <= x.lo() && x.hi() <= hi;
        let held = nearest(p.chart.on(other, at(th.t)));
        if !(ts <= th.t && th.t <= te && within(ends.along(th.t), (a0, a1)) && within(held, rb)) {
            return None;
        }
        let ts = Interval::from_bounds(ta, tb);
        let vertex = match column {
            Axis::U => [ends.along(th.t), held],
            Axis::V => [held, ends.along(th.t)],
        };
        if let Err(e) = leaves_graph(piece, th, p, ts, rising, above, vertex, (x, y), band)? {
            return Some(Err(e));
        }
    }
    Some(Ok(Graph {
        x,
        y,
        above,
        t: (ta, tb),
    }))
}

/// **Every lift of the curve whose column coordinate can meet the
/// columns `a`**, as runs of consecutive pieces ([`present_curved`]'s
/// step 1): each piece's interval enclosure on the column axis, moved by
/// every whole turn it can meet the columns at. A set that closes on
/// itself — a closed curve wholly within the columns — is no run.
///
/// Every turn is tried from the columns' low end against the curve's
/// high reach to their high end against its low one; more than 16 is a
/// curve whose reach along the column axis is unbounded, which no cell
/// reads.
fn runs(p: &Pieces<'_>, surface: &Surface<Interval>, (a0, a1): (f64, f64)) -> Option<Vec<Run>> {
    let reach: Vec<Interval> = p.t.iter().map(|&t| p.chart.on(p.column, t)).collect();
    let (lo, hi) = reach
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), r| {
            (l.min(r.lo()), h.max(r.hi()))
        });
    if !(lo.is_finite() && hi.is_finite()) {
        return None;
    }
    #[allow(clippy::cast_possible_truncation)]
    let turns_met: Vec<i64> = if periodic(surface, p.column) {
        turn_ladder((lo, hi), (a0, a1))?
            .into_iter()
            .map(|k| k as i64)
            .collect()
    } else {
        vec![0]
    };
    #[allow(clippy::cast_precision_loss)]
    let meets = |r: Interval, k: i64| meets_range(r + turns(k as f64), (a0, a1));
    let mut nodes: BTreeSet<Node> = turns_met
        .iter()
        .flat_map(|&k| {
            reach
                .iter()
                .enumerate()
                .filter(move |&(_, &r)| meets(r, k))
                .map(move |(i, _)| (k, i))
        })
        .collect();
    let mut out = Vec::new();
    while let Some(start) = nodes
        .iter()
        .copied()
        .find(|&n| p.prev(n).is_none_or(|(q, _)| !nodes.contains(&q)))
    {
        let mut run = Vec::new();
        let mut at_node = Some((start, 0.0));
        while let Some((n, offset)) = at_node.filter(|(n, _)| nodes.contains(n)) {
            nodes.remove(&n);
            run.push((n, offset));
            at_node = p.next(n).map(|(q, step)| (q, offset + step));
        }
        out.push(run);
    }
    nodes.is_empty().then_some(out)
}

/// **The run strictly monotone across the columns `a`, passing both
/// their ends** ([`present_curved`]'s step 1), and whether it rises:
/// each piece's advance along the column axis decided at [`COLUMN`],
/// one sign over every piece. `lever` scales an advance into metres; a
/// sign of the advance is a sign of it times any positive lever, so the
/// cell's own lever serves the whole run.
///
/// A run that stops on a column end, where the next piece lies ulps
/// beyond it (a closed edge's join), takes that piece too, on the same
/// terms.
fn monotone_run(
    p: &Pieces<'_>,
    mut run: Run,
    (a0, a1): (f64, f64),
    lever: f64,
    band: Band,
) -> Option<(Run, bool)> {
    let sign = |t: Interval| {
        let advance = p.chart.rate(p.column, t) * at(t.hi() - t.lo()) * at(lever);
        sign_of(advance, COLUMN, band)
    };
    let first = sign(p.params(run.first()?))?;
    // Every piece, not the first alone. On the charts read today — a
    // plane's affine image of a conic, a cylinder's harmonic row — the
    // column coordinate along a run is linear, or `c + a cos t + b sin t`
    // over less than a turn of `t` (a run visits a piece once, and a
    // closed one is none). Past both column ends at its ends, such a
    // curve crosses each end's level, and every level between, once:
    // twice more would be three crossings of one level within a turn. So
    // what the columns hold is monotone whatever this reads, and the
    // narrowing's predicates are too; this check is not load-bearing for
    // them. It does change verdicts: a run that turns past the exit end,
    // as an open arc may, refuses here, conservatively. It stays as the
    // step's definition and the defence for charts whose coordinates
    // turn more often.
    if !run.iter().all(|n| sign(p.params(n)) == Some(first)) {
        return None;
    }
    let rising = first == Sign::Positive;
    let ends = Ends::of(p, &run, rising, (a0, a1));
    let grow = |run: &mut Run, at_start: bool| -> Option<()> {
        let &(n, offset) = if at_start { run.first()? } else { run.last()? };
        let (q, step) = if at_start { p.prev(n)? } else { p.next(n)? };
        let member = (q, offset + step);
        if sign(p.params(&member)) != Some(first) {
            return None;
        }
        if at_start {
            run.insert(0, member);
        } else {
            run.push(member);
        }
        Some(())
    };
    if !ends.before(p.params(run.first()?).lo()) {
        grow(&mut run, true)?;
    }
    if !ends.after(p.params(run.last()?).hi()) {
        grow(&mut run, false)?;
    }
    // Passing both column ends. A monotone run that stops inside the
    // columns stops at an end of its edge: it otherwise stops where the
    // next piece misses the columns, past an end, and a closed edge's run
    // continues through its join. That end is a vertex in `R`, and a
    // valid loop runs on from it along another edge, which [`alone`]
    // finds in `R` and refuses — an analytic seam included, across which
    // the next edge is again at the vertex. So on the loops built today
    // `alone` refuses first and this is not load-bearing; it stays as the
    // step's definition, and the defence for a boundary `alone` reads
    // otherwise.
    let (ts, te) = (p.params(run.first()?).lo(), p.params(run.last()?).hi());
    (ends.before(ts) && ends.after(te)).then_some((run, rising))
}

/// Where a run's column coordinate stands against the columns' ends:
/// its lift on the column axis, and which way it runs.
struct Ends<'p, 'c> {
    p: &'p Pieces<'c>,
    lift: Interval,
    rising: bool,
    a: (f64, f64),
}

impl<'p, 'c> Ends<'p, 'c> {
    /// A member `((k, i), offset)` continues the run's lift `k₀` by
    /// `offset / width` periods, each moving `k` by the curve's winding,
    /// so `k₀ = k − (offset / width)·wind` from any member.
    fn of(p: &'p Pieces<'c>, run: &Run, rising: bool, a: (f64, f64)) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let lift = run.first().map_or(0.0, |&((k, _), offset)| {
            let periods = (offset / p.width).round() as i64;
            (k - periods * p.wind) as f64
        });
        Self {
            p,
            lift: turns(lift),
            rising,
            a,
        }
    }

    fn along(&self, t: f64) -> Interval {
        self.p.chart.on(self.p.column, at(t)) + self.lift
    }

    /// Before the columns, at `t`: not yet past their entry end.
    fn before(&self, t: f64) -> bool {
        let x = self.along(t);
        if self.rising {
            x.hi() <= self.a.0
        } else {
            x.lo() >= self.a.1
        }
    }

    /// Past the columns, at `t`: beyond their exit end.
    fn after(&self, t: f64) -> bool {
        let x = self.along(t);
        if self.rising {
            x.lo() >= self.a.1
        } else {
            x.hi() <= self.a.0
        }
    }
}

/// **The parameters the columns hold**, narrowed by halving: before the
/// first the curve has not reached the columns, past the second it has
/// left them. The run is monotone, so every parameter whose point is in
/// the columns lies between.
fn narrowed(p: &Pieces<'_>, run: &Run, rising: bool, a: (f64, f64)) -> Option<(f64, f64)> {
    let ends = Ends::of(p, run, rising, a);
    let (ts, te) = (p.params(run.first()?).lo(), p.params(run.last()?).hi());
    let halve = |keep_low: &dyn Fn(f64) -> bool| {
        let (mut lo, mut hi) = (ts, te);
        for _ in 0..NARROW {
            let m = geom::mid_param(lo, hi);
            if !(lo < m && m < hi) {
                break;
            }
            if keep_low(m) { lo = m } else { hi = m }
        }
        (lo, hi)
    };
    let ta = halve(&|t| ends.before(t)).0;
    let tb = halve(&|t| !ends.after(t)).1;
    (ta < tb).then_some((ta, tb))
}

/// **No other run in reach of `R`'s range `rb` on the other axis**
/// ([`present_curved`]'s step 2), each other run's reach on the copy
/// nearest the cell and, on a periodic axis, the copies a turn either
/// side. The copies a turn away are defence for the charts to come: on
/// a cylinder — the one periodic other axis read today, `u`, the column
/// `v` — the curve is `u = φ ± t`, one point per azimuth. A copy a turn
/// away can meet `rb` while the nearest misses it only within `rb`'s
/// reach past the cell on one side, where it reaches further than on
/// the other; those azimuths are the chosen run's own, its pieces
/// running on from the curve over the columns to the cell, and no other
/// run lies there.
///
/// It is the one guard against a second run: a run the cell itself
/// holds meets `rb`, which holds the cell's range, so [`graph`] does not
/// test for one apart from it. The chosen run's own copies need no
/// reading: `rb` spans less than a turn and holds the run's reach, so a
/// copy a turn away lies past it.
fn apart(rb: (f64, f64), others: &[Interval], periodic: bool) -> bool {
    let copies: &[f64] = if periodic { &[-1.0, 0.0, 1.0] } else { &[0.0] };
    others.iter().all(|&g| {
        copies.iter().all(|&k| {
            let copy = g + turns(k);
            copy.hi() < rb.0 || rb.1 < copy.lo()
        })
    })
}

/// **The curve `ei` the only boundary in reach** ([`present_curved`]'s
/// step 2): every other local edge an analytic seam, or, through a
/// corner, its `other` edge, and every local vertex an end of the curve
/// ([`only`]). Both ends are admitted: the run read passes both column
/// ends, so an end not continued past lies outside `R`.
fn alone(piece: &Piece<'_>, ei: usize, other: Option<usize>, local: &Local) -> bool {
    let Some(e) = piece.boundary.edges.get(ei) else {
        return false;
    };
    let edges: Vec<usize> = core::iter::once(ei).chain(other).collect();
    only(piece, local, &edges, &e.ends)
}

/// **Nothing in reach bounds the face but `edges`**: every local edge
/// one of them or an analytic seam, which is no boundary, and every
/// local vertex one of `vertices`.
fn only(piece: &Piece<'_>, local: &Local, edges: &[usize], vertices: &[VertexKey]) -> bool {
    let b = piece.boundary;
    local.edges.iter().all(|&j| {
        edges.contains(&j)
            || (!piece.whole
                && b.edges
                    .get(j)
                    .is_some_and(|r| r.arc.is_some() && r.uses.len() > 1))
    }) && local.vertices.iter().all(|v| vertices.contains(v))
}

/// **Which side of a graph its face lies on** ([`present_curved`]'s
/// step 3), in a chart positively oriented about the outward normal
/// (the sense bit reads it against the chart's): the face lies left of
/// its traversal. Travel along the column axis is `forward` where the
/// curve rises with its parameter and the loop runs it forward; the
/// left of a forward `u` is `+v`, of a forward `v` is `−u`.
fn left_is_above(forward: bool, column: Axis) -> bool {
    match column {
        Axis::U => forward,
        Axis::V => !forward,
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
        /// The edge's parameters `γ` is its graph over.
        piece: (f64, f64),
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
            Self::Graph { m, t, slope, .. } => m - t * slope,
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
/// - A curve, a graph over one axis: zero along it, so its sign on the
///   face's side is the sign of the derivative along the other axis,
///   over the whole rectangle ([`present_curved`]).
///
/// A side is zero by decision (`ON_SHARED`), except under the graph
/// separator, which vanishes on its one shared edge by construction —
/// and only there. A zero side's edges are that edge, so a side needs
/// no reading; a corner POINT is a shared vertex, which need not lie on
/// the edge, and a graph's value there is not read, so the graph
/// separator answers nothing at a point. A curve is zero by decision
/// under every separator, off a bound of it along the curve
/// ([`Curved::psi`]).
///
/// **A side that runs past the shared cell** ([`Zero::Past`]) is read
/// as a side only `against` another presentation whose zero set is on
/// the shared cell (never in the carrier arm, where `against` is `None`
/// and every point of the separating face is on its zero set), under a
/// read separator (not the graph's), on an analytic face: there a
/// coincidence off the shared cell would be two points of this face
/// with one image ([`partial_zero`]). Anywhere else it answers nothing.
/// This is the one place that guard is kept.
fn sign_on(
    psi: &Psi,
    p: &Presented<'_>,
    against: Option<&Presented<'_>>,
    band: Band,
) -> Option<Sign> {
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
    match &p.zero {
        Zero::None => sign_of(centred(psi, p, p.x, p.y)?, DEPARTURE, band),
        &Zero::Side(s) => {
            if !zero_on(s) {
                return None;
            }
            sign_of(across(s.axis()) * at(s.into()), DEPARTURE, band)
        }
        &Zero::Past(s) => {
            let exact = against.is_some_and(|q| !matches!(q.zero, Zero::Past(_)));
            let analytic = !matches!(p.surface, Surface::Nurbs(_) | Surface::Approx(_));
            if !(exact && checked && analytic && zero_on(s)) {
                return None;
            }
            sign_of(across(s.axis()) * at(s.into()), DEPARTURE, band)
        }
        &Zero::Corner(s, t) => {
            let n = psi.linear()?;
            if !(zero_on(s) && zero_on(t)) {
                return None;
            }
            let g = n.dot(fxy) * at(p.span(Axis::U) * p.span(Axis::V) * s.into() * t.into());
            sign_of(g, DEPARTURE, band)
        }
        Zero::Curve(s, c) => {
            if !c.psi(psi).is_some_and(|v| is_zero(v, band)) {
                return None;
            }
            sign_of(across(s.axis()) * at((*s).into()), DEPARTURE, band)
        }
        &Zero::Point(s, t) => {
            if !checked || !psi.value(p.corner(s, t)).is_some_and(|v| is_zero(v, band)) {
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
/// with the other presented, a plane through straight shared edges or a
/// conic one's own plane, the shared edge's graph.
fn departure(x: &Piece<'_>, y: &Piece<'_>, near: &Near, band: Band, two: bool) -> Outcome {
    let shared = near.cell(x.boundary).unwrap_or(SharedCell::Face(x.face));
    let split = Outcome::Split {
        arm: CertificateArm::Departure,
        shared,
    };
    let (ex, ey) = (near.side(false), near.side(true));
    let px = present(x, &ex, &near.vertices, band);
    let py = present(y, &ey, &near.vertices, band);
    for (p, face, side) in [(&px, x.face, Which::First), (&py, y.face, Which::Second)] {
        if let Err(Unpresented::Collapsed(v)) = p {
            return Outcome::Refuse(EngineRefusal::CollapsedChart {
                face,
                side,
                vertex: *v,
            });
        }
    }
    if matches!(px, Ok(Presentation::Vacuous)) || matches!(py, Ok(Presentation::Vacuous)) {
        return Outcome::Certified { corner: false };
    }
    // One face's carrier, the other departing it. Not between two faces
    // of one face: the face is its own carrier.
    if two {
        for (sep, other) in [(x, &py), (y, &px)] {
            if let (Some(imp), Ok(Presentation::Of(p))) = (implicit_of(sep, band), other)
                && sign_on(&Psi::Carrier(imp), p, None, band).is_some()
            {
                return Outcome::Certified { corner: p.corner };
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
    let opposite = |psi: &Psi| match (
        sign_on(psi, qx, Some(qy), band),
        sign_on(psi, qy, Some(qx), band),
    ) {
        (Some(a), Some(b)) => a == flip(b),
        _ => false,
    };
    let corner = qx.corner || qy.corner;
    if let Some(psi) = plane_through(x, near, qx, qy)
        && opposite(&psi)
    {
        return Outcome::Certified { corner };
    }
    if let Some(psi) = graph_of(x, near, qx, qy, band)
        && opposite(&psi)
    {
        return Outcome::Certified { corner };
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
/// rectangle's middle, or, across a curved side, toward the middle of
/// the rectangle's far side, which the face's side reaches. Chooses a
/// separator; decides nothing.
fn heading(p: &Presented<'_>, q: Point3<f64>) -> Option<Vec3<f64>> {
    let mut x = geom::mid_param(p.x.0, p.x.1);
    let mut y = geom::mid_param(p.y.0, p.y.1);
    if let &Zero::Curve(s, _) = &p.zero {
        let far = match s {
            Side::XLo => Side::XHi,
            Side::XHi => Side::XLo,
            Side::YLo => Side::YHi,
            Side::YHi => Side::YLo,
        };
        match s.axis() {
            Axis::U => x = p.bound(far),
            Axis::V => y = p.bound(far),
        }
    }
    unit(pmid(p.image((x, x), (y, y))) - q)
}

/// **A plane through the shared cell**: one conic shared edge's own
/// plane; or, where the shared edges are straight, through one line
/// edge, bisecting the faces' headings about it, through two line edges
/// at a shared vertex, or, at a shared vertex alone, across the faces'
/// two headings.
fn plane_through(
    x: &Piece<'_>,
    near: &Near,
    px: &Presented<'_>,
    py: &Presented<'_>,
) -> Option<Psi> {
    let q = anchor(x, near)?;
    let q0 = pmid(q);
    let (hx, hy) = (heading(px, q0)?, heading(py, q0)?);
    if let [(i, _)] = near.edges.as_slice()
        && let Some((Curve3::Circle { center, axis, .. } | Curve3::Ellipse { center, axis, .. }, _)) =
            x.boundary.edges.get(*i).and_then(|e| e.arc.as_ref())
    {
        // A planar conic: its own plane.
        return Some(Psi::Linear {
            n: lift(unit(vmid(*axis))?),
            q: *center,
        });
    }
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
    let at_k = |k: usize| t0 + width * (k as f64) / (PIECES as f64);
    let hit: Vec<bool> = (0..PIECES).map(|k| meets(at_k(k), at_k(k + 1))).collect();
    let first = hit.iter().position(|&h| h)?;
    let last = hit.iter().rposition(|&h| h)?;
    let conic = matches!(carrier, Curve3::Circle { .. } | Curve3::Ellipse { .. });
    // The run of pieces, in order, the hull spans: `(start, count)`,
    // cyclic through the seam on a closed conic.
    let (start, count) = if closed && conic && first == 0 && last == PIECES - 1 {
        // The widest cyclic run of misses; the hull is its complement.
        let mut best = (0usize, 0usize);
        for s in 0..PIECES {
            if hit[s] {
                continue;
            }
            let run = (0..PIECES).take_while(|&k| !hit[(s + k) % PIECES]).count();
            if run > best.1 {
                best = (s, run);
            }
        }
        if best.1 == 0 {
            return Some((t0, t1));
        }
        ((best.0 + best.1) % PIECES, PIECES - best.1)
    } else {
        (first, last - first + 1)
    };
    // Piece `k` of the run, unwrapped past the seam by one period.
    let piece = |k: usize| {
        let i = start + k;
        let shift = if i >= PIECES { width } else { 0.0 };
        (at_k(i % PIECES) + shift, at_k(i % PIECES + 1) + shift)
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
        piece,
    })
}

/// **A wedge-0/2π shared edge**: the faces' outward normals opposed at
/// the middle of the side or curve each presents on it
/// ([`Presented::shared_normal`]). The second-order arm that would read
/// it is not built.
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
    let nx = px.shared_normal(x.boundary.sense)?;
    let ny = py.shared_normal(y_sense)?;
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

    fn face(n: u64) -> FaceKey {
        slotmap::KeyData::from_ffi((1u64 << 32) | n).into()
    }

    fn origin() -> Point3<Interval> {
        Point3::new(iv0(0.0), iv0(0.0), iv0(0.0))
    }

    /// The degree-1 patch `S(u, v) = (u, 0, v)` on `[0, 1]²`: the plane
    /// `y = 0` as a free-form carrier.
    fn sheet() -> Surface<Interval> {
        use geom_core::spline::{KnotVector, Knots};
        let knots = || {
            Knots::from(KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap())
                .lift(Interval::from_f64)
        };
        let p = |x: f64, z: f64| Point3::new(iv0(x), iv0(0.0), iv0(z));
        Surface::Nurbs(std::sync::Arc::new(
            geom::NurbsSurface::new(
                knots(),
                knots(),
                vec![p(0.0, 0.0), p(0.0, 1.0), p(1.0, 0.0), p(1.0, 1.0)],
                vec![iv0(1.0); 4],
            )
            .unwrap(),
        ))
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
                harmonic: None,
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
        chart_near(surface, b, local, &[], (u, v))
    }

    /// [`chart`], the edges `near` shared with the pair's other face.
    fn chart_near(
        surface: &Surface<Interval>,
        b: &Boundary,
        local: &[usize],
        near: &[usize],
        (u, v): ((f64, f64), (f64, f64)),
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
        read(present_chart(&piece, near, &[], band()))
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

    /// **A band**: a face whose outer loop is one whole-turn circle at
    /// `v = 0` (the rest of it lies between that loop and a ring). The
    /// loop's hull is `v = 0` alone, which is not the face's extent: a
    /// cell above it holds the face and must not read vacuous off it.
    #[test]
    fn a_wrapping_outer_loop_bounds_nothing() {
        let s = cylinder();
        let b = Boundary {
            sense: true,
            edges: Vec::new(),
            vertices: BTreeMap::new(),
            extent: Some([(0.0, TURN), (0.0, 0.0)]),
        };
        assert_eq!(
            chart(&s, &b, &[], (1.0, 2.0), (0.5, 1.0)),
            Read::Of((1.0, 2.0), (0.5, 1.0))
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

    /// **A finger narrower than the band**: a face over `u ∈ [0, 3]`,
    /// `v ∈ [0, 0.5]`, with a finger `1e-13` wide rising at `u = 1` to
    /// `v = 1`, its `u = 1` wall shared. A cell across the finger's foot
    /// holds the face's points beside the finger, `0.1` from the shared
    /// wall; its rectangle clipped to the finger is a subset, which the
    /// coverage test refuses. Read as a sliver first, it was vacuous.
    #[test]
    fn a_finger_beside_a_shared_wall_is_not_read_as_a_sliver() {
        let s = cylinder();
        let w = 1.0 + 1e-13;
        let b = Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 2], Axis::U, 1.0, (1.0, 1.0), (1.0, 0.5)),
                rim(2, [3, 4], Axis::U, w, (w, 0.5), (w, 1.0)),
                rim(3, [5, 3], Axis::V, 0.5, (3.0, 0.5), (w, 0.5)),
                rim(4, [2, 6], Axis::V, 0.5, (1.0, 0.5), (0.0, 0.5)),
            ],
            vertices: BTreeMap::new(),
            extent: Some([(0.0, 3.0), (0.0, 1.0)]),
        };
        assert_eq!(
            chart_near(&s, &b, &all(4), &[0], ((0.9, 1.1), (0.4, 0.6))),
            Read::Local,
            "the clipped rectangle is not yet a superset: not decided, never vacuous"
        );
    }

    /// **Copies past two turns**: an extent lifted to `[−6.18, −0.05]`
    /// and a cell at `[11.98, 12.98]` meet copies two and three turns up,
    /// so the face enters the cell twice; a cell at `[13, 14]` meets the
    /// third copy alone. Copies tried only within two turns read the
    /// first as the second copy's part and the second as missed.
    #[test]
    fn every_copy_a_cell_can_meet_is_tried() {
        let s = cylinder();
        let b = Boundary {
            sense: true,
            edges: Vec::new(),
            vertices: BTreeMap::new(),
            extent: Some([(-6.18, -0.05), (0.0, 1.0)]),
        };
        assert_eq!(
            chart(&s, &b, &[], (11.98, 12.98), (0.3, 0.7)),
            Read::Local,
            "two copies in one cell"
        );
        assert_eq!(
            chart(&s, &b, &[], (13.0, 14.0), (0.3, 0.7)),
            Read::Of((13.0, 14.0), (0.3, 0.7)),
            "the third copy alone, [12.67, 18.80]"
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
            harmonic: None,
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
            harmonic: None,
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
            harmonic: None,
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
                    harmonic: None,
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

    /// The plane `z = 0`, `u_ref = x`.
    fn floor() -> Surface<Interval> {
        Surface::Plane {
            origin: origin(),
            normal: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        }
    }

    /// **A disk** of radius `r` on [`floor`], its one rim a closed circle
    /// from `u_ref = start` run forward (`plus`) or backward; `sense`
    /// the face's.
    fn disk(r: f64, start: Vec3<Interval>, plus: bool, sense: bool) -> Boundary {
        Boundary {
            sense,
            edges: vec![Rim {
                key: edge(1),
                arc: Some((
                    Curve3::Circle {
                        center: origin(),
                        axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                        radius: iv0(r),
                        u_ref: start,
                    },
                    (iv0(0.0), iv0(TURN)),
                )),
                ends: [vertex(1), vertex(1)],
                uses: vec![Use {
                    plus,
                    image: None,
                    iso: None,
                    harmonic: None,
                }],
            }],
            vertices: BTreeMap::from([(vertex(1), origin())]),
            extent: None,
        }
    }

    /// [`present_plane`] of a [`disk`] in the cell `(u, v)`, its rim
    /// shared and near.
    fn on_disk(b: &Boundary, u: (f64, f64), v: (f64, f64)) -> Read {
        let s = floor();
        let local = Local {
            edges: vec![0],
            vertices: Vec::new(),
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: b,
            u,
            v,
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        read(present_plane(&piece, &[0], &[], band()))
    }

    /// **A curved rim is presented in every cell beside it**: across the
    /// circle at its join, at a turning point of either chart axis, and
    /// at a generic point — the rectangle holding the cell's columns and
    /// the curve over them, on the face's side.
    #[test]
    fn a_disks_rim_is_presented_at_its_join_and_its_turning_points() {
        let x = Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0));
        let minus_x = Vec3::new(iv0(-1.0), iv0(0.0), iv0(0.0));
        for (start, what) in [(x, "join at +x"), (minus_x, "join at -x")] {
            let b = disk(0.7, start, true, true);
            for (u, v) in [
                ((-0.7, -0.697), (-0.0014, 0.0)),
                ((0.697, 0.7), (-0.0014, 0.0014)),
                ((0.49, 0.5), (0.48, 0.49)),
                ((-0.0014, 0.0014), (0.697, 0.702)),
            ] {
                let got = on_disk(&b, u, v);
                assert!(
                    matches!(got, Read::Of(..)),
                    "{what}: the cell {u:?} × {v:?} beside the rim is presented: {got:?}"
                );
            }
        }
    }

    /// The zero of [`present_plane`]'s presentation of a [`disk`] in a
    /// cell, its rim shared.
    fn zero_on_disk(b: &Boundary, u: (f64, f64), v: (f64, f64)) -> Option<Side> {
        let s = floor();
        let local = Local {
            edges: vec![0],
            vertices: Vec::new(),
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: b,
            u,
            v,
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        match present_plane(&piece, &[0], &[], band()) {
            Ok(Presentation::Of(q)) => match q.zero {
                Zero::Curve(side, _) => Some(side),
                _ => None,
            },
            _ => None,
        }
    }

    /// **The face's side of a curve is read off its loop and its sense.**
    /// Beside the disk's rim at `+x` the curve is a graph over `v`, and
    /// the disk lies at lower `u` — inside the circle — when the rim runs
    /// counter-clockwise about the outward normal. Run backward, or with
    /// the sense flipped, the face is the other side.
    #[test]
    fn a_curved_side_names_the_faces_side_of_the_curve() {
        let x = Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0));
        let cell = ((0.69, 0.71), (0.1, 0.11));
        let side = |plus, sense| zero_on_disk(&disk(0.7, x, plus, sense), cell.0, cell.1);
        assert_eq!(side(true, true), Some(Side::XHi), "inside the circle");
        assert_eq!(side(false, true), Some(Side::XLo), "the rim run backward");
        assert_eq!(side(true, false), Some(Side::XLo), "the sense flipped");
    }

    /// **A rim folded on itself is a cusp; across it, an annulus
    /// certifies.** [`certify_pair`] on the cell `(0.69, 0.71) × (0.1,
    /// 0.11)` beside the rim of a [`disk`], paired with the same cell of
    /// a twin sharing the rim: the twin run backward with its sense
    /// flipped lies on the same side, its normal opposed (wedge 0), and
    /// the pair is [`Outcome::Cusp`] — the rim pair's own refusal, which
    /// the sweep types `SecondOrder`. Run backward alone the twin lies
    /// across the rim (wedge π) and the pair certifies. Red if a curved
    /// side's normal is not read ([`Presented::shared_normal`]), which
    /// leaves the cusp to split.
    #[test]
    fn a_rim_folded_on_itself_is_a_cusp_and_across_it_certifies() {
        let s = floor();
        let x = Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0));
        let local = Local {
            edges: vec![0],
            vertices: Vec::new(),
        };
        let near = Near {
            edges: vec![(0, 0)],
            vertices: Vec::new(),
        };
        let disk_b = disk(0.7, x, true, true);
        let pair = |twin: &Boundary| {
            let [a, b] = [(&disk_b, face(1)), (twin, face(2))].map(|(boundary, face)| Piece {
                face,
                surface: &s,
                whole: false,
                boundary,
                u: (0.69, 0.71),
                v: (0.1, 0.11),
                local: &local,
                ball: Ball::holding(origin(), origin()),
            });
            certify_pair(&a, &b, &near, band())
        };
        let lamina = pair(&disk(0.7, x, false, false));
        assert!(
            matches!(lamina, Outcome::Cusp { shared: SharedCell::Edge(e) } if e == edge(1)),
            "the folded rim is a cusp: {lamina:?}"
        );
        let annulus = pair(&disk(0.7, x, false, true));
        assert!(
            matches!(annulus, Outcome::Certified { .. }),
            "across the rim the pair certifies: {annulus:?}"
        );
    }

    /// A line edge of a face on [`floor`], from `from` to `to`, run
    /// forward.
    fn segment(n: u64, ends: [u64; 2], from: (f64, f64), to: (f64, f64)) -> Rim {
        let p = |(x, y): (f64, f64)| Point3::new(iv0(x), iv0(y), iv0(0.0));
        Rim {
            key: edge(n),
            arc: Some((
                Curve3::Line {
                    origin: p(from),
                    dir: Vec3::new(iv0(to.0 - from.0), iv0(to.1 - from.1), iv0(0.0)),
                },
                (iv0(0.0), iv0(1.0)),
            )),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                harmonic: None,
            }],
        }
    }

    /// **A corner of the unit circle's arc on [`floor`]**: the arc from
    /// `(1, 0)` (vertex 1) counter-clockwise to `(0, 1)` (vertex 2), its
    /// face inside the circle, and `other` the edge the loop reaches
    /// `(1, 0)` along, plus `extra` edges. The arc is edge 0, `other`
    /// edge 1.
    fn quarter(other: Rim, extra: Vec<Rim>) -> Boundary {
        let z = Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0));
        let arc = Rim {
            key: edge(10),
            arc: Some((
                Curve3::Circle {
                    center: origin(),
                    axis: z,
                    radius: iv0(1.0),
                    u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                },
                (iv0(0.0), iv0(core::f64::consts::FRAC_PI_2)),
            )),
            ends: [vertex(1), vertex(2)],
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                harmonic: None,
            }],
        };
        let p = |x: f64, y: f64| Point3::new(iv0(x), iv0(y), iv0(0.0));
        let mut vertices = BTreeMap::from([(vertex(1), p(1.0, 0.0)), (vertex(2), p(0.0, 1.0))]);
        for e in core::iter::once(&other).chain(&extra) {
            if let Some((Curve3::Line { origin, dir }, _)) = &e.arc {
                vertices.entry(e.ends[0]).or_insert(*origin);
                vertices.entry(e.ends[1]).or_insert(*origin + *dir);
            }
        }
        Boundary {
            sense: true,
            edges: [arc, other].into_iter().chain(extra).collect(),
            vertices,
            extent: None,
        }
    }

    /// [`present_plane`] of `b` in the cell `u × v`, its local edges
    /// `edges` and vertices `vertices`, the arc (edge 0) shared.
    fn on_floor(
        b: &Boundary,
        edges: Vec<usize>,
        vertices: Vec<VertexKey>,
        u: (f64, f64),
        v: (f64, f64),
    ) -> Read {
        let s = floor();
        let local = Local { edges, vertices };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: b,
            u,
            v,
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        read(present_plane(&piece, &[0], &[], band()))
    }

    /// **A convex corner of a curved shared edge is read through its
    /// vertex** ([`present_corner`]): the quarter disc's arc meets the
    /// line from the origin at `(1, 0)`, which leaves the vertex into
    /// the circle, so a cell holding the vertex is presented by the
    /// region inside the circle's closed form continued past it. A cell
    /// holding only the arc is read by the arc alone.
    #[test]
    fn a_convex_corner_of_a_curved_shared_edge_is_read_through_its_vertex() {
        let b = quarter(segment(20, [3, 1], (0.0, 0.0), (1.0, 0.0)), Vec::new());
        assert!(
            matches!(
                on_floor(&b, vec![0, 1], vec![vertex(1)], (0.98, 1.02), (-0.02, 0.02)),
                Read::Of(..)
            ),
            "the corner at (1, 0) is presented"
        );
        assert!(
            matches!(
                on_floor(&b, vec![0], Vec::new(), (0.69, 0.72), (0.69, 0.72)),
                Read::Of(..)
            ),
            "the arc alone, at 45°"
        );
    }

    /// **A reflex corner is read by no presentation**: the line reaches
    /// `(1, 0)` from `(2, 0)`, so it leaves the vertex outside the
    /// circle, on the arc's other side, and the face is the three
    /// quarters about the vertex the region inside the circle does not
    /// hold. Red if [`leaves_graph`] admits a decided `Negative`.
    #[test]
    fn a_reflex_corner_of_a_curved_shared_edge_is_structural() {
        let b = quarter(segment(20, [3, 1], (2.0, 0.0), (1.0, 0.0)), Vec::new());
        assert_eq!(
            on_floor(&b, vec![0, 1], vec![vertex(1)], (0.98, 1.02), (-0.02, 0.02)),
            Read::Structural,
            "the reflex corner at (1, 0)"
        );
    }

    /// **A line tangent to the arc at its end is not read through the
    /// corner**: it leaves `(1, 0)` along the circle's tangent, against
    /// the arc's run, so it stands outside the circle, by second order,
    /// while the face beside it is inside the line — not the circle.
    /// Red if [`leaves_graph`] admits an undecided sign.
    #[test]
    fn a_line_tangent_to_the_arc_at_the_corner_is_not_read_through_it() {
        let b = quarter(segment(20, [3, 1], (1.0, -1.0), (1.0, 0.0)), Vec::new());
        assert_eq!(
            on_floor(&b, vec![0, 1], vec![vertex(1)], (0.98, 1.02), (-0.02, 0.02)),
            Read::Local,
            "the tangent join at (1, 0)"
        );
    }

    /// **An edge or a vertex in reach of the corner's rectangle but not
    /// of the cell is read**: the cell `(0.96, 0.995) × (−0.02, 0.02)`
    /// stops short of the vertex, so the rectangle is widened to hold
    /// it, and the line `x = 1.0065` past the vertex, whose ends are far
    /// off, lies in the widened rectangle's reach though clear of the
    /// cell's; so does a ring's lone vertex on it. Red if the widened
    /// rectangle reads the cell's local boundary, or [`alone`] admits
    /// any edge but the corner's two, or a vertex off the curve.
    #[test]
    fn an_edge_in_reach_of_the_widened_corner_refuses() {
        let line = || segment(20, [3, 1], (0.0, 0.0), (1.0, 0.0));
        let cell = ((0.96, 0.995), (-0.02, 0.02));
        let clear = quarter(line(), Vec::new());
        assert!(
            matches!(
                on_floor(&clear, vec![0, 1], vec![vertex(1)], cell.0, cell.1),
                Read::Of(..)
            ),
            "the corner alone is presented from the cell short of it"
        );
        let crowded = quarter(
            line(),
            vec![segment(21, [5, 6], (1.0065, -0.5), (1.0065, 0.5))],
        );
        assert_eq!(
            on_floor(&crowded, vec![0, 1], vec![vertex(1)], cell.0, cell.1),
            Read::Local,
            "a third edge in reach of the widened rectangle"
        );
        let mut ringed = quarter(line(), Vec::new());
        ringed
            .vertices
            .insert(vertex(7), Point3::new(iv0(1.0065), iv0(0.0), iv0(0.0)));
        assert_eq!(
            on_floor(&ringed, vec![0, 1], vec![vertex(1)], cell.0, cell.1),
            Read::Local,
            "a lone vertex in reach of the widened rectangle"
        );
    }

    /// **A curved corner folded on itself is a cusp**: [`certify_pair`]
    /// on the quarter disc's corner cell and the same cell of a twin
    /// lying on the same side of the arc, its loop reversed and its
    /// sense flipped — wedge 0 along the arc. Both are presented through
    /// the corner, no separator departs, and the pair is
    /// [`Outcome::Cusp`], which the sweep types `SecondOrder`.
    #[test]
    fn a_curved_corner_folded_on_itself_is_a_cusp() {
        let s = floor();
        let b = quarter(segment(20, [3, 1], (0.0, 0.0), (1.0, 0.0)), Vec::new());
        let mut twin = b.clone();
        twin.sense = false;
        for e in &mut twin.edges {
            for u in &mut e.uses {
                u.plus = !u.plus;
            }
        }
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let near = Near {
            edges: vec![(0, 0)],
            vertices: vec![vertex(1)],
        };
        let [a, c] = [(&b, face(1)), (&twin, face(2))].map(|(boundary, face)| Piece {
            face,
            surface: &s,
            whole: false,
            boundary,
            u: (0.98, 1.02),
            v: (-0.02, 0.02),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        });
        let got = certify_pair(&a, &c, &near, band());
        assert!(
            matches!(got, Outcome::Cusp { shared: SharedCell::Edge(e) } if e == edge(10)),
            "the folded corner is a cusp: {got:?}"
        );
    }

    /// A cylinder face's edge of one use whose chart image is the
    /// harmonic `h` over `t`, from vertex `ends[0]` to `ends[1]`.
    fn harmonic_rim(n: u64, ends: [u64; 2], h: Harmonic, t: (f64, f64)) -> Rim {
        Rim {
            key: edge(n),
            arc: Some((
                Curve3::Line {
                    origin: Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    dir: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                },
                (iv0(t.0), iv0(t.1)),
            )),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                harmonic: Some(h),
            }],
        }
    }

    /// `u = p0u + lu·t`, `v = p0v + av·cos t + bv·sin t + lv·t`.
    fn harmonic(p0: [f64; 2], lu: f64, (av, bv, lv): (f64, f64, f64)) -> Harmonic {
        Harmonic {
            p0: p0.map(iv0),
            pa: [iv0(0.0), iv0(av)],
            pb: [iv0(0.0), iv0(bv)],
            pl: [iv0(lu), iv0(lv)],
        }
    }

    /// A vertex of [`cylinder`] at the chart point `(u, v)`.
    fn on_cyl(u: f64, v: f64) -> Point3<Interval> {
        Point3::new(iv0(u.cos()), iv0(u.sin()), iv0(v))
    }

    /// **A corner of a cylinder face at `(1, ½)`**: the ruling `u = 1`
    /// (edge 0, vertex 1 to 2) run up from it, the face at `u < 1`, and
    /// the harmonic `other` (edge 1) reaching it; `extra` edges after.
    fn ruling_corner(other: Rim, extra: Vec<Rim>, far: (f64, f64)) -> Boundary {
        let ruling = Rim {
            key: edge(10),
            arc: Some((
                Curve3::Line {
                    origin: on_cyl(1.0, 0.5),
                    dir: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                },
                (iv0(0.0), iv0(0.5)),
            )),
            ends: [vertex(1), vertex(2)],
            uses: vec![Use {
                plus: true,
                image: Some([p2((1.0, 0.5)), p2((1.0, 1.0))]),
                iso: Some(Iso {
                    held: Axis::U,
                    at: iv0(1.0),
                    full: false,
                }),
                harmonic: None,
            }],
        };
        let mut vertices = BTreeMap::from([
            (vertex(1), on_cyl(1.0, 0.5)),
            (vertex(2), on_cyl(1.0, 1.0)),
            (vertex(3), on_cyl(far.0, far.1)),
        ]);
        for e in &extra {
            if let Some((Curve3::Line { origin, dir }, _)) = &e.arc {
                vertices.entry(e.ends[0]).or_insert(*origin);
                vertices.entry(e.ends[1]).or_insert(*origin + *dir);
            }
        }
        Boundary {
            sense: true,
            edges: [ruling, other].into_iter().chain(extra).collect(),
            vertices,
            extent: Some([(0.0, 1.0), (0.0, 1.0)]),
        }
    }

    /// [`present_chart`] of a [`ruling_corner`] in the cell `u × v`, the
    /// ruling shared.
    fn on_cylinder(b: &Boundary, u: (f64, f64), v: (f64, f64)) -> Read {
        let s = cylinder();
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: b,
            u,
            v,
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        read(present_chart(&piece, &[0], &[vertex(1)], band()))
    }

    /// **A coordinate line's corner is read by the line when the curve
    /// leaves into its face side** ([`present_line_corner`]): the curve
    /// `u = t`, `v = 1.5 − t` reaches `(1, ½)` at `t = 1` from `u < 1`,
    /// the face's side of the ruling; run the other way, `u = 2 − t`, it
    /// leaves into `u > 1` and the reflex corner is read by none. Red if
    /// [`leaves_line`] admits a decided `Negative`.
    #[test]
    fn a_rulings_corner_is_read_by_the_line_only_where_the_curve_leaves_into_the_face() {
        let cell = ((0.95, 1.0), (0.45, 0.55));
        let convex = ruling_corner(
            harmonic_rim(
                11,
                [3, 1],
                harmonic([0.0, 1.5], 1.0, (0.0, 0.0, -1.0)),
                (0.5, 1.0),
            ),
            Vec::new(),
            (0.5, 1.0),
        );
        assert!(
            matches!(on_cylinder(&convex, cell.0, cell.1), Read::Of(..)),
            "the convex corner is presented"
        );
        let reflex = ruling_corner(
            harmonic_rim(
                11,
                [3, 1],
                harmonic([2.0, 1.5], -1.0, (0.0, 0.0, -1.0)),
                (0.5, 1.0),
            ),
            Vec::new(),
            (1.5, 1.0),
        );
        assert_eq!(
            on_cylinder(&reflex, cell.0, cell.1),
            Read::Structural,
            "the reflex corner is read by none"
        );
    }

    /// **A corner of a cylinder face at `(1, ½)` on the rim `v = ½`**:
    /// the rim (edge 0) run from the vertex (vertex 1) toward `u = ½`
    /// (vertex 2), the face below it, and `curve` (edge 1) leaving the
    /// vertex, its far end (vertex 3) at the chart point `far`.
    fn rim_corner(curve: Rim, far: (f64, f64)) -> Boundary {
        let rim_line = Rim {
            key: edge(10),
            arc: Some((
                Curve3::Circle {
                    center: Point3::new(iv0(0.0), iv0(0.0), iv0(0.5)),
                    axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                    radius: iv0(1.0),
                    u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                },
                (iv0(0.5), iv0(1.0)),
            )),
            ends: [vertex(2), vertex(1)],
            uses: vec![Use {
                plus: false,
                image: Some([p2((0.5, 0.5)), p2((1.0, 0.5))]),
                iso: Some(Iso {
                    held: Axis::V,
                    at: iv0(0.5),
                    full: false,
                }),
                harmonic: None,
            }],
        };
        Boundary {
            sense: true,
            edges: vec![rim_line, curve],
            vertices: BTreeMap::from([
                (vertex(1), on_cyl(1.0, 0.5)),
                (vertex(2), on_cyl(0.5, 0.5)),
                (vertex(3), on_cyl(far.0, far.1)),
            ]),
            extent: Some([(0.0, 2.6), (0.0, 1.0)]),
        }
    }

    /// **A curve tangent to a rim at the corner is not read by the
    /// line**: the rim `v = ½` runs from `(1, ½)` toward `u = ½` with
    /// the face below it, and the curve `u = 2 − t`,
    /// `v = ½ + a − a·cos(t − 1)` reaches the vertex at `t = 1` level
    /// with the rim, standing above it — on the rim's other side — by
    /// second order. Its height's rate at the vertex is zero, so no piece
    /// there is told. Red if [`leaves_line`] admits an undecided rate.
    #[test]
    fn a_curve_tangent_to_a_rim_at_the_corner_is_not_read_by_the_line() {
        let a = 0.2;
        let (s1, c1) = 1.0f64.sin_cos();
        let curve = harmonic_rim(
            11,
            [3, 1],
            harmonic([2.0, 0.5 + a], -1.0, (-a * c1, -a * s1, 0.0)),
            (0.5, 1.0),
        );
        let b = rim_corner(curve, (1.5, 0.5 + a - a * 0.5f64.cos()));
        assert_eq!(
            on_cylinder(&b, (0.95, 1.05), (0.45, 0.5)),
            Read::Local,
            "the tangent join at (1, ½)"
        );
    }

    /// `asin 0.9`, the phase at which the rows' harmonics below stand at
    /// `0.9`.
    fn phase() -> f64 {
        0.9f64.asin()
    }

    /// **The other edge must stay on the face's side of the continued
    /// curve, not only leave into it** — a cylinder. The face lies below
    /// the curve `v = −0.4 + sin(u + asin 0.9)`, shared, which ends at the
    /// vertex `(0, ½)`, and below the rim `v = ½`, which leaves the vertex
    /// toward `+u` under the curve's continuation, rising there. The
    /// continuation comes back down across the rim at `u ≈ 0.90`, inside
    /// the cell `(−0.1, 1.0) × (0.4, 0.62)`. Red if [`leaves_graph`]
    /// decides the tangents at the vertex alone.
    #[test]
    fn a_rim_the_continued_curve_comes_back_across_is_not_read_through_the_corner() {
        let a = phase();
        let mut curve = harmonic_rim(
            10,
            [2, 1],
            harmonic([0.0, -0.4], 1.0, (a.sin(), a.cos(), 0.0)),
            (-1.5, 0.0),
        );
        curve.uses[0].plus = false;
        let rim_line = Rim {
            key: edge(11),
            arc: Some((
                Curve3::Line {
                    origin: Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    dir: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                },
                (iv0(0.0), iv0(1.0)),
            )),
            ends: [vertex(3), vertex(1)],
            uses: vec![Use {
                plus: true,
                image: Some([p2((2.5, 0.5)), p2((0.0, 0.5))]),
                iso: Some(Iso {
                    held: Axis::V,
                    at: iv0(0.5),
                    full: false,
                }),
                harmonic: None,
            }],
        };
        let b = Boundary {
            sense: true,
            edges: vec![curve, rim_line],
            vertices: BTreeMap::from([
                (vertex(1), on_cyl(0.0, 0.5)),
                (vertex(2), on_cyl(-1.5, -0.4 + (a - 1.5).sin())),
                (vertex(3), on_cyl(2.5, 0.5)),
            ]),
            extent: Some([(-1.6, 2.6), (-1.5, 0.6)]),
        };
        assert_eq!(
            on_cylinder(&b, (-0.1, 1.0), (0.4, 0.62)),
            Read::Local,
            "the rim comes back above the continued curve"
        );
    }

    /// **The same, on a plane**: the quarter disc's line runs from
    /// `(1, 0)` toward `(0.98, −1)`, inside the continued circle at first,
    /// and crosses back out at `y ≈ −0.04`, inside the cell
    /// `(0.97, 1.02) × (−0.05, 0.05)`. Red if [`leaves_graph`] decides
    /// the tangents at the vertex alone.
    #[test]
    fn a_line_the_continued_arc_comes_back_across_is_not_read_through_the_corner() {
        let b = quarter(segment(20, [3, 1], (0.98, -1.0), (1.0, 0.0)), Vec::new());
        assert_eq!(
            on_floor(&b, vec![0, 1], vec![vertex(1)], (0.97, 1.02), (-0.05, 0.05)),
            Read::Local,
            "the line crosses back out of the circle"
        );
    }

    /// **A reflex corner whose curve runs backward along its column is
    /// read by none**: the arc from `(1, 0)` clockwise to `(0, −1)`, its
    /// face inside the circle, the loop reaching the vertex along it and
    /// leaving along the line to `(2, 0)`, outside the circle — the
    /// quarter disc's reflex corner mirrored, its curve falling along
    /// `v`. Red if [`leaves_graph`] drops the sign of the column's rate.
    #[test]
    fn a_reflex_corner_whose_curve_falls_along_its_column_is_structural() {
        let mut b = quarter(segment(20, [1, 3], (1.0, 0.0), (2.0, 0.0)), Vec::new());
        b.edges[0].arc = Some((
            Curve3::Circle {
                center: origin(),
                axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(-1.0)),
                radius: iv0(1.0),
                u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            },
            (iv0(0.0), iv0(core::f64::consts::FRAC_PI_2)),
        ));
        b.edges[0].uses[0].plus = false;
        b.vertices
            .insert(vertex(2), Point3::new(iv0(0.0), iv0(-1.0), iv0(0.0)));
        assert_eq!(
            on_floor(&b, vec![0, 1], vec![vertex(1)], (0.98, 1.02), (-0.02, 0.02)),
            Read::Structural,
            "the mirrored reflex corner"
        );
    }

    /// **The other edge must stay on the line's face side while it is in
    /// `R`, not only leave into it**: off the rim, the curve `u = 1 + t`,
    /// `v = 1.4 − sin(t + asin 0.9)` dips below the rim from the vertex,
    /// then rises back across it at `u ≈ 1.9`, inside the cell
    /// `(0.95, 2.0) × (0.3, 0.55)`. Red if [`leaves_line`] admits a piece
    /// off the vertex whose rate is not told.
    #[test]
    fn a_curve_that_rises_back_across_the_rim_is_not_read_by_the_line() {
        let a = phase();
        let curve = harmonic_rim(
            11,
            [1, 3],
            harmonic([1.0, 1.4], 1.0, (-a.sin(), -a.cos(), 0.0)),
            (0.0, 1.5),
        );
        let b = rim_corner(curve, (2.5, 1.4 - (1.5 + a).sin()));
        assert_eq!(
            on_cylinder(&b, (0.95, 2.0), (0.3, 0.55)),
            Read::Local,
            "the curve rises back above the rim"
        );
    }

    /// **The same off the ruling**: the curve `u = 1 − 0.2·sin t`,
    /// `v = ½ − 0.01·t` leaves the vertex into `u < 1` and curls back
    /// across the ruling's continuation at `t = π`, inside the cell
    /// `(0.75, 1.0) × (0.45, 0.55)`. Red as that row is.
    #[test]
    fn a_curve_that_curls_back_across_the_ruling_is_not_read_by_the_line() {
        let curl = Harmonic {
            p0: [iv0(1.0), iv0(0.5)],
            pa: [iv0(0.0), iv0(0.0)],
            pb: [iv0(-0.2), iv0(0.0)],
            pl: [iv0(0.0), iv0(-0.01)],
        };
        let far = (1.0 - 0.2 * 3.5f64.sin(), 0.5 - 0.035);
        let b = ruling_corner(harmonic_rim(11, [1, 3], curl, (0.0, 3.5)), Vec::new(), far);
        assert_eq!(
            on_cylinder(&b, (0.75, 1.0), (0.45, 0.55)),
            Read::Local,
            "the curve curls back across the ruling"
        );
    }

    /// **A piece that comes back into `R` after the run has left is not
    /// read as the run**, at whatever rate: off the rim, the curve
    /// `u = 1.15 − 0.15·cos t + 0.008·t`, `v = ½ − 0.15·sin t` leaves the
    /// vertex downward, out of the cell `(0.85, 1.1) × (0.45, 0.55)`
    /// through its bottom, loops round outside it, and comes back down
    /// across the rim's continuation at `u ≈ 1.05` — onto the face's side,
    /// so its rate there is the run's sign. Red if [`leaves_line`] never
    /// marks the run gone, or reads a piece after it as the run.
    #[test]
    fn a_curve_that_comes_back_into_the_rectangle_is_not_read_by_the_line() {
        let loop_back = Harmonic {
            p0: [iv0(1.15), iv0(0.5)],
            pa: [iv0(-0.15), iv0(0.0)],
            pb: [iv0(0.0), iv0(-0.15)],
            pl: [iv0(0.008), iv0(0.0)],
        };
        let t1 = TURN + 1.5;
        let far = (1.15 - 0.15 * t1.cos() + 0.008 * t1, 0.5 - 0.15 * t1.sin());
        let b = rim_corner(harmonic_rim(11, [1, 3], loop_back, (0.0, t1)), far);
        assert_eq!(
            on_cylinder(&b, (0.85, 1.1), (0.45, 0.55)),
            Read::Local,
            "the curve comes back across the rim's continuation"
        );
    }

    /// **The cylinder wall through a corner of a disc's arc**, for the
    /// folds below: the unit cylinder's chart, the arc `v = 0`,
    /// `u ∈ [0, π/2]` (edge 0, run from `u = π/2` to the vertex `(0, 0)`)
    /// with the face below it, and the ruling `u = 0` (edge 1) run up from
    /// the vertex, the face past it at `u < 0` — so the wall holds the
    /// circle `v = 0` past the vertex, at `u < 0`.
    fn wall_past_the_corner() -> Boundary {
        let arc = Rim {
            key: edge(30),
            arc: Some((
                Curve3::Line {
                    origin: Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    dir: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                },
                (iv0(0.0), iv0(1.0)),
            )),
            ends: [vertex(1), vertex(2)],
            uses: vec![Use {
                plus: false,
                image: Some([p2((0.0, 0.0)), p2((core::f64::consts::FRAC_PI_2, 0.0))]),
                iso: Some(Iso {
                    held: Axis::V,
                    at: iv0(0.0),
                    full: false,
                }),
                harmonic: None,
            }],
        };
        let ruling = Rim {
            key: edge(31),
            arc: Some((
                Curve3::Line {
                    origin: Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    dir: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                },
                (iv0(0.0), iv0(1.0)),
            )),
            ends: [vertex(1), vertex(8)],
            uses: vec![Use {
                plus: true,
                image: Some([p2((0.0, 0.0)), p2((0.0, 0.3))]),
                iso: Some(Iso {
                    held: Axis::U,
                    at: iv0(0.0),
                    full: false,
                }),
                harmonic: None,
            }],
        };
        Boundary {
            sense: true,
            edges: vec![arc, ruling],
            vertices: BTreeMap::from([
                (vertex(1), on_cyl(0.0, 0.0)),
                (vertex(2), on_cyl(core::f64::consts::FRAC_PI_2, 0.0)),
                (vertex(8), on_cyl(0.0, 0.3)),
            ]),
            extent: Some([(-0.5, 1.6), (-1.0, 0.3)]),
        }
    }

    /// [`certify_pair`] of a face on [`floor`] in the cell `disc` and
    /// [`wall_past_the_corner`] in the cell `wall`, sharing the arc
    /// (edge 0 of each) and the vertex at `(1, 0, 0)`.
    fn across_the_fold(disc: &Boundary, cells: [((f64, f64), (f64, f64)); 2]) -> Outcome {
        let (floor_s, cyl) = (floor(), cylinder());
        let wall = wall_past_the_corner();
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let near = Near {
            edges: vec![(0, 0)],
            vertices: vec![vertex(1)],
        };
        let piece = |surface, boundary, (u, v): ((f64, f64), (f64, f64)), face| Piece {
            face,
            surface,
            whole: false,
            boundary,
            u,
            v,
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        certify_pair(
            &piece(&floor_s, disc, cells[0], face(1)),
            &piece(&cyl, &wall, cells[1], face(2)),
            &near,
            band(),
        )
    }

    /// **A fold through a reflex corner is not certified.** The quarter
    /// disc's reflex face (the line reaching `(1, 0)` from `(2, 0)`)
    /// holds the circle past the vertex; so does the wall past the
    /// corner. The two coincide there, within the corner cells, a fold of
    /// M on no shared cell. Red if [`leaves_graph`] admits a decided
    /// `Negative`: the disc is then presented by the inside of the
    /// circle, the wall's carrier departs it, and the pair certifies.
    #[test]
    fn a_fold_through_a_reflex_corner_is_not_certified() {
        let disc = quarter(segment(20, [3, 1], (2.0, 0.0), (1.0, 0.0)), Vec::new());
        let got = across_the_fold(
            &disc,
            [
                ((0.98, 1.02), (-0.02, 0.02)),
                ((-0.02, 0.02), (-0.02, 0.02)),
            ],
        );
        assert!(
            !matches!(got, Outcome::Certified { .. }),
            "the fold is not certified: {got:?}"
        );
    }

    /// **A fold where the other edge crosses back is not certified.** The
    /// quarter disc whose line crosses back out of the continued circle
    /// at `y ≈ −0.04` holds the circle past that point; so does the wall
    /// past the corner. Red if [`leaves_graph`] decides the tangents at
    /// the vertex alone.
    #[test]
    fn a_fold_past_a_line_that_crosses_back_is_not_certified() {
        let disc = quarter(segment(20, [3, 1], (0.98, -1.0), (1.0, 0.0)), Vec::new());
        let got = across_the_fold(
            &disc,
            [
                ((0.97, 1.02), (-0.05, 0.05)),
                ((-0.05, 0.05), (-0.02, 0.02)),
            ],
        );
        assert!(
            !matches!(got, Outcome::Certified { .. }),
            "the fold is not certified: {got:?}"
        );
    }

    /// **An other edge that misses the vertex by more than the band is
    /// read by neither reading**: the quarter disc's line ends at
    /// `(1.001, 0)`, a millimetre off the arc's end, and the rim corner's
    /// curve ends at `(0.999, ½)`, a millimetre off the ruling. Red if
    /// [`on_vertex`] reads two points that far apart as one.
    #[test]
    fn an_other_edge_off_the_vertex_is_not_read_through_it() {
        let disc = quarter(segment(20, [3, 1], (0.0, 0.0), (1.001, 0.0)), Vec::new());
        assert_eq!(
            on_floor(
                &disc,
                vec![0, 1],
                vec![vertex(1)],
                (0.98, 1.02),
                (-0.02, 0.02)
            ),
            Read::Local,
            "the line a millimetre off the arc's end"
        );
        let off = ruling_corner(
            harmonic_rim(
                11,
                [3, 1],
                harmonic([-0.001, 1.5], 1.0, (0.0, 0.0, -1.0)),
                (0.5, 1.0),
            ),
            Vec::new(),
            (0.499, 1.0),
        );
        assert_eq!(
            on_cylinder(&off, (0.95, 1.0), (0.45, 0.55)),
            Read::Local,
            "the curve a millimetre off the ruling"
        );
    }

    /// **An edge or a vertex in reach of a line corner's rectangle but
    /// not of the cell is read**: the cell `(0.9, 0.98) × (0.45, 0.55)`
    /// stops short of the ruling, so the rectangle is widened to the
    /// vertex, and the ruling `u = 1.015`, whose ends are far off, lies
    /// in its reach though clear of the cell's; so does a ring's lone
    /// vertex on it. Red if [`present_line_corner`] admits any edge but
    /// the corner's two, or any vertex but the corner's.
    #[test]
    fn an_edge_in_reach_of_a_widened_line_corner_refuses() {
        let cell = ((0.9, 0.98), (0.45, 0.55));
        let other = || {
            harmonic_rim(
                11,
                [3, 1],
                harmonic([0.0, 1.5], 1.0, (0.0, 0.0, -1.0)),
                (0.5, 1.0),
            )
        };
        let clear = ruling_corner(other(), Vec::new(), (0.5, 1.0));
        assert!(
            matches!(on_cylinder(&clear, cell.0, cell.1), Read::Of(..)),
            "the corner alone is presented from the cell short of it"
        );
        let crowded = ruling_corner(
            other(),
            vec![Rim {
                key: edge(12),
                arc: Some((
                    Curve3::Line {
                        origin: on_cyl(1.015, -0.5),
                        dir: Vec3::new(iv0(0.0), iv0(0.0), iv0(2.0)),
                    },
                    (iv0(0.0), iv0(1.0)),
                )),
                ends: [vertex(5), vertex(6)],
                uses: Vec::new(),
            }],
            (0.5, 1.0),
        );
        assert_eq!(
            on_cylinder(&crowded, cell.0, cell.1),
            Read::Local,
            "a third edge in reach of the widened rectangle"
        );
        let mut ringed = ruling_corner(other(), Vec::new(), (0.5, 1.0));
        ringed.vertices.insert(vertex(7), on_cyl(1.015, 0.5));
        assert_eq!(
            on_cylinder(&ringed, cell.0, cell.1),
            Read::Local,
            "a lone vertex in reach of the widened rectangle"
        );
    }

    /// **A separator along a curve on its zero set reads rounding at any
    /// width of the parameters**: the oblique section of the unit
    /// cylinder, read through the cylinder's carrier, over half a turn.
    /// The same curve's image box read through the carrier is wide by
    /// the whole box.
    #[test]
    fn a_separator_along_a_curve_on_its_zero_set_reads_rounding() {
        let cyl = cylinder();
        let (s, c) = 0.4f64.sin_cos();
        // `u = t`, `v = 2 + tan θ · cos t`: the section by a plane tilted
        // `θ` about `y`, on the cylinder's chart.
        let h = Harmonic {
            p0: [iv0(0.0), iv0(2.0)],
            pa: [iv0(0.0), iv0(s / c)],
            pb: [iv0(0.0), iv0(0.0)],
            pl: [iv0(1.0), iv0(0.0)],
        };
        let image = Trig::of(&cyl, &h).expect("a section winding the cylinder once");
        let Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } = cyl
        else {
            unreachable!()
        };
        let psi = Psi::Carrier(Implicit::Cylinder {
            o: origin,
            axis,
            r: radius,
        });
        let t = Interval::from_bounds(0.0, core::f64::consts::PI);
        let along = image.psi(&psi, t).expect("a carrier reads");
        assert!(
            along.lo().abs() < 1e-12 && along.hi().abs() < 1e-12,
            "on the zero set: {along:?}"
        );
        let (u, v) = h.at(t);
        let boxed = psi.value(cyl.eval(u, v)).expect("a closed form");
        assert!(boxed.hi() - boxed.lo() > 0.1, "the box read: {boxed:?}");
    }

    /// **A zero side may run on past the shared cell** only as the one
    /// clip holding a shared edge, beside an edge the other face does not
    /// share, every shared vertex an end of its edges.
    #[test]
    fn a_partial_zero_side_is_one_clip_past_the_shared_edge() {
        let b = seam_strip(true);
        let clip = |side, edges: Vec<usize>| Clip {
            side,
            at: iv0(0.0),
            edges,
            collapsed: None,
        };
        let line = clip(Side::YLo, vec![1, 3]);
        assert_eq!(
            partial_zero(&b, &[line], &[1], &[vertex(3)]),
            Some(Side::YLo),
            "one shared edge, one not"
        );
        let line = clip(Side::YLo, vec![1, 3]);
        assert_eq!(
            partial_zero(&b, &[line], &[], &[]),
            None,
            "no shared edge on it"
        );
        let (a, c) = (clip(Side::YLo, vec![1, 3]), clip(Side::XHi, vec![0]));
        assert_eq!(
            partial_zero(&b, &[a, c], &[1, 0], &[]),
            None,
            "two clips hold shared edges"
        );
        let line = clip(Side::YLo, vec![1, 3]);
        assert_eq!(
            partial_zero(&b, &[line], &[1], &[vertex(9)]),
            None,
            "a shared vertex off its edges"
        );
    }

    /// **A disk-like plane face bounded by one closed ellipse** of
    /// semi-axes `a`, `b`, its major axis at `angle` in the plane's chart,
    /// run forward, sense up.
    fn oval(a: f64, b: f64, angle: f64) -> Boundary {
        let (s, c) = angle.sin_cos();
        Boundary {
            sense: true,
            edges: vec![Rim {
                key: edge(1),
                arc: Some((
                    Curve3::Ellipse {
                        center: origin(),
                        axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                        semi_u: iv0(a),
                        semi_v: iv0(b),
                        u_ref: Vec3::new(iv0(c), iv0(s), iv0(0.0)),
                    },
                    (iv0(0.0), iv0(TURN)),
                )),
                ends: [vertex(1), vertex(1)],
                uses: vec![Use {
                    plus: true,
                    image: None,
                    iso: None,
                    harmonic: None,
                }],
            }],
            vertices: BTreeMap::new(),
            extent: None,
        }
    }

    /// **A second run of the curve in reach of the widened rectangle is
    /// not read past.** A thin oval, its major axis tilted: over the
    /// cell's columns on `v` the run through the cell is shallow, so `R`
    /// widens along `u` far past the cell, into the reach of the oval's
    /// other run over those columns, which the cell itself does not
    /// meet; over `u` the same holds of the other pair of runs. Neither
    /// reading presents it; read past the other run, `R` would hold a
    /// boundary the column argument does not know.
    #[test]
    fn a_second_run_in_reach_of_the_widened_rectangle_refuses() {
        let b = oval(
            0.880_878_980_650_731_8,
            0.025_497_676_308_968_174,
            0.485_098_449_269_096_57,
        );
        assert_eq!(
            on_disk(
                &b,
                (0.703_916_199_917_691_5, 0.734_125_180_463_093_3),
                (0.395_250_072_325_091_6, 0.403_810_419_745_554_4)
            ),
            Read::Local
        );
    }

    /// **A rectangle a whole turn wide is not read.** An open arc of the
    /// cylinder's oblique section, `u = t`, `v = sin t / 2` for `t` in
    /// `[−0.5, 1]`: beside the arc, in a cell `0.1` wide, it is read as a
    /// graph over `v`. In a cell spanning the whole turn of `u`, `R`
    /// holds the curve over the columns on the lift nearest the cell's
    /// middle, `t + 2π`, and the cell holds it again at `t`: two copies
    /// of one curve, which no graph reads. Red if `R` may span a whole
    /// turn.
    #[test]
    fn a_rectangle_a_whole_turn_wide_is_not_read() {
        let cyl = cylinder();
        let (t0, t1) = (-0.5, 1.0);
        let n = (iv0(-0.5), iv0(1.0));
        let norm = 1.25f64.sqrt();
        let on = |t: f64| Point3::new(iv0(t.cos()), iv0(t.sin()), iv0(0.5 * t.sin()));
        let b = Boundary {
            sense: true,
            edges: vec![Rim {
                key: edge(1),
                arc: Some((
                    Curve3::Ellipse {
                        center: origin(),
                        axis: Vec3::new(iv0(0.0), iv0(-0.5 / norm), iv0(1.0 / norm)),
                        semi_u: iv0(1.0),
                        semi_v: iv0(norm),
                        u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                    },
                    n,
                )),
                ends: [vertex(1), vertex(2)],
                uses: vec![Use {
                    plus: true,
                    image: None,
                    iso: None,
                    harmonic: Some(Harmonic {
                        p0: [iv0(0.0), iv0(0.0)],
                        pa: [iv0(0.0), iv0(0.0)],
                        pb: [iv0(0.0), iv0(0.5)],
                        pl: [iv0(1.0), iv0(0.0)],
                    }),
                }],
            }],
            vertices: BTreeMap::from([(vertex(1), on(t0)), (vertex(2), on(t1))]),
            extent: None,
        };
        let strip = (-0.148, 0.05);
        let beside = chart_near(&cyl, &b, &[0], &[0], ((-0.05, 0.05), strip));
        assert!(matches!(beside, Read::Of(..)), "beside the arc: {beside:?}");
        let whole = chart_near(&cyl, &b, &[0], &[0], ((0.0, TURN), strip));
        assert!(
            !matches!(whole, Read::Of(..)),
            "a whole turn of u: {whole:?}"
        );
    }

    /// **Another edge in reach of the widened rectangle is not read
    /// past.** The disk's rim, and a short line edge — material past the
    /// rim, a C's inner lip — clear of the cell's ball but inside the
    /// rectangle the rim widens it to: just above the rim's top, the
    /// cell's columns hold the rim below the cell, so `R` reaches down to
    /// it, past the line. Over `v` the rim turns back in the cell.
    #[test]
    fn another_edge_in_reach_of_the_widened_rectangle_refuses() {
        let x = Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0));
        let mut b = disk(0.7, x, true, true);
        let lip = |u: f64| Point3::new(iv0(u), iv0(0.6985), iv0(0.0));
        b.edges.push(Rim {
            key: edge(2),
            arc: Some((
                Curve3::Line {
                    origin: lip(0.0502),
                    dir: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                },
                (iv0(0.0), iv0(1e-4)),
            )),
            ends: [vertex(2), vertex(3)],
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                harmonic: None,
            }],
        });
        b.vertices.insert(vertex(2), lip(0.0502));
        b.vertices.insert(vertex(3), lip(0.0503));
        let cell = ((0.05, 0.0505), (0.6999, 0.700_05));
        assert_eq!(
            on_disk(&b, cell.0, cell.1),
            Read::Local,
            "the lip is in reach"
        );
        b.edges.pop();
        b.vertices.retain(|&v, _| v == vertex(1));
        let got = on_disk(&b, cell.0, cell.1);
        assert!(
            matches!(got, Read::Of(..)),
            "without the lip the rim alone is read: {got:?}"
        );
    }

    /// The plane `z = 0`'s disk of radius `0.7` presented in the cell
    /// `(0.69, 0.71) × (0.1, 0.11)` across its rim, with its curve.
    fn rim_cell(b: &Boundary, s: &Surface<Interval>) -> (Box<Curved>, Psi) {
        let local = Local {
            edges: vec![0],
            vertices: Vec::new(),
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: s,
            whole: false,
            boundary: b,
            u: (0.69, 0.71),
            v: (0.1, 0.11),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let Ok(Presentation::Of(q)) = present_plane(&piece, &[0], &[], band()) else {
            panic!("the rim is presented");
        };
        let Zero::Curve(_, c) = &q.zero else {
            panic!("across a curve");
        };
        let carrier = |r: f64| {
            Psi::Carrier(Implicit::Cylinder {
                o: origin(),
                axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                r: iv0(r),
            })
        };
        assert!(
            sign_on(&carrier(0.7), &q, None, band()).is_some(),
            "the rim's own cylinder separates"
        );
        assert_eq!(
            sign_on(&carrier(0.69), &q, None, band()),
            None,
            "a cylinder the rim is 0.01 off does not, though it is monotone across"
        );
        (c.clone(), carrier(0.69))
    }

    /// **A separator the curve is off reads no zero along it**, and the
    /// presentation then has no sign under it: the zero check along a
    /// curve is red when it is too lax, not only when too strict.
    #[test]
    fn a_separator_the_curve_is_off_reads_no_zero_along_it() {
        let s = floor();
        let x = Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0));
        let (c, off) = rim_cell(&disk(0.7, x, true, true), &s);
        let v = c.psi(&off).expect("a carrier reads");
        assert!(
            !is_zero(v, band()) && v.hi() >= 0.009,
            "0.01 off the curve: {v:?}"
        );
    }

    /// **The graph separator's bound along a curve grows with the side's
    /// distance from the edge's carrier**: zero where the carrier is the
    /// side's own circle, and at least the distance where it is a circle
    /// `0.01` larger.
    #[test]
    fn the_graph_separators_bound_reads_the_sides_distance_from_its_carrier() {
        let s = floor();
        let x = Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0));
        let (c, _) = rim_cell(&disk(0.7, x, true, true), &s);
        let graph = Psi::Graph {
            m: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            t: Vec3::new(iv0(0.0), iv0(1.0), iv0(0.0)),
            slope: iv0(0.0),
            piece: (c.t.0 - 0.1, c.t.1 + 0.1),
        };
        let on = c.psi(&graph).expect("the edge is a conic");
        assert!(is_zero(on, band()), "on its own carrier: {on:?}");
        let mut off = *c;
        let wider = Curve3::Circle {
            center: origin(),
            axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            radius: iv0(0.71),
            u_ref: x,
        };
        off.edge = conic(&wider);
        let v = off.psi(&graph).expect("the edge is a conic");
        assert!(v.hi() >= 0.01 && !is_zero(v, band()), "0.01 off: {v:?}");
    }

    /// **A side past the shared cell reads a sign only where it is
    /// admitted** ([`sign_on`]): against a presentation whose zero set is
    /// on the shared cell, under a read separator. Not in the carrier arm
    /// (no other presentation), not under the graph separator, not
    /// against another past side, not where the side is off the
    /// separator's zero set, and not on a free-form carrier, whose
    /// injectivity no kind gives.
    #[test]
    fn a_side_past_the_shared_cell_reads_only_where_admitted() {
        let cyl = cylinder();
        let past = |zero| Presented {
            surface: &cyl,
            map: Map::Chart,
            x: (0.1, 0.2),
            y: (0.0, 0.1),
            zero,
            corner: false,
            cache: Cache::default(),
        };
        let p = past(Zero::Past(Side::YLo));
        let exact = past(Zero::Side(Side::YLo));
        let other = past(Zero::Past(Side::YLo));
        let z = Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0));
        let plane = Psi::Linear { n: z, q: origin() };
        let graph = Psi::Graph {
            m: z,
            t: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            slope: iv0(0.0),
            piece: (0.0, 1.0),
        };
        assert_eq!(
            sign_on(&plane, &p, Some(&exact), band()),
            Some(Sign::Positive),
            "admitted"
        );
        assert_eq!(sign_on(&plane, &p, None, band()), None, "the carrier arm");
        assert_eq!(
            sign_on(&graph, &p, Some(&exact), band()),
            None,
            "the graph separator"
        );
        assert_eq!(
            sign_on(&plane, &p, Some(&other), band()),
            None,
            "against a past side"
        );
        let off = Psi::Linear {
            n: z,
            q: Point3::new(iv0(0.0), iv0(0.0), iv0(0.01)),
        };
        assert_eq!(
            sign_on(&off, &p, Some(&exact), band()),
            None,
            "a separator the side is 0.01 off"
        );
        let patch = sheet();
        let free = Presented {
            surface: &patch,
            ..past(Zero::Past(Side::YLo))
        };
        assert_eq!(
            sign_on(&plane, &free, Some(&exact), band()),
            None,
            "a free-form carrier"
        );
    }
}
