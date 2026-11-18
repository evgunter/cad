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
//! graph over one chart axis ([`present_curved`]; a spiric's wall image
//! on a torus, over the minor angle, [`present_wall`]), and a derivative
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
//! - **the sliver** ([`SELF_INTERSECTION_SLIVER`], [`present_chart`],
//!   [`present_line_corner`], [`present_plane_corner`]): a rectangle
//!   within the band of a shared line reads as holding nothing off it;
//! - **the sheet's overlap** ([`sheets`]): a cell overlapping a copy of
//!   the face's extent by no more than [`chart_slack`] meets none of it,
//!   and an extent within [`few_ulps`] of a turn reads as wrapping;
//! - **the touch** ([`vacuous`], [`present_chart`]): a line within
//!   [`few_ulps`] of the cell's side touches it, and a cell reaching no
//!   more than [`few_ulps`] past a line onto its face side lies outside
//!   it. The touch is read in a pose ([`present_posed`]) too, where a
//!   line is an enclosure of its edge's lift: it touches only where
//!   every point of the enclosure does (its far bound), so the pose's
//!   touch is the stored one's, no wider; a posed extent's overlap is
//!   read as a stored one's;
//! - **the cover** ([`covered`]): an edge whose range stops within
//!   [`few_ulps`] of the cell's side covers it. (A side's edge is first
//!   run on past a corner it makes with a line on the other axis, to
//!   that line's far bound ([`run_on`]): structure, not a concession,
//!   as the corner is on both.)
//! - **the join** ([`present_curved`], [`present_wall`]): a closed
//!   edge's periodic chart curve is read through its vertex by its
//!   closed form, past a span
//!   that stops short of the period, in the curve's parameter, by the
//!   rounding of `τ`;
//! - **the vertex** ([`on_vertex`]): at a corner the two edges' points at
//!   the vertex, decided within the band of each other, are read as one,
//!   so a face point between the two edges within the band of the corner's
//!   other edge goes unread.
//!
//! A separator concedes once more, in what it lists as vanishing on
//! ([`On`]): a vertex at a conic's end. A conic's own plane
//! ([`planes_through`]) lists its arcs' ends, and the graph separator
//! at a vertex the pair shares alone ([`graph_of`]) lists that vertex,
//! the end of the arc it is built over. Each vanishes exactly on the
//! carrier at the end's parameter, and the stored vertex lies off it by
//! up to the edge's certified tolerance (`EdgeCurve::certify`'s end
//! check), no wider. A partner presented with its corner at that vertex
//! ([`Zero::Point`]) reads `ψ` zero there, so its sign may be misread
//! within that tolerance, over `ψ`'s lean across the partner, of the
//! vertex: the stored vertex's own tolerance, not a new one.

use std::collections::{BTreeMap, BTreeSet};

use geom::{Curve3, Surface};
use geom_brep::PolygonOrientation;
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
pub(crate) const SELF_INTERSECTION_ON_SHARED: &str = "self_intersection_on_shared";
/// The funnel site of a separator's sign over a presented face: its
/// derivative across the face's shared side times the rectangle's extent
/// across it, or its value where no side is shared, metres.
pub(crate) const SELF_INTERSECTION_DEPARTURE: &str = "self_intersection_departure";
/// The funnel site of the graph arm's monotone edge: the edge's advance
/// along the projection's first axis over its piece, metres.
pub(crate) const SELF_INTERSECTION_GRAPH: &str = "self_intersection_graph";
/// The funnel site of a planar presentation's reach along its edge:
/// how far inside the edge's two ends the cell's extent stays, metres.
pub(crate) const SELF_INTERSECTION_SPAN: &str = "self_intersection_span";
/// The funnel site of a planar corner's convexity: how far the second
/// edge's far end stands on the first edge's face side, metres.
pub(crate) const SELF_INTERSECTION_CORNER: &str = "self_intersection_corner";
/// The funnel site of a vertex at a collapse of its face's chart, a
/// cone's apex or a sphere's pole: its distance from that point, metres.
/// `Zero` is the collapse.
pub(crate) const SELF_INTERSECTION_APEX: &str = "self_intersection_apex";
/// The funnel site of a cone face's nappe: a vertex's signed height
/// along the axis from the apex, metres. Its sign is the nappe.
pub(crate) const SELF_INTERSECTION_NAPPE: &str = "self_intersection_nappe";
/// The funnel site of a presentation clipped to a shared line: the
/// rectangle's extent across the line times the chart's speed across it
/// (an upper bound, `rect_jet`'s), metres. `Zero` reads the strip as
/// holding no point of the face off the line — one of the presentation's
/// ε concessions (the module docs list them).
pub(crate) const SELF_INTERSECTION_SLIVER: &str = "self_intersection_sliver";
/// The funnel site of a wedge-0/2π shared edge: the length of the sum of
/// the two faces' unit outward normals at the shared side's middle,
/// times the ball's reach, metres. `Zero` is opposed normals.
pub(crate) const SELF_INTERSECTION_CUSP: &str = "self_intersection_cusp";
/// The funnel site of the monotone arm: the least eigenvalue of the
/// symmetric part of the projected chart's Jacobian, each column scaled
/// by the rectangle's span on its axis, metres.
pub(crate) const SELF_INTERSECTION_MONOTONE: &str = "self_intersection_monotone";
/// The funnel rows of a torus's ring condition
/// ([`geom::RingTorus::of_at`]): its minor radius, then its major radius
/// less its minor radius, metres.
pub(crate) const RING_TORUS: geom::RingSites = geom::RingReader::SelfIntersection.sites();

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
    /// The image, where it is a spiric's on a torus wall.
    pub(crate) wall: Option<Wall>,
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

/// **A spiric's chart image on a torus wall**
/// (`geom_brep::SpiricImage::Wall`), on the curve's own parameter:
/// `u = u0 + κ·atan2(f(t), d)`, `v = v0 + σ·t`, with
/// `f(t) = √((R + r·cos t)² − d²)` and `R`, `r`, `d` the carrier's.
/// `σ` and `κ` are exactly `±1`: `σ` the image's sense, `κ` it times
/// the wall torus's sign.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Wall {
    pub(crate) u0: Interval,
    pub(crate) v0: Interval,
    pub(crate) sigma: f64,
    pub(crate) kappa: f64,
    pub(crate) major: Interval,
    pub(crate) minor: Interval,
    pub(crate) offset: Interval,
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
    /// its lift ([`crate::pcurves::Lift::read`]) is the face's boundary in
    /// the chart, its rows meeting at every joint and, at a pole or an
    /// apex, joined by a run along the singular latitude that lies
    /// between their ends. So on a periodic axis the hull is on ONE
    /// sheet, not wrapped. A row that is a coordinate line whose moving
    /// coordinate is monotone gives its ends, which bound it; a harmonic
    /// or spiric wall row its enclosure over its span, so the hull may be
    /// a superset, which holds the face. It is the FACE's extent only
    /// where the loop does not wrap a periodic axis. `None` where the loop
    /// has no lift or a row has no closed form. Read only through
    /// [`face_extent`], which reads it against the face's own lines.
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
    /// How its loops wind in its chart.
    fn orientation(&self) -> PolygonOrientation {
        PolygonOrientation::of_face(self.sense)
    }

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
                    .filter(|&i| x.edges.get(i).is_some_and(seam))
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
                Some((carrier, span)) => !ball.clear_of_arc_as(carrier, *span, band),
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
/// The torus that is not decided a ring, carrying the convention's
/// refusal ([`geom_brep::TorusRefusal`]): a half decided outside it, or
/// a half the band cannot decide, which escalates as its own margin and
/// is never read as the definite miss. A free-form carrier, which no
/// kind makes injective: its own pairs go to the monotone arm.
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
        } => geom_brep::TorusRefusal::require_at(*major_radius, *minor_radius, RING_TORUS, band)
            .map(|_| ())
            .map_err(|refusal| EngineRefusal::TorusNotRing {
                face,
                side: Which::First,
                refusal: Box::new(refusal),
            }),
        Surface::Nurbs(_) | Surface::Approx(_) => Err(EngineRefusal::Unsupported {
            carrier: "a free-form face, which no carrier kind makes injective",
            face,
            side: Which::First,
        }),
        Surface::Spindle { .. } => Err(EngineRefusal::Unsupported {
            carrier: "a spindle sheet's face, which has no injectivity lane yet",
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
    v.checked_direction().ok().map(|(d, _)| d)
}

fn perp(v: Vec3<f64>, t: Vec3<f64>) -> Vec3<f64> {
    v - t * v.dot(t)
}

/// The unit bisector across two headings' parts perpendicular to `t`:
/// `m·hx⊥ > 0 > m·hy⊥` at any dihedral between them off 0, whatever
/// the headings' along-`t` parts. Only a separator: [`sign_on`] decides
/// both faces' signs against it.
fn bisector(hx: Vec3<f64>, hy: Vec3<f64>, t: Vec3<f64>) -> Option<Vec3<f64>> {
    unit(unit(perp(hx, t))? - unit(perp(hy, t))?)
}

fn interval_sign_as(x: Interval, row: &'static str, band: Band) -> Option<Sign> {
    match decide(row, Margin::of(x), band) {
        Ok(s @ (Sign::Positive | Sign::Negative)) => Some(s),
        _ => None,
    }
}

fn is_zero(x: Interval, band: Band) -> bool {
    matches!(
        decide(SELF_INTERSECTION_ON_SHARED, Margin::of(x), band),
        Ok(Sign::Zero)
    )
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

/// **Whether a chart axis is periodic**: the azimuth of a cylinder, cone,
/// sphere or spindle sheet, and a torus's both angles; no axis of a
/// plane or a free-form chart.
pub(crate) fn periodic<T: Real>(surface: &Surface<T>, axis: Axis) -> bool {
    match surface {
        Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Sphere { .. }
        | Surface::Spindle { .. } => axis == Axis::U,
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

/// Which of a presentation's sides the separator vanishes on, each with
/// the shared cells it lies on.
#[derive(Clone, Debug)]
enum Zero {
    /// None: the separator's sign is read off its value.
    None,
    /// One side.
    Side(Side, Along),
    /// One side that runs on past the shared cell, over an edge the
    /// other face does not share ([`partial_zero`]); read only where
    /// [`sign_on`] admits it.
    Past(Side, Along),
    /// Two adjacent sides.
    Corner((Side, Along), (Side, Along)),
    /// Only the corner point of two adjacent sides, a shared vertex.
    Point(Side, Side, VertexKey),
    /// A curve across the rectangle, the face on this side's side of it
    /// ([`present_curved`]).
    Curve(Side, Box<Curved>),
}

/// **The cells a zero side or curve lies on**: the edges along it, by
/// key, so both faces name them alike, or the shared vertex a collapsed
/// side is. They are shared, but for a curve read on through a vertex
/// the pair shares alone, which names the arc it is read on along
/// ([`Reading::Conic`]); and whether the side's whole chart line is those
/// edges' own line or circle, so a separator holding their carriers
/// holds all of it, past the edges too. A side on one edge carries its
/// parameters on that edge's carrier where its presentation reads them
/// ([`Along::over`]): the graph separator, which vanishes on its edge's
/// carrier over a piece alone, reads them ([`sign_on`]).
#[derive(Clone, Debug, Default)]
struct Along {
    edges: Vec<EdgeKey>,
    vertex: Option<VertexKey>,
    whole: bool,
    /// Where on its one edge's carrier the zero lies, read for a whole
    /// side or curve only.
    params: Option<Params>,
    /// The side lies on its edges: they cover its whole chart line in the
    /// presentation's rectangle, with no run-on past a vertex
    /// ([`present_chart`]), or it is a plane segment's checked span.
    on_edges: bool,
}

/// **Where a zero lies on its edge's carrier**: its parameters, and
/// whether they are read modulo whole turns (a circle's or a curve's on
/// a conic).
#[derive(Clone, Copy, Debug)]
struct Params {
    t: (f64, f64),
    periodic: bool,
}

impl Params {
    /// The parameters, a periodic one's moved by the whole turns (none,
    /// or one either way) that leave them nearest `piece`.
    fn nearest(self, (lo, hi): (f64, f64)) -> Option<(f64, f64)> {
        let turns_of: &[f64] = if self.periodic {
            &[-1.0, 0.0, 1.0]
        } else {
            &[0.0]
        };
        turns_of
            .iter()
            .map(|&k| {
                let s = iv(self.t) + turns(k);
                (s.lo(), s.hi())
            })
            .filter(|(a, b)| a.is_finite() && b.is_finite())
            .min_by(|x, y| {
                let over = |(a, b): (f64, f64)| (lo - a).max(b - hi).max(0.0);
                over(*x).total_cmp(&over(*y))
            })
    }
}

impl Along {
    /// The edges `edges` of `b`, along a coordinate line of `surface`'s
    /// chart (a plane's affine one included). On an analytic carrier that
    /// line's image is a line or a circle, so an edge on it whose carrier
    /// is a line or a circle has that line or circle as its own.
    fn of(surface: &Surface<Interval>, b: &Boundary, edges: &[usize]) -> Self {
        let analytic = matches!(
            surface,
            Surface::Plane { .. }
                | Surface::Cylinder { .. }
                | Surface::Cone { .. }
                | Surface::Sphere { .. }
                | Surface::Torus { .. }
        );
        let rims: Vec<&Rim> = edges.iter().filter_map(|&i| b.edges.get(i)).collect();
        let own = |r: &&Rim| matches!(r.arc, Some((Curve3::Line(_) | Curve3::Circle(_), _)));
        Self {
            edges: rims.iter().map(|r| r.key).collect(),
            vertex: None,
            whole: analytic && rims.len() == edges.len() && rims.iter().all(own),
            params: None,
            on_edges: false,
        }
    }

    /// A collapsed side: its whole image is the vertex.
    fn at(vertex: VertexKey) -> Self {
        Self {
            edges: Vec::new(),
            vertex: Some(vertex),
            whole: true,
            params: None,
            on_edges: false,
        }
    }

    /// The side on `b`'s one edge `edge`, whether it lies on the edge,
    /// and, where it is whole, its parameters on the edge's carrier,
    /// which `params` is asked for only then.
    fn over(
        surface: &Surface<Interval>,
        b: &Boundary,
        edge: usize,
        on_edges: bool,
        params: impl FnOnce() -> Option<(f64, f64)>,
    ) -> Self {
        let of = Self::of(surface, b, &[edge]);
        let periodic = b
            .edges
            .get(edge)
            .is_some_and(|r| matches!(r.arc, Some((Curve3::Circle(_), _))));
        Self {
            params: if of.whole {
                params().map(|t| Params { t, periodic })
            } else {
                None
            },
            on_edges,
            ..of
        }
    }

    /// **Whether the graph separator over `piece` is zero along this
    /// side or curve**: the one rule both are read by. The graph vanishes
    /// on its edge's carrier over the piece and nowhere else
    /// ([`graph_of`]), so a whole zero is zero where its parameters lie
    /// in the piece. A zero whose chart line is not its edge's own line
    /// or circle has none read: it is zero where it lies on its edge, its
    /// points the edge's in the presentation's image, which the piece
    /// holds up to the rounding of the chart against the stored carrier
    /// (`edge-piece-misses-a-chart-sides-stored-carrier-by-the-charts-rounding`).
    /// A whole zero whose parameters were not read answers nothing.
    fn under_graph(&self, piece: (f64, f64)) -> bool {
        match self.params {
            Some(params) => params
                .nearest(piece)
                .is_some_and(|(a, b)| piece.0 <= a && b <= piece.1),
            None => !self.whole && self.on_edges,
        }
    }
}

/// **A chart side's parameters on its edge**: the side over `range` on
/// the moving axis of `rim`'s one coordinate-line use, mapped by the
/// affine relation its image's ends and its span's ends fix. On an
/// analytic chart a coordinate line's image is a line or a circle, and
/// an edge on it whose carrier is that line or circle (a whole side,
/// [`Along::of`]) has a parameter affine in the moving coordinate: its
/// pcurve is an iso line or arc, or a harmonic whose moving channel has
/// no cosine or sine term, which is checked. On a periodic axis the
/// image is first moved by the whole turns that bring it nearest the
/// range, which may lie any number of turns off the image's sheet; that
/// puts the parameters on the span's sheet, and [`Params::nearest`] then
/// meets a piece unwrapped past the span by a turn. `None` for anything
/// else.
fn iso_params(surface: &Surface<Interval>, rim: &Rim, range: (f64, f64)) -> Option<(f64, f64)> {
    let (_, (t0, t1)) = rim.arc.as_ref()?;
    let [u] = rim.uses.as_slice() else {
        return None;
    };
    let iso = u.iso?;
    let moving = iso.held.other().index();
    if let Some(h) = u.harmonic
        && !(exactly(h.pa[moving], 0.0) && exactly(h.pb[moving], 0.0))
    {
        return None;
    }
    let [p, q] = u.image?;
    let (i0, i1) = (iso.moving(p), iso.moving(q));
    let (i0, i1) = if periodic(surface, iso.held.other()) {
        let k = turns(turns_toward(
            hull(i0, i1),
            geom::mid_param(range.0, range.1),
        ));
        (i0 + k, i1 + k)
    } else {
        (i0, i1)
    };
    let t = *t0 + (*t1 - *t0) * ((iv(range) - i0) / (i1 - i0));
    (t.is_certified() && t.lo().is_finite() && t.hi().is_finite()).then(|| (t.lo(), t.hi()))
}

/// **A segment's parameters**: the edge over `range` of an affine
/// coordinate that is `ta` at 0 and `tb` at 1, its carrier a line.
fn segment_params(ta: Interval, tb: Interval, range: (f64, f64)) -> Option<(f64, f64)> {
    let t = ta + (tb - ta) * iv(range);
    (t.is_certified() && t.lo().is_finite() && t.hi().is_finite()).then(|| (t.lo(), t.hi()))
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
            &(Zero::Side(side, _) | Zero::Past(side, _)) => {
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
    if !delta.is_certified() {
        return None;
    }
    match (delta.lo() > 0.0, delta.hi() < 0.0) {
        (true, _) => Some(1.0),
        (_, true) => Some(-1.0),
        _ => None,
    }
}

/// Which side of an iso edge's line its face lies on, as the sign of the
/// held coordinate: `left` is the traversal's left in the chart, and the
/// face lies there or opposite as its loop's `orientation` says.
fn face_side(u: &Use, iso: Iso, orientation: PolygonOrientation) -> Option<f64> {
    let forward = forward(u, iso)?;
    let traversal = if u.plus { forward } else { -forward };
    let mut tau = [0.0; 2];
    tau[iso.held.other().index()] = traversal;
    Some(orientation.to_interior(left_turn(tau)[iso.held.index()]))
}

/// The chart's quarter turn to the left, `(x, y)` to `(−y, x)`: the
/// left of a direction in the `(u, v)` plane.
fn left_turn<T: core::ops::Neg<Output = T>>([x, y]: [T; 2]) -> [T; 2] {
    [-y, x]
}

/// A line edge's chart traversal `tau` turned a quarter toward its face,
/// on the side its loop's `orientation` puts the face.
fn toward_face(tau: [Interval; 2], orientation: PolygonOrientation) -> [Interval; 2] {
    left_turn(tau).map(|x| orientation.to_interior(x))
}

/// The moving coordinate's range along a use, outer hull of its ends;
/// `None` where an end does not certify.
fn along(u: &Use, iso: Iso) -> Option<Interval> {
    let [p, q] = u.image?;
    let (a, b) = (iso.moving(p), iso.moving(q));
    if !(a.is_certified() && b.is_certified()) {
        return None;
    }
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
///
/// A sphere face the stored chart does not present is read in a pose
/// ([`present_posed`]), held in `poses`; a pose's `Local` stands for the
/// stored chart's `Structural`, as a smaller cell may be read in it.
fn present<'a: 'h, 'h>(
    piece: &Piece<'a>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
    poses: &'h Poses,
) -> Result<Presentation<'h>, Unpresented> {
    match piece.surface {
        Surface::Plane { .. } => present_plane(piece, near_edges, near_vertices, band),
        Surface::Nurbs(_) => present_chart(piece, near_edges, near_vertices, band),
        Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Sphere { .. }
        // One arm for the quadrics, though only a sphere has poses
        // ([`poses_of`] finds none on any other kind, and
        // [`present_posed`] answers `None`): the stored chart's reading
        // stands for every other kind.
        | Surface::Torus { .. } => match present_chart(piece, near_edges, near_vertices, band) {
            Err(stored @ (Unpresented::Structural | Unpresented::Local)) => {
                match present_posed(piece, near_edges, near_vertices, band, poses) {
                    Some(Ok(read)) => Ok(read),
                    Some(Err(Unpresented::Local)) => Err(Unpresented::Local),
                    _ => Err(stored),
                }
            }
            read => read,
        },
        Surface::Spindle { .. } | Surface::Approx(_) => Err(Unpresented::Structural),
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

/// **The slack the face's extent is read with**: [`few_ulps`] of the
/// largest of `values` and a whole turn, the rounding of a value placed
/// by whole turns. [`face_extent`] and [`sheets`] read the extent at
/// this scale; the touch, [`vacuous`] and the zero-width guard in
/// [`present_chart`] read [`few_ulps`] of the value read, the scale of
/// the cell's own side.
fn chart_slack(values: &[f64]) -> f64 {
    few_ulps(values.iter().fold(TURN, |m, x| m.max(x.abs())))
}

/// **Whether an edge is a seam of the face**: the face holds it twice,
/// once each side, so it is no boundary.
fn seam(e: &Rim) -> bool {
    e.uses.len() > 1
}

/// The face's extent as [`face_extent`] reads it.
enum Extent {
    /// The face has no extent ([`Boundary::extent`] is `None`): nothing on
    /// a periodic axis is placed.
    Unknown,
    /// The outer loop wraps a periodic axis, so it bounds nothing: a band
    /// lies between it and a ring, a polar cap between it and the pole.
    /// Its hull says nothing of the face on either axis.
    Wraps,
    /// The face's extent, `[u, v]`, read against its lines and nappe.
    Known([(f64, f64); 2]),
}

/// **The face's extent, read against the face's own lines and nappe**
/// ([`Boundary::extent`]); the one reader of that field.
///
/// **A face lies within the hull of its boundary**, so every line of a
/// boundary edge, placed by [`placed`] on the extent's sheet, meets the
/// extent and has room in it, more than [`chart_slack`], on the line's
/// face side. A line at the extent's high end with the face above it,
/// or at its low end with the face below, or a line apart from the
/// extent, contradicts it. On a cone, the face's nappe ([`nappe`], read
/// off its vertices) holds the extent's `v`. A contradiction is the
/// face's fact, the same in every cell, so it refuses `Structural`.
///
/// Every edge is read, a seam's two uses too. On a periodic axis a seam
/// lies on the outer loop's chain at both of its values, a turn apart, so
/// a face with one has an extent that wraps and is not read here. On an
/// axis that is not periodic the face lies on both sides of a seam, so
/// each use has room on its face side.
///
/// **What this does not read**: a line whose face side or value does not
/// certify, every row that is not a coordinate line, and a cone face's
/// nappe where it is not decided or where the extent wraps or is unknown
/// (this returns before it). On an axis where the face's boundary holds
/// no line read here, the extent is taken from the stored lift unchecked.
fn face_extent(
    surface: &Surface<Interval>,
    b: &Boundary,
    band: Band,
) -> Result<Extent, Unpresented> {
    let Some(extent) = b.extent else {
        return Ok(Extent::Unknown);
    };
    if [Axis::U, Axis::V].into_iter().any(|axis| {
        let (lo, hi) = extent[axis.index()];
        periodic(surface, axis) && whole_turn(lo, hi)
    }) {
        return Ok(Extent::Wraps);
    }
    let sheet = extent.map(|(lo, hi)| Sheet::Copy(lo, hi));
    let lines = b
        .edges
        .iter()
        .filter(|e| e.arc.is_some())
        .flat_map(|e| &e.uses)
        .all(|u| {
            let Some(iso) = u.iso.filter(|iso| iso.at.is_certified()) else {
                return true;
            };
            let Some(sigma) = face_side(u, iso, b.orientation()) else {
                return true;
            };
            let (lo, hi) = extent[iso.held.index()];
            let Some(line) = placed(surface, iso, sheet, (lo, hi)) else {
                return true;
            };
            let slack = chart_slack(&[lo, hi]);
            let meets = lo - slack <= line.hi() && line.lo() <= hi + slack;
            meets
                && if sigma > 0.0 {
                    line.lo() < hi - slack
                } else {
                    lo + slack < line.hi()
                }
        });
    let on_nappe = face_nappe(surface, b, band).is_none_or(|up| {
        let (lo, hi) = extent[Axis::V.index()];
        let slack = chart_slack(&[lo, hi]);
        if up {
            lo >= -slack && hi > slack
        } else {
            hi <= slack && lo < -slack
        }
    });
    if lines && on_nappe {
        Ok(Extent::Known(extent))
    } else {
        Err(Unpresented::Structural)
    }
}

/// **A cone face's nappe** ([`nappe`]); `None` on any other surface, or
/// where the nappe is not decided.
fn face_nappe(surface: &Surface<Interval>, b: &Boundary, band: Band) -> Option<bool> {
    match surface {
        Surface::Cone { apex, axis, .. } => nappe(b, *apex, *axis, band),
        _ => None,
    }
}

/// What [`sheets`] reads of a cell.
enum Sheets {
    /// The cell meets no copy of the face's extent: it holds none of the
    /// face, read [`Presentation::Vacuous`].
    Missed,
    /// The face's sheet on each axis, `[u, v]`.
    On([Sheet; 2]),
}

/// **The face's sheet in `cell`, per axis**: on a periodic axis the copy
/// of its own extent ([`face_extent`]) moved by whole turns that the cell
/// meets, so every line of it is placed on the same sheet ([`placed`])
/// whatever order its edges are read in. A face whose extent is unknown
/// leaves the sheet unknown: a line on that axis then has no placement
/// and the face is not presented there. On an axis that is not periodic
/// the extent still says what the cell holds.
///
/// Refuses `Local` where the cell meets two or more copies (the face
/// enters it more than once, which one rectangle does not present, and a
/// smaller cell meets fewer), and `Structural` where the cell's bounds
/// are not finite (no turn count places it, nor any of its children) or
/// [`face_extent`] refuses.
///
/// A cell MEETS a copy where their overlap is wider than [`chart_slack`]
/// of the values involved, the rounding of a copy moved by whole turns.
/// **That is an ε concession**: a cell overlapping a copy by less is
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
    b: &Boundary,
    cell: ((f64, f64), (f64, f64)),
    band: Band,
) -> Result<Sheets, Unpresented> {
    let mut on = [Sheet::Free; 2];
    let extent = match face_extent(surface, b, band)? {
        Extent::Known(extent) => extent,
        Extent::Wraps => return Ok(Sheets::On(on)),
        Extent::Unknown => {
            for axis in [Axis::U, Axis::V] {
                if periodic(surface, axis) {
                    on[axis.index()] = Sheet::Unknown;
                }
            }
            return Ok(Sheets::On(on));
        }
    };
    for axis in [Axis::U, Axis::V] {
        let (lo, hi) = extent[axis.index()];
        let (clo, chi) = axis.of(cell);
        let slack = chart_slack(&[lo, hi, clo, chi]);
        let meets = |(a, b): (f64, f64)| b.min(chi) - a.max(clo) > slack;
        if !periodic(surface, axis) {
            if !meets((lo, hi)) {
                return Ok(Sheets::Missed);
            }
            on[axis.index()] = Sheet::Within(lo, hi);
            continue;
        }
        let (k0, k1) = (((clo - hi) / TURN).floor(), ((chi - lo) / TURN).ceil());
        if !(k0.is_finite() && k1.is_finite()) {
            return Err(Unpresented::Structural);
        }
        let copies: Vec<(f64, f64)> = core::iter::successors(Some(k0), |k| Some(k + 1.0))
            .take_while(|&k| k <= k1)
            .map(|k| (lo + k * TURN, hi + k * TURN))
            .filter(|&c| meets(c))
            .take(2)
            .collect();
        match copies.as_slice() {
            [] => return Ok(Sheets::Missed),
            [one] => on[axis.index()] = Sheet::Copy(one.0, one.1),
            _ => return Err(Unpresented::Local),
        }
    }
    Ok(Sheets::On(on))
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
    posed: bool,
) -> bool {
    outside.iter().any(|&(side, line, ei)| {
        let (lo, hi) = side.axis().of(cell);
        // In a pose a line is an enclosure of its edge, so it touches
        // only where every point of it does, its far end (the end the
        // comparison below binds); a stored line is a point.
        let (near, far) = (line.lo(), line.hi());
        let (near, far) = match side {
            Side::XLo | Side::YLo => (near, far),
            Side::XHi | Side::YHi => (far, near),
        };
        let x = if posed { far } else { near };
        let touches = match side {
            Side::XLo | Side::YLo => x <= hi + few_ulps(hi),
            Side::XHi | Side::YHi => x >= lo - few_ulps(lo),
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

/// **The rectangle's sides are the face's own**, for a face that is not
/// a whole patch: each clip's side is covered by its edges, and each
/// corner two clips on different axes make is convex ([`convex`]). An
/// edge that ends inside the cell leaves the face free to wrap round
/// its end, and the clipped quadrant is the face only where the corner
/// is convex.
///
/// **A side's edges run on past a corner** ([`run_on`]): an edge that
/// ends at a vertex it shares with a clip on the other axis covers the
/// side out to that clip's line's far bound. The run-on is sound only
/// with the other two checks, which is why all three are read here and
/// nowhere else: that the other line bounds the face there is that
/// line's own coverage, and that the corner is the face's is
/// [`convex`].
fn sides_are_the_faces(
    b: &Boundary,
    merged: &[Clip],
    rect: ((f64, f64), (f64, f64)),
    periodic: impl Fn(Axis) -> bool,
) -> bool {
    let covers = |c: &Clip| {
        let axis = c.side.axis().other();
        let periodic = periodic(axis);
        run_on(b, c, merged, periodic)
            .is_some_and(|ranges| covered(axis.of(rect), &ranges, periodic))
    };
    let other_axis = |c: &Clip, d: &Clip| {
        c.side.axis() != d.side.axis() && c.collapsed.is_none() && d.collapsed.is_none()
    };
    merged.iter().all(|c| c.collapsed.is_some() || covers(c))
        && merged.iter().enumerate().all(|(i, c)| {
            merged
                .iter()
                .skip(i + 1)
                .all(|d| !other_axis(c, d) || convex(b, c, d))
        })
}

/// **The vertices two clips' edges share**: the corners they make, one
/// at each such vertex (two edges that meet twice round a periodic
/// chart make two).
fn corners(b: &Boundary, c: &Clip, d: &Clip) -> BTreeSet<VertexKey> {
    let ends = |c: &Clip| -> BTreeSet<VertexKey> {
        c.edges
            .iter()
            .filter_map(|&i| b.edges.get(i))
            .flat_map(|e| e.ends)
            .collect()
    };
    ends(c).intersection(&ends(d)).copied().collect()
}

/// **The ranges a clip's side is its edges' over, run on past its
/// corners**: its edges' ([`edge_ranges`]), each run on, from an end at
/// a corner it makes with a clip on the other axis ([`corners`]), to
/// that clip's line's far bound. The rectangle's side ends at that
/// bound, while the edge ends at the corner, whose coordinate the
/// line's enclosure and the edge end's both hold: two enclosures of one
/// point, which no cell narrows. Past the edge end's own enclosure is
/// past the corner, on the other line's non-face side, where the face
/// has no point to wrap round the end into. Read only by
/// [`sides_are_the_faces`], beside the checks that make that so.
fn run_on(b: &Boundary, c: &Clip, merged: &[Clip], periodic: bool) -> Option<Vec<Interval>> {
    let held = c.side.axis();
    let mut out = edge_ranges(b, &c.edges, held)?;
    for d in merged
        .iter()
        .filter(|d| d.side.axis() != held && d.collapsed.is_none())
    {
        let at = corners(b, c, d);
        for &ei in &c.edges {
            let e = b.edges.get(ei)?;
            for (k, _) in e.ends.iter().enumerate().filter(|(_, v)| at.contains(v)) {
                for u in &e.uses {
                    let iso = u.iso?;
                    if iso.held != held || iso.full {
                        continue;
                    }
                    let end = iso.moving(u.image?[k]);
                    let end = if periodic {
                        wrap(end, geom::mid_param(d.at.lo(), d.at.hi()))
                    } else {
                        end
                    };
                    out.push(match d.side {
                        Side::XHi | Side::YHi => {
                            Interval::from_bounds(end.lo(), end.hi().max(d.at.hi()))
                        }
                        Side::XLo | Side::YLo => {
                            Interval::from_bounds(end.lo().min(d.at.lo()), end.hi())
                        }
                    });
                }
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
        .filter_map(|&q| interval_sign_as((q - apex).dot(axis), SELF_INTERSECTION_NAPPE, band));
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
///   when it misses the face's extent ([`sheets`]), or every near line
///   puts it on its non-face side and one of them touches it along an
///   edge that covers its side ([`vacuous`]), or it lies on a cone's
///   other nappe, or its rectangle — read only once every side and
///   corner is proven the face's own — is a sliver beside a shared line.
/// - A face whose lines or nappe contradict its extent ([`face_extent`])
///   is never presented, and a rectangle its own clips leave with no
///   width on an axis is not presented in that cell: neither is a
///   sliver.
fn present_chart<'a>(
    piece: &Piece<'a>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> Result<Presentation<'a>, Unpresented> {
    present_chart_in(piece, near_edges, near_vertices, band, false)
}

/// [`present_chart`], in the stored chart or, `posed`, in a pose of a
/// sphere ([`present_posed`]): there the cell is narrower than half a
/// turn, its sheet is the posed extent's ([`posed_extent`]), a side
/// carries no parameters, and no rectangle is read as a sliver.
fn present_chart_in<'a>(
    piece: &Piece<'a>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
    posed: bool,
) -> Result<Presentation<'a>, Unpresented> {
    let b = piece.boundary;
    let cell = (piece.u, piece.v);
    let sheet = if piece.whole {
        [Sheet::Free; 2]
    } else if posed {
        // A pose's extent is the pose's reading ([`posed_extent`]), not the
        // face's fact: where it is not granted, or its lines contradict it
        // ([`face_extent`]), the cell is placed by itself, under half a
        // turn wide ([`posed_cell`]), never refused.
        match b.extent.map(|_| sheets(piece.surface, b, cell, band)) {
            Some(Ok(Sheets::On(s))) => s,
            Some(Ok(Sheets::Missed)) => return Ok(Presentation::Vacuous),
            _ => [Sheet::Free; 2],
        }
    } else {
        match sheets(piece.surface, b, cell, band)? {
            Sheets::On(s) => s,
            Sheets::Missed => return Ok(Presentation::Vacuous),
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
            .filter(|&i| b.edges.get(i).is_some_and(|e| e.arc.is_some() && !seam(e)))
            .collect();
        if let Some(read) = curved_walls(piece, &walls, sheet, near_edges, near_vertices, band) {
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
        if !piece.whole && seam(e) {
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
            let Some(sigma) = face_side(u, iso, b.orientation()) else {
                return Err(unread(ei, near_edges));
            };
            // A line on a periodic axis whose sheet is unknown has no
            // place, at any depth.
            let Some(line) = placed(piece.surface, iso, sheet, (lo, hi)) else {
                return Err(Unpresented::Structural);
            };
            // Every read below, and the merge's folds, see only a
            // certified line.
            if !line.is_certified() {
                return Err(Unpresented::Local);
            }
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
        return if vacuous(b, piece.surface, cell, &outside, posed) {
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
    // A cone's apex is a collapsed side of its chart.
    if let Surface::Cone { apex, axis, .. } = piece.surface {
        for &v in &piece.local.vertices {
            let Some(&p) = b.vertices.get(&v) else {
                continue;
            };
            if !matches!(
                decide(SELF_INTERSECTION_APEX, Margin::of((p - *apex).norm()), band),
                Ok(Sign::Zero)
            ) {
                continue;
            }
            // A face with an apex vertex in reach and no one nappe is not
            // presented in any cell that holds that vertex; a cell away
            // from the apex reads it as any face.
            let Some(lo) = nappe(b, *apex, *axis, band) else {
                return Err(Unpresented::Structural);
            };
            // The cell on the other nappe holds only the apex of the
            // face. This is read only where the face's extent wraps or is
            // unknown: a known one is on the face's nappe ([`face_extent`]),
            // and a cell on the other misses it in [`sheets`] first.
            let (cv0, cv1) = piece.v;
            if (lo && cv1 <= 0.0) || (!lo && cv0 >= 0.0) {
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
            // A line on the face's low side that does not hold `0`, in a
            // rectangle that still reaches `0`, lies within the slack of
            // `0` (every line meets the extent, which is on the nappe): the
            // collapse and the line are two readings of one side that do
            // not agree, which a smaller cell does not hold both of.
            match merged.iter_mut().find(|m| m.side == side) {
                Some(m) if m.at.lo() <= 0.0 && 0.0 <= m.at.hi() => m.collapsed = Some(v),
                Some(_) => {
                    return Err(Unpresented::Local);
                }
                None => merged.push(Clip {
                    side,
                    at: at(0.0),
                    edges: Vec::new(),
                    collapsed: Some(v),
                }),
            }
        }
    }
    // A sphere's pole is a collapsed side of its chart, `v = ±π/2`, a
    // bound of its canonical domain. A vertex is at it by the one reading
    // of that incidence.
    if let &Surface::Sphere {
        center,
        radius,
        axis,
        ..
    } = piece.surface
    {
        use geom_brep::chart_singularity::{Singularity, sphere_poles, vertex_at_singular_point};
        let half = Interval::pi() * at(0.5);
        let poles = sphere_poles(
            geom::Line {
                origin: center,
                dir: axis,
            },
            radius,
        );
        for &v in &piece.local.vertices {
            let Some(&p) = b.vertices.get(&v) else {
                continue;
            };
            for (pole, point) in poles {
                if !matches!(vertex_at_singular_point(p, point, band), Ok(true)) {
                    continue;
                }
                let north = pole == Singularity::North;
                let line = if north { half } else { -half };
                if (north && y.1 < line.lo()) || (!north && y.0 > line.hi()) {
                    continue;
                }
                if (north && y.0 > line.hi()) || (!north && y.1 < line.lo()) {
                    return Err(Unpresented::Local);
                }
                if north {
                    y.1 = line.hi();
                } else {
                    y.0 = line.lo();
                }
                let side = Side::of(Axis::V, !north);
                match merged.iter_mut().find(|m| m.side == side) {
                    Some(m) if m.at.lo() <= line.hi() && line.lo() <= m.at.hi() => {
                        m.collapsed = Some(v);
                    }
                    Some(_) => {
                        return Err(Unpresented::Local);
                    }
                    None => merged.push(Clip {
                        side,
                        at: line,
                        edges: Vec::new(),
                        collapsed: Some(v),
                    }),
                }
            }
        }
    }
    // No width on an axis, a negative one included: a low line above a
    // high one is a face that leaves the cell and comes back into it,
    // which no one rectangle presents, and a zero one is the face's clips
    // contradicting one another, as it has interior wherever a line has
    // it on the cell's side; neither is a sliver. A smaller cell holds
    // one of the parts, or neither line. A whole patch keeps its cell.
    // The width is read at the touch's scale, never wider, as
    // `a_filleted_drum_certifies_across_its_fillet` pins.
    let no_width = |(lo, hi): (f64, f64)| hi - lo <= few_ulps(lo).max(few_ulps(hi));
    if !piece.whole && (no_width(x) || no_width(y)) {
        return Err(Unpresented::Local);
    }
    // Every clip's side must be the edges' own, run on past its corners,
    // and every corner convex.
    if !piece.whole && !sides_are_the_faces(b, &merged, (x, y), |a| periodic(piece.surface, a)) {
        return Err(Unpresented::Local);
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
    //
    // A pose reads no sliver: the sliver is frozen, so no new reader
    // adopts it, and a posed rectangle is read or refused.
    if !posed {
        for c in merged.iter() {
            let on_shared = c.edges.iter().all(|e| near_edges.contains(e)) && !c.edges.is_empty();
            if !on_shared {
                continue;
            }
            let (across, lever) = match c.side.axis() {
                Axis::U => (x.1 - x.0, rect_jet(piece.surface, x, y).0),
                Axis::V => (y.1 - y.0, rect_jet(piece.surface, x, y).1),
            };
            let width = at(across) * lever.norm();
            if matches!(
                decide(SELF_INTERSECTION_SLIVER, Margin::of(width), band),
                Ok(Sign::Zero)
            ) {
                return Ok(Presentation::Vacuous);
            }
        }
    }
    let zero = match zero_of(piece.surface, b, &merged, near_edges, near_vertices) {
        Ok(Zero::Side(side, a)) if a.vertex.is_none() => {
            let held = side.axis();
            let clip = merged.iter().find(|c| c.side == side);
            let range = held.other().of((x, y));
            // On its edges only where they cover the side without the
            // run-on past a vertex ([`run_on`]), whose far bound is the
            // other line's enclosure, off the edge's span.
            let on_edges = clip.is_some_and(|c| {
                edge_ranges(b, &c.edges, held).is_some_and(|ranges| {
                    covered(range, &ranges, periodic(piece.surface, held.other()))
                })
            });
            match clip.map(|c| c.edges.as_slice()) {
                Some(&[e]) => Zero::Side(
                    side,
                    Along::over(piece.surface, b, e, on_edges, || {
                        if posed {
                            return None;
                        }
                        iso_params(piece.surface, b.edges.get(e)?, range)
                    }),
                ),
                _ => Zero::Side(side, Along { on_edges, ..a }),
            }
        }
        Ok(zero) => zero,
        Err(e) => match partial_zero(piece.surface, b, &merged, near_edges, near_vertices) {
            Some((side, along)) => Zero::Past(side, along),
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

// ------------------------------------------------- a sphere, re-posed

/// A rectangle of a chart, `(u, v)`.
type ChartRect = ((f64, f64), (f64, f64));

/// **The frame a sphere is posed in about a circle** ([`present_posed`]):
/// about the sphere's centre `c`, the circle's axis and its seam's part
/// across it, each normalized. The vectors are enclosures of the exact
/// unit frame the stored ones normalize to, so every reading in the pose
/// holds for that exact chart of the sphere. `None` where either does not
/// normalize.
fn pose_frame(
    c: Point3<Interval>,
    axis: Vec3<Interval>,
    u_ref: Vec3<Interval>,
) -> Option<geom::Frame<Interval>> {
    let unit = |w: Vec3<Interval>| {
        let n = w.norm();
        (n.is_certified() && n.lo() > 0.0).then(|| w * (at(1.0) / n))
    };
    let a = unit(axis)?;
    let x = unit(u_ref - a * u_ref.dot(a))?;
    Some(geom::Frame::new(c, a, x))
}

/// The sphere of `radius` about `frame`'s origin, charted in it.
fn posed_sphere(frame: &geom::Frame<Interval>, radius: Interval) -> Surface<Interval> {
    Surface::Sphere {
        center: frame.origin,
        radius,
        axis: frame.axis,
        u_ref: frame.u_ref,
    }
}

/// **An edge's use, lifted into a pose** ([`present_posed`]): the image
/// the mint would store of its carrier on the posed sphere
/// ([`geom_brep::chart_pcurve_over`]), read as [`crate::clearance`]'s
/// boundary reads a stored row ([`crate::clearance::chart_use`]). On a
/// sphere a circle is a parallel of the chart about its own axis, and a
/// great circle through that axis a meridian, which the mint stores as
/// coordinate lines by variant; any other circle's image is the projected
/// one, which no presentation reads as a line. **A pose reads circles
/// only**, by this check: a sphere holds no other conic, and a side a
/// pose reads is then a line or circle's, which is what
/// [`Along::under_graph`]'s reading of a side with no parameters rests on.
/// A seam's two uses are lifted alike: a seam of the stored chart is no
/// boundary of the face in any chart, and [`present_chart`] passes it by.
fn posed_use(surface: &Surface<Interval>, rim: &Rim, u: &Use, band: Band) -> Use {
    let none = Use {
        plus: u.plus,
        image: None,
        iso: None,
        harmonic: None,
        wall: None,
    };
    let Some((carrier, (t0, t1))) = rim.arc.as_ref().filter(|(c, _)| c.as_circle().is_some())
    else {
        return none;
    };
    let got = geom_brep::chart_pcurve_over(carrier, *t0, *t1, surface, band);
    got.ok()
        .and_then(|image| {
            crate::clearance::chart_use(&image, (*t0, *t1), u.plus, rim.ends[0] == rim.ends[1])
        })
        .unwrap_or(none)
}

/// **A face's boundary lifted into a pose**: each edge as
/// [`posed_use`] reads it on `surface`, the vertices as they are, and its
/// extent in the pose where one is granted ([`posed_extent`]).
fn posed_boundary(
    frame: &geom::Frame<Interval>,
    surface: &Surface<Interval>,
    radius: Interval,
    b: &Boundary,
    band: Band,
) -> Boundary {
    Boundary {
        sense: b.sense,
        edges: b
            .edges
            .iter()
            .map(|e| Rim {
                key: e.key,
                arc: e.arc.clone(),
                ends: e.ends,
                uses: e
                    .uses
                    .iter()
                    .map(|u| posed_use(surface, e, u, band))
                    .collect(),
            })
            .collect(),
        vertices: b.vertices.clone(),
        extent: posed_extent(frame, radius, b),
    }
}

/// **A face's extent in a pose**, granted only where it is decided:
/// the face lies left of its outer loop about its outward normal (the
/// stored extent's premise, [`Boundary::extent`]), and a pose has the
/// stored chart's orientation, as every sphere chart's normal is outward;
/// so where the loop's lift into the pose closes on one sheet and runs the
/// way the face's sense says, the face is the lift's bounded side and lies
/// within its hull. Read here, not assumed: in the stored chart that is
/// the B-rep's invariant about how a face is stored, and in a pose a loop
/// that runs the other way is a face holding a pose's pole, or the loop's
/// outside.
///
/// - **The loop**: the face's half-edges must make one cycle, each vertex
///   the start of exactly one; a face with a ring, a seam or a null edge
///   has none here.
/// - **The lift**: each half-edge's carrier, in its traversal order, cut
///   into [`pieces_of`] its span, each piece's box placed in the pose
///   ([`posed_cell`]), so every row has an enclosure — a line's or any
///   other circle's — that holds it; each piece moved by whole turns to
///   meet the one before, and the last meeting the first unmoved, so the
///   lift closes without wrapping the azimuth. A piece whose box holds the
///   pose's axis has no placement, so a loop through a pose's pole has no
///   extent.
/// - **Its orientation**: the signed area of the polygon through the
///   pieces' ends, lifted alike, certified to the sign the face's sense
///   gives with a margin past the pieces' rectangles' areas, which bound
///   how far the loop's area can differ from the polygon's (the loop and
///   each chord lie in their piece's rectangle).
///
/// The hull of the pieces' rectangles is then the extent, a superset of
/// the face. `None` where any of these does not certify: the pose then
/// places by the cell alone, as it does with no extent.
fn posed_extent(
    frame: &geom::Frame<Interval>,
    radius: Interval,
    b: &Boundary,
) -> Option<[(f64, f64); 2]> {
    let mut steps: BTreeMap<VertexKey, (usize, bool)> = BTreeMap::new();
    for (i, e) in b.edges.iter().enumerate() {
        e.arc.as_ref()?;
        for u in &e.uses {
            let start = if u.plus { e.ends[0] } else { e.ends[1] };
            if steps.insert(start, (i, u.plus)).is_some() {
                return None;
            }
        }
    }
    let &first = steps.keys().next()?;
    let mut order = Vec::new();
    let mut at_vertex = first;
    loop {
        let &(i, plus) = steps.get(&at_vertex)?;
        order.push((i, plus));
        let e = b.edges.get(i)?;
        at_vertex = if plus { e.ends[1] } else { e.ends[0] };
        if at_vertex == first {
            break;
        }
        if order.len() > steps.len() {
            return None;
        }
    }
    if order.len() != steps.len() {
        return None;
    }
    let place = |p: Point3<Interval>| posed_cell(frame, radius, p);
    let shift = |(lo, hi): (f64, f64), k: f64| {
        let d = k * TURN;
        (lo + d - few_ulps(lo + d), hi + d + few_ulps(hi + d))
    };
    let meets = |a: (f64, f64), b: (f64, f64)| {
        let slack = chart_slack(&[a.0, a.1, b.0, b.1]);
        a.0 <= b.1 + slack && b.0 <= a.1 + slack
    };
    let mut hull = [(f64::INFINITY, f64::NEG_INFINITY); 2];
    let mut slack_area = 0.0;
    let mut corners: Vec<(Interval, Interval)> = Vec::new();
    let mut first_lon: Option<(f64, f64)> = None;
    let mut prev: Option<(f64, f64)> = None;
    for (i, plus) in order {
        let (carrier, (t0, t1)) = b.edges.get(i)?.arc.as_ref()?;
        let mut pieces = pieces_of((t0.lo(), t1.hi()));
        if !plus {
            pieces.reverse();
        }
        for t in pieces {
            let (lon, lat) = place(carrier.eval(t))?;
            let k = prev.map_or(0.0, |p| ((mid(iv(p)) - mid(iv(lon))) / TURN).round());
            let lon = shift(lon, k);
            if prev.is_some_and(|p| !meets(p, lon)) {
                return None;
            }
            first_lon.get_or_insert(lon);
            prev = Some(lon);
            hull[0] = (hull[0].0.min(lon.0), hull[0].1.max(lon.1));
            hull[1] = (hull[1].0.min(lat.0), hull[1].1.max(lat.1));
            slack_area += (lon.1 - lon.0) * (lat.1 - lat.0);
            let start = if plus { t.lo() } else { t.hi() };
            let (plon, plat) = place(carrier.eval(at(start)))?;
            let plon = shift(plon, ((mid(iv(lon)) - mid(iv(plon))) / TURN).round());
            corners.push((iv(plon), iv(plat)));
        }
    }
    if !(meets(prev?, first_lon?) && hull[0].1 - hull[0].0 < TURN) {
        return None;
    }
    let n = corners.len();
    let twice: Interval = (0..n).fold(at(0.0), |acc, j| {
        let ((x0, y0), (x1, y1)) = (corners[j], corners[(j + 1) % n]);
        acc + x0 * y1 - x1 * y0
    });
    let area = twice * at(0.5);
    let margin = at(slack_area);
    let ccw = matches!(b.orientation(), PolygonOrientation::CounterClockwise);
    let decided = if ccw {
        (area - margin).lo() > 0.0
    } else {
        (area + margin).hi() < 0.0
    };
    (decided && area.is_certified()).then_some(hull)
}

/// **Whether each pose of a sphere face holds `point` in its extent**
/// ([`posed_extent`]): one entry per pose, `None` where the pose grants
/// none. The poses are those about every off-axis circle of the face, a
/// superset of the ones [`poses_of`] tries for any one cell. A test
/// reads it: a face point a granted extent leaves out is a premise of
/// the extent broken.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn posed_extents_holding(
    surface: &Surface<Interval>,
    b: &Boundary,
    point: Point3<Interval>,
) -> Vec<Option<bool>> {
    let Some((center, radius)) = sphere_of(surface) else {
        return Vec::new();
    };
    let mut axes: Vec<Vec3<Interval>> = Vec::new();
    let mut out = Vec::new();
    for e in &b.edges {
        let Some(c) = e.arc.as_ref().and_then(|(c, _)| c.as_circle()) else {
            continue;
        };
        if seam(e) || !is_curved(surface, e) || axes.iter().any(|&a| one_line(a, c.frame.axis)) {
            continue;
        }
        axes.push(c.frame.axis);
        let Some(frame) = pose_frame(center, c.frame.axis, c.frame.u_ref) else {
            continue;
        };
        out.push(posed_extent(&frame, radius, b).map(|[lon, lat]| {
            let Some(((plo, phi), (tlo, thi))) = posed_cell(&frame, radius, point) else {
                return true;
            };
            let meets = |(a0, a1): (f64, f64), (b0, b1): (f64, f64)| a0 <= b1 && b0 <= a1;
            meets((tlo, thi), lat)
                && (-2..=2).any(|k| {
                    let d = f64::from(k) * TURN;
                    meets((plo + d, phi + d), lon)
                })
        }));
    }
    out
}

/// **A cell in a pose**: a rectangle of the pose's chart holding every
/// point of the sphere in the box `image`, which holds the face's part of
/// the cell. On the sphere `Z/r` is the sine of the latitude, so the
/// latitude lies in the arcsine of the box's `Z/r` clamped to `[−1, 1]`;
/// the azimuth is the box's direction in the frame, read about its middle
/// ([`geom::Azimuth::about_middle`]), within a quarter turn of it, so the
/// rectangle is narrower than half a turn. `None` where the box is not
/// decided off the pose's axis, about which every azimuth meets: a
/// smaller cell may be.
fn posed_cell(
    frame: &geom::Frame<Interval>,
    radius: Interval,
    image: Point3<Interval>,
) -> Option<ChartRect> {
    let s = (image - frame.origin).dot(frame.axis) / radius;
    if !s.is_certified() {
        return None;
    }
    let (lo, hi) = (s.lo().max(-1.0), s.hi().min(1.0));
    if lo > hi {
        return None;
    }
    let lat = Interval::from_bounds(lo, hi).asin();
    let lon = frame.azimuth(image).about_middle()?;
    lat.is_certified()
        .then_some(((lon.lo(), lon.hi()), (lat.lo(), lat.hi())))
}

/// One pose a sphere face's cell is read in: the chart, the boundary
/// lifted into it, and the cell there with its ball and local boundary.
struct Posed {
    surface: Surface<Interval>,
    boundary: Boundary,
    x: (f64, f64),
    y: (f64, f64),
    ball: Ball,
    local: Local,
}

/// **The poses one face's cell is read in**, held for as long as a
/// presentation in one of them is read ([`present`]).
#[derive(Default)]
pub(crate) struct Poses(std::cell::OnceCell<PoseSet>);

/// What [`poses_of`] finds: the poses placed, whether a pose was left
/// out for want of a placement, and whether the cell reaches a corner no
/// chart reads ([`unread_corner`]).
#[derive(Default)]
struct PoseSet {
    poses: Vec<Posed>,
    unplaced: bool,
    unread_corner: bool,
}

/// **The poses of a sphere face's cell**: one per circle in its local
/// boundary that the stored chart does not read as a coordinate line,
/// charted about that circle's axis from its seam (one per axis line),
/// each with the cell placed in it ([`posed_cell`]). A pose in which a
/// shared edge (`near_edges`) is no coordinate line is left out: that
/// edge is shared by every smaller cell of the pair, and unread there at
/// any depth. The flag says a pose was left out for want of a placement,
/// which a smaller cell may have.
fn poses_of(
    piece: &Piece<'_>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> PoseSet {
    let Some((center, radius)) = sphere_of(piece.surface) else {
        return PoseSet::default();
    };
    let b = piece.boundary;
    let mut axes: Vec<Vec3<Interval>> = Vec::new();
    let mut frames = Vec::new();
    for &i in &piece.local.edges {
        let Some(e) = b.edges.get(i) else { continue };
        let Some((Curve3::Circle(c), _)) = e.arc.as_ref() else {
            continue;
        };
        if seam(e)
            || !is_curved(piece.surface, e)
            || axes.iter().any(|&a| one_line(a, c.frame.axis))
        {
            continue;
        }
        axes.push(c.frame.axis);
        frames.extend(pose_frame(center, c.frame.axis, c.frame.u_ref));
    }
    let charted: Vec<(geom::Frame<Interval>, Surface<Interval>, Boundary)> = frames
        .into_iter()
        .map(|f| {
            let surface = posed_sphere(&f, radius);
            let boundary = posed_boundary(&f, &surface, radius, b, band);
            (f, surface, boundary)
        })
        .collect();
    let unread_corner = unread_corner(piece, &charted, near_vertices);
    let image = piece.surface.rect_enclosure(piece.u, piece.v);
    let mut unplaced = false;
    let mut out = Vec::new();
    for (frame, surface, boundary) in charted {
        if !lines(&boundary, near_edges) {
            continue;
        }
        let Some(cell) = posed_cell(&frame, radius, image) else {
            unplaced = true;
            continue;
        };
        let (x, y) = cell;
        let m = |(l, h): (f64, f64)| geom::mid_param(l, h);
        let ball = Ball::holding(surface.point_eval(m(x), m(y)), surface.rect_enclosure(x, y));
        let local = Local::of(&boundary, ball, band);
        out.push(Posed {
            surface,
            boundary,
            x,
            y,
            ball,
            local,
        });
    }
    PoseSet {
        poses: out,
        unplaced,
        unread_corner,
    }
}

/// A sphere's centre and radius; `None` for any other surface, which has
/// no pose.
fn sphere_of(surface: &Surface<Interval>) -> Option<(Point3<Interval>, Interval)> {
    match *surface {
        Surface::Sphere { center, radius, .. } => Some((center, radius)),
        _ => None,
    }
}

/// Whether every use of each of the edges `at` is a coordinate line of
/// `b`'s chart.
fn lines(b: &Boundary, at: &[usize]) -> bool {
    at.iter().all(|&i| {
        b.edges
            .get(i)
            .is_some_and(|e| e.uses.iter().all(|u| u.iso.is_some()))
    })
}

/// **A corner no chart reads, at a vertex the pair touches through**
/// (`near_vertices`): two or more edges of the face meet at it in the
/// cell's reach, one is no coordinate line of the stored chart, and in
/// every pose one is no coordinate line either. Every smaller pair
/// touching through the vertex holds that corner, so it is read at no
/// depth, as [`beside_curve`] types an unread corner. Read off the cell's
/// own local boundary, not a pose's wider rectangle, and only where that
/// vertex is the one the pair is near in reach: a split parts two such
/// vertices, and the other may be read, so the pair is not refused for a
/// corner it can be split away from.
fn unread_corner(
    piece: &Piece<'_>,
    charted: &[(geom::Frame<Interval>, Surface<Interval>, Boundary)],
    near_vertices: &[VertexKey],
) -> bool {
    let b = piece.boundary;
    let on_shared = |v: &VertexKey| near_vertices.contains(v);
    let touched: Vec<&VertexKey> = piece
        .local
        .vertices
        .iter()
        .filter(|v| on_shared(v))
        .collect();
    if touched.len() != 1 {
        return false;
    }
    touched.into_iter().any(|v| {
        let at: Vec<usize> = piece
            .local
            .edges
            .iter()
            .copied()
            .filter(|&i| {
                b.edges
                    .get(i)
                    .is_some_and(|e| e.arc.is_some() && !seam(e) && e.ends.contains(v))
            })
            .collect();
        at.len() > 1 && !lines(b, &at) && charted.iter().all(|(_, _, bd)| !lines(bd, &at))
    })
}

/// **Present a sphere face in a pose** where its stored chart does not:
/// a sphere is its own image under every rotation about its centre, so
/// the face is a region of the sphere's chart about any axis, bounded by
/// its edges' images there ([`posed_use`]). A circle on the sphere is a
/// parallel of the chart about its own axis, and a great circle through
/// that axis is a meridian of it; so a face beside a circle off the
/// stored axis is read by [`present_chart`], unchanged, in the chart
/// that makes the circle a coordinate line ([`poses_of`]).
///
/// **Why that is the face's part of the cell.** The cell's image lies in
/// a box, and the pose's rectangle holds every point of the sphere in the
/// box ([`posed_cell`]), so every point of the face in the cell. The
/// rectangle is narrower than half a turn, so it holds one copy of each
/// point and of each line. Its ball and local boundary are the pose's own
/// rectangle's, so every argument [`present_chart`] makes about the
/// boundary in reach is made over the rectangle it presents. A cell
/// holding a pole of the pose is not placed in it.
///
/// **The face's sheet in a pose** is its posed extent's
/// ([`posed_extent`]), the posed boundary's [`Boundary::extent`], read by
/// [`sheets`] as a stored one is: granted only where every row of the
/// outer loop has an enclosure in the pose, the lift closes on one sheet
/// without wrapping the azimuth, and its orientation is certified to
/// make the face the bounded side. A rectangle that misses it holds none
/// of the face (`Vacuous`). Where no extent is granted, or its lines
/// contradict it, the rectangle is the face's sheet (`Sheet::Free`):
/// never refused, never `Structural`, as the pose's extent is the pose's
/// reading, not the face's fact.
///
/// **What a pose does not read**: an edge whose image is no coordinate
/// line there is unread as in any chart, so a face bounded by circles in
/// two planes is read, per cell, in the pose of the one its cell
/// reaches, and not at a corner where both meet ([`unread_corner`]). A
/// side is read as its edge's carrier is in any chart ([`Along::of`]),
/// but carries no parameters, so the graph separator reads it only where
/// it lies on its edges ([`Along::under_graph`]). No rectangle is read as
/// a sliver: the sliver is frozen.
///
/// Every pose is tried in turn: `Structural` where the cell reaches a
/// corner no chart reads ([`unread_corner`]); `None` where the face has
/// no pose that reads its shared edges ([`poses_of`]); otherwise the
/// first presentation, else `Local`. Both answers that keep the stored
/// chart's `Structural` are **cost guards**, not soundness: each is a
/// cell every smaller cell of the pair also holds unread, so reading on
/// only splits further before the pair refuses. A pose's rectangle
/// and ball are wider than the cell's, so what it finds unread in reach
/// may lie outside the cell, and a smaller cell is placed tighter: no
/// pose's own failure decides `Structural`.
fn present_posed<'h>(
    piece: &Piece<'_>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
    store: &'h Poses,
) -> Option<Result<Presentation<'h>, Unpresented>> {
    let set = store
        .0
        .get_or_init(|| poses_of(piece, near_edges, near_vertices, band));
    if set.unread_corner {
        return Some(Err(Unpresented::Structural));
    }
    if set.poses.is_empty() && !set.unplaced {
        return None;
    }
    for p in &set.poses {
        let posed = Piece {
            face: piece.face,
            surface: &p.surface,
            whole: false,
            boundary: &p.boundary,
            u: p.x,
            v: p.y,
            local: &p.local,
            ball: p.ball,
        };
        let got = present_chart_in(&posed, near_edges, near_vertices, band, true);
        if let Ok(read) = got {
            return Some(Ok(read));
        }
    }
    Some(Err(Unpresented::Local))
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
    // EVERY corner the two clips make must be convex. No corner: the two
    // lines cross where neither edge reaches, which the cover has already
    // refused.
    let ok = |c: &Clip, d: &Clip, v: VertexKey| -> bool {
        // Every edge of `d` at `v` (one at least: `v` is a corner) leaves
        // it along `c`'s held axis, toward `c`'s face side; one whose
        // direction is not decided fails.
        d.edges
            .iter()
            .filter(|&&j| b.edges.get(j).is_some_and(|e| e.ends.contains(&v)))
            .all(|&j| away(j, v).is_some_and(|s| s * c.side.into() > 0.0))
    };
    corners(b, c, d)
        .into_iter()
        .all(|v| ok(c, d, v) && ok(d, c, v))
}

/// The sides of a presentation the separator vanishes on: the clips
/// whose every edge is shared, a collapsed side at a shared vertex, or
/// the corner of two clips at a shared vertex.
fn zero_of(
    surface: &Surface<Interval>,
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
    let along = |c: &Clip| match c.collapsed {
        Some(v) => Along::at(v),
        None => Along::of(surface, b, &c.edges),
    };
    match (zeros.as_slice(), loose.as_slice()) {
        ([], []) => Ok(Zero::None),
        ([c], []) => Ok(Zero::Side(c.side, along(c))),
        ([c, d], []) if c.side.axis() != d.side.axis() => {
            Ok(Zero::Corner((c.side, along(c)), (d.side, along(d))))
        }
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
                        corners.push(Zero::Point(c.side, d.side, *v));
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
/// (`work/embed/an-analytic-face-may-cover-its-carrier-more-than-once.md`).
fn partial_zero(
    surface: &Surface<Interval>,
    b: &Boundary,
    clips: &[Clip],
    near_edges: &[usize],
    near_vertices: &[VertexKey],
) -> Option<(Side, Along)> {
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
    let shared: Vec<usize> = c
        .edges
        .iter()
        .copied()
        .filter(|e| near_edges.contains(e))
        .collect();
    near_vertices
        .iter()
        .all(|v| ends.contains(v))
        .then(|| (c.side, Along::of(surface, b, &shared)))
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
    let orientation = b.orientation();
    let edges: Vec<usize> = piece
        .local
        .edges
        .iter()
        .copied()
        .filter(|&i| b.edges.get(i).is_some_and(|e| e.arc.is_some()))
        .collect();
    let cell_u = iv(piece.u);
    let cell_v = iv(piece.v);
    // A plane has no periodic axis: no sheet to choose.
    if let Some(read) = curved_walls(
        piece,
        &edges,
        [Sheet::Free; 2],
        near_edges,
        near_vertices,
        band,
    ) {
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
    // An edge's chart points in the loop's order, each with its parameter.
    type End = (Point2<Interval>, Interval);
    let segment = |ei: usize| -> Result<(End, End), Unpresented> {
        let e = b.edges.get(ei).ok_or(Unpresented::Local)?;
        let Some((carrier, (t0, t1))) = e.arc.as_ref().filter(|(c, _)| !c.is_curved()) else {
            return Err(unread(ei, near_edges));
        };
        let [u] = e.uses.as_slice() else {
            return Err(unread(ei, near_edges));
        };
        let (p, q) = (
            (chart(carrier.eval(*t0)), *t0),
            (chart(carrier.eval(*t1)), *t1),
        );
        Ok(if u.plus { (p, q) } else { (q, p) })
    };
    match edges.as_slice() {
        [e] => {
            if !piece.local.vertices.is_empty() {
                return Err(Unpresented::Local);
            }
            let ((p, ta), (q, tb)) = segment(*e)?;
            let a = [q.x - p.x, q.y - p.y];
            let bv = toward_face(a, orientation);
            let len2 = a[0].powi(2) + a[1].powi(2);
            let d = [cell_u - p.x, cell_v - p.y];
            let x = (d[0] * a[0] + d[1] * a[1]) / len2;
            let y = (d[0] * bv[0] + d[1] * bv[1]) / len2;
            // Wholly on the edge's non-face side holds none of the face:
            // every vertex and every other edge is clear of the cell's
            // ball, so the plane's disk inside the ball meets the
            // boundary only in a whole chord of this edge, and the
            // disk's half off the face side is all outside the face. The
            // cell lies in that half. A `len2` holding zero leaves `y`
            // uncertified, and a zero-length edge bounds no half.
            if y.is_certified() && y.hi() <= 0.0 {
                return Ok(Presentation::Vacuous);
            }
            let len = len2.sqrt();
            let inside = (x * len).min((at(1.0) - x) * len);
            if interval_sign_as(inside, SELF_INTERSECTION_SPAN, band) != Some(Sign::Positive) {
                return Err(Unpresented::Local);
            }
            let zero = if near_edges.contains(e) {
                Zero::Side(
                    Side::YLo,
                    Along::over(piece.surface, b, *e, true, || {
                        segment_params(ta, tb, (x.lo(), x.hi()))
                    }),
                )
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
            let ((p1, s1), (q1, f1)) = segment(*e1)?;
            let ((p2, s2), (q2, f2)) = segment(*e2)?;
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
            // Each edge's parameters at the vertex and its far end ride
            // with its points.
            let ((o, v1), (far1, w1)) = if start1 == vtx {
                ((p1, s1), (q1, f1))
            } else {
                ((q1, f1), (p1, s1))
            };
            let ((_, v2), (far2, w2)) = if start2 == vtx {
                ((p2, s2), (q2, f2))
            } else {
                ((q2, f2), (p2, s2))
            };
            let t1 = [q1.x - p1.x, q1.y - p1.y];
            let t2 = [q2.x - p2.x, q2.y - p2.y];
            let a = [far1.x - o.x, far1.y - o.y];
            let bv = [far2.x - o.x, far2.y - o.y];
            let n1 = toward_face(t1, orientation);
            let n2 = toward_face(t2, orientation);
            let lean = |n: [Interval; 2], w: [Interval; 2]| {
                (n[0] * w[0] + n[1] * w[1]) / (n[0].powi(2) + n[1].powi(2)).sqrt()
            };
            if interval_sign_as(lean(n1, bv), SELF_INTERSECTION_CORNER, band)
                != Some(Sign::Positive)
                || interval_sign_as(lean(n2, a), SELF_INTERSECTION_CORNER, band)
                    != Some(Sign::Positive)
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
            if interval_sign_as((at(1.0) - x) * la, SELF_INTERSECTION_SPAN, band)
                != Some(Sign::Positive)
                || interval_sign_as((at(1.0) - y) * lb, SELF_INTERSECTION_SPAN, band)
                    != Some(Sign::Positive)
            {
                return Err(Unpresented::Local);
            }
            let z1 = near_edges.contains(e1);
            let z2 = near_edges.contains(e2);
            // Each edge over its coordinate, 0 at the vertex and 1 at its
            // far end, out to `hi`.
            let on = |e: usize, (v, w): (Interval, Interval), hi: Interval| {
                Along::over(piece.surface, b, e, true, || {
                    segment_params(v, w, (0.0, hi.hi()))
                })
            };
            let (on1, on2) = (on(*e1, (v1, w1), x), on(*e2, (v2, w2), y));
            let zero = match (z1, z2) {
                (true, true) => Zero::Corner((Side::YLo, on1), (Side::XLo, on2)),
                (true, false) => Zero::Side(Side::YLo, on1),
                (false, true) => Zero::Side(Side::XLo, on2),
                (false, false) if near_vertices.contains(&vtx) => {
                    Zero::Point(Side::XLo, Side::YLo, vtx)
                }
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
pub(crate) const SELF_INTERSECTION_COLUMN: &str = "self_intersection_column";

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
    /// enclosed, `[u, v]`.
    pub(crate) fn at(&self, t: Interval) -> [Interval; 2] {
        [self.on(Axis::U, t), self.on(Axis::V, t)]
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
                let frame = geom::Frame::new(origin, axis, u_ref);
                // cos(φ + w·t) = cos φ·cos t − w·sin φ·sin t, and
                // sin(φ + w·t) = sin φ·cos t + w·cos φ·sin t: the radial
                // and its derivative at φ.
                let at_cos = frame.radial(h.p0[0]) * radius;
                let at_sin = frame.tangential(h.p0[0]) * (radius * at(w));
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
            let m = q.abs() / d;
            (m.is_certified() && d.lo() > 0.0).then(|| Interval::from_bounds(-m.hi(), m.hi()))
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

/// **A presentation's curved side**: the face's chart curve over the
/// parameters `t` its columns hold, its image there, and the edge it is
/// read along (the shared edge, or at a vertex the pair shares alone the
/// arc read through it), `whole` where its carrier is a conic, whose own
/// plane holds the curve continued past the edge too. `past`, the vertex
/// it runs on through, where the side runs on past that edge over the
/// face's next arc of the same conic ([`Reading::Conic`]), so the face
/// has points on it off the shared cell: read only where [`sign_on`]
/// admits it, as [`Zero::Past`] is.
#[derive(Clone, Debug)]
struct Curved {
    form: Form,
    t: (f64, f64),
    along: Along,
    past: Option<VertexKey>,
}

/// What a curved side's chart curve is.
#[derive(Clone, Copy, Debug)]
enum Form {
    /// A harmonic chart curve and its image on the face.
    Harmonic { chart: Harmonic, image: Trig },
    /// A spiric's wall image on a torus, and its image on the face.
    Wall(WallImage),
}

impl Curved {
    /// `ψ` along the side, bounded ([`Trig::psi`], [`WallImage::psi`]).
    fn psi(&self, psi: &Psi) -> Option<Interval> {
        let t = iv(self.t);
        match self.form {
            Form::Harmonic { image, .. } => image.psi(psi, t),
            Form::Wall(w) => w.psi(psi, t),
        }
    }

    /// The chart point at the middle of the side's parameters.
    fn middle(&self) -> (Interval, Interval) {
        let t = at(geom::mid_param(self.t.0, self.t.1));
        let [u, v] = match self.form {
            Form::Harmonic { chart, .. } => chart.at(t),
            Form::Wall(w) => w.wall.at(t),
        };
        (u, v)
    }
}

/// A conic carrier as a [`Trig`]: `center + u_ref·a·cos t + v_ref·b·sin t`.
fn conic(c: &Curve3<Interval>) -> Option<Trig> {
    let (center, axis, a, b, x) = match *c {
        Curve3::Circle(geom::Circle {
            frame:
                geom::Frame {
                    origin: center,
                    axis,
                    u_ref,
                },
            radius,
        }) => (center, axis, radius, radius, u_ref),
        Curve3::Ellipse(geom::Ellipse {
            frame:
                geom::Frame {
                    origin: center,
                    axis,
                    u_ref,
                },
            semi_u,
            semi_v,
        }) => (center, axis, semi_u, semi_v, u_ref),
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
        Surface::Plane { .. } => rim.arc.as_ref().is_none_or(|(c, _)| c.is_curved()),
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
    sheet: [Sheet; 2],
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> Option<Result<Presentation<'a>, Unpresented>> {
    let b = piece.boundary;
    let curved = |i: &usize| b.edges.get(*i).is_some_and(|e| is_curved(piece.surface, e));
    let &first = walls.iter().find(|i| curved(i))?;
    Some(match walls {
        [e] => match b.edges.get(*e).and_then(|r| wall_image(piece.surface, r)) {
            Some(w) => present_wall(piece, *e, w, None, near_edges, near_vertices, band),
            None => present_curved(piece, *e, None, near_edges, near_vertices, band),
        },
        _ => corner_of(b, walls, near_edges, near_vertices, &piece.local.vertices)
            .and_then(|c| present_corner(piece, c, sheet, near_edges, near_vertices, band))
            .or_else(|| {
                vertex_corner(piece, walls, near_edges, near_vertices)
                    .map(|(v, l, o, h)| present_vertex_corner(piece, v, l, o, h, band))
            })
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
            && corner_of(b, &at_v, near_edges, near_vertices, &[*v])
                .is_none_or(|c| reading(piece, c).is_none())
            && vertex_corner(piece, &at_v, near_edges, near_vertices).is_none()
    });
    if unread_corner {
        return Unpresented::Structural;
    }
    match curved.iter().find(|&&i| {
        b.edges.get(i).is_none_or(|e| {
            !reads_curve(piece.surface, e) && wall_image(piece.surface, e).is_none()
        })
    }) {
        Some(&i) => unread(i, near_edges),
        None if curved.is_empty() => unread(first, near_edges),
        None => Unpresented::Local,
    }
}

/// **Whether the curved edge `ei` is the shared cell**, for a
/// presentation by it alone: one that is not, beside a shared vertex, is
/// read by no curve presentation (a smaller cell may hold the vertex
/// alone).
fn sharing(
    ei: usize,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
) -> Result<bool, Unpresented> {
    let shared = near_edges.contains(&ei);
    if !shared && !near_vertices.is_empty() {
        return Err(Unpresented::Local);
    }
    Ok(shared)
}

/// **The curve `e` read through a `corner`'s vertex**: its closed form
/// continued past that end, over its own span `(t0, t1)` and as much
/// again, or half what a turn leaves, whichever is less. `None` with no
/// corner; a closed edge has no end to read through.
///
/// The half-turn cap keeps the continued span short of a turn, which
/// [`present_curved`]'s and [`present_wall`]'s step 1 rest on: a span a
/// turn long or more would carry its continuation round to its own far
/// end, so the columns could hold the edge and its continuation both.
///
/// A corner whose other edge continues the curve's conic ([`continues`],
/// [`Reading::Conic`]) is read over the whole conic instead: a span
/// centred on the vertex and a little longer than a turn, so every point
/// of the conic is on it and the two ends' overlap lies opposite the
/// vertex. A cell holding the conic twice there is refused by step 2's
/// [`apart`], the guard the cap stands in for.
fn through_of(
    b: &Boundary,
    e: &Rim,
    corner: Option<Corner>,
    (t0, t1): (f64, f64),
) -> Result<Option<Through>, Unpresented> {
    let Some(c) = corner else {
        return Ok(None);
    };
    if e.ends[0] == e.ends[1] {
        return Err(Unpresented::Local);
    }
    let start = e.ends[0] == c.vertex;
    let t = if start { t0 } else { t1 };
    let conic = b
        .edges
        .get(c.other)
        .is_some_and(|o| continues(e, o, c.vertex));
    let span = if conic {
        let half = TURN * (0.5 + 1.0 / 64.0);
        (t - half, t + half)
    } else {
        let ext = (t1 - t0).min((TURN - (t1 - t0)) * 0.5);
        if ext.is_nan() || ext <= 0.0 {
            return Err(Unpresented::Local);
        }
        if start {
            (t0 - ext, t1)
        } else {
            (t0, t1 + ext)
        }
    };
    Ok(Some(Through {
        corner: c,
        t,
        span,
        conic,
    }))
}

/// The range `(lo, hi)` widened to hold `x`.
fn widened((lo, hi): (f64, f64), x: Interval) -> (f64, f64) {
    (lo.min(x.lo()), hi.max(x.hi()))
}

/// Whether `x` lies within the closed range `(lo, hi)`.
fn within(x: Interval, (lo, hi): (f64, f64)) -> bool {
    lo <= x.lo() && x.hi() <= hi
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
    // The curve is the zero: the shared edge, or at a vertex the pair
    // shares alone the arc read on through it.
    let zeroed = corner.is_some_and(|c| c.alone) || sharing(ei, near_edges, near_vertices)?;
    let through = through_of(piece.boundary, e, corner, (t0, t1))?;
    for column in [Axis::U, Axis::V] {
        let span = through.as_ref().map_or((t0, t1), |th| th.span);
        let pieces = Pieces::of(&chart, column, span, continued);
        let Graph { x, y, above, t } =
            match graph(piece, ei, u.plus, &pieces, through.as_ref(), band) {
                Ok(g) => g,
                Err(Unpresented::Local) => continue,
                Err(e) => return Err(e),
            };
        let zero = if zeroed {
            Zero::Curve(
                Side::of(column.other(), above),
                Box::new(Curved {
                    form: Form::Harmonic { chart, image },
                    t,
                    along: Along {
                        edges: vec![e.key],
                        vertex: None,
                        whole: matches!(carrier, Curve3::Circle(_) | Curve3::Ellipse(_)),
                        params: Some(Params { t, periodic: true }),
                        on_edges: false,
                    },
                    past: through
                        .as_ref()
                        .filter(|th| th.conic)
                        .map(|th| th.corner.vertex),
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

// ---------------------------------------------------- the spiric wall

impl Wall {
    /// `(R + r·cos t, f(t))` over `t`, with `cos t` given.
    fn radial(&self, cos: Interval) -> (Interval, Interval) {
        geom::spiric_radial(self.major, self.minor, self.offset, cos)
    }

    /// The image on `axis` over the parameters `t`, enclosed.
    fn on(&self, axis: Axis, t: Interval) -> Interval {
        match axis {
            Axis::U => {
                let (_, f) = self.radial(t.sin_cos().1);
                self.u0 + at(self.kappa) * f.atan2(self.offset)
            }
            Axis::V => self.v0 + at(self.sigma) * t,
        }
    }

    /// Its derivative on `axis` over `t`, enclosed: `σ` on `v`, and on
    /// `u` `κ·θ′ = −κ·d·r·sin t / (ρ·f)`, since `θ′ = d·f′/ρ²` and
    /// `f′ = −r·ρ·sin t / f`.
    fn rate(&self, axis: Axis, t: Interval) -> Interval {
        match axis {
            Axis::U => {
                let (s, c) = t.sin_cos();
                let (rho, f) = self.radial(c);
                Interval::from_f64(0.0) - at(self.kappa) * self.offset * self.minor * s / (rho * f)
            }
            Axis::V => at(self.sigma),
        }
    }

    /// The chart point at `t`, or the image over the parameters `t`,
    /// enclosed, `[u, v]`.
    pub(crate) fn at(&self, t: Interval) -> [Interval; 2] {
        [self.on(Axis::U, t), self.on(Axis::V, t)]
    }
}

/// **A chart curve a corner's other edge is read by**
/// ([`leaves_line`]).
#[derive(Clone, Copy, Debug)]
enum ChartCurve {
    Harmonic(Harmonic),
    Wall(Wall),
}

impl ChartCurve {
    fn at(&self, t: Interval) -> [Interval; 2] {
        match self {
            Self::Harmonic(h) => h.at(t),
            Self::Wall(w) => w.at(t),
        }
    }

    fn rate(&self, axis: Axis, t: Interval) -> Interval {
        match self {
            Self::Harmonic(h) => h.rate(axis, t),
            Self::Wall(w) => w.rate(axis, t),
        }
    }
}

/// **A wall image's image on its torus**, `S(u(t), v(t))`, in the
/// closed form `ψ` is read along it by ([`WallImage::psi`]):
///
/// ```text
/// S = c + q(t)·(d·e₀ + f(t)·e₁) + z_c·cos t + z_s·sin t
/// q = ρ_S / ρ = 1 + (δ₀ + δ_c·cos t + δ_s·sin t) / ρ
/// ```
///
/// with `ρ = R + r·cos t` the carrier's and `ρ_S` the torus's own at
/// `v(t)`, `e₀` the azimuth `u0` and `e₁` the turn `κ·90°` from it,
/// `c` the torus's centre moved `d·e₀`. Since `θ = atan2(f, d)` has
/// `cos θ = d/ρ` and `sin θ = f/ρ`, the azimuth `u0 + κθ` puts the
/// radial direction at `(d·e₀ + f·e₁)/ρ`, and `ρ_S` scales it. Every
/// coefficient is formed before `t` is read: where the image lies on a
/// plane through the curve, the plane reads `n·e₁`, `n·z_c`, `n·z_s`
/// and `c`'s offset as rounding, and `q − 1` is the torus's drift from
/// the carrier's radii, at any width of `t`.
#[derive(Clone, Copy, Debug)]
struct WallImage {
    wall: Wall,
    c: Point3<Interval>,
    de0: Vec3<Interval>,
    e1: Vec3<Interval>,
    zc: Vec3<Interval>,
    zs: Vec3<Interval>,
    delta: [Interval; 3],
}

impl WallImage {
    /// The image of `w` on `surface`, a torus; `None` on anything else.
    fn of(surface: &Surface<Interval>, w: Wall) -> Option<Self> {
        let Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } = *surface
        else {
            return None;
        };
        let v_ref = axis.cross(u_ref);
        let (su, cu) = w.u0.sin_cos();
        let (sv, cv) = w.v0.sin_cos();
        let sigma = at(w.sigma);
        let de0 = (u_ref * cu + v_ref * su) * w.offset;
        // cos(v0 + σt) = cos v0·cos t − σ·sin v0·sin t, and
        // sin(v0 + σt) = sin v0·cos t + σ·cos v0·sin t.
        Some(Self {
            wall: w,
            c: center + de0,
            de0,
            e1: (v_ref * cu - u_ref * su) * at(w.kappa),
            zc: axis * (minor_radius * sv),
            zs: axis * (minor_radius * cv * sigma),
            delta: [
                major_radius - w.major,
                minor_radius * cv - w.minor,
                Interval::from_f64(0.0) - minor_radius * sv * sigma,
            ],
        })
    }

    /// **`ψ` along the image over `t`**, for a linear separator
    /// ([`WallImage`]'s closed form); `None` for any other.
    fn psi(&self, psi: &Psi, t: Interval) -> Option<Interval> {
        let (n, q) = match *psi {
            Psi::Linear { n, q } => (n, q),
            Psi::Carrier(Implicit::Plane { o, n }) => (n, o),
            _ => return None,
        };
        let (s, c) = t.sin_cos();
        let (rho, f) = self.wall.radial(c);
        let radial = n.dot(self.de0) + n.dot(self.e1) * f;
        let drift = (self.delta[0] + self.delta[1] * c + self.delta[2] * s) / rho;
        Some(
            n.dot(self.c - q)
                + n.dot(self.e1) * f
                + n.dot(self.zc) * c
                + n.dot(self.zs) * s
                + drift * radial,
        )
    }
}

/// **The wall image of a torus face's one use of an edge**, where it
/// has one.
fn wall_image(surface: &Surface<Interval>, rim: &Rim) -> Option<Wall> {
    match (surface, rim.uses.as_slice()) {
        (Surface::Torus { .. }, [u]) => u.wall,
        _ => None,
    }
}

/// Whether an edge is a parallel of its face's chart: its one use a
/// coordinate line holding `v`.
fn parallel(rim: &Rim) -> bool {
    matches!(rim.uses.as_slice(), [u] if u.iso.is_some_and(|i| i.held == Axis::V))
}

/// **Present a torus face by the region on one side of the spiric edge
/// `ei`**, the one boundary edge near the cell, whose chart image is the
/// wall image `w`: the face's part of the cell lies in
/// `{(u, v) : v ∈ A, u on the face's side of g(v)}`, `A` the cell's
/// range on `v` (its columns), `g` the curve read as a graph over `v`.
///
/// **Why that region holds every point of the face in the cell** —
/// [`present_curved`]'s three steps, on the curve's closed form. Read on
/// the cover of the chart, so no sheet is chosen:
///
/// 1. `v = v0 + σ·t`, `σ = ±1`, is strictly monotone in `t`: the curve
///    is a graph over `v` everywhere on every lift, with no turning
///    point. An open edge has a lift by whole turns of `v` whose `v`
///    range holds `A` strictly inside, so the edge runs past both
///    column ends and its vertices lie outside the columns; that range
///    is narrower than a turn, so no other lift meets `A`. A closed
///    edge whose span is a whole period is continued by its closed
///    form, which returns to its vertex's chart point a whole turn of
///    `v` on: one curve over all of `v`. Either way each column holds exactly one point of the
///    lift read, `(g(v), v)`, at the parameters `t` the columns hold.
/// 2. `R` is the columns times the hull, on `u`, of the cell and of the
///    curve over `t`. Its `u` range is narrower than a turn, so the
///    curve's copies a turn either side miss it, and every other edge,
///    every vertex off the curve too, is decided clear of `R`'s ball
///    ([`alone`]): the curve is the only boundary in `R`.
/// 3. A point of the cell on the wrong side of `g(v)` in its column
///    joins `(g(v), v)` by a segment in the column that meets no
///    boundary until it crosses the curve, transversally since
///    `dv/dt ≠ 0`, from the side the face is not on
///    ([`left_is_above`]). So it lies outside the face.
///
/// `R` is presented with the curve as its zero, as [`present_curved`]
/// presents a harmonic one; [`sign_on`] reads `ψ` zero along it
/// ([`WallImage::psi`], linear separators only) and strictly monotone
/// across it over `R`. A closed edge is read through its join by the
/// closed form past its span: [`present_curved`]'s ε concession.
///
/// Through a `corner` whose other edge is straight in the chart (a
/// coordinate line), the open edge is continued past the vertex by its
/// closed form as [`present_curved`] continues one, the columns are
/// widened to hold the vertex, and the other edge is admitted in reach
/// where it leaves the vertex onto the face's side ([`leaves_graph`];
/// [`present_corner`] argues the region).
fn present_wall<'a>(
    piece: &Piece<'a>,
    ei: usize,
    w: Wall,
    corner: Option<Corner>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> Result<Presentation<'a>, Unpresented> {
    let e = piece.boundary.edges.get(ei).ok_or(Unpresented::Local)?;
    let [u] = e.uses.as_slice() else {
        return Err(unread(ei, near_edges));
    };
    let Some(image) = WallImage::of(piece.surface, w) else {
        return Err(unread(ei, near_edges));
    };
    let Some((_, span)) = e.arc.as_ref() else {
        return Err(Unpresented::Local);
    };
    let (t0, t1) = (span.0.lo(), span.1.hi());
    if !(t1 - t0).is_finite() || t1 <= t0 {
        return Err(Unpresented::Structural);
    }
    let shared = sharing(ei, near_edges, near_vertices)?;
    let closed = e.ends[0] == e.ends[1] && whole_turn(t0, t1);
    let through = through_of(piece.boundary, e, corner, (t0, t1))?;
    // Through a corner `R` holds the vertex: the cell is widened to its
    // chart point, on the copy nearest the cell.
    let mut cell = (piece.u, piece.v);
    if let Some(th) = &through {
        let [vu, vv] = w.at(at(th.t));
        let near = |(lo, hi): (f64, f64), x: Interval| wrap(x, geom::mid_param(lo, hi));
        cell = (
            widened(cell.0, near(cell.0, vu)),
            widened(cell.1, near(cell.1, vv)),
        );
    }
    let (cu, a) = cell;
    // Step 1: the one lift across the columns, and the parameters they
    // hold on it.
    let held = |k: f64| (iv(a) - w.v0 - turns(k)) * at(w.sigma);
    let lift = if closed {
        0.0
    } else {
        // The lift whose span holds the columns strictly inside. Its `v`
        // range is narrower than a turn, so no other lift meets them.
        let (s0, s1) = through.as_ref().map_or((t0, t1), |th| th.span);
        let reach = w.on(Axis::V, Interval::from_bounds(s0, s1));
        turn_ladder((reach.lo(), reach.hi()), a)
            .ok_or(Unpresented::Local)?
            .into_iter()
            .find(|&k| {
                let t = held(k);
                s0 < t.lo() && t.hi() < s1
            })
            .ok_or(Unpresented::Local)?
    };
    let t = held(lift);
    if !Real::is_bounded(t) {
        return Err(Unpresented::Local);
    }
    // Step 2: `R`, and nothing in it but the curve.
    let g = wrap(w.on(Axis::U, t), geom::mid_param(cu.0, cu.1));
    if !Real::is_bounded(g) {
        return Err(Unpresented::Local);
    }
    // `R`'s `u` range holds `g` and is narrower than a turn, so the
    // curve's copies a turn either side miss it.
    let x = (cu.0.min(g.lo()), cu.1.max(g.hi()));
    if whole_turn(x.0, x.1) {
        return Err(Unpresented::Local);
    }
    let local = if x == piece.u && a == piece.v {
        piece.local.clone()
    } else {
        let m = |(l, h): (f64, f64)| geom::mid_param(l, h);
        let ball = Ball::holding(
            piece.surface.point_eval(m(x), m(a)),
            piece.surface.rect_enclosure(x, a),
        );
        Local::of(piece.boundary, ball, band)
    };
    if !alone(
        piece,
        ei,
        through.as_ref().map(|th| th.corner.other),
        &local,
    ) {
        return Err(Unpresented::Local);
    }
    // Step 3: the face's side.
    let rising = w.sigma > 0.0;
    let above = piece
        .boundary
        .orientation()
        .interior_is_on(left_is_above(rising == u.plus, Axis::V));
    if let Some(th) = &through {
        // The vertex is in `R`, on the lift read: its parameter among
        // the columns', its point within `R`'s `u` range.
        let held = wrap(w.on(Axis::U, at(th.t)), geom::mid_param(x.0, x.1));
        if !(t.lo() <= th.t && th.t <= t.hi() && within(held, x)) {
            return Err(Unpresented::Local);
        }
        let along = w.on(Axis::V, at(th.t)) + turns(lift);
        // The other edge is a parallel ([`reading`]), so its direction
        // has no `v` part and the curve's rate on `u` multiplies zero.
        let tau = (w.rate(Axis::V, t), w.rate(Axis::U, t));
        let vertex = [held, along];
        match leaves_graph(
            piece,
            th,
            Axis::V,
            tau,
            t,
            rising,
            above,
            vertex,
            (x, a),
            band,
        ) {
            Some(Ok(())) => {}
            Some(Err(e)) => return Err(e),
            None => return Err(Unpresented::Local),
        }
    }
    let zero = if shared {
        Zero::Curve(
            Side::of(Axis::U, above),
            Box::new(Curved {
                form: Form::Wall(image),
                t: (t.lo(), t.hi()),
                along: Along {
                    edges: vec![e.key],
                    vertex: None,
                    whole: false,
                    params: Some(Params {
                        t: (t.lo(), t.hi()),
                        periodic: true,
                    }),
                    on_edges: false,
                },
                past: None,
            }),
        )
    } else {
        Zero::None
    };
    Ok(Presentation::Of(Box::new(Presented {
        surface: piece.surface,
        map: Map::Chart,
        x,
        y: a,
        zero,
        corner: through.is_some(),
        cache: Cache::default(),
    })))
}

// -------------------------------------------------- the curved corner

/// The funnel site of a curved corner's convexity: how fast the
/// corner's other edge leaves the edge read through the vertex toward
/// that edge's face side — off the line, or off the curve's graph —
/// times the extent it is read over and the chart's speed (an upper
/// bound, `rect_jet`'s), metres. Only a definite `Positive` reads the
/// corner convex; a definite `Negative` is a reflex corner, which this
/// presentation never reads.
pub(crate) const SELF_INTERSECTION_CORNER_DEPARTS: &str = "self_intersection_corner_departs";

/// The funnel site of a corner's vertex: the chart distance between the
/// other edge's end and the read edge's point at the vertex, each axis
/// times the chart's speed along it over the rectangle (an upper bound,
/// `rect_jet`'s), metres. Only a definite `Zero` reads them as one
/// point — one of the presentation's ε concessions (the module docs
/// list them).
pub(crate) const SELF_INTERSECTION_CORNER_VERTEX: &str = "self_intersection_corner_vertex";

/// **The two edges' points at a corner's vertex are one**, decided at
/// [`SELF_INTERSECTION_CORNER_VERTEX`]: `a` and `b` chart points `[u, v]` on one copy,
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
///
/// The chart gap is levered into metres by `speed`, the chart's speed
/// bound over the rectangle: exact on a constant-speed chart (a plane's,
/// a cylinder's), an upper bound on a torus's. Every chart read through a
/// corner is orthogonal (`S_u ⊥ S_v`), so the root of the two levered
/// squares bounds the metres between the two points over the rectangle,
/// and a gap decided `Zero` on it is within the band.
fn on_vertex(a: [Interval; 2], b: [Interval; 2], speed: [f64; 2], band: Band) -> bool {
    let off = |i: usize| ((a[i] - b[i]) * at(speed[i])).powi(2);
    let gap = (off(0) + off(1)).sqrt();
    matches!(
        decide(SELF_INTERSECTION_CORNER_VERTEX, Margin::of(gap), band),
        Ok(Sign::Zero)
    )
}

/// How many halvings [`leaves_line`] gives a piece of the other edge it
/// cannot yet tell.
const LEAVES: u32 = 24;

/// **A corner of a cell's boundary**: its two walls meeting at `vertex`,
/// `read` the one on the shared cell, read through the vertex, and
/// `other`, which leaves it. `alone`: the shared cell is `vertex` alone,
/// neither wall on it, and `read` is the one of lower key.
#[derive(Clone, Copy, Debug)]
struct Corner {
    vertex: VertexKey,
    read: usize,
    other: usize,
    alone: bool,
}

/// A curve read through a corner: the corner, the curve's parameter at
/// the vertex, the span continued past it, and whether the corner's
/// other edge is an arc of the curve's own conic ([`Reading::Conic`]).
struct Through {
    corner: Corner,
    t: f64,
    span: (f64, f64),
    conic: bool,
}

/// **The vertex that is the shared cell alone**: no shared edge, and one
/// shared vertex.
fn vertex_alone(no_edges: bool, vertices: &[VertexKey]) -> Option<VertexKey> {
    match vertices {
        &[v] if no_edges => Some(v),
        _ => None,
    }
}

/// **The corner a cell's two walls make**: two open edges with one end
/// in common, one of them on the shared cell, or neither where that end
/// is the shared cell alone (`alone`). Two edges meeting at both
/// ends, a lens, make a corner at the one of its ends among `at` (the
/// cell's local vertices), where exactly one is; `None` for anything
/// else. A lens is read at one end like any corner: its other end lies
/// outside `R`, which the curve reading's run passes on both column
/// ends of, and which the line readings admit no other vertex into.
/// Keeping a shared `other` off is not needed for soundness — it would
/// only be left off the zero, which costs a certificate, never a wrong
/// one — and keeps such cells to the readings they had.
fn corner_of(
    b: &Boundary,
    walls: &[usize],
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    at: &[VertexKey],
) -> Option<Corner> {
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
    let vertex = match *common.as_slice() {
        [v] => v,
        [v, w] => match (at.contains(&v), at.contains(&w)) {
            (true, false) => v,
            (false, true) => w,
            _ => return None,
        },
        _ => return None,
    };
    match (near_edges.contains(&e), near_edges.contains(&f)) {
        (true, false) => Some(Corner {
            vertex,
            read: e,
            other: f,
            alone: false,
        }),
        (false, true) => Some(Corner {
            vertex,
            read: f,
            other: e,
            alone: false,
        }),
        (false, false) if vertex_alone(near_edges.is_empty(), near_vertices) == Some(vertex) => {
            // The arc read is the one of lower key, so the reading does not
            // depend on the order the walls come in.
            let (read, other) = if re.key <= rf.key { (e, f) } else { (f, e) };
            Some(Corner {
                vertex,
                read,
                other,
                alone: true,
            })
        }
        _ => None,
    }
}

/// **Present a cell at a corner** ([`corner_of`]) by its shared edge
/// alone, read through the vertex: a curve by its closed form continued
/// ([`present_curved`]), a coordinate line of a chart by the line
/// ([`present_line_corner`]), a plane's line edge by its line
/// ([`present_plane_corner`]). `None` where none reads it.
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
///
/// **Two arcs of one conic** ([`Reading::Conic`]): `O` is `E`'s
/// continuation through `V` ([`continues`]): one conic as one value,
/// leaving `V` the way `E` arrives, the two no more than a turn, so they
/// meet only at their common ends. The curve is read over the whole
/// conic ([`through_of`]), so step 2's guard leaves one run of it in
/// `R`, a graph across every column, and nothing else in reach but `E`,
/// `O` and `E`'s ends. That run holds `V`. Followed from `V` either way
/// it stays on `E` or `O` until it leaves `R`: an end of either in reach
/// that is not an end of both would bring another edge, or a vertex
/// that is not `E`'s, into reach. So the boundary in `R` is the run,
/// which every column crosses, traversed one way along it through `V`,
/// so the face lies on the side `E`'s traversal names. The run past `V`
/// is `O`: the zero is a curve with points off the shared cell, `past`
/// ([`Curved`]), read only where [`past_read`] admits it.
///
/// **At a vertex the pair shares alone** (`alone`) neither arc is on the
/// shared cell, and this is the one reading: the region is argued as
/// above, and only `V` of the run is on the shared cell.
fn present_corner<'a>(
    piece: &Piece<'a>,
    c: Corner,
    sheet: [Sheet; 2],
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> Option<Result<Presentation<'a>, Unpresented>> {
    Some(match reading(piece, c)? {
        Reading::Curve | Reading::Conic => {
            present_curved(piece, c.read, Some(c), near_edges, near_vertices, band)
        }
        Reading::Wall(w) => {
            present_wall(piece, c.read, w, Some(c), near_edges, near_vertices, band)
        }
        Reading::Line(o) => present_line_corner(piece, c, sheet, o, band),
        Reading::PlaneLine(o) => present_plane_corner(piece, c, o, band),
    })
}

/// How [`present_corner`] reads a corner.
enum Reading {
    /// The shared edge is a curve [`present_curved`] reads, the other
    /// edge straight in the chart.
    Curve,
    /// The shared edge is a curve on a conic carrier, the other edge its
    /// continuation through the vertex along the same conic
    /// ([`continues`]): the curve is read on through the vertex along the
    /// other edge.
    Conic,
    /// The shared edge is a spiric's wall image on a torus
    /// ([`present_wall`]), the other edge a parallel of it.
    Wall(Wall),
    /// The shared edge is a coordinate line of a cylinder's chart, the
    /// other edge a harmonic chart curve; or a parallel of a torus's,
    /// the other edge a wall image.
    Line(ChartCurve),
    /// The shared edge is a plane's line edge, the other edge a conic,
    /// its chart curve this.
    PlaneLine(Harmonic),
}

/// **Which reading a corner has, if any**: one fixed by the two edges'
/// kinds, so a corner with none has none at any depth.
fn reading(piece: &Piece<'_>, c: Corner) -> Option<Reading> {
    let b = piece.boundary;
    let (e, o) = (b.edges.get(c.read)?, b.edges.get(c.other)?);
    let (s, curved) = (piece.surface, |r: &Rim| is_curved(piece.surface, r));
    // A curve no presentation reads is read by no corner: `Structural`,
    // as [`present_curved`] types it unread on the shared cell.
    if curved(e) && reads_curve(s, e) && continues(e, o, c.vertex) {
        return Some(Reading::Conic);
    }
    if c.alone {
        return None;
    }
    if curved(e) {
        let straight = straight(s, o, c.vertex).is_some();
        if let Some(w) = wall_image(s, e) {
            return (straight && parallel(o)).then_some(Reading::Wall(w));
        }
        (reads_curve(s, e) && straight).then_some(Reading::Curve)
    } else if let Surface::Plane { .. } = s {
        let [u] = o.uses.as_slice() else {
            return None;
        };
        if !curved(o) || piece.whole {
            return None;
        }
        chart_curve(s, o, u).map(Reading::PlaneLine)
    } else {
        // The argument is made on a chart that never collapses: a
        // cylinder's, or a ring torus's. On a sphere a circle is read in
        // the pose that makes it a coordinate line ([`present_posed`]),
        // and a curve no chart makes one is not argued here;
        // a cone's apex collapses a line of its chart, and is not read
        // past here, conservatively: an apex in reach is a vertex the
        // reading refuses. On a torus the line is a parallel, and the
        // curve a wall image, whose `v` is linear in its parameter: that
        // is what [`leaves_line`]'s rate on the held axis reads. A
        // meridian, along which the wall's azimuth can turn, and a
        // harmonic curve, which can turn on both axes, are not argued.
        let line = matches!(e.uses.as_slice(), [u] if u.iso.is_some_and(|i| !i.full));
        if !(line && curved(o)) || piece.whole {
            return None;
        }
        let harmonic = match o.uses.as_slice() {
            [u] => u.harmonic,
            _ => None,
        };
        match s {
            Surface::Cylinder { .. } => harmonic.map(|h| Reading::Line(ChartCurve::Harmonic(h))),
            Surface::Torus { .. } if parallel(e) => {
                wall_image(s, o).map(|w| Reading::Line(ChartCurve::Wall(w)))
            }
            _ => None,
        }
    }
}

/// **Whether two edges' stored carriers are one conic, as one value**:
/// one plane through one centre ([`one_plane`]) and the same radius; on
/// an ellipse the same semi-axes along the same lines (`u_ref` equal or
/// opposed, a half-turn of the parameter). Every comparison is
/// [`one_value`]'s, so a component held as a bracket wider than a point
/// is never one conic, even against itself: at `Interval` this admits
/// only carriers stored as points. The two parametrizations may differ;
/// the reading never maps one onto the other ([`continues`]).
fn one_conic(e: &Rim, o: &Rim) -> bool {
    let (Some((ce, _)), Some((co, _))) = (e.arc.as_ref(), o.arc.as_ref()) else {
        return false;
    };
    let plane = |f: &geom::Frame<Interval>, g: &geom::Frame<Interval>| {
        one_plane((f.origin, f.axis), (g.origin, g.axis))
    };
    match (ce, co) {
        (Curve3::Circle(a), Curve3::Circle(b)) => {
            plane(&a.frame, &b.frame) && one_value(a.radius, b.radius, false)
        }
        (Curve3::Ellipse(a), Curve3::Ellipse(b)) => {
            plane(&a.frame, &b.frame)
                && one_line(a.frame.u_ref, b.frame.u_ref)
                && one_value(a.semi_u, b.semi_u, false)
                && one_value(a.semi_v, b.semi_v, false)
        }
        _ => false,
    }
}

/// **`o` is `e`'s continuation through `v` along one conic**
/// ([`Reading::Conic`]): the two are [`one_conic`], `o` has one use (a
/// seam would put the face on both its sides), the two arcs together are
/// no more than a turn of the conic, and `o` leaves `v` along the
/// conic's tangent the way `e` arrives there, decided as an interval: the
/// two directions are parallel, so their product is far from zero. So
/// `o` does not lie back over `e`, and the two meet only at their common
/// ends.
fn continues(e: &Rim, o: &Rim, v: VertexKey) -> bool {
    let (Some((ce, se)), Some((co, so))) = (e.arc.as_ref(), o.arc.as_ref()) else {
        return false;
    };
    if o.uses.len() != 1 || !one_conic(e, o) || e.ends[0] == e.ends[1] || o.ends[0] == o.ends[1] {
        return false;
    }
    let width = |(t0, t1): &(Interval, Interval)| t1.hi() - t0.lo();
    // A NaN width reads as more than a turn.
    let within_a_turn = (width(se) + width(so))
        .partial_cmp(&(TURN + few_ulps(TURN)))
        .is_some_and(core::cmp::Ordering::is_le);
    if !within_a_turn {
        return false;
    }
    // Away from `v`: along each edge's parameter from the end it has there.
    let away = |c: &Curve3<Interval>, (t0, t1): &(Interval, Interval), ends: [VertexKey; 2]| {
        if ends[0] == v {
            Some(c.deriv(*t0))
        } else if ends[1] == v {
            Some(c.deriv(*t1) * at(-1.0))
        } else {
            None
        }
    };
    match (away(ce, se, e.ends), away(co, so, o.ends)) {
        // `e` arrives along the negation of its own away direction.
        (Some(de), Some(d_o)) => (de.dot(d_o) * at(-1.0)).lo() > 0.0,
        _ => false,
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
        let (carrier, (t0, t1)) = rim.arc.as_ref().filter(|(c, _)| !c.is_curved())?;
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

/// Which chart axes of `surface` are periodic, `[u, v]`.
fn periodic_axes(surface: &Surface<Interval>) -> [bool; 2] {
    [periodic(surface, Axis::U), periodic(surface, Axis::V)]
}

/// The whole turns, per chart axis, that move `x` nearest `to`: zero on
/// an axis that is not periodic.
fn turns_to(periodic: [bool; 2], x: [Interval; 2], to: [Interval; 2]) -> [f64; 2] {
    core::array::from_fn(|i| {
        if periodic[i] {
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
    periodic: [bool; 2],
    reach: [(f64, f64); 2],
    r: ((f64, f64), (f64, f64)),
) -> Option<Vec<[f64; 2]>> {
    let counts = |axis: Axis| -> Option<Vec<f64>> {
        if !periodic[axis.index()] {
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

/// The chart point or box `x`, `[u, v]`, moved `k` whole turns per axis.
fn shifted(x: [Interval; 2], k: [f64; 2]) -> [Interval; 2] {
    [x[0] + turns(k[0]), x[1] + turns(k[1])]
}

/// **The chart's speeds over the rectangle `u × v`**, `[|S_u|, |S_v|]`,
/// upper bounds (`rect_jet`'s): what a chart length along each axis
/// levers into metres.
fn speeds(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> [f64; 2] {
    let (ju, jv, _) = rect_jet(surface, u, v);
    [ju.norm().hi(), jv.norm().hi()]
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
/// curve's chart tangent `τ = (τ_a, τ_b)`, `a` the column axis, and the
/// edge's direction `d` make `(d_b·τ_a − d_a·τ_b)·sign τ_a` one strict
/// sign, the face's side's.
/// That is the derivative of the edge's height over the graph along it,
/// which is zero at the vertex, so the edge stands strictly on the
/// face's side wherever it is in `R`. `None` where this is not decided;
/// a reflex corner, decided, is `Structural`.
#[allow(clippy::too_many_arguments)]
fn leaves_graph(
    piece: &Piece<'_>,
    th: &Through,
    column: Axis,
    (tau_a, tau_b): (Interval, Interval),
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
    let k = turns_to(periodic_axes(surface), [near.x, near.y], vertex);
    let reach = reach_of([[near.x, near.y], [far.x, far.y]].into_iter());
    // Another copy meets `R` only where the edge, a segment from the
    // vertex, comes round by a turn to a range of `R` that holds the
    // vertex's: its far end then lies in `R`, a vertex [`alone`] refuses.
    // Defence for the reading; not load-bearing on it.
    for c in copies(periodic_axes(surface), reach, r)? {
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
    let speed = speeds(surface, r.0, r.1);
    if !on_vertex(shifted([near.x, near.y], k), vertex, speed, band) {
        return None;
    }
    let (a, b) = (column, column.other());
    let (da, db) = (a.of((d[0], d[1])), b.of((d[0], d[1])));
    let length = mid((d[0].powi(2) + d[1].powi(2)).sqrt());
    let lever = speed[0].max(speed[1]);
    let scale = (ts.hi() - ts.lo()) * lever / length;
    if !(scale.is_finite() && scale > 0.0) {
        return None;
    }
    let side = if rising == above { 1.0 } else { -1.0 };
    let rate = (db * tau_a - da * tau_b) * at(side * scale);
    match decide(SELF_INTERSECTION_CORNER_DEPARTS, Margin::of(rate), band) {
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
    sheet: [Sheet; 2],
    h: ChartCurve,
    band: Band,
) -> Result<Presentation<'a>, Unpresented> {
    let b = piece.boundary;
    let surface = piece.surface;
    let (Some(e), Some(o)) = (b.edges.get(c.read), b.edges.get(c.other)) else {
        return Err(Unpresented::Local);
    };
    let ([ue], [_]) = (e.uses.as_slice(), o.uses.as_slice()) else {
        return Err(Unpresented::Local);
    };
    let (Some(iso), Some((_, span))) = (ue.iso, o.arc.as_ref()) else {
        return Err(Unpresented::Local);
    };
    if iso.full {
        return Err(Unpresented::Local);
    }
    let cell = (piece.u, piece.v);
    let (held, moving) = (iso.held, iso.held.other());
    let line = placed(surface, iso, sheet, held.of(cell)).ok_or(Unpresented::Structural)?;
    let sigma = face_side(ue, iso, b.orientation()).ok_or(Unpresented::Local)?;
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
    // two copies of the face's extent is refused by [`sheets`] before any
    // corner is read.
    if [Axis::U, Axis::V].into_iter().any(|axis| {
        let (lo, hi) = axis.of((x, y));
        periodic(surface, axis) && whole_turn(lo, hi)
    }) {
        return Err(Unpresented::Local);
    }
    let m = |(l, h): (f64, f64)| geom::mid_param(l, h);
    let ball = Ball::holding(surface.point_eval(m(x), m(y)), surface.rect_enclosure(x, y));
    let local = Local::of(b, ball, band);
    // The line admits only the vertex: its far end in `R` would leave the
    // face free to come round it.
    if !only(piece, &local, &[c.read, c.other], &[c.vertex]) {
        return Err(Unpresented::Local);
    }
    let from_start = o.ends[0] == c.vertex;
    let speed = speeds(surface, x, y);
    leaves_line(
        periodic_axes(surface),
        &h,
        (span.0.lo(), span.1.hi()),
        from_start,
        (held, sigma),
        vertex,
        (x, y),
        speed,
        band,
    )?;
    // [`present_chart`]'s sliver, on a rectangle the reading above has
    // proven holds the face's part of the cell; read before that, a
    // reflex corner's thin cell would drop the face beyond the vertex.
    // The strip is within the band of the shared line, and the points it
    // drops lie within the band of it or, by the vertex concession, of
    // the other edge.
    let width = at(across.1 - across.0) * at(speed[held.index()]);
    if matches!(
        decide(SELF_INTERSECTION_SLIVER, Margin::of(width), band),
        Ok(Sign::Zero)
    ) {
        return Ok(Presentation::Vacuous);
    }
    Ok(Presentation::Of(Box::new(Presented {
        surface,
        map: Map::Chart,
        x,
        y,
        zero: Zero::Side(
            Side::of(held, sigma > 0.0),
            Along::over(surface, b, c.read, false, || iso_params(surface, e, along)),
        ),
        corner: true,
        cache: Cache::default(),
    })))
}

/// **Present a cell at a corner by a plane's line edge**
/// ([`present_corner`]), in [`present_plane`]'s affine chart: the line
/// `y = 0`, the face's side `y > 0`. `R` is the cell's affine hull
/// widened to hold the vertex and the line; nothing in reach of it may
/// bound the face but the two edges, every vertex in reach is the
/// corner's, and the other edge's chart curve `h` leaves the vertex into
/// `y > 0` and stays there within `R` ([`leaves_line`]). So `R`'s part
/// below the line and the line's continuation past the vertex hold no
/// boundary, and, meeting the vertex's outside, lie outside the face:
/// the face's part of the cell is in `R` at `y ≥ 0`, and its points on
/// the line are the line's edge. The far end needs no span check: it is
/// out of reach.
fn present_plane_corner<'a>(
    piece: &Piece<'a>,
    c: Corner,
    h: Harmonic,
    band: Band,
) -> Result<Presentation<'a>, Unpresented> {
    let b = piece.boundary;
    let surface = piece.surface;
    let &Surface::Plane {
        origin,
        normal,
        u_ref,
    } = surface
    else {
        return Err(Unpresented::Local);
    };
    let (Some(e), Some(o)) = (b.edges.get(c.read), b.edges.get(c.other)) else {
        return Err(Unpresented::Local);
    };
    let ([ue], Some((_, span))) = (e.uses.as_slice(), o.arc.as_ref()) else {
        return Err(Unpresented::Local);
    };
    let Some((line, (t0, t1))) = e.arc.as_ref().filter(|(k, _)| !k.is_curved()) else {
        return Err(Unpresented::Local);
    };
    let v_ref = normal.cross(u_ref);
    let chart = |w: Vec3<Interval>| [w.dot(u_ref), w.dot(v_ref)];
    let (p, q) = (
        chart(line.eval(*t0) - origin),
        chart(line.eval(*t1) - origin),
    );
    // The line's chart points and parameters in the loop's order.
    let ((p, ta), (q, tb), vertex_first) = if ue.plus {
        ((p, *t0), (q, *t1), e.ends[0] == c.vertex)
    } else {
        ((q, *t1), (p, *t0), e.ends[1] == c.vertex)
    };
    let a = [q[0] - p[0], q[1] - p[1]];
    let bv = toward_face(a, b.orientation());
    let len2 = a[0].powi(2) + a[1].powi(2);
    let lin = |w: [Interval; 2]| {
        [
            (w[0] * a[0] + w[1] * a[1]) / len2,
            (w[0] * bv[0] + w[1] * bv[1]) / len2,
        ]
    };
    let cell = lin([iv(piece.u) - p[0], iv(piece.v) - p[1]]);
    if !(cell[0].is_certified() && cell[1].is_certified()) {
        return Err(Unpresented::Local);
    }
    let vx = if vertex_first { 0.0 } else { 1.0 };
    let x = (cell[0].lo().min(vx), cell[0].hi().max(vx));
    let y = (cell[1].lo().min(0.0), cell[1].hi().max(0.0));
    let affine = Map::Affine { o: p, a, b: bv };
    let reach = Presented {
        surface,
        map: affine,
        x,
        y,
        zero: Zero::None,
        corner: true,
        cache: Cache::default(),
    };
    if !reach_holds_only(piece, &reach, &[c.read, c.other], &[c.vertex], band) {
        return Err(Unpresented::Local);
    }
    let zero = at(0.0);
    let curve = in_frame(&h, p, lin);
    let speed = |w: [Interval; 2]| (u_ref * w[0] + v_ref * w[1]).norm().hi();
    leaves_line(
        [false; 2],
        &ChartCurve::Harmonic(curve),
        (span.0.lo(), span.1.hi()),
        o.ends[0] == c.vertex,
        (Axis::V, 1.0),
        [at(vx), zero],
        (x, y),
        [speed(a), speed(bv)],
        band,
    )?;
    // [`present_line_corner`]'s sliver, read once the reading above has
    // proven `R` holds the face's part of the cell. The points it drops
    // lie within the band of the line or, by the vertex concession, of
    // the other edge.
    let width = at(y.1) * at(speed(bv));
    if matches!(
        decide(SELF_INTERSECTION_SLIVER, Margin::of(width), band),
        Ok(Sign::Zero)
    ) {
        return Ok(Presentation::Vacuous);
    }
    Ok(Presentation::Of(Box::new(Presented {
        surface,
        map: affine,
        x,
        y: (0.0, y.1),
        zero: Zero::Side(
            Side::YLo,
            Along::over(surface, b, c.read, false, || segment_params(ta, tb, x)),
        ),
        corner: true,
        cache: Cache::default(),
    })))
}

/// **A corner at a vertex the pair shares alone**, where an edge
/// straight in the face's chart meets a curved one: the straight edge,
/// the curved one, and the curved one's chart curve. Read on a plane (a
/// line edge and a conic) and on a cylinder (a coordinate line and a
/// harmonic chart curve), the charts that are regular everywhere and
/// whose curved images are harmonic. `None` for any other corner: the
/// vertex not shared alone, a wall on the shared cell, or edges or a
/// surface of other kinds.
fn vertex_corner(
    piece: &Piece<'_>,
    walls: &[usize],
    near_edges: &[usize],
    near_vertices: &[VertexKey],
) -> Option<(VertexKey, usize, usize, Harmonic)> {
    let b = piece.boundary;
    let (&[e, f], &[v]) = (walls, near_vertices) else {
        return None;
    };
    let surface = piece.surface;
    if piece.whole || !matches!(surface, Surface::Plane { .. } | Surface::Cylinder { .. }) {
        return None;
    }
    if near_edges.contains(&e) || near_edges.contains(&f) {
        return None;
    }
    let (re, rf) = (b.edges.get(e)?, b.edges.get(f)?);
    let ends = |r: &Rim| r.ends[0] != r.ends[1] && r.ends.contains(&v);
    if !(ends(re) && ends(rf)) || re.ends.iter().filter(|w| rf.ends.contains(w)).count() != 1 {
        return None;
    }
    let (line, other) = if is_curved(surface, re) {
        (f, e)
    } else {
        (e, f)
    };
    let (rl, ro) = (b.edges.get(line)?, b.edges.get(other)?);
    if is_curved(surface, rl) || !is_curved(surface, ro) {
        return None;
    }
    let [u] = ro.uses.as_slice() else {
        return None;
    };
    Some((v, line, other, chart_curve(surface, ro, u)?))
}

/// **Present a cell at a corner whose vertex `V` is the shared cell
/// alone** ([`vertex_corner`]), as the quadrant of an affine chart about
/// `V` with its corner the zero (`Zero::Point`, as [`present_plane`]
/// reads a corner of two lines).
///
/// The quadrant's first axis `A` is the straight edge's chart direction
/// away from `V`, so that edge is `y = 0`, `x ≥ 0`; its second `B` leans
/// back off the curve's chart tangent at `V` by a quarter of `A`, the
/// two measured in the surface's metric at `V`. Soundness does not rest
/// on that choice, since everything below is decided for whatever `B`
/// is; whether the corner is read at all does. With the lean in chart
/// units, a thin cylinder's quadrant opens to nearly a straight angle
/// on the surface, or is not presented. `V` is the straight edge's
/// chart point at it, moved by whole turns nearest the cell. `R` is the
/// cell's affine hull widened to hold `V`, its chart footprint narrower
/// than a turn on each periodic axis. Nothing in reach of it bounds the
/// face but the two edges, `V` the only vertex ([`only`]); no copy of
/// the curve by whole turns but the one through `V` meets the footprint
/// ([`copy_misses`]); `B` lies on the edge's face side; and the curve's
/// copy through `V` leaves it into `y > 0` and `x > 0` and stays there
/// within `R` ([`leaves_line`], once per axis).
///
/// **No copy of the straight edge but `V`'s meets `R`**, without a scan:
/// `R` is convex and holds `V`. A coordinate line's copy held a turn
/// away on its held axis meets `R` only if the footprint spans a turn,
/// which is refused. A copy moved a turn along its own line meets `R`,
/// on that line, in an interval holding `V` and shorter than a turn, so
/// only with its far end inside it: a vertex [`only`] refuses. The edge
/// itself is shorter than a turn ([`straight`] reads no full line). A
/// plane's chart has no copies.
///
/// **Why the face's part of the cell lies in the quadrant `Q`**
/// (`x ≥ 0, y ≥ 0`): the face's preimage in the chart, every copy of it
/// by whole turns, is bounded by the copies of its edges. In `R` those
/// are the straight edge's copy through `V`, on `Q`'s side `y = 0`, and
/// the curve's, in `Q`'s interior. So `R` off `Q`, which is connected,
/// holds none of the boundary, and it meets the outside of the face
/// near `V`, whose face side is the sector from `A` to the curve's
/// tangent through `y > 0`, inside `Q`. So it lies outside the face. The
/// cell lies in `R`, so the face's part of it is in `R ∩ Q`, and of
/// `Q`'s two sides only their common corner `V` is on the shared cell.
/// A curve leaving into the edge's other side is a reflex corner, which
/// this reads at no depth: `Structural`.
fn present_vertex_corner<'a>(
    piece: &Piece<'a>,
    v: VertexKey,
    line: usize,
    other: usize,
    h: Harmonic,
    band: Band,
) -> Result<Presentation<'a>, Unpresented> {
    let b = piece.boundary;
    let surface = piece.surface;
    let (Some(e), Some(o)) = (b.edges.get(line), b.edges.get(other)) else {
        return Err(Unpresented::Local);
    };
    let ([ue], Some((_, span))) = (e.uses.as_slice(), o.arc.as_ref()) else {
        return Err(Unpresented::Local);
    };
    let periodic = periodic_axes(surface);
    let (a, [near, _]) = straight(surface, e, v).ok_or(Unpresented::Local)?;
    let raw = [near.x, near.y];
    let cell_mid = [
        at(geom::mid_param(piece.u.0, piece.u.1)),
        at(geom::mid_param(piece.v.0, piece.v.1)),
    ];
    let k = turns_to(periodic, raw, cell_mid);
    let vc = shifted(raw, k);
    // The face's side of the straight edge, as a chart direction: its
    // traversal turned a quarter toward the interior, on a plane's chart
    // as on a cylinder's.
    let start = if ue.plus { e.ends[0] } else { e.ends[1] };
    let tau = if start == v { a } else { a.map(|x| -x) };
    let n = toward_face(tau, b.orientation());
    let (t0, t1) = (span.0.lo(), span.1.hi());
    let from_start = o.ends[0] == v;
    let t_v = if from_start { t0 } else { t1 };
    let away = if from_start { 1.0 } else { -1.0 };
    // Lengths in the chart's metric at `V`, so the lean below is an
    // angle on the surface, not in the chart's units.
    let jet = surface.jet(vc[0], vc[1]);
    let (du, dv) = (vmid(jet.du), vmid(jet.dv));
    let length = |w: [f64; 2]| (du * w[0] + dv * w[1]).norm();
    let unit2 = |w: [f64; 2]| {
        let l = length(w);
        (l.is_finite() && l > 0.0).then(|| [w[0] / l, w[1] / l])
    };
    let a_mid = [mid(a[0]), mid(a[1])];
    let d1 = unit2(a_mid).ok_or(Unpresented::Local)?;
    let tv = at(t_v);
    let d2 = unit2(core::array::from_fn(|i| {
        away * mid(-h.pa[i] * tv.sin() + h.pb[i] * tv.cos() + h.pl[i])
    }))
    .ok_or(Unpresented::Local)?;
    // `B` leans back off the curve's tangent by a quarter of `A`, so the
    // tangent is `A/4 + B`, strictly inside the quadrant.
    let len = length(a_mid);
    let bv = [
        at((d2[0] - 0.25 * d1[0]) * len),
        at((d2[1] - 0.25 * d1[1]) * len),
    ];
    let lean = (n[0] * bv[0] + n[1] * bv[1]) / (n[0].powi(2) + n[1].powi(2)).sqrt();
    match interval_sign_as(lean, SELF_INTERSECTION_CORNER, band) {
        Some(Sign::Positive) => {}
        Some(Sign::Negative) => return Err(Unpresented::Structural),
        _ => return Err(Unpresented::Local),
    }
    let det = a[0] * bv[1] - a[1] * bv[0];
    let lin = |w: [Interval; 2]| {
        [
            (w[0] * bv[1] - w[1] * bv[0]) / det,
            (a[0] * w[1] - a[1] * w[0]) / det,
        ]
    };
    let cell = lin([iv(piece.u) - vc[0], iv(piece.v) - vc[1]]);
    if !(cell[0].is_certified() && cell[1].is_certified()) {
        return Err(Unpresented::Local);
    }
    let x = (cell[0].lo().min(0.0), cell[0].hi().max(0.0));
    let y = (cell[1].lo().min(0.0), cell[1].hi().max(0.0));
    if !(x.1 > 0.0 && y.1 > 0.0) {
        return Err(Unpresented::Local);
    }
    let affine = Map::Affine { o: vc, a, b: bv };
    let reach = Presented {
        surface,
        map: affine,
        x,
        y,
        zero: Zero::None,
        corner: true,
        cache: Cache::default(),
    };
    let (fu, fv) = reach.chart(iv(x), iv(y));
    if !(fu.is_certified() && fv.is_certified()) {
        return Err(Unpresented::Local);
    }
    let footprint = ((fu.lo(), fu.hi()), (fv.lo(), fv.hi()));
    // The premise of the straight edge's copies (the doc above). No row
    // reaches it alone: a cell a turn wide meets two copies of the face's
    // extent and [`sheets`] refuses it first, and `R`'s footprint along a
    // coordinate line's held axis is the hull of the cell and `V`.
    if [Axis::U, Axis::V].into_iter().any(|axis| {
        let (lo, hi) = axis.of(footprint);
        periodic[axis.index()] && whole_turn(lo, hi)
    }) {
        return Err(Unpresented::Local);
    }
    let chart = ChartCurve::Harmonic(h);
    let kc = turns_to(periodic, chart.at(tv), vc);
    let curve_reach = reach_of(pieces_of((t0, t1)).iter().map(|&t| chart.at(t)));
    for c in copies(periodic, curve_reach, footprint).ok_or(Unpresented::Local)? {
        if c != kc && !copy_misses(&chart, (t0, t1), c, footprint) {
            return Err(Unpresented::Local);
        }
    }
    if !reach_holds_only(piece, &reach, &[line, other], &[v], band) {
        return Err(Unpresented::Local);
    }
    let curve = ChartCurve::Harmonic(in_frame(&h, shifted(vc, kc.map(|k| -k)), lin));
    let (fx, fy, _) = reach.jet();
    let speeds = [fx.norm().hi(), fy.norm().hi()];
    let zero = at(0.0);
    let leaves = |held: Axis| {
        leaves_line(
            [false; 2],
            &curve,
            (t0, t1),
            from_start,
            (held, 1.0),
            [zero, zero],
            (x, y),
            speeds,
            band,
        )
    };
    leaves(Axis::V)?;
    // The curve's tangent at `V` is inside `Q` by `B`'s choice, so a
    // decided fall in `x` there is no reflex corner: not read here.
    leaves(Axis::U).map_err(|_| Unpresented::Local)?;
    Ok(Presentation::Of(Box::new(Presented {
        surface,
        map: affine,
        x: (0.0, x.1),
        y: (0.0, y.1),
        zero: Zero::Point(Side::XLo, Side::YLo, v),
        corner: true,
        cache: Cache::default(),
    })))
}

/// **Whether no piece of the chart curve `h` over `(t0, t1)`, moved `c`
/// whole turns, meets the rectangle `r`**: each of [`PIECES`] pieces
/// that is not told is halved, [`LEAVES`] times at most, and one still
/// meeting `r` then is `false`. The scan of a curve's copies by whole
/// turns, for [`leaves_line`] and [`present_vertex_corner`];
/// [`leaves_graph`]'s straight edge is one segment, read by its hull.
fn copy_misses(
    h: &ChartCurve,
    (t0, t1): (f64, f64),
    c: [f64; 2],
    r: ((f64, f64), (f64, f64)),
) -> bool {
    let mut todo: Vec<((f64, f64), u32)> = pieces_of((t0, t1))
        .iter()
        .map(|t| ((t.lo(), t.hi()), 0))
        .collect();
    while let Some(((p, q), depth)) = todo.pop() {
        if !meets_rect(shifted(h.at(Interval::from_bounds(p, q)), c), r) {
            continue;
        }
        let m = geom::mid_param(p, q);
        if depth >= LEAVES || !(p < m && m < q) {
            return false;
        }
        todo.extend([((p, m), depth + 1), ((m, q), depth + 1)]);
    }
    true
}

/// **A harmonic chart curve in an affine frame**: `h` less the frame's
/// origin `o`, every coefficient carried by the frame's linear map `lin`
/// (its linear term too, so a helix keeps its pitch).
fn in_frame(
    h: &Harmonic,
    o: [Interval; 2],
    lin: impl Fn([Interval; 2]) -> [Interval; 2],
) -> Harmonic {
    Harmonic {
        p0: lin([h.p0[0] - o[0], h.p0[1] - o[1]]),
        pa: lin(h.pa),
        pb: lin(h.pb),
        pl: lin(h.pl),
    }
}

/// **Whether nothing in reach of a corner presentation's rectangle
/// `reach` bounds the face but `edges`, its only vertices `vertices`**
/// ([`only`], over the ball holding `reach`'s image).
fn reach_holds_only(
    piece: &Piece<'_>,
    reach: &Presented<'_>,
    edges: &[usize],
    vertices: &[VertexKey],
    band: Band,
) -> bool {
    let m = |(l, h): (f64, f64)| geom::mid_param(l, h);
    let (x, y) = (reach.x, reach.y);
    let ball = Ball::holding(reach.image((m(x), m(x)), (m(y), m(y))), reach.whole());
    only(
        piece,
        &Local::of(piece.boundary, ball, band),
        edges,
        vertices,
    )
}

/// **The corner's other edge leaves the line toward the face's side and
/// does not come back within `R`** ([`present_corner`]'s step 1, for a
/// line): every piece of its chart curve `h`, on every copy by whole
/// turns, that meets `R` is one of a run of consecutive pieces from the
/// vertex, on the copy through it, over each of which the curve's
/// height above the line — zero at the vertex — strictly grows, decided
/// at [`SELF_INTERSECTION_CORNER_DEPARTS`] off its rate on the held axis times the piece's
/// width and `lever`. A piece that is not told is halved, [`LEAVES`]
/// times at most. A reflex corner, decided at the vertex, is
/// `Structural`; anything else not read, `Local`.
#[allow(clippy::too_many_arguments)]
fn leaves_line(
    periodic: [bool; 2],
    h: &ChartCurve,
    (t0, t1): (f64, f64),
    from_start: bool,
    (held, sigma): (Axis, f64),
    vertex: [Interval; 2],
    r: ((f64, f64), (f64, f64)),
    speed: [f64; 2],
    band: Band,
) -> Result<(), Unpresented> {
    let t_v = if from_start { t0 } else { t1 };
    let start = h.at(at(t_v));
    let k = turns_to(periodic, start, vertex);
    if !on_vertex(shifted(start, k), vertex, speed, band) {
        return Err(Unpresented::Local);
    }
    let lever = speed[held.index()];
    let away = if from_start { 1.0 } else { -1.0 };
    let width = t1 - t0;
    if !(width.is_finite() && width > 0.0) {
        return Err(Unpresented::Local);
    }
    let pieces = pieces_of((t0, t1));
    let reach = reach_of(pieces.iter().map(|&t| h.at(t)));
    // Two guards, each load-bearing. The rate keeps the run from the
    // vertex on the face's side while it is in `R`; `gone` keeps a piece
    // that comes back into `R` after the run has left — from any side, at
    // any rate — from being read as the run.
    for c in copies(periodic, reach, r).ok_or(Unpresented::Local)? {
        if c != k {
            if copy_misses(h, (t0, t1), c, r) {
                continue;
            }
            return Err(Unpresented::Local);
        }
        // The pieces in order from the vertex, each with its depth.
        let mut todo: Vec<((f64, f64), u32)> =
            pieces.iter().map(|t| ((t.lo(), t.hi()), 0)).collect();
        if from_start {
            todo.reverse();
        }
        let mut gone = false;
        while let Some(((a, z), depth)) = todo.pop() {
            let t = Interval::from_bounds(a, z);
            if !meets_rect(shifted(h.at(t), c), r) {
                gone = true;
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
            if gone {
                if split(&mut todo) {
                    continue;
                }
                return Err(Unpresented::Local);
            }
            let rise = h.rate(held, t) * at(sigma * away * (z - a) * lever);
            match decide(SELF_INTERSECTION_CORNER_DEPARTS, Margin::of(rise), band) {
                Ok(Sign::Positive) => {}
                Ok(Sign::Negative) if (if from_start { a } else { z }) == t_v => {
                    return Err(Unpresented::Structural);
                }
                // Off the vertex a decided `Negative` is refused outright.
                // It is not reached otherwise — the rate is continuous, so
                // the run meets a piece the funnel cannot sign first, which
                // refuses at the [`LEAVES`] cap, and a piece past the run's
                // exit is refused by `gone` before its rate is read — but
                // the refusal does not rest on those three.
                Ok(Sign::Negative) => return Err(Unpresented::Local),
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
        #[allow(clippy::cast_possible_truncation)]
        let wind = chart.pl[column.index()].lo() as i64;
        Self {
            chart,
            column,
            t: pieces_of((t0, t1)),
            width: t1 - t0,
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

/// A span `(t0, t1)` cut into [`PIECES`] pieces, in order.
fn pieces_of((t0, t1): (f64, f64)) -> Vec<Interval> {
    let width = t1 - t0;
    #[allow(clippy::cast_precision_loss)]
    let bound = |i: usize| {
        if i == PIECES {
            t1
        } else {
            t0 + width * (i as f64) / (PIECES as f64)
        }
    };
    (0..PIECES)
        .map(|i| Interval::from_bounds(bound(i), bound(i + 1)))
        .collect()
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
        Some((vc, vo)) => match column {
            Axis::U => (widened(own.0, vc), widened(own.1, vo)),
            Axis::V => (widened(own.0, vo), widened(own.1, vc)),
        },
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
    let lever = speeds(surface, cell.0, cell.1)[column.index()];
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
    if periodic(surface, other) && whole_turn(rb.0, rb.1) {
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
        let m = |(l, h): (f64, f64)| geom::mid_param(l, h);
        let ball = Ball::holding(surface.point_eval(m(x), m(y)), surface.rect_enclosure(x, y));
        Local::of(piece.boundary, ball, band)
    };
    if !alone(piece, ei, through.map(|th| th.corner.other), &local) {
        return None;
    }
    // Step 3: the face's side.
    let above = piece
        .boundary
        .orientation()
        .interior_is_on(left_is_above(rising == plus, column));
    if let Some(th) = through {
        // The vertex is in `R`, on the run read: its parameter in the
        // run's, and its point on the run's copy. The cell was widened to
        // the vertex, so the vertex's own run meets it; a run read that is
        // not that one has it among `others`, which `apart` refused above.
        // So this is not load-bearing; it states step 3's premise.
        let ends = Ends::of(p, &run, rising, (a0, a1));
        let (ts, te) = (p.params(run.first()?).lo(), p.params(run.last()?).hi());
        let held = nearest(p.chart.on(other, at(th.t)));
        if !(ts <= th.t && th.t <= te && within(ends.along(th.t), (a0, a1)) && within(held, rb)) {
            return None;
        }
        let ts = Interval::from_bounds(ta, tb);
        let vertex = match column {
            Axis::U => [ends.along(th.t), held],
            Axis::V => [held, ends.along(th.t)],
        };
        let tau = (p.chart.rate(column, ts), p.chart.rate(other, ts));
        // An arc of the curve's own conic is on the run read: nothing
        // leaves it ([`Reading::Conic`]).
        if !th.conic
            && let Err(e) = leaves_graph(
                piece,
                th,
                column,
                tau,
                ts,
                rising,
                above,
                vertex,
                (x, y),
                band,
            )?
        {
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
/// each piece's advance along the column axis decided at [`SELF_INTERSECTION_COLUMN`],
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
        interval_sign_as(advance, SELF_INTERSECTION_COLUMN, band)
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
/// test for one apart from it.
///
/// **Columns wider than a turn on a periodic column axis** are refused
/// here too. Some column `a` has `a` plus a turn a column as well. The
/// run read passes both column ends ([`monotone_run`]), so it holds a
/// point at `a`, at a height in `rb`. Its lift a turn over is another run
/// ([`runs`] lists each lift's pieces apart), and it holds the same point
/// at `a` plus a turn, at the same height. So that run meets `rb`, and it
/// is not apart. The chosen run's own copies need no
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
            || (!piece.whole && b.edges.get(j).is_some_and(|r| r.arc.is_some() && seam(r)))
    }) && local.vertices.iter().all(|v| vertices.contains(v))
}

/// **Whether a graph's traversal has its left above it** in the chart,
/// "above" being the far side of the other axis ([`present_wall`]'s and
/// [`graph_read`]'s step 3, which read the face's side off this and the
/// loop's orientation). Travel along the column axis is `forward` where
/// the curve rises with its parameter and the loop runs it forward.
fn left_is_above(forward: bool, column: Axis) -> bool {
    let mut tau = [0_i8; 2];
    tau[column.index()] = if forward { 1 } else { -1 };
    left_turn(tau)[column.other().index()] > 0
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
                geom::Line {
                    origin: o,
                    dir: axis,
                }
                .distance(p)
                    - r
            }
            Self::Carrier(Implicit::Sphere { c, r }) => (p - c).norm() - r,
            Self::Carrier(Implicit::Cone {
                apex,
                axis,
                cos,
                sin,
                nappe,
            }) => {
                let (h, rho) = geom::Line {
                    origin: apex,
                    dir: axis,
                }
                .height_and_distance(p);
                rho * cos - nappe * h * sin
            }
            Self::Graph { .. } => return None,
        })
    }

    /// `∇ψ` over a box.
    fn gradient(&self, p: Point3<Interval>) -> Vec3<Interval> {
        match *self {
            Self::Linear { n, .. } | Self::Carrier(Implicit::Plane { n, .. }) => n,
            Self::Carrier(Implicit::Cylinder { o, axis, .. }) => {
                let radial = geom::Line {
                    origin: o,
                    dir: axis,
                }
                .offset(p);
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
                let radial = geom::Line {
                    origin: apex,
                    dir: axis,
                }
                .offset(p);
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

/// `ψ∘Φ` over a side `x × y` of the presentation. On an analytic chart
/// whose coordinate line there is a circle ([`iso_circle`]), `ψ` along
/// that circle's closed form ([`Trig::psi`]): a side lying in `ψ`'s zero
/// set reads rounding at any width, where the centred form's error
/// grows with the side's length squared. A linear `ψ` on a spline chart
/// is the hull of its values at the side's restricted control points
/// ([`geom::NurbsSurface::rect_net`]): `ψ` is affine and the side's
/// points are convex combinations of those, so the hull encloses it,
/// and an iso lying in `ψ`'s zero set reads zero at any width. Anywhere
/// else, and where neither form reads it (the net door refusing a
/// restricted weight not certified positive, or a cone's lever not
/// certified positive), the centred form.
fn on_side(psi: &Psi, p: &Presented<'_>, x: (f64, f64), y: (f64, f64)) -> Option<Interval> {
    if let Map::Chart = p.map {
        let circle = match (x.0 == x.1, y.0 == y.1) {
            (true, false) => iso_circle(p.surface, Axis::U, x.0).map(|c| (c, y)),
            (false, true) => iso_circle(p.surface, Axis::V, y.0).map(|c| (c, x)),
            _ => None,
        };
        if let Some(value) = circle.and_then(|(c, t)| c.psi(psi, iv(t))) {
            return Some(value);
        }
    }
    let net = match (&p.map, p.surface) {
        (Map::Chart, Surface::Nurbs(net)) if psi.linear().is_some() => net.rect_net(x, y),
        _ => None,
    };
    match net {
        Some(points) => {
            let values = points.into_iter().map(|q| psi.value(q));
            values.collect::<Option<Vec<_>>>()?.into_iter().reduce(hull)
        }
        None => centred(psi, p, x, y),
    }
}

/// **The image of an analytic chart's coordinate line `held = b`, where
/// it is a circle**, in the moving coordinate: a torus's meridians and
/// parallels, a sphere's, and a cylinder's or a cone's parallels. `None`
/// for a ruling, which is a line, and for every other carrier.
fn iso_circle(surface: &Surface<Interval>, held: Axis, b: f64) -> Option<Trig> {
    let b = at(b);
    let ring = |frame: geom::Frame<Interval>, c: Point3<Interval>, r: Interval| Trig {
        c,
        a: frame.u_ref * r,
        b: frame.v_ref() * r,
    };
    match (surface, held) {
        (
            &Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            },
            Axis::U,
        ) => {
            let radial = geom::Frame::new(center, axis, u_ref).radial(b);
            Some(Trig {
                c: center + radial * major_radius,
                a: radial * minor_radius,
                b: axis * minor_radius,
            })
        }
        (
            &Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            },
            Axis::V,
        ) => {
            let (s, c) = b.sin_cos();
            Some(ring(
                geom::Frame::new(center, axis, u_ref),
                center + axis * (minor_radius * s),
                major_radius + minor_radius * c,
            ))
        }
        (
            &Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            },
            Axis::U,
        ) => {
            let radial = geom::Frame::new(center, axis, u_ref).radial(b);
            Some(Trig {
                c: center,
                a: radial * radius,
                b: axis * radius,
            })
        }
        (
            &Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            },
            Axis::V,
        ) => {
            let (s, c) = b.sin_cos();
            Some(ring(
                geom::Frame::new(center, axis, u_ref),
                center + axis * (radius * s),
                radius * c,
            ))
        }
        (
            &Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            },
            Axis::V,
        ) => Some(ring(
            geom::Frame::new(origin, axis, u_ref),
            origin + axis * b,
            radius,
        )),
        (
            &Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            },
            Axis::V,
        ) => {
            let (s, c) = half_angle.sin_cos();
            Some(ring(
                geom::Frame::new(apex, axis, u_ref),
                apex + axis * (b * c),
                b * s,
            ))
        }
        _ => None,
    }
}

/// **The cells a separator the engine builds vanishes on**, by its
/// construction from them ([`planes_through`], [`graph_of`]): edges by
/// key, vertices, and whether it holds each listed edge's whole line or
/// circle (a plane does; the graph separator vanishes on its edge over
/// its piece alone).
#[derive(Clone, Debug, Default)]
struct On {
    edges: Vec<EdgeKey>,
    vertices: Vec<VertexKey>,
    whole: bool,
}

impl On {
    /// Whether the separator vanishes on every cell `a` lies on; with
    /// `whole`, on the side's whole chart line too.
    fn holds(&self, a: &Along, whole: bool) -> bool {
        (a.vertex.is_some() || !a.edges.is_empty())
            && a.edges.iter().all(|e| self.edges.contains(e))
            && a.vertex.is_none_or(|v| self.vertices.contains(&v))
            && (!whole || (self.whole && a.whole))
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
/// - A curve, a graph over one axis: zero along it, so its sign on the
///   face's side is the sign of the derivative along the other axis,
///   over the whole rectangle ([`present_curved`]).
///
/// **Where `ψ` is zero.** A separator the engine builds carries the
/// cells it vanishes on (`on`), and a zero side, corner point or curve
/// is zero under it where it lies on those cells: membership, no value
/// read, and:
/// - **a side on its edges** ([`Along::on_edges`]) needs nothing more
///   under a plane;
/// - **a side or curve that may run past its edges** (a corner
///   presentation's side, past its vertex, [`present_corner`]; a side
///   past the shared cell; two sides at a corner; a curve) is zero under
///   a plane only where the plane holds its edges' whole line or circle
///   and that is the side's own chart line;
/// - **under the graph separator** every side and curve is read by one
///   rule, [`Along::under_graph`]: a whole one where its parameters lie
///   in the graph's piece, one whose chart line is not its edge's own
///   line or circle where it lies on its edge, and a whole one whose
///   parameters were not read nowhere.
///
/// The graph vanishes on its edge's carrier over the piece and nowhere
/// else. Past the piece its extension leaves a circle by second order in
/// the distance `δ` past it, about `κ·δ²`, against a departure of first
/// order, so a side read zero past the piece would misread `ψ`'s sign
/// only across a sliver about `κ·δ²/(lean·|∇ψ|)` wide past the vertex,
/// and that is not proven empty. [`graph_of`] widens the piece over every
/// whole side and curve on its edge instead.
///
/// A face's own carrier (`on` `None`, the carrier arm) is read over the
/// side (`SELF_INTERSECTION_ON_SHARED`).
///
/// **A side that runs past the shared cell** ([`Zero::Past`]) is read
/// as a side only `against` another presentation whose zero set is on
/// the shared cell (never in the carrier arm, where `against` is `None`
/// and every point of the separating face is on its zero set), under a
/// plane (not the graph's), on an analytic face: there a coincidence off
/// the shared cell would be two points of this face with one image
/// ([`partial_zero`]). A curve read on past the shared edge (a `past`
/// [`Curved`]) is read under the same guard, [`past_read`], which holds
/// its argument. Anywhere else either answers nothing.
fn sign_on(
    psi: &Psi,
    on: Option<&On>,
    p: &Presented<'_>,
    against: Option<&Presented<'_>>,
    band: Band,
) -> Option<Sign> {
    let read = |s: Side| {
        let b = p.bound(s);
        let (x, y) = match s.axis() {
            Axis::U => ((b, b), p.y),
            Axis::V => (p.x, (b, b)),
        };
        on_side(psi, p, x, y).is_some_and(|v| is_zero(v, band))
    };
    let piece = match *psi {
        Psi::Graph { piece, .. } => Some(piece),
        _ => None,
    };
    let zero_on = |s: Side, a: &Along, whole: bool| {
        on.map_or_else(
            || read(s),
            |on| {
                on.holds(a, whole)
                    && match piece {
                        Some(p) => a.under_graph(p),
                        None => a.on_edges || (on.whole && a.whole),
                    }
            },
        )
    };
    let (fx, fy, fxy) = p.jet();
    let image = p.whole();
    let grad = psi.gradient(image);
    let across = |axis: Axis| match axis {
        Axis::U => grad.dot(fx) * at(p.span(Axis::U)),
        Axis::V => grad.dot(fy) * at(p.span(Axis::V)),
    };
    let departs = |s: Side| {
        interval_sign_as(
            across(s.axis()) * at(s.into()),
            SELF_INTERSECTION_DEPARTURE,
            band,
        )
    };
    match &p.zero {
        Zero::None => interval_sign_as(
            centred(psi, p, p.x, p.y)?,
            SELF_INTERSECTION_DEPARTURE,
            band,
        ),
        Zero::Side(s, a) => {
            if !zero_on(*s, a, false) {
                return None;
            }
            departs(*s)
        }
        Zero::Past(s, a) => {
            if !(past_read(p, against) && zero_on(*s, a, true)) {
                return None;
            }
            departs(*s)
        }
        Zero::Corner((s, a), (t, b)) => {
            let n = psi.linear()?;
            if !(zero_on(*s, a, true) && zero_on(*t, b, true)) {
                return None;
            }
            let g = n.dot(fxy)
                * at(p.span(Axis::U) * p.span(Axis::V) * Side::into(*s) * Side::into(*t));
            interval_sign_as(g, SELF_INTERSECTION_DEPARTURE, band)
        }
        Zero::Curve(s, c) => {
            if c.past.is_some() && !past_read(p, against) {
                return None;
            }
            let zero = match (on, psi) {
                (Some(on), Psi::Graph { piece, .. }) => {
                    on.holds(&c.along, false) && c.along.under_graph(*piece)
                }
                (Some(on), _) => on.holds(&c.along, true),
                (None, _) => c.psi(psi).is_some_and(|v| is_zero(v, band)),
            };
            if !zero {
                return None;
            }
            departs(*s)
        }
        Zero::Point(s, t, v) => {
            let zero = match on {
                Some(on) => on.holds(&Along::at(*v), false),
                None => psi
                    .value(p.corner(*s, *t))
                    .is_some_and(|x| is_zero(x, band)),
            };
            if !zero {
                return None;
            }
            let a = departs(*s)?;
            let b = departs(*t)?;
            (a == b).then_some(a)
        }
    }
}

/// **Whether a zero side or curve that runs past the shared cell is
/// read** ([`Zero::Past`], a past [`Curved`]): `against` a presentation
/// whose zero set is on the shared cell, on an analytic face. So never
/// in the carrier arm, where `against` is `None` and the separating
/// face's every point is on its zero set.
///
/// Where both are zero, a point lies on the other face's zero set, so on
/// the shared cell, and on this face's zero in `R`:
/// - **a past curve**: on its edge `E` or on `O`, `E`'s continuation
///   ([`continues`]). Nothing of the shared cell but `E` and its ends is
///   in `R`'s reach ([`alone`]), and `O` meets `E` only at their common
///   ends, so the point is on `E` or one of its ends: the shared cell.
///   Where the shared cell is the vertex `V` alone, neither arc on it,
///   the point's image is `V`, and of the run only `V` itself has that
///   image: the run lies within the span read, less than a turn and a
///   little more centred on `V` ([`through_of`]), over which the conic
///   meets `V` once. This rests on [`continues`], checked in the
///   reading, and not on the face's injectivity;
/// - **a past side**: a point of the side off the shared cell with the
///   other face's image would be two points of this face with one
///   image, which its injectivity by kind rules out ([`partial_zero`]).
fn past_read(p: &Presented<'_>, against: Option<&Presented<'_>>) -> bool {
    // The partner's zero set is on the shared cell: no zero, a side or
    // corner on shared edges, a shared vertex, a curve on the shared edge.
    // Matched without a wildcard, so a new kind of zero is decided here.
    let exact = against.is_some_and(|q| match &q.zero {
        Zero::None | Zero::Side(..) | Zero::Corner(..) | Zero::Point(..) => true,
        Zero::Curve(_, c) => c.past.is_none(),
        Zero::Past(..) => false,
    });
    exact && geom_brep::is_analytic(p.surface)
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
    // A wall image is read against a plane's carrier only
    // ([`WallImage::psi`]): across a shared spiric between two faces
    // neither of which is a plane, no arm certifies at any depth.
    let walled = |p: &Piece<'_>, edges: &[usize]| {
        edges.iter().any(|&i| {
            p.boundary
                .edges
                .get(i)
                .is_some_and(|e| wall_image(p.surface, e).is_some())
        })
    };
    let plane = |p: &Piece<'_>| matches!(p.surface, Surface::Plane { .. });
    if (walled(x, &ex) || walled(y, &ey)) && !plane(x) && !plane(y) {
        return Outcome::Refuse(EngineRefusal::Uncertified {
            a: x.face,
            b: y.face,
            shared,
            arm: CertificateArm::Departure,
        });
    }
    let (sx, sy) = (Poses::default(), Poses::default());
    let px = present(x, &ex, &near.vertices, band, &sx);
    let py = present(y, &ey, &near.vertices, band, &sy);
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
                && sign_on(&Psi::Carrier(imp), None, p, None, band).is_some()
            {
                return Outcome::Certified { corner: p.corner };
            }
        }
    }
    if stuck(x, &px) && stuck(y, &py) {
        return Outcome::Refuse(EngineRefusal::Uncertified {
            a: x.face,
            b: y.face,
            shared,
            arm: CertificateArm::Departure,
        });
    }
    let (Ok(Presentation::Of(qx)), Ok(Presentation::Of(qy))) = (&px, &py) else {
        return split;
    };
    let opposite = |(psi, on): (Psi, On)| match (
        sign_on(&psi, Some(&on), qx, Some(qy), band),
        sign_on(&psi, Some(&on), qy, Some(qx), band),
    ) {
        (Some(a), Some(b)) => a == flip(b),
        _ => false,
    };
    let corner = qx.corner || qy.corner;
    if planes_through(x, near, qx, qy).into_iter().any(opposite) {
        return Outcome::Certified { corner };
    }
    if graph_of(x, y, near, qx, qy, band).is_some_and(opposite) {
        return Outcome::Certified { corner };
    }
    if let Some(shared) = cusp(x, near, qx, qy, y.boundary.sense, band) {
        return Outcome::Cusp { shared };
    }
    split
}

/// **Whether a face's part of a cell pair is refused as unreadable**:
/// it is not presented for a reason no smaller cell changes
/// ([`Unpresented::Structural`]), or it is presented by a curve run on
/// past the shared cell through a vertex in its reach. Every smaller
/// cell holding that vertex reaches both its arcs, so it is read past
/// the shared cell again or not at all, and a past curve is read only
/// against a presented partner whose zero set is on the shared cell
/// ([`past_read`]), which no stuck partner is. A pair both of whose
/// parts are stuck is refused as two `Structural` parts are: a split
/// could still part the two corners where they are at different
/// vertices, which this does not wait for. Such a part is the corner
/// [`beside_curve`] types `Structural` where the two arcs are not read.
fn stuck(piece: &Piece<'_>, p: &Result<Presentation<'_>, Unpresented>) -> bool {
    match p {
        Err(Unpresented::Structural) => true,
        Ok(Presentation::Of(q)) => matches!(&q.zero,
            Zero::Curve(_, c) if c.past.is_some_and(|v| piece.local.vertices.contains(&v))),
        _ => false,
    }
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

/// **The planes through the shared cell, built from the shared edges'
/// own carriers**, each with the cells it vanishes on ([`On`]):
///
/// - where every shared edge is a planar conic, each one's own plane in
///   turn: `n` its stored axis, `q` its stored centre. It lists that
///   conic, every other whose stored plane is the same value
///   ([`one_plane`]), and their ends; a plane already listed is not
///   built twice;
/// - through one line edge: `n` a chosen direction bisecting the faces'
///   headings about the line, less its part along the line's stored
///   direction, `q` a point of the line. It lists the edge and its ends;
/// - through two line edges with a common end, at a shared vertex: `n`
///   their stored directions' cross product, `q` a point of the first.
///   It lists both and their ends;
/// - at a shared vertex alone, through the vertex, `n` across the faces'
///   two headings. It lists the vertex.
///
/// Every value is computed at the lane scalar from the stored carrier,
/// so each member of a family has its own `n` and `q` in these
/// enclosures, normal to its own edge and on it: the plane vanishes on
/// the listed cells for every member. The headings and the bisector only
/// choose a direction; any direction is exact. A shared vertex off the
/// lines a plane was built through is not listed.
fn planes_through(
    x: &Piece<'_>,
    near: &Near,
    px: &Presented<'_>,
    py: &Presented<'_>,
) -> Vec<(Psi, On)> {
    let b = x.boundary;
    let Some(rims) = near
        .edges
        .iter()
        .map(|&(i, _)| b.edges.get(i))
        .collect::<Option<Vec<&Rim>>>()
    else {
        return Vec::new();
    };
    let through = |rims: &[&Rim]| On {
        edges: rims.iter().map(|r| r.key).collect(),
        vertices: rims.iter().flat_map(|r| r.ends).collect(),
        whole: true,
    };
    let conic = |r: &Rim| match r.arc.as_ref()? {
        (
            Curve3::Circle(geom::Circle {
                frame: geom::Frame { origin, axis, .. },
                ..
            })
            | Curve3::Ellipse(geom::Ellipse {
                frame: geom::Frame { origin, axis, .. },
                ..
            }),
            _,
        ) => Some((*origin, *axis)),
        _ => None,
    };
    if !rims.is_empty()
        && let Some(frames) = rims.iter().map(|r| conic(r)).collect::<Option<Vec<_>>>()
    {
        return frames
            .iter()
            .enumerate()
            .filter(|&(i, &f)| !frames[..i].iter().any(|&g| one_plane(g, f)))
            .map(|(i, &(q, n))| {
                let held: Vec<&Rim> = rims
                    .iter()
                    .zip(&frames)
                    .enumerate()
                    .filter(|&(j, (_, &g))| j == i || one_plane((q, n), g))
                    .map(|(_, (r, _))| *r)
                    .collect();
                (Psi::Linear { n, q }, through(&held))
            })
            .collect();
    }
    let lines: Option<Vec<(Point3<Interval>, Vec3<Interval>)>> = rims
        .iter()
        .map(|r| match r.arc.as_ref()? {
            (line @ Curve3::Line(geom::Line { dir, .. }), (t0, t1)) => {
                Some((line.eval(at(geom::mid_param(t0.lo(), t1.hi()))), *dir))
            }
            _ => None,
        })
        .collect();
    let plane = || {
        let lines = lines?;
        let q0 = pmid(anchor(x, near)?);
        let (hx, hy) = (heading(px, q0)?, heading(py, q0)?);
        match (lines.as_slice(), rims.as_slice()) {
            ([], []) => {
                let v = *near.vertices.first()?;
                let on = On {
                    edges: Vec::new(),
                    vertices: vec![v],
                    whole: true,
                };
                let q = *b.vertices.get(&v)?;
                Some((
                    Psi::Linear {
                        n: lift(unit(hx - hy)?),
                        q,
                    },
                    on,
                ))
            }
            ([(q, d)], [r]) => {
                let m = lift(bisector(hx, hy, unit(vmid(*d))?)?);
                let n = m - *d * (m.dot(*d) / d.dot(*d));
                Some((Psi::Linear { n, q: *q }, through(&[*r])))
            }
            ([(q, d), (_, e)], [r, s])
                if !near.vertices.is_empty() && r.ends.iter().any(|v| s.ends.contains(v)) =>
            {
                Some((
                    Psi::Linear {
                        n: d.cross(*e),
                        q: *q,
                    },
                    through(&[*r, *s]),
                ))
            }
            _ => None,
        }
    };
    plane().into_iter().collect()
}

/// **Whether two planar conics' stored frames hold one plane, as one
/// value**: every component of the first's centre and axis is a point,
/// and the second's are the same points, its axis the first's or its
/// negation. A point is the one value every member has, so the plane
/// holds both conics for every member. Two brackets alike are not one
/// value: two members may take different values in them.
fn one_plane(
    (q, n): (Point3<Interval>, Vec3<Interval>),
    (p, m): (Point3<Interval>, Vec3<Interval>),
) -> bool {
    one_vector(q - Point3::origin(), p - Point3::origin()) && one_line(n, m)
}

/// **`y` is `x`'s one value**: `x` is a point and `y` the same point, or
/// with `negated` its negation. A point is the one value every member
/// has; two brackets alike are not one value.
fn one_value(x: Interval, y: Interval, negated: bool) -> bool {
    let k = if negated { -x.lo() } else { x.lo() };
    x.lo() == x.hi() && y.lo() == k && y.hi() == k
}

/// [`one_value`] on every component.
fn one_vector(a: Vec3<Interval>, b: Vec3<Interval>) -> bool {
    [(a.x, b.x), (a.y, b.y), (a.z, b.z)]
        .iter()
        .all(|&(x, y)| one_value(x, y, false))
}

/// One direction or its negation, as one value.
fn one_line(a: Vec3<Interval>, b: Vec3<Interval>) -> bool {
    one_vector(a, b)
        || [(a.x, b.x), (a.y, b.y), (a.z, b.z)]
            .iter()
            .all(|&(x, y)| one_value(x, y, true))
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
    let conic = matches!(carrier, Curve3::Circle(..) | Curve3::Ellipse(..));
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
/// and `m`, bisecting the faces' headings about `t` ([`bisector`]).
/// Where `t·c′` is decided positive over the piece, the projected piece
/// is the graph of `γ` over `s`. It lists the edge: `ψ` vanishes on the
/// carrier over the piece. A curved side on a conic edge, and a side
/// whose chart line is its edge's own line or circle, is the carrier
/// past the edge's span too (through its join, a corner, or the
/// rounding of a chart line against the stored carrier), so the piece
/// also holds that side's parameters, a circle's or conic's moved by
/// the whole turns that bring them nearest it.
///
/// **At a vertex `V` the pair shares alone**, the edge is the one a
/// curve presented past `V` lies on ([`Reading::Conic`] at a corner
/// whose two arcs are off the shared cell): it lists that edge and `V`.
/// The curve's parameters hold `V`'s end parameter `t_V`, since its
/// columns were widened to hold `V`'s chart point ([`graph_read`]) and
/// every parameter whose point is in them is among its own
/// ([`narrowed`]); the piece holds them, so `ψ` vanishes at the
/// carrier's point at `t_V`. `V`'s stored point lies off that by up to
/// the edge's certified tolerance: the separator's ε concession (module
/// docs), as a conic's own plane lists its ends.
fn graph_of<'a>(
    x: &Piece<'a>,
    y: &Piece<'a>,
    near: &Near,
    px: &Presented<'_>,
    py: &Presented<'_>,
    band: Band,
) -> Option<(Psi, On)> {
    let alone = vertex_alone(near.edges.is_empty(), &near.vertices);
    let (e, vertices) = match (near.edges.as_slice(), alone) {
        ([(i, _)], _) => (x.boundary.edges.get(*i)?, Vec::new()),
        (_, Some(v)) => {
            let past = |p: &Presented<'_>| match &p.zero {
                Zero::Curve(_, c) if c.past == Some(v) => match c.along.edges.as_slice() {
                    &[key] => Some(key),
                    _ => None,
                },
                _ => None,
            };
            let find = |b: &'a Boundary, key| b.edges.iter().find(|r: &&Rim| r.key == key);
            let e = match (past(px), past(py)) {
                (Some(k), _) => find(x.boundary, k),
                (None, Some(k)) => find(y.boundary, k),
                (None, None) => None,
            };
            (e?, vec![v])
        }
        _ => return None,
    };
    let (carrier, span) = e.arc.as_ref()?;
    let reached = edge_piece(
        carrier,
        *span,
        e.ends[0] == e.ends[1],
        &[px.whole(), py.whole()],
    )?;
    let piece = [px, py]
        .into_iter()
        .filter_map(|p| {
            let along = match &p.zero {
                Zero::Curve(_, c) => &c.along,
                Zero::Side(_, a) => a,
                _ => return None,
            };
            (along.whole && along.edges == [e.key])
                .then_some(along.params?)?
                .nearest(reached)
        })
        .fold(reached, |(a, b), (c, d)| (a.min(c), b.max(d)));
    let middle = at(geom::mid_param(piece.0, piece.1));
    let t = unit(vmid(carrier.deriv(middle)))?;
    let q0 = pmid(carrier.eval(middle));
    let (hx, hy) = (heading(px, q0)?, heading(py, q0)?);
    let m = bisector(hx, hy, t)?;
    let (t, m) = (lift(t), lift(m));
    let d = arc_deriv(carrier, piece);
    let advance = t.dot(d);
    if interval_sign_as(
        advance * at(piece.1 - piece.0),
        SELF_INTERSECTION_GRAPH,
        band,
    ) != Some(Sign::Positive)
    {
        return None;
    }
    Some((
        Psi::Graph {
            m,
            t,
            slope: m.dot(d) / advance,
            piece,
        },
        On {
            edges: vec![e.key],
            vertices,
            whole: false,
        },
    ))
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
    if !matches!(
        decide(SELF_INTERSECTION_CUSP, Margin::of(gap), band),
        Ok(Sign::Zero)
    ) {
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
    let centre = surface.point_jet(geom::mid_param(u.0, u.1), geom::mid_param(v.0, v.1));
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
        decide(SELF_INTERSECTION_MONOTONE, Margin::of(least), band),
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

/// **`by_kind` keeps the torus convention's verdict** (D3,
/// escalate-never-guess): a torus whose ring half falls in the band is
/// refused as that half undecided, never as a torus decided not a ring,
/// and one whose tube is wider than its ring as the half decided.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod by_kind_rows {
    use super::{EngineRefusal, by_kind};
    use geom::Surface;
    use geom_brep::{TorusConvention, TorusStanding};
    use geom_core::interval::Interval;
    use geom_core::{Band, Point3, Tol, Vec3};

    fn torus(major: f64) -> Surface<Interval> {
        let p = |x: f64| Interval::from_bounds(x, x);
        Surface::Torus {
            center: Point3::new(p(0.0), p(0.0), p(0.0)),
            axis: Vec3::new(p(0.0), p(0.0), p(1.0)),
            major_radius: p(major),
            minor_radius: p(1.0),
            u_ref: Vec3::new(p(1.0), p(0.0), p(0.0)),
        }
    }

    #[test]
    fn a_torus_the_band_cannot_decide_a_ring_escalates_its_half() {
        let band = Band::linear(Tol::witness()).unwrap();
        let face = crate::FaceKey::default();
        let verdict = |major: f64| match by_kind(face, &torus(major), band) {
            Err(EngineRefusal::TorusNotRing { refusal, .. }) => {
                assert_eq!(refusal.convention, TorusConvention::Ring, "{major}");
                refusal.standing()
            }
            other => panic!("{major}: a torus not decided a ring: {other:?}"),
        };
        let in_band = verdict(1.0 + 0.5 * (band.zero() + band.escalate()));
        assert!(
            matches!(
                in_band,
                TorusStanding::Ranked(geom_core::Indeterminate { predicate, .. })
                    if predicate == Some(geom::RingReader::SelfIntersection.sites().ring())
            ),
            "a ring half in band escalates as its own margin, under the engine's own row: \
             {in_band:?}"
        );
        assert!(
            matches!(verdict(0.5), TorusStanding::InvalidInput(_)),
            "a tube wider than its ring is decided outside the convention"
        );
        assert_eq!(
            by_kind(face, &torus(2.0), band),
            Ok(()),
            "a ring is injective"
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

    /// **`unit` reads a heading as every refusing door does**: an
    /// overflowed, underflowed or zero vector names no heading, and one
    /// past the squares' range but in the format names its own.
    #[test]
    fn unit_refuses_a_vector_out_of_the_format_or_zero() {
        for v in [
            Vec3::new(1e200, 0.0, 0.0),
            Vec3::new(1e-170, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        ] {
            assert!(unit(v).is_none(), "{v:?} names no heading");
        }
        let d = unit(Vec3::new(0.0, 3e-160, -4e-160)).expect("in the format");
        assert_eq!(d.to_array(), [0.0, 0.6, -0.8], "a small heading");
    }
    use crate::test_support_fixtures::trv;
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
                Curve3::line(origin(), Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0))),
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
                wall: None,
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

    /// **A half-band stored as a revolved ball's half-band is**: a
    /// cylinder face over `u ∈ [π, 2π]`, `v ∈ [0, 1]`, its loop
    /// counter-clockwise about the outward normal, its high meridian's
    /// image stored at `u = 0`, the seam's stored value. `extent` is its
    /// face's own (`[π, 2π]`) or the complement's (`[0, π]`), an extent
    /// off the face's sheet. What ties it to what the revolve door stores
    /// is `a_revolved_ball_holds_only_after_it_splits`, on the ball
    /// itself.
    /// `edges[2]` is the `u = π` meridian.
    fn half_band(extent: (f64, f64)) -> Boundary {
        let half = core::f64::consts::PI;
        Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 2], Axis::U, 0.0, (0.0, 0.0), (0.0, 1.0)),
                rim(2, [2, 3], Axis::V, 1.0, (TURN, 1.0), (half, 1.0)),
                rim(3, [3, 4], Axis::U, half, (half, 1.0), (half, 0.0)),
                rim(4, [4, 1], Axis::V, 0.0, (half, 0.0), (TURN, 0.0)),
            ],
            vertices: BTreeMap::new(),
            extent: Some([extent, (0.0, 1.0)]),
        }
    }

    /// **A face is never read as holding nothing in a cell it fills,
    /// whichever sheet its extent is on.** On its own sheet the
    /// half-band is presented in its whole window and in its own half
    /// turn, and the other half turn holds none of it. On the
    /// complement's sheet its own lines put it outside its extent
    /// ([`face_extent`]), so it is not presented at any depth: neither
    /// the whole window's zero-width rectangle `u = π`, read as a sliver
    /// beside the shared meridian, nor its own half turn, which meets no
    /// copy of that extent, is read vacuous. Red if either reading is
    /// taken from an extent its lines contradict.
    #[test]
    fn a_half_band_is_not_read_vacuous_where_it_lies() {
        let s = cylinder();
        let half = core::f64::consts::PI;
        let whole = ((0.0, TURN), (0.0, 1.0));
        let own = ((half, TURN), (0.0, 1.0));
        let other = ((0.0, half), (0.0, 1.0));
        let right = half_band((half, TURN));
        let at = |b: &Boundary, cell| chart_near(&s, b, &all(4), &[2], cell);
        for (cell, what) in [(whole, "whole window"), (own, "own half")] {
            assert!(
                matches!(at(&right, cell), Read::Of((lo, hi), _) if lo <= half + 1e-9 && hi >= TURN - 1e-9),
                "own sheet, {what}: {:?}",
                at(&right, cell)
            );
        }
        assert_eq!(
            at(&right, other),
            Read::Vacuous,
            "own sheet, the other half"
        );
        let wrong = half_band((0.0, half));
        for (cell, what) in [
            (own, "own half"),
            (whole, "whole window"),
            (other, "other half"),
        ] {
            assert_eq!(
                at(&wrong, cell),
                Read::Structural,
                "complement's sheet, {what}"
            );
        }
    }

    /// **The face's own clips leaving an axis no width refuse**: a
    /// cylinder face whose two lines `u = 3/2` have it above one and below
    /// the other, each with room in its extent `u ∈ [1, 2]`, so the
    /// extent agrees with both ([`face_extent`]). In a cell across them
    /// they clip `u` to `3/2` from both sides. They contradict each other,
    /// and the rectangle they leave is no sliver beside the shared line.
    /// Red if the zero width is read vacuous.
    #[test]
    fn a_rectangle_its_own_clips_leave_no_width_refuses() {
        let s = cylinder();
        let b = Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 2], Axis::U, 1.5, (1.5, 1.0), (1.5, 0.0)),
                rim(2, [2, 3], Axis::V, 0.0, (1.0, 0.0), (2.0, 0.0)),
                rim(3, [3, 4], Axis::U, 1.5, (1.5, 0.0), (1.5, 1.0)),
                rim(4, [4, 1], Axis::V, 1.0, (2.0, 1.0), (1.0, 1.0)),
            ],
            vertices: BTreeMap::new(),
            extent: Some([(1.0, 2.0), (0.0, 1.0)]),
        };
        assert_eq!(
            chart_near(&s, &b, &all(4), &[0], ((1.2, 1.8), (0.0, 1.0))),
            Read::Local
        );
    }

    /// **A face whose line contradicts its extent is not presented in
    /// any cell**: a cylinder face over `v ∈ [1/2, 3]` whose extent claims
    /// `v ∈ [0, 1/2]`, on an axis no sheet is chosen on. The contradiction
    /// is the face's ([`face_extent`]), not the cell's, so it refuses
    /// `Structural` in the cell across the extent's end and in the cells
    /// one split below it, wholly inside the face, where the extent alone
    /// would read the face missed. Red while any of them reads the face as
    /// holding none of it, or refuses as though a smaller cell could read
    /// it.
    #[test]
    fn a_face_whose_extent_its_line_contradicts_is_not_vacuous_below_the_guard() {
        let s = cylinder();
        let (q, h) = (core::f64::consts::FRAC_PI_2, core::f64::consts::PI);
        let b = Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 2], Axis::U, h, (h, 0.5), (h, 3.0)),
                rim(2, [2, 3], Axis::V, 3.0, (h, 3.0), (q, 3.0)),
                rim(3, [3, 4], Axis::U, q, (q, 3.0), (q, 0.5)),
                rim(4, [4, 1], Axis::V, 0.5, (q, 0.5), (h, 0.5)),
            ],
            vertices: BTreeMap::new(),
            extent: Some([(q, h), (0.0, 0.5)]),
        };
        for v in [(0.2, 0.8), (0.5, 0.8), (1.0, 2.0)] {
            let read = chart_near(&s, &b, &all(4), &[3], ((q + 0.2, h - 0.2), v));
            assert_eq!(read, Read::Structural, "v ∈ {v:?}");
        }
    }

    /// **A cone face whose lines stand on the other nappe from its
    /// vertices**: its vertices off the apex stand on the positive nappe
    /// while its lines and extent put it at `v ∈ [−1, −1/2]`. Read in the
    /// cell `u ∈ [1.2, 1.8]`, `v`, at its apex vertex.
    fn off_its_nappe(v: (f64, f64)) -> Read {
        let s = Surface::Cone {
            apex: origin(),
            axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
            u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        };
        let up = |x: f64| Point3::new(iv0(x), iv0(0.0), iv0(1.0));
        let b = Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 2], Axis::V, -1.0, (1.0, -1.0), (2.0, -1.0)),
                rim(2, [2, 3], Axis::U, 2.0, (2.0, -1.0), (2.0, -0.5)),
                rim(3, [3, 4], Axis::V, -0.5, (2.0, -0.5), (1.0, -0.5)),
                rim(4, [4, 1], Axis::U, 1.0, (1.0, -0.5), (1.0, -1.0)),
            ],
            vertices: BTreeMap::from([
                (vertex(1), up(1.0)),
                (vertex(2), up(2.0)),
                (vertex(9), origin()),
            ]),
            extent: Some([(1.0, 2.0), (-1.0, -0.5)]),
        };
        let local = Local {
            edges: all(4),
            vertices: vec![vertex(9)],
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &b,
            u: (1.2, 1.8),
            v,
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        read(present_chart(&piece, &[], &[], band()))
    }

    /// **A cone face whose lines contradict its nappe is not presented in
    /// any cell** ([`off_its_nappe`]): the contradiction is the face's
    /// ([`face_extent`]), so the cell at its apex vertex reaching both
    /// nappes, the cells on either nappe below it, and the cell on its
    /// lines' nappe each refuse `Structural`. Red if any reads the face
    /// as holding none of it, or refuses as though a smaller cell could
    /// read it.
    #[test]
    fn a_face_whose_lines_contradict_its_nappe_refuses_in_every_cell() {
        for v in [
            (-0.75, 0.5),
            (-0.75, 0.0),
            (-0.75, -0.125),
            (-0.125, 0.5),
            (0.0, 0.5),
        ] {
            assert_eq!(off_its_nappe(v), Read::Structural, "v ∈ {v:?}");
        }
    }

    /// **A face is not read as holding nothing on both nappes**
    /// ([`off_its_nappe`]): the two cells one split below the cell
    /// `a_cell_reaching_the_faces_nappe_is_not_vacuous_off_its_clips`
    /// refuses. The face has points on the nappe its vertices stand on or
    /// on the one its lines stand on. Red while the cell on its vertices'
    /// nappe misses its extent and the cell on its lines' nappe is read as
    /// the other nappe: the refusal above is `Local`, so the pair splits
    /// into these two and certifies on both.
    #[test]
    fn a_face_whose_lines_contradict_its_nappe_is_not_vacuous_on_both() {
        let (lines, vertices) = (off_its_nappe((-0.75, -0.125)), off_its_nappe((-0.125, 0.5)));
        assert!(
            !(lines == Read::Vacuous && vertices == Read::Vacuous),
            "the lines' nappe reads {lines:?}, the vertices' nappe {vertices:?}"
        );
    }

    /// **A line apart from the face's extent contradicts it**
    /// ([`face_extent`]): a cylinder face over `u ∈ [1, 3]` whose extent
    /// claims `u ∈ [1, 2]`. Each of its lines has room in that extent on
    /// its face side, so only the `u = 3` line's lying apart from the
    /// extent says the extent does not hold the face. Red if a line is
    /// read for that room alone: the cell `u ∈ [2.2, 2.8]`, wholly inside
    /// the face, then misses the extent and reads the face as holding
    /// nothing.
    #[test]
    fn a_line_apart_from_the_faces_extent_contradicts_it() {
        let s = cylinder();
        let b = Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 2], Axis::V, 0.0, (1.0, 0.0), (3.0, 0.0)),
                rim(2, [2, 3], Axis::U, 3.0, (3.0, 0.0), (3.0, 1.0)),
                rim(3, [3, 4], Axis::V, 1.0, (3.0, 1.0), (1.0, 1.0)),
                rim(4, [4, 1], Axis::U, 1.0, (1.0, 1.0), (1.0, 0.0)),
            ],
            vertices: BTreeMap::new(),
            extent: Some([(1.0, 2.0), (0.0, 1.0)]),
        };
        for u in [(2.2, 2.8), (1.2, 1.8)] {
            let read = chart_near(&s, &b, &all(4), &[], (u, (0.2, 0.8)));
            assert_eq!(read, Read::Structural, "u ∈ {u:?}");
        }
    }

    /// **A line a turn's rounding moves past its extent agrees with it**:
    /// the half-band of [`half_band`] with its extent ending two ulps
    /// short of a turn, where its `u = 0` line, placed a turn on, lies.
    /// That is the rounding of a value moved by whole turns, which
    /// [`face_extent`] reads with [`chart_slack`], not a contradiction: the
    /// face's own half is presented. Red if the extent is read exactly.
    #[test]
    fn a_line_a_turns_rounding_moves_past_its_extent_agrees_with_it() {
        let s = cylinder();
        let half = core::f64::consts::PI;
        let short = f64::from_bits(TURN.to_bits() - 2);
        let b = half_band((half, short));
        assert!(matches!(
            chart_near(&s, &b, &all(4), &[2], ((half, TURN), (0.0, 1.0))),
            Read::Of(..)
        ));
    }

    /// **A cell past the face's extent on an axis no sheet is chosen on
    /// holds none of it**: the half-band of [`half_band`], on its own
    /// sheet, read in `v ∈ [1.5, 2.5]`, above its `v ∈ [0, 1]`. Red if a
    /// cell that misses the extent there is refused instead.
    #[test]
    fn a_cell_past_the_extent_on_a_flat_axis_holds_none_of_the_face() {
        let s = cylinder();
        let half = core::f64::consts::PI;
        let b = half_band((half, TURN));
        assert_eq!(
            chart_near(&s, &b, &all(4), &[2], ((half, TURN), (1.5, 2.5))),
            Read::Vacuous
        );
    }

    /// **A cone face at its apex is read on its own nappe alone**: a face
    /// on the positive nappe over `v ∈ [0, 1]`, bounded by the parallel
    /// `v = 1` and, where it does not wrap, by two meridians to the apex.
    ///
    /// Wrapping the cone, its extent says nothing ([`Extent::Wraps`]), so
    /// the cell on the other nappe at its apex vertex holds none of it by
    /// the apex's own reading, and the cell on its nappe presents. Red if
    /// the other nappe is not read as holding nothing there.
    ///
    /// Not wrapping, with its extent's low end an ulp below the apex (a
    /// row's end rounded), its extent is on its nappe within the slack
    /// ([`face_extent`]) and the cell on its nappe presents. Red if the
    /// nappe is read against the extent with no slack.
    #[test]
    fn a_cone_face_at_its_apex_is_read_on_its_own_nappe() {
        let s = Surface::Cone {
            apex: origin(),
            axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
            u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        };
        let vertices = BTreeMap::from([
            (vertex(3), Point3::new(iv0(1.0), iv0(0.0), iv0(1.0))),
            (vertex(9), origin()),
        ]);
        let at = |b: &Boundary, v: (f64, f64)| {
            let local = Local {
                edges: all(b.edges.len()),
                vertices: vec![vertex(9)],
            };
            let piece = Piece {
                face: FaceKey::default(),
                surface: &s,
                whole: false,
                boundary: b,
                u: (1.2, 1.8),
                v,
                local: &local,
                ball: Ball::holding(origin(), origin()),
            };
            read(present_chart(&piece, &[], &[], band()))
        };
        let wrapping = Boundary {
            sense: true,
            edges: vec![rim(1, [3, 3], Axis::V, 1.0, (TURN, 1.0), (0.0, 1.0))],
            vertices: vertices.clone(),
            extent: Some([(0.0, TURN), (0.0, 1.0)]),
        };
        assert_eq!(
            at(&wrapping, (-0.5, -0.1)),
            Read::Vacuous,
            "the other nappe"
        );
        assert!(
            matches!(at(&wrapping, (0.2, 0.8)), Read::Of(..)),
            "its own nappe: {:?}",
            at(&wrapping, (0.2, 0.8))
        );
        let sector = Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 3], Axis::U, 2.0, (2.0, 0.0), (2.0, 1.0)),
                rim(2, [3, 4], Axis::V, 1.0, (2.0, 1.0), (1.0, 1.0)),
                rim(3, [4, 1], Axis::U, 1.0, (1.0, 1.0), (1.0, 0.0)),
            ],
            vertices,
            extent: Some([(1.0, 2.0), (-1e-15, 1.0)]),
        };
        assert!(
            matches!(at(&sector, (0.2, 0.8)), Read::Of(..)),
            "an extent an ulp below the apex: {:?}",
            at(&sector, (0.2, 0.8))
        );
    }

    /// **A cone face across its apex is not presented at its apex
    /// vertex**: a face wrapping the cone (so [`face_extent`] reads no
    /// nappe of it), with vertices on both nappes, so it has no one nappe
    /// ([`nappe`]). That is the face's, so the cell at its apex vertex
    /// refuses `Structural`. Red if it refuses as though a smaller cell
    /// holding the vertex could read it.
    #[test]
    fn a_cone_face_with_no_one_nappe_refuses_at_its_apex() {
        let s = Surface::Cone {
            apex: origin(),
            axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
            u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        };
        let b = Boundary {
            sense: true,
            edges: vec![rim(1, [3, 3], Axis::V, 1.0, (TURN, 1.0), (0.0, 1.0))],
            vertices: BTreeMap::from([
                (vertex(3), Point3::new(iv0(1.0), iv0(0.0), iv0(1.0))),
                (vertex(4), Point3::new(iv0(1.0), iv0(0.0), iv0(-1.0))),
                (vertex(9), origin()),
            ]),
            extent: Some([(0.0, TURN), (-1.0, 1.0)]),
        };
        let local = Local {
            edges: all(1),
            vertices: vec![vertex(9)],
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &b,
            u: (1.2, 1.8),
            v: (-0.5, 0.5),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        assert_eq!(
            read(present_chart(&piece, &[], &[], band())),
            Read::Structural
        );
    }

    /// **A cone face's extent with no height on its nappe contradicts it**:
    /// vertices standing on one nappe while the extent's `v` is the apex
    /// alone. Read against the nappe it holds no point of it past the
    /// apex, so [`face_extent`] refuses it on either nappe. Red if the
    /// extent is only asked to stand on the nappe's side of the apex, not
    /// to reach past it: the cell across the apex then misses the extent
    /// and reads the face as holding nothing.
    #[test]
    fn a_cone_face_whose_extent_has_no_height_on_its_nappe_refuses() {
        let s = Surface::Cone {
            apex: origin(),
            axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
            u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        };
        for z in [1.0, -1.0] {
            let b = Boundary {
                sense: true,
                edges: Vec::new(),
                vertices: BTreeMap::from([(vertex(3), Point3::new(iv0(1.0), iv0(0.0), iv0(z)))]),
                extent: Some([(1.0, 2.0), (0.0, 0.0)]),
            };
            assert_eq!(
                chart(&s, &b, &[], (1.2, 1.8), (-0.5, 0.5)),
                Read::Structural,
                "the nappe at z = {z}"
            );
        }
    }

    /// **A wrapping outer loop leaves every axis free, a periodic one its
    /// lines are on included**: a torus band wrapping `u`, bounded by the
    /// parallels `v = 1/2` and `v = 1` (`v` periodic too). Its lines are
    /// placed toward the cell, and the band presents between them. Red if
    /// a wrapping extent is read as an unknown one: the parallels then
    /// have no placement.
    #[test]
    fn a_wrapping_band_places_its_lines_on_a_periodic_axis() {
        let s = ring(2.0);
        let b = Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 1], Axis::V, 0.5, (0.0, 0.5), (TURN, 0.5)),
                rim(2, [2, 2], Axis::V, 1.0, (TURN, 1.0), (0.0, 1.0)),
            ],
            vertices: BTreeMap::new(),
            extent: Some([(0.0, TURN), (0.5, 1.0)]),
        };
        assert_eq!(
            chart(&s, &b, &all(2), (1.0, 2.0), (0.4, 1.1)),
            Read::Of((1.0, 2.0), (0.5, 1.0))
        );
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

    /// **`face_side`**: the face lies left of its boundary where its loop
    /// winds counter-clockwise, and right where it winds clockwise.
    #[test]
    fn face_side_reads_the_loop_and_its_orientation() {
        let iso = |held, at| Iso {
            held,
            at: iv0(at),
            full: false,
        };
        let up = Use {
            plus: true,
            image: Some([p2((1.0, 0.0)), p2((1.0, 1.0))]),
            iso: None,
            wall: None,
            harmonic: None,
        };
        assert_eq!(
            face_side(&up, iso(Axis::U, 1.0), PolygonOrientation::of_face(true)),
            Some(-1.0),
            "v rising at u = 1: face below u"
        );
        assert_eq!(
            face_side(&up, iso(Axis::U, 1.0), PolygonOrientation::of_face(false)),
            Some(1.0),
            "clockwise"
        );
        let east = Use {
            plus: true,
            image: Some([p2((0.0, 0.0)), p2((1.0, 0.0))]),
            iso: None,
            wall: None,
            harmonic: None,
        };
        assert_eq!(
            face_side(&east, iso(Axis::V, 0.0), PolygonOrientation::of_face(true)),
            Some(1.0),
            "u rising at v = 0: face above v"
        );
        let back = Use {
            plus: false,
            ..east
        };
        assert_eq!(
            face_side(&back, iso(Axis::V, 0.0), PolygonOrientation::of_face(true)),
            Some(-1.0),
            "run backward"
        );
        let flat = Use {
            plus: true,
            image: Some([p2((0.0, 0.0)), p2((0.0, 0.0))]),
            iso: None,
            wall: None,
            harmonic: None,
        };
        assert_eq!(
            face_side(&flat, iso(Axis::V, 0.0), PolygonOrientation::of_face(true)),
            None,
            "no direction, no side"
        );
    }

    /// The lines of `edges` held `w` wide either side of their values.
    fn widened(mut edges: Vec<Rim>, w: f64) -> Vec<Rim> {
        for e in &mut edges {
            for u in &mut e.uses {
                if let Some(iso) = u.iso.as_mut() {
                    let at = iso.at.lo();
                    iso.at = Interval::from_bounds(at - w, at + w);
                }
            }
        }
        edges
    }

    /// A face of `edges` over the chart rectangle `u ∈ [1, 2]`,
    /// `v ∈ [0, 1]`.
    fn unit_face(edges: Vec<Rim>) -> Boundary {
        Boundary {
            sense: true,
            edges,
            vertices: BTreeMap::new(),
            extent: Some([(1.0, 2.0), (0.0, 1.0)]),
        }
    }

    /// **A rectangle on the cylinder's chart**, `u ∈ [1, 2]`,
    /// `v ∈ [0, 1]`, counter-clockwise: its ends' images exact, its
    /// lines' values held `w` wide either side, as an intersection's
    /// pcurve holds its line wider than another edge's end at the corner
    /// they share.
    fn wide_rectangle(w: f64) -> Boundary {
        unit_face(widened(
            vec![
                rim(1, [1, 2], Axis::V, 0.0, (1.0, 0.0), (2.0, 0.0)),
                rim(2, [2, 3], Axis::U, 2.0, (2.0, 0.0), (2.0, 1.0)),
                rim(3, [3, 4], Axis::V, 1.0, (2.0, 1.0), (1.0, 1.0)),
                rim(4, [4, 1], Axis::U, 1.0, (1.0, 1.0), (1.0, 0.0)),
            ],
            w,
        ))
    }

    /// **The rectangle's corners, each of its two edges cut short**: the
    /// eight ways one edge of a corner is `1e-7` long, the other whole,
    /// both lines `1e-6` wide. Each is `(what, boundary, past, within)`:
    /// the two edges alone, a cell along the short edge's moving axis
    /// reaching `5e-7` from the corner (past the short edge's far end),
    /// and one reaching `5e-8` (within it), each spanning the corner by
    /// `0.5` on the other axis. `gap` instead keeps the edge whole and
    /// moves its corner end `1e-7` off the corner, to a vertex the other
    /// edge does not have.
    #[allow(clippy::type_complexity)]
    fn cut_corners(gap: bool) -> Vec<(String, Boundary, [((f64, f64), (f64, f64)); 2])> {
        let (short, w) = (1e-7, 1e-6);
        // The loop's edges: held axis, value, start, end, end vertices.
        let sides = [
            (Axis::V, 0.0, (1.0, 0.0), (2.0, 0.0), [1, 2]),
            (Axis::U, 2.0, (2.0, 0.0), (2.0, 1.0), [2, 3]),
            (Axis::V, 1.0, (2.0, 1.0), (1.0, 1.0), [3, 4]),
            (Axis::U, 1.0, (1.0, 1.0), (1.0, 0.0), [4, 1]),
        ];
        let toward = |from: (f64, f64), to: (f64, f64), by: f64| {
            let n = ((to.0 - from.0).powi(2) + (to.1 - from.1).powi(2)).sqrt();
            (
                from.0 + (to.0 - from.0) / n * by,
                from.1 + (to.1 - from.1) / n * by,
            )
        };
        let mut out = Vec::new();
        for i in 0..4 {
            let (arrive, leave) = (sides[i], sides[(i + 1) % 4]);
            let corner = arrive.3;
            for cut_arriving in [true, false] {
                let (mut a, mut l) = (arrive, leave);
                let far;
                if cut_arriving {
                    far = toward(corner, a.2, short);
                    if gap {
                        a.3 = far;
                        a.4[1] = 9;
                    } else {
                        a.2 = far;
                    }
                } else {
                    far = toward(corner, l.3, short);
                    if gap {
                        l.2 = far;
                        l.4[0] = 9;
                    } else {
                        l.3 = far;
                    }
                }
                let edges = widened(
                    vec![
                        rim(1, a.4, a.0, a.1, a.2, a.3),
                        rim(2, l.4, l.0, l.1, l.2, l.3),
                    ],
                    w,
                );
                // The short edge's moving axis is the one its far end
                // leaves the corner along.
                let along_u = (far.0 - corner.0).abs() > 0.0;
                let dir = if along_u {
                    (far.0 - corner.0).signum()
                } else {
                    (far.1 - corner.1).signum()
                };
                let cell = |reach: f64| {
                    let (m, o) = if along_u {
                        (corner.0, corner.1)
                    } else {
                        (corner.1, corner.0)
                    };
                    let (p, q) = (m + reach * dir, m - 0.5 * dir);
                    let moving = (p.min(q), p.max(q));
                    let other = (o - 0.5, o + 0.5);
                    if along_u {
                        (moving, other)
                    } else {
                        (other, moving)
                    }
                };
                out.push((
                    format!(
                        "corner {corner:?}, the {} edge cut",
                        if cut_arriving { "arriving" } else { "leaving" }
                    ),
                    unit_face(edges),
                    [cell(5e-7), cell(5e-8)],
                ));
            }
        }
        out
    }

    /// The cell `(u, v)` moved `turn` round on the periodic `u` axis.
    fn turned(((a, b), v): ((f64, f64), (f64, f64)), turn: f64) -> ((f64, f64), (f64, f64)) {
        ((a + turn, b + turn), v)
    }

    /// **Each corner of a rectangle presents, its lines held far wider
    /// than its edges' ends** ([`run_on`]): at every corner the
    /// rectangle's side runs to the other line's outer bound, past the
    /// edge's exact end, and the side is still the edge's own because
    /// the edge ends on that line, on the cell's own sheet a turn round
    /// too. Red if a side must be covered by its edges alone: the corner
    /// then reads `Local` at every depth, the lines' widths being no
    /// cell's; or if a low side is run on the wrong way.
    #[test]
    fn a_corner_presents_though_its_lines_are_wider_than_its_edges_ends() {
        let s = cylinder();
        let b = wide_rectangle(1e-9);
        let corners = [
            ((1.5, 2.5), (0.5, 1.5)),
            ((1.5, 2.5), (-0.5, 0.5)),
            ((0.5, 1.5), (0.5, 1.5)),
            ((0.5, 1.5), (-0.5, 0.5)),
        ];
        for turn in [0.0, TURN] {
            for cell in corners.map(|c| turned(c, turn)) {
                let (u, v) = cell;
                assert!(
                    matches!(chart(&s, &b, &all(4), u, v), Read::Of(..)),
                    "the corner in {u:?} × {v:?} presents: {:?}",
                    chart(&s, &b, &all(4), u, v)
                );
            }
        }
    }

    /// **An edge is run on from its own end at the corner, outward, on
    /// the line's sheet, and only against a line on the other axis**:
    /// at each of the rectangle's corners, with either of its edges cut
    /// `1e-7` short against the other's `1e-6`-wide line, a cell reaching
    /// past the short edge's far end is not presented and one within it
    /// is, on the stored sheet and a turn round. Both arms of the
    /// run-on, high and low, are read on both axes. Red if the line's
    /// whole enclosure is taken as the edge's (at either arm), if a low
    /// side is run on the wrong way, if the edge's end is read on its
    /// stored sheet against a line placed on the cell's (the run-on then
    /// spans the turn between them), or if a clip is run on against
    /// itself: each such cell then presents with its side running past
    /// the edge.
    #[test]
    fn a_corner_runs_its_edge_on_from_its_own_end_only() {
        let s = cylinder();
        for (what, b, [past, within]) in cut_corners(false) {
            for turn in [0.0, TURN] {
                let ((u, v), (wu, wv)) = (turned(past, turn), turned(within, turn));
                assert_eq!(
                    chart(&s, &b, &[0, 1], u, v),
                    Read::Local,
                    "{what}, {turn} round: a cell past the short edge is not presented"
                );
                assert!(
                    matches!(chart(&s, &b, &[0, 1], wu, wv), Read::Of(..)),
                    "{what}, {turn} round: a cell within the short edge presents: {:?}",
                    chart(&s, &b, &[0, 1], wu, wv)
                );
            }
        }
    }

    /// **An edge that stops short of a line it does not end on runs on
    /// past nothing**: at each corner, either edge ends `1e-7` short of
    /// the other's line, inside that line's `1e-6` enclosure, at a
    /// vertex the line's edge does not have. The face may turn round
    /// that end, so the corner cell is not presented. Red if an edge is
    /// run on to a line it shares no vertex with.
    #[test]
    fn an_edge_ending_short_of_a_line_it_shares_no_vertex_with_is_not_run_on() {
        let s = cylinder();
        for (what, b, [past, _]) in cut_corners(true) {
            // The cell holds the gap and the edge's end past it.
            let (u, v) = past;
            assert_eq!(
                chart(&s, &b, &[0, 1], u, v),
                Read::Local,
                "{what}: the cell beside the gap is not presented"
            );
        }
    }

    /// **`convex` decides a corner `present_chart` reads**: two `T`
    /// junctions on the rectangle's chart, at the corner `(2, 1)` where
    /// the `u = 2` line (face on its low side) meets the `v = 1` line
    /// (face on its high side). In the first the `u = 2` line runs on
    /// through the corner, so one of its edges leaves the corner away
    /// from the `v = 1` line's face side; in the second the `v = 1` line
    /// runs on through it. Every side is covered, and the face is not
    /// the clipped quadrant. Red if `convex` reads only one of the two
    /// clips' edges, or is not read.
    #[test]
    fn a_corner_one_clips_edges_run_through_is_not_convex() {
        let s = cylinder();
        let through_u = unit_face(vec![
            rim(1, [5, 3], Axis::U, 2.0, (2.0, 0.0), (2.0, 1.0)),
            rim(2, [3, 6], Axis::U, 2.0, (2.0, 1.0), (2.0, 2.0)),
            rim(3, [7, 3], Axis::V, 1.0, (1.0, 1.0), (2.0, 1.0)),
        ]);
        let through_v = unit_face(vec![
            rim(1, [5, 3], Axis::U, 2.0, (2.0, 0.0), (2.0, 1.0)),
            rim(2, [7, 3], Axis::V, 1.0, (1.0, 1.0), (2.0, 1.0)),
            rim(3, [3, 6], Axis::V, 1.0, (2.0, 1.0), (3.0, 1.0)),
        ]);
        // Each junction's extent is the one its lines agree with: the face
        // lies above `v = 1`.
        let extent = |mut b: Boundary, u: (f64, f64)| {
            b.extent = Some([u, (0.0, 2.0)]);
            b
        };
        let (through_u, through_v) = (extent(through_u, (1.0, 2.0)), extent(through_v, (1.0, 3.0)));
        for (what, b) in [("through u", through_u), ("through v", through_v)] {
            assert_eq!(
                chart(&s, &b, &all(3), (1.5, 2.5), (0.5, 1.5)),
                Read::Local,
                "{what}"
            );
        }
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
        let s = cylinder();
        let (k0, k3) = (b.edges[0].key, b.edges[3].key);
        assert!(matches!(
            zero_of(&s, &b, &clips, &[0], &[]),
            Ok(Zero::Side(Side::XHi, a)) if a.edges == [k0]
        ));
        assert!(matches!(
            zero_of(&s, &b, &clips, &[0, 3], &[]),
            Ok(Zero::Corner((Side::XHi, a), (Side::YLo, c))) if a.edges == [k0] && c.edges == [k3]
        ));
        assert!(matches!(
            zero_of(&s, &b, &clips, &[], &[vertex(1)]),
            Ok(Zero::Point(Side::XHi, Side::YLo, v)) if v == vertex(1)
        ));
        assert!(matches!(zero_of(&s, &b, &clips, &[], &[]), Ok(Zero::None)));
        let left = clip(Side::XLo, 2);
        assert!(zero_of(&s, &b, &[clip(Side::XHi, 0), left], &[0, 2], &[]).is_err());
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
                    Curve3::line(origin(), Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0))),
                    (iv0(0.0), iv0(4.0)),
                )),
                ends: [vertex(1), vertex(2)],
                uses: vec![Use {
                    plus: true,
                    image: None,
                    iso: None,
                    wall: None,
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
                    matches!(&q.zero, Zero::Side(Side::YLo, a) if a.edges == [edge(1)] && a.whole),
                    "the shared edge is the zero side, its own line"
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

    /// **`present_plane`, two line edges**: a convex corner presents its
    /// quadrant; the same corner run backward, or with the sense flipped,
    /// is reflex (the face is the three other quadrants) and is not
    /// presented as the wedge between its edges. Red if the lean gate
    /// reads the face's side the wrong way, or not at all.
    #[test]
    fn a_planes_reflex_corner_of_two_lines_is_not_its_wedge() {
        let plane = Surface::Plane {
            origin: origin(),
            normal: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        };
        let line = |d: Vec3<Interval>, far: u64, plus: bool| Rim {
            key: edge(far),
            arc: Some((Curve3::line(origin(), d), (iv0(0.0), iv0(4.0)))),
            ends: [vertex(1), vertex(far)],
            uses: vec![Use {
                plus,
                image: None,
                iso: None,
                wall: None,
                harmonic: None,
            }],
        };
        let y = Vec3::new(iv0(0.0), iv0(1.0), iv0(0.0));
        let x = Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0));
        // Forward: (0, 4) to the vertex at the origin to (4, 0), the
        // first quadrant on its left.
        let corner = |forward: bool, sense: bool| Boundary {
            sense,
            edges: vec![line(y, 2, !forward), line(x, 3, forward)],
            vertices: BTreeMap::new(),
            extent: None,
        };
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let read = |b: &Boundary| {
            let piece = Piece {
                face: FaceKey::default(),
                surface: &plane,
                whole: false,
                boundary: b,
                u: (0.5, 1.0),
                v: (0.5, 1.0),
                local: &local,
                ball: Ball::holding(origin(), origin()),
            };
            match present_plane(&piece, &[], &[], band()) {
                Ok(Presentation::Of(_)) => "presented",
                Err(Unpresented::Local) => "refused locally",
                _ => "something else",
            }
        };
        assert!(
            read(&corner(true, true)) == "presented",
            "the convex corner presents its quadrant"
        );
        for (forward, sense, what) in [
            (false, true, "run backward"),
            (true, false, "sense flipped"),
        ] {
            assert!(
                read(&corner(forward, sense)) == "refused locally",
                "{what}: the reflex corner is not presented as its wedge"
            );
        }
    }

    /// **`edge_piece`**: the hull of an edge's parameters whose points the
    /// boxes may hold, narrowed to them from both ends.
    #[test]
    fn edge_piece_narrows_to_the_parameters_the_boxes_hold() {
        let line = Curve3::line(origin(), Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)));
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
                    Curve3::circle(
                        origin(),
                        Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                        iv0(r),
                        start,
                    ),
                    (iv0(0.0), iv0(TURN)),
                )),
                ends: [vertex(1), vertex(1)],
                uses: vec![Use {
                    plus,
                    image: None,
                    iso: None,
                    wall: None,
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
                Curve3::line(
                    p(from),
                    Vec3::new(iv0(to.0 - from.0), iv0(to.1 - from.1), iv0(0.0)),
                ),
                (iv0(0.0), iv0(1.0)),
            )),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                wall: None,
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
                Curve3::circle(
                    origin(),
                    z,
                    iv0(1.0),
                    Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                ),
                (iv0(0.0), iv0(core::f64::consts::FRAC_PI_2)),
            )),
            ends: [vertex(1), vertex(2)],
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                wall: None,
                harmonic: None,
            }],
        };
        let p = |x: f64, y: f64| Point3::new(iv0(x), iv0(y), iv0(0.0));
        let mut vertices = BTreeMap::from([(vertex(1), p(1.0, 0.0)), (vertex(2), p(0.0, 1.0))]);
        for e in core::iter::once(&other).chain(&extra) {
            if let Some((Curve3::Line(geom::Line { origin, dir }), _)) = &e.arc {
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

    /// [`present_plane`] of a [`quarter`] in the cell `u × v`, its
    /// local edges the arc and the line (edges 0 and 1) and its local
    /// vertex `(1, 0)`, the line shared.
    fn on_floor_by_line(b: &Boundary, u: (f64, f64), v: (f64, f64)) -> Read {
        let s = floor();
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
        read(present_plane(&piece, &[1], &[vertex(1)], band()))
    }

    /// **A plane's line edge is read through its corner with an arc**
    /// ([`present_plane_corner`]): the line from the origin reaches
    /// `(1, 0)`, where the arc leaves into its face side, so a cell
    /// holding the vertex is presented by the line's face side, whether
    /// it reaches past the vertex or stops short of it. Where the line
    /// leaves the vertex the other way the corner is reflex, which no
    /// presentation reads; where it runs along the arc's tangent, the
    /// arc's departure is second order, which is not read.
    #[test]
    fn a_planes_line_edge_is_read_through_its_corner_with_an_arc() {
        let b = quarter(segment(20, [3, 1], (0.0, 0.0), (1.0, 0.0)), Vec::new());
        for (u, v, what) in [
            ((0.98, 1.02), (-0.02, 0.02), "about the vertex"),
            (
                (0.97, 0.995),
                (0.0, 0.02),
                "short of it, on the face's side",
            ),
        ] {
            assert!(
                matches!(on_floor_by_line(&b, u, v), Read::Of(_, (0.0, _))),
                "{what}: the line's face side is presented: {:?}",
                on_floor_by_line(&b, u, v)
            );
        }
        let reflex = quarter(segment(20, [3, 1], (2.0, 0.0), (1.0, 0.0)), Vec::new());
        assert_eq!(
            on_floor_by_line(&reflex, (0.98, 1.02), (-0.02, 0.02)),
            Read::Structural,
            "the reflex corner at (1, 0)"
        );
        let tangent = quarter(segment(20, [3, 1], (1.0, -1.0), (1.0, 0.0)), Vec::new());
        assert_eq!(
            on_floor_by_line(&tangent, (0.98, 1.02), (-0.02, 0.02)),
            Read::Local,
            "the tangent join at (1, 0)"
        );
    }

    /// **A lens is read at its one end in the cell** ([`corner_of`]): the
    /// upper half of the unit disc, its arc (edge 0, shared) from `(1, 0)`
    /// round to `(−1, 0)` and the diameter back, meets the diameter at
    /// both ends. A cell holding one end is presented through that
    /// corner; a cell holding both is not, but a smaller one may be, so
    /// it is `Local`. Red, `Structural` at both, if a lens has no corner.
    #[test]
    fn a_lens_is_read_through_its_one_end_in_the_cell() {
        let mut b = quarter(segment(20, [2, 1], (-1.0, 0.0), (1.0, 0.0)), Vec::new());
        if let Some(e) = b.edges.first_mut() {
            e.arc = e
                .arc
                .take()
                .map(|(c, (t0, _))| (c, (t0, iv0(core::f64::consts::PI))));
        }
        b.vertices
            .insert(vertex(2), Point3::new(iv0(-1.0), iv0(0.0), iv0(0.0)));
        assert!(
            matches!(
                on_floor(&b, vec![0, 1], vec![vertex(1)], (0.98, 1.02), (-0.02, 0.02)),
                Read::Of(..)
            ),
            "the lens's end at (1, 0) is presented"
        );
        assert_eq!(
            on_floor(
                &b,
                vec![0, 1],
                vec![vertex(1), vertex(2)],
                (-1.1, 1.1),
                (-0.1, 1.1)
            ),
            Read::Local,
            "a cell holding both ends"
        );
    }

    /// **The unit circle on [`floor`] as two arcs of one conic**, meeting
    /// at `(1, 0)` (vertex 1) and `(-1, 0)` (vertex 2), as a split rim's
    /// halves are stored: edge 0 (key 10) the upper half counter-clockwise
    /// on the axis `+z`, edge 1 (key `lower`) the lower half on the axis
    /// `-z` from `(1, 0)`, the same points on the other parametrization.
    /// `inside`: the disc, its loop counter-clockwise; else the face
    /// outside the circle, its loop clockwise. `lower`, when given, is the
    /// lower arc's own carrier in place of the circle's, over `(0, end)`.
    fn halves(inside: bool, key: u64, lower: Option<(Curve3<Interval>, f64)>) -> Boundary {
        let z = Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0));
        let x = Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0));
        let half = (iv0(0.0), iv0(core::f64::consts::PI));
        let rim = |key: u64, carrier: Curve3<Interval>, span, plus: bool| Rim {
            key: edge(key),
            arc: Some((carrier, span)),
            ends: [vertex(1), vertex(2)],
            uses: vec![Use {
                plus,
                image: None,
                iso: None,
                wall: None,
                harmonic: None,
            }],
        };
        let upper = rim(10, Curve3::circle(origin(), z, iv0(1.0), x), half, inside);
        let lower = match lower {
            Some((c, end)) => rim(key, c, (iv0(0.0), iv0(end)), !inside),
            None => rim(
                key,
                Curve3::circle(origin(), -z, iv0(1.0), x),
                half,
                !inside,
            ),
        };
        let p = |x: f64| Point3::new(iv0(x), iv0(0.0), iv0(0.0));
        Boundary {
            sense: true,
            edges: vec![upper, lower],
            vertices: BTreeMap::from([(vertex(1), p(1.0)), (vertex(2), p(-1.0))]),
            extent: None,
        }
    }

    /// [`present_plane`] of `b` in the cell `u × v` about `(1, 0)`, both
    /// arcs and the vertex in reach, the upper arc shared.
    fn on_halves<'a>(
        s: &'a Surface<Interval>,
        b: &'a Boundary,
        local: &'a Local,
    ) -> Result<Presentation<'a>, Unpresented> {
        present_plane(&piece_on(s, b, local, 1), &[0], &[], band())
    }

    fn piece_on<'a>(
        s: &'a Surface<Interval>,
        b: &'a Boundary,
        local: &'a Local,
        n: u64,
    ) -> Piece<'a> {
        Piece {
            face: face(n),
            surface: s,
            whole: false,
            boundary: b,
            u: (0.98, 1.02),
            v: (-0.02, 0.02),
            local,
            ball: Ball::holding(origin(), origin()),
        }
    }

    fn both_arcs() -> Local {
        Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        }
    }

    /// **Two arcs of one circle are read on through their vertex**
    /// ([`Reading::Conic`]): about `(1, 0)` the disc on the circle's two
    /// halves, stored on opposite axes, is presented inside the circle
    /// (at lower `u`), its zero the circle run on past the shared upper
    /// arc through the vertex. A cell holding both vertices is `Local`.
    /// A lower arc of another circle through both vertices, centred at
    /// `(0, 1)`, makes a corner of 135° there, which no reading reads:
    /// red if [`one_conic`] admits two circles that are not one.
    #[test]
    fn two_arcs_of_one_circle_are_read_on_through_their_vertex() {
        let s = floor();
        let local = both_arcs();
        let disc = halves(true, 11, None);
        match on_halves(&s, &disc, &local) {
            Ok(Presentation::Of(q)) => match &q.zero {
                Zero::Curve(side, c) => {
                    assert_eq!(*side, Side::XHi, "the disc is inside the circle");
                    assert_eq!(c.past, Some(vertex(1)), "run on past the shared arc");
                    assert!(
                        c.t.0 < 0.0 && 0.0 < c.t.1,
                        "the curve's parameters straddle the vertex: {:?}",
                        c.t
                    );
                }
                other => panic!("the disc's zero is the circle: {other:?}"),
            },
            other => panic!("the disc is presented about (1, 0): {:?}", read(other)),
        }
        let everything = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1), vertex(2)],
        };
        let piece = Piece {
            u: (-1.1, 1.1),
            v: (-1.1, 1.1),
            ..piece_on(&s, &disc, &everything, 1)
        };
        assert_eq!(
            read(present_plane(&piece, &[0], &[], band())),
            Read::Local,
            "a cell holding both vertices"
        );
        let r = core::f64::consts::SQRT_2;
        let lens = Curve3::circle(
            Point3::new(iv0(0.0), iv0(1.0), iv0(0.0)),
            Vec3::new(iv0(0.0), iv0(0.0), iv0(-1.0)),
            iv0(r),
            Vec3::new(iv0(1.0 / r), iv0(-1.0 / r), iv0(0.0)),
        );
        let tip = halves(true, 11, Some((lens, core::f64::consts::FRAC_PI_2)));
        assert_eq!(
            read(on_halves(&s, &tip, &local)),
            Read::Structural,
            "a corner of two circles that are not one"
        );
    }

    /// **An arc of the same circle that does not continue the shared
    /// arc through the vertex is not read on along it** ([`continues`]):
    /// stored on the shared arc's own frame from `(1, 0)` over a quarter
    /// turn, it leaves the vertex back over the shared arc; on the
    /// opposed axis over three quarters of a turn, it continues the shared
    /// arc and then runs back over it. Neither is read, at any depth. Red
    /// if the reading compares the carriers alone. A lower half on a seam
    /// (two uses) is not read either: the face would lie on both its sides.
    #[test]
    fn an_arc_of_one_circle_that_does_not_continue_the_shared_arc_is_not_read() {
        let s = floor();
        let local = both_arcs();
        let (z, x) = (
            Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        );
        let quarter = core::f64::consts::FRAC_PI_2;
        let back = halves(
            true,
            11,
            Some((Curve3::circle(origin(), z, iv0(1.0), x), quarter)),
        );
        assert_eq!(
            read(on_halves(&s, &back, &local)),
            Read::Structural,
            "an arc lying back over the shared arc"
        );
        let long = halves(
            true,
            11,
            Some((Curve3::circle(origin(), -z, iv0(1.0), x), 3.0 * quarter)),
        );
        assert_eq!(
            read(on_halves(&s, &long, &local)),
            Read::Structural,
            "two arcs more than a turn"
        );
        let mut seam = halves(true, 11, None);
        let u = seam.edges[1].uses[0].clone();
        seam.edges[1].uses.push(Use { plus: !u.plus, ..u });
        assert_eq!(
            read(on_halves(&s, &seam, &local)),
            Read::Structural,
            "a lower half on a seam"
        );
    }

    /// **A curve run on past the shared cell reads only where admitted**
    /// ([`sign_on`], [`past_read`]): against a presentation whose zero set
    /// is on the shared cell, under a separator that lists the shared
    /// edge. Not in the carrier arm, though the circle's cylinder vanishes
    /// on the whole curve and departs across it; not against no partner,
    /// nor against another curve run on past it. Red if the past curve is
    /// read as an exact one.
    #[test]
    fn a_curve_past_the_shared_cell_reads_only_where_admitted() {
        let s = floor();
        let local = both_arcs();
        let disc = halves(true, 11, None);
        let Ok(Presentation::Of(p)) = on_halves(&s, &disc, &local) else {
            panic!("the disc is presented about (1, 0)");
        };
        let exact = Presented {
            surface: &s,
            map: Map::Chart,
            x: (1.0, 1.02),
            y: (-0.02, 0.02),
            zero: Zero::Side(Side::XLo, Along::of(&s, &disc, &[0])),
            corner: false,
            cache: Cache::default(),
        };
        let (z, x, y) = (
            Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            Vec3::new(iv0(0.0), iv0(1.0), iv0(0.0)),
        );
        let graph = Psi::Graph {
            m: x,
            t: y,
            slope: iv0(0.0),
            piece: (-1.0, 1.0),
        };
        let on = On {
            edges: vec![edge(10)],
            vertices: Vec::new(),
            whole: false,
        };
        assert_eq!(
            sign_on(&graph, Some(&on), &p, Some(&exact), band()),
            Some(Sign::Negative),
            "admitted"
        );
        let cylinder = Psi::Carrier(Implicit::Cylinder {
            o: origin(),
            axis: z,
            r: iv0(1.0),
        });
        assert_eq!(
            sign_on(&cylinder, None, &p, None, band()),
            None,
            "the carrier arm"
        );
        assert_eq!(
            sign_on(&graph, Some(&on), &p, None, band()),
            None,
            "against no partner"
        );
        let twin = halves(false, 12, None);
        let Ok(Presentation::Of(q)) = on_halves(&s, &twin, &local) else {
            panic!("the outside is presented about (1, 0)");
        };
        assert_eq!(
            sign_on(&graph, Some(&on), &p, Some(&q), band()),
            None,
            "against another curve run on past it"
        );
    }

    /// **Two faces run on along one circle from both sides of the shared
    /// arc do not certify**: the disc and the face outside the circle
    /// share the upper arc, and each is bounded below `(1, 0)` by its own
    /// lower arc on the same circle, so they meet along the lower half,
    /// off the shared cell. Each is read on through the vertex, so the
    /// pair is refused at once ([`stuck`]); red if it is not. Were a past
    /// curve also read against another ([`past_read`]), the graph
    /// separator would certify this pair.
    #[test]
    fn two_faces_run_on_along_one_circle_from_both_sides_do_not_certify() {
        let s = floor();
        let local = both_arcs();
        let (disc, out) = (halves(true, 11, None), halves(false, 12, None));
        let near = Near {
            edges: vec![(0, 0)],
            vertices: Vec::new(),
        };
        let got = certify_pair(
            &piece_on(&s, &disc, &local, 1),
            &piece_on(&s, &out, &local, 2),
            &near,
            band(),
        );
        assert!(
            matches!(
                got,
                Outcome::Refuse(EngineRefusal::Uncertified {
                    arm: CertificateArm::Departure,
                    ..
                })
            ),
            "the pair meets along the lower half: {got:?}"
        );
        let mut single = out.clone();
        single.edges[1].key = edge(11);
        let across = certify_pair(
            &piece_on(&s, &disc, &local, 1),
            &piece_on(&s, &single, &local, 2),
            &near,
            band(),
        );
        assert!(
            matches!(across, Outcome::Refuse(..)),
            "sharing the lower arc too, neither is read on past a shared cell: {across:?}"
        );
    }

    /// [`present_plane`] of `b` in the cell about `(1, 0)` of
    /// [`piece_on`], where the pair shares that vertex alone: neither arc
    /// is on the shared cell.
    fn at_the_vertex_alone<'a>(
        s: &'a Surface<Interval>,
        b: &'a Boundary,
        local: &'a Local,
    ) -> Result<Presentation<'a>, Unpresented> {
        present_plane(&piece_on(s, b, local, 1), &[], &[vertex(1)], band())
    }

    /// **Two arcs of one circle meeting at a vertex a pair shares alone
    /// are read on through it** ([`Reading::Conic`] at an `alone`
    /// corner): the disc on the circle's two halves, neither shared, is
    /// presented inside the circle about `(1, 0)`, its zero the circle run
    /// on through the vertex both ways, along the arc of lower key whatever
    /// order the walls come in. A lower half of an ellipse leaving
    /// the vertex along the circle's tangent, a straight angle of two
    /// conics that are not one, is read by nothing: red if the reading at
    /// a vertex alone skips [`continues`]. A cell about the vertex where
    /// nothing is shared is not read on: red if a corner off the shared
    /// cell is read as one at it. The disc against the face outside the
    /// circle, which meets it along both halves, is refused: each is read
    /// on past the vertex, and neither is read against the other.
    #[test]
    fn two_arcs_of_one_circle_at_a_vertex_alone_are_read_on_through_it() {
        let s = floor();
        let local = both_arcs();
        let disc = halves(true, 11, None);
        match at_the_vertex_alone(&s, &disc, &local) {
            Ok(Presentation::Of(q)) => match &q.zero {
                Zero::Curve(side, c) => {
                    assert_eq!(*side, Side::XHi, "the disc is inside the circle");
                    assert_eq!(c.past, Some(vertex(1)), "run on through the vertex");
                    assert!(
                        c.t.0 < 0.0 && 0.0 < c.t.1,
                        "the curve's parameters straddle the vertex: {:?}",
                        c.t
                    );
                }
                other => panic!("the disc's zero is the circle: {other:?}"),
            },
            other => panic!("the disc is presented about (1, 0): {:?}", read(other)),
        }
        let swapped = Local {
            edges: vec![1, 0],
            vertices: vec![vertex(1)],
        };
        for (local, order) in [(&local, "in order"), (&swapped, "swapped")] {
            let read = match at_the_vertex_alone(&s, &disc, local) {
                Ok(Presentation::Of(q)) => match &q.zero {
                    Zero::Curve(_, c) => c.along.edges.clone(),
                    _ => Vec::new(),
                },
                _ => Vec::new(),
            };
            assert_eq!(
                read,
                vec![edge(10)],
                "the arc of lower key is read, the walls {order}"
            );
        }
        let flat = Curve3::ellipse(
            origin(),
            Vec3::new(iv0(0.0), iv0(0.0), iv0(-1.0)),
            iv0(1.0),
            iv0(0.5),
            Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            band(),
        )
        .expect("the half-ellipse");
        let straight = halves(true, 11, Some((flat, core::f64::consts::PI)));
        assert_eq!(
            read(at_the_vertex_alone(&s, &straight, &local)),
            Read::Structural,
            "a straight angle of a circle and an ellipse"
        );
        assert_eq!(
            read(present_plane(
                &piece_on(&s, &disc, &local, 1),
                &[],
                &[],
                band()
            )),
            Read::Local,
            "a cell at the vertex with nothing shared"
        );
        assert_eq!(
            read(present_plane(
                &piece_on(&s, &disc, &local, 1),
                &[],
                &[vertex(2)],
                band()
            )),
            Read::Local,
            "a cell at the vertex with another vertex shared"
        );
        let out = halves(false, 12, None);
        let near = Near {
            edges: Vec::new(),
            vertices: vec![vertex(1)],
        };
        let got = certify_pair(
            &piece_on(&s, &disc, &local, 1),
            &piece_on(&s, &out, &local, 2),
            &near,
            band(),
        );
        assert!(
            matches!(got, Outcome::Refuse(..)),
            "the disc and the outside meet along both halves: {got:?}"
        );
    }

    /// **At a vertex a pair shares alone, the graph separator is built over
    /// the conic a face is read on along** ([`graph_of`]): it lists the
    /// read arc and the vertex. A quadrant of the plane `x = 1` leaving
    /// `(1, 0)` along `(0, 0.6, 0.8)` and `(0, −0.8, 0.6)`, neither tangent
    /// to the circle, meets the disc there alone, and the two take opposite
    /// signs under it: red if the separator does not list the vertex, or
    /// if the disc is read on the wrong side of the circle. A quadrant of
    /// the plane `y = 0`, `x ≤ 1, z ≥ 0`, whose edge runs into the disc,
    /// crosses it, and is not parted from it.
    #[test]
    fn the_graph_through_a_vertex_alone_parts_a_corner_off_the_disc_and_not_one_across_it() {
        let s = floor();
        let local = both_arcs();
        let disc = halves(true, 11, None);
        let x = piece_on(&s, &disc, &local, 1);
        let Ok(Presentation::Of(p)) = at_the_vertex_alone(&s, &disc, &local) else {
            panic!("the disc is presented about (1, 0)");
        };
        let near = Near {
            edges: Vec::new(),
            vertices: vec![vertex(1)],
        };
        let axis = |x: f64, y: f64, z: f64| Vec3::new(iv0(x), iv0(y), iv0(z));
        let plane = |normal, u_ref| Surface::Plane {
            origin: Point3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            normal,
            u_ref,
        };
        let (off, across) = (
            plane(axis(1.0, 0.0, 0.0), axis(0.0, 0.6, 0.8)),
            plane(axis(0.0, 1.0, 0.0), axis(-1.0, 0.0, 0.0)),
        );
        let quadrant = |surface| Presented {
            surface,
            map: Map::Affine {
                o: [iv0(0.0), iv0(0.0)],
                a: [iv0(1.0), iv0(0.0)],
                b: [iv0(0.0), iv0(1.0)],
            },
            x: (0.0, 0.02),
            y: (0.0, 0.02),
            zero: Zero::Point(Side::XLo, Side::YLo, vertex(1)),
            corner: true,
            cache: Cache::default(),
        };
        let parted = |q: &Presented<'_>| {
            graph_of(&x, &x, &near, &p, q, band()).is_some_and(|(psi, on)| {
                match (
                    sign_on(&psi, Some(&on), &p, Some(q), band()),
                    sign_on(&psi, Some(&on), q, Some(&p), band()),
                ) {
                    (Some(a), Some(b)) => a == flip(b),
                    _ => false,
                }
            })
        };
        let (_, on) = graph_of(&x, &x, &near, &p, &quadrant(&off), band()).expect("a graph");
        assert_eq!(
            (on.edges.as_slice(), on.vertices.as_slice()),
            (&[edge(10)][..], &[vertex(1)][..]),
            "the read arc and the vertex"
        );
        assert!(parted(&quadrant(&off)), "the quadrant off the disc");
        assert!(
            !parted(&quadrant(&across)),
            "the quadrant whose edge runs into the disc"
        );
    }

    /// **Near misses at a vertex a pair shares alone** (review rows):
    /// the crossing quadrant of the row above with its axes swapped, its
    /// second axis the one into the disc (red if a [`Zero::Point`]'s
    /// sign is read off its first axis alone), and a quadrant whose edge
    /// leaves `(1, 0)` a little inside the circle's tangent, so it runs
    /// into the disc at second order: neither is parted from the disc. A
    /// quadrant a little outside the tangent meets the disc at the
    /// vertex alone, and is parted.
    #[test]
    fn a_crossing_quadrant_tangent_to_the_conic_at_a_vertex_alone_is_not_parted() {
        let s = floor();
        let local = both_arcs();
        let disc = halves(true, 11, None);
        let x = piece_on(&s, &disc, &local, 1);
        let Ok(Presentation::Of(p)) = at_the_vertex_alone(&s, &disc, &local) else {
            panic!("the disc is presented about (1, 0)");
        };
        let near = Near {
            edges: Vec::new(),
            vertices: vec![vertex(1)],
        };
        let axis = |x: f64, y: f64, z: f64| Vec3::new(iv0(x), iv0(y), iv0(z));
        let plane = |normal, u_ref| Surface::Plane {
            origin: Point3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            normal,
            u_ref,
        };
        let quadrant = |surface| Presented {
            surface,
            map: Map::Affine {
                o: [iv0(0.0), iv0(0.0)],
                a: [iv0(1.0), iv0(0.0)],
                b: [iv0(0.0), iv0(1.0)],
            },
            x: (0.0, 0.02),
            y: (0.0, 0.02),
            zero: Zero::Point(Side::XLo, Side::YLo, vertex(1)),
            corner: true,
            cache: Cache::default(),
        };
        let parted = |q: &Presented<'_>| {
            graph_of(&x, &x, &near, &p, q, band()).is_some_and(|(psi, on)| {
                match (
                    sign_on(&psi, Some(&on), &p, Some(q), band()),
                    sign_on(&psi, Some(&on), q, Some(&p), band()),
                ) {
                    (Some(a), Some(b)) => a == flip(b),
                    _ => false,
                }
            })
        };
        // The plane y = 0: first axis +z, second axis -x, into the disc.
        let swapped = plane(axis(0.0, -1.0, 0.0), axis(0.0, 0.0, 1.0));
        // First axis +z, second axis the tangent (0, 1, 0) turned by
        // `-s` toward the centre.
        let tangent = |s: f64| {
            let l = (1.0 + s * s).sqrt();
            plane(axis(-1.0 / l, -s / l, 0.0), axis(0.0, 0.0, 1.0))
        };
        assert!(
            !parted(&quadrant(&swapped)),
            "the crossing quadrant, its second axis into the disc"
        );
        let (inward, outward) = (tangent(0.05), tangent(-0.05));
        assert!(
            !parted(&quadrant(&inward)),
            "the quadrant just inside the tangent"
        );
        assert!(
            parted(&quadrant(&outward)),
            "the quadrant just outside the tangent"
        );
    }

    /// **`one_conic` admits two carriers only as one conic**, each
    /// component on its own: the same circle stored on the opposed axis
    /// and another `u_ref`, and an ellipse with axis and `u_ref` both
    /// turned over, are one; a circle moved along its axis, of another
    /// radius, on a tilted axis, an ellipse of other semi-axes or turned
    /// a quarter about its axis, and a circle beside an ellipse of equal
    /// semi-axes are not; a bracketed centre is not one value even
    /// against itself. Red if any one comparison is dropped.
    #[test]
    fn one_conic_compares_each_component() {
        let p = |x: f64, y: f64, z: f64| Point3::new(iv0(x), iv0(y), iv0(z));
        let v = |x: f64, y: f64, z: f64| Vec3::new(iv0(x), iv0(y), iv0(z));
        let rim = |c: Curve3<Interval>| Rim {
            key: edge(1),
            arc: Some((c, (iv0(0.0), iv0(1.0)))),
            ends: [vertex(1), vertex(2)],
            uses: Vec::new(),
        };
        let circle = |o, a, r: f64, u| rim(Curve3::circle(o, a, iv0(r), u));
        let ellipse = |o, a, su: f64, sv: f64, u| {
            rim(Curve3::Ellipse(geom::Ellipse::new(
                geom::Frame::new(o, a, u),
                iv0(su),
                iv0(sv),
            )))
        };
        let (o, z, x, y) = (p(0., 0., 0.), v(0., 0., 1.), v(1., 0., 0.), v(0., 1., 0.));
        let c = circle(o, z, 1.0, x);
        assert!(
            one_conic(&c, &circle(o, v(0., 0., -1.), 1.0, y)),
            "opposed axis"
        );
        assert!(
            !one_conic(&c, &circle(p(0., 0., 1.), z, 1.0, x)),
            "moved along its axis"
        );
        assert!(!one_conic(&c, &circle(o, z, 2.0, x)), "another radius");
        assert!(
            !one_conic(&c, &circle(o, v(0., 1., 0.), 1.0, x)),
            "a tilted axis"
        );
        assert!(!one_conic(&c, &ellipse(o, z, 1.0, 1.0, x)), "an ellipse");
        let e = ellipse(o, z, 2.0, 1.0, x);
        assert!(
            one_conic(&e, &ellipse(o, v(0., 0., -1.), 2.0, 1.0, v(-1., 0., 0.))),
            "turned over"
        );
        assert!(
            !one_conic(&e, &ellipse(o, z, 2.0, 1.5, x)),
            "other semi-axes"
        );
        assert!(
            !one_conic(&e, &ellipse(o, z, 2.0, 1.0, y)),
            "a quarter turn"
        );
        let bracket = Point3::new(Interval::from_bounds(-1e-9, 1e-9), iv0(0.0), iv0(0.0));
        let b = circle(bracket, z, 1.0, x);
        assert!(!one_conic(&b, &b.clone()), "a bracketed centre");
    }

    /// [`present_plane`] of a [`quarter`] in the cell `u × v`, its local
    /// edges the arc and the line and its local vertex `(1, 0)`, that
    /// vertex alone shared: the zero it reads, or why it reads none.
    fn at_vertex_alone(b: &Boundary, u: (f64, f64), v: (f64, f64)) -> Result<Zero, Read> {
        let s = floor();
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
        match present_plane(&piece, &[], &[vertex(1)], band()) {
            Ok(Presentation::Of(q)) => Ok(q.zero),
            other => Err(read(other)),
        }
    }

    /// **A plane's corner of a line and an arc at a vertex the pair
    /// shares alone is its quadrant** ([`present_vertex_corner`]): the
    /// quarter disc's line from the origin meets its arc at `(1, 0)`, a
    /// convex corner, so a cell holding the vertex, or stopping short of
    /// it, is presented with the corner its zero. Where the line leaves
    /// the vertex the other way the corner is reflex (`Structural`);
    /// where it runs along the arc's tangent the corner is straight,
    /// which no quadrant holds (`Local`). With the face outside the
    /// circle the arc bends back across the quadrant's second side
    /// within a large enough cell, which is not presented: red if the
    /// conic's rise along the line is not read.
    #[test]
    fn a_planes_corner_at_a_vertex_alone_is_its_quadrant() {
        let convex = quarter(segment(20, [3, 1], (0.0, 0.0), (1.0, 0.0)), Vec::new());
        for (u, v, what) in [
            ((0.98, 1.02), (-0.02, 0.02), "about the vertex"),
            (
                (0.97, 0.995),
                (0.0, 0.02),
                "short of it, on the face's side",
            ),
        ] {
            let got = at_vertex_alone(&convex, u, v);
            assert!(
                matches!(got, Ok(Zero::Point(Side::XLo, Side::YLo, v)) if v == vertex(1)),
                "{what}: the quadrant's corner is the zero: {got:?}"
            );
        }
        let reflex = quarter(segment(20, [3, 1], (2.0, 0.0), (1.0, 0.0)), Vec::new());
        assert_eq!(
            at_vertex_alone(&reflex, (0.98, 1.02), (-0.02, 0.02)).err(),
            Some(Read::Structural),
            "the reflex corner at (1, 0)"
        );
        let tangent = quarter(segment(20, [3, 1], (1.0, -1.0), (1.0, 0.0)), Vec::new());
        assert_eq!(
            at_vertex_alone(&tangent, (0.98, 1.02), (-0.02, 0.02)).err(),
            Some(Read::Local),
            "the straight join at (1, 0)"
        );
        let mut outside = reflex;
        outside.sense = false;
        assert!(
            matches!(
                at_vertex_alone(&outside, (0.98, 1.02), (-0.02, 0.02)),
                Ok(Zero::Point(..))
            ),
            "outside the circle, about the vertex"
        );
        assert_eq!(
            at_vertex_alone(&outside, (0.5, 1.05), (-0.05, 0.6)).err(),
            Some(Read::Local),
            "outside the circle, the arc bending back within the cell"
        );
    }

    /// **The unit sphere's octant about its north pole**: the equator
    /// from `u = 0` to `π/2`, the meridian `u = π/2` up to the pole
    /// (vertex 3), and the meridian `u = 0` back down; edges 0, 1, 2.
    fn octant() -> (Surface<Interval>, Boundary) {
        let z = Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0));
        let x = Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0));
        let s = Surface::Sphere {
            center: origin(),
            radius: iv0(1.0),
            axis: z,
            u_ref: x,
        };
        let q = core::f64::consts::FRAC_PI_2;
        let p = |x: f64, y: f64, z: f64| Point3::new(iv0(x), iv0(y), iv0(z));
        let b = Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 2], Axis::V, 0.0, (0.0, 0.0), (q, 0.0)),
                rim(2, [2, 3], Axis::U, q, (q, 0.0), (q, q)),
                rim(3, [3, 1], Axis::U, 0.0, (0.0, q), (0.0, 0.0)),
            ],
            vertices: BTreeMap::from([
                (vertex(1), p(1.0, 0.0, 0.0)),
                (vertex(2), p(0.0, 1.0, 0.0)),
                (vertex(3), p(0.0, 0.0, 1.0)),
            ]),
            extent: Some([(0.0, q), (0.0, q)]),
        };
        (s, b)
    }

    /// **A sphere's pole is a collapsed side of its chart**
    /// ([`present_chart`]): a cell of the octant reaching `v = π/2`, the
    /// pole a vertex the pair shares alone, is presented with the side
    /// `v = π/2` its zero, whether it holds both meridians or neither.
    /// A cell short of the pole holds no zero and is not presented.
    #[test]
    fn a_spheres_pole_is_a_collapsed_side_of_its_chart() {
        let (s, b) = octant();
        let top = Interval::pi().hi() * 0.5;
        let at_pole = |u: (f64, f64), v: (f64, f64)| {
            let local = Local {
                edges: vec![1, 2],
                vertices: vec![vertex(3)],
            };
            let piece = Piece {
                face: FaceKey::default(),
                surface: &s,
                whole: false,
                boundary: &b,
                u,
                v,
                local: &local,
                ball: Ball::holding(origin(), origin()),
            };
            match present_chart(&piece, &[], &[vertex(3)], band()) {
                Ok(Presentation::Of(q)) => Some((q.zero, q.y)),
                _ => None,
            }
        };
        for (u, what) in [
            ((0.7, 0.8), "between the meridians"),
            ((0.0, core::f64::consts::FRAC_PI_2), "across both"),
        ] {
            let got = at_pole(u, (1.5, top));
            assert!(
                matches!(&got, Some((Zero::Side(Side::YHi, a), (1.5, hi))) if *hi >= top && a.vertex == Some(vertex(3))),
                "{what}: the pole's side is the zero: {got:?}"
            );
        }
        assert!(
            at_pole((0.7, 0.8), (1.4, 1.5)).is_none(),
            "a cell short of the pole"
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
                Curve3::line(
                    Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                ),
                (iv0(t.0), iv0(t.1)),
            )),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                wall: None,
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
                Curve3::line(on_cyl(1.0, 0.5), Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0))),
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
                wall: None,
                harmonic: None,
            }],
        };
        let mut vertices = BTreeMap::from([
            (vertex(1), on_cyl(1.0, 0.5)),
            (vertex(2), on_cyl(1.0, 1.0)),
            (vertex(3), on_cyl(far.0, far.1)),
        ]);
        for e in &extra {
            if let Some((Curve3::Line(geom::Line { origin, dir }), _)) = &e.arc {
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
        // `u = t`, `v = ½ + (t − 1)/1000` from a turn and more back: it
        // leaves the vertex into the face, and its turn before passes
        // through the cell at `v ≈ 0.494`. Red if [`leaves_line`] does
        // not read the curve's other copies ([`copy_misses`]).
        let helix = ruling_corner(
            harmonic_rim(
                11,
                [3, 1],
                harmonic([0.0, 0.499], 1.0, (0.0, 0.0, 0.001)),
                (1.0 - TURN - 0.3, 1.0),
            ),
            Vec::new(),
            (0.7, 0.4927),
        );
        assert_eq!(
            on_cylinder(&helix, cell.0, cell.1),
            Read::Local,
            "the helix's turn before passes through the cell"
        );
    }

    /// **A cylinder face's corner at `(1, ½)`, a vertex it shares
    /// alone**: the ruling `u = 1` (edge `n`) run from the vertex up
    /// (`up`) or down, and the parallel `v = ½` (edge `n + 1`) between the
    /// vertex and `u = ½` (`left`) or `u = 3/2`, its 3-D arc its circle,
    /// its parameter `u`, its image stored as a harmonic, not a
    /// coordinate line, as a re-posed body's is. The face is the quadrant
    /// between them, so with `sense` it lies on the ruling's side the
    /// parallel leaves into. A `curve` in its place replaces the
    /// parallel: a helix or a weave, which no analytic carrier holds, so
    /// its 3-D arc is [`harmonic_rim`]'s, out of reach, and [`only`] sees
    /// its ends where they are.
    fn vertex_quadrant(n: u64, up: bool, left: bool, sense: bool, curve: Option<Rim>) -> Boundary {
        let dv = if up { 1.0 } else { -1.0 };
        let du = if left { 1.0 } else { -1.0 };
        let ruling = Rim {
            key: edge(n),
            arc: Some((
                Curve3::line(on_cyl(1.0, 0.5), Vec3::new(iv0(0.0), iv0(0.0), iv0(dv))),
                (iv0(0.0), iv0(0.5)),
            )),
            ends: [vertex(1), vertex(2)],
            uses: vec![Use {
                plus: true,
                image: Some([p2((1.0, 0.5)), p2((1.0, 0.5 + 0.5 * dv))]),
                iso: Some(Iso {
                    held: Axis::U,
                    at: iv0(1.0),
                    full: false,
                }),
                wall: None,
                harmonic: None,
            }],
        };
        let parallel = curve.unwrap_or_else(|| {
            let (ends, t) = if left {
                ([3, 1], (0.5, 1.0))
            } else {
                ([1, 3], (1.0, 1.5))
            };
            let mut rim = harmonic_rim(n + 1, ends, harmonic([0.0, 0.5], 1.0, (0.0, 0.0, 0.0)), t);
            rim.arc = Some((
                Curve3::circle(
                    Point3::new(iv0(0.0), iv0(0.0), iv0(0.5)),
                    Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                    iv0(1.0),
                    Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                ),
                (iv0(t.0), iv0(t.1)),
            ));
            rim
        });
        Boundary {
            sense,
            edges: vec![ruling, parallel],
            vertices: BTreeMap::from([
                (vertex(1), on_cyl(1.0, 0.5)),
                (vertex(2), on_cyl(1.0, 0.5 + 0.5 * dv)),
                (vertex(3), on_cyl(1.0 - 0.5 * du, 0.5)),
            ]),
            extent: Some([(0.0, 2.0), (0.0, 1.0)]),
        }
    }

    /// A cylinder piece of `b` in the cell `u × v`, its local edges the
    /// corner's two and its local vertex `(1, ½)`.
    fn quadrant_piece<'a>(
        s: &'a Surface<Interval>,
        b: &'a Boundary,
        local: &'a Local,
        n: u64,
        (u, v): ((f64, f64), (f64, f64)),
    ) -> Piece<'a> {
        Piece {
            face: face(n),
            surface: s,
            whole: false,
            boundary: b,
            u,
            v,
            local,
            ball: Ball::holding(origin(), origin()),
        }
    }

    /// **A cylinder's corner of a ruling and a harmonic curve at a
    /// vertex the pair shares alone is its quadrant**
    /// ([`present_vertex_corner`]), the torus belly's corner: the ruling
    /// up from `(1, ½)` and the parallel reaching it from `u < 1` meet
    /// at a right angle of the chart, which no quadrant of the chart's
    /// own axes holds strictly, so the quadrant's second axis leans off
    /// the parallel. The reflex corner, the parallel leaving into
    /// `u > 1`, is `Structural`. A helix that reaches the vertex along
    /// `v = ½` and comes round a turn later through the cell below it is
    /// not presented: red if the curve's copies by whole turns are not
    /// read.
    #[test]
    fn a_cylinders_corner_at_a_vertex_alone_is_its_quadrant() {
        let s = cylinder();
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let at_vertex = |b: &Boundary, cell| {
            let piece = quadrant_piece(&s, b, &local, 1, cell);
            match present_chart(&piece, &[], &[vertex(1)], band()) {
                Ok(Presentation::Of(q)) => Ok(q.zero),
                other => Err(read(other)),
            }
        };
        let about = ((0.98, 1.02), (0.48, 0.52));
        let convex = vertex_quadrant(10, true, true, true, None);
        for (cell, what) in [
            (about, "about the vertex"),
            (((0.97, 0.995), (0.5, 0.52)), "short of it, inside"),
        ] {
            let got = at_vertex(&convex, cell);
            assert!(
                matches!(got, Ok(Zero::Point(Side::XLo, Side::YLo, v)) if v == vertex(1)),
                "{what}: the quadrant's corner is the zero: {got:?}"
            );
        }
        let reflex = vertex_quadrant(10, true, false, true, None);
        assert_eq!(
            at_vertex(&reflex, about).err(),
            Some(Read::Structural),
            "the reflex corner at (1, ½)"
        );
        // `u = t`, `v = ½ + (t − 1)/1000` from a turn and more back: the
        // turn before reaches `u = 1` at `v ≈ 0.494`, inside the cell.
        let helix = harmonic_rim(
            11,
            [3, 1],
            harmonic([0.0, 0.499], 1.0, (0.0, 0.0, 0.001)),
            (1.0 - TURN - 0.3, 1.0),
        );
        let wound = vertex_quadrant(10, true, true, true, Some(helix));
        assert!(
            matches!(
                at_vertex(&convex, ((0.98, 1.02), (0.49, 0.52))),
                Ok(Zero::Point(..))
            ),
            "the parallel's corner over the helix's cell"
        );
        assert_eq!(
            at_vertex(&wound, ((0.98, 1.02), (0.49, 0.52))).err(),
            Some(Read::Local),
            "the helix's turn before passes through the cell"
        );
        assert_eq!(
            at_vertex(&convex, ((0.45, 1.02), (0.48, 0.52))).err(),
            Some(Read::Local),
            "the parallel's far end, (½, ½), in the cell"
        );
        // `u = 1 − sin(t)/50`, `v = ½ + t/250` from the vertex: it leaves
        // into `u < 1` and is back across the ruling at `t = π`, inside
        // the cell, before it leaves it.
        let weave = harmonic_rim(
            11,
            [1, 3],
            Harmonic {
                p0: [iv0(1.0), iv0(0.5)],
                pa: [iv0(0.0), iv0(0.0)],
                pb: [iv0(-0.02), iv0(0.0)],
                pl: [iv0(0.0), iv0(0.004)],
            },
            (0.0, 20.0),
        );
        let woven = vertex_quadrant(10, true, true, true, Some(weave));
        assert_eq!(
            at_vertex(&woven, ((0.98, 1.02), (0.48, 0.515))).err(),
            Some(Read::Local),
            "a curve back across the ruling within the cell"
        );
    }

    /// **Two cylinder corners at a vertex they share alone**
    /// ([`certify_pair`]): the face above the parallel and left of the
    /// ruling against the one below it and right, diagonally opposite
    /// quadrants of one tangent plane, certify through the plane across
    /// their headings. Against the face below the parallel and LEFT of the
    /// ruling, whose quadrant shares the parallel's direction with the
    /// first — two faces meeting along a curve neither bounds by a shared
    /// edge — no separator departs both strictly, and the pair is not
    /// certified. Red if either quadrant is read on the wrong side.
    #[test]
    fn two_cylinder_corners_at_a_vertex_alone_certify_only_where_their_quadrants_part() {
        let s = cylinder();
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let near = Near {
            edges: Vec::new(),
            vertices: vec![vertex(1)],
        };
        let cell = ((0.98, 1.02), (0.48, 0.52));
        let above = vertex_quadrant(10, true, true, true, None);
        let across = vertex_quadrant(20, false, false, true, None);
        let beside = vertex_quadrant(30, false, true, false, None);
        let a = quadrant_piece(&s, &above, &local, 1, cell);
        for (other, n, parts, what) in [
            (&across, 2, true, "diagonally opposite"),
            (&beside, 3, false, "along the parallel"),
        ] {
            let b = quadrant_piece(&s, other, &local, n, cell);
            let got = certify_pair(&a, &b, &near, band());
            assert_eq!(
                matches!(got, Outcome::Certified { .. }),
                parts,
                "{what}: {got:?}"
            );
        }
    }

    /// **A cylinder face's corner at `(1, ½)` where the parallel `v = ½`
    /// (edge 10, run from vertex 2 at `u = ½` to the vertex) meets the
    /// plane section `u = t`, `v = ½ + s·(sin t − sin 1)` (edge 11,
    /// reaching the vertex at `t = 1` from `t = 0.3`), the face below the
    /// parallel.** The section's 3-D arc is the ellipse it is, the
    /// parallel's its circle: `only` reads the curves where they are.
    fn parallel_quadrant(s: f64) -> Boundary {
        let parallel = Rim {
            key: edge(10),
            arc: Some((
                Curve3::circle(
                    Point3::new(iv0(0.0), iv0(0.0), iv0(0.5)),
                    Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                    iv0(1.0),
                    Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                ),
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
                wall: None,
                harmonic: None,
            }],
        };
        let b2 = 1.0 + s * s;
        let section = Curve3::ellipse(
            Point3::new(iv0(0.0), iv0(0.0), iv0(0.5 - s * 1f64.sin())),
            Vec3::new(iv0(0.0), iv0(-s / b2.sqrt()), iv0(1.0 / b2.sqrt())),
            iv0(1.0),
            iv0(b2.sqrt()),
            Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            band(),
        )
        .expect("the section's ellipse");
        let curve = Rim {
            key: edge(11),
            arc: Some((section, (iv0(0.3), iv0(1.0)))),
            ends: [vertex(3), vertex(1)],
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                wall: None,
                harmonic: Some(Harmonic {
                    p0: [iv0(0.0), iv0(0.5 - s * 1f64.sin())],
                    pa: [iv0(0.0), iv0(0.0)],
                    pb: [iv0(0.0), iv0(s)],
                    pl: [iv0(1.0), iv0(0.0)],
                }),
            }],
        };
        Boundary {
            sense: true,
            edges: vec![parallel, curve],
            vertices: BTreeMap::from([
                (vertex(1), on_cyl(1.0, 0.5)),
                (vertex(2), on_cyl(0.5, 0.5)),
                (
                    vertex(3),
                    on_cyl(0.3, 0.5 + s * (0.3f64.sin() - 1f64.sin())),
                ),
            ]),
            extent: Some([(0.0, 2.0), (-1.0, 1.0)]),
        }
    }

    /// **A parallel's corner at a vertex alone is its quadrant too**
    /// ([`present_vertex_corner`] with the straight edge's held axis
    /// `v`): the section leaving `(1, ½)` below the parallel, `s = 1`,
    /// is presented with the corner its zero; leaving above it, `s = −1`,
    /// the corner is reflex, `Structural`. Red if the face side of a
    /// parallel is read on the wrong axis or sign.
    #[test]
    fn a_parallels_corner_at_a_vertex_alone_is_its_quadrant() {
        let s = cylinder();
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let at_vertex = |b: &Boundary| {
            let piece = quadrant_piece(&s, b, &local, 1, ((0.98, 1.02), (0.48, 0.52)));
            match present_chart(&piece, &[], &[vertex(1)], band()) {
                Ok(Presentation::Of(q)) => Ok(q.zero),
                other => Err(read(other)),
            }
        };
        let got = at_vertex(&parallel_quadrant(1.0));
        assert!(
            matches!(got, Ok(Zero::Point(Side::XLo, Side::YLo, v)) if v == vertex(1)),
            "the section below the parallel: {got:?}"
        );
        assert_eq!(
            at_vertex(&parallel_quadrant(-1.0)).err(),
            Some(Read::Structural),
            "the section above the parallel"
        );
    }

    /// **On a cylinder of radius 0.055 the vertex-alone corner is still
    /// its quadrant** ([`present_vertex_corner`]): the chart's `u`-speed
    /// is 0.055 against `v`'s 1, the torus belly's, and a lean of a
    /// quarter of `A` taken in chart units does not present the corner
    /// (`Local`, measured). Red if the lean is
    /// chosen in chart units rather than in the surface's metric at the
    /// vertex; every other row's cylinder has radius 1, where the two
    /// agree.
    #[test]
    fn a_thin_cylinders_corner_at_a_vertex_alone_leans_in_the_surfaces_metric() {
        let r = 0.055;
        let s = Surface::Cylinder {
            origin: origin(),
            axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            radius: iv0(r),
            u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        };
        let mut b = vertex_quadrant(10, true, true, true, None);
        let thin = |p: Point3<Interval>| Point3::new(p.x * iv0(r), p.y * iv0(r), p.z);
        for p in b.vertices.values_mut() {
            *p = thin(*p);
        }
        if let Some((Curve3::Line(l), _)) = &mut b.edges[0].arc {
            l.origin = thin(l.origin);
        }
        if let Some((Curve3::Circle(c), _)) = &mut b.edges[1].arc {
            c.radius = iv0(r);
        }
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let piece = quadrant_piece(&s, &b, &local, 1, ((0.98, 1.02), (0.48, 0.52)));
        let got = match present_chart(&piece, &[], &[vertex(1)], band()) {
            Ok(Presentation::Of(q)) => Ok(q.zero),
            other => Err(read(other)),
        };
        assert!(
            matches!(got, Ok(Zero::Point(Side::XLo, Side::YLo, v)) if v == vertex(1)),
            "the thin cylinder's quadrant about the vertex: {got:?}"
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
                Curve3::circle(
                    Point3::new(iv0(0.0), iv0(0.0), iv0(0.5)),
                    Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                    iv0(1.0),
                    Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                ),
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
                wall: None,
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
                Curve3::line(
                    Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                ),
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
                wall: None,
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
            Curve3::circle(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(-1.0)),
                iv0(1.0),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
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
                Curve3::line(
                    Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                ),
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
                wall: None,
                harmonic: None,
            }],
        };
        let ruling = Rim {
            key: edge(31),
            arc: Some((
                Curve3::line(
                    Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                ),
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
                wall: None,
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

    /// **An other edge that misses the vertex is read by neither
    /// reading**, whichever chart axis it misses along, and by however
    /// little the funnel cannot call nothing:
    /// - the quarter disc's line ends a millimetre off the arc's end, along
    ///   `u` at `(1.001, 0)` and along `v` at `(1, 0.001)`;
    /// - the ruling corner's curve ends a millimetre off the vertex, along
    ///   `u` at `(0.999, ½)` and along `v` at `(1, ½ + 0.001)`;
    /// - the line ends off the arc's end along `v` by a gap between the
    ///   band's zero and its escalation, which no sign decides.
    ///
    /// Red if [`on_vertex`] reads two points that far apart as one, reads
    /// one axis's gap only, or reads an undecided gap as none.
    #[test]
    fn an_other_edge_off_the_vertex_is_not_read_through_it() {
        let disc = |end: (f64, f64)| quarter(segment(20, [3, 1], (0.0, 0.0), end), Vec::new());
        let floor_at = |end, what| {
            assert_eq!(
                on_floor(
                    &disc(end),
                    vec![0, 1],
                    vec![vertex(1)],
                    (0.98, 1.02),
                    (-0.02, 0.02)
                ),
                Read::Local,
                "{what}"
            );
        };
        floor_at((1.001, 0.0), "the line a millimetre off along u");
        floor_at((1.0, 0.001), "the line a millimetre off along v");
        let b = band();
        floor_at(
            (1.0, (b.zero() * b.escalate()).sqrt()),
            "the line off by an undecided gap",
        );
        let ruling_at = |p0: [f64; 2], what| {
            let off = ruling_corner(
                harmonic_rim(11, [3, 1], harmonic(p0, 1.0, (0.0, 0.0, -1.0)), (0.5, 1.0)),
                Vec::new(),
                (p0[0] + 0.5, p0[1] - 0.5),
            );
            assert_eq!(
                on_cylinder(&off, (0.95, 1.0), (0.45, 0.55)),
                Read::Local,
                "{what}"
            );
        };
        ruling_at([-0.001, 1.5], "the curve a millimetre off along u");
        ruling_at([0.0, 1.501], "the curve a millimetre off along v");
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
                    Curve3::line(on_cyl(1.015, -0.5), Vec3::new(iv0(0.0), iv0(0.0), iv0(2.0))),
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
        let [u, v] = h.at(t);
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
        let s = cylinder();
        let side = |clips: &[Clip], near: &[usize], vertices: &[VertexKey]| {
            partial_zero(&s, &b, clips, near, vertices).map(|(side, a)| (side, a.edges))
        };
        let line = clip(Side::YLo, vec![1, 3]);
        assert_eq!(
            side(&[line], &[1], &[vertex(3)]),
            Some((Side::YLo, vec![b.edges[1].key])),
            "one shared edge, one not: the shared one is the side's cell"
        );
        let line = clip(Side::YLo, vec![1, 3]);
        assert_eq!(side(&[line], &[], &[]), None, "no shared edge on it");
        let (a, c) = (clip(Side::YLo, vec![1, 3]), clip(Side::XHi, vec![0]));
        assert_eq!(
            side(&[a, c], &[1, 0], &[]),
            None,
            "two clips hold shared edges"
        );
        let line = clip(Side::YLo, vec![1, 3]);
        assert_eq!(
            side(&[line], &[1], &[vertex(9)]),
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
                    Curve3::Ellipse(geom::Ellipse::new(
                        geom::Frame::new(
                            origin(),
                            Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                            Vec3::new(iv0(c), iv0(s), iv0(0.0)),
                        ),
                        iv0(a),
                        iv0(b),
                    )),
                    (iv0(0.0), iv0(TURN)),
                )),
                ends: [vertex(1), vertex(1)],
                uses: vec![Use {
                    plus: true,
                    image: None,
                    iso: None,
                    wall: None,
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
                    Curve3::Ellipse(geom::Ellipse::new(
                        geom::Frame::new(
                            origin(),
                            Vec3::new(iv0(0.0), iv0(-0.5 / norm), iv0(1.0 / norm)),
                            Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                        ),
                        iv0(1.0),
                        iv0(norm),
                    )),
                    n,
                )),
                ends: [vertex(1), vertex(2)],
                uses: vec![Use {
                    plus: true,
                    image: None,
                    iso: None,
                    wall: None,
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
                Curve3::line(lip(0.0502), Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0))),
                (iv0(0.0), iv0(1e-4)),
            )),
            ends: [vertex(2), vertex(3)],
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                wall: None,
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
            sign_on(&carrier(0.7), None, &q, None, band()).is_some(),
            "the rim's own cylinder separates"
        );
        assert_eq!(
            sign_on(&carrier(0.69), None, &q, None, band()),
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

    /// **A curve is zero under a separator the engine builds where it
    /// lists the curve's edge** ([`sign_on`]): no value is read along
    /// it. A plane listing the rim reads the curve zero and the face's
    /// sign off it; a plane listing another edge, or one that does not
    /// hold the rim's whole circle, answers nothing; the graph separator
    /// reads it zero where the curve's parameters lie in its piece, and
    /// answers nothing past it.
    #[test]
    fn a_curve_is_zero_where_its_separator_lists_its_edge() {
        let s = floor();
        let b = disk(0.7, Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)), true, true);
        let local = Local {
            edges: vec![0],
            vertices: Vec::new(),
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &b,
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
        let rim = b.edges[0].key;
        assert!(c.along.whole, "a circle's own plane holds it whole");
        let x = Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0));
        // `x = 0` holds none of the rim: membership is what is read.
        let plane = Psi::Linear { n: x, q: origin() };
        let on = |edges: Vec<EdgeKey>, whole: bool| On {
            edges,
            vertices: Vec::new(),
            whole,
        };
        assert!(
            sign_on(&plane, Some(&on(vec![rim], true)), &q, None, band()).is_some(),
            "listed"
        );
        assert_eq!(
            sign_on(&plane, Some(&on(vec![edge(9)], true)), &q, None, band()),
            None,
            "another edge listed"
        );
        assert_eq!(
            sign_on(&plane, Some(&on(vec![rim], false)), &q, None, band()),
            None,
            "the rim's circle not held whole"
        );
        let graph = |piece| Psi::Graph {
            m: x,
            t: Vec3::new(iv0(0.0), iv0(1.0), iv0(0.0)),
            slope: iv0(0.0),
            piece,
        };
        let one = on(vec![rim], false);
        assert!(
            sign_on(
                &graph((c.t.0 - 0.1, c.t.1 + 0.1)),
                Some(&one),
                &q,
                None,
                band()
            )
            .is_some(),
            "within the graph's piece"
        );
        assert_eq!(
            sign_on(
                &graph((c.t.0 + 1e-3, c.t.1 + 0.1)),
                Some(&one),
                &q,
                None,
                band()
            ),
            None,
            "past the graph's piece"
        );
    }

    /// **A side past the shared cell reads a sign only where it is
    /// admitted** ([`sign_on`]): against a presentation whose zero set is
    /// on the shared cell, under a plane that lists the side's shared
    /// edge and holds its whole line. Not in the carrier arm (no other
    /// presentation), not under the graph separator, not against another
    /// past side, not under a plane that does not list the edge, not
    /// where the side's chart line is not the edge's own, and not on a
    /// free-form carrier, whose injectivity no kind gives.
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
        let line = Along {
            edges: vec![edge(1)],
            vertex: None,
            whole: true,
            params: None,
            on_edges: false,
        };
        let p = past(Zero::Past(Side::YLo, line.clone()));
        let exact = past(Zero::Side(Side::YLo, line.clone()));
        let other = past(Zero::Past(Side::YLo, line.clone()));
        let z = Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0));
        let plane = Psi::Linear { n: z, q: origin() };
        let through = |edges: Vec<EdgeKey>, whole: bool| On {
            edges,
            vertices: Vec::new(),
            whole,
        };
        let on = through(vec![edge(1)], true);
        let graph = Psi::Graph {
            m: z,
            t: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            slope: iv0(0.0),
            piece: (0.0, 1.0),
        };
        assert_eq!(
            sign_on(&plane, Some(&on), &p, Some(&exact), band()),
            Some(Sign::Positive),
            "admitted"
        );
        assert_eq!(
            sign_on(&plane, None, &p, None, band()),
            None,
            "the carrier arm"
        );
        assert_eq!(
            sign_on(
                &graph,
                Some(&through(vec![edge(1)], false)),
                &p,
                Some(&exact),
                band()
            ),
            None,
            "the graph separator"
        );
        assert_eq!(
            sign_on(&plane, Some(&on), &p, Some(&other), band()),
            None,
            "against a past side"
        );
        assert_eq!(
            sign_on(
                &plane,
                Some(&through(vec![edge(2)], true)),
                &p,
                Some(&exact),
                band()
            ),
            None,
            "a plane not built through the side's edge"
        );
        let short = past(Zero::Past(
            Side::YLo,
            Along {
                whole: false,
                ..line.clone()
            },
        ));
        assert_eq!(
            sign_on(&plane, Some(&on), &short, Some(&exact), band()),
            None,
            "a side whose chart line is not its edge's own"
        );
        let patch = sheet();
        let free = Presented {
            surface: &patch,
            ..past(Zero::Past(Side::YLo, line))
        };
        assert_eq!(
            sign_on(&plane, Some(&on), &free, Some(&exact), band()),
            None,
            "a free-form carrier"
        );
    }

    /// The plane `z = 0` and the wall `z ≥ 0` meeting it along the line
    /// from the origin in direction `(1, s, 0)`, `s = ±w`, a family whose
    /// shared edge turns with `s`: the floor's half `y ≥ 0` and the wall's
    /// upper half, each presented in a cell about the edge's middle.
    fn turning(w: f64) -> (Surface<Interval>, Surface<Interval>, Boundary, Boundary) {
        let s = Interval::from_bounds(-w, w);
        let d = Vec3::new(iv0(1.0), s, iv0(0.0));
        let line = Curve3::line(origin(), d);
        let face = |sense: bool| Boundary {
            sense,
            edges: vec![Rim {
                key: edge(1),
                arc: Some((line.clone(), (iv0(0.0), iv0(4.0)))),
                ends: [vertex(1), vertex(2)],
                uses: vec![Use {
                    plus: true,
                    image: None,
                    iso: None,
                    wall: None,
                    harmonic: None,
                }],
            }],
            vertices: BTreeMap::from([
                (vertex(1), origin()),
                (vertex(2), line.eval(iv0(4.0))),
                (vertex(9), Point3::new(iv0(2.0), iv0(1.0), iv0(0.0))),
            ]),
            extent: None,
        };
        let floor = Surface::Plane {
            origin: origin(),
            normal: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        };
        let wall = Surface::Plane {
            origin: origin(),
            normal: Vec3::new(Interval::from_f64(0.0) - s, iv0(1.0), iv0(0.0)),
            u_ref: d,
        };
        (floor, wall, face(true), face(false))
    }

    /// **A plane through a line edge that turns with the parameter holds
    /// the edge for every member** ([`planes_through`]): its `n` and `q`
    /// are computed from the edge's stored direction and a point of it,
    /// so for each member direction `d` and point `p` of that member's
    /// line, `n·d` and `n·(p − q)` enclose 0. It lists the edge and its
    /// ends, and certifies the floor against the wall. Red if `n` is
    /// rounded to one direction: it is then normal to the middle member's
    /// edge alone.
    #[test]
    fn a_plane_through_a_turning_line_edge_holds_it_for_every_member() {
        let w = 1e-2;
        let (floor, wall, below, above) = turning(w);
        let local = Local {
            edges: vec![0],
            vertices: Vec::new(),
        };
        let piece = |surface, boundary| Piece {
            face: FaceKey::default(),
            surface,
            whole: false,
            boundary,
            u: (1.9, 2.1),
            v: (-0.1, 0.1),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let (x, y) = (piece(&floor, &below), piece(&wall, &above));
        let (Ok(Presentation::Of(px)), Ok(Presentation::Of(py))) = (
            present_plane(&x, &[0], &[], band()),
            present_plane(&y, &[0], &[], band()),
        ) else {
            panic!("both faces are presented on the edge");
        };
        let near = Near {
            edges: vec![(0, 0)],
            vertices: Vec::new(),
        };
        let planes = planes_through(&x, &near, &px, &py);
        let [(psi @ Psi::Linear { n, q }, on)] = planes.as_slice() else {
            panic!("one plane through the line: {planes:?}");
        };
        assert_eq!(on.edges, [edge(1)], "it lists the edge");
        assert!(
            on.vertices.contains(&vertex(1)) && on.vertices.contains(&vertex(2)),
            "and its ends: {on:?}"
        );
        for k in [-1.0, -0.5, 0.0, 0.5, 1.0] {
            let d = Vec3::new(iv0(1.0), iv0(k * w), iv0(0.0));
            let p = Point3::new(iv0(3.0), iv0(3.0 * k * w), iv0(0.0));
            for (what, v) in [("n·d", n.dot(d)), ("n·(p − q)", n.dot(p - *q))] {
                assert!(
                    v.lo() <= 0.0 && 0.0 <= v.hi(),
                    "the member at s = {}: {what} = {v:?}",
                    k * w
                );
            }
        }
        let (a, b) = (
            sign_on(psi, Some(on), &px, Some(&py), band()),
            sign_on(psi, Some(on), &py, Some(&px), band()),
        );
        assert!(
            matches!((a, b), (Some(a), Some(b)) if a == flip(b)),
            "the plane separates the floor from the wall: {a:?} {b:?}"
        );
    }

    /// **Two conics' stored frames hold one plane only as one value**
    /// ([`one_plane`]): a sphere's two seam meridians, centred alike with
    /// their axes turned over, hold one plane; the pair with one axis a
    /// half turn's rounding off does not, nor with a centre an ulp off,
    /// nor a pair stored in the same brackets. Red if the frames are
    /// compared within the band, or a bracket is read as one value.
    #[test]
    fn two_conics_hold_one_plane_only_as_one_value() {
        let o = Point3::new(iv0(0.0), iv0(1.4), iv0(0.0));
        let down = Vec3::new(iv0(0.0), iv0(0.0), iv0(-1.0));
        let up = Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0));
        assert!(one_plane((o, down), (o, up)), "turned over");
        assert!(one_plane((o, down), (o, down)), "alike");
        let off = Vec3::new(iv0(-1.224_646_799_147_353_2e-16), iv0(0.0), iv0(1.0));
        assert!(!one_plane((o, down), (o, off)), "an axis a rounding off");
        let moved = Point3::new(iv0(0.0), iv0(1.4_f64.next_up()), iv0(0.0));
        assert!(!one_plane((o, down), (moved, up)), "a centre an ulp off");
        let w = Interval::from_bounds(-1e-9, 1e-9);
        let wide = Vec3::new(w, iv0(0.0), iv0(-1.0));
        assert!(!one_plane((o, wide), (o, wide)), "one bracket twice");
    }

    /// **A plane through a line edge answers nothing at a shared vertex
    /// off the line** ([`planes_through`], [`sign_on`]): the pair shares
    /// the edge and the vertex `(2, 1, 0)` apart from it, and the plane,
    /// built through the edge, lists the edge's ends and not that vertex.
    /// A corner point at an end reads the plane's sign; the same corner
    /// at the other vertex answers nothing. Red if the plane is read as
    /// vanishing on every cell.
    #[test]
    fn a_plane_through_a_line_edge_answers_nothing_at_a_vertex_off_it() {
        let (floor, wall, below, above) = turning(0.0);
        let local = Local {
            edges: vec![0],
            vertices: Vec::new(),
        };
        let piece = |surface, boundary| Piece {
            face: FaceKey::default(),
            surface,
            whole: false,
            boundary,
            u: (1.9, 2.1),
            v: (-0.1, 0.1),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let (x, y) = (piece(&floor, &below), piece(&wall, &above));
        let (Ok(Presentation::Of(px)), Ok(Presentation::Of(py))) = (
            present_plane(&x, &[0], &[], band()),
            present_plane(&y, &[0], &[], band()),
        ) else {
            panic!("both faces are presented on the edge");
        };
        let near = Near {
            edges: vec![(0, 0)],
            vertices: vec![vertex(9)],
        };
        let planes = planes_through(&x, &near, &px, &py);
        let [(psi, on)] = planes.as_slice() else {
            panic!("one plane through the line: {planes:?}");
        };
        assert!(
            !on.vertices.contains(&vertex(9)),
            "the vertex off the line is not listed: {on:?}"
        );
        // A rectangle of the floor leaving the point along `-y`, both its
        // axes climbing the plane.
        let at = |v| Presented {
            surface: &floor,
            map: Map::Affine {
                o: [iv0(2.0), iv0(0.0)],
                a: [iv0(0.0), iv0(-1.0)],
                b: [iv0(0.5), iv0(-1.0)],
            },
            x: (0.0, 0.1),
            y: (0.0, 0.1),
            zero: Zero::Point(Side::XLo, Side::YLo, v),
            corner: false,
            cache: Cache::default(),
        };
        assert!(
            sign_on(psi, Some(on), &at(vertex(1)), None, band()).is_some(),
            "at the edge's end, the plane's sign"
        );
        assert_eq!(
            sign_on(psi, Some(on), &at(vertex(9)), None, band()),
            None,
            "at the vertex off the line, nothing"
        );
    }

    // ------------------------------------- a spline side off its iso net

    /// The rational patch `S(u, v) = (a(u), v)`: `a` the quadratic arc on
    /// `(1, 0), (1, 1), (0, 1)` with middle weight `w`, in `z = 0` and
    /// `z = 1`. Its `v = 0` iso lies in the plane `z = 0`.
    fn arc_wall(w: Interval) -> Surface<Interval> {
        use geom_core::spline::{KnotVector, Knots};
        let knots = |p: usize, k: Vec<f64>| {
            Knots::from(KnotVector::clamped(k, p).unwrap()).lift(Interval::from_f64)
        };
        let p = |x: f64, y: f64, z: f64| Point3::new(iv0(x), iv0(y), iv0(z));
        Surface::Nurbs(std::sync::Arc::new(
            geom::NurbsSurface::new(
                knots(2, vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]),
                knots(1, vec![0.0, 0.0, 1.0, 1.0]),
                vec![
                    p(1.0, 0.0, 0.0),
                    p(1.0, 0.0, 1.0),
                    p(1.0, 1.0, 0.0),
                    p(1.0, 1.0, 1.0),
                    p(0.0, 1.0, 0.0),
                    p(0.0, 1.0, 1.0),
                ],
                vec![iv0(1.0), iv0(1.0), w, w, iv0(1.0), iv0(1.0)],
            )
            .unwrap(),
        ))
    }

    /// `ψ = n·(p − q)`.
    fn plane_psi(n: (f64, f64, f64), q: (f64, f64, f64)) -> Psi {
        Psi::Linear {
            n: Vec3::new(iv0(n.0), iv0(n.1), iv0(n.2)),
            q: Point3::new(iv0(q.0), iv0(q.1), iv0(q.2)),
        }
    }

    /// `ψ` over the `v = 0` side of `wall`'s cell `x × (0, 0.5)`.
    fn on_iso(psi: &Psi, wall: &Surface<Interval>, x: (f64, f64)) -> Interval {
        let p = Presented {
            surface: wall,
            map: Map::Chart,
            x,
            y: (0.0, 0.5),
            zero: Zero::Side(Side::YLo, Along::default()),
            corner: false,
            cache: Cache::default(),
        };
        on_side(psi, &p, x, (0.0, 0.0)).expect("a linear separator has a value")
    }

    /// **A spline side reads `ψ` off its own restricted iso net**
    /// ([`on_side`]). The iso lies in `z = 0`, so that plane reads zero
    /// on a wide cell, where the centred form reads the patch's first
    /// order, and [`sign_on`] certifies the cell off that side; a plane the iso crosses between its ends reads mixed; a
    /// plane the cell's piece of the iso lies strictly beside reads that
    /// sign though the whole iso crosses it; and every enclosure holds
    /// the iso's own points inside the cell, a plane through the iso's
    /// end that it only approaches included.
    #[test]
    fn a_spline_side_reads_a_plane_off_its_restricted_iso_net() {
        let wall = arc_wall(iv0(std::f64::consts::FRAC_1_SQRT_2));
        let cell = (0.2, 0.3);
        let flat = plane_psi((0.0, 0.0, 1.0), (0.0, 0.0, 0.0));
        let zero = on_iso(&flat, &wall, cell);
        assert!(
            is_zero(zero, band()),
            "the iso in z = 0 reads zero: {zero:?}"
        );
        let p = Presented {
            surface: &wall,
            map: Map::Chart,
            x: cell,
            y: (0.0, 0.5),
            zero: Zero::Side(Side::YLo, Along::default()),
            corner: false,
            cache: Cache::default(),
        };
        let first_order = centred(&flat, &p, cell, (0.0, 0.0)).unwrap();
        assert!(
            !is_zero(first_order, band()),
            "the centred form does not read the side zero on this cell: {first_order:?}"
        );
        assert_eq!(
            sign_on(&flat, None, &p, None, band()),
            Some(Sign::Positive),
            "the side zero, the wall departs z = 0 upward across it"
        );
        let chord = plane_psi((1.0, 1.0, 0.0), (0.6, 0.6, 0.0));
        let crossed = on_iso(&chord, &wall, (0.0, 1.0));
        assert!(
            crossed.lo() < 0.0 && crossed.hi() > 0.0 && !is_zero(crossed, band()),
            "x + y = 1.2 crosses the iso between its ends, which read -0.2: {crossed:?}"
        );
        assert_eq!(
            interval_sign_as(crossed, SELF_INTERSECTION_DEPARTURE, band()),
            None,
            "mixed"
        );
        let half = plane_psi((1.0, 0.0, 0.0), (0.5, 0.0, 0.0));
        let beside = on_iso(&half, &wall, (0.0, 0.1));
        assert_eq!(
            interval_sign_as(beside, SELF_INTERSECTION_DEPARTURE, band()),
            Some(Sign::Positive),
            "the cell's piece of the iso is beyond x = 0.5, which the whole iso crosses: \
             {beside:?}"
        );
        let below = plane_psi((0.0, 0.0, 1.0), (0.0, 0.0, -0.01));
        assert_eq!(
            interval_sign_as(
                on_iso(&below, &wall, cell),
                SELF_INTERSECTION_DEPARTURE,
                band()
            ),
            Some(Sign::Positive),
            "the iso 0.01 above z = -0.01"
        );
        let Surface::Nurbs(net) = &wall else {
            panic!("a spline wall")
        };
        let grazing = (0.0, 1e-3, 1.0);
        for (n, q, x) in [
            ((0.0, 0.0, 1.0), (0.0, 0.0, 0.0), cell),
            ((1.0, 1.0, 0.0), (0.6, 0.6, 0.0), (0.0, 1.0)),
            ((1.0, 0.0, 0.0), (0.5, 0.0, 0.0), (0.0, 0.1)),
            (grazing, (1.0, 0.0, 0.0), (0.0, 0.1)),
        ] {
            let psi = plane_psi(n, q);
            let v = on_iso(&psi, &wall, x);
            for k in 1..16 {
                let u = x.0 + (x.1 - x.0) * f64::from(k) / 16.0;
                let at = psi.value(net.eval(iv0(u), iv0(0.0))).unwrap();
                assert!(
                    v.lo() <= at.lo() && at.hi() <= v.hi(),
                    "{n:?} through {q:?} on {x:?}: {v:?} holds the iso at u = {u}: {at:?}"
                );
            }
        }
    }

    /// **A weight the net cannot certify positive falls back to the
    /// centred form** ([`on_side`]), which reads nothing on such a net:
    /// the net door refuses it, and so does the centred form's jet.
    /// A weight that is positive but not certified reaches the door as
    /// NaI, since the restriction refuses an uncertified coefficient. A
    /// zero or a negative weight, which [`geom::NurbsSurface::new`]
    /// refuses but `map_scalar` carries, reaches it as itself over the
    /// whole span, where the restriction is the net. The guard is on the
    /// restricted weights, so a cell whose restricted weights are all
    /// positive reads the hull, which is sound there.
    #[test]
    fn a_weight_the_net_cannot_certify_falls_back_off_the_iso_net() {
        let unproven = Interval::from_bounds(-1.0, 1.0).sqrt() * iv0(0.0) + iv0(1.0);
        assert!(
            !unproven.is_certified() && unproven.lo() == 1.0 && unproven.hi() == 1.0,
            "a weight of 1 not certified: {unproven:?}"
        );
        let middle = std::f64::consts::FRAC_1_SQRT_2;
        let lifted = |to: f64| {
            let Surface::Nurbs(net) = arc_wall(iv0(middle)) else {
                panic!("a spline wall")
            };
            let mapped = net.map_scalar(|x| {
                if x.lo() == middle && x.hi() == middle {
                    iv0(to)
                } else {
                    x
                }
            });
            assert!(
                mapped.weights().iter().any(|w| w.lo() == to),
                "the middle weight is {to}"
            );
            Surface::Nurbs(std::sync::Arc::new(mapped))
        };
        let flat = plane_psi((0.0, 0.0, 1.0), (0.0, 0.0, 0.0));
        for (what, wall, cell) in [
            ("not certified", arc_wall(unproven), (0.2, 0.3)),
            ("zero", lifted(0.0), (0.0, 1.0)),
            ("negative", lifted(-0.5), (0.0, 1.0)),
        ] {
            let Surface::Nurbs(net) = &wall else {
                panic!("a spline wall")
            };
            assert!(
                net.rect_net(cell, (0.0, 0.0)).is_none(),
                "{what}: the net door refuses the weight"
            );
            let v = on_iso(&flat, &wall, cell);
            assert!(!is_zero(v, band()), "{what}: nothing is read zero: {v:?}");
        }
    }

    // ------------------------------------------------- the spiric wall

    const OFF: f64 = 0.3;

    fn ez() -> Vec3<Interval> {
        Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0))
    }

    fn ex() -> Vec3<Interval> {
        Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0))
    }

    /// The torus `(major, 0.5)` about `z`, `u_ref = x`.
    fn ring(major: f64) -> Surface<Interval> {
        Surface::Torus {
            center: origin(),
            axis: ez(),
            major_radius: iv0(major),
            minor_radius: iv0(0.5),
            u_ref: ex(),
        }
    }

    /// The point of [`ring`]`(2)` at `(u, v)`.
    fn on_ring(u: f64, v: f64) -> Point3<Interval> {
        let rho = 2.0 + 0.5 * v.cos();
        Point3::new(iv0(rho * u.cos()), iv0(rho * u.sin()), iv0(0.5 * v.sin()))
    }

    /// The spiric the plane `x = 0.3` cuts from [`ring`]`(2)` on its `+y`
    /// side.
    fn spiric() -> Curve3<Interval> {
        Curve3::Spiric {
            center: origin(),
            axis: ez(),
            u_ref: ex(),
            major_radius: iv0(2.0),
            minor_radius: iv0(0.5),
            offset: iv0(OFF),
        }
    }

    /// Its wall image on [`ring`]: `u = atan2(f(t), 0.3)`, `v = t`.
    fn wall() -> Wall {
        Wall {
            u0: iv0(0.0),
            v0: iv0(0.0),
            sigma: 1.0,
            kappa: 1.0,
            major: iv0(2.0),
            minor: iv0(0.5),
            offset: iv0(OFF),
        }
    }

    /// The wall image's azimuth at `t`.
    fn g(t: f64) -> f64 {
        let rho = 2.0 + 0.5 * t.cos();
        (rho * rho - OFF * OFF).sqrt().atan2(OFF)
    }

    /// The spiric over `t`, a wall rim of [`ring`] run forward (`plus`)
    /// or backward, from vertex `ends[0]` at `t.0`.
    fn wall_rim(n: u64, ends: [u64; 2], t: (f64, f64), plus: bool) -> Rim {
        Rim {
            key: edge(n),
            arc: Some((spiric(), (iv0(t.0), iv0(t.1)))),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus,
                image: Some([p2((g(t.0), t.0)), p2((g(t.1), t.1))]),
                iso: None,
                harmonic: None,
                wall: Some(wall()),
            }],
        }
    }

    /// The parallel `v` of [`ring`]`(2)` from azimuth `u.0` (vertex
    /// `ends[0]`) to `u.1`, a coordinate line of its chart.
    fn parallel(n: u64, ends: [u64; 2], v: f64, u: (f64, f64), plus: bool) -> Rim {
        let circle = Curve3::circle(
            Point3::new(iv0(0.0), iv0(0.0), iv0(0.5 * v.sin())),
            ez(),
            iv0(2.0 + 0.5 * v.cos()),
            ex(),
        );
        Rim {
            key: edge(n),
            arc: Some((circle, (iv0(u.0), iv0(u.1)))),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus,
                image: Some([p2((u.0, v)), p2((u.1, v))]),
                iso: Some(Iso {
                    held: Axis::V,
                    at: iv0(v),
                    full: false,
                }),
                harmonic: None,
                wall: None,
            }],
        }
    }

    /// The meridian `u` of [`ring`]`(2)` from `v.0` (vertex `ends[0]`)
    /// to `v.1`, a coordinate line of its chart.
    fn meridian(n: u64, ends: [u64; 2], u: f64, v: (f64, f64)) -> Rim {
        let circle = Curve3::circle(
            Point3::new(iv0(2.0 * u.cos()), iv0(2.0 * u.sin()), iv0(0.0)),
            Vec3::new(iv0(u.sin()), iv0(-u.cos()), iv0(0.0)),
            iv0(0.5),
            Vec3::new(iv0(u.cos()), iv0(u.sin()), iv0(0.0)),
        );
        Rim {
            key: edge(n),
            arc: Some((circle, (iv0(v.0.min(v.1)), iv0(v.0.max(v.1))))),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus: true,
                image: Some([p2((u, v.0.min(v.1))), p2((u, v.0.max(v.1)))]),
                iso: Some(Iso {
                    held: Axis::U,
                    at: iv0(u),
                    full: false,
                }),
                harmonic: None,
                wall: None,
            }],
        }
    }

    /// The torus face of `rims`, its vertices at `points`.
    fn walled(rims: Vec<Rim>, points: &[(u64, Point3<Interval>)], sense: bool) -> Boundary {
        Boundary {
            sense,
            edges: rims,
            vertices: points.iter().map(|&(n, p)| (vertex(n), p)).collect(),
            extent: None,
        }
    }

    /// The face whose boundary is the whole spiric, closed at `t = 0`.
    fn spiric_face(plus: bool, sense: bool) -> Boundary {
        walled(
            vec![wall_rim(1, [1, 1], (0.0, TURN), plus)],
            &[(1, on_ring(g(0.0), 0.0))],
            sense,
        )
    }

    /// [`present`] of `b` on `surface` in the cell `(u, v)`, its local
    /// edges and vertices `local`, those shared `near`.
    fn on_wall<'a>(
        surface: &'a Surface<Interval>,
        b: &'a Boundary,
        local: &'a Local,
        near: (&[usize], &[VertexKey]),
        (u, v): ((f64, f64), (f64, f64)),
    ) -> Result<Presentation<'a>, Unpresented> {
        let piece = Piece {
            face: FaceKey::default(),
            surface,
            whole: false,
            boundary: b,
            u,
            v,
            local,
            ball: Ball::holding(origin(), origin()),
        };
        present(&piece, near.0, near.1, band(), Box::leak(Box::default()))
    }

    fn only_edge() -> Local {
        Local {
            edges: vec![0],
            vertices: Vec::new(),
        }
    }

    /// The side a presentation's curve is zero along, `None` for
    /// anything else.
    fn curve_side(p: Result<Presentation<'_>, Unpresented>) -> Option<Side> {
        match p {
            Ok(Presentation::Of(q)) => match q.zero {
                Zero::Curve(side, _) => Some(side),
                _ => None,
            },
            _ => None,
        }
    }

    /// **A torus face beside a spiric edge is presented by its wall
    /// image, on the face's side of it** ([`present_wall`]'s steps 1 and
    /// 3): beside the curve, and at the closed edge's join, the
    /// rectangle holds the cell's columns and the curve over them, its
    /// curve the zero. The face lies at lower `u`, inside the oval's
    /// azimuth, when the loop runs the edge forward about the outward
    /// normal; run backward, or with the sense flipped, the other side.
    /// Red if the face's side is not read off the loop and the sense,
    /// or if the closed edge is not continued through its join (two
    /// lifts then meet the join's columns).
    #[test]
    fn a_wall_image_presents_the_faces_side_of_its_spiric() {
        let (s, local) = (ring(2.0), only_edge());
        let beside = ((g(1.0) - 0.02, g(1.0) - 0.01), (0.95, 1.05));
        let side = |plus, sense| {
            curve_side(on_wall(
                &s,
                &spiric_face(plus, sense),
                &local,
                (&[0], &[]),
                beside,
            ))
        };
        assert_eq!(
            side(true, true),
            Some(Side::XHi),
            "inside the oval's azimuth"
        );
        assert_eq!(side(false, true), Some(Side::XLo), "the edge run backward");
        assert_eq!(side(true, false), Some(Side::XLo), "the sense flipped");
        let b = spiric_face(true, true);
        let Ok(Presentation::Of(q)) = on_wall(&s, &b, &local, (&[0], &[]), beside) else {
            panic!("presented beside the curve");
        };
        assert!(
            q.x.0 <= beside.0.0 && q.x.1 >= g(0.95).max(g(1.05)) && q.y == beside.1,
            "the rectangle holds the cell and the curve over its columns: {:?} × {:?}",
            q.x,
            q.y
        );
        let join = ((g(0.0) - 0.01, g(0.0) + 0.01), (-0.02, 0.02));
        assert_eq!(
            curve_side(on_wall(&s, &b, &local, (&[0], &[]), join)),
            Some(Side::XHi),
            "through the join"
        );
    }

    /// **An open wall rim is presented only where it runs past both
    /// column ends** ([`present_wall`]'s step 1): over `t ∈ [0.5, 1.5]`,
    /// a cell whose columns hold its end is not presented, one in the
    /// middle is. Red if an end inside the columns is read as the curve
    /// running on, or the open rim as a closed one.
    #[test]
    fn an_open_wall_rim_ending_in_the_columns_is_not_presented() {
        let (s, local) = (ring(2.0), only_edge());
        let b = walled(
            vec![wall_rim(1, [1, 2], (0.5, 1.5), true)],
            &[(1, on_ring(g(0.5), 0.5)), (2, on_ring(g(1.5), 1.5))],
            true,
        );
        let cell = |t: f64| ((g(t) - 0.02, g(t) - 0.01), (t - 0.05, t + 0.05));
        assert!(
            matches!(
                on_wall(&s, &b, &local, (&[0], &[]), cell(1.0)),
                Ok(Presentation::Of(_))
            ),
            "the middle is presented"
        );
        assert!(
            matches!(
                on_wall(&s, &b, &local, (&[0], &[]), cell(1.5)),
                Err(Unpresented::Local)
            ),
            "the end is not"
        );
    }

    /// **A cell a whole turn wide beside a wall is not presented**
    /// ([`present_wall`]'s step 2): `R` would hold a copy of the curve a
    /// turn on. Red if `R`'s width is not checked.
    #[test]
    fn a_cell_a_whole_turn_wide_beside_a_wall_is_not_presented() {
        let (s, local) = (ring(2.0), only_edge());
        assert!(matches!(
            on_wall(
                &s,
                &spiric_face(true, true),
                &local,
                (&[0], &[]),
                ((-3.2, 3.2), (0.95, 1.05))
            ),
            Err(Unpresented::Local)
        ));
    }

    /// **Another edge in reach of a wall's widened rectangle is not
    /// presented past** ([`present_wall`]'s step 2, [`alone`]): a cell
    /// beside the curve, whose own local boundary is the curve alone,
    /// is widened to the curve, over a meridian between the two. Red if
    /// `R`'s reach is not read.
    #[test]
    fn another_edge_in_reach_of_a_walls_rectangle_is_not_presented() {
        let s = ring(2.0);
        let um = g(1.0) - 0.001;
        let meridian = meridian(2, [2, 3], um, (0.5, 1.5));
        let mut b = spiric_face(true, true);
        b.edges.push(meridian);
        b.vertices.insert(vertex(2), on_ring(um, 0.5));
        b.vertices.insert(vertex(3), on_ring(um, 1.5));
        let local = only_edge();
        let cell = ((g(1.0) - 0.004, g(1.0) - 0.003), (0.995, 1.005));
        assert!(matches!(
            on_wall(&s, &b, &local, (&[0], &[]), cell),
            Err(Unpresented::Local)
        ));
        let alone = spiric_face(true, true);
        assert!(
            matches!(
                on_wall(&s, &alone, &local, (&[0], &[]), cell),
                Ok(Presentation::Of(_))
            ),
            "presented without it"
        );
    }

    /// **The wall's closed form encloses the separator along the face's
    /// image of its chart curve** ([`WallImage::psi`]): on a torus whose
    /// radii drift from the carrier's, a wall wound either way on either
    /// torus (each sign of `σ` and of `κ`), its minor angle off a whole
    /// turn (so `sin v0` weighs its height term), against tilted planes, `ψ` over a piece
    /// of parameters meets `ψ` at points of it evaluated through the
    /// surface. Red if any coefficient, the drift's included, is
    /// dropped or mis-signed.
    #[test]
    fn the_walls_closed_form_encloses_the_separator_along_its_image() {
        let s = Surface::Torus {
            center: Point3::new(iv0(0.1), iv0(-0.2), iv0(0.05)),
            axis: Vec3::new(iv0(0.0), iv0(0.6), iv0(0.8)),
            major_radius: iv0(2.03),
            minor_radius: iv0(0.47),
            u_ref: ex(),
        };
        for (sigma, kappa) in [(1.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)] {
            let w = Wall {
                u0: iv0(0.7),
                v0: iv0(TURN + 0.4),
                sigma,
                kappa,
                major: iv0(2.0),
                minor: iv0(0.5),
                offset: iv0(OFF),
            };
            let image = WallImage::of(&s, w).expect("a torus");
            for (n, q) in [
                ((0.6, 0.0, 0.8), (0.3, 0.1, 0.0)),
                ((0.0, 0.0, 1.0), (0.0, 0.0, 0.2)),
                ((0.48, 0.6, -0.64), (-1.0, 0.5, 0.3)),
            ] {
                let n = Vec3::new(iv0(n.0), iv0(n.1), iv0(n.2));
                let q = Point3::new(iv0(q.0), iv0(q.1), iv0(q.2));
                for k in 0..12 {
                    let t0 = -3.0 + 0.5 * f64::from(k);
                    let piece = Interval::from_bounds(t0, t0 + 0.01);
                    let closed = image.psi(&Psi::Linear { n, q }, piece).expect("linear");
                    for t in [t0, t0 + 0.005, t0 + 0.01] {
                        let [u, v] = w.at(iv0(t));
                        let at_t = n.dot(s.eval(u, v) - q);
                        assert!(
                            closed.lo() <= at_t.hi() && at_t.lo() <= closed.hi(),
                            "σ {sigma}, κ {kappa}, ψ at t = {t}: {at_t:?} outside the closed form's \
                         {closed:?}"
                        );
                    }
                }
            }
        }
        let image = WallImage::of(&s, wall()).expect("a torus");
        assert!(
            image
                .psi(
                    &Psi::Carrier(Implicit::Sphere {
                        c: origin(),
                        r: iv0(1.0)
                    }),
                    iv0(0.0)
                )
                .is_none(),
            "a quadric separator is not read along a wall"
        );
    }

    /// The curved side of the wall presentation beside the curve, on
    /// [`ring`]`(major)`.
    fn wall_side(major: f64) -> Box<Curved> {
        let s = ring(major);
        let (b, local) = (spiric_face(true, true), only_edge());
        let cell = ((g(1.0) - 0.02, g(1.0) - 0.01), (0.2, 2.8));
        let Ok(Presentation::Of(q)) = on_wall(&s, &b, &local, (&[0], &[]), cell) else {
            panic!("presented");
        };
        let Zero::Curve(_, c) = q.zero else {
            panic!("across the curve");
        };
        c
    }

    fn plane_at(x: f64) -> Psi {
        Psi::Carrier(Implicit::Plane {
            o: Point3::new(iv0(x), iv0(0.0), iv0(0.0)),
            n: ex(),
        })
    }

    /// **The separator along a wall reads rounding on the spiric's own
    /// plane and its distance off it** ([`sign_on`]'s zero along a
    /// curve, [`WallImage::psi`]), over columns `2.6` rad wide: on the
    /// plane `x = 0.3`, zero; on `x = 0.31`, at least `0.009`; on its own
    /// plane but a torus `0.01` larger than the carrier's, at least the
    /// drift's `0.3·0.01/2.5`; on a plane tilted through the curve's
    /// point at the columns' middle, not zero. Red if the zero check, or
    /// the drift, is lax, or if `ψ` is read at one parameter.
    #[test]
    fn a_separator_along_a_wall_reads_zero_only_on_the_spirics_plane() {
        let on = wall_side(2.0).psi(&plane_at(OFF)).expect("a plane");
        assert!(is_zero(on, band()), "on its plane: {on:?}");
        let off = wall_side(2.0).psi(&plane_at(OFF + 0.01)).expect("a plane");
        assert!(
            !is_zero(off, band()) && off.lo() <= -0.009,
            "0.01 off: {off:?}"
        );
        let drift = wall_side(2.01).psi(&plane_at(OFF)).expect("a plane");
        assert!(
            !is_zero(drift, band()) && drift.hi() >= 0.001,
            "a torus 0.01 larger: {drift:?}"
        );
        let (a, mid) = (0.3f64, on_ring(g(1.5), 1.5));
        let tilted = Psi::Carrier(Implicit::Plane {
            o: mid,
            n: Vec3::new(iv0(a.cos()), iv0(a.sin()), iv0(0.0)),
        });
        let through = wall_side(2.0).psi(&tilted).expect("a plane");
        assert!(
            !is_zero(through, band()),
            "a plane through the curve's middle only, over t in (0.2, 2.8): {through:?}"
        );
        let graph = Psi::Graph {
            m: ex(),
            t: ez(),
            slope: iv0(0.0),
            piece: (0.0, 1.0),
        };
        assert!(wall_side(2.0).psi(&graph).is_none(), "no graph separator");
    }

    /// The cap of [`spiric`]: the plane `x = 0.3`, the rim its one edge.
    fn cap() -> (Surface<Interval>, Boundary) {
        let plane = Surface::Plane {
            origin: Point3::new(iv0(OFF), iv0(0.0), iv0(0.0)),
            normal: ex(),
            u_ref: Vec3::new(iv0(0.0), iv0(1.0), iv0(0.0)),
        };
        let mut b = spiric_face(false, true);
        b.edges[0].uses[0].wall = None;
        (plane, b)
    }

    /// [`certify_pair`] of a cap cell and a torus cell `cell` of
    /// [`spiric_face`], across the rim.
    fn across_the_wall(cell: ((f64, f64), (f64, f64))) -> Outcome {
        let ((plane, cap), (torus, wall_face)) = (cap(), (ring(2.0), spiric_face(true, true)));
        let local = only_edge();
        let near = Near {
            edges: vec![(0, 0)],
            vertices: Vec::new(),
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
            &piece(&plane, &cap, ((0.0, 0.01), (0.0, 0.01)), face(1)),
            &piece(&torus, &wall_face, cell, face(2)),
            &near,
            band(),
        )
    }

    /// **Across a spiric the cap's plane separates the torus where it
    /// departs to one side, and not where it folds back across it**
    /// ([`departure`]'s carrier arm over [`present_wall`]): a cell
    /// beside the curve certifies; a cell reaching round the tube past
    /// `u = 0`, where the torus turns back toward the plane and crosses
    /// it again at `u = −g`, does not — the separator's derivative across
    /// the curve is not one sign over `R`. Red if that sign is read
    /// laxly.
    #[test]
    fn a_torus_folded_back_across_its_caps_plane_is_not_certified() {
        let beside = ((g(1.0) - 0.02, g(1.0) - 0.01), (0.95, 1.05));
        assert!(
            matches!(
                across_the_wall(beside),
                Outcome::Certified { corner: false }
            ),
            "beside the curve"
        );
        let fold = ((-1.6, 1.0), (0.95, 1.05));
        let got = across_the_wall(fold);
        assert!(
            matches!(got, Outcome::Split { .. }),
            "across the fold: {got:?}"
        );
    }

    /// **A side on a torus's meridian reads a plane through it as zero
    /// off the circle's closed form** ([`on_side`], [`iso_circle`]): the
    /// meridian `u = 0.7` of [`ring`]`(2)` lies in the plane through the
    /// axis there, and a quarter-turn side of it reads rounding, where the
    /// centred form's error, the length squared, reads it nonzero. The
    /// face below the side departs that plane to its negative side.
    #[test]
    fn a_side_on_a_meridian_reads_a_plane_through_it_off_its_closed_form() {
        let (u0, sv) = (0.7, (0.0, 0.25));
        let torus = ring(2.0);
        let meridian = Psi::Carrier(Implicit::Plane {
            o: origin(),
            n: Vec3::new(iv0(-u0.sin()), iv0(u0.cos()), iv0(0.0)),
        });
        let p = Presented {
            surface: &torus,
            map: Map::Chart,
            x: (u0 - 0.1, u0),
            y: sv,
            zero: Zero::Side(Side::XHi, Along::default()),
            corner: false,
            cache: Cache::default(),
        };
        let along = on_side(&meridian, &p, (u0, u0), sv).unwrap();
        assert!(is_zero(along, band()), "the meridian reads zero: {along:?}");
        let first_order = centred(&meridian, &p, (u0, u0), sv).unwrap();
        assert!(
            !is_zero(first_order, band()),
            "the centred form does not read the side zero: {first_order:?}"
        );
        assert_eq!(
            sign_on(&meridian, None, &p, None, band()),
            Some(Sign::Negative),
            "the face at u < 0.7 lies on the plane's negative side"
        );
        let tilted = Psi::Carrier(Implicit::Plane {
            o: origin(),
            n: Vec3::new(iv0(-(u0 + 1e-3).sin()), iv0((u0 + 1e-3).cos()), iv0(0.0)),
        });
        let off = on_side(&tilted, &p, (u0, u0), sv).unwrap();
        assert!(
            !is_zero(off, band()),
            "a plane a milliradian off the meridian does not: {off:?}"
        );
    }

    /// **A corner of a wall and a parallel** on [`ring`]`(2)`: the wall
    /// over `t ∈ [1, 2]` from vertex 1, run forward, and the parallel
    /// `v = at` from vertex 1 to azimuth `g(1) + du` (vertex 3), run
    /// forward where `parallel_plus`. The wall puts the face inside the
    /// oval's azimuth; the parallel, run backward from `du < 0`, puts it
    /// above itself.
    fn wall_corner(du: f64, parallel_plus: bool, at: f64) -> Boundary {
        let mut b = walled(
            vec![
                wall_rim(1, [1, 2], (1.0, 2.0), true),
                parallel(2, [1, 3], at, (g(1.0), g(1.0) + du), parallel_plus),
            ],
            &[
                (1, on_ring(g(1.0), 1.0)),
                (2, on_ring(g(2.0), 2.0)),
                (3, on_ring(g(1.0) + du, at)),
            ],
            true,
        );
        b.extent = Some([(g(1.0) - 0.6, g(1.0) + 0.6), (0.5, 2.5)]);
        b
    }

    fn at_the_corner() -> Local {
        Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        }
    }

    /// The corner cell of [`wall_corner`], its `shared` edge near.
    fn corner_cell(b: &Boundary, shared: usize) -> Result<Presentation<'_>, Unpresented> {
        let s = Box::leak(Box::new(ring(2.0)));
        let local = Box::leak(Box::new(at_the_corner()));
        let cell = ((g(1.0) - 0.01, g(1.0) + 0.01), (0.99, 1.01));
        on_wall(s, b, local, (&[shared][..], &[vertex(1)]), cell)
    }

    fn cornered(p: &Result<Presentation<'_>, Unpresented>) -> bool {
        matches!(p, Ok(Presentation::Of(q)) if q.corner)
    }

    /// **A corner of a shared wall is read through its vertex where the
    /// parallel leaves into the face's side** ([`present_wall`] through
    /// a corner, [`leaves_graph`]): leaving to lower `u`, inside the
    /// oval's azimuth, the cell is presented through the corner; leaving
    /// to higher `u` it is a reflex corner, `Structural`. Red if a
    /// decided `Negative` is admitted.
    #[test]
    fn a_corner_of_a_shared_wall_is_read_only_where_the_parallel_leaves_into_the_face() {
        let convex = wall_corner(-0.5, false, 1.0);
        assert!(cornered(&corner_cell(&convex, 0)), "convex");
        let reflex = wall_corner(0.5, true, 1.0);
        assert!(
            matches!(corner_cell(&reflex, 0), Err(Unpresented::Structural)),
            "reflex"
        );
    }

    /// **A parallel off the wall's vertex is not read through it**
    /// ([`on_vertex`]): the parallel `v = 1.01` ends at the wall's
    /// vertex's azimuth, `0.005` m off its point. Red if the two edges'
    /// points at the vertex are not compared.
    #[test]
    fn a_parallel_off_the_walls_vertex_is_not_read_through_it() {
        let off = wall_corner(-0.5, false, 1.01);
        assert!(!cornered(&corner_cell(&off, 0)));
    }

    /// **A corner of a shared parallel on a torus is read by the line
    /// where the wall leaves into the face's side** ([`reading`]'s torus
    /// line, [`leaves_line`]): the face above the parallel, the wall
    /// rising from it, is presented through the corner by the line; the
    /// parallel run the other way puts the face below it, a reflex
    /// corner, `Structural`. Red if a decided `Negative` is admitted, or
    /// if the torus is not read by the line.
    #[test]
    fn a_corner_of_a_shared_parallel_is_read_by_the_line_only_where_the_wall_rises_into_the_face() {
        let convex = wall_corner(-0.5, false, 1.0);
        let got = corner_cell(&convex, 1);
        assert!(cornered(&got), "convex: {:?}", got.as_ref().err());
        let reflex = wall_corner(-0.5, true, 1.0);
        assert!(
            matches!(corner_cell(&reflex, 1), Err(Unpresented::Structural)),
            "reflex"
        );
    }

    /// **A line corner on a torus whose cell stops at the line on its
    /// other side reads vacuous** ([`present_line_corner`]'s sliver):
    /// the cell under the parallel, reaching it, clips to a rectangle
    /// within the band of the line. Red if the sliver is not read there
    /// (the rectangle is then presented with no width across the line,
    /// on which no separator's sign is decided).
    #[test]
    fn a_line_corner_cell_reaching_the_line_from_outside_reads_vacuous() {
        let b = wall_corner(-0.5, false, 1.0);
        let s = ring(2.0);
        let local = at_the_corner();
        let cell = ((g(1.0) - 0.01, g(1.0) + 0.01), (0.98, 1.0));
        assert!(matches!(
            on_wall(&s, &b, &local, (&[1], &[vertex(1)]), cell),
            Ok(Presentation::Vacuous)
        ));
    }

    /// **A wall frame**: the torus, the carrier, its wall image there,
    /// and the chart point of the curve at `t`. `Own` is [`ring`] and
    /// [`spiric`]; `Flipped` the same torus and plane with the carrier's
    /// axis reversed (`σ = κ = −1`, the oval on the `−y` side); `Mirror`
    /// the oval of [`spiric`] on its mirror torus through `x = 0.3`
    /// (`σ = 1`, `κ = −1`).
    #[derive(Clone, Copy, Debug)]
    enum Frame {
        Own,
        Flipped,
        Mirror,
    }

    impl Frame {
        fn surface(self) -> Surface<Interval> {
            let center = match self {
                Self::Mirror => Point3::new(iv0(2.0 * OFF), iv0(0.0), iv0(0.0)),
                _ => origin(),
            };
            Surface::Torus {
                center,
                axis: ez(),
                major_radius: iv0(2.0),
                minor_radius: iv0(0.5),
                u_ref: ex(),
            }
        }

        fn carrier(self) -> Curve3<Interval> {
            let axis = match self {
                Self::Flipped => Vec3::new(iv0(0.0), iv0(0.0), iv0(-1.0)),
                _ => ez(),
            };
            Curve3::Spiric {
                center: origin(),
                axis,
                u_ref: ex(),
                major_radius: iv0(2.0),
                minor_radius: iv0(0.5),
                offset: iv0(OFF),
            }
        }

        fn wall(self) -> Wall {
            let (u0, sigma, kappa) = match self {
                Self::Own => (0.0, 1.0, 1.0),
                Self::Flipped => (0.0, -1.0, -1.0),
                Self::Mirror => (core::f64::consts::PI, 1.0, -1.0),
            };
            Wall {
                u0: iv0(u0),
                sigma,
                kappa,
                ..wall()
            }
        }

        fn chart(self, t: f64) -> (f64, f64) {
            match self {
                Self::Own => (g(t), t),
                Self::Flipped => (-g(t), -t),
                Self::Mirror => (core::f64::consts::PI - g(t), t),
            }
        }

        /// The face bounded by the whole oval, run forward.
        fn face(self) -> Boundary {
            let (u0, v0) = self.chart(0.0);
            let (u1, v1) = self.chart(TURN);
            let p = self.carrier().eval(iv0(0.0));
            Boundary {
                sense: true,
                edges: vec![Rim {
                    key: edge(1),
                    arc: Some((self.carrier(), (iv0(0.0), iv0(TURN)))),
                    ends: [vertex(1), vertex(1)],
                    uses: vec![Use {
                        plus: true,
                        image: Some([p2((u0, v0)), p2((u1, v1))]),
                        iso: None,
                        harmonic: None,
                        wall: Some(self.wall()),
                    }],
                }],
                vertices: BTreeMap::from([(vertex(1), p)]),
                extent: None,
            }
        }

        /// A cell beside the curve at `t = 1`, on the side `side` of it
        /// in `u` (`-1` below).
        fn beside(self, side: f64) -> ((f64, f64), (f64, f64)) {
            let (u, v) = self.chart(1.0);
            let near = (u + side * 0.01, u + side * 0.02);
            (
                (near.0.min(near.1), near.0.max(near.1)),
                (v - 0.05, v + 0.05),
            )
        }
    }

    /// **The face's side of a wall is read for each sign of `σ` and of
    /// `κ`** ([`present_wall`]'s step 3): the face lies left of its
    /// traversal about the outward normal, so where the loop runs the
    /// edge forward and `v` falls with `t` (`σ = −1`), it lies at higher
    /// `u`; where `v` rises, at lower `u`, whatever `κ`. Each face is
    /// presented on its side and certified there against the cap's
    /// plane. Red if the side is read off anything but `σ`, the loop and
    /// the sense, or if the azimuth's sign is mis-read.
    #[test]
    fn a_wall_is_read_on_its_faces_side_for_each_sign_of_its_image() {
        let local = only_edge();
        for (frame, side, toward) in [
            (Frame::Own, Side::XHi, -1.0),
            (Frame::Flipped, Side::XLo, 1.0),
            (Frame::Mirror, Side::XHi, -1.0),
        ] {
            let (s, b) = (frame.surface(), frame.face());
            let got = curve_side(on_wall(&s, &b, &local, (&[0], &[]), frame.beside(toward)));
            assert_eq!(got, Some(side), "{frame:?}");
            let (plane, mut cap) = cap();
            cap.edges[0].arc = Some((frame.carrier(), (iv0(0.0), iv0(TURN))));
            let near = Near {
                edges: vec![(0, 0)],
                vertices: Vec::new(),
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
            let got = certify_pair(
                &piece(&plane, &cap, ((0.0, 0.01), (0.0, 0.01)), face(1)),
                &piece(&s, &b, frame.beside(toward), face(2)),
                &near,
                band(),
            );
            assert!(
                matches!(got, Outcome::Certified { .. }),
                "{frame:?}: the cap's plane separates: {got:?}"
            );
        }
    }

    /// **A meridian corner on a torus is read by neither corner
    /// reading** ([`reading`]): the wall leaving a meridian into the
    /// face's side, with either edge shared, refuses `Structural` — the
    /// readings argue parallels only. Red if a meridian is admitted.
    #[test]
    fn a_meridian_corner_of_a_wall_refuses_typed() {
        let s = ring(2.0);
        let mut b = walled(
            vec![
                wall_rim(1, [1, 2], (1.0, 2.0), true),
                meridian(2, [1, 3], g(1.0), (1.0, 0.5)),
            ],
            &[
                (1, on_ring(g(1.0), 1.0)),
                (2, on_ring(g(2.0), 2.0)),
                (3, on_ring(g(1.0), 0.5)),
            ],
            true,
        );
        b.extent = Some([(g(1.0) - 0.6, g(1.0) + 0.6), (0.4, 2.5)]);
        let local = at_the_corner();
        let cell = ((g(1.0) - 0.01, g(1.0) + 0.01), (0.99, 1.01));
        for shared in [0, 1] {
            let got = on_wall(&s, &b, &local, (&[shared][..], &[vertex(1)]), cell);
            assert!(
                matches!(got, Err(Unpresented::Structural)),
                "shared edge {shared}: {:?}",
                read(got)
            );
        }
    }

    /// **A reflex line corner's thin cell is not read vacuous**
    /// ([`present_line_corner`]): the cell reaching just past the
    /// parallel onto its non-face side clips to a sliver, but the corner
    /// is reflex, the face beyond the vertex is in the cell, and the
    /// reading refuses `Structural` before the sliver is read. Red if the
    /// sliver is read first.
    #[test]
    fn a_reflex_line_corners_thin_cell_is_not_read_vacuous() {
        let b = wall_corner(-0.5, true, 1.0);
        let s = ring(2.0);
        let local = at_the_corner();
        let cell = ((g(1.0) - 0.01, g(1.0) + 0.01), (1.0, 1.02));
        let got = on_wall(&s, &b, &local, (&[1], &[vertex(1)]), cell);
        assert!(
            matches!(got, Err(Unpresented::Structural)),
            "{:?}",
            read(got)
        );
    }

    /// **A wall read through a corner is continued less than a turn**
    /// ([`through_of`]'s cap): the wall over `t ∈ [0, 4]` read through its
    /// vertex at `t = 0` is continued past it by `(τ − 4)/2 ≈ 1.14`, so
    /// columns reaching `2` below the vertex are not presented. This pins
    /// the cap's definition: continued by its own span instead, those
    /// columns are presented, soundly here, since no other lift meets
    /// them. Where one would, the continued span is a turn long, which
    /// step 1's premise excludes; the argument is at [`through_of`].
    #[test]
    fn a_wall_read_through_a_corner_is_continued_less_than_a_turn() {
        let mut b = walled(
            vec![
                wall_rim(1, [1, 2], (0.0, 4.0), true),
                parallel(2, [1, 3], 0.0, (g(0.0), g(0.0) - 0.5), false),
            ],
            &[
                (1, on_ring(g(0.0), 0.0)),
                (2, on_ring(g(4.0), 4.0)),
                (3, on_ring(g(0.0) - 0.5, 0.0)),
            ],
            true,
        );
        b.extent = Some([(g(0.0) - 0.6, g(0.0) + 0.6), (-0.1, 4.1)]);
        let s = ring(2.0);
        let local = at_the_corner();
        let near = |v: (f64, f64)| {
            on_wall(
                &s,
                &b,
                &local,
                (&[0], &[vertex(1)]),
                ((g(0.0) - 0.01, g(0.0) + 0.01), v),
            )
        };
        assert!(cornered(&near((-0.01, 0.01))), "at the vertex");
        assert!(cornered(&near((-1.0, 0.01))), "one below it");
        assert!(!cornered(&near((-2.0, 0.01))), "two below it");
    }

    /// **A shared spiric between two tori refuses at once**
    /// ([`departure`]): the wall image is read against a plane's carrier
    /// only, so no arm certifies across it at any depth. Red if the pair
    /// is split instead.
    #[test]
    fn a_spiric_shared_by_two_tori_refuses_at_once() {
        let (s, b) = (ring(2.0), spiric_face(true, true));
        let twin = spiric_face(false, false);
        let local = only_edge();
        let near = Near {
            edges: vec![(0, 0)],
            vertices: Vec::new(),
        };
        let piece = |boundary, face| Piece {
            face,
            surface: &s,
            whole: false,
            boundary,
            u: (g(1.0) - 0.02, g(1.0) - 0.01),
            v: (0.95, 1.05),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let got = certify_pair(&piece(&b, face(1)), &piece(&twin, face(2)), &near, band());
        assert!(
            matches!(
                got,
                Outcome::Refuse(EngineRefusal::Uncertified {
                    arm: CertificateArm::Departure,
                    ..
                })
            ),
            "{got:?}"
        );
    }

    /// **An iso use's ends that do not certify decide neither its
    /// direction nor its range** ([`forward`], [`along`]). The image runs
    /// `v` from `0` to a refused `1`: certified, it runs forward over
    /// `[0, 1]`. Red if either read takes the refused end's bracket.
    #[test]
    fn an_iso_use_with_a_refused_end_reads_no_direction_or_range() {
        let iso = Iso {
            held: Axis::U,
            at: iv0(1.0),
            full: false,
        };
        let image = |start: Interval, end: Interval| Use {
            plus: true,
            image: Some([Point2::new(iv0(1.0), start), Point2::new(iv0(1.0), end)]),
            iso: Some(iso),
            wall: None,
            harmonic: None,
        };
        let certified = image(iv0(0.0), iv0(1.0));
        assert_eq!(forward(&certified, iso), Some(1.0), "the certified control");
        assert!(along(&certified, iso).is_some(), "the certified control");
        for (what, refused) in [
            ("start", image(trv(0.0), iv0(1.0))),
            ("end", image(iv0(0.0), trv(1.0))),
        ] {
            assert_eq!(
                forward(&refused, iso),
                None,
                "direction off a refused {what}"
            );
            assert!(along(&refused, iso).is_none(), "range off a refused {what}");
        }
    }

    /// **A line that does not certify places nothing** in
    /// [`present_chart`]: [`slot`]'s right line at a refused `1.5`. The
    /// cell beside it on its face side is presented from it when it
    /// certifies; the cell inside the hole, touching the line from its
    /// other side, reads vacuous off it when it certifies. Refused,
    /// neither is decided. Red if any read of a placed line precedes
    /// its ask.
    #[test]
    fn a_refused_line_bounds_no_rectangle() {
        let s = cylinder();
        let mut b = slot();
        let cell = ((1.6, 2.0), (0.3, 0.7));
        assert_eq!(
            chart(&s, &b, &[0, 1], cell.0, cell.1),
            Read::Of((1.5, 2.0), (0.3, 0.7)),
            "the certified control"
        );
        let Some(iso) = b.edges[1].uses[0].iso.as_mut() else {
            panic!("the slot's right edge is an iso");
        };
        iso.at = trv(1.5);
        assert_eq!(chart(&s, &b, &[0, 1], cell.0, cell.1), Read::Local);
        let hole = ((1.1, 1.5), (0.3, 0.7));
        assert_eq!(
            chart(&s, &slot(), &[0, 1], hole.0, hole.1),
            Read::Vacuous,
            "the certified control inside the hole"
        );
        assert_eq!(chart(&s, &b, &[0, 1], hole.0, hole.1), Read::Local);
    }

    /// **A lever that does not certify measures no sliver** (the sliver in
    /// [`present_chart`]): a cell `1e-13` wide beside [`slot`]'s shared
    /// right line on a cylinder whose radius is a refused `1`. Certified,
    /// the strip is within the band and vacuous; refused, the width is
    /// not decided. Red if the width is minted from the lever's upper
    /// end.
    #[test]
    fn a_refused_lever_measures_no_sliver() {
        let b = slot();
        let cell = ((1.5, 1.5 + 1e-13), (0.3, 0.7));
        let on = |radius| {
            let s = Surface::Cylinder {
                origin: origin(),
                axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                radius,
                u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            };
            chart_near(&s, &b, &[0, 1], &[1], cell)
        };
        assert_eq!(on(iv0(1.0)), Read::Vacuous, "the certified control");
        assert_ne!(on(trv(1.0)), Read::Vacuous);
    }

    /// **A separator whose coefficients do not certify bounds nothing
    /// along a curve** ([`Trig::psi`]'s quotient bound, [`Curved::psi`]'s
    /// graph bound): the rim's cylinder with a refused radius, and the
    /// graph separator with a refused gradient. Each reads with the
    /// same values certified. Red if either bound is minted from a
    /// refused one's upper end.
    #[test]
    fn a_refused_separator_bounds_nothing_along_a_curve() {
        let s = floor();
        let x = Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0));
        let (c, _) = rim_cell(&disk(0.7, x, true, true), &s);
        let cylinder = |r| {
            Psi::Carrier(Implicit::Cylinder {
                o: origin(),
                axis: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                r,
            })
        };
        assert!(
            c.psi(&cylinder(iv0(0.7))).is_some(),
            "the certified control"
        );
        assert!(c.psi(&cylinder(trv(0.7))).is_none(), "a refused radius");
    }

    /// **An edge whose chart line does not certify bounds no half
    /// plane** ([`present_plane`]'s one-edge arm): a zero-length edge,
    /// whose `len2` is `[0, 0]`, and an edge whose span ends at a
    /// refused `4`. Neither `y` certifies, so a cell on either side is
    /// not read vacuous off it; the certified edge reads the cell below
    /// it vacuous. Red if the arm reads `y`'s upper end unasked.
    #[test]
    fn an_edge_that_does_not_certify_bounds_no_half_plane() {
        let plane = Surface::Plane {
            origin: origin(),
            normal: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            u_ref: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
        };
        let along = |end: Interval| Boundary {
            sense: true,
            edges: vec![Rim {
                key: edge(1),
                arc: Some((
                    Curve3::line(origin(), Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0))),
                    (iv0(0.0), end),
                )),
                ends: [vertex(1), vertex(2)],
                uses: vec![Use {
                    plus: true,
                    image: None,
                    iso: None,
                    wall: None,
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
        let vacuous = |b: &Boundary, v| {
            let piece = Piece {
                face: FaceKey::default(),
                surface: &plane,
                whole: false,
                boundary: b,
                u: (1.0, 2.0),
                v,
                local: &local,
                ball: Ball::holding(origin(), origin()),
            };
            matches!(
                present_plane(&piece, &[0], &[], band()),
                Ok(Presentation::Vacuous)
            )
        };
        let below = (-0.5, -0.1);
        assert!(vacuous(&along(iv0(4.0)), below), "the certified control");
        for (what, end) in [("zero-length", iv0(0.0)), ("refused", trv(4.0))] {
            for v in [below, (0.1, 0.5)] {
                assert!(
                    !vacuous(&along(end), v),
                    "the {what} edge, the cell at v = {v:?}"
                );
            }
        }
    }

    /// The graph separator through the rim of [`rim_corner`] at
    /// `u = 1`, over `piece`: the face departs from it down the
    /// cylinder.
    fn rim_graph(piece: (f64, f64)) -> (Psi, On) {
        let (s1, c1) = 1.0f64.sin_cos();
        (
            Psi::Graph {
                m: Vec3::new(iv0(0.0), iv0(0.0), iv0(-1.0)),
                t: Vec3::new(iv0(-s1), iv0(c1), iv0(0.0)),
                slope: iv0(0.0),
                piece,
            },
            On {
                edges: vec![edge(10)],
                vertices: Vec::new(),
                whole: false,
            },
        )
    }

    /// **A corner side past its vertex is not zero under the graph
    /// separator over the edge's piece** ([`sign_on`]): the rim of
    /// [`rim_corner`] runs `u ∈ [½, 1]`, and the other edge leaves its
    /// vertex `(1, ½)` down and on toward `u > 1`, so the corner is read
    /// by the line ([`present_line_corner`]) and its zero side runs over
    /// the cell's `u ∈ (0.97, 1.03)`, past the vertex. The graph vanishes
    /// on the circle over its piece alone: over the edge's `(0.94, 1)` it
    /// answers nothing, and over a piece holding the side's parameters
    /// the face's sign. A side of the same rim off the corner, inside the
    /// edge ([`present_chart`]), reads its sign over a piece holding it
    /// and nothing over one that stops short. Under the rim's plane the
    /// corner side is zero, the plane holding the whole circle, and not
    /// where its chart line is taken off the circle. Red if membership
    /// under the graph drops the parameters, reads them off the wrong
    /// end of the rim, or a plane reads a side past its vertex without
    /// its whole line.
    #[test]
    fn a_corner_side_past_its_vertex_is_zero_under_the_graph_only_within_its_piece() {
        let s = cylinder();
        let b = rim_corner(
            harmonic_rim(
                11,
                [1, 3],
                harmonic([1.0, 0.5], 1.0, (0.0, 0.0, -1.0)),
                (0.0, 0.4),
            ),
            (1.4, 0.1),
        );
        let at_corner = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let off_corner = only_edge();
        fn presented<'a>(
            s: &'a Surface<Interval>,
            b: &'a Boundary,
            local: &'a Local,
            u: (f64, f64),
            v: (f64, f64),
        ) -> Box<Presented<'a>> {
            let piece = Piece {
                face: FaceKey::default(),
                surface: s,
                whole: false,
                boundary: b,
                u,
                v,
                local,
                ball: Ball::holding(origin(), origin()),
            };
            match present(
                &piece,
                &[0],
                &local.vertices,
                band(),
                Box::leak(Box::default()),
            ) {
                Ok(Presentation::Of(q)) => q,
                _ => panic!("presented"),
            }
        }
        let sign = |q: &Presented<'_>, piece| {
            let (psi, on) = rim_graph(piece);
            sign_on(&psi, Some(&on), q, None, band())
        };
        let corner = presented(&s, &b, &at_corner, (0.97, 1.03), (0.45, 0.5));
        assert!(
            corner.corner && matches!(corner.zero, Zero::Side(Side::YHi, _)),
            "read through the corner by the rim: {:?}",
            corner.zero
        );
        assert_eq!(sign(&corner, (0.94, 1.0)), None, "past the edge's piece");
        let rim_plane = Psi::Linear {
            n: ez(),
            q: Point3::new(iv0(0.0), iv0(0.0), iv0(0.5)),
        };
        let holds_rim = On {
            edges: vec![edge(10)],
            vertices: Vec::new(),
            whole: true,
        };
        assert!(
            sign_on(&rim_plane, Some(&holds_rim), &corner, None, band()).is_some(),
            "the rim's plane holds the side's whole circle"
        );
        let mut off_line = Presented {
            surface: corner.surface,
            map: corner.map,
            x: corner.x,
            y: corner.y,
            zero: corner.zero.clone(),
            corner: true,
            cache: Cache::default(),
        };
        if let Zero::Side(_, a) = &mut off_line.zero {
            a.whole = false;
        }
        assert_eq!(
            sign_on(&rim_plane, Some(&holds_rim), &off_line, None, band()),
            None,
            "a side past its vertex whose chart line is not the rim's circle"
        );
        assert_eq!(
            sign(&corner, (0.94, 1.04)),
            Some(Sign::Positive),
            "a piece holding the side"
        );
        assert_eq!(
            sign(&corner, (0.94 + TURN, 1.04 + TURN)),
            Some(Sign::Positive),
            "the piece a turn on: the circle's parameters are read modulo turns"
        );
        // The rim stored as an ellipse of equal semi-axes: the same circle,
        // which the side's chart line is not taken to be its own, so no
        // parameters are read and the corner side is not on its edge.
        let mut spline = b.clone();
        spline.edges[0].arc = Some((
            Curve3::Ellipse(geom::Ellipse::new(
                geom::Frame::new(Point3::new(iv0(0.0), iv0(0.0), iv0(0.5)), ez(), ex()),
                iv0(1.0),
                iv0(1.0),
            )),
            (iv0(0.5), iv0(1.0)),
        ));
        let not_whole = presented(&s, &spline, &at_corner, (0.97, 1.03), (0.45, 0.5));
        assert!(
            not_whole.corner
                && matches!(&not_whole.zero, Zero::Side(_, a) if !a.whole && a.params.is_none()),
            "read through the corner, not whole: {:?}",
            not_whole.zero
        );
        assert_eq!(
            sign(&not_whole, (-TURN, TURN)),
            None,
            "a corner side not on its edge, with no parameters, answers nothing"
        );
        let inside = presented(&s, &b, &off_corner, (0.8, 0.9), (0.45, 0.5));
        assert!(
            !inside.corner && matches!(inside.zero, Zero::Side(Side::YHi, _)),
            "read off the corner by the rim: {:?}",
            inside.zero
        );
        assert_eq!(
            sign(&inside, (0.75, 0.95)),
            Some(Sign::Positive),
            "within the piece"
        );
        assert_eq!(sign(&inside, (0.85, 0.95)), None, "short of the side");
    }

    /// **The graph separator's piece holds a whole side's parameters**
    /// ([`graph_of`]): the corner of [`rim_corner`] read by its rim, the
    /// side over `u ∈ (0.97, 1.03)`, past the vertex at `u = 1`, against
    /// a torus whose crest is the rim, the torus leaving it outward. The
    /// side's chart line is the rim's own circle, so the piece reaches
    /// past the edge's span to hold it, and the side reads its sign under
    /// the graph. Red if the piece holds only what the edge reaches.
    #[test]
    fn the_graphs_piece_holds_a_whole_sides_parameters_past_its_edge() {
        let half = core::f64::consts::FRAC_PI_2;
        let rho = 0.05;
        let cyl = cylinder();
        let bx = rim_corner(
            harmonic_rim(
                11,
                [1, 3],
                harmonic([1.0, 0.5], 1.0, (0.0, 0.0, -1.0)),
                (0.0, 0.4),
            ),
            (1.4, 0.1),
        );
        let torus = Surface::Torus {
            center: Point3::new(iv0(0.0), iv0(0.0), iv0(0.5 + rho)),
            axis: ez(),
            major_radius: iv0(1.0),
            minor_radius: iv0(rho),
            u_ref: ex(),
        };
        let on_torus = |u: f64, v: f64| {
            let r = 1.0 + rho * v.cos();
            Point3::new(
                iv0(r * u.cos()),
                iv0(r * u.sin()),
                iv0(0.5 + rho + rho * v.sin()),
            )
        };
        let top = 0.3;
        let mut crest = bx.edges[0].clone();
        crest.uses = vec![Use {
            plus: true,
            image: Some([p2((0.5, -half)), p2((1.0, -half))]),
            iso: Some(Iso {
                held: Axis::V,
                at: iv0(-half),
                full: false,
            }),
            wall: None,
            harmonic: None,
        }];
        let (s1, c1) = 1.0f64.sin_cos();
        let meridian = Rim {
            key: edge(12),
            arc: Some((
                Curve3::circle(
                    Point3::new(iv0(c1), iv0(s1), iv0(0.5 + rho)),
                    Vec3::new(iv0(-s1), iv0(c1), iv0(0.0)),
                    iv0(rho),
                    Vec3::new(iv0(c1), iv0(s1), iv0(0.0)),
                ),
                (iv0(-half), iv0(top)),
            )),
            ends: [vertex(1), vertex(4)],
            uses: vec![Use {
                plus: true,
                image: Some([p2((1.0, -half)), p2((1.0, top))]),
                iso: Some(Iso {
                    held: Axis::U,
                    at: iv0(1.0),
                    full: false,
                }),
                wall: None,
                harmonic: None,
            }],
        };
        let by = Boundary {
            sense: true,
            edges: vec![crest, meridian],
            vertices: BTreeMap::from([
                (vertex(1), on_torus(1.0, -half)),
                (vertex(2), on_torus(0.5, -half)),
                (vertex(4), on_torus(1.0, top)),
            ]),
            extent: Some([(0.5, 1.0), (-half, top)]),
        };
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let near = Near {
            edges: vec![(0, 0)],
            vertices: vec![vertex(1)],
        };
        let piece = |face, surface, boundary, u, v| Piece {
            face,
            surface,
            whole: false,
            boundary,
            u,
            v,
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let x = piece(face(1), &cyl, &bx, (0.97, 1.03), (0.45, 0.5));
        let y = piece(face(2), &torus, &by, (0.94, 1.0), (-half, -half + 0.5));
        let poses = (Poses::default(), Poses::default());
        let (Ok(Presentation::Of(qx)), Ok(Presentation::Of(qy))) = (
            present(&x, &[0], &[vertex(1)], band(), &poses.0),
            present(&y, &[0], &[vertex(1)], band(), &poses.1),
        ) else {
            panic!("both presented");
        };
        assert!(qx.corner, "the cylinder read through its corner");
        let (psi, on) = graph_of(&x, &y, &near, &qx, &qy, band()).expect("a graph");
        let Psi::Graph { piece: held, .. } = psi else {
            panic!("the graph");
        };
        assert!(
            held.0 <= 0.97 && 1.03 <= held.1,
            "the piece holds the side past the vertex: {held:?}"
        );
        assert!(
            sign_on(&psi, Some(&on), &qx, Some(&qy), band()).is_some(),
            "the side reads its sign"
        );
    }

    /// **A chart side run on past its edge's end is not on its edge**
    /// ([`present_chart`], [`run_on`]): a cylinder face below the rim
    /// `v = ½` and left of the ruling `u = 1`, the ruling's line stored
    /// as an enclosure `1e-9` wide, so the rim's side runs on to the
    /// line's far bound, past the rim's end at `u = 1`. With the rim
    /// stored as an ellipse of equal semi-axes, no parameters are read:
    /// the side answers nothing under the graph separator, over any
    /// piece. Red if the run-on counts as the edge's own.
    #[test]
    fn a_chart_side_run_on_past_its_edges_end_answers_nothing_under_the_graph() {
        let s = cylinder();
        let mut b = rim_corner(
            rim(11, [3, 1], Axis::U, 1.0, (1.0, 0.0), (1.0, 0.5)),
            (1.0, 0.0),
        );
        b.edges[0].arc = Some((
            Curve3::Ellipse(geom::Ellipse::new(
                geom::Frame::new(Point3::new(iv0(0.0), iv0(0.0), iv0(0.5)), ez(), ex()),
                iv0(1.0),
                iv0(1.0),
            )),
            (iv0(0.5), iv0(1.0)),
        ));
        let read = |at: Interval| {
            let mut b = b.clone();
            b.edges[1].uses[0].iso = Some(Iso {
                held: Axis::U,
                at,
                full: false,
            });
            let local = Local {
                edges: vec![0, 1],
                vertices: vec![vertex(1)],
            };
            let piece = Piece {
                face: FaceKey::default(),
                surface: &s,
                whole: false,
                boundary: &b,
                u: (0.97, 1.01),
                v: (0.45, 0.51),
                local: &local,
                ball: Ball::holding(origin(), origin()),
            };
            let Ok(Presentation::Of(q)) = present_chart(&piece, &[0], &[vertex(1)], band()) else {
                panic!("presented");
            };
            let Zero::Side(_, a) = &q.zero else {
                panic!("one side: {:?}", q.zero);
            };
            let (psi, on) = rim_graph((-TURN, TURN));
            (a.on_edges, sign_on(&psi, Some(&on), &q, None, band()))
        };
        assert_eq!(
            read(iv0(1.0)),
            (true, Some(Sign::Positive)),
            "the ruling at one value"
        );
        assert_eq!(
            read(Interval::from_bounds(1.0, 1.0 + 1e-9)),
            (false, None),
            "the run-on past the rim's end"
        );
    }

    /// **A side's parameters are read only off an affine moving
    /// channel** ([`iso_params`]): the rim of [`rim_corner`], its use a
    /// harmonic whose held channel is the constant `v = ½`. With no
    /// cosine or sine term on `u` the map is the rim's own, `t = u`; with
    /// one, the two-point fit would be wrong, and no parameters are read.
    /// A side two turns off the image's sheet reads the span's own
    /// parameters (the tour's `diecomposed` meets one). Red if the fit is
    /// taken without the check, or the image is not moved to the side.
    #[test]
    fn a_sides_parameters_are_read_only_off_an_affine_moving_channel() {
        let b = rim_corner(
            rim(11, [3, 1], Axis::U, 1.0, (1.0, 0.0), (1.0, 0.5)),
            (1.0, 0.0),
        );
        let with = |a: f64| {
            let mut r = b.edges[0].clone();
            let mut h = harmonic([1.5, 0.5], -1.0, (0.0, 0.0, 0.0));
            h.pa[0] = iv0(a);
            r.uses[0].harmonic = Some(h);
            iso_params(&cylinder(), &r, (0.8, 0.9))
        };
        let affine = with(0.0).expect("an affine channel");
        assert!(
            (affine.0 - 0.8).abs() < 1e-12 && (affine.1 - 0.9).abs() < 1e-12,
            "the rim's own parameters: {affine:?}"
        );
        // The side two turns off the image's sheet: the image is moved to
        // it, so the parameters are the span's own, which no shift of one
        // turn either way would reach from two off.
        let far = iso_params(
            &cylinder(),
            &b.edges[0],
            (0.8 + 2.0 * TURN, 0.9 + 2.0 * TURN),
        )
        .expect("read");
        assert!(
            (far.0 - 0.8).abs() < 1e-9 && (far.1 - 0.9).abs() < 1e-9,
            "two turns off the image's sheet: {far:?}"
        );
        assert_eq!(with(0.1), None, "a cosine term on the moving channel");
    }

    /// **A plane's side on a line edge carries its parameters**
    /// ([`present_plane`]): the edge from `(0, 0)` to `(1, 0)` on
    /// [`floor`], the face above it, read over `u ∈ (0.3, 0.4)`. Under
    /// the graph separator the side is zero over a piece holding
    /// `(0.3, 0.4)` and nothing over one that stops short. Red if the
    /// segment's parameters are read off the wrong end.
    #[test]
    fn a_plane_side_is_zero_under_the_graph_only_within_its_piece() {
        let s = floor();
        let mut b = Boundary {
            sense: true,
            edges: vec![segment(1, [1, 2], (0.0, 0.0), (1.0, 0.0))],
            vertices: BTreeMap::new(),
            extent: None,
        };
        b.vertices.insert(vertex(1), origin());
        b.vertices
            .insert(vertex(2), Point3::new(iv0(1.0), iv0(0.0), iv0(0.0)));
        let local = only_edge();
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &b,
            u: (0.3, 0.4),
            v: (-0.01, 0.05),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let Ok(Presentation::Of(q)) = present_plane(&piece, &[0], &[], band()) else {
            panic!("presented");
        };
        let graph = |piece| Psi::Graph {
            m: Vec3::new(iv0(0.0), iv0(1.0), iv0(0.0)),
            t: ex(),
            slope: iv0(0.0),
            piece,
        };
        let on = On {
            edges: vec![edge(1)],
            vertices: Vec::new(),
            whole: false,
        };
        assert_eq!(
            sign_on(&graph((0.25, 0.45)), Some(&on), &q, None, band()),
            Some(Sign::Positive),
            "within the piece"
        );
        assert_eq!(
            sign_on(&graph((0.35, 0.45)), Some(&on), &q, None, band()),
            None,
            "short of the side"
        );
        assert_eq!(
            sign_on(&graph((0.55, 0.75)), Some(&on), &q, None, band()),
            None,
            "the segment's other end"
        );
        // A corner of two segments at the origin, the face in the first
        // quadrant: the shared one along `+x` (traversed toward the
        // vertex, its parameter falling to it), the other up `+y`.
        let mut corner = Boundary {
            sense: true,
            edges: vec![
                segment(1, [2, 1], (1.0, 0.0), (0.0, 0.0)),
                segment(2, [1, 3], (0.0, 0.0), (0.0, 1.0)),
            ],
            vertices: BTreeMap::new(),
            extent: None,
        };
        corner.edges[0].uses[0].plus = false;
        corner.edges[1].uses[0].plus = false;
        corner.vertices.insert(vertex(1), origin());
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &corner,
            u: (-0.01, 0.3),
            v: (-0.01, 0.3),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let Ok(Presentation::Of(q)) = present_plane(&piece, &[0], &[vertex(1)], band()) else {
            panic!("the corner presented");
        };
        assert!(
            matches!(q.zero, Zero::Side(..)),
            "one side at the corner: {:?}",
            q.zero
        );
        assert!(
            sign_on(&graph((0.6, 1.0)), Some(&on), &q, None, band()).is_some(),
            "the shared edge's end at the vertex, its parameter 1"
        );
        assert_eq!(
            sign_on(&graph((0.0, 0.4)), Some(&on), &q, None, band()),
            None,
            "the shared edge's far end"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod posed_rows {
    //! A sphere face beside circles off its stored axis, read in the
    //! chart about each circle's own axis ([`present_posed`]).
    use super::*;
    use geom_core::Tol;

    /// The caps' planes `x = A` and `y = A`.
    const A: f64 = 0.25;
    /// Where the cone strip crosses the sphere, `x = B`.
    const B: f64 = 0.3;

    fn iv0(x: f64) -> Interval {
        Interval::from_bounds(x, x)
    }

    fn v3(x: f64, y: f64, z: f64) -> Vec3<Interval> {
        Vec3::new(iv0(x), iv0(y), iv0(z))
    }

    fn pt(x: f64, y: f64, z: f64) -> Point3<Interval> {
        Point3::new(iv0(x), iv0(y), iv0(z))
    }

    fn p2((u, v): (f64, f64)) -> Point2<Interval> {
        Point2::new(iv0(u), iv0(v))
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn vertex(n: u64) -> VertexKey {
        VertexKey::from(slotmap::KeyData::from_ffi(n | (1 << 32)))
    }

    fn edge(n: u64) -> EdgeKey {
        EdgeKey::from(slotmap::KeyData::from_ffi(n | (1 << 32)))
    }

    fn face(n: u64) -> FaceKey {
        FaceKey::from(slotmap::KeyData::from_ffi(n | (1 << 32)))
    }

    /// `√(1 − x²)`: the radius of the unit sphere's circle `x` off its
    /// centre.
    fn across(x: f64) -> f64 {
        (1.0 - x * x).sqrt()
    }

    /// A use whose chart image is no coordinate line, from `a` to `b`.
    fn curve_use(plus: bool, a: (f64, f64), b: (f64, f64)) -> Use {
        Use {
            plus,
            image: Some([p2(a), p2(b)]),
            iso: None,
            harmonic: None,
            wall: None,
        }
    }

    /// An edge of one use on the circle about `centre` along `axis`,
    /// seam `u_ref`, of `radius`, over `span`.
    fn circle_rim(
        n: u64,
        ends: [u64; 2],
        (centre, axis, u_ref, radius): (Point3<Interval>, Vec3<Interval>, Vec3<Interval>, f64),
        span: (f64, f64),
        u: Use,
    ) -> Rim {
        Rim {
            key: edge(n),
            arc: Some((
                Curve3::circle(centre, axis, iv0(radius), u_ref),
                (iv0(span.0), iv0(span.1)),
            )),
            ends: ends.map(vertex),
            uses: vec![u],
        }
    }

    /// The unit sphere about the origin, charted about `z` from `x`.
    fn sphere() -> Surface<Interval> {
        Surface::Sphere {
            center: pt(0.0, 0.0, 0.0),
            radius: iv0(1.0),
            axis: v3(0.0, 0.0, 1.0),
            u_ref: v3(1.0, 0.0, 0.0),
        }
    }

    /// The circle `x = A`, about `x` from `y`: `(A, s·cos t, s·sin t)`.
    fn x_circle() -> (Point3<Interval>, Vec3<Interval>, Vec3<Interval>, f64) {
        (
            pt(A, 0.0, 0.0),
            v3(1.0, 0.0, 0.0),
            v3(0.0, 1.0, 0.0),
            across(A),
        )
    }

    /// **The spherical triangle `x ≥ A`, `y ≥ A`, `z ≥ 0`**, the
    /// two-arc lune's cavity face in miniature: from `E1 = (A, s, 0)` up
    /// the circle `x = A` (edge 1) to `T = (A, A, h)`, down the circle
    /// `y = A` (edge 2) to `E2 = (s, A, 0)`, and back along the equator
    /// (edge 3), counter-clockwise about the outward normal. The stored
    /// chart reads the equator as `v = 0` and neither circle as any
    /// coordinate line.
    fn triangle() -> Boundary {
        let (s, h) = (across(A), (1.0 - 2.0 * A * A).sqrt());
        let lon = |x: f64, y: f64| y.atan2(x);
        let (e1, e2, top) = (lon(A, s), lon(s, A), (h.asin(), lon(A, A)));
        let t_top = h.atan2(A);
        Boundary {
            sense: true,
            edges: vec![
                circle_rim(
                    1,
                    [1, 3],
                    x_circle(),
                    (0.0, t_top),
                    curve_use(true, (e1, 0.0), (top.1, top.0)),
                ),
                circle_rim(
                    2,
                    [3, 2],
                    (pt(0.0, A, 0.0), v3(0.0, 1.0, 0.0), v3(0.0, 0.0, 1.0), s),
                    (A.atan2(h), core::f64::consts::FRAC_PI_2),
                    curve_use(true, (top.1, top.0), (e2, 0.0)),
                ),
                circle_rim(
                    3,
                    [2, 1],
                    (pt(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0), 1.0),
                    (e2, e1),
                    Use {
                        iso: Some(Iso {
                            held: Axis::V,
                            at: iv0(0.0),
                            full: false,
                        }),
                        ..curve_use(true, (e2, 0.0), (e1, 0.0))
                    },
                ),
            ],
            vertices: BTreeMap::from([
                (vertex(1), pt(A, s, 0.0)),
                (vertex(2), pt(s, A, 0.0)),
                (vertex(3), pt(A, A, h)),
            ]),
            extent: Some([(e2, e1), (0.0, top.0)]),
        }
    }

    /// The local boundary of `b` on `surface`'s cell `u × v`, and its ball.
    fn reach(
        surface: &Surface<Interval>,
        b: &Boundary,
        (u, v): ((f64, f64), (f64, f64)),
    ) -> (Local, Ball) {
        let m = |(l, h): (f64, f64)| geom::mid_param(l, h);
        let ball = Ball::holding(surface.point_eval(m(u), m(v)), surface.rect_enclosure(u, v));
        (Local::of(b, ball, band()), ball)
    }

    /// A presentation without its borrow: the zero, the rectangle, and
    /// the pole axis of the sphere it is read on.
    type Posed = (Zero, (f64, f64), (f64, f64), Vec3<f64>);

    /// What presenting `b` in the cell came to, the shared cell `near`,
    /// read through [`present`].
    fn posed(
        surface: &Surface<Interval>,
        b: &Boundary,
        cell: ((f64, f64), (f64, f64)),
        near: (&[usize], &[VertexKey]),
    ) -> Result<Posed, Unpresented> {
        let (local, ball) = reach(surface, b, cell);
        let piece = Piece {
            face: face(1),
            surface,
            whole: false,
            boundary: b,
            u: cell.0,
            v: cell.1,
            local: &local,
            ball,
        };
        let poses = Poses::default();
        match present(&piece, near.0, near.1, band(), &poses)? {
            Presentation::Vacuous => panic!("the cell holds the face"),
            Presentation::Of(q) => {
                let Surface::Sphere { axis, .. } = *q.surface else {
                    panic!("a sphere is presented on a sphere")
                };
                Ok((q.zero.clone(), q.x, q.y, vmid(axis)))
            }
        }
    }

    /// The poses [`poses_of`] finds for the cell `cell` of the face `b` on
    /// `surface`, the shared cell being `near`: how many are placed,
    /// whether one went unplaced, and whether a corner no chart reads
    /// stopped them, the exit [`present_posed`] takes.
    fn exit(
        surface: &Surface<Interval>,
        b: &Boundary,
        cell: ((f64, f64), (f64, f64)),
        near: (&[usize], &[VertexKey]),
    ) -> (usize, bool, bool) {
        let (local, ball) = reach(surface, b, cell);
        let piece = Piece {
            face: face(1),
            surface,
            whole: false,
            boundary: b,
            u: cell.0,
            v: cell.1,
            local: &local,
            ball,
        };
        let set = poses_of(&piece, near.0, near.1, band());
        (set.poses.len(), set.unplaced, set.unread_corner)
    }

    /// **A sphere face beside an off-axis circle reads it as a parallel
    /// of the circle's own pose** ([`present_posed`]). At `E1`, beside
    /// the circle `x = A`, the stored chart has no coordinate line for the
    /// circle and reads `Structural`; the pose about `x` reads the circle
    /// as the parallel `asin A`, the face above it, and the equator as
    /// the meridian `0`, the face past it. Across either, that side is
    /// the zero. Red if a pose reads a line on the wrong side of its face,
    /// misplaces the parallel, reads the equator as no line, or is not
    /// tried.
    #[test]
    fn a_sphere_face_beside_an_off_axis_circle_reads_it_as_a_parallel_of_its_pose() {
        let (s, b) = (sphere(), triangle());
        let at_e1 = ((1.29, 1.34), (-0.03, 0.03));
        let parallel = A.asin();
        match posed(&s, &b, at_e1, (&[0], &[vertex(1)])) {
            Ok((Zero::Side(Side::YLo, along), x, y, axis)) => {
                assert_eq!(along.edges, vec![edge(1)], "the circle is the zero");
                assert!(
                    along.params.is_none(),
                    "a pose's side carries no parameters"
                );
                assert_eq!(axis.to_array(), [1.0, 0.0, 0.0], "the pose about x");
                assert!(
                    y.0 <= parallel && parallel - y.0 < 1e-12,
                    "the side is the parallel asin A: {y:?}"
                );
                assert!(
                    x.0 < 0.0 && 0.0 < x.1,
                    "the rectangle reaches E1's meridian: {x:?}"
                );
            }
            other => panic!("across the circle at E1: {other:?}"),
        }
        match posed(&s, &b, at_e1, (&[2], &[vertex(1)])) {
            Ok((Zero::Side(Side::XLo, along), x, y, _)) => {
                assert_eq!(along.edges, vec![edge(3)], "the equator is the zero");
                assert!(
                    x.0 <= 0.0 && x.0 > -1e-12,
                    "the side is the meridian 0: {x:?}"
                );
                assert!(y.0 <= parallel, "the rectangle reaches the parallel: {y:?}");
            }
            other => panic!("across the equator at E1: {other:?}"),
        }
    }

    /// **A corner no chart reads is `Structural` only where the cell
    /// reaches it** ([`unread_corner`]). At `T` the two circles meet,
    /// each a parallel only of its own pose, and `T` ends the shared
    /// edge: every smaller cell holding `T` holds that corner, so the
    /// cell is `Structural`. A cell holding the whole triangle, across the
    /// equator, reaches `E1` and `E2`, each a corner some pose reads (the
    /// circle's own, where the equator is a meridian), and `T`, which the
    /// shared cell does not hold; a smaller cell parts them, so it is
    /// `Local`. Red if a pose's own failure is reported as `Structural`,
    /// or a corner no chart reads is not.
    #[test]
    fn a_corner_no_chart_reads_is_structural_only_where_the_cell_reaches_it() {
        let (s, b) = (sphere(), triangle());
        assert_eq!(
            posed(&s, &b, ((0.76, 0.81), (1.19, 1.23)), (&[0], &[vertex(3)])).err(),
            Some(Unpresented::Structural),
            "at T"
        );
        assert!(
            exit(&s, &b, ((0.76, 0.81), (1.19, 1.23)), (&[0], &[vertex(3)])).2,
            "at T, by the corner no chart reads"
        );
        assert_eq!(
            posed(
                &s,
                &b,
                ((0.2, 1.4), (-0.05, 1.25)),
                (&[2], &[vertex(1), vertex(2)])
            )
            .err(),
            Some(Unpresented::Local),
            "the whole triangle"
        );
        assert!(
            matches!(
                exit(
                    &s,
                    &b,
                    ((0.2, 1.4), (-0.05, 1.25)),
                    (&[2], &[vertex(1), vertex(2)])
                ),
                (1.., _, false)
            ),
            "the whole triangle, by the poses' own failure"
        );
        assert!(
            !exit(
                &s,
                &b,
                ((0.2, 1.4), (-0.05, 1.25)),
                (&[0], &[vertex(1), vertex(3)])
            )
            .2,
            "a cell touching the pair at two vertices, one of them T: a smaller cell parts them"
        );
    }

    /// **The cap `x ≥ A`**: the one closed circle `x = A`, its vertex at
    /// `E1`, about the pole `(1, 0, 0)` of its own pose.
    fn cap() -> Boundary {
        let e1 = across(A).atan2(A);
        Boundary {
            sense: true,
            edges: vec![circle_rim(
                1,
                [1, 1],
                x_circle(),
                (0.0, TURN),
                curve_use(true, (e1, 0.0), (e1, 0.0)),
            )],
            vertices: BTreeMap::from([(vertex(1), pt(A, across(A), 0.0))]),
            extent: None,
        }
    }

    /// **A cell holding its pose's pole is not placed in the pose**
    /// ([`posed_cell`]): the cap `x ≥ A` holds the pose's pole `(1, 0,
    /// 0)`, where every azimuth of the pose meets. A cell holding the
    /// pole and reaching the circle is `Local`; one beside the circle,
    /// clear of the pole, reads the circle — closed, a whole turn — as
    /// the parallel. Red if a rectangle is placed about the pole.
    #[test]
    fn a_cell_holding_its_poses_pole_is_not_placed() {
        let (s, b) = (sphere(), cap());
        assert_eq!(
            posed(&s, &b, ((-1.35, 1.35), (-0.1, 0.1)), (&[0], &[])).err(),
            Some(Unpresented::Local),
            "the cell holds the pole"
        );
        let top = across(A).asin();
        match posed(
            &s,
            &b,
            ((-0.05, 0.05), (top - 0.03, top + 0.03)),
            (&[0], &[]),
        ) {
            Ok((Zero::Side(Side::YLo, along), _, y, axis)) => {
                assert_eq!(along.edges, vec![edge(1)]);
                assert_eq!(axis.to_array(), [1.0, 0.0, 0.0], "the pose about x");
                assert!(
                    y.0 <= A.asin() && A.asin() - y.0 < 1e-12,
                    "the parallel: {y:?}"
                );
            }
            other => panic!("beside the circle: {other:?}"),
        }
    }

    /// **The cone through the circles `x = A` and `x = B`**: its apex on
    /// the `x` axis at `c`, opening toward `−x` at `α`; `(c, α)`.
    fn cone_apex() -> (f64, f64) {
        let slope = (across(B) - across(A)) / (B - A);
        (A - across(A) / slope, (-slope).atan())
    }

    /// **A cone strip on the triangle's edge 1**: the cone through the
    /// circles `x = A` and `x = B`, over the azimuths of edge 1 and from
    /// `x = A` to `x = 0.5`, so it leaves the sphere at the circle and
    /// crosses it again at `x = B`. Its chart is the cone's about `−x`
    /// from `y`, the azimuth `u = −t` of edge 1's parameter `t`, the
    /// slant `v` from the apex.
    fn strip() -> (Surface<Interval>, Boundary) {
        let (c, alpha) = cone_apex();
        let slant = |x: f64| (c - x) / alpha.cos();
        let (near, far) = (slant(A), slant(0.5));
        let t_top = (1.0 - 2.0 * A * A).sqrt().atan2(A);
        let s = Surface::Cone {
            apex: pt(c, 0.0, 0.0),
            axis: v3(-1.0, 0.0, 0.0),
            half_angle: iv0(alpha),
            u_ref: v3(0.0, 1.0, 0.0),
        };
        let on = |u: f64, v: f64| {
            let r = v * alpha.sin();
            pt(c - v * alpha.cos(), r * u.cos(), -r * u.sin())
        };
        let line = |n: u64, ends: [u64; 2], u: f64, (v0, v1): (f64, f64)| {
            let (p, q) = (vmid_point(on(u, v0)), vmid_point(on(u, v1)));
            Rim {
                key: edge(n),
                arc: Some((Curve3::line(lift3(p), lift(q - p)), (iv0(0.0), iv0(1.0)))),
                ends: ends.map(vertex),
                uses: vec![Use {
                    iso: Some(Iso {
                        held: Axis::U,
                        at: iv0(u),
                        full: false,
                    }),
                    ..curve_use(true, (u, v0), (u, v1))
                }],
            }
        };
        let iso_v = |u: Use, at: f64| Use {
            iso: Some(Iso {
                held: Axis::V,
                at: iv0(at),
                full: false,
            }),
            ..u
        };
        let shared = circle_rim(
            1,
            [1, 3],
            x_circle(),
            (0.0, t_top),
            iso_v(curve_use(true, (0.0, near), (-t_top, near)), near),
        );
        let rim = far * alpha.sin();
        let outer = circle_rim(
            4,
            [5, 6],
            (
                pt(0.5, 0.0, 0.0),
                v3(-1.0, 0.0, 0.0),
                v3(0.0, 1.0, 0.0),
                rim,
            ),
            (-t_top, 0.0),
            iso_v(curve_use(true, (-t_top, far), (0.0, far)), far),
        );
        let b = Boundary {
            sense: true,
            edges: vec![
                outer,
                line(5, [6, 1], 0.0, (far, near)),
                shared,
                line(6, [3, 5], -t_top, (near, far)),
            ],
            vertices: BTreeMap::from([
                (vertex(1), on(0.0, near)),
                (vertex(3), on(-t_top, near)),
                (vertex(5), on(-t_top, far)),
                (vertex(6), on(0.0, far)),
            ]),
            extent: Some([(-t_top, 0.0), (far, near)]),
        };
        (s, b)
    }

    fn vmid_point(p: Point3<Interval>) -> Point3<f64> {
        pmid(p)
    }

    fn lift3(p: Point3<f64>) -> Point3<Interval> {
        Point3::new(iv0(p.x), iv0(p.y), iv0(p.z))
    }

    /// **A pair crossing past a posed circle is not certified**: the
    /// triangle against the cone strip on its edge 1, which crosses the
    /// sphere at `x = B`, 0.05 past the circle. The triangle is presented
    /// in its pose about `x`; cells reaching the crossing are not
    /// certified, while cells short of it are. Red if a pose reads the
    /// face on the wrong side of its circle, or presents a rectangle that
    /// misses part of the face's cell.
    #[test]
    fn a_cone_crossing_a_posed_sphere_face_past_its_circle_is_not_certified() {
        let (sf, bf) = (sphere(), triangle());
        let (sg, bg) = strip();
        let shared = Shared::of(&bf, &bg, false);
        let (c, alpha) = cone_apex();
        let slant = |x: f64| (c - x) / alpha.cos();
        let outcome = |fu: (f64, f64), fv: (f64, f64), gu: (f64, f64), gv: (f64, f64)| {
            let (lf, ballf) = reach(&sf, &bf, (fu, fv));
            let (lg, ballg) = reach(&sg, &bg, (gu, gv));
            let piece = |surface, boundary, u, v, local, ball| Piece {
                face: face(1),
                surface,
                whole: false,
                boundary,
                u,
                v,
                local,
                ball,
            };
            let (x, y) = (
                piece(&sf, &bf, fu, fv, &lf, ballf),
                Piece {
                    face: face(2),
                    ..piece(&sg, &bg, gu, gv, &lg, ballg)
                },
            );
            let near = Near::of(&shared, &lf, &lg);
            assert!(!near.is_empty(), "the cells touch across edge 1");
            certify_pair(&x, &y, &near, band())
        };
        // About edge 1's point at t = 0.6548: the triangle's cell reaches
        // x = B at its azimuth's low end, the strip's its slant's low end.
        let (fu, fv) = ((1.15, 1.28), (0.6, 0.66));
        let gu = (-0.70, -0.61);
        assert!(
            posed(&sf, &bf, (fu, fv), (&[0], &[])).is_ok(),
            "the triangle is presented in its pose there"
        );
        let crossing = outcome(fu, fv, gu, (slant(B) - 0.02, slant(A) + 0.02));
        assert!(
            !matches!(crossing, Outcome::Certified { .. }),
            "cells reaching the crossing: {crossing:?}"
        );
        let short = outcome(
            (1.24, 1.27),
            (0.625, 0.637),
            (-0.66, -0.65),
            (slant(A) - 0.005, slant(A) + 0.005),
        );
        assert!(
            matches!(short, Outcome::Certified { .. }),
            "cells short of the crossing: {short:?}"
        );
    }
    /// **A pose is not tried where a shared edge is no conic**
    /// ([`poses_of`]): the triangle with its edge 2 stored on a line, a
    /// carrier no pose projects to a coordinate line, shared, beside the
    /// circle `x = A`, which a pose would read. The shared edge is unread
    /// in every pose and every smaller cell of the pair holds it, so the
    /// stored chart's `Structural` stands. Red if a pose whose shared edge
    /// is no line is tried: its failure reads `Local`.
    #[test]
    fn a_pose_is_not_tried_where_a_shared_edge_is_no_conic() {
        let s = sphere();
        let mut b = triangle();
        let (p, q) = (b.vertices[&vertex(3)], b.vertices[&vertex(2)]);
        b.edges[1].arc = Some((Curve3::line(p, q - p), (iv0(0.0), iv0(1.0))));
        // The shared edge first: [`beside_curve`] types a cell by the
        // first unread curved edge in its local boundary.
        let local = Local {
            edges: vec![1, 0],
            vertices: Vec::new(),
        };
        let piece = Piece {
            face: face(1),
            surface: &s,
            whole: false,
            boundary: &b,
            u: (0.7, 0.9),
            v: (0.9, 1.1),
            local: &local,
            ball: Ball::holding(pt(0.0, 0.0, 0.0), pt(0.0, 0.0, 0.0)),
        };
        let poses = Poses::default();
        assert_eq!(
            present(&piece, &[1], &[], band(), &poses).err(),
            Some(Unpresented::Structural),
            "the shared line is unread in every pose"
        );
        let set = poses_of(&piece, &[1], &[], band());
        assert!(
            set.poses.is_empty() && !set.unplaced && !set.unread_corner,
            "by the pose left out, not a corner or a placement"
        );
    }

    /// The triangle with its circle `x = A` split into two arcs at `S`,
    /// the circle's parameter `tm` (`vertex(4)`): edge 0 runs `E1 → S`,
    /// edge 3 `S → T`.
    fn split_triangle(tm: f64) -> (Boundary, (f64, f64)) {
        let mut b = triangle();
        let (c, (t0, t1)) = b.edges[0].arc.clone().expect("the circle");
        let sp = c.eval(iv0(tm));
        let lonlat = |p: Point3<Interval>| (mid(p.y).atan2(mid(p.x)), mid(p.z).asin());
        let (e1, top) = (
            lonlat(b.vertices[&vertex(1)]),
            lonlat(b.vertices[&vertex(3)]),
        );
        let ps = lonlat(sp);
        b.edges[0] = Rim {
            key: edge(1),
            arc: Some((c.clone(), (t0, iv0(tm)))),
            ends: [vertex(1), vertex(4)],
            uses: vec![curve_use(true, e1, ps)],
        };
        b.edges.push(Rim {
            key: edge(4),
            arc: Some((c, (iv0(tm), t1))),
            ends: [vertex(4), vertex(3)],
            uses: vec![curve_use(true, ps, top)],
        });
        b.vertices.insert(vertex(4), sp);
        (b, ps)
    }

    /// **A pose reads a side that runs on past the shared cell**
    /// ([`partial_zero`]): the circle `x = A` split into two arcs at `S`,
    /// the pair sharing the arc `E1 → S` and the vertex `S` alone. In the
    /// pose about `x` both arcs lie on the one parallel, so its clip holds
    /// an edge the other face does not share, and the zero is that side,
    /// run on past the shared cell (`Zero::Past`). Red if a pose does not
    /// reach it, or reads the side as the shared edge's alone.
    #[test]
    fn a_pose_reads_a_side_run_on_past_the_shared_cell() {
        let s = sphere();
        let (b, (lon, lat)) = split_triangle(0.4);
        let d = 0.02;
        let got = posed(
            &s,
            &b,
            ((lon - d, lon + d), (lat - d, lat + d)),
            (&[0], &[vertex(4)]),
        );
        assert!(
            matches!(got, Ok((Zero::Past(..), _, _, axis)) if (axis.x, axis.y, axis.z) == (1.0, 0.0, 0.0)),
            "the parallel through S of the pose about x, run on past the shared arc: {got:?}"
        );
    }

    /// **A pose reads a corner zero** ([`present_chart_in`]): at `E1`
    /// with both the circle and the equator shared, [`zero_of`] finds the
    /// corner of the two sides, the parallel `asin A` and the meridian
    /// `0` of the pose about `x`, and the arms read the separator as
    /// vanishing on both. Red if a pose refuses the corner, or
    /// reads either side as the zero alone.
    #[test]
    fn a_pose_reads_a_corner_zero() {
        let (s, b) = (sphere(), triangle());
        let got = posed(
            &s,
            &b,
            ((1.29, 1.34), (-0.03, 0.03)),
            (&[0, 2], &[vertex(1)]),
        );
        assert!(
            matches!(got, Ok((Zero::Corner(..), ..))),
            "the corner of the circle and the equator, both shared: {got:?}"
        );
    }

    /// **A pose reads no sliver** ([`present_chart_in`]): a cell 2e-11
    /// across about a point of the circle `x = A`, whose pose rectangle,
    /// clipped to the parallel, is within the band of it. The stored
    /// chart's sliver would read it vacuous; the sliver is frozen, so a
    /// pose presents it. Red if a pose reads the sliver.
    #[test]
    fn a_pose_reads_no_sliver() {
        let (s, b) = (sphere(), triangle());
        let t = 0.6548_f64;
        let p = (A, across(A) * t.cos(), across(A) * t.sin());
        let (lon, lat) = (p.1.atan2(p.0), p.2.asin());
        let d = 1e-11;
        let got = posed(
            &s,
            &b,
            ((lon - d, lon + d), (lat - d, lat + d)),
            (&[0], &[]),
        );
        assert!(
            matches!(got, Ok((Zero::Side(Side::YLo, _), ..))),
            "the strip beside the circle is presented: {got:?}"
        );
    }

    /// `w` scaled onto the unit sphere.
    fn on_sphere(w: [f64; 3]) -> Point3<Interval> {
        let n = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
        pt(w[0] / n, w[1] / n, w[2] / n)
    }

    /// The extents the triangle's two poses grant, read at `p`.
    fn holding(b: &Boundary, p: Point3<Interval>) -> Vec<Option<bool>> {
        posed_extents_holding(&sphere(), b, p)
    }

    /// **A pose's extent holds its face** ([`posed_extent`]): the
    /// triangle `x ≥ A`, `y ≥ A`, `z ≥ 0` is granted an extent in both its
    /// circles' poses, and each holds the face's points: 64 along each
    /// edge's carrier, its corners, and positive mixtures of its corners
    /// (the face is convex on the sphere). Red if either pose grants
    /// none, or a hull leaves out a point of the face.
    #[test]
    fn a_poses_extent_holds_its_face() {
        let b = triangle();
        let mut points: Vec<Point3<Interval>> = b.vertices.values().copied().collect();
        for e in &b.edges {
            let (c, (t0, t1)) = e.arc.as_ref().expect("a carrier");
            for t in pieces_of((t0.lo(), t1.hi())) {
                points.push(c.eval(at(mid(t))));
            }
        }
        let corners: Vec<[f64; 3]> = b
            .vertices
            .values()
            .map(|p| [mid(p.x), mid(p.y), mid(p.z)])
            .collect();
        for (i, j, k) in [
            (1.0, 1.0, 1.0),
            (5.0, 1.0, 1.0),
            (1.0, 5.0, 1.0),
            (1.0, 1.0, 5.0),
            (8.0, 8.0, 1.0),
        ] {
            let w = [0, 1, 2].map(|d| i * corners[0][d] + j * corners[1][d] + k * corners[2][d]);
            points.push(on_sphere(w));
        }
        for p in points {
            assert_eq!(
                holding(&b, p),
                vec![Some(true), Some(true)],
                "both poses grant an extent holding {p:?}"
            );
        }
    }

    /// **A face whose bounded side in a pose is not the face is granted
    /// no extent** ([`posed_extent`]): the triangle's complement, its
    /// loops the same, its sense flipped, so the face is the rest of the
    /// sphere and holds both poses' poles. The lift closes in each pose,
    /// unwrapped, and winds the wrong way for the face. Red if the
    /// orientation goes unread.
    #[test]
    fn a_face_whose_bounded_side_in_a_pose_is_not_it_is_granted_no_extent() {
        let b = Boundary {
            sense: false,
            ..triangle()
        };
        assert_eq!(holding(&b, on_sphere([-1.0, 0.0, 0.0])), vec![None, None]);
    }

    /// **A face with an edge of no enclosure is granted no extent**
    /// ([`posed_extent`]): the triangle with its equator's carrier gone.
    /// Red if a row with no enclosure is left out of the hull.
    #[test]
    fn a_face_with_an_edge_of_no_enclosure_is_granted_no_extent() {
        let mut b = triangle();
        b.edges[2].arc = None;
        assert_eq!(holding(&b, on_sphere([1.0, 1.0, 0.5])), vec![None, None]);
    }

    /// The cap `x ≥ A` (`sense`) or `x ≤ A` (`!sense`): one face of the
    /// circle `x = A` alone, a whole turn from the vertex `(A, s, 0)`.
    fn x_cap(sense: bool) -> Boundary {
        let s = across(A);
        let e = s.atan2(A);
        Boundary {
            sense,
            edges: vec![circle_rim(
                1,
                [1, 1],
                x_circle(),
                (0.0, TURN),
                curve_use(true, (e, 0.0), (e, 0.0)),
            )],
            vertices: BTreeMap::from([(vertex(1), pt(A, s, 0.0))]),
            extent: None,
        }
    }

    /// **A cap holding its pose's pole is granted no extent**
    /// ([`posed_extent`]): the cap `x ≥ A`, and the larger cap `x ≤ A`,
    /// each holding a pole of the pose about `x`. Its circle lifts to a
    /// whole parallel, which wraps the azimuth: unwrapped, the lift does
    /// not close. Red if the wrap goes unchecked.
    #[test]
    fn a_cap_holding_its_poses_pole_is_granted_no_extent() {
        for sense in [true, false] {
            let p = on_sphere([if sense { 1.0 } else { -1.0 }, 0.1, 0.1]);
            assert_eq!(
                holding(&x_cap(sense), p),
                vec![None],
                "the cap, sense {sense}"
            );
        }
    }

    /// The face `x ≥ A`, `y ≤ B` of the unit sphere, its loop the long arc
    /// of the circle `x = A` through `y < B` and the arc of `y = B`
    /// through `x > A`, walked one way (`forward`) or the other: one
    /// direction is the face, the other its complement.
    fn x_and_y_cut(forward: bool) -> Boundary {
        let b = 0.5_f64;
        let s = across(A);
        let (r, z0) = ((1.0 - b * b).sqrt(), (1.0 - A * A - b * b).sqrt());
        let tx = (b / s).acos();
        let (t0, t1) = (A.atan2(z0), A.atan2(-z0));
        let flat = curve_use(forward, (0.0, 0.0), (0.0, 0.0));
        Boundary {
            sense: true,
            edges: vec![
                circle_rim(1, [1, 2], x_circle(), (tx, TURN - tx), flat.clone()),
                circle_rim(
                    2,
                    [1, 2],
                    (pt(0.0, b, 0.0), v3(0.0, 1.0, 0.0), v3(0.0, 0.0, 1.0), r),
                    (t0, t1),
                    curve_use(!forward, (0.0, 0.0), (0.0, 0.0)),
                ),
            ],
            vertices: BTreeMap::from([(vertex(1), pt(A, b, z0)), (vertex(2), pt(A, b, -z0))]),
            extent: None,
        }
    }

    /// **A loop round its pose's pole is granted no extent in that pose,
    /// whatever its lifted area** ([`posed_extent`]): the face `x ≥ A`,
    /// `y ≤ B` holds the pole `(1, 0, 0)` of the pose about `x`, its loop
    /// winding once round it at varying latitude, so the forced-closed
    /// polygon's area is no rounding, and in the pose about `y` it holds
    /// neither pole. Walked as the face, only the pose about `y` grants an
    /// extent, holding a point near `(1, 0, 0)`; walked as the complement,
    /// which holds a pole of each, neither does. Red if the wrap goes
    /// unchecked: the pose about `x` then grants a turn-wide hull.
    #[test]
    fn a_loop_round_its_poses_pole_is_granted_no_extent_there() {
        let p = on_sphere([0.95, 0.1, 0.1]);
        let mut got = [true, false].map(|f| holding(&x_and_y_cut(f), p));
        got.sort();
        assert_eq!(got, [vec![None, None], vec![None, Some(true)]]);
    }

    /// **A cell outside the face beside its pose's parallel misses the
    /// posed extent** ([`posed_extent`], [`sheets`]): a cell of the
    /// triangle's stored chart at latitude 0.5, its longitude 5e-4 to
    /// 1.5e-3 past the circle `x = A`, the circle in its reach, so outside the face (`x < A`) though
    /// inside its stored extent, and clear of the circle by more than the
    /// touch. The stored chart reads the circle as no line; in the pose
    /// about `x` the cell lies below the parallel `asin A`, which the
    /// face's posed extent starts at, so it holds none of the face
    /// (`Vacuous`). Red if a missed posed extent is not read.
    #[test]
    fn a_cell_outside_the_face_beside_its_poses_parallel_misses_the_posed_extent() {
        let (s, b) = (sphere(), triangle());
        let phi: f64 = 0.5;
        let lc = (A / phi.cos()).acos();
        let cell = ((lc + 5e-4, lc + 1.5e-3), (phi - 5e-4, phi + 5e-4));
        let (_, ball) = reach(&s, &b, cell);
        // The circle in the stored cell's reach, as a cell beside it holds
        // it; the pose reads its own rectangle's reach ([`poses_of`]).
        let local = Local {
            edges: vec![0],
            vertices: Vec::new(),
        };
        let piece = Piece {
            face: face(1),
            surface: &s,
            whole: false,
            boundary: &b,
            u: cell.0,
            v: cell.1,
            local: &local,
            ball,
        };
        let poses = Poses::default();
        let got = present(&piece, &[0], &[], band(), &poses);
        assert!(
            matches!(got, Ok(Presentation::Vacuous)),
            "outside the face, beside the parallel: {:?} (local {:?})",
            got.as_ref().err(),
            local.edges
        );
    }

    /// **A face with its pose's pole on its boundary is granted no
    /// extent** ([`posed_extent`]): the half cap `x ≥ A`, `z ≥ 0`, its
    /// equator arc running through the pose's pole `(1, 0, 0)` mid-edge.
    /// The piece holding the pole is placed at no azimuth
    /// ([`posed_cell`]), and either side of it the lift is half a turn
    /// apart, so the lift is not read closing. Red if the pose grants it
    /// an extent.
    #[test]
    fn a_face_with_its_poses_pole_on_its_boundary_is_granted_no_extent() {
        let s = across(A);
        let e = s.atan2(A);
        let b = Boundary {
            sense: true,
            edges: vec![
                circle_rim(
                    1,
                    [1, 2],
                    x_circle(),
                    (0.0, core::f64::consts::PI),
                    curve_use(true, (e, 0.0), (-e, 0.0)),
                ),
                circle_rim(
                    2,
                    [2, 1],
                    (pt(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0), 1.0),
                    (-e, e),
                    Use {
                        iso: Some(Iso {
                            held: Axis::V,
                            at: iv0(0.0),
                            full: false,
                        }),
                        ..curve_use(true, (-e, 0.0), (e, 0.0))
                    },
                ),
            ],
            vertices: BTreeMap::from([(vertex(1), pt(A, s, 0.0)), (vertex(2), pt(A, -s, 0.0))]),
            extent: None,
        };
        assert_eq!(holding(&b, on_sphere([1.0, 0.0, 0.5])), vec![None]);
    }

    /// **A pose's reach is its own rectangle's** ([`poses_of`]): a short
    /// radial segment stands on the sphere at a corner of the pose's
    /// rectangle for a cell beside the circle `x = A`, clear of the stored
    /// cell's ball but in the rectangle. The
    /// pose's presentation would hold it, so the cell is `Local`. Red if a
    /// pose reads the stored cell's local boundary in its wider rectangle.
    #[test]
    fn a_poses_reach_is_its_own_rectangles() {
        let (s, mut b) = (sphere(), triangle());
        let cell = ((1.0, 1.34), (0.5, 0.6));
        let (Ok((_, x, y, _)), (_, stored)) =
            (posed(&s, &b, cell, (&[0], &[])), reach(&s, &b, cell))
        else {
            panic!("the cell beside the circle is presented");
        };
        let frame = pose_frame(pt(0.0, 0.0, 0.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0))
            .expect("the pose about x");
        let pose = posed_sphere(&frame, iv0(1.0));
        let corner = [(x.0, y.1), (x.1, y.1), (x.0, y.0), (x.1, y.0)]
            .into_iter()
            .map(|(u, v)| pmid(pose.point_eval(u, v)))
            .find(|&c| stored.clear_of_point(lift3(c), band()))
            .expect("a corner of the pose's rectangle off the stored cell's ball");
        let tip = Point3::new(corner.x * 1.001, corner.y * 1.001, corner.z * 1.001);
        b.edges.push(Rim {
            key: edge(9),
            arc: Some((
                Curve3::line(lift3(corner), lift(tip - corner)),
                (iv0(0.0), iv0(1.0)),
            )),
            ends: [vertex(7), vertex(8)],
            uses: vec![curve_use(true, (0.0, 0.0), (0.0, 0.0))],
        });
        assert!(
            !reach(&s, &b, cell).0.edges.contains(&3),
            "the segment is clear of the stored cell"
        );
        assert_eq!(
            posed(&s, &b, cell, (&[0], &[])).err(),
            Some(Unpresented::Local),
            "the segment in the pose's rectangle"
        );
    }
}
