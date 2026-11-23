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
//! - **Departure** ([`certify_pair_in`]), across a shared edge or vertex:
//!   a separator `ψ`, vanishing on the shared cell, takes one strict sign
//!   on one face's part of the pair and the other strict sign on the
//!   other's, so a coincidence can only happen on the shared cell, where
//!   `f` is injective. The separators and their order are listed at
//!   [`departure`].
//! - **Second order**, at a wedge-0/2π edge, is not built: such an edge
//!   refuses typed, naming it.
//!
//! Each arm's sign is read off a [`Presented`] face: the face's part of
//! the cell written as the image of a rectangle on whose sides the shared
//! cell lies, or which a curved shared edge's chart curve crosses as a
//! graph over one chart axis ([`present_curved`]; a spiric's wall image
//! on a torus, over the minor angle, [`present_wall`]), and a derivative
//! across those sides or that curve. At a corner where a curved edge
//! meets another, the shared one is read alone, through the vertex,
//! where the other is decided to leave onto its face side; where it is
//! not (a tangent joint, a reflex one), the two are read as one run, a
//! graph over one chart axis through the vertex ([`present_corner`]).
//!
//! # The ε concessions
//!
//! A presentation drops points of the face in six places, each stated
//! where it is decided. Every point dropped lies within a few ulps, in
//! the chart, of a line the face's boundary or extent is placed on, or
//! within a few ulps, in the curve's parameter, of a closed edge's
//! vertex, or within the band, in metres, of a corner's other edge
//! or of a vertex two edges of one line meet at:
//!
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
//! - **the joint** ([`joints`], read by [`vacuous`], [`run_on`] and
//!   [`present_chart`]'s merge and zero side): two edges of one family
//!   of coordinate lines, the face on the same side of each, the one
//!   ending at the vertex the other begins at, by key, are one line and
//!   cover it end to end. What is bridged is the range between the two
//!   rows' stored readings of that vertex along the line, and across
//!   it, the hull of the two rows' held values. Nothing compares them;
//!   two certified checks bound them: each row's envelope
//!   (`pcurve_envelope`, within ε of its carrier over its span) and the
//!   edge certificate's end pins (`carrier_endpoint_start` / `_end`,
//!   within ε of the vertex), so the two readings map within `4ε` of
//!   one another in metres. In the chart that is `4ε` over the chart's
//!   speed there, so on a parallel a few ε from a pole or an apex it
//!   may be a large part of a turn of azimuth while still `4ε` of
//!   boundary. A face that truly came through a gap that narrow goes
//!   unread by the cover. The distance arm reads it: pieces of the
//!   boundary that share no cell are more than the band apart, or the
//!   pair is `Violated`.
//!
//!   **The bound is the body's, not this module's.** A row is stored
//!   only where its mint certified it, and a body at rest has had every
//!   row read again by tier 3 (`validate_pcurves`). The document door
//!   and the at-rest gate ask bodies at rest. The shell's cavity gate
//!   (`shell::cavity_embedded`) asks the moved clone before it is
//!   grafted, construction state, whose rows are those its producers'
//!   closing mints certified; [`crate::clearance::Side`] takes any
//!   body, so nothing in the signature says which.
//! - **the join** ([`present_curved`], [`present_wall`]): a closed
//!   edge's periodic chart curve is read through its vertex by its
//!   closed form, past a span
//!   that stops short of the period, in the curve's parameter, by the
//!   rounding of `τ`. Its readers are a harmonic row, a spiric's wall
//!   image, and a section ellipse's focal image on a cone, continued a
//!   turn of azimuth on by its sense ([`ChartCurve::wind`]). The gap is
//!   ulps of `τ` in the parameter, so in metres it is that times the
//!   curve's speed: for a focal image no more than
//!   `|v|·sin α·(1 + |β|)/(1 − |β|) + |(va, vb)|` per unit of
//!   parameter, the azimuth's greatest rate at the slant's greatest
//!   lever plus the slant's own;
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
//!
//! The same membership reads a collapsed side ([`Along::at`]) zero under
//! a plane through the stored vertex: the side's image is the cone's
//! apex or the sphere's pole, which the vertex is within the band of
//! (`vertex_at_singular_point`, the one reading of both), so `ψ` is
//! read zero along a side it may miss by that distance over its lean.
//! [`departure`]'s plane through the pair's one shared vertex reads it
//! wherever a face against that vertex is presented by its collapse.

use geom_core::UnitVec3;
use std::collections::{BTreeMap, BTreeSet};

use geom::{Curve3, Surface};
use geom_brep::PolygonOrientation;
use geom_core::interval::Interval;
use geom_core::{Band, Bounds, Lever, Margin, Point2, Point3, Real, Sign, Vec3};

use crate::cell_region::Ball;
use crate::clearance::{CertificateArm, EngineRefusal, SharedCell, Which};
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::validate::decide;

/// The funnel site of a presented face's side that lies on the shared
/// cell: the separator over that side, metres. `Zero` is "the side lies
/// on the separator at the run's tolerance"; anything else fails the
/// arm.
pub(crate) const SELF_INTERSECTION_ON_SHARED: &str = "self_intersection_on_shared";
/// The funnel site of a separator's sign over a presented face, metres.
/// It carries two quantities: where a side is shared, the sine of the
/// angle at which the face leaves the separator across it, levered at
/// the reach of the faces the pair shares that cell between
/// ([`arm_of`]); where none is, the separator's value, a distance.
/// The two are not split into two sites here: the sine reading decides
/// an edge's class over a cell, which
/// `work/embed/the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict.md`
/// removes (the class is read from tier 3's verdict), and that unit is
/// where the site's two quantities part.
pub(crate) const SELF_INTERSECTION_DEPARTURE: &str = "self_intersection_departure";
/// The funnel site of the graph arm's monotone edge: the edge's advance
/// along the projection's first axis over its piece, metres; and, where
/// the graph runs on along a run's other edge ([`graph_run`]), that
/// edge's advance along the same axis over its reach from the vertex,
/// metres.
pub(crate) const SELF_INTERSECTION_GRAPH: &str = "self_intersection_graph";
/// The funnel site of a planar presentation's reach along its edge:
/// how far inside the edge's two ends the cell's extent stays, metres.
pub(crate) const SELF_INTERSECTION_SPAN: &str = "self_intersection_span";
/// The funnel site of a plane face's convexity in a cell: how far one
/// line edge, at its far end or over the cell's reach, stands on another's
/// face side, metres. Each reading is a distance from a line along its
/// unit normal: a corner's far end from the other edge, or the points of
/// the base a strip's end caps from the cap.
pub(crate) const SELF_INTERSECTION_CORNER: &str = "self_intersection_corner";
/// The funnel site of a cone face's nappe: a vertex's signed height
/// along the axis from the apex, metres. Its sign is the nappe.
pub(crate) const SELF_INTERSECTION_NAPPE: &str = "self_intersection_nappe";
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
    /// The image's closed form on the curve's own parameter, where it has
    /// one of these: a coordinate line that is a harmonic row has both.
    pub(crate) curve: Option<ChartCurve>,
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

/// Which way a focal image's azimuth runs as its parameter grows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Sense {
    /// With the chart's frame.
    With,
    /// Against it.
    Against,
}

impl Sense {
    /// `+1` with the frame, `−1` against it.
    fn sign(self) -> i64 {
        match self {
            Self::With => 1,
            Self::Against => -1,
        }
    }
}

/// **A focal-section chart image** (`geom_brep::Pcurve::FocalSection`:
/// a plane's section ellipse on its cone), on the curve's own parameter:
/// a Kepler azimuth `u = u0 ± (E + 2·atan2(β·sin E, 1 − β·cos E))`,
/// `E = t − t0`, and a harmonic second channel `v`, `[v0, va, vb, vl]`.
/// It holds the stored row's own image, lifted, and evaluates that.
///
/// Built only by [`Focal::of`], so every number is certified and `β`'s
/// enclosure lies strictly inside `(−1, 1)`. Two facts every reader
/// rests on follow, for every member: the azimuth's derivative
/// `±(1 − β²)/(1 − 2β·cos E + β²)` has its sense's sign, so the azimuth
/// is strictly monotone in `t`; and `1 − β·cos E > 0`, so the Kepler
/// angle's arctangent never meets its cut.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Focal {
    image: geom_brep::FocalImage<Interval>,
    sense: Sense,
}

/// **A chart image's closed form on its curve's own parameter**: what a
/// curved edge's readings enclose it and its derivative by
/// ([`present_curved`], [`present_wall`], [`leaves_line`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum ChartCurve {
    /// A harmonic row.
    Harmonic(Harmonic),
    /// A spiric's wall image on a torus.
    Wall(Wall),
    /// A focal section's image.
    Focal(Focal),
}

/// One edge of a face's boundary, lifted.
#[derive(Clone, Debug)]
pub(crate) struct Rim {
    pub(crate) key: EdgeKey,
    /// The carrier and its parameter span; `None` for null scaffolding,
    /// whose locus is its vertex.
    pub(crate) arc: Option<(Curve3<Interval>, (Interval, Interval))>,
    /// The carrier as the body stores it, where the body's scalar is a
    /// point scalar (`f64`): the claim its lift encloses. Two edges whose
    /// stored carriers are one conic as values, their frames one up to the
    /// signs of their axes ([`one_stated_plane`]), name one exact conic,
    /// which their lifts, enclosures of it a rounding wide, cannot show
    /// as one value ([`one_conic`]). `None` on a body held as enclosures,
    /// and for null scaffolding.
    pub(crate) stated: Option<Curve3<f64>>,
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
    /// sheet, not wrapped. It is the hull of the rows alone: a row's end
    /// at a pole is its pcurve's value there, a rounding to either side
    /// of the singular latitude, and [`face_extent`] puts the latitude
    /// in. A row that is a coordinate line whose moving
    /// coordinate is monotone gives its ends, which bound it; a harmonic,
    /// spiric wall or focal row its enclosure over its span, so the hull may be
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
    ///
    /// In a pose's lifted boundary this is the posed extent instead, read
    /// off the lift rather than assumed; [`posed_extent`] says what it
    /// rests on.
    pub(crate) extent: Option<[(f64, f64); 2]>,
    /// **The outer loop's rows in the loop's own order**, each an index
    /// into `edges` and whether its use runs along the edge: the body's
    /// walk of the loop ([`crate::live::FaceView::boundary_by_loop`]),
    /// kept by the lift. `None` for a face with a ring, or one whose walk
    /// does not resolve.
    pub(crate) outer: Option<Vec<(usize, bool)>>,
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
/// touches through the face's own identifications. The face's poses are
/// read from `face`, the walk's store for the ask ([`FacePoses`]).
pub(crate) fn certify_self(
    x: &Piece<'_>,
    y: &Piece<'_>,
    near: &Near,
    band: Band,
    face: &FacePoses,
) -> Outcome {
    let collapsed = x.boundary.collapsed();
    if let Some(&v) = near.vertices.iter().find(|v| collapsed.contains(v)) {
        return Outcome::Refuse(EngineRefusal::CollapsedChart {
            face: x.face,
            side: Which::First,
            vertex: v,
        });
    }
    match near.edges.as_slice() {
        [_] => departure(x, y, near, band, false, (face, face)),
        _ => Outcome::Split {
            arm: CertificateArm::Departure,
            shared: near.cell(x.boundary).unwrap_or(SharedCell::Face(x.face)),
        },
    }
}

/// **A touching pair of two faces**: the departure arm, its separators
/// in a fixed order.
#[cfg(test)]
fn certify_pair(x: &Piece<'_>, y: &Piece<'_>, near: &Near, band: Band) -> Outcome {
    let (fx, fy) = (FacePoses::default(), FacePoses::default());
    certify_pair_in(x, y, near, band, (&fx, &fy))
}

/// The pair's certificate, each face's poses read from the walk's store
/// for the ask ([`FacePoses`]); its rows' door, with fresh stores, is
/// `certify_pair`.
pub(crate) fn certify_pair_in(
    x: &Piece<'_>,
    y: &Piece<'_>,
    near: &Near,
    band: Band,
    faces: (&FacePoses, &FacePoses),
) -> Outcome {
    departure(x, y, near, band, true, faces)
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
    definite_as(row, Margin::of(x), band)
}

/// A margin's definite sign at `row`, or `None` where it is zero, in
/// band or does not certify.
fn definite_as(row: &'static str, margin: Margin<Interval>, band: Band) -> Option<Sign> {
    match decide(row, margin, band) {
        Ok(s @ (Sign::Positive | Sign::Negative)) => Some(s),
        _ => None,
    }
}

/// The definite sign of a sine levered at `arm`, decided at the
/// departure's site, or `None`.
fn levered_sign(sine: Interval, arm: Lever<Interval>, band: Band) -> Option<Sign> {
    definite_as(
        SELF_INTERSECTION_DEPARTURE,
        Margin::levered(sine, arm),
        band,
    )
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
/// columns ([`present_curved`]). In those searches it only coarsens or
/// refines. It is also the width two margins are levered at:
/// [`monotone_run`] decides each piece's advance over the piece's own
/// parameter width, and [`leaves_line`] a piece's rise over its width
/// under its own halving, so a longer edge or a different count moves
/// what those two decide
/// (`work/embed/five-more-injectivity-margins-are-levered-at-a-cell-of-the-sweep.md`).
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

    /// Two sides of different axes, the `u` side first.
    fn by_axis(s: Self, t: Self) -> (Self, Self) {
        if s.axis() == Axis::U { (s, t) } else { (t, s) }
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
    /// The ends of the side's one edge at which a line of the other axis
    /// whose own edge ends there clips the rectangle ([`corners`]): the
    /// side ends at that vertex, by key ([`Along::ending`]).
    stops: Vec<VertexKey>,
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
            stops: Vec::new(),
        }
    }

    /// The vertex a collapsed side is ([`Self::at`]): the side lies on no
    /// edge. Read off what the side was built from, the chart's own
    /// singular vertex, never off a width or a derivative. An invariant
    /// of construction, not of the type: [`Self::at`] is the one place
    /// outside a test that sets `vertex`, and it sets no edge.
    fn collapsed(&self) -> Option<VertexKey> {
        self.vertex.filter(|_| self.edges.is_empty())
    }

    /// A collapsed side: its whole image is the vertex.
    fn at(vertex: VertexKey) -> Self {
        Self {
            edges: Vec::new(),
            vertex: Some(vertex),
            whole: true,
            params: None,
            on_edges: false,
            stops: Vec::new(),
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

    /// **The side's parameters nearest `piece`, ending at a run's vertex
    /// where the side ends there by key.** A side clipped by a line whose
    /// edge ends at the vertex `end.vertex` ([`Along::stops`]) ends at
    /// that vertex: its edge does, and the clip is the line through it.
    /// Its parameters past `end.at` are cut at `end.at`.
    ///
    /// **What is bridged, and what bounds it.** The stretch cut off is
    /// the strip between the edge's stored end and the other line's
    /// outer bound: two rows' readings of one vertex's coordinate (the
    /// edge's end and the line's `at`), times the chart's speed along
    /// the side. This code compares no length and bounds none: a keyed
    /// side is cut however far past the vertex its parameters reach
    /// (`a_keyed_side_is_cut_at_the_vertex_however_far_past_it_reaches`
    /// pins that at `0.1`). The bound is the body's own, and is enforced
    /// where the stop is recorded, not here: [`present_chart`] records a
    /// stop only on a rectangle [`sides_are_the_faces`] has passed
    /// (every side its edges' own run on to the other line's far bound
    /// and no farther, every corner convex), so the side's reach past
    /// the vertex is that line's enclosure and the corner is the face's.
    /// A side built with a stop it was not recorded with has no such
    /// bound.
    ///
    /// This is one more private spelling of a vertex's reading on an
    /// edge (the row
    /// `a-vertexs-reading-on-an-edge-is-spelled-privately-by-each-reader-that-joins-at-it`
    /// lists them).
    ///
    /// Not checked here: that the clip's enclosure is the enclosure of
    /// the line through that vertex. [`present_chart`] records the stop
    /// from the clip's own edges ([`corners`]), a line that clips the
    /// rectangle and whose edge has the vertex for an end, and from
    /// nothing about its width.
    fn ending(&self, piece: (f64, f64), end: Option<&RunEnd>) -> Option<(f64, f64)> {
        let (a, b) = self.params?.nearest(piece)?;
        Some(match end {
            Some(end) if self.stops.contains(&end.vertex) => {
                if end.start {
                    (a.max(end.at), b)
                } else {
                    (a, b.min(end.at))
                }
            }
            _ => (a, b),
        })
    }

    /// **Whether the graph separator over `piece` is zero along this
    /// side or curve**: the one rule both are read by. The graph vanishes
    /// on its edge's carrier over the piece and nowhere else
    /// ([`graph_of`]), so a whole zero is zero where its parameters lie
    /// in the piece, a side that ends at the graph's run's vertex by key
    /// read to that vertex ([`Along::ending`]). A zero whose chart line
    /// is not its edge's own line or circle has none read: it is zero
    /// where it lies on its edge, its points the edge's in the
    /// presentation's image, which the piece holds up to the rounding of
    /// the chart against the stored carrier
    /// (`edge-piece-misses-a-chart-sides-stored-carrier-by-the-charts-rounding`).
    /// A whole zero whose parameters were not read answers nothing.
    fn under_graph(&self, piece: (f64, f64), end: Option<&RunEnd>) -> bool {
        match self.params {
            Some(_) => self
                .ending(piece, end)
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
    if let Some(ChartCurve::Harmonic(h)) = u.curve
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

    /// The image of the corner of two sides.
    fn corner(&self, s: Side, t: Side) -> Point3<Interval> {
        let (xs, ys) = Side::by_axis(s, t);
        let (x, y) = (self.bound(xs), self.bound(ys));
        self.image((x, x), (y, y))
    }

    /// The unit axes a [`Zero::Point`]'s corner leaves into the rectangle
    /// along, read off the rectangle's jet: `Φ_x` and `Φ_y` turned toward
    /// the sides' far ends. `None` for any other zero.
    fn corner_axes(&self) -> Option<[Vec3<f64>; 2]> {
        let Zero::Point(s, t, _) = self.zero else {
            return None;
        };
        let (xs, ys) = Side::by_axis(s, t);
        let (fx, fy, _) = self.jet();
        Some([unit(vmid(fx) * xs.into())?, unit(vmid(fy) * ys.into())?])
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
    /// The edge ends that end on a collapsed side ([`ends_on_collapse`]),
    /// each an index into the boundary's edges and which end; empty for
    /// any other side.
    ends_here: Vec<(usize, usize)>,
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
    poses: &'h Poses<'_>,
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
            Err(stored @ (Unpresented::Structural | Unpresented::Local))
                if unplaced_line(piece, band) =>
            {
                Err(stored)
            }
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

/// **A line in reach with no place at any depth**: a coordinate line of
/// the stored chart, on a periodic axis whose sheet the face's extent
/// does not give ([`Sheet::Unknown`], [`placed`]), in the cell's local
/// boundary. That is the face's fact, the same in every cell holding
/// the line, so no smaller cell changes it, and [`present`] keeps the
/// stored chart's answer rather than try a pose.
fn unplaced_line(piece: &Piece<'_>, band: Band) -> bool {
    let b = piece.boundary;
    let Ok(Sheets::On(sheet)) = sheets(piece.surface, b, (piece.u, piece.v), band) else {
        return false;
    };
    piece.local.edges.iter().any(|&i| {
        b.edges.get(i).is_some_and(|e| {
            e.arc.is_some()
                && !seam(e)
                && e.uses.iter().any(|u| {
                    u.iso.is_some_and(|iso| {
                        placed(piece.surface, iso, sheet, iso.held.of((piece.u, piece.v))).is_none()
                    })
                })
        })
    })
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

/// **The face's extent, read against the face's own lines, nappe and
/// singular points** ([`Boundary::extent`]); the one reader of that field.
///
/// **A face reaches each singular latitude it holds a vertex at**, a
/// sphere's pole or a cone's apex ([`reaching_its_singular_points`]), in
/// every cell, as the vertex is the face's: the extent returned ends on
/// that latitude where the stored end is itself at the singular point
/// by the band, and a stored end that is not contradicts the vertex and
/// refuses `Structural`. The lines and the nappe are read against the
/// STORED extent, before it is moved: they are what the stored value is
/// checked by. A cone face's end past its apex, on the other nappe, is on
/// its nappe where it is at the apex by the same decision.
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
    let on_nappe = face_nappe(surface, b, band).is_none_or(|(up, apex)| {
        let (lo, hi) = extent[Axis::V.index()];
        let slack = chart_slack(&[lo, hi]);
        let u = extent[Axis::U.index()].0;
        let at_apex = |v| parallel_at(surface, (u, v), apex, band);
        if up {
            (lo >= 0.0 || at_apex(lo)) && hi > slack
        } else {
            (hi <= 0.0 || at_apex(hi)) && lo < -slack
        }
    });
    if !(lines && on_nappe) {
        return Err(Unpresented::Structural);
    }
    reaching_its_singular_points(surface, b, band, extent)
        .map(Extent::Known)
        .ok_or(Unpresented::Structural)
}

/// **A cone face's nappe** ([`nappe`]) **and its cone's apex**; `None`
/// on any other surface, or where the nappe is not decided.
fn face_nappe(
    surface: &Surface<Interval>,
    b: &Boundary,
    band: Band,
) -> Option<(bool, Point3<Interval>)> {
    match surface {
        Surface::Cone { frame, .. } => {
            let apex = frame.origin();
            Some((nappe(b, apex, frame.axis(), band)?, apex))
        }
        _ => None,
    }
}

/// **Whether a chart's parallel is at the singular point `point`**, by
/// the incidence a vertex is read at it with (`vertex_at_singular_point`,
/// at `band`): the surface's point at chart `(u, v)` decided at `point`.
/// A sphere's parallel and a cone's circle lie at one distance from the
/// pole or the apex, so one point of it answers for all of it.
fn parallel_at(
    surface: &Surface<Interval>,
    (u, v): (f64, f64),
    point: Point3<Interval>,
    band: Band,
) -> bool {
    use geom_brep::chart_singularity::vertex_at_singular_point;
    matches!(
        vertex_at_singular_point(surface.eval(at(u), at(v)), point, band),
        Ok(true)
    )
}

/// **A sphere's polar latitude**, `v = π/2` at the north pole and
/// `-π/2` at the south: a bound of its chart's canonical domain.
fn pole_latitude(north: bool) -> Interval {
    let half = Interval::pi() * at(0.5);
    if north { half } else { -half }
}

/// The singular latitudes of a chart a point is at, `[high, low]`: the
/// one that is the high end of the face's `v`, and the one that is its
/// low end, each with the singular point it collapses to.
type SingularEnds = [Option<(Interval, Point3<Interval>)>; 2];

/// **The singular latitudes of its chart that the face's vertex `p` is
/// at** ([`SingularEnds`]): a sphere's `π/2` at its north pole, the high
/// end, and `-π/2` at its south, the low; a cone's `0` at its apex, the
/// low end of a face on the positive nappe and the high end of one on
/// the negative ([`nappe`]). None on any other surface.
///
/// `p` is at a singular point by the one reading of that incidence
/// (`vertex_at_singular_point`): its distance from the point decided
/// zero at `band`, which a vertex within the band's zero of the point
/// is. An incidence the band does not decide reads as off the point.
///
/// `None` where `p` is at a cone's apex and the face has no one nappe:
/// the apex is then no one end of its `v`.
fn singular_ends(
    surface: &Surface<Interval>,
    b: &Boundary,
    p: Point3<Interval>,
    band: Band,
) -> Option<SingularEnds> {
    use geom_brep::chart_singularity::{Singularity, sphere_poles, vertex_at_singular_point};
    let held = |point| matches!(vertex_at_singular_point(p, point, band), Ok(true));
    match *surface {
        Surface::Sphere { frame, radius, .. } => {
            let line = geom::Line {
                origin: frame.origin(),
                dir: frame.axis_unit(),
            };
            let mut ends = [None; 2];
            for (pole, point) in sphere_poles(line, radius) {
                let north = pole == Singularity::North;
                if held(point) {
                    ends[usize::from(!north)] = Some((pole_latitude(north), point));
                }
            }
            Some(ends)
        }
        Surface::Cone { frame, .. } if held(frame.origin()) => {
            let apex = frame.origin();
            let low = nappe(b, apex, frame.axis(), band)?;
            let mut ends = [None; 2];
            ends[usize::from(low)] = Some((at(0.0), apex));
            Some(ends)
        }
        _ => Some([None; 2]),
    }
}

/// **A face's extent, reaching each singular latitude the face holds a
/// vertex at** ([`singular_ends`]). A vertex at a sphere's pole or a
/// cone's apex is a point of the face on that latitude, an end of the
/// face's `v`, so the face's extent ends on it. The stored hull ends
/// where the lift's rows do, and a meridian's end at the point is its
/// pcurve evaluated at the edge's parameter: a rounding at the scale of
/// the edge's coordinates, to either side of the latitude, or, for a
/// vertex in the band of the point and off it, the vertex's own
/// latitude. The hull with the latitude is a superset of the stored one,
/// so it holds the face still.
///
/// **The extent moves where the stored end is itself at the singular
/// point**: the surface's parallel at the stored end latitude decided at
/// the point by the incidence that decided the vertex ([`parallel_at`]),
/// so within the band's zero of it, in metres. That is the one decision
/// asked of the row's end as it is of the vertex; this reader has no
/// scale of its own. Every point of the strip between the stored end
/// and the latitude is then at the singular point by that decision.
///
/// **A stored end that is not at the point contradicts the vertex**,
/// `None`: the vertex is at the singular point and the row that ends at
/// that vertex is not, one face's data naming two places.
///
/// A cone face with no one nappe is left as stored: the apex is no one
/// end of it.
fn reaching_its_singular_points(
    surface: &Surface<Interval>,
    b: &Boundary,
    band: Band,
    mut extent: [(f64, f64); 2],
) -> Option<[(f64, f64); 2]> {
    let u = extent[Axis::U.index()].0;
    let v = &mut extent[Axis::V.index()];
    for &p in b.vertices.values() {
        let Some([high, low]) = singular_ends(surface, b, p, band) else {
            continue;
        };
        if let Some((line, point)) = high {
            if v.1 < line.lo() && !parallel_at(surface, (u, v.1), point, band) {
                return None;
            }
            v.1 = v.1.max(line.hi());
        }
        if let Some((line, point)) = low {
            if v.0 > line.hi() && !parallel_at(surface, (u, v.0), point, band) {
                return None;
            }
            v.0 = v.0.min(line.lo());
        }
    }
    Some(extent)
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
    let lines = Lines::of(surface);
    outside.iter().any(|&(side, line, ei)| {
        let (lo, hi) = side.axis().of(cell);
        // Which end of the line is read. The stored branch reads its near
        // end: at `f64` a stored line is a point, its ends one; at
        // `Interval` it is the bracket the lift rounds it to, and the touch
        // holds where any point of the bracket touches, `few_ulps` and
        // the bracket's width, the stored concession as the stored chart
        // has always read it. A pose reads its far end: its line is an
        // enclosure of its edge's lift, and it touches only where every
        // point of the enclosure does (the end the comparison below
        // binds), so a pose's touch is never wider than the stored reading
        // of a line of that width.
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
        let (held, axis) = (side.axis(), side.axis().other());
        // The edges this one's line may run on through: the face on the
        // same side of each. Which of them are its line is the joints'.
        let same_side: Vec<usize> = outside
            .iter()
            .filter(|&&(s, ..)| s == side)
            .map(|&(.., e)| e)
            .collect();
        touches
            && line_ranges(b, lines, &same_side, held, Some(ei))
                .is_some_and(|r| covered(axis.of(cell), &r, lines.periodic[axis.index()]))
    })
}

/// **What the cover reads of a chart**: which axes are periodic, and
/// whether a line's edges are joined on it ([`joints`]).
#[derive(Clone, Copy)]
struct Lines {
    periodic: [bool; 2],
    /// A closed-form chart. A spline chart may carry one point at two
    /// parameters of a line (a closed net), where a shared vertex does
    /// not say two ranges are end to end.
    joined: bool,
}

impl Lines {
    fn of(surface: &Surface<Interval>) -> Self {
        Self {
            periodic: [Axis::U, Axis::V].map(|axis| periodic(surface, axis)),
            joined: !matches!(surface, Surface::Nurbs(_)),
        }
    }
}

/// **A line edge's two ends along its line**: the vertex and the moving
/// coordinate at its low end, then at its high. `None` for an edge the
/// face uses twice, a whole turn's image, a use holding the other axis,
/// and one whose direction does not certify.
fn line_ends(e: &Rim, held: Axis) -> Option<[(VertexKey, Interval); 2]> {
    let [u] = e.uses.as_slice() else {
        return None;
    };
    let iso = u.iso.filter(|iso| iso.held == held && !iso.full)?;
    let [p, q] = u.image?;
    let ends = [(e.ends[0], iso.moving(p)), (e.ends[1], iso.moving(q))];
    Some(if forward(u, iso)? > 0.0 {
        ends
    } else {
        [ends[1], ends[0]]
    })
}

/// **The joints of `edges`**, edges holding `held` with the face on the
/// same side of each (the caller's: one side's clips, or the lines a
/// cell is outside of on one side): each pair `(i, j)` where the high
/// end of `i` and the low end of `j` are one vertex by key, with the
/// range between the two stored readings of that vertex's coordinate
/// along the line, `j`'s moved by whole turns to `i`'s on a periodic
/// axis.
///
/// **Two edges so joined are one line, and end to end on it**, and no
/// value is asked. A coordinate line of a chart through a point is one
/// line, and the two run one way along it (the face is on one side), so
/// one ends at the vertex and the next begins there: together they
/// cover the line from the one's far end to the other's. Their stored
/// held values, and their stored ends at the vertex, are each row's own
/// evaluation of one line and one point, and which is larger by a
/// rounding is no fact of the body.
///
/// **That is an ε concession** (the module docs give its bound and
/// who holds it): what lies between the two readings is read as
/// boundary, within `4ε` in metres by two certified checks, neither of
/// them made here.
///
/// **One continuation.** A vertex where two of the edges begin, or two
/// end, joins none of them: the line does not run on one way there.
///
/// **On the closed-form charts only** ([`Lines::joined`]).
fn joints(
    b: &Boundary,
    lines: Lines,
    edges: &[usize],
    held: Axis,
) -> Vec<(usize, usize, Interval)> {
    if !lines.joined {
        return Vec::new();
    }
    let periodic = lines.periodic[held.other().index()];
    let ends: Vec<(usize, [(VertexKey, Interval); 2])> = edges
        .iter()
        .filter_map(|&i| Some((i, line_ends(b.edges.get(i)?, held)?)))
        .collect();
    let mut out = Vec::new();
    for &(i, [_, (v, hi)]) in &ends {
        for &(j, [(w, lo), _]) in &ends {
            if i == j || v != w {
                continue;
            }
            let lo = if periodic { wrap(lo, mid(hi)) } else { lo };
            out.push((
                i,
                j,
                Interval::from_bounds(hi.lo().min(lo.lo()), hi.hi().max(lo.hi())),
            ));
        }
    }
    let alone = |&(i, j, _): &(usize, usize, Interval)| {
        out.iter().filter(|&&(a, _, _)| a == i).count() == 1
            && out.iter().filter(|&&(_, b, _)| b == j).count() == 1
    };
    out.iter().copied().filter(alone).collect()
}

/// The edges joined to `ei`, end to end, by `joints`; `ei` among them.
fn run_through(joints: &[(usize, usize, Interval)], ei: usize) -> Vec<usize> {
    let mut run = vec![ei];
    while let Some(next) =
        joints
            .iter()
            .find_map(|&(i, j, _)| match (run.contains(&i), run.contains(&j)) {
                (true, false) => Some(j),
                (false, true) => Some(i),
                _ => None,
            })
    {
        run.push(next);
    }
    run
}

/// **The ranges a line's edges cover together**: each edge's
/// ([`edge_ranges`]) and, across each joint, the range between its two
/// readings ([`joints`]). Of `edges`, the ones joined to `through`
/// where it is given ([`run_through`]), else all of them.
fn line_ranges(
    b: &Boundary,
    lines: Lines,
    edges: &[usize],
    held: Axis,
    through: Option<usize>,
) -> Option<Vec<Interval>> {
    let joints = joints(b, lines, edges, held);
    let run = through.map(|ei| run_through(&joints, ei));
    let on = |e: &usize| run.as_ref().is_none_or(|run| run.contains(e));
    let mut out = edge_ranges(b, run.as_deref().unwrap_or(edges), held)?;
    out.extend(
        joints
            .iter()
            .filter(|(i, j, _)| on(i) && on(j))
            .map(|&(.., r)| r),
    );
    Some(out)
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
/// side out to that clip's line's far bound, and one that ends at the
/// vertex a collapsed side is covers it out to that side
/// ([`ends_on_collapse`]). The run-on is sound only
/// with the other two checks, which is why all three are read here and
/// nowhere else: that the other line bounds the face there is that
/// line's own coverage, and that the corner is the face's is
/// [`convex`].
fn sides_are_the_faces(
    b: &Boundary,
    lines: Lines,
    merged: &[Clip],
    rect: ((f64, f64), (f64, f64)),
) -> bool {
    let covers = |c: &Clip| {
        let axis = c.side.axis().other();
        run_on(b, lines, c, merged)
            .is_some_and(|ranges| covered(axis.of(rect), &ranges, lines.periodic[axis.index()]))
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
fn run_on(b: &Boundary, lines: Lines, c: &Clip, merged: &[Clip]) -> Option<Vec<Interval>> {
    let held = c.side.axis();
    let periodic = lines.periodic[held.other().index()];
    let mut out = line_ranges(b, lines, &c.edges, held, None)?;
    // The end `k` of the edge `ei`, run on to `d`'s line's far bound.
    let mut run = |ei: usize, k: usize, d: &Clip| -> Option<()> {
        let e = b.edges.get(ei)?;
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
                Side::XHi | Side::YHi => Interval::from_bounds(end.lo(), end.hi().max(d.at.hi())),
                Side::XLo | Side::YLo => Interval::from_bounds(end.lo().min(d.at.lo()), end.hi()),
            });
        }
        Some(())
    };
    for d in merged.iter().filter(|d| d.side.axis() != held) {
        if d.collapsed.is_some() {
            // An edge that ends on the collapsed side, by its end vertex's
            // key ([`ends_on_collapse`]).
            for &(ei, k) in d.ends_here.iter().filter(|(ei, _)| c.edges.contains(ei)) {
                run(ei, k, d)?;
            }
            continue;
        }
        let at = corners(b, c, d);
        for &ei in &c.edges {
            let e = b.edges.get(ei)?;
            for (k, _) in e.ends.iter().enumerate().filter(|(_, v)| at.contains(v)) {
                run(ei, k, d)?;
            }
        }
    }
    Some(out)
}

/// **The edge ends that end on a collapsed side**, the side being the
/// vertex `v`, read at the singular point `point` ([`singular_ends`]):
/// each end of a boundary edge whose end vertex is `v`, by key, and
/// whose chart image there is at `point` by the band ([`parallel_at`]).
/// [`run_on`] runs such an end's line on to the collapsed side.
///
/// **Why that is the face's side, with no slack read**: the edge's curve
/// ends at `v`; `v` is at the singular point, by the one incidence; and
/// the image's end, a rounding of the edge's own parameters or the
/// vertex's own latitude, is at the singular point by the same decision,
/// so every surface point of the strip between it and the collapsed side
/// is too. The boundary in that strip is nowhere else at the band, and
/// the face has no point there to wrap round the edge's end into.
///
/// An end whose vertex is another, however near the side, is not one,
/// nor an end whose image is off the point by the band: that edge and
/// its vertex name two places, and it covers only its own range.
fn ends_on_collapse(
    surface: &Surface<Interval>,
    b: &Boundary,
    v: VertexKey,
    point: Point3<Interval>,
    band: Band,
) -> Vec<(usize, usize)> {
    let at_point = |e: &Rim, k: usize| {
        e.uses.iter().all(|u| {
            let (Some(iso), Some(image)) = (u.iso, u.image) else {
                return false;
            };
            let (held, end) = (iso.at, iso.moving(image[k]));
            let chart = |h: f64, m: f64| match iso.held {
                Axis::U => (h, m),
                Axis::V => (m, h),
            };
            held.is_certified()
                && end.is_certified()
                && [end.lo(), end.hi()]
                    .into_iter()
                    .all(|m| parallel_at(surface, chart(held.lo(), m), point, band))
        })
    };
    b.edges
        .iter()
        .enumerate()
        .flat_map(|(ei, e)| (0..2).map(move |k| (ei, e, k)))
        .filter(|&(_, e, k)| e.ends[k] == v && at_point(e, k))
        .map(|(ei, _, k)| (ei, k))
        .collect()
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
///   other nappe: each a cell that lies off the face. A rectangle
///   however thin beside a shared line is presented and read.
/// - A face whose lines or nappe contradict its extent ([`face_extent`])
///   is never presented, and a rectangle its own clips leave with no
///   width on an axis is not presented in that cell.
fn present_chart<'a>(
    piece: &Piece<'a>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
) -> Result<Presentation<'a>, Unpresented> {
    present_chart_in(piece, near_edges, near_vertices, band, false)
}

/// [`present_chart`], in the stored chart or, `posed`, in a pose of a
/// sphere ([`present_posed`]): there the cell is under a turn wide, its
/// sheet is the posed extent's ([`posed_extent`]), and a side carries no
/// parameters.
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
        // A pose's extent is the pose's reading ([`posed_extent`] says
        // what it rests on), not the face's fact: where it is not granted,
        // or its lines contradict it ([`face_extent`]), the cell is placed
        // by itself, under a turn wide ([`posed_cell`]), never refused.
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
                            ends_here: Vec::new(),
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
                ends_here: Vec::new(),
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
    // Lines on one side are one line where they hold one common value,
    // or where their edges are joined end to end by key ([`joints`]),
    // whatever values their rows hold; two different lines on one side
    // do not bound a face the clip reads.
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
            let edges: Vec<usize> = on.iter().flat_map(|c| c.edges.iter().copied()).collect();
            let joints = joints(b, Lines::of(piece.surface), &edges, side.axis());
            let run = run_through(&joints, first.edges[0]);
            if !edges.iter().all(|e| run.contains(e)) {
                return Err(Unpresented::Local);
            }
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
            ends_here: Vec::new(),
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
    // A sphere's pole and a cone's apex are collapsed sides of the chart:
    // `v = ±π/2`, bounds of the sphere's canonical domain, and `v = 0`.
    let on_cone = matches!(piece.surface, Surface::Cone { .. });
    for &v in &piece.local.vertices {
        let Some(&p) = b.vertices.get(&v) else {
            continue;
        };
        // A face with an apex vertex in reach and no one nappe is not
        // presented in any cell that holds that vertex; a cell away
        // from the apex reads it as any face.
        let Some(ends) = singular_ends(piece.surface, b, p, band) else {
            return Err(Unpresented::Structural);
        };
        for (low, end) in [false, true].into_iter().zip(ends) {
            let Some((line, point)) = end else {
                continue;
            };
            // The cell on a cone's other nappe holds only the apex of the
            // face. This is read only where the face's extent wraps or is
            // unknown: a known one is on the face's nappe ([`face_extent`]),
            // and a cell on the other misses it in [`sheets`] first.
            let (cv0, cv1) = piece.v;
            if on_cone && ((low && cv1 <= 0.0) || (!low && cv0 >= 0.0)) {
                return Ok(Presentation::Vacuous);
            }
            // A rectangle short of the latitude is a cell short of it, or
            // one a line of the face bounds short of it. The extent, where
            // it is known, reaches the latitude ([`face_extent`]), so its
            // clip is never what stops the rectangle short.
            if (low && y.0 > line.hi()) || (!low && y.1 < line.lo()) {
                continue;
            }
            // A rectangle wholly past the latitude is left with no width
            // here, which is refused below. Where the extent was moved to
            // the latitude, the strip between its stored end and the
            // latitude is at the singular point by the band
            // ([`reaching_its_singular_points`]), so the face's points in
            // it are read as the vertex, the side's zero.
            if low {
                y.0 = line.lo();
            } else {
                y.1 = line.hi();
            }
            let side = Side::of(Axis::V, low);
            // A line on that side that does not hold the latitude, in a
            // rectangle that still reaches it, lies within the slack of it
            // (every line meets the extent): the collapse and the line are
            // two readings of one side that do not agree, which a smaller
            // cell does not hold both of.
            match merged.iter_mut().find(|m| m.side == side) {
                Some(m) if m.at.lo() <= line.hi() && line.lo() <= m.at.hi() => {
                    // A second vertex at the same collapsed side replaces the
                    // first here. That only refuses: the first is then on no
                    // zero side, and its edges are not run on.
                    m.collapsed = Some(v);
                    m.ends_here = ends_on_collapse(piece.surface, b, v, point, band);
                }
                Some(_) => return Err(Unpresented::Local),
                None => merged.push(Clip {
                    side,
                    at: line,
                    edges: Vec::new(),
                    collapsed: Some(v),
                    ends_here: ends_on_collapse(piece.surface, b, v, point, band),
                }),
            }
        }
    }
    // No width on an axis, a negative one included: a low line above a
    // high one is a face that leaves the cell and comes back into it,
    // which no one rectangle presents, and a zero one is the face's clips
    // contradicting one another, as it has interior wherever a line has
    // it on the cell's side. A smaller cell holds
    // one of the parts, or neither line. A whole patch keeps its cell.
    // The width is read at the touch's scale, never wider, as
    // `a_filleted_drum_certifies_across_its_fillet` pins.
    let no_width = |(lo, hi): (f64, f64)| hi - lo <= few_ulps(lo).max(few_ulps(hi));
    if !piece.whole && (no_width(x) || no_width(y)) {
        return Err(Unpresented::Local);
    }
    // Every clip's side must be the edges' own, run on past its corners,
    // and every corner convex.
    //
    // **A cell at a reflex corner is refused here at every depth.** The
    // face's part of it is the cell less a quadrant, or the whole cell with
    // the corner on a side, and no one presentation of that part is signed
    // by a first-order separator holding the shared edge's carrier: it
    // vanishes on that edge's chart line past the corner, the face's
    // interior, and the part lies on both sides of the line. Nor by one
    // through the vertex alone, which cuts the face along a curve one of
    // whose rays enters the face's 270°. The carrier arm reads the corner
    // instead, with the face's own carrier against the partner's
    // presentation, where (i) the face has a carrier implicit
    // (`implicit_of`), (ii) the partner is presented, and (iii) the
    // partner lies on one side of the face's whole carrier in the cell.
    // The code tests (iii) by `sign_on(carrier, None, p, None).is_some()`,
    // which is sufficient and stricter: a tangent partner on one side is
    // refused, and a `Zero::Past` side is never read there. Two guards
    // come before it: the arm is not asked inside one face (`two`), and a
    // `Vacuous` partner certifies the pair first. Across a shared edge with a transverse partner (iii) holds for a
    // valid body; at a vertex alone (four or more faces there) a valid
    // partner may cross the carrier in the quadrant the face lacks. Where
    // (i) to (iii) fail, the reading is the cell cut at the vertex into
    // two convex pieces, each certified against the partner with its own
    // separator: `embed/corner-pieces` for a plane,
    // `work/embed/a-reflex-corner-of-a-face-with-no-implicit-carrier-has-no-reading.md`
    // for a face with no implicit.
    if !piece.whole && !sides_are_the_faces(b, Lines::of(piece.surface), &merged, (x, y)) {
        return Err(Unpresented::Local);
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
                line_ranges(b, Lines::of(piece.surface), &c.edges, held, None).is_some_and(
                    |ranges| covered(range, &ranges, periodic(piece.surface, held.other())),
                )
            });
            match clip.map(|c| c.edges.as_slice()) {
                Some(&[e]) => {
                    // The ends of the side's edge where a line of the other
                    // axis, its own edge ending there, clips the rectangle.
                    // None on a whole patch, which keeps its cell and is
                    // not asked whether its sides are its edges'
                    // ([`sides_are_the_faces`]): the bound on what a stop
                    // bridges is that test's ([`Along::ending`]). The
                    // other axis only: a clip is no corner with itself,
                    // and its own edges' ends are no stops. A collapsed
                    // clip names no edge, so it makes no corner here and
                    // needs no filter ([`run_on`] filters it for its
                    // line, which this does not read).
                    let stops = clip
                        .into_iter()
                        .filter(|_| !piece.whole)
                        .flat_map(|c| {
                            merged
                                .iter()
                                .filter(move |d| d.side.axis() != c.side.axis())
                                .flat_map(move |d| corners(b, c, d))
                        })
                        .collect();
                    Zero::Side(
                        side,
                        Along {
                            stops,
                            ..Along::over(piece.surface, b, e, on_edges, || {
                                if posed {
                                    return None;
                                }
                                iso_params(piece.surface, b.edges.get(e)?, range)
                            })
                        },
                    )
                }
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
        frame: *frame,
        radius,
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
        curve: None,
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
                stated: e.stated.clone(),
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
        outer: b.outer.clone(),
    }
}

/// **A face's extent in a pose**, granted only where it is decided; the
/// one statement of what a posed extent rests on, which
/// [`present_posed`] and [`present_chart_in`] point to. The face lies
/// left of its outer loop about its outward normal (the stored extent's
/// premise, [`Boundary::extent`]), and a pose has the stored chart's
/// orientation, as every sphere chart built by [`geom::Frame::new`] has
/// its normal outward; so where the loop's lift into the pose closes on
/// one sheet and runs the way the face's sense says, the face is the
/// lift's bounded side and lies within its hull. Read here, not
/// assumed: in the stored chart that is the B-rep's invariant about how
/// a face is stored, and in a pose a loop that runs the other way is a
/// face holding a pose's pole, or the loop's outside.
///
/// - **The loop** is the body's walk of the face's outer loop, kept by
///   the lift ([`Boundary::outer`]); a face with a ring has none here.
/// - **Every row on this sphere, exactly as stored** ([`on_sphere`]):
///   a row's points are placed by their latitude `asin(Z/R)`, which is
///   the latitude only of a point on the sphere.
/// - **The lift**: each row's carrier, in its traversal order, cut into
///   [`pieces_of`] its span, each piece's box placed in the pose
///   ([`posed_cell`]), so every row has an enclosure that holds it; each
///   piece moved by whole turns to meet the one before. A piece whose box
///   holds the pose's axis has no placement, so a loop through a pose's
///   pole has no extent.
/// - **One sheet**: the hull under a turn wide. A loop's lift that does
///   not close winds round the axis, from a longitude to a whole number
///   of turns past it, so its pieces span a turn or more; under a turn,
///   it closes, and the loop does not wrap the azimuth.
/// - **Its orientation**: the signed area of the polygon through the
///   pieces' starts, certified to the sign the face's sense gives with a
///   margin of the pieces' rectangles' areas, which bound how far the
///   loop's area can differ from the polygon's (the loop and each chord
///   lie in their piece's rectangle). The margin decides only for a face
///   narrower than its rows' chords stray, which may wind either way.
///   The sign itself, a face read as its complement, is pinned by one
///   unit row only (`a_face_thinner_than_its_rows_chords_is_granted_no_extent`,
///   the thin band both ways round). No body row reaches it: it needs a face holding both of the pose's poles
///   while its loop winds round neither, and every sphere face this
///   kernel builds is a revolve's half, which holds a pole its loop
///   winds round, so the wrap refuses first. That premise is the
///   kernel's, not this reader's: a face larger than a hemisphere (an
///   import, say) reopens it, with that row still the sign's only pin.
///
/// The hull of the pieces' rectangles is then the extent, a superset of
/// the face. `None` where any of these does not certify: the pose then
/// places by the cell alone, as it does with no extent.
fn posed_extent(
    frame: &geom::Frame<Interval>,
    radius: Interval,
    b: &Boundary,
) -> Option<[(f64, f64); 2]> {
    let order = b.outer.as_ref()?;
    for &(i, _) in order {
        let (carrier, _) = b.edges.get(i)?.arc.as_ref()?;
        if !on_sphere(carrier, frame.origin(), radius) {
            return None;
        }
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
    let mut prev: Option<(f64, f64)> = None;
    for &(i, plus) in order {
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
    // A lift that does not close winds round the axis, from a longitude
    // to a whole number of turns past it, so its pieces span a turn or
    // more: under a turn wide, the lift closes on one sheet. The reader
    // would refuse such a hull too ([`face_extent`] reads an extent a
    // turn wide as wrapping, which gives no sheet), so this says here
    // what the extent's premise is rather than leaving it to that reading.
    if hull[0].1 - hull[0].0 >= TURN {
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

/// **Whether a carrier lies on the sphere of `radius` about `centre`,
/// exactly as it is stored**: a circle whose stored centre lies on its
/// axis line through `centre` and whose stored radius and centre's
/// distance make up the sphere's (`|c − C|² + ρ² = R²`), each to within
/// the rounding of the few products that read them ([`few_ulps`] of `R`
/// and of `R²`, below any edge tolerance). [`posed_extent`] places a row's points by
/// their latitude `asin(Z/R)`, which is the latitude only of a point on
/// the sphere; a row stored off it by an edge's tolerance (a spline
/// section, a circle of another sphere) would shift the hull by that
/// offset times the latitude's tangent, the unsound way, so it is no
/// row of a granted extent.
fn on_sphere(carrier: &Curve3<Interval>, centre: Point3<Interval>, radius: Interval) -> bool {
    let Some(c) = carrier.as_circle() else {
        return false;
    };
    let r2 = radius.powi(2);
    let off = c.frame().origin() - centre;
    let on_axis = off.cross(c.frame().axis());
    let rest = off.dot(off) + c.radius().powi(2) - r2;
    let small = |x: Interval, tol: f64| x.is_certified() && x.lo() >= -tol && x.hi() <= tol;
    let (length, area) = (few_ulps(radius.hi()), few_ulps(r2.hi()));
    [on_axis.x, on_axis.y, on_axis.z]
        .into_iter()
        .all(|x| small(x, length))
        && small(rest, area)
}

/// **Whether each pose of a sphere face holds `point` in its extent**:
/// one entry per pose of the face ([`FacePoses`]), `None` where the
/// extent the presentation reads ([`face_extent`] over the posed
/// boundary's [`Boundary::extent`]) is not known. The poses and the
/// extents are the ones [`present_chart_in`] is handed, so a mis-wired
/// posed extent is read here as it is there. A test reads it: a face
/// point a granted extent leaves out is a premise of the extent broken.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn posed_extents_holding(
    surface: &Surface<Interval>,
    b: &Boundary,
    point: Point3<Interval>,
    band: Band,
) -> Vec<Option<bool>> {
    let Some((_, radius)) = sphere_of(surface) else {
        return Vec::new();
    };
    let face = FacePoses::default();
    face.get(surface, b, band)
        .iter()
        .map(|pose| {
            let Ok(Extent::Known([lon, lat])) = face_extent(&pose.surface, &pose.boundary, band)
            else {
                return None;
            };
            let Some(((plo, phi), (tlo, thi))) = posed_cell(&pose.frame, radius, point) else {
                return Some(true);
            };
            let meets = |(a0, a1): (f64, f64), (b0, b1): (f64, f64)| a0 <= b1 && b0 <= a1;
            Some(
                meets((tlo, thi), lat)
                    && (-2..=2).any(|k| {
                        let d = f64::from(k) * TURN;
                        meets((plo + d, phi + d), lon)
                    }),
            )
        })
        .collect()
}

/// **A cell in a pose**: a rectangle of the pose's chart holding every
/// point of the sphere in the box `image`, which holds the face's part of
/// the cell. On the sphere `Z/r` is the sine of the latitude, so the
/// latitude lies in the arcsine of the box's `Z/r` clamped to `[−1, 1]`;
/// the azimuth is the box's direction in the frame, read about its middle
/// ([`geom::Azimuth::about_middle`]), within a quarter turn of it up to
/// rounding, so the rectangle is under a turn wide, which is what the
/// pose reads it by: it holds at most one copy of each point and each
/// line of the periodic axis. `None` where the box is not
/// decided off the pose's axis, about which every azimuth meets: a
/// smaller cell may be.
fn posed_cell(
    frame: &geom::Frame<Interval>,
    radius: Interval,
    image: Point3<Interval>,
) -> Option<ChartRect> {
    let s = (image - frame.origin()).dot(frame.axis()) / radius;
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
struct Posed<'f> {
    pose: &'f FacePose,
    x: (f64, f64),
    y: (f64, f64),
    ball: Ball,
    local: Local,
}

/// **The poses one face's cell is read in**, held for as long as a
/// presentation in one of them is read ([`present`]).
pub(crate) struct Poses<'f> {
    face: &'f FacePoses,
    set: std::cell::OnceCell<PoseSet<'f>>,
}

impl<'f> Poses<'f> {
    /// A cell's store, drawing on its face's poses.
    pub(crate) fn of(face: &'f FacePoses) -> Self {
        Self {
            face,
            set: std::cell::OnceCell::new(),
        }
    }

    /// A store that offers no pose: its set is empty before any cell asks,
    /// so `face` is never read and [`present`] reads the stored chart only.
    pub(crate) fn none(face: &'f FacePoses) -> Self {
        let set = std::cell::OnceCell::new();
        let _ = set.set(PoseSet::default());
        Self { face, set }
    }
}

/// **A sphere face's poses, built once per ask** ([`present_posed`]): one
/// per axis line of the face's circles that its stored chart reads as no
/// coordinate line ([`pose_circles`]), each with its frame, its sphere
/// and the face's boundary lifted into it, the posed extent included
/// ([`posed_boundary`]). They are facts of the face, so the walk keeps
/// them beside the face's boundary, and a cell selects the poses of the
/// circles in its reach ([`poses_of`]).
#[derive(Default)]
pub(crate) struct FacePoses {
    of: std::cell::OnceCell<Vec<FacePose>>,
    builds: std::cell::Cell<usize>,
}

/// A copy of a face's window starts its own store: the poses are built
/// again for it, once.
impl Clone for FacePoses {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl core::fmt::Debug for FacePoses {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FacePoses")
            .field("built", &self.of.get().map(Vec::len))
            .field("builds", &self.builds.get())
            .finish()
    }
}

/// One of a face's poses: the circles' edges it is about, its frame, its
/// sphere and the face's boundary lifted into it.
struct FacePose {
    edges: Vec<usize>,
    frame: geom::Frame<Interval>,
    surface: Surface<Interval>,
    boundary: Boundary,
}

impl FacePoses {
    /// The face's poses, each built once.
    fn get(&self, surface: &Surface<Interval>, b: &Boundary, band: Band) -> &[FacePose] {
        self.of.get_or_init(|| {
            let Some((center, radius)) = sphere_of(surface) else {
                return Vec::new();
            };
            pose_circles(surface, b, 0..b.edges.len())
                .into_iter()
                .filter_map(|(edges, axis, u_ref)| {
                    let frame = pose_frame(center, axis, u_ref)?;
                    let posed = posed_sphere(&frame, radius);
                    self.builds.set(self.builds.get() + 1);
                    let boundary = posed_boundary(&frame, &posed, radius, b, band);
                    Some(FacePose {
                        edges,
                        frame,
                        surface: posed,
                        boundary,
                    })
                })
                .collect()
        })
    }

    /// How many posed boundaries this face has built in this ask.
    pub(crate) fn builds(&self) -> usize {
        self.builds.get()
    }
}

/// **The circles a sphere face is posed about** among `edges`: each
/// circle edge of `b` that is no seam and that the stored chart reads as
/// no coordinate line ([`is_curved`]), grouped by axis line, with that
/// axis and the first such circle's seam direction. The one filter both
/// [`FacePoses`] and the containment rows read.
fn pose_circles(
    surface: &Surface<Interval>,
    b: &Boundary,
    edges: impl IntoIterator<Item = usize>,
) -> Vec<(Vec<usize>, Vec3<Interval>, Vec3<Interval>)> {
    let mut out: Vec<(Vec<usize>, Vec3<Interval>, Vec3<Interval>)> = Vec::new();
    for i in edges {
        let Some(e) = b.edges.get(i) else { continue };
        let Some(c) = e.arc.as_ref().and_then(|(c, _)| c.as_circle()) else {
            continue;
        };
        if seam(e) || !is_curved(surface, e) {
            continue;
        }
        match out
            .iter_mut()
            .find(|(_, a, _)| one_line(*a, c.frame().axis()))
        {
            Some((es, ..)) => es.push(i),
            None => out.push((vec![i], c.frame().axis(), c.frame().u_ref())),
        }
    }
    out
}

/// What [`poses_of`] finds: the poses placed, whether a pose was left
/// out for want of a placement, and whether the cell reaches a corner no
/// chart reads ([`unread_corner`]).
#[derive(Default)]
struct PoseSet<'f> {
    poses: Vec<Posed<'f>>,
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
fn poses_of<'f>(
    piece: &Piece<'_>,
    near_edges: &[usize],
    near_vertices: &[VertexKey],
    band: Band,
    face: &'f FacePoses,
) -> PoseSet<'f> {
    let Some((_, radius)) = sphere_of(piece.surface) else {
        return PoseSet::default();
    };
    let all = face.get(piece.surface, piece.boundary, band);
    // The poses of the circles in reach, in the order the cell's local
    // boundary first reaches each.
    let mut charted: Vec<&'f FacePose> = Vec::new();
    for &i in &piece.local.edges {
        if let Some(p) = all.iter().find(|p| p.edges.contains(&i))
            && !charted.iter().any(|q| core::ptr::eq(*q, p))
        {
            charted.push(p);
        }
    }
    let unread_corner = unread_corner(piece, &charted, near_vertices);
    let image = piece.surface.rect_enclosure(piece.u, piece.v);
    let mut unplaced = false;
    let mut out = Vec::new();
    for pose in charted {
        if !lines(&pose.boundary, near_edges) {
            continue;
        }
        let Some((x, y)) = posed_cell(&pose.frame, radius, image) else {
            unplaced = true;
            continue;
        };
        let m = |(l, h): (f64, f64)| geom::mid_param(l, h);
        let ball = Ball::holding(
            pose.surface.point_eval(m(x), m(y)),
            pose.surface.rect_enclosure(x, y),
        );
        let local = Local::of(&pose.boundary, ball, band);
        out.push(Posed {
            pose,
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
        Surface::Sphere { frame, radius } => Some((frame.origin(), radius)),
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
fn unread_corner(piece: &Piece<'_>, charted: &[&FacePose], near_vertices: &[VertexKey]) -> bool {
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
        at.len() > 1 && !lines(b, &at) && charted.iter().all(|p| !lines(&p.boundary, &at))
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
/// rectangle is under a turn wide, so it holds one copy of each point and
/// of each line. Its ball and local boundary are the pose's own
/// rectangle's, so every argument [`present_chart`] makes about the
/// boundary in reach is made over the rectangle it presents. A cell
/// holding a pole of the pose is not placed in it.
///
/// **The face's sheet in a pose** is its posed extent's, the posed
/// boundary's [`Boundary::extent`], read by [`sheets`] as a stored one
/// is; what it is granted on is [`posed_extent`]'s to say. A rectangle that misses it holds none
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
/// it lies on its edges ([`Along::under_graph`]).
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
    store: &'h Poses<'_>,
) -> Option<Result<Presentation<'h>, Unpresented>> {
    let set = store
        .set
        .get_or_init(|| poses_of(piece, near_edges, near_vertices, band, store.face));
    if set.unread_corner {
        return Some(Err(Unpresented::Structural));
    }
    if set.poses.is_empty() && !set.unplaced {
        return None;
    }
    for p in &set.poses {
        let posed = Piece {
            face: piece.face,
            surface: &p.pose.surface,
            whole: false,
            boundary: &p.pose.boundary,
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
/// at a convex corner are the positive axes; at a strip's end, the end's
/// other neighbour across the face caps the rectangle.
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
    let Surface::Plane { frame } = *piece.surface else {
        return Err(Unpresented::Structural);
    };
    let (origin, normal, u_ref) = (frame.origin(), frame.axis(), frame.u_ref());
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
    // A point's coordinates in the frame `o + x·a + y·b`.
    let coords =
        |o: Point2<Interval>, a: [Interval; 2], bv: [Interval; 2], pt: Point2<Interval>| {
            let det = a[0] * bv[1] - a[1] * bv[0];
            let d = [pt.x - o.x, pt.y - o.y];
            (
                (d[0] * bv[1] - d[1] * bv[0]) / det,
                (a[0] * d[1] - a[1] * d[0]) / det,
            )
        };
    // How far `w` stands along the unit of `n`.
    let lean = |n: [Interval; 2], w: [Interval; 2]| {
        (n[0] * w[0] + n[1] * w[1]) / (n[0].powi(2) + n[1].powi(2)).sqrt()
    };
    let positive =
        |x: Interval, site: &'static str| interval_sign_as(x, site, band) == Some(Sign::Positive);
    // **The face's part of the cell where the face is convex there**: the
    // intersection of the half-planes of the line edges the cell holds,
    // read inside a parallelogram `R` two of whose sides lie on one edge
    // or on a corner's two edges (the frame), and whose far sides are the
    // cell's reach or, at a strip's end, the end's other neighbour across
    // the face from them (the cap). Every point of `R` off the face is a
    // point of the plane `R`'s reading holds too, so `R` need only hold
    // the face's part. The face is convex in the cell where each corner in
    // it is convex and the cap and the edge it caps stand on each other's
    // face side over the reach: every vertex and every other edge is clear
    // of the cell's ball, so the plane's disk inside the ball meets the
    // boundary only in these edges' whole chords, and the region between
    // them, on each one's face side, is the face's part of the disk.
    //
    // Each reading decides that convexity before anything else, `Vacuous`
    // included: a cell off the frame's face side holds none of the face
    // only where nothing else in the cell bounds the face, so a cap is
    // read, and its leans decided, before the cell is placed.
    //
    // Two lines with no common end (a strip's middle) are not read: no
    // measured body needs them
    // (`work/embed/a-plane-cell-holding-both-sides-of-a-thin-face-is-read-only-below-its-width.md`).
    //
    // **The cap**: `f`, the end's other neighbour, a line across the face
    // from the frame's base line `o + x·a`, through the frame's `(0, 1)`,
    // its corner with `e2`: the highest its line stands in the frame's `y`
    // over the reach `x`, where the base stands on its face side there.
    // That lean is a distance in metres (the base's points along `f`'s
    // face normal), read at `SELF_INTERSECTION_CORNER` as a corner's lean
    // is, and linear in `x`, read in the base's own frame, so a face
    // turned in its chart reads no more slack than one along its axes.
    // The cap's line, at height 1 at `x = 0` and kept off the base's line
    // over the reach by that lean, stands above the base there too.
    let capped = |f: usize,
                  (o, a, bv): (Point2<Interval>, [Interval; 2], [Interval; 2]),
                  x: Interval|
     -> Result<f64, Unpresented> {
        let ((pf, _), (qf, _)) = segment(f)?;
        let ((x0, y0), (x1, y1)) = (coords(o, a, bv, pf), coords(o, a, bv, qf));
        let nf = toward_face([qf.x - pf.x, qf.y - pf.y], orientation);
        let base = lean(nf, [o.x - pf.x, o.y - pf.y]) + x * lean(nf, a);
        if !positive(base, SELF_INTERSECTION_CORNER) {
            return Err(Unpresented::Local);
        }
        // A height whose run along `x` is not decided bounds nothing: the
        // far side is then the cell's reach, which holds the face's part
        // too.
        let h = y0 + (y1 - y0) / (x1 - x0) * (x - x0);
        Ok(if h.is_certified() {
            h.hi()
        } else {
            f64::INFINITY
        })
    };
    // One line edge `e` as `y = 0`, the face `y ≥ 0`.
    let along = |e: usize| -> Result<Presentation<'a>, Unpresented> {
        if !piece.local.vertices.is_empty() {
            return Err(Unpresented::Local);
        }
        let ((p, ta), (q, tb)) = segment(e)?;
        let a = [q.x - p.x, q.y - p.y];
        let bv = toward_face(a, orientation);
        let len2 = a[0].powi(2) + a[1].powi(2);
        let d = [cell_u - p.x, cell_v - p.y];
        let x = (d[0] * a[0] + d[1] * a[1]) / len2;
        let y = (d[0] * bv[0] + d[1] * bv[1]) / len2;
        // Wholly on the edge's non-face side holds none of the face: the
        // disk's half off the face side is all outside the face. The cell
        // lies in that half. A `len2` holding zero leaves `y`
        // uncertified, and a zero-length edge bounds no half.
        if y.is_certified() && y.hi() <= 0.0 {
            return Ok(Presentation::Vacuous);
        }
        let len = len2.sqrt();
        let inside = (x * len).min((at(1.0) - x) * len);
        if !positive(inside, SELF_INTERSECTION_SPAN) {
            return Err(Unpresented::Local);
        }
        let zero = if near_edges.contains(&e) {
            Zero::Side(
                Side::YLo,
                Along::over(piece.surface, b, e, true, || {
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
    };
    // Two line edges at a convex corner `vtx` as the positive axes, `e1`
    // along `x`; `cap`, the line at `e2`'s far end and that vertex,
    // across the face from `e1` (a strip's end).
    //
    // The far corner, `e2`'s with the cap, is convex wherever the cap's
    // lean holds at `x = 0`: there it reads `vtx` on the cap's face side,
    // and the loop runs the cap into `e2` (or out of it) with the face on
    // one hand of both, so the cap's far end then lies on `e2`'s face
    // side.
    //
    let corner = |e1: usize,
                  e2: usize,
                  vtx: VertexKey,
                  cap: Option<(usize, VertexKey)>|
     -> Result<Presentation<'a>, Unpresented> {
        let (Some(r1), Some(r2)) = (b.edges.get(e1), b.edges.get(e2)) else {
            return Err(Unpresented::Local);
        };
        let at_a_corner = |w: VertexKey| w == vtx || cap.is_some_and(|(_, v)| w == v);
        if !piece.local.vertices.iter().all(|&w| at_a_corner(w)) {
            return Err(Unpresented::Local);
        }
        let ((p1, s1), (q1, f1)) = segment(e1)?;
        let ((p2, s2), (q2, f2)) = segment(e2)?;
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
        if !(positive(lean(n1, bv), SELF_INTERSECTION_CORNER)
            && positive(lean(n2, a), SELF_INTERSECTION_CORNER))
        {
            return Err(Unpresented::Local);
        }
        let det = a[0] * bv[1] - a[1] * bv[0];
        let d = [cell_u - o.x, cell_v - o.y];
        let x = (d[0] * bv[1] - d[1] * bv[0]) / det;
        let y = (a[0] * d[1] - a[1] * d[0]) / det;
        let z1 = near_edges.contains(&e1);
        let z2 = near_edges.contains(&e2);
        let capped_at = match cap {
            None => None,
            Some((f, end)) => {
                // A shared vertex at the far corner is on no zero side.
                if near_vertices.contains(&end) && !z2 {
                    return Err(Unpresented::Local);
                }
                let reach = iv((0.0, x.hi().max(0.0)));
                Some(capped(f, (o, a, bv), reach)?)
            }
        };
        if x.hi() <= 0.0 || y.hi() <= 0.0 {
            return Ok(Presentation::Vacuous);
        }
        let la = (a[0].powi(2) + a[1].powi(2)).sqrt();
        let lb = (bv[0].powi(2) + bv[1].powi(2)).sqrt();
        if !positive((at(1.0) - x) * la, SELF_INTERSECTION_SPAN) {
            return Err(Unpresented::Local);
        }
        let top = match capped_at {
            None => {
                if !positive((at(1.0) - y) * lb, SELF_INTERSECTION_SPAN) {
                    return Err(Unpresented::Local);
                }
                y.hi()
            }
            Some(h) => h.min(y.hi()),
        };
        // Each edge over its coordinate, 0 at the vertex and 1 at its
        // far end, out to `hi`. A side lies on its edge where `hi` is
        // decided not past the far end: a capped end's side runs to the
        // cap's height, which its key places through the far end but not
        // along `e1`, so a face whose cap leans out has points read off
        // that side past the edge.
        let on = |e: usize, (v, w): (Interval, Interval), hi: f64| {
            Along::over(piece.surface, b, e, hi <= 1.0, || {
                segment_params(v, w, (0.0, hi))
            })
        };
        let (on1, on2) = (on(e1, (v1, w1), x.hi()), on(e2, (v2, w2), top));
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
            y: (0.0, top),
            zero,
            corner: false,
            cache: Cache::default(),
        })))
    };
    let meet = |e: usize, f: usize| {
        let (r, s) = (b.edges.get(e)?, b.edges.get(f)?);
        r.ends.iter().find(|v| s.ends.contains(v)).copied()
    };
    match *edges.as_slice() {
        [e] => along(e),
        [e1, e2] => match meet(e1, e2) {
            Some(vtx) => corner(e1, e2, vtx, None),
            None => Err(Unpresented::Local),
        },
        // An edge `m` with a neighbour at each end, the one of a sliver
        // triangle's three whose ends the cell holds: read at either of
        // `m`'s corners whose other neighbour, the cap, is not shared.
        [e1, e2, e3] => {
            let in_cell = |v: VertexKey| piece.local.vertices.contains(&v);
            let ends = [(e1, e2, e3), (e2, e3, e1), (e3, e1, e2)]
                .into_iter()
                .filter_map(|(m, n1, n2)| Some((m, (n1, meet(m, n1)?), (n2, meet(m, n2)?))));
            let mut ends: Vec<_> = ends.collect();
            ends.sort_by_key(|&(_, (_, v1), (_, v2))| !(in_cell(v1) && in_cell(v2)));
            // Of a sliver triangle whose short side's ends are not both
            // in the cell, only the first such edge is read.
            let Some(&(m, (n1, v1), (n2, v2))) = ends.first() else {
                return Err(Unpresented::Local);
            };
            // Each corner decides its own convexity, so where the first
            // refuses locally the other is read. A structural refusal is
            // the edges' (an edge neither corner reads), the same from
            // either corner, and stands.
            let mut last = Err(Unpresented::Local);
            for ((n, v), cap) in [((n1, v1), (n2, v2)), ((n2, v2), (n1, v1))] {
                if near_edges.contains(&cap.0) {
                    continue;
                }
                match corner(n, m, v, Some(cap)) {
                    Err(Unpresented::Local) => last = Err(Unpresented::Local),
                    read => return read,
                }
            }
            last
        }
        _ => Err(Unpresented::Local),
    }
}

// ------------------------------------------------ the curved presentation

/// The funnel site of a curved presentation's columns: the chart
/// curve's advance along the column axis over one of its pieces, times
/// the chart's speed along that axis (an upper bound, `rect_jet`'s),
/// metres. Only a definite sign, one sign over every piece, reads the
/// curve as a graph over that axis. Also a run's straight other edge
/// ([`line_onward`]): its one segment's advance along the column axis
/// from the vertex, times the same speed, metres, which must be
/// decided the way the run goes.
pub(crate) const SELF_INTERSECTION_COLUMN: &str = "self_intersection_column";

/// How many halvings place a curved presentation's run on its columns'
/// two ends.
const NARROW: u32 = 48;

/// `p0 + pa·cos t + pb·sin t + pl·t` over the parameters `t`, enclosed:
/// one channel of a harmonic image, `[p0, pa, pb, pl]`.
fn channel_on([p0, pa, pb, pl]: [Interval; 4], t: Interval) -> Interval {
    let (s, c) = t.sin_cos();
    p0 + pa * c + pb * s + pl * t
}

/// [`channel_on`]'s derivative over `t`, enclosed.
fn channel_rate([_, pa, pb, pl]: [Interval; 4], t: Interval) -> Interval {
    let (s, c) = t.sin_cos();
    pb * c - pa * s + pl
}

impl Harmonic {
    /// One axis's channel.
    fn channel(&self, axis: Axis) -> [Interval; 4] {
        let i = axis.index();
        [self.p0[i], self.pa[i], self.pb[i], self.pl[i]]
    }

    /// The image on `axis` over the parameters `t`, enclosed.
    fn on(&self, axis: Axis, t: Interval) -> Interval {
        channel_on(self.channel(axis), t)
    }

    /// Its derivative on `axis` over `t`, enclosed.
    fn rate(&self, axis: Axis, t: Interval) -> Interval {
        channel_rate(self.channel(axis), t)
    }
}

impl Focal {
    /// **A focal image from its eight numbers**, or `None` where they do
    /// not make the azimuth monotone for every member: every number
    /// certified, `sense` exactly `±1` and `β`'s enclosure strictly inside
    /// `(−1, 1)`.
    pub(crate) fn of(image: &geom_brep::FocalImage<Interval>) -> Option<Self> {
        let sense = if exactly(image.sense, 1.0) {
            Sense::With
        } else if exactly(image.sense, -1.0) {
            Sense::Against
        } else {
            return None;
        };
        let certified = [
            image.u0, image.t0, image.v0, image.va, image.vb, image.vl, image.beta,
        ]
        .iter()
        .all(|x| x.is_certified());
        let inside = certified && -1.0 < image.beta.lo() && image.beta.hi() < 1.0;
        inside.then_some(Self {
            image: *image,
            sense,
        })
    }

    /// The image on `axis` over the parameters `t`, enclosed: the stored
    /// row's own chart box over the span
    /// (`geom_brep::Pcurve::closed_form_span_box`, the one home of the
    /// construction and of its enclosure argument: the azimuth's hull of
    /// its two end evaluations, which monotonicity makes its range, and
    /// the slant's two end values widened by the chord-dip charge), read
    /// at the interval scalar, where each end encloses the true bound for
    /// every member of the image's eight numbers. Unbounded where a bound
    /// does not certify.
    fn on(&self, axis: Axis, t: Interval) -> Interval {
        let image = geom_brep::Pcurve::FocalSection(self.image);
        let Some(window) = image.closed_form_span_box(at(t.lo()), at(t.hi())) else {
            return whole_line();
        };
        let (low, high) = match axis {
            Axis::U => (window.u_min, window.u_max),
            Axis::V => (window.v_min, window.v_max),
        };
        if low.is_certified() && high.is_certified() {
            Interval::from_bounds(low.lo(), high.hi())
        } else {
            whole_line()
        }
    }

    /// Its derivative on `axis` over `t`, enclosed: on the azimuth
    /// `±(1 − β²)/(1 − 2β·cos E + β²)`, `cos E` named once; unbounded
    /// where the denominator is not certified above zero.
    fn rate(&self, axis: Axis, t: Interval) -> Interval {
        let image = &self.image;
        match axis {
            Axis::U => {
                let c = (t - image.t0).sin_cos().1;
                let b2 = image.beta.powi(2);
                let den = at(1.0) + b2 - image.beta * c * at(2.0);
                if !(den.is_certified() && den.lo() > 0.0) {
                    return whole_line();
                }
                let rate = (at(1.0) - b2) / den;
                match self.sense {
                    Sense::With => rate,
                    Sense::Against => at(0.0) - rate,
                }
            }
            Axis::V => channel_rate([image.v0, image.va, image.vb, image.vl], t),
        }
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
            Surface::Plane { frame } => {
                let (origin, normal, u_ref) = (frame.origin(), frame.axis(), frame.u_ref());
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
                frame: carrier_frame,
                radius,
            } => {
                let (origin, axis) = (carrier_frame.origin(), carrier_frame.axis());
                let w = [1.0, -1.0].into_iter().find(|&w| exactly(h.pl[0], w))?;
                if !(exactly(h.pa[0], 0.0) && exactly(h.pb[0], 0.0) && exactly(h.pl[1], 0.0)) {
                    return None;
                }
                let frame = carrier_frame;
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
                let (w0, w1, w2) = radial(o, axis.get());
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
                let (w0, w1, w2) = radial(apex, axis.get());
                let h = |w: Vec3<Interval>| w.dot(axis.get());
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
/// face's next arc of the same conic ([`Reading::Conic`]) or along its
/// next edge (`run`), so the face has points on it off the shared cell:
/// read only where [`sign_on`] admits it, as [`Zero::Past`] is. With
/// `run`, `t` stops at the vertex's parameter, and `along` is the run's
/// part on the shared cell, which may be the other edge's.
#[derive(Clone, Debug)]
struct Curved {
    form: Form,
    t: (f64, f64),
    along: Along,
    past: Option<VertexKey>,
    /// The next edge, where the side past the vertex is that edge and
    /// not the curve continued ([`RunOn`]).
    run: Option<RunOn>,
}

/// **A corner's other edge a curved side runs on along past the
/// vertex** ([`present_corner`], "one run through the vertex"): its key,
/// and whether the vertex is the curve's start, not its end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RunOn {
    edge: EdgeKey,
    start: bool,
}

/// What a curved side's chart curve is.
#[derive(Clone, Copy, Debug)]
enum Form {
    /// A harmonic chart curve and its image on the face.
    Harmonic { chart: Harmonic, image: Trig },
    /// A spiric's wall image on a torus, and its image on the face.
    Wall(WallImage),
    /// A section ellipse's focal image on a cone. Its image on the face
    /// is read by no value: a separator is zero along it by membership
    /// alone ([`sign_on`], "Where `ψ` is zero").
    Focal(Focal),
}

impl Form {
    /// **The form [`present_curved`] reads a chart curve in on
    /// `surface`**: a harmonic row whose image is a [`Trig`]
    /// ([`Trig::of`]: on a plane or a cylinder), or a focal image on a
    /// cone. `None` for every other curve and chart: a wall image is
    /// [`present_wall`]'s, and a focal image on a torus (a Villarceau
    /// circle) is read by nothing.
    fn of(surface: &Surface<Interval>, chart: ChartCurve) -> Option<Self> {
        match (chart, surface) {
            (ChartCurve::Harmonic(chart), _) => {
                Trig::of(surface, &chart).map(|image| Self::Harmonic { chart, image })
            }
            (ChartCurve::Focal(f), Surface::Cone { .. }) => Some(Self::Focal(f)),
            (ChartCurve::Focal(_) | ChartCurve::Wall(_), _) => None,
        }
    }

    /// The chart curve.
    fn chart(&self) -> ChartCurve {
        match *self {
            Self::Harmonic { chart, .. } => ChartCurve::Harmonic(chart),
            Self::Wall(w) => ChartCurve::Wall(w.wall),
            Self::Focal(f) => ChartCurve::Focal(f),
        }
    }
}

impl Curved {
    /// `ψ` along the side, bounded ([`Trig::psi`], [`WallImage::psi`]).
    fn psi(&self, psi: &Psi) -> Option<Interval> {
        let t = iv(self.t);
        match self.form {
            Form::Harmonic { image, .. } => image.psi(psi, t),
            Form::Wall(w) => w.psi(psi, t),
            Form::Focal(_) => None,
        }
    }

    /// The chart point at the middle of the side's parameters.
    fn middle(&self) -> (Interval, Interval) {
        let t = at(geom::mid_param(self.t.0, self.t.1));
        let [u, v] = self.form.chart().at(t);
        (u, v)
    }
}

/// A conic carrier as a [`Trig`]: `center + u_ref·a·cos t + v_ref·b·sin t`.
fn conic(c: &Curve3<Interval>) -> Option<Trig> {
    let (center, axis, a, b, x) = match *c {
        Curve3::Circle(circle) => {
            let (center, axis, u_ref) = (
                circle.frame().origin(),
                circle.frame().axis(),
                circle.frame().u_ref(),
            );
            let radius = circle.radius();
            (center, axis, radius, radius, u_ref)
        }
        Curve3::Ellipse(ellipse) => {
            let (center, axis, u_ref) = (
                ellipse.frame().origin(),
                ellipse.frame().axis(),
                ellipse.frame().u_ref(),
            );
            let (semi_u, semi_v) = (ellipse.semi_u(), ellipse.semi_v());
            (center, axis, semi_u, semi_v, u_ref)
        }
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
/// any other face's stored closed form. Which of these a face reads is
/// [`Form::of`]'s to say, and nobody else's.
fn chart_curve(surface: &Surface<Interval>, rim: &Rim, u: &Use) -> Option<ChartCurve> {
    match surface {
        Surface::Plane { .. } => plane_conic(surface, rim).map(ChartCurve::Harmonic),
        _ => u.curve,
    }
}

/// A plane's image of a conic carrier, through its affine chart.
fn plane_conic(surface: &Surface<Interval>, rim: &Rim) -> Option<Harmonic> {
    let Surface::Plane { frame } = *surface else {
        return None;
    };
    let (origin, normal, u_ref) = (frame.origin(), frame.axis(), frame.u_ref());
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
        .is_some_and(|c| Form::of(surface, c).is_some()))
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
        start,
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

/// **Whether a rectangle `x × y` of a cone's chart is one the curved
/// readings are argued on**: off the apex (`v = 0`, where every `u` is
/// one point, so the chart is no longer one-to-one and a chart length
/// along `u` is no length at all) and under a turn wide in `u`. There
/// the chart is one-to-one and orthogonal, `|S_v| = 1` and
/// `|S_u| = |v|·sin α`, as [`present_curved`]'s and
/// [`present_line_corner`]'s region arguments and [`on_vertex`]'s metre
/// bound take it to be. Every other chart is such a rectangle wherever
/// those readings read it: a cylinder's and a ring torus's never
/// collapse, and each reading refuses a rectangle a turn wide on its own
/// terms.
fn cone_regular(surface: &Surface<Interval>, x: (f64, f64), y: (f64, f64)) -> bool {
    match surface {
        Surface::Cone { .. } => (y.0 > 0.0 || y.1 < 0.0) && x.1 - x.0 < TURN,
        _ => true,
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
/// vertex onto the face's side. Where it is not decided to, the curve
/// stops at the vertex and the other edge, straight in the chart, is
/// read on from it: one run, the zero `past` the vertex
/// ([`Curved::run`]). [`present_corner`] argues both regions. A column
/// axis over which the run is no graph does not stop the other being
/// tried.
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
    let Some(form) = chart_curve(piece.surface, e, u).and_then(|c| Form::of(piece.surface, c))
    else {
        return Err(unread(ei, near_edges));
    };
    let chart = form.chart();
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
        chart
            .wind(axis)
            .is_some_and(|k| k == 0 || periodic(piece.surface, axis))
    });
    let continued = e.ends[0] == e.ends[1] && whole_turn(t0, t1) && lattice;
    // The curve is the zero: the shared edge, or at a vertex the pair
    // shares alone the arc read on through it.
    let zeroed =
        corner.is_some_and(|c| c.alone || c.run) || sharing(ei, near_edges, near_vertices)?;
    let through = through_of(piece.boundary, e, corner, (t0, t1))?;
    let mut unread = Unpresented::Local;
    for column in [Axis::U, Axis::V] {
        let span = through.as_ref().map_or((t0, t1), |th| th.span);
        let pieces = Pieces::of(&chart, column, span, continued);
        let Graph {
            x,
            y,
            above,
            t,
            run,
        } = match graph(piece, ei, u.plus, &pieces, through.as_ref(), band) {
            Ok(g) => g,
            Err(Unpresented::Local) => continue,
            // No graph over this axis at any depth: the other may be.
            Err(Unpresented::Structural) => {
                unread = Unpresented::Structural;
                continue;
            }
            Err(e) => return Err(e),
        };
        // A run through the vertex: the part of it on the shared cell is
        // the curve's, or the straight edge's it gives way to.
        let running = through.as_ref().zip(run).and_then(|(th, share)| {
            let o = piece.boundary.edges.get(th.corner.other)?;
            Some(if th.corner.run {
                let (line, (t0, t1)) = o.arc.as_ref()?;
                let start = o.ends[0] == th.corner.vertex;
                let (tv, tf) = if start { (*t0, *t1) } else { (*t1, *t0) };
                let along = Along::over(
                    piece.surface,
                    piece.boundary,
                    th.corner.other,
                    false,
                    || segment_params(tv, tf, (0.0, share)).filter(|_| !line.is_curved()),
                );
                (Some(along), RunOn { edge: e.key, start })
            } else {
                (
                    None,
                    RunOn {
                        edge: o.key,
                        start: th.start,
                    },
                )
            })
        });
        if run.is_some() && running.is_none() {
            return Err(Unpresented::Local);
        }
        let (shared, run) = match running {
            Some((along, run)) => (along, Some(run)),
            None => (None, None),
        };
        let zero = if zeroed {
            Zero::Curve(
                Side::of(column.other(), above),
                Box::new(Curved {
                    form,
                    t,
                    along: shared.unwrap_or(Along {
                        edges: vec![e.key],
                        vertex: None,
                        whole: matches!(carrier, Curve3::Circle(_) | Curve3::Ellipse(_)),
                        params: Some(Params { t, periodic: true }),
                        on_edges: false,
                        stops: Vec::new(),
                    }),
                    past: through
                        .as_ref()
                        .filter(|th| th.conic || run.is_some())
                        .map(|th| th.corner.vertex),
                    run,
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
    Err(unread)
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

impl ChartCurve {
    /// The image on `axis` over the parameters `t`, enclosed.
    fn on(&self, axis: Axis, t: Interval) -> Interval {
        match self {
            Self::Harmonic(h) => h.on(axis, t),
            Self::Wall(w) => w.on(axis, t),
            Self::Focal(f) => f.on(axis, t),
        }
    }

    /// The chart point at `t`, or the image over the parameters `t`,
    /// enclosed, `[u, v]`.
    pub(crate) fn at(&self, t: Interval) -> [Interval; 2] {
        [self.on(Axis::U, t), self.on(Axis::V, t)]
    }

    /// Its derivative on `axis` over `t`, enclosed.
    fn rate(&self, axis: Axis, t: Interval) -> Interval {
        match self {
            Self::Harmonic(h) => h.rate(axis, t),
            Self::Wall(w) => w.rate(axis, t),
            Self::Focal(f) => f.rate(axis, t),
        }
    }

    /// **How many whole turns one period of the parameter moves the image
    /// on `axis`**: a harmonic channel's linear coefficient where it is
    /// exactly `0` or `±1`, a focal image's sense on its azimuth. `None`
    /// where the period moves it by no whole number of turns, and for a
    /// wall image, which [`present_wall`] continues itself.
    fn wind(&self, axis: Axis) -> Option<i64> {
        let whole = |l: Interval| {
            [(0.0, 0_i64), (1.0, 1), (-1.0, -1)]
                .into_iter()
                .find_map(|(x, k)| exactly(l, x).then_some(k))
        };
        match self {
            Self::Harmonic(h) => whole(h.pl[axis.index()]),
            Self::Focal(f) => match axis {
                Axis::U => Some(f.sense.sign()),
                Axis::V => whole(f.image.vl).filter(|&k| k == 0),
            },
            Self::Wall(_) => None,
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
            frame,
            major_radius,
            minor_radius,
        } = *surface
        else {
            return None;
        };
        let (center, axis, u_ref) = (frame.origin(), frame.axis(), frame.u_ref());
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
        (Surface::Torus { .. }, [u]) => match u.curve {
            Some(ChartCurve::Wall(w)) => Some(w),
            _ => None,
        },
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
                    stops: Vec::new(),
                },
                past: None,
                run: None,
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
/// corner convex, by the read edge alone; a definite `Negative` is a
/// reflex corner; anything else, or a reflex corner, is read only as one
/// run of the two edges ([`present_corner`]), or by nothing.
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
/// a cylinder's), an upper bound on a torus's and on a cone's, whose
/// speed along `u` is `|v|·sin α` and is taken at the rectangle's far
/// slant. Every chart read through a
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
/// `read` the one read through the vertex and `other` the one that
/// leaves it. `read` is the wall on the shared cell, except under `run`,
/// where it is the curve the shared line gives way to. `alone`: the
/// shared cell is `vertex` alone, neither wall on it, and `read` is the
/// one of lower key.
#[derive(Clone, Copy, Debug)]
struct Corner {
    vertex: VertexKey,
    read: usize,
    other: usize,
    alone: bool,
    /// `other` is the wall on the shared cell, a plane's line edge, and
    /// `read` the curve it gives way to at the vertex: the two are read
    /// as one run ([`present_corner`], "one run through the vertex").
    run: bool,
}

/// A curve read through a corner: the corner, the curve's parameter at
/// the vertex, the span continued past it, and whether the corner's
/// other edge is an arc of the curve's own conic ([`Reading::Conic`]).
struct Through {
    corner: Corner,
    /// The vertex is the curve's start, not its end.
    start: bool,
    t: f64,
    span: (f64, f64),
    conic: bool,
}

/// **The pair's one shared vertex**: the shared vertex, where the cells
/// are near exactly one. Read with no shared edge near, it is the shared
/// cell alone ([`corner_of`], [`graph_of`]); with shared edges near, the
/// vertex [`departure`]'s last separator is built through. With two or
/// more, none is chosen: reading at the first would be as sound (the
/// argument at that separator's site names no one vertex), but nothing
/// measured needs it and no row reads it.
fn one_shared_vertex(vertices: &[VertexKey]) -> Option<VertexKey> {
    match vertices {
        &[v] => Some(v),
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
            run: false,
        }),
        (false, true) => Some(Corner {
            vertex,
            read: f,
            other: e,
            alone: false,
            run: false,
        }),
        (false, false)
            if near_edges.is_empty() && one_shared_vertex(near_vertices) == Some(vertex) =>
        {
            // The arc read is the one of lower key, so the reading does not
            // depend on the order the walls come in.
            let (read, other) = if re.key <= rf.key { (e, f) } else { (f, e) };
            Some(Corner {
                vertex,
                read,
                other,
                alone: true,
                run: false,
            })
        }
        _ => None,
    }
}

/// **Present a cell at a corner** ([`corner_of`]): by its shared edge
/// alone, read through the vertex (a curve by its closed form continued,
/// [`present_curved`]; a coordinate line of a chart by the line,
/// [`present_line_corner`]; a plane's line edge by its line,
/// [`present_plane_corner`]), or, where a curve's straight other edge is
/// not decided to leave onto its face side, by the two edges as one run
/// through the vertex (below). `None` where none reads it.
///
/// **Why the face's part of the cell lies on the read edge's face
/// side**, for `R` the presentation's rectangle, which holds the vertex
/// `V`, and `D` the points of `R` across the read edge `E` — its line,
/// or its curve's graph, continued past `V` — from the face:
///
/// 1. Nothing in reach of `R` bounds the face but `E` and the other
///    edge `O` ([`alone`], or [`present_line_corner`]'s own reading),
///    and `O` leaves `V` strictly into `E`'s face side and stays there
///    within `R` ([`leaving`], [`leaves_line`]). So `D`, and the
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
/// (reflex) is read by neither of these: by a curve as one run (below),
/// or by nothing, at any depth: `Structural`.
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
///
/// **One run through the vertex** ([`RunOn`]): where `O` is not decided
/// to leave onto `E`'s face side ([`Leaves`]) — it continues `E` at
/// first order, as a fillet's straight foot continues its arc, or leaves
/// onto the other side — `E` continued bounds nothing, and steps 1 to 3
/// above do not hold. `E` up to `V` and `O` from it are read together:
///
/// 1. `E`'s chart curve is a graph over the column axis up to `V`
///    ([`graph_read`]'s step 1), and `O` runs on from `V` across the
///    columns the same way, past their far end ([`line_onward`]): it is
///    straight in the chart, one segment. So every column holds exactly one point of the
///    run: `E`'s before `V`'s column, `O`'s after it.
/// 2. `R` holds `O`'s reach over the columns it crosses, and nothing in
///    reach of `R` bounds the face but `E`, `O` and `E`'s ends
///    ([`alone`]); no other copy of `O` meets `R`.
/// 3. So the boundary in `R` is the run, traversed one way through `V`,
///    and `R` off it is two connected parts, the face the one `E`'s
///    traversal names: every point of the face in the cell is on that
///    side of the run, or on it.
///
/// No angle between the two edges is read, so none is decided: the
/// reading is the same for a joint that is tangent, tangent within the
/// band, or reflex. The run is read by the curve of the two (`read`),
/// from either edge: where the shared edge is a plane's line and the
/// curve the other ([`Corner::run`]), the part of the run on the shared
/// cell is the line's. The zero is the run, `past` `V`: its part along
/// the edge that is not shared holds face points off the shared cell,
/// read only where [`past_read`] admits it and only under a graph
/// separator run on along that edge ([`graph_run`]).
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
        Reading::PlaneLine(o) => present_plane_corner(piece, c, o, band).or_else(|unread| {
            // The conic is not decided onto the line's face side: the two
            // as one run, read by the conic.
            let run = Corner {
                read: c.other,
                other: c.read,
                run: true,
                ..c
            };
            let curve = piece.boundary.edges.get(c.other);
            if !curve.is_some_and(|r| reads_curve(piece.surface, r)) {
                return Err(unread);
            }
            present_curved(piece, c.other, Some(run), near_edges, near_vertices, band).map_err(
                // A run a smaller cell may read; one no cell reads leaves
                // the corner as the line's reading typed it.
                |e| match e {
                    Unpresented::Local => Unpresented::Local,
                    _ => unread,
                },
            )
        }),
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
    /// the other edge a wall image; or a ruling or a rim's arc of a
    /// cone's, the other edge a section ellipse's focal image.
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
        let [_] = o.uses.as_slice() else {
            return None;
        };
        if !curved(o) || piece.whole {
            return None;
        }
        plane_conic(s, o).map(Reading::PlaneLine)
    } else {
        // The argument is made on a rectangle the chart maps one-to-one
        // with no collapse: any of a cylinder's or a ring torus's, and a
        // cone's off its apex and under a turn wide ([`cone_regular`],
        // which the line reading asks of its rectangle). On a sphere a
        // circle is read in the pose that makes it a coordinate line
        // ([`present_posed`]), and a curve no chart makes one is not
        // argued here. On a torus the line is a parallel, and the curve a
        // wall image, whose `v` is linear in its parameter: that is what
        // [`leaves_line`]'s rate on the held axis reads. A meridian, along
        // which the wall's azimuth can turn, and a harmonic curve, which
        // can turn on both axes, are not argued. On a cone the line is a
        // ruling or a rim's arc and the curve a section ellipse's focal
        // image.
        let line = matches!(e.uses.as_slice(), [u] if u.iso.is_some_and(|i| !i.full));
        if !(line && curved(o)) || piece.whole {
            return None;
        }
        let curve = match o.uses.as_slice() {
            [u] => u.curve,
            _ => None,
        };
        match (s, curve) {
            (Surface::Cylinder { .. }, Some(c @ ChartCurve::Harmonic(_)))
            | (Surface::Cone { .. }, Some(c @ ChartCurve::Focal(_))) => Some(Reading::Line(c)),
            (Surface::Torus { .. }, Some(c @ ChartCurve::Wall(_))) if parallel(e) => {
                Some(Reading::Line(c))
            }
            _ => None,
        }
    }
}

/// **Whether two edges' carriers are one conic, as one value**: a
/// point-scalar body's stored carriers one conic ([`one_stated_conic`]),
/// or their lifts one plane through one centre ([`one_plane`]) and the
/// same radius, and on an ellipse the same semi-axes along the same lines
/// (`u_ref` equal or opposed, a half-turn of the parameter). Every
/// comparison of the lifts is [`one_value`]'s, so a component held as a
/// bracket wider than a point is never one conic, even against itself:
/// the lifts are one value only where the frames are exactly rigid,
/// where a frame's axis is the one value of its polar factor. The two
/// parametrizations may differ; the reading never maps one onto the
/// other ([`continues`]).
fn one_conic(e: &Rim, o: &Rim) -> bool {
    if let (Some(se), Some(so)) = (e.stated.as_ref(), o.stated.as_ref())
        && one_stated_conic(se, so)
    {
        return true;
    }
    let (Some((ce, _)), Some((co, _))) = (e.arc.as_ref(), o.arc.as_ref()) else {
        return false;
    };
    let plane = |f: &geom::Frame<Interval>, g: &geom::Frame<Interval>| {
        one_plane((f.origin(), f.axis()), (g.origin(), g.axis()))
    };
    match (ce, co) {
        (Curve3::Circle(a), Curve3::Circle(b)) => {
            plane(&a.frame(), &b.frame()) && one_value(a.radius(), b.radius(), false)
        }
        (Curve3::Ellipse(a), Curve3::Ellipse(b)) => {
            plane(&a.frame(), &b.frame())
                && one_line(a.frame().u_ref(), b.frame().u_ref())
                && one_value(a.semi_u(), b.semi_u(), false)
                && one_value(a.semi_v(), b.semi_v(), false)
        }
        _ => false,
    }
}

/// [`one_conic`] on two carriers as a point-scalar body stores them: two
/// circles whose stored frames place one plane ([`one_stated_plane`]) and
/// whose radii are one; two ellipses stated alike
/// ([`Curve3::same_content`]: one placement, held at one point and phase,
/// one pair of semi-axes) with one `v` beside, which that comparison does
/// not read. An ellipse held on its arc derives its centre from its
/// anchor, rounded, so two of them may round to one centre from two
/// anchors whose true centres differ: their frames, read at the centre,
/// are no stored fact, and only their stored content is.
fn one_stated_conic(e: &Curve3<f64>, o: &Curve3<f64>) -> bool {
    match (e, o) {
        (Curve3::Circle(a), Curve3::Circle(b)) => {
            one_stated_plane(&a.frame(), &b.frame()) && a.radius() == b.radius()
        }
        (Curve3::Ellipse(a), Curve3::Ellipse(b)) => {
            e.same_content(o)
                && a.frame()
                    .motion()
                    .v()
                    .get()
                    .equals_exactly(b.frame().motion().v().get())
        }
        _ => false,
    }
}

/// **Whether two stored `f64` frames place one plane as their lifts
/// read it**: one origin, and each of the second's axes `u`, `v`, `w` the
/// first's or its negation, as numbers (a signed zero is its negation).
/// A stored frame is lifted as the enclosure of its polar factor, the
/// rotation nearest its stored axes; the polar factor of `M·D`, with `D`
/// a diagonal of signs, is `M`'s times `D`, so the two lifts enclose one
/// origin and one axis line, the one value each holds. The axis `w`
/// alone would not do: the polar factor's axis leans on all three stored
/// axes wherever they are not exactly rigid, and two frames with one `w`
/// and a `u` an ulp apart lift to disjoint enclosures of `w`. The sign
/// patterns admitted are wider than the argument needs: one of
/// determinant −1 would make one of two stored frames a reflection,
/// which no stored frame is (a reflection lifts to poison).
fn one_stated_plane(f: &geom::Frame<f64>, g: &geom::Frame<f64>) -> bool {
    let (m, n) = (f.motion(), g.motion());
    f.origin().equals_exactly(g.origin())
        && [(m.u(), n.u()), (m.v(), n.v()), (m.w(), n.w())]
            .into_iter()
            .all(|(a, b)| a.get().equals_exactly(b.get()) || a.get().equals_exactly(-b.get()))
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
    let (p, q, d) = if let Surface::Plane { frame } = *surface {
        let (origin, normal, u_ref) = (frame.origin(), frame.axis(), frame.u_ref());
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

/// How a corner's other edge leaves the curve read through the vertex
/// ([`leaving`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Leaves {
    /// Strictly onto the curve's face side.
    Face,
    /// Strictly onto its other side: a reflex corner.
    Reflex,
    /// Neither is decided: the two edges may meet at first order.
    Undecided,
}

/// **A corner's other edge, straight in the chart, on the copy through
/// the vertex** ([`leaving`]): its direction away from the vertex, its
/// chart points there and at its far end, the chart's speeds over `R`,
/// and how it leaves the curve.
struct Leaving {
    d: [Interval; 2],
    near: [Interval; 2],
    far: [Interval; 2],
    speed: [f64; 2],
    leaves: Leaves,
}

/// **No copy of the segment `near`–`far` by whole turns but `k` meets
/// `r`.** Another copy meets `R` only where the edge, a segment from the
/// vertex, comes round by a turn to a range of `R` that holds the
/// vertex's: its far end then lies in `R`, a vertex [`alone`] refuses.
/// Defence for the readings; not load-bearing on them.
fn no_other_copy(
    surface: &Surface<Interval>,
    [near, far]: [Point2<Interval>; 2],
    k: [f64; 2],
    r: ((f64, f64), (f64, f64)),
) -> Option<()> {
    let reach = reach_of([[near.x, near.y], [far.x, far.y]].into_iter());
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
    Some(())
}

/// **How the corner's other edge leaves the curve read through it**
/// ([`present_corner`]'s step 1, for a curve): it is straight in the
/// chart, every copy of it but the one through the vertex misses `R`,
/// its end is the curve's point at the vertex ([`on_vertex`]), and over
/// the parameters `ts` the columns hold, the curve's chart tangent
/// `τ = (τ_a, τ_b)`, `a` the column axis, and the edge's direction `d`
/// make `(d_b·τ_a − d_a·τ_b)·sign τ_a` one strict sign: the face's
/// side's ([`Leaves::Face`]), or the other ([`Leaves::Reflex`]).
/// That is the derivative of the edge's height over the graph along it,
/// which is zero at the vertex, so an edge that leaves onto the face's
/// side stands strictly there wherever it is in `R`. `None` where the
/// edge is not read at all.
#[allow(clippy::too_many_arguments)]
fn leaving(
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
) -> Option<Leaving> {
    let surface = piece.surface;
    let o = piece.boundary.edges.get(th.corner.other)?;
    let (d, ends) = straight(surface, o, th.corner.vertex)?;
    let [near, far] = ends;
    let k = turns_to(periodic_axes(surface), [near.x, near.y], vertex);
    no_other_copy(surface, ends, k, r)?;
    let speed = speeds(surface, r.0, r.1);
    let (near, far) = (shifted([near.x, near.y], k), shifted([far.x, far.y], k));
    if !on_vertex(near, vertex, speed, band) {
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
    let leaves = match decide(SELF_INTERSECTION_CORNER_DEPARTS, Margin::of(rate), band) {
        Ok(Sign::Positive) => Leaves::Face,
        Ok(Sign::Negative) => Leaves::Reflex,
        _ => Leaves::Undecided,
    };
    Some(Leaving {
        d,
        near,
        far,
        speed,
        leaves,
    })
}

/// **The corner's other edge leaves the curve read through it toward
/// the face's side** ([`leaving`]), for a reading with no run through
/// the vertex ([`present_wall`]): `None` where this is not decided; a
/// reflex corner, decided, is `Structural`.
#[allow(clippy::too_many_arguments)]
fn leaves_graph(
    piece: &Piece<'_>,
    th: &Through,
    column: Axis,
    tau: (Interval, Interval),
    ts: Interval,
    rising: bool,
    above: bool,
    vertex: [Interval; 2],
    r: ((f64, f64), (f64, f64)),
    band: Band,
) -> Option<Result<(), Unpresented>> {
    match leaving(piece, th, column, tau, ts, rising, above, vertex, r, band)?.leaves {
        Leaves::Face => Some(Ok(())),
        Leaves::Reflex => Some(Err(Unpresented::Structural)),
        Leaves::Undecided => None,
    }
}

/// **What a run's other edge adds to `R`** ([`graph_read`]): its reach
/// on the other axis over the columns it crosses, from the vertex to
/// their far end, and, for a straight edge, no less than the share of it
/// crossed.
struct Onward {
    reach: Interval,
    share: f64,
}

/// **A straight edge run on from the vertex across the columns `a`**
/// ([`present_corner`], "one run through the vertex"): its advance along
/// the column axis is decided, `onward` or not, as a curve's pieces are
/// ([`SELF_INTERSECTION_COLUMN`]), and its far end lies past the
/// columns' far end. `Structural` where it does not advance that way,
/// which no smaller cell changes; `Local` where it ends inside the
/// columns.
fn line_onward(
    l: &Leaving,
    column: Axis,
    onward: bool,
    (a0, a1): (f64, f64),
    band: Band,
) -> Result<Onward, Unpresented> {
    let other = column.other();
    let step = column.of((l.d[0], l.d[1]));
    let wanted = if onward {
        Sign::Positive
    } else {
        Sign::Negative
    };
    let advance = step * at(l.speed[column.index()]);
    if interval_sign_as(advance, SELF_INTERSECTION_COLUMN, band) != Some(wanted) {
        return Err(Unpresented::Structural);
    }
    // An edge that ends inside the columns ends at a vertex in `R`, which
    // [`alone`] refuses: this states the run's premise, and is not
    // load-bearing beside it.
    let end = column.of((l.far[0], l.far[1]));
    let passes = if onward {
        end.lo() >= a1
    } else {
        end.hi() <= a0
    };
    let exit = at(if onward { a1 } else { a0 });
    let share = ((exit - column.of((l.near[0], l.near[1]))) / step).hi();
    if !(passes && share.is_finite() && share >= 0.0) {
        return Err(Unpresented::Local);
    }
    let share = share.min(1.0);
    let reach = other.of((l.near[0], l.near[1])) + other.of((l.d[0], l.d[1])) * iv((0.0, share));
    if !Real::is_bounded(reach) {
        return Err(Unpresented::Local);
    }
    Ok(Onward { reach, share })
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
    }) || !cone_regular(surface, x, y)
    {
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
    let &Surface::Plane { frame } = surface else {
        return Err(Unpresented::Local);
    };
    let (origin, normal, u_ref) = (frame.origin(), frame.axis(), frame.u_ref());
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
    // A focal image at a vertex the pair shares alone is read by no
    // quadrant
    // (`work/embed/a-torus-or-cone-corner-at-a-vertex-alone-reads-no-quadrant.md`).
    let ChartCurve::Harmonic(h) = chart_curve(surface, ro, u)? else {
        return None;
    };
    Some((v, line, other, h))
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
/// [`leaving`]'s straight edge is one segment, read by its hull.
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
/// whole turns on the column axis ([`ChartCurve::wind`]: a harmonic
/// channel's linear part, a focal image's sense on its azimuth).
struct Pieces<'c> {
    chart: &'c ChartCurve,
    column: Axis,
    t: Vec<Interval>,
    width: f64,
    continued: bool,
    wind: i64,
}

impl<'c> Pieces<'c> {
    /// `continued` only where the period moves the image by whole turns
    /// on both axes ([`ChartCurve::wind`]): the column's is `wind`.
    fn of(chart: &'c ChartCurve, column: Axis, (t0, t1): (f64, f64), continued: bool) -> Self {
        let wind = chart.wind(column).unwrap_or(0);
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
    /// Where the graph runs on along the corner's other edge past the
    /// vertex ([`RunOn`]), how much of that edge, from the vertex, the
    /// columns hold: no more than this share of it. `t` then stops at the
    /// vertex.
    run: Option<f64>,
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

/// [`graph`], `None` where the curve is not read as one. Through a
/// corner whose other edge is decided onto the curve's face side
/// ([`leaving`]) the curve is read alone, continued past the vertex.
/// Where it is not decided so (reflex, or no rate decided) and the
/// other edge is straight in the chart, the two are read as one run
/// ([`line_onward`]; [`present_corner`], "one run through the vertex"),
/// `Graph::run` the vertex's parameter. A line that is no graph with
/// the curve over this column is `Some(Err(Structural))` where the
/// corner is decided reflex or is read from its line ([`Corner::run`]),
/// and `None` where no rate was decided; an other edge that is not
/// straight in the chart is `None`.
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
    // `R` over the range `rb` on the other axis, where nothing is in it
    // but the curve, and through a corner its other edge.
    let clear = |rb: (f64, f64)| {
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
        if !cone_regular(surface, x, y) {
            return None;
        }
        let local = if rb == (b0, b1) && cell == own {
            piece.local.clone()
        } else {
            let m = |(l, h): (f64, f64)| geom::mid_param(l, h);
            let ball = Ball::holding(surface.point_eval(m(x), m(y)), surface.rect_enclosure(x, y));
            Local::of(piece.boundary, ball, band)
        };
        alone(piece, ei, through.map(|th| th.corner.other), &local).then_some((x, y))
    };
    let rb = (b0.min(g.lo()), b1.max(g.hi()));
    let (x, y) = clear(rb)?;
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
        if !th.conic {
            let o = piece.boundary.edges.get(th.corner.other)?;
            // An edge that runs on from the vertex crosses the columns the
            // way the curve came to it.
            let onward = rising != th.start;
            let run = if let Some((_, ends)) = straight(surface, o, th.corner.vertex) {
                let l = leaving(
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
                )?;
                if th.corner.run || l.leaves != Leaves::Face {
                    // The other edge is not decided onto the face's side
                    // of the curve continued: the two are one run. One
                    // that is no graph with the curve over this axis is
                    // none at any depth: the corner stays unread, as it
                    // was typed.
                    match line_onward(&l, column, onward, (a0, a1), band) {
                        Ok(on) => {
                            let k =
                                turns_to(periodic_axes(surface), [ends[0].x, ends[0].y], vertex);
                            Some((on, ends, k))
                        }
                        Err(Unpresented::Local) => return None,
                        Err(e) => {
                            return (th.corner.run || l.leaves == Leaves::Reflex).then_some(Err(e));
                        }
                    }
                } else {
                    None
                }
            } else {
                // [`reading`] gives a curve this reading only beside an
                // edge straight in the chart.
                return None;
            };
            if let Some((on, ends, k)) = run {
                let (x, y) = clear((rb.0.min(on.reach.lo()), rb.1.max(on.reach.hi())))?;
                // Defence, as in [`leaving`]: on the grown `R` too.
                no_other_copy(surface, ends, k, (x, y))?;
                // The curve's own part of the run: its parameters up to
                // the vertex's.
                let t = if th.start { (th.t, tb) } else { (ta, th.t) };
                return (t.0 < t.1).then_some(Ok(Graph {
                    x,
                    y,
                    above,
                    t,
                    run: Some(on.share),
                }));
            }
        }
    }
    Some(Ok(Graph {
        x,
        y,
        above,
        t: (ta, tb),
        run: None,
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
        axis: UnitVec3<Interval>,
        r: Interval,
    },
    Sphere {
        c: Point3<Interval>,
        r: Interval,
    },
    Cone {
        apex: Point3<Interval>,
        axis: UnitVec3<Interval>,
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

    /// **`ψ`'s rate along `v` over a box**: `∇ψ(q)·v` for every `q` in
    /// `p`, each arm spelled so a quantity the true rate does not depend
    /// on is not named twice:
    ///
    /// - plane and linear: `n·v`, exact;
    /// - cylinder: the radial unit against `v` less its part along the
    ///   axis ([`square_to`]). The radial unit is square to the axis at
    ///   every point, so the two dots agree; a box's enclosure of it holds
    ///   directions with an axial part, which a dot with `v` itself would
    ///   read, and along a ruling, where `v` is the axis, that part is all
    ///   of it. Factoring the norm out (`offset·square_to / |offset|`) is
    ///   tighter across and radially and looser along the ruling, the
    ///   reading this spelling exists for;
    /// - cone: the cylinder's term times `cos`, less `nappe·sin` times
    ///   `v`'s part along the unit axis, `axis·v/|axis|`, as [`Self::value`]
    ///   reads the height (a stored axis is unit only to its convention);
    /// - sphere: `(w·v)/|w|`, `w = p − c`, naming each component of `w`
    ///   once in the dot where `w/|w|` names it with the norm;
    /// - graph: `m·v − (t·v)·γ′`, the slope named once: one real slope
    ///   makes the two equal, and `(m − t·γ′)·v` names it per component.
    fn rate(&self, p: Point3<Interval>, v: Vec3<Interval>) -> Interval {
        let radial = |w: Vec3<Interval>| w * (at(1.0) / w.norm());
        match *self {
            Self::Linear { n, .. } | Self::Carrier(Implicit::Plane { n, .. }) => n.dot(v),
            Self::Carrier(Implicit::Cylinder { o, axis, .. }) => {
                let line = geom::Line {
                    origin: o,
                    dir: axis,
                };
                radial(line.offset(p)).dot(square_to(axis.get(), v))
            }
            Self::Carrier(Implicit::Sphere { c, .. }) => {
                let w = p - c;
                w.dot(v) / w.norm()
            }
            Self::Carrier(Implicit::Cone {
                apex,
                axis,
                cos,
                sin,
                nappe,
            }) => {
                let line = geom::Line {
                    origin: apex,
                    dir: axis,
                };
                radial(line.offset(p)).dot(square_to(axis.get(), v)) * cos
                    - axis.get().dot(v) / axis.get().norm() * (nappe * sin)
            }
            Self::Graph { m, t, slope, .. } => m.dot(v) - t.dot(v) * slope,
        }
    }

    /// **`|∇ψ|` over a box**: 1 for a cylinder's and a sphere's carrier
    /// (a distance less a radius), `√(cos² + sin²)` for a cone's (its
    /// gradient is `cos` times a unit square to the unit axis, less
    /// `nappe·sin` times that axis), `|n|` for a plane and a linear
    /// separator, and the graph's `|m − t·γ′|` over its slope. A reader of
    /// the angle between `∇ψ` and a direction divides [`Self::rate`] by
    /// this and the direction's norm: a vector gradient read over a box
    /// would bring back the slop `rate` keeps out.
    fn gradient_norm(&self) -> Interval {
        match *self {
            Self::Linear { n, .. } | Self::Carrier(Implicit::Plane { n, .. }) => n.norm(),
            Self::Carrier(Implicit::Cylinder { .. } | Implicit::Sphere { .. }) => at(1.0),
            Self::Carrier(Implicit::Cone { cos, sin, .. }) => (cos.powi(2) + sin.powi(2)).sqrt(),
            Self::Graph { m, t, slope, .. } => (m - t * slope).norm(),
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

/// **`v` less its part along `axis`**: `v − axis·(axis·v)/|axis|²`. A
/// vector square to the axis has the same dot with it as with `v`.
///
/// Read over enclosures it holds, for every member `a` of `axis` and `w`
/// of `v`, `w`'s part square to `a`: interval evaluation holds each
/// expression's value at every assignment of members, and here `axis` is
/// named three times, so the three uses may take different members. That
/// is a superset of the single-member value, so it is wider, never
/// narrower, than the truth: a radial unit square to the carrier's axis
/// has the same dot with every enclosed part as with `w`, so the rate it
/// gives holds the true rate for whichever member is the carrier's axis.
/// Its width grows with the axis enclosure's width over `|axis|`, and
/// with `v`'s. An axis enclosure holding zero makes `|axis|²` hold zero,
/// and the quotient is unbounded or not a number: no sign is read off it.
fn square_to(axis: Vec3<Interval>, v: Vec3<Interval>) -> Vec3<Interval> {
    v - axis * (axis.dot(v) / axis.norm_squared())
}

/// `ψ∘Φ` over `x × y` in its centred form: its value at the middle, and
/// its rate along each chart direction over the image ([`Psi::rate`])
/// times the half-spans.
/// Tight where the plain enclosure of a slanted image's box is not.
fn centred(psi: &Psi, p: &Presented<'_>, x: (f64, f64), y: (f64, f64)) -> Option<Interval> {
    let (xm, ym) = (geom::mid_param(x.0, x.1), geom::mid_param(y.0, y.1));
    let middle = psi.value(p.image((xm, xm), (ym, ym)))?;
    let whole = x == p.x && y == p.y;
    let (fx, fy, _) = if whole { p.jet() } else { p.jet_on(x, y) };
    let image = if whole { p.whole() } else { p.image(x, y) };
    Some(
        middle
            + psi.rate(image, fx) * iv((x.0 - xm, x.1 - xm))
            + psi.rate(image, fy) * iv((y.0 - ym, y.1 - ym)),
    )
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
        a: frame.u_ref() * r,
        b: frame.v_ref() * r,
    };
    match (surface, held) {
        (
            &Surface::Torus {
                frame: torus_frame,
                major_radius,
                minor_radius,
            },
            Axis::U,
        ) => {
            let (center, axis) = (torus_frame.origin(), torus_frame.axis());
            let radial = torus_frame.radial(b);
            Some(Trig {
                c: center + radial * major_radius,
                a: radial * minor_radius,
                b: axis * minor_radius,
            })
        }
        (
            &Surface::Torus {
                frame: torus_frame,
                major_radius,
                minor_radius,
            },
            Axis::V,
        ) => {
            let (center, axis) = (torus_frame.origin(), torus_frame.axis());
            let (s, c) = b.sin_cos();
            Some(ring(
                torus_frame,
                center + axis * (minor_radius * s),
                major_radius + minor_radius * c,
            ))
        }
        (
            &Surface::Sphere {
                frame: sphere_frame,
                radius,
            },
            Axis::U,
        ) => {
            let (center, axis) = (sphere_frame.origin(), sphere_frame.axis());
            let radial = sphere_frame.radial(b);
            Some(Trig {
                c: center,
                a: radial * radius,
                b: axis * radius,
            })
        }
        (
            &Surface::Sphere {
                frame: sphere_frame,
                radius,
            },
            Axis::V,
        ) => {
            let (center, axis) = (sphere_frame.origin(), sphere_frame.axis());
            let (s, c) = b.sin_cos();
            Some(ring(sphere_frame, center + axis * (radius * s), radius * c))
        }
        (
            &Surface::Cylinder {
                frame: cylinder_frame,
                radius,
            },
            Axis::V,
        ) => {
            let (origin, axis) = (cylinder_frame.origin(), cylinder_frame.axis());
            Some(ring(cylinder_frame, origin + axis * b, radius))
        }
        (
            &Surface::Cone {
                frame: cone_frame,
                half_angle,
            },
            Axis::V,
        ) => {
            let (apex, axis) = (cone_frame.origin(), cone_frame.axis());
            let (s, c) = half_angle.sin_cos();
            Some(ring(cone_frame, apex + axis * (b * c), b * s))
        }
        _ => None,
    }
}

/// **The cells a separator the engine builds vanishes on**, by its
/// construction from them ([`planes_through`], [`graph_of`]): edges by
/// key, vertices, and whether it holds each listed edge's whole line or
/// circle (a plane does; the graph separator vanishes on its edge over
/// its piece alone), and the edge its graph runs on along past its own
/// edge's end, where it does ([`graph_run`]).
#[derive(Clone, Debug, Default)]
struct On {
    edges: Vec<EdgeKey>,
    vertices: Vec<VertexKey>,
    whole: bool,
    /// Where the graph separator's graph runs on along another edge past
    /// its own edge's end ([`graph_of`]).
    run: Option<RunEnd>,
}

/// **Where a graph separator's graph leaves its own edge for the next**
/// ([`graph_run`]): the edge it runs on along, the vertex the two share,
/// that vertex's parameter on the graph's own edge, and whether it is
/// that edge's start.
#[derive(Clone, Copy, Debug, PartialEq)]
struct RunEnd {
    edge: EdgeKey,
    vertex: VertexKey,
    at: f64,
    start: bool,
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
/// - Two adjacent sides, one collapsed, `ψ` linear: zero along both, so
///   its sign is the mixed derivative's, oriented by the corner.
/// - A corner point: zero there, so its sign is the common sign of the
///   two derivatives along the rectangle's axes away from it.
/// - A curve, a graph over one axis: zero along it, so its sign on the
///   face's side is the sign of the derivative along the other axis,
///   over the whole rectangle ([`present_curved`]).
///
/// **The margin.** Each derivative is decided as the sine of the angle
/// at which the face leaves the separator, `∇ψ·Φ_a / (|∇ψ|·|Φ_a|)` over
/// the rectangle, levered at `arm`, the reach of the faces the pair
/// shares the cell between ([`arm_of`]): an angle metered where it
/// displaces the geometry it governs, never at the rectangle's own
/// extent, so a rectangle however thin is read. Two faces each ε off
/// over their run `r` AWAY from the edge meet at an angle uncertain by
/// about `ε/r`, and a sine that clears the band at `r` is one that such
/// an error cannot turn. **The arm read is longer than that run in two
/// ways, both the permissive side**:
/// - it is measured from the edge's middle, so a face's run ALONG the
///   edge counts: two strips that part by less than ε across their whole
///   width are signed apart where they are long enough
///   (`work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md`);
/// - it is the reach alone, where tier 3's wedge check folds the faces'
///   curvature arms in, so beside a face whose radius is far below its
///   reach a sine is signed here that check 4 classes smooth
///   (`work/embed/the-departure-is-levered-at-the-reach-alone-where-check-4-folds-the-faces-curvature-arms.md`).
///
/// With the other face's carrier as `ψ` and the zero side on the shared
/// edge, `Φ_a` is the face's direction across the edge, and the sine is
/// tier 3's wedge check's own, `|n₁ × n₂| / (|n₁||n₂|)`
/// (`n₁·(n₂ × t̂) = t̂·(n₁ × n₂)`, `t̂` the edge's direction), enclosed
/// over the cell's stretch of the edge. Across a curve read as a graph
/// it is that sine times the cosine of the curve's lean off the other
/// axis: an under-reading. No `arm` decides nothing.
///
/// What is decided is the ENCLOSURE of that quotient, which is not held
/// to `[−1, 1]`: where the norms' enclosures are wide (a rectangle
/// beside a pole, a tangent seam at `Interval`) its far end runs past
/// one, to any size. Only the end nearer zero can decide, so a wide
/// enclosure never signs more than a narrow one would.
///
/// In the carrier arm a side is "zero" where `ψ` along it reads within
/// the band's zero ([`is_zero`]), not exactly zero. So over a strip
/// thinner than that zero over the sine, the sign proved is the
/// departure's, the side `ψ` grows to across the strip, and not a sign
/// of `ψ`'s values on it.
///
/// **Two sides, one of them collapsed** (a pole, an apex: the side's
/// whole image is one vertex, [`Along::collapsed`], read off how the
/// side was built). The `Zero::Corner` arm below rests on this bound. Say the
/// regular side is `x = x₀` and the collapsed one `y = y₀`, so
/// `Φ_x(·, y₀) = 0`. Then over the rectangle
/// `Φ_x(x, y) = ∫ Φ_xy(x, η) dη` from `y₀` to `y`, so
/// `n·Φ_x ≥ (y − y₀)·min n·Φ_xy` and `|Φ_x| ≤ (y − y₀)·max |Φ_xy|`,
/// and the sine across the regular side, `n·Φ_x / (|n||Φ_x|)`, which is
/// `0/0` on the collapsed side itself, is at least
/// `min n·Φ_xy / (|n|·max |Φ_xy|)` wherever `n·Φ_xy` is of one sign:
/// what the enclosure `n·Φ_xy / (|n|·|Φ_xy|)` reads from below. That
/// rests on `Φ_x` vanishing on the collapsed side. Between two regular
/// sides it does not: there the face is tangent to the separator at the
/// corner, the sine falls to zero at it while that quotient does not,
/// and the reading answers nothing (the second-order arm's).
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
/// its argument. Anywhere else either answers nothing. A curve run on
/// along the face's next edge ([`RunOn`]) is zero only under the graph
/// separator whose graph runs on along that edge ([`On::run`]): a plane
/// holding the curve's conic does not hold the next edge, and a graph
/// that stops at the vertex is the conic continued past it.
fn sign_on(
    psi: &Psi,
    on: Option<&On>,
    p: &Presented<'_>,
    against: Option<&Presented<'_>>,
    arm: Option<Lever<Interval>>,
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
                        Some(p) => a.under_graph(p, on.run.as_ref()),
                        None => a.on_edges || (on.whole && a.whole),
                    }
            },
        )
    };
    let (fx, fy, fxy) = p.jet();
    let image = p.whole();
    let grad_norm = psi.gradient_norm();
    let leaving = |axis: Axis| {
        let f = match axis {
            Axis::U => fx,
            Axis::V => fy,
        };
        psi.rate(image, f) / (grad_norm * f.norm())
    };
    let departs = |s: Side| levered_sign(leaving(s.axis()) * at(s.into()), arm?, band);
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
            // The sine's limit at a collapsed side (the bound is in this
            // function's docs, "Two sides, one of them collapsed"); two
            // regular sides, or two collapsed ones, are not read by it.
            if a.collapsed().is_some() == b.collapsed().is_some() {
                return None;
            }
            let sine = n.dot(fxy) / (n.norm() * fxy.norm());
            levered_sign(sine * at(Side::into(*s) * Side::into(*t)), arm?, band)
        }
        Zero::Curve(s, c) => {
            if c.past.is_some() && !past_read(p, against) {
                return None;
            }
            // A side that runs on along the next edge is zero only under
            // the graph built along that edge too. A plane holding the
            // curve's whole conic holds the conic continued, which the
            // next edge is not on; the carrier arm reads no past curve.
            let run = c.run.map(|r| r.edge);
            let zero = match (on, psi) {
                (Some(on), Psi::Graph { piece, .. }) => {
                    on.holds(&c.along, false)
                        && c.along.under_graph(*piece, on.run.as_ref())
                        && run.is_none_or(|o| on.run.is_some_and(|r| r.edge == o))
                }
                (Some(on), _) => run.is_none() && on.holds(&c.along, true),
                (None, _) => run.is_none() && c.psi(psi).is_some_and(|v| is_zero(v, band)),
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
/// - **a curve run on along the next edge** ([`RunOn`]): the point is
///   on the run in `R`, and its image is the shared cell's, so on the
///   shared edge of the run's two. Were it on the run's other part, some
///   lift of the shared edge by whole turns would pass through it, in
///   `R`: the lift read, which holds one point to a column and lies
///   across `V`'s column from the other part ([`present_corner`]), or
///   another, which no cell read holds ([`graph_read`]'s step 2, and
///   [`alone`] where the shared edge is the straight one). So it is on
///   the run's shared part: the shared cell;
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
        Surface::Plane { frame, .. } => {
            let (origin, normal) = (frame.origin(), frame.axis());
            Implicit::Plane {
                o: origin,
                n: normal,
            }
        }
        Surface::Cylinder { frame, radius, .. } => {
            let (origin, axis) = (frame.origin(), frame.axis_unit());
            Implicit::Cylinder {
                o: origin,
                axis,
                r: radius,
            }
        }
        Surface::Sphere { frame, radius, .. } => {
            let center = frame.origin();
            Implicit::Sphere {
                c: center,
                r: radius,
            }
        }
        Surface::Cone {
            frame, half_angle, ..
        } => {
            let (apex, axis) = (frame.origin(), frame.axis_unit());
            let (sin, cos) = half_angle.sin_cos();
            let positive = nappe(piece.boundary, apex, axis.get(), band)?;
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

/// **The departure arm.** The separators, in the order tried (the one
/// place they are listed):
/// 1. one face's carrier with the other presented ([`implicit_of`]);
/// 2. the planes through the shared cell ([`planes_through`]): through
///    straight shared edges, a conic one's own plane, or a vertex the pair
///    shares alone;
/// 3. the shared edge's graph ([`graph_of`]);
/// 4. the plane through the pair's one shared vertex, with shared edges
///    near, each face presented against that vertex alone
///    ([`one_shared_vertex`]; the argument is at its site below).
///
/// A face is presented through [`present`], so a sphere face the stored
/// chart leaves unread is presented in the pose of one of its circles
/// ([`present_posed`]), from that face's poses (`faces`), wherever
/// separators 1 to 3 read it presented. Separator 4 re-presents each face
/// against the shared vertex on its stored chart only.
///
/// A pair none of them parts is a [`cusp`] or splits.
fn departure(
    x: &Piece<'_>,
    y: &Piece<'_>,
    near: &Near,
    band: Band,
    two: bool,
    faces: (&FacePoses, &FacePoses),
) -> Outcome {
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
    let (sx, sy) = (Poses::of(faces.0), Poses::of(faces.1));
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
    // Read once a sign may be asked: the returns above never use it.
    let arm = arm_of(x, y, near);
    // One face's carrier, the other departing it. Not between two faces
    // of one face: the face is its own carrier.
    if two {
        for (sep, other) in [(x, &py), (y, &px)] {
            if let (Some(imp), Ok(Presentation::Of(p))) = (implicit_of(sep, band), other)
                && sign_on(&Psi::Carrier(imp), None, p, None, arm, band).is_some()
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
    let apart = |qx: &Presented<'_>, qy: &Presented<'_>, (psi, on): (Psi, On)| match (
        sign_on(&psi, Some(&on), qx, Some(qy), arm, band),
        sign_on(&psi, Some(&on), qy, Some(qx), arm, band),
    ) {
        (Some(a), Some(b)) => a == flip(b),
        _ => false,
    };
    let opposite = |sep| apart(qx, qy, sep);
    let corner = qx.corner || qy.corner;
    if planes_through(x, near, qx, qy).into_iter().any(opposite) {
        return Outcome::Certified { corner };
    }
    if graph_of(x, y, near, qx, qy, band).is_some_and(opposite) {
        return Outcome::Certified { corner };
    }
    // The pair's one shared vertex `v`, with shared edges near: the plane
    // through `v` alone, each face presented against `v` alone. Its own
    // path, not one more separator of the list above: those are read on
    // the pair's presentations against the whole shared cell, this one on
    // presentations against `v`.
    //
    // **Why it is sound.** Each presentation against `{v}` is a superset
    // of the face's part of its cell up to the module's ε concessions:
    // the sliver cannot fire against `{v}` (it reads a side on shared
    // edges), but the touch, the cover and the sheet's overlap can drop
    // points within their few ulps, as they do in any presentation. `apart`
    // accepts only where the plane is strictly of one sign on each off its
    // zero set and the two signs are opposite. Against `{v}` the zero sets
    // lie at `v`: a zero carrying an edge reads nothing under a plane that
    // lists `v` alone ([`On::holds`]). So a common point of the two parts
    // other than `v` would carry both signs, and none exists. A shared
    // edge's points in both cells, or a second ruling of a cone the plane
    // face holds, are such common points: there the pair is not signed
    // apart, and splits as before. Tangency, a second meeting off the
    // shared cell, and a vertex that is no apex are refused by the same
    // reading, not by a premise of their own.
    //
    // **Why a collapsed side is read.** A cone's apex or a sphere's pole
    // is presented as a side whose whole image is `v` ([`Along::at`]), and
    // `sign_on` reads it as any side, by the mean value theorem along the
    // axis across it at each value of the other: `ψ∘Φ` vanishes on the
    // side, so its sign off it is the sign of `ψ∘Φ`'s derivative across it
    // over the whole rectangle. That never reads the derivative that
    // vanishes at the collapse; across the side the chart's derivative is
    // a ruling's or a meridian's direction. `ψ` vanishes on the side by
    // membership, `q` being `v`'s stored point, and the side's image is
    // `v` within the collapse's own reading: `vertex_at_singular_point`,
    // at an apex and at a pole alike (the module's separator
    // concessions).
    //
    // **Where it fires.** It certifies only where each face against `v` is
    // presented with no zero (read by value), a corner point at `v`
    // ([`Zero::Point`]: the plane's two-line corner, or the corner of two
    // coordinate-line clips on a chart), or a collapsed side at `v` (a
    // cone's apex, a sphere's pole). Measured: a plane against a cone at
    // its apex (a wedge of a revolved cone past a half turn, a box face
    // through the apex), and the two half-bands of a sphere at a pole (a
    // union of revolved balls, the tour's dice's pips).
    //
    // With no shared edge near, the presentations against `v` are the
    // pair's own, which the planes above have read: the edge test saves
    // that work and is no premise.
    if !near.edges.is_empty()
        && let Some(v) = one_shared_vertex(&near.vertices)
    {
        let alone = Near {
            edges: Vec::new(),
            vertices: vec![v],
        };
        // The faces are re-presented against `v` on their stored charts
        // only, never in a pose: no review has read a posed presentation
        // re-presented against `{v}`
        // (`work/embed/the-one-shared-vertex-arm-reads-no-posed-presentation`).
        let unposed = FacePoses::default();
        let (nx, ny) = (Poses::none(&unposed), Poses::none(&unposed));
        let (rx, ry) = (
            present(x, &[], &alone.vertices, band, &nx),
            present(y, &[], &alone.vertices, band, &ny),
        );
        if let (Ok(Presentation::Of(rx)), Ok(Presentation::Of(ry))) = (&rx, &ry)
            && planes_through(x, &alone, rx, ry)
                .into_iter()
                .any(|sep| apart(rx, ry, sep))
        {
            return Outcome::Certified {
                corner: rx.corner || ry.corner,
            };
        }
    }
    if let Some(shared) = cusp(x, near, qx, qy, y.boundary.sense, band) {
        return Outcome::Cusp { shared };
    }
    split
}

/// **The arm a cell pair's departure is levered at**: the reach of the
/// faces the reading governs, never the cell's own extent.
///
/// - Where the pair is near shared edges, each one's reach
///   ([`crate::wedge_reach::reach_over`], the recipe
///   [`crate::edge_wedge_reach`] is built by), over the lifted carrier
///   and the two faces' lifted vertices: the least of them. Tier 3's
///   wedge check is handed the same reach and folds the faces'
///   curvature arms into it; this reading does not.
/// - Where it is near shared vertices alone, the two faces' reach from
///   each ([`crate::wedge_reach::reach_from_vertex`]): the least.
///
/// The least is the safe one of several: a shorter arm levers a smaller
/// margin. `None` where a near edge is null scaffolding or a vertex does
/// not resolve: a reading with no arm decides nothing.
///
/// **This is the reach as [`crate::wedge_reach`] computes it, not the
/// extent `docs/DESIGN.md` `[topology.tiers]` rules**: the reach is the
/// farther face's, measured from the edge's middle (a run along the
/// edge counts), and it is recomputed here per cell pair. The clause
/// levers at how far the narrower face runs from what the two share,
/// measured across it, read from tier 3's verdict. Building that is
/// `work/embed/the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict.md`.
///
/// The arm is the pair's, by what it is NEAR, not by what a presentation
/// reads: a pair near an edge that is presented against one of that
/// edge's vertices alone is still levered at the edge's reach, not at
/// [`crate::wedge_reach::reach_from_vertex`]. A reader that re-presents
/// a pair against less than it is near owes that choice here.
///
/// Read per cell pair, a pass over both faces' vertices each time.
/// The key a memo needs is here (`x.face`, `y.face` and the near
/// edges' keys); what is not is a cache threaded through the walk, and
/// none is added: the unit named above reads the extent once, from the
/// verdict, and this pass goes with it.
fn arm_of(x: &Piece<'_>, y: &Piece<'_>, near: &Near) -> Option<Lever<Interval>> {
    // One face against itself reads its boundary once.
    let other = (x.face != y.face).then_some(y.boundary);
    let faces = || {
        x.boundary
            .vertices
            .values()
            .chain(other.into_iter().flat_map(|b| b.vertices.values()))
            .copied()
    };
    let arms: Option<Vec<Lever<Interval>>> = if near.edges.is_empty() {
        near.vertices
            .iter()
            .map(|v| {
                let at = *x.boundary.vertices.get(v)?;
                Some(crate::wedge_reach::reach_from_vertex(at, faces()))
            })
            .collect()
    } else {
        near.edges
            .iter()
            .map(|&(i, _)| {
                let (carrier, span) = x.boundary.edges.get(i)?.arc.as_ref()?;
                Some(crate::wedge_reach::reach_over(carrier, *span, faces()))
            })
            .collect()
    };
    arms?.into_iter().reduce(Lever::min)
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
/// A curve run on along the next edge ([`RunOn`]) is not stuck: a
/// smaller cell may decide that edge onto the face's side and read the
/// curve alone, with nothing past the shared cell.
fn stuck(piece: &Piece<'_>, p: &Result<Presentation<'_>, Unpresented>) -> bool {
    match p {
        Err(Unpresented::Structural) => true,
        Ok(Presentation::Of(q)) => matches!(&q.zero,
            Zero::Curve(_, c) if c.run.is_none()
                && c.past.is_some_and(|v| piece.local.vertices.contains(&v))),
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

/// **The normal of a plane through a vertex that parts two faces each
/// presented there as a quadrant** ([`Zero::Point`]): each face leaves
/// the vertex into the cone of its two corner axes, `a₁, a₂` and
/// `b₁, b₂`, read over its rectangle ([`Presented::corner_axes`]). A
/// plane through the vertex parts them, `n·aᵢ > 0 > n·bⱼ`, exactly where
/// the hull of `{a₁, a₂, −b₁, −b₂}` misses the origin (Gordan): where the
/// cones share no ray and neither is a straight corner, whose
/// antiparallel axes put the origin in the hull whatever the other is.
/// The hull's point nearest the origin is then the normal whose least
/// cosine to the four axes is greatest
/// ([`geom_core::linalg::min_norm::min_norm_point`], at `f64`). On a
/// pair that parts with a least cosine above about `1e-7` it is found;
/// nearer `√ε` it can be missed or come a few `1e-8` short; it is kept
/// only where it parts all four axes at `f64`. `None` for any other
/// presentation, or where nothing parts.
///
/// It chooses a direction and decides nothing: [`sign_on`] reads each
/// face's sign at the band, so a plane that does not part the faces
/// certifies nothing, and a poor `n` costs a certificate, never a wrong
/// one.
fn wedges_apart(px: &Presented<'_>, py: &Presented<'_>) -> Option<Vec3<f64>> {
    parting_normal(px.corner_axes()?, py.corner_axes()?)
}

/// [`wedges_apart`]'s normal from the two corners' axes.
fn parting_normal([a1, a2]: [Vec3<f64>; 2], [b1, b2]: [Vec3<f64>; 2]) -> Option<Vec3<f64>> {
    let g = [a1, a2, -b1, -b2];
    let n = geom_core::linalg::min_norm::min_norm_point(&g.map(|d| [d.x, d.y, d.z]))?;
    let n = Vec3::new(n[0], n[1], n[2]);
    g.iter().all(|d| n.dot(*d) > 0.0).then(|| unit(n)).flatten()
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
/// - at a shared vertex alone, through the vertex, listing the vertex:
///   where both faces are presented as quadrants there, `n` the plane
///   leaning furthest from both wedges ([`wedges_apart`]); against any
///   other presentation (a sphere's pole read as its chart's collapsed
///   side, a curve through the vertex), where a face has no corner
///   axes, `n` across the faces' two headings. Two quadrants the wedge
///   plane does not part get no plane: any plane [`sign_on`] reads
///   departing both of their rectangles parts their axes, so the wedge
///   plane is missing only where those axes part by a least cosine
///   below about `1e-7`, and the heading plane is no fallback there.
///
/// Every value is computed at the lane scalar from the stored carrier,
/// so each member of a family has its own `n` and `q` in these
/// enclosures, normal to its own edge and on it: the plane vanishes on
/// the listed cells for every member. The headings, the bisector and the
/// wedges only choose a direction; any direction is exact. A shared
/// vertex off the lines a plane was built through is not listed. Where
/// these are tried among the other separators is [`departure`]'s list.
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
        run: None,
    };
    let conic = |r: &Rim| match r.arc.as_ref()? {
        (conic @ (Curve3::Circle(_) | Curve3::Ellipse(_)), _) => {
            let frame = conic.frame()?;
            Some((frame.origin(), frame.axis()))
        }
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
                Some((line.eval(at(geom::mid_param(t0.lo(), t1.hi()))), dir.get()))
            }
            _ => None,
        })
        .collect();
    let plane = || {
        let lines = lines?;
        let headings = || {
            let q0 = pmid(anchor(x, near)?);
            Some((heading(px, q0)?, heading(py, q0)?))
        };
        match (lines.as_slice(), rims.as_slice()) {
            ([], []) => {
                let v = *near.vertices.first()?;
                let q = *b.vertices.get(&v)?;
                let n = match (&px.zero, &py.zero) {
                    (Zero::Point(..), Zero::Point(..)) => wedges_apart(px, py)?,
                    _ => {
                        let (hx, hy) = headings()?;
                        unit(hx - hy)?
                    }
                };
                let on = On {
                    edges: Vec::new(),
                    vertices: vec![v],
                    whole: true,
                    run: None,
                };
                Some((Psi::Linear { n: lift(n), q }, on))
            }
            ([(q, d)], [r]) => {
                let (hx, hy) = headings()?;
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

/// **Whether two planar conics' lifted frames hold one plane, as one
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
    // The last piece ends at the span's own end, not at `t0 + width`,
    // which rounds a step past it: the hull is the edge's parameters.
    #[allow(clippy::cast_precision_loss)]
    let at_k = |k: usize| {
        if k == PIECES {
            t1
        } else {
            t0 + width * (k as f64) / (PIECES as f64)
        }
    };
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
///
/// **Where one face's curved side runs on along its next edge**
/// ([`RunOn`]), `γ` past the vertex is that edge's graph, not the
/// carrier's continuation, and `slope` holds its slopes too
/// ([`graph_run`]); `ψ`'s gradient is `m − γ′(s)·t` with `γ′` in `slope`
/// wherever `γ` is differentiable, which is all [`sign_on`] reads of it.
/// Two such sides answer nothing.
fn graph_of<'a>(
    x: &Piece<'a>,
    y: &Piece<'a>,
    near: &Near,
    px: &Presented<'_>,
    py: &Presented<'_>,
    band: Band,
) -> Option<(Psi, On)> {
    let alone = one_shared_vertex(&near.vertices).filter(|_| near.edges.is_empty());
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
    // One face's side running on along its next edge: the graph leaves
    // its own edge for that one at the vertex the two share.
    let running = |p: &Presented<'_>| match &p.zero {
        Zero::Curve(_, c) => c.run,
        _ => None,
    };
    let end = match (running(px), running(py)) {
        (None, None) => None,
        (Some(r), None) | (None, Some(r)) => Some(RunEnd {
            edge: r.edge,
            vertex: if r.start { e.ends[0] } else { e.ends[1] },
            at: if r.start { span.0.lo() } else { span.1.hi() },
            start: r.start,
        }),
        (Some(_), Some(_)) => return None,
    };
    let piece = [px, py]
        .into_iter()
        .filter_map(|p| {
            let along = match &p.zero {
                Zero::Curve(_, c) => &c.along,
                Zero::Side(_, a) => a,
                _ => return None,
            };
            (along.whole && along.edges == [e.key]).then(|| along.ending(reached, end.as_ref()))?
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
    let slope = m.dot(d) / advance;
    let run_of = |p: &Presented<'_>, b: &'a Boundary| {
        running(p)
            .zip(end)
            .map(|(_, end)| graph_run(piece, b, end, p.whole(), (t, m), band))
    };
    let slope = match run_of(px, x.boundary).or_else(|| run_of(py, y.boundary)) {
        Some(s) => hull(slope, s?),
        None => slope,
    };
    let run = end;
    Some((
        Psi::Graph { m, t, slope, piece },
        On {
            edges: vec![e.key],
            vertices,
            whole: false,
            run,
        },
    ))
}

/// **The graph run on along a corner's other edge** ([`graph_of`]): the
/// slopes of `γ` over that edge. `piece` is what the graph holds of its
/// own edge's carrier, and `end` where it leaves that edge for the
/// other edge of `b`, a presentation's curved side running on
/// along it over the image `image` of its rectangle.
///
/// - The other edge, over every parameter of it `image` may hold
///   ([`edge_piece`]), advances along `t` away from the vertex the way
///   `e` comes to it, decided as the graph's own edge is
///   ([`SELF_INTERSECTION_GRAPH`]): it is a graph over `s` too, running
///   on from the vertex.
/// - `piece` stops at the vertex's parameter: past it `γ` is the other
///   edge's graph, not the carrier's. A side that ends at that vertex by
///   key is read to it ([`Along::ending`]) and does not widen the piece;
///   a piece any other side has widened past the vertex answers
///   nothing, by however little.
fn graph_run(
    piece: (f64, f64),
    b: &Boundary,
    end: RunEnd,
    image: Point3<Interval>,
    (t, m): (Vec3<Interval>, Vec3<Interval>),
    band: Band,
) -> Option<Interval> {
    let o = b.edges.iter().find(|r| r.key == end.edge)?;
    let (other, other_span) = o.arc.as_ref()?;
    // Asked as "at or before the vertex", so a parameter that is no
    // number is read as past it.
    let past = if end.start {
        end.at - piece.0
    } else {
        piece.1 - end.at
    };
    if !matches!(
        past.partial_cmp(&0.0),
        Some(core::cmp::Ordering::Less | core::cmp::Ordering::Equal)
    ) {
        return None;
    }
    let reached = edge_piece(other, *other_span, false, &[image])?;
    let d = arc_deriv(other, reached);
    let away = if o.ends[0] == end.vertex {
        1.0
    } else if o.ends[1] == end.vertex {
        -1.0
    } else {
        return None;
    };
    // `s` grows toward the vertex where it is the graph's edge's end, and
    // the other edge must carry it on: `γ` is one function of `s`. The
    // chart reading has the run a graph over a chart axis, not over `t`,
    // so this is the separator's own premise, as its edge's advance is.
    let onward = if end.start { -1.0 } else { 1.0 };
    let advance = t.dot(d);
    if interval_sign_as(
        advance * at(away * onward * (reached.1 - reached.0)),
        SELF_INTERSECTION_GRAPH,
        band,
    ) != Some(Sign::Positive)
    {
        return None;
    }
    Some(m.dot(d) / advance)
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
            frame: geom::Frame::new(
                Point3::new(p(0.0), p(0.0), p(0.0)),
                Vec3::new(p(0.0), p(0.0), p(1.0)),
                Vec3::new(p(1.0), p(0.0), p(0.0)),
            ),
            major_radius: p(major),
            minor_radius: p(1.0),
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
/// **A test row's segment from `a` to `b`**, stated points: its chord, a
/// unit direction over its length, and its span in metres, as a line's
/// carrier is stated since its direction is a [`UnitVec3`].
#[cfg(test)]
fn chord_segment(
    a: Point3<Interval>,
    b: Point3<Interval>,
) -> (Curve3<Interval>, (Interval, Interval)) {
    // The unit chord and its length as enclosures ([`UnitVec3::chord`]):
    // a point box of the rounded quotient is no unit vector at an
    // enclosure scalar, and is poison wherever its squares do not sum
    // to exactly 1.
    let (dir, len) = UnitVec3::chord(b - a);
    (Curve3::line(a, dir), (Interval::from_bounds(0.0, 0.0), len))
}

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

    /// A box around a ruling's segment 1.2e-5 long, `r` off a tilted axis
    /// through `o` and `along` metres up it, and the segment's direction.
    fn on_a_ruling(
        o: Vec3<f64>,
        d: Vec3<f64>,
        along: f64,
        r: f64,
    ) -> (Point3<Interval>, Vec3<Interval>) {
        let off = Vec3::new(0.8, 0.6, 0.0);
        let c = Point3::origin() + o + d * along + off * r;
        let h = 6e-6;
        let span = |x: f64, k: f64| Interval::from_bounds(x - h * k.abs(), x + h * k.abs());
        (
            Point3::new(span(c.x, d.x), span(c.y, d.y), span(c.z, d.z)),
            d.map(iv0) * iv0(2.0 * h),
        )
    }

    /// **A cylinder carrier's rate along its ruling reads zero over a
    /// thin box on a tilted axis, at the origin and 100 m up it**: its
    /// gradient is square to the axis at every point, so its rate along
    /// the axis is 0, which a box's
    /// enclosure of the radial unit (it holds directions with an axial
    /// part, the more the box is tilted) must not read (red where the
    /// rate dots the gradient box with the ruling itself). The cone's
    /// radial term is the same reading; its rate along its axis is
    /// `−nappe·sin` exactly.
    #[test]
    fn a_carriers_rate_along_its_ruling_reads_tight_on_a_tilted_axis_near_and_far() {
        let d = Vec3::new(0.36, -0.48, 0.8);
        let o = Vec3::new(1.5, -2.25, 0.75);
        for along in [0.0, 100.0] {
            let (b, v) = on_a_ruling(o, d, along, 0.006);
            let cyl = Psi::Carrier(Implicit::Cylinder {
                o: Point3::origin() + o.map(iv0),
                axis: geom_core::UnitVec3::chord(d.map(iv0)).0,
                r: iv0(0.006),
            });
            let rate = cyl.rate(b, v);
            assert!(
                rate.hi() - rate.lo() < 1e-15 && rate.lo() <= 0.0 && 0.0 <= rate.hi(),
                "cylinder, {along} m up: rate {rate:?}"
            );
            // Retired: the same carrier about an axis twice as long. A
            // carrier's axis is a `UnitVec3<Interval>` (PR 7349, `onehome/carrier-directions`),
            // which no door admits off unit, so a non-unit axis reaching
            // `rate` is unrepresentable: a stronger guarantee than this
            // sub-case gave.
            let (sin, cos) = (0.6, 0.8);
            let cone = Psi::Carrier(Implicit::Cone {
                apex: Point3::origin() + o.map(iv0),
                axis: geom_core::UnitVec3::chord(d.map(iv0)).0,
                cos: iv0(cos),
                sin: iv0(sin),
                nappe: iv0(1.0),
            });
            let axial = d.map(iv0) * iv0(1.0);
            let rate = cone.rate(b, axial);
            assert!(
                rate.hi() - rate.lo() < 1e-12 && rate.lo() <= -sin && -sin <= rate.hi(),
                "cone, {along} m up: rate {rate:?}"
            );
        }
    }
    /// **A cone's rate holds its nappe and its half-angle apart**: at a
    /// point off the axis, along the radial unit it is `cos`, along the
    /// axis it is `−nappe·sin`, on either nappe (red where the nappe's
    /// sign is dropped or `cos` and `sin` trade places).
    #[test]
    fn a_cones_rate_reads_its_radial_and_axial_parts_on_each_nappe() {
        let (sin, cos) = (0.6, 0.8);
        let apex = Point3::new(iv0(1.0), iv0(-2.0), iv0(0.5));
        let axis = geom_core::UnitVec3::exact_z();
        for nappe in [1.0, -1.0] {
            let cone = Psi::Carrier(Implicit::Cone {
                apex,
                axis,
                cos: iv0(cos),
                sin: iv0(sin),
                nappe: iv0(nappe),
            });
            let p = Point3::new(iv0(4.0), iv0(-2.0), iv0(0.5 + 4.0 * nappe));
            let radial = cone.rate(p, Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)));
            let axial = cone.rate(p, axis.get());
            let near = |x: Interval, want: f64| x.lo() <= want + 1e-15 && want - 1e-15 <= x.hi();
            assert!(near(radial, cos), "nappe {nappe}: radial rate {radial:?}");
            assert!(
                near(axial, -nappe * sin),
                "nappe {nappe}: axial rate {axial:?}"
            );
        }
    }

    /// **The graph separator's rate is `(m − γ′·t)·v`**: with `γ′` in
    /// `slope`, along `t` it is `m·t − γ′·|t|²` over the slope's range
    /// (red where the slope term's sign is flipped).
    #[test]
    fn the_graph_separators_rate_subtracts_its_slope_along_t() {
        let graph = Psi::Graph {
            m: Vec3::new(iv0(0.0), iv0(1.0), iv0(0.0)),
            t: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            slope: Interval::from_bounds(0.25, 0.5),
            piece: (0.0, 1.0),
        };
        let p = origin();
        let along_t = graph.rate(p, Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)));
        assert!(
            along_t.lo() <= -0.5 && -0.25 <= along_t.hi() && along_t.hi() < 0.0,
            "along t: {along_t:?}"
        );
        let along_m = graph.rate(p, Vec3::new(iv0(0.0), iv0(1.0), iv0(0.0)));
        assert!(
            along_m.lo() <= 1.0 && 1.0 <= along_m.hi(),
            "along m: {along_m:?}"
        );
    }

    /// **A bracketed axis and a bracketed direction**: the cylinder's
    /// rate over a box encloses `unit_a(q)·w` for members `a` of the
    /// axis, `q` of the box and `w` of the direction, each read at
    /// `f64`; an axis enclosure holding zero reads no sign.
    #[test]
    fn a_carriers_rate_encloses_every_members_rate_where_the_axis_is_a_bracket() {
        let e = 1e-7;
        let wide = |x: f64| Interval::from_bounds(x - e, x + e);
        let d = Vec3::new(0.36, -0.48, 0.8);
        let o = Vec3::new(1.5, -2.25, 0.75);
        let (b, _) = on_a_ruling(o, d, 3.0, 0.2);
        let w0 = Vec3::new(0.3, 0.9, -0.2);
        let cyl = Psi::Carrier(Implicit::Cylinder {
            o: Point3::origin() + o.map(iv0),
            axis: geom_core::UnitVec3::checked(d.map(wide)),
            r: iv0(0.2),
        });
        let rate = cyl.rate(b, w0.map(wide));
        let corner = |i: &Interval, k: f64| if k < 0.0 { i.lo() } else { i.hi() };
        for k in [-1.0, 1.0] {
            for j in [-1.0, 1.0] {
                let a = d + Vec3::new(k, -k, j) * (0.9 * e);
                let w = w0 + Vec3::new(j, k, -j) * (0.9 * e);
                let q = Point3::new(corner(&b.x, k), corner(&b.y, j), corner(&b.z, -k));
                // The member's axis is its direction, `a` over its length.
                let u = a * (1.0 / a.norm());
                let off = q - (Point3::origin() + o);
                let r = off - u * off.dot(u);
                let want = r.dot(w) / r.norm();
                assert!(
                    rate.lo() <= want && want <= rate.hi(),
                    "{rate:?} misses member ({k}, {j}): {want}"
                );
            }
        }
        // Retired: an axis enclosure holding zero. A carrier's axis is a
        // `UnitVec3<Interval>` (PR 7349, `onehome/carrier-directions`); a box
        // holding zero holds no unit vector, so no door admits one.
    }
    use crate::test_support_fixtures::trv;
    use geom_core::Tol;

    const ALPHA: f64 = 0.2;
    const BETA: f64 = 4.0;

    fn iv0(x: f64) -> Interval {
        Interval::from_bounds(x, x)
    }

    /// A stated arm of `length` metres, for rows that state a
    /// presentation by hand and so have no faces to read a reach from.
    /// The one spelling of a fixture arm in this file's rows.
    pub(super) fn arm(length: f64) -> Option<Lever<Interval>> {
        Some(geom_core::test_support::fixture_lever(at(length)))
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
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            radius: iv0(1.0),
        }
    }

    /// An edge of one use, from `a` to `b` in its loop, whose chart image
    /// holds `held` at `at`.
    fn rim(n: u64, ends: [u64; 2], held: Axis, at: f64, a: (f64, f64), b: (f64, f64)) -> Rim {
        Rim {
            stated: None,
            key: edge(n),
            arc: Some((
                Curve3::line(origin(), geom_core::UnitVec3::exact_x()),
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
                curve: None,
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
            outer: None,
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
            outer: None,
        }
    }

    /// **A face is never read as holding nothing in a cell it fills,
    /// whichever sheet its extent is on.** On its own sheet the
    /// half-band is presented in its whole window and in its own half
    /// turn, and the other half turn holds none of it. On the
    /// complement's sheet its own lines put it outside its extent
    /// ([`face_extent`]), so it is not presented at any depth: neither
    /// the whole window's zero-width rectangle `u = π` beside the shared
    /// meridian, nor its own half turn, which meets no
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
    /// and the rectangle they leave is no thin strip beside the shared
    /// line to present. Red if the zero width is read vacuous.
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
            outer: None,
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
            outer: None,
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
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
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
            outer: None,
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
            outer: None,
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
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
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
            outer: None,
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
            outer: None,
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
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
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
            outer: None,
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
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
        };
        for z in [1.0, -1.0] {
            let b = Boundary {
                sense: true,
                edges: Vec::new(),
                vertices: BTreeMap::from([(vertex(3), Point3::new(iv0(1.0), iv0(0.0), iv0(z)))]),
                extent: Some([(1.0, 2.0), (0.0, 0.0)]),
                outer: None,
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
            outer: None,
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
            outer: None,
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
            outer: None,
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
    /// coverage test refuses. Red if the clipped rectangle is presented
    /// as the face's part of the cell, or read vacuous.
    #[test]
    fn a_finger_beside_a_shared_wall_is_not_presented_by_its_clip() {
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
            outer: None,
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
            outer: None,
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
            curve: None,
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
            curve: None,
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
            curve: None,
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
            outer: None,
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

    /// **A stepped face's low line `v = 0.5`, the face above it, as two
    /// edges**: the first from `u = 0.2` to `ends[0]` at vertex
    /// `keys[0]`, the second from `ends[1]`, on `line`, at vertex
    /// `keys[1]`, to `u = 0.8`. The face's hull reaches below the line
    /// (its step is elsewhere), so no cell beside the line misses its
    /// extent.
    fn stepped(ends: [f64; 2], keys: [u64; 2], line: f64) -> Boundary {
        Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, keys[0]], Axis::V, 0.5, (0.2, 0.5), (ends[0], 0.5)),
                rim(2, [keys[1], 2], Axis::V, line, (ends[1], line), (0.8, line)),
            ],
            vertices: BTreeMap::new(),
            extent: Some([(0.2, 0.8), (0.0, 1.0)]),
            outer: None,
        }
    }

    /// The cells across [`stepped`]'s joint, below its line and above.
    const BELOW: ((f64, f64), (f64, f64)) = ((0.45, 0.55), (0.4, 0.5));
    const ABOVE: ((f64, f64), (f64, f64)) = ((0.45, 0.55), (0.5, 0.6));

    /// One ulp above `0.5`.
    fn up() -> f64 {
        f64::from_bits(0.5f64.to_bits() + 1)
    }

    /// **A line's edges joined at a vertex cover a side together**
    /// ([`joints`]): two edges of one line, the face above both, the one
    /// ending at the vertex the other starts at. A cell below the line
    /// across the joint holds none of the face ([`vacuous`]) and one
    /// above it presents with the line as its side
    /// ([`sides_are_the_faces`]), whether the two stored ends at the
    /// joint are equal, an ulp apart or an ulp over: they are two
    /// readings of one vertex. Red if a side is read against one edge
    /// alone, or if the two readings are compared.
    #[test]
    fn a_lines_edges_joined_at_a_vertex_cover_a_side_together() {
        for (kind, s) in [("cylinder", cylinder()), ("torus", ring(2.0))] {
            for (what, ends) in [
                ("equal", [0.5, 0.5]),
                ("an ulp apart", [0.5, up()]),
                ("an ulp over", [up(), 0.5]),
            ] {
                let b = stepped(ends, [5, 5], 0.5);
                assert_eq!(
                    chart(&s, &b, &[0, 1], BELOW.0, BELOW.1),
                    Read::Vacuous,
                    "{kind}, {what}: the cell below"
                );
                assert!(
                    matches!(chart(&s, &b, &[0, 1], ABOVE.0, ABOVE.1), Read::Of(..)),
                    "{kind}, {what}: the cell above: {:?}",
                    chart(&s, &b, &[0, 1], ABOVE.0, ABOVE.1)
                );
            }
        }
    }

    /// **A joint is read by each edge's low and high end along the
    /// line, not by its curve's start and end**: the first edge stored
    /// against its loop (its curve runs down the line, the loop runs it
    /// backward), so its start is its high end, the joint. Red if an
    /// edge's ends are taken in the curve's order.
    #[test]
    fn a_joint_is_read_at_an_edges_high_end_whichever_way_its_curve_runs() {
        let mut b = stepped([0.5, up()], [5, 5], 0.5);
        b.edges[0] = Rim {
            uses: vec![Use {
                plus: false,
                ..rim(1, [5, 1], Axis::V, 0.5, (0.5, 0.5), (0.2, 0.5)).uses[0].clone()
            }],
            ..rim(1, [5, 1], Axis::V, 0.5, (0.5, 0.5), (0.2, 0.5))
        };
        let s = cylinder();
        assert_eq!(chart(&s, &b, &[0, 1], BELOW.0, BELOW.1), Read::Vacuous);
        assert!(matches!(
            chart(&s, &b, &[0, 1], ABOVE.0, ABOVE.1),
            Read::Of(..)
        ));
    }

    /// **A zero side across a joint lies on its edges**
    /// ([`Along::on_edges`]): both edges of the line shared, the cell
    /// above it across the joint, the two stored ends an ulp apart. Red
    /// if the zero side's cover is read without the joint (the side is
    /// then taken to run past its edges, and a plane holding both no
    /// longer vanishes on it by membership).
    #[test]
    fn a_zero_side_across_a_joint_lies_on_its_edges() {
        let b = stepped([0.5, up()], [5, 5], 0.5);
        let (s, local) = (
            cylinder(),
            Local {
                edges: vec![0, 1],
                vertices: Vec::new(),
            },
        );
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &b,
            u: ABOVE.0,
            v: ABOVE.1,
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let Ok(Presentation::Of(p)) = present_chart(&piece, &[0, 1], &[], band()) else {
            panic!("the cell above presents")
        };
        let Zero::Side(Side::YLo, along) = &p.zero else {
            panic!("the line is the zero side: {:?}", p.zero)
        };
        assert!(along.on_edges);
    }

    /// **A vertex where two edges of the line begin joins none**
    /// ([`joints`]' one continuation): a third edge of the line starts
    /// at the joint's vertex too, short of the cell's far end. Red if
    /// every pair that shares the vertex is joined.
    #[test]
    fn a_vertex_two_edges_of_the_line_begin_at_joins_none() {
        let mut b = stepped([0.5, 0.5], [5, 5], 0.5);
        b.edges
            .push(rim(3, [5, 6], Axis::V, 0.5, (0.5, 0.5), (0.52, 0.5)));
        let s = cylinder();
        for cell in [BELOW, ABOVE] {
            assert_ne!(
                chart(&s, &b, &[0, 1, 2], cell.0, cell.1),
                Read::Vacuous,
                "{cell:?}"
            );
        }
        assert_eq!(chart(&s, &b, &[0, 1, 2], BELOW.0, BELOW.1), Read::Local);
    }

    /// **Two rulings through a cone's apex are not one line**: on
    /// opposite nappes, the one ending at the apex's vertex and the other
    /// beginning there by key, their rows holding `u = 0.5` and `u = 1`.
    /// The face is on opposite sides of the two, so no joint is read, and
    /// each cell at the apex's height reads as it does with no joint at
    /// all: the one on the face side of both is not presented, and the
    /// one between them, which neither line reaches, keeps its cell. Red
    /// if a joint through the apex makes the two one line (the first
    /// cell would then present, the second be clipped or read empty).
    #[test]
    fn rulings_joined_through_an_apex_are_not_one_line() {
        let s = Surface::Cone {
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
        };
        let b = Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 9], Axis::U, 0.5, (0.5, -0.5), (0.5, 0.0)),
                rim(2, [9, 2], Axis::U, 1.0, (1.0, 0.0), (1.0, 0.5)),
            ],
            vertices: BTreeMap::new(),
            extent: Some([(0.0, 1.0), (-0.5, 0.5)]),
            outer: None,
        };
        let v = (-0.05, 0.05);
        assert_eq!(chart(&s, &b, &[0, 1], (0.3, 0.45), v), Read::Local);
        assert_eq!(
            chart(&s, &b, &[0, 1], (0.6, 0.7), v),
            Read::Of((0.6, 0.7), v)
        );
    }

    /// **A side stopped at a corner by a line of two joined edges**: the
    /// rectangle's left edge, shared, ends at the corner `(1, 0)`; the
    /// bottom line is two edges joined at `u = 1.05`, their stored ends
    /// an ulp apart, inside the cell. The cell at the corner presents
    /// with the left edge as its zero side, stopped at the corner's
    /// vertex, and the bottom covered across its joint: the corner's
    /// stop is read off the one-edge side and the joint off the
    /// two-edge line, and neither reads the other's clip. Red if the
    /// joined line does not cover its side, or if the corner is not
    /// recorded against it.
    #[test]
    fn a_side_stopped_at_a_corner_by_a_line_of_two_joined_edges() {
        let joint = 1.05;
        let b = unit_face(vec![
            rim(1, [4, 1], Axis::U, 1.0, (1.0, 1.0), (1.0, 0.0)),
            rim(2, [1, 5], Axis::V, 0.0, (1.0, 0.0), (joint, 0.0)),
            rim(
                3,
                [5, 2],
                Axis::V,
                0.0,
                (f64::from_bits(joint.to_bits() + 1), 0.0),
                (2.0, 0.0),
            ),
        ]);
        let (s, local) = (
            cylinder(),
            Local {
                edges: vec![0, 1, 2],
                vertices: Vec::new(),
            },
        );
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &b,
            u: (1.0, 1.1),
            v: (0.0, 0.1),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let Ok(Presentation::Of(p)) = present_chart(&piece, &[0], &[], band()) else {
            panic!("the corner cell presents")
        };
        let Zero::Side(Side::XLo, along) = &p.zero else {
            panic!("the left edge is the zero side: {:?}", p.zero)
        };
        assert_eq!(along.stops, vec![vertex(1)]);
    }

    /// **Edges that share no vertex are not joined**: the two edges end
    /// at two vertices, an ulp apart, so the face may come through
    /// between them, and neither cell is read. Where their stored ranges
    /// overlap instead, the cell below is still not read vacuous: a join
    /// is a vertex, never an overlap. Red if two edges are joined by
    /// their ranges.
    #[test]
    fn edges_of_a_line_that_share_no_vertex_are_not_joined() {
        let s = cylinder();
        let gap = stepped([0.5, up()], [5, 6], 0.5);
        for cell in [BELOW, ABOVE] {
            assert_eq!(chart(&s, &gap, &[0, 1], cell.0, cell.1), Read::Local);
        }
        // Ends that overlap, or are equal, at two vertices. Above the
        // line the clip's cover still joins them by their ranges
        // (`work/embed/a-clips-cover-joins-its-edges-ranges-by-overlap-with-no-shared-vertex`):
        // pinned as it reads, not as it should.
        for ends in [[0.52, 0.48], [0.5, 0.5]] {
            let over = stepped(ends, [5, 6], 0.5);
            assert_eq!(
                chart(&s, &over, &[0, 1], BELOW.0, BELOW.1),
                Read::Local,
                "{ends:?}"
            );
            assert!(
                matches!(chart(&s, &over, &[0, 1], ABOVE.0, ABOVE.1), Read::Of(..)),
                "{ends:?}"
            );
        }
    }

    /// **Rows joined at a vertex are one line whatever values they
    /// hold** ([`joints`], [`present_chart`]'s merge): the second edge
    /// starts at the first's vertex and its row holds the line an ulp
    /// higher, as two rows of one parallel do in a frame that is not the
    /// chart's. The cell below holds none of the face and the cell above
    /// has the line as its side. Red if which edges are one line is
    /// decided by their rows' values.
    #[test]
    fn rows_joined_at_a_vertex_are_one_line_whatever_values_they_hold() {
        for (kind, s) in [("cylinder", cylinder()), ("torus", ring(2.0))] {
            let b = stepped([0.5, 0.5], [5, 5], up());
            assert_eq!(
                chart(&s, &b, &[0, 1], BELOW.0, BELOW.1),
                Read::Vacuous,
                "{kind}: the cell below"
            );
            assert!(
                matches!(chart(&s, &b, &[0, 1], ABOVE.0, ABOVE.1), Read::Of(..)),
                "{kind}: the cell above: {:?}",
                chart(&s, &b, &[0, 1], ABOVE.0, ABOVE.1)
            );
        }
    }

    /// **Two lines on one side stay two lines**: the second edge is on
    /// the line `v = 0.55`, starting at a vertex of its own above the
    /// first's end. Below both, the first covers half the cell's side
    /// and the second is no part of its line; between them, two lines
    /// bound the cell on one side. Red if edges are one line with no
    /// vertex joining them.
    #[test]
    fn two_lines_on_one_side_stay_two_lines() {
        let b = stepped([0.5, 0.5], [5, 6], 0.55);
        let s = cylinder();
        assert_eq!(chart(&s, &b, &[0, 1], BELOW.0, BELOW.1), Read::Local);
        assert_eq!(chart(&s, &b, &[0, 1], ABOVE.0, ABOVE.1), Read::Local);
    }

    /// **Edges with the face on opposite sides are not joined**
    /// ([`vacuous`]'s line): a slit's two edges on one line, the one
    /// ending where the other starts, handed to [`vacuous`] as lines the
    /// cell lies outside of. Red if the line's edges are gathered
    /// whatever side their face is on.
    #[test]
    fn edges_with_the_face_on_opposite_sides_are_not_joined() {
        let b = stepped([0.5, 0.5], [5, 5], 0.5);
        let s = cylinder();
        let line = iv0(0.5);
        assert!(
            vacuous(
                &b,
                &s,
                BELOW,
                &[(Side::YLo, line, 0), (Side::YLo, line, 1)],
                false
            ),
            "the control: both edges with the face above"
        );
        assert!(!vacuous(
            &b,
            &s,
            BELOW,
            &[(Side::YLo, line, 0), (Side::YHi, line, 1)],
            false
        ));
    }

    /// **A spline chart's edges are not joined** ([`joints`]): a closed
    /// net carries one point at two parameters of a line, so a shared
    /// vertex does not put two ranges end to end. The patch's one edge
    /// over the whole line is the control. Red if a joint is read on a
    /// free-form chart.
    #[test]
    fn a_spline_charts_edges_are_not_joined() {
        let s = sheet();
        let whole = Boundary {
            edges: vec![rim(1, [1, 2], Axis::V, 0.5, (0.2, 0.5), (0.8, 0.5))],
            ..stepped([0.5, 0.5], [5, 5], 0.5)
        };
        assert_eq!(chart(&s, &whole, &[0], BELOW.0, BELOW.1), Read::Vacuous);
        assert!(matches!(
            chart(&s, &whole, &[0], ABOVE.0, ABOVE.1),
            Read::Of(..)
        ));
        let b = stepped([0.5, up()], [5, 5], 0.5);
        for cell in [BELOW, ABOVE] {
            assert_eq!(chart(&s, &b, &[0, 1], cell.0, cell.1), Read::Local);
        }
        // The hazard: one edge ends at `u = 1`, the next starts at
        // `u = 0`, one vertex by key, and the line has no edge over
        // `(0.3, 0.5)`.
        let closed = Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 5], Axis::V, 0.5, (0.5, 0.5), (1.0, 0.5)),
                rim(2, [5, 2], Axis::V, 0.5, (0.0, 0.5), (0.3, 0.5)),
            ],
            vertices: BTreeMap::new(),
            extent: Some([(0.0, 1.0), (0.0, 1.0)]),
            outer: None,
        };
        assert_eq!(
            chart(&s, &closed, &[0, 1], (0.35, 0.45), BELOW.1),
            Read::Local,
            "a cell the line has no edge beside"
        );
    }

    /// **A line's edges joined across the chart's seam cover a side
    /// together**: a parallel of the cylinder's chart whose first edge
    /// ends at `u = 2π` and whose second starts, at the same vertex, a
    /// turn below and a few ulps on. The cells across the joint read as
    /// across any other, on either sheet; a cell where the line has no
    /// edge is not read vacuous. Red if the second end is not moved by
    /// whole turns to the first (the range between them is then most of
    /// the turn, and covers the cell the line has no edge beside).
    #[test]
    fn a_lines_edges_joined_across_the_seam_cover_a_side_together() {
        let s = cylinder();
        let b = Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 5], Axis::V, 0.5, (5.5, 0.5), (TURN, 0.5)),
                rim(2, [5, 2], Axis::V, 0.5, (4e-15, 0.5), (0.7, 0.5)),
            ],
            vertices: BTreeMap::new(),
            extent: Some([(0.0, TURN), (0.0, 1.0)]),
            outer: None,
        };
        for turn in [0.0, TURN] {
            let u = (turn - 0.05, turn + 0.05);
            assert_eq!(
                chart(&s, &b, &[0, 1], u, BELOW.1),
                Read::Vacuous,
                "{turn} round: the cell below"
            );
            assert!(
                matches!(chart(&s, &b, &[0, 1], u, ABOVE.1), Read::Of(..)),
                "{turn} round: the cell above: {:?}",
                chart(&s, &b, &[0, 1], u, ABOVE.1)
            );
        }
        assert_eq!(
            chart(&s, &b, &[0, 1], (3.0, 3.1), BELOW.1),
            Read::Local,
            "a cell the line has no edge beside"
        );
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
            ends_here: Vec::new(),
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
            ends_here: Vec::new(),
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
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
        };
        // The edge from (0, 0) to (4, 0), run forward: the face above it.
        let b = Boundary {
            sense: true,
            edges: vec![Rim {
                stated: None,
                key: edge(1),
                arc: Some((
                    Curve3::line(origin(), geom_core::UnitVec3::exact_x()),
                    (iv0(0.0), iv0(4.0)),
                )),
                ends: [vertex(1), vertex(2)],
                uses: vec![Use {
                    plus: true,
                    image: None,
                    iso: None,
                    curve: None,
                }],
            }],
            vertices: BTreeMap::new(),
            extent: None,
            outer: None,
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
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
        };
        let line = |d: geom_core::UnitVec3<Interval>, far: u64, plus: bool| Rim {
            stated: None,
            key: edge(far),
            arc: Some((Curve3::line(origin(), d), (iv0(0.0), iv0(4.0)))),
            ends: [vertex(1), vertex(far)],
            uses: vec![Use {
                plus,
                image: None,
                iso: None,
                curve: None,
            }],
        };
        let y = geom_core::UnitVec3::<Interval>::exact_y();
        let x = geom_core::UnitVec3::<Interval>::exact_x();
        // Forward: (0, 4) to the vertex at the origin to (4, 0), the
        // first quadrant on its left.
        let corner = |forward: bool, sense: bool| Boundary {
            sense,
            edges: vec![line(y, 2, !forward), line(x, 3, forward)],
            vertices: BTreeMap::new(),
            extent: None,
            outer: None,
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

    /// A side of a plane face: its key, its chart ends in its run, and
    /// its start and end vertices.
    type Seg = (u64, (f64, f64), (f64, f64), [u64; 2]);

    /// The plane through `o` with axes `u` and `v`, normal `u × v`, and a
    /// face of it bounded by line segments, each run with the face on
    /// its left in the chart.
    fn faced(
        o: [f64; 3],
        u: [f64; 3],
        v: [f64; 3],
        sides: &[Seg],
    ) -> (Surface<Interval>, Boundary) {
        let vec = |a: [f64; 3]| Vec3::new(iv0(a[0]), iv0(a[1]), iv0(a[2]));
        let at = |(x, y): (f64, f64)| {
            Point3::new(
                iv0(o[0] + x * u[0] + y * v[0]),
                iv0(o[1] + x * u[1] + y * v[1]),
                iv0(o[2] + x * u[2] + y * v[2]),
            )
        };
        let normal = vec(u).cross(vec(v));
        let plane = Surface::Plane {
            frame: geom::Frame::new(at((0.0, 0.0)), normal, vec(u)),
        };
        let rim = |&(key, p, q, [s, e]): &Seg| Rim {
            stated: None,
            key: edge(key),
            arc: Some(chord_segment(at(p), at(q))),
            ends: [vertex(s), vertex(e)],
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                curve: None,
            }],
        };
        let b = Boundary {
            sense: true,
            edges: sides.iter().map(rim).collect(),
            vertices: BTreeMap::new(),
            extent: None,
            outer: None,
        };
        (plane, b)
    }

    /// A face of the plane `z = 0`, `u_ref = x`.
    fn segments(sides: &[Seg]) -> (Surface<Interval>, Boundary) {
        faced([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], sides)
    }

    /// The edges `held` (indices) and the vertices `at` a cell holds.
    fn holding(held: &[usize], at: &[u64]) -> Local {
        Local {
            edges: held.to_vec(),
            vertices: at.iter().map(|&n| vertex(n)).collect(),
        }
    }

    /// [`present_plane`] of `b` in the cell `u × v` holding `local`,
    /// `shared` the indices of the edges it shares.
    fn present_in<'a>(
        plane: &'a Surface<Interval>,
        b: &'a Boundary,
        cell: ((f64, f64), (f64, f64)),
        local: &'a Local,
        shared: &[usize],
    ) -> Result<Presentation<'a>, Unpresented> {
        present_near(plane, b, cell, local, (shared, &[]))
    }

    /// [`present_in`], the pair sharing the vertices `near` as well.
    fn present_near<'a>(
        plane: &'a Surface<Interval>,
        b: &'a Boundary,
        (u, v): ((f64, f64), (f64, f64)),
        local: &'a Local,
        (shared, near): (&[usize], &[u64]),
    ) -> Result<Presentation<'a>, Unpresented> {
        let near: Vec<VertexKey> = near.iter().map(|&n| vertex(n)).collect();
        let piece = Piece {
            face: FaceKey::default(),
            surface: plane,
            whole: false,
            boundary: b,
            u,
            v,
            local,
            ball: Ball::holding(origin(), origin()),
        };
        present_plane(&piece, shared, &near, band())
    }

    /// The rectangle `[0, 10] × [0, 10⁻³]`, run counter-clockwise from
    /// the origin: its bottom, right, top and left sides, edges 1 to 4.
    fn thin_face() -> (Surface<Interval>, Boundary) {
        let (l, w) = (10.0, 1e-3);
        segments(&[
            (1, (0.0, 0.0), (l, 0.0), [1, 2]),
            (2, (l, 0.0), (l, w), [2, 3]),
            (3, (l, w), (0.0, w), [3, 4]),
            (4, (0.0, w), (0.0, 0.0), [4, 1]),
        ])
    }

    /// **`present_plane` does not read a strip's middle**: a cell across a
    /// thin face, holding both its long sides and no end, is `Local`
    /// whichever side is shared. A cell holding one side and a lone
    /// vertex is `Local`: the side's chord is not all the boundary there.
    /// Red if a strip's middle is presented again without a body that
    /// needs it (`work/embed/a-plane-cell-holding-both-sides-of-a-thin-face-is-read-only-below-its-width.md`).
    #[test]
    fn a_strip_middle_is_not_read() {
        let (plane, b) = thin_face();
        let cell = ((4.0, 5.0), (-1e-3, 2e-3));
        let pair = holding(&[0, 2], &[]);
        for shared in [&[][..], &[0], &[2], &[0, 2]] {
            assert!(
                matches!(
                    present_in(&plane, &b, cell, &pair, shared),
                    Err(Unpresented::Local)
                ),
                "{shared:?} shared: a strip's middle is Local"
            );
        }
        assert!(
            matches!(
                present_in(&plane, &b, cell, &holding(&[0], &[9]), &[0]),
                Err(Unpresented::Local)
            ),
            "a lone vertex in the cell beside one side: the side's chord is not all there is"
        );
    }

    /// **`present_plane`, a strip's end**: a cell holding a thin face's
    /// whole short side with both its neighbours presents the face's
    /// end, read at one corner and capped at the other's neighbour: the
    /// short side shared is the zero side along its whole length, a long
    /// side shared the zero side along the cell's reach, both shared the
    /// corner. Red on a planar arm that reads three lines as `Local`.
    #[test]
    fn a_cell_holding_a_thin_faces_end_presents_it() {
        let (plane, b) = thin_face();
        let cell = ((9.99, 10.5), (-1.0, 1.0));
        let end = holding(&[0, 1, 2], &[2, 3]);
        let read = |shared: &[usize]| present_in(&plane, &b, cell, &end, shared);
        match read(&[1]) {
            Ok(Presentation::Of(q)) => {
                assert!(
                    matches!(&q.zero, Zero::Side(_, a) if a.edges == [edge(2)] && a.on_edges),
                    "the short side shared is the zero side, on its edge"
                );
                let image = q.whole();
                assert!(
                    -1e-12 < image.y.lo()
                        && image.y.hi() < 1e-3 + 1e-12
                        && 9.99 - 1e-9 < image.x.lo()
                        && image.x.hi() < 10.0 + 1e-12,
                    "the image is the face's end in the cell: {image:?}"
                );
            }
            _ => panic!("the end is presented, its short side shared"),
        }
        assert!(
            matches!(read(&[0]), Ok(Presentation::Of(q))
                if matches!(&q.zero, Zero::Side(_, a) if a.edges == [edge(1)])),
            "a long side shared is the zero side"
        );
        assert!(
            matches!(read(&[2]), Ok(Presentation::Of(q))
                if matches!(&q.zero, Zero::Side(_, a) if a.edges == [edge(3)])),
            "the other long side shared is the zero side"
        );
        assert!(
            matches!(read(&[0, 1]), Ok(Presentation::Of(q))
                if matches!(&q.zero, Zero::Corner(..))),
            "a long side and the short one shared are the corner"
        );
        assert!(
            matches!(read(&[0, 2]), Err(Unpresented::Local)),
            "both long sides shared: no one corner holds the zero"
        );
        assert!(
            matches!(read(&[]), Ok(Presentation::Of(q)) if matches!(q.zero, Zero::None)),
            "nothing shared: the end is presented with no zero"
        );
        assert!(
            matches!(
                present_near(&plane, &b, cell, &end, (&[], &[2])),
                Ok(Presentation::Of(q)) if matches!(q.zero, Zero::Point(_, _, v) if v == vertex(2))
            ),
            "a vertex shared alone at one corner: read at that corner, the other neighbour capping"
        );
        assert!(
            matches!(
                present_in(&plane, &b, cell, &holding(&[0, 1, 2], &[1, 2, 3]), &[1]),
                Err(Unpresented::Local)
            ),
            "a vertex past the end's two in the cell: more than the end"
        );
        // A sliver triangle: its sides meet at its apex, out of the cell
        // at its short side, and in the cell holding it whole.
        let (plane, triangle) = segments(&[
            (1, (0.0, 0.0), (10.0, 0.0), [1, 2]),
            (2, (10.0, 0.0), (10.0, 1e-3), [2, 3]),
            (3, (10.0, 1e-3), (0.0, 0.0), [3, 1]),
        ]);
        assert!(
            matches!(
                present_in(&plane, &triangle, cell, &end, &[1]),
                Ok(Presentation::Of(q)) if matches!(&q.zero, Zero::Side(_, a) if a.edges == [edge(2)])
            ),
            "a sliver triangle's short side is a strip's end"
        );
        assert!(
            matches!(
                present_in(
                    &plane,
                    &triangle,
                    ((-1.0, 11.0), (-1.0, 1.0)),
                    &holding(&[0, 1, 2], &[1, 2, 3]),
                    &[1]
                ),
                Err(Unpresented::Local)
            ),
            "a whole triangle in the cell is more than an end"
        );
        // An end whose sides splay apart: the shared short side is the
        // zero side past its own end, the rectangle reaching the top's
        // line, so it does not lie on its edge.
        let (plane, splayed) = segments(&[
            (1, (0.0, 0.0), (10.0, 0.0), [1, 2]),
            (2, (10.0, 0.0), (10.0, 1e-3), [2, 3]),
            (3, (10.0, 1e-3), (0.0, 2e-3), [3, 4]),
        ]);
        assert!(
            matches!(
                present_in(&plane, &splayed, cell, &end, &[1]),
                Ok(Presentation::Of(q)) if matches!(&q.zero, Zero::Side(_, a) if a.edges == [edge(2)] && !a.on_edges)
            ),
            "a short side read past its end is not on its edge"
        );
    }

    /// **Lines that are not of one convex piece are no strip's end**:
    /// the two sides of a slot or of a hole, each with the face on its far
    /// side from the other; two sides whose face lies above both; a slit's
    /// two sides, meeting at its tip; a rim and a hole's side; and ends
    /// whose far corner turns in, whose sides cross in the cell, or whose
    /// three edges meet at one vertex. Each is `Local`. Red if the end's
    /// leans read a face side the wrong way, or not at all, or if two
    /// lines with no common end are presented.
    #[test]
    fn lines_not_of_one_convex_end_are_local() {
        let (l, w) = (10.0, 1e-3);
        let cell = ((4.0, 5.0), (-1e-3, 2e-3));
        // A hole's two long sides, run clockwise: the face below the one
        // and above the other.
        let (plane, hole) = segments(&[
            (1, (l, 0.0), (0.0, 0.0), [2, 1]),
            (3, (0.0, w), (l, w), [4, 3]),
        ]);
        for shared in [&[][..], &[0], &[1]] {
            assert!(
                matches!(
                    present_in(&plane, &hole, cell, &holding(&[0, 1], &[]), shared),
                    Err(Unpresented::Local)
                ),
                "a hole's two sides, {shared:?} shared, are no strip"
            );
        }
        // Two sides whose face lies above both: the lower one stands off
        // the upper's face side, so they bound no region between them,
        // and the rectangle they would cap has no height.
        let (plane, stacked) = segments(&[
            (1, (0.0, 0.0), (l, 0.0), [1, 2]),
            (3, (0.0, -w), (l, -w), [3, 4]),
        ]);
        assert!(
            matches!(
                present_in(&plane, &stacked, cell, &holding(&[0, 1], &[]), &[0]),
                Err(Unpresented::Local)
            ),
            "a side below the shared one, its face above it, caps nothing"
        );
        // A slit from the origin along `x`: its two sides meet at its tip.
        let (plane, slit) = segments(&[
            (1, (l, 0.0), (0.0, 0.0), [2, 1]),
            (3, (0.0, 0.0), (l, w), [1, 3]),
        ]);
        assert!(
            matches!(
                present_in(
                    &plane,
                    &slit,
                    ((-1.0, 1.0), (-1.0, 1.0)),
                    &holding(&[0, 1], &[1]),
                    &[0]
                ),
                Err(Unpresented::Local)
            ),
            "a slit's two sides at its tip are no corner"
        );
        // A thin face's end whose far corner turns in: its top side runs
        // back from `(l, w)` toward the bottom's start.
        let (plane, notched) = segments(&[
            (1, (0.0, 0.0), (l, 0.0), [1, 2]),
            (2, (l, 0.0), (l, w), [2, 3]),
            (3, (l, w), (l + 1.0, 2.0 * w), [3, 4]),
        ]);
        let end = holding(&[0, 1, 2], &[2, 3]);
        let at_end = ((9.99, 10.5), (-1.0, 1.0));
        for shared in [&[1][..], &[0]] {
            assert!(
                matches!(
                    present_in(&plane, &notched, at_end, &end, shared),
                    Err(Unpresented::Local)
                ),
                "an end with a reflex corner, {shared:?} shared, is no strip's end"
            );
        }
        // A thin face's end whose top side, past a convex corner, runs
        // down across the bottom's line inside the cell.
        let (plane, crossing) = segments(&[
            (1, (0.0, 0.0), (l, 0.0), [1, 2]),
            (2, (l, 0.0), (l, w), [2, 3]),
            (3, (l, w), (l - 0.005, -w), [3, 4]),
        ]);
        for shared in [&[1][..], &[0]] {
            assert!(
                matches!(
                    present_in(&plane, &crossing, at_end, &end, shared),
                    Err(Unpresented::Local)
                ),
                "an end whose sides cross in the cell, {shared:?} shared, is no strip's end"
            );
        }
        // Three edges at one vertex: the cap's line meets the corner's,
        // so it caps nothing.
        let (plane, three) = segments(&[
            (1, (0.0, 0.0), (l, 0.0), [1, 2]),
            (2, (0.0, 0.0), (0.0, w), [1, 3]),
            (3, (0.0, 0.0), (-l, w), [1, 4]),
        ]);
        assert!(
            matches!(
                present_in(
                    &plane,
                    &three,
                    ((-1.0, 1.0), (-1.0, 1.0)),
                    &holding(&[0, 1, 2], &[1]),
                    &[0]
                ),
                Err(Unpresented::Local)
            ),
            "three edges at one vertex are no strip's end"
        );
        // The face's rim along `y = 0` and a hole's side at `y = w`, its
        // hole above: a strip's middle, which is not read.
        let (plane, beside) = segments(&[
            (1, (0.0, 0.0), (l, 0.0), [1, 2]),
            (3, (l, w), (0.0, w), [4, 3]),
        ]);
        assert!(
            matches!(
                present_in(&plane, &beside, cell, &holding(&[0, 1], &[]), &[0]),
                Err(Unpresented::Local)
            ),
            "a rim and a hole facing it: a strip's middle, Local"
        );
    }

    /// [`certify_pair`] of a thin face's end against a face sharing its
    /// short side: the face `[0, 10] × [0, 10⁻³]` of the plane `y = 0`
    /// (chart `(x, z)`, outward `−y`), its short side `x = 10` shared
    /// (edge 2, vertices 2 and 3), each face read in a cell holding the
    /// shared side and both its neighbours.
    fn against_the_end(
        other: (Surface<Interval>, Boundary),
        cell: ((f64, f64), (f64, f64)),
        held: &[usize],
        at: usize,
    ) -> Outcome {
        let w = 1e-3;
        let (sa, a) = faced(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            &[
                (1, (0.0, 0.0), (10.0, 0.0), [1, 2]),
                (2, (10.0, 0.0), (10.0, w), [2, 3]),
                (3, (10.0, w), (0.0, w), [3, 4]),
                (4, (0.0, w), (0.0, 0.0), [4, 1]),
            ],
        );
        let (sb, b) = other;
        let la = Local {
            edges: vec![0, 1, 2],
            vertices: vec![vertex(2), vertex(3)],
        };
        let lb = Local {
            edges: held.to_vec(),
            vertices: vec![vertex(2), vertex(3)],
        };
        let piece = |surface, boundary, (u, v), local, face| Piece {
            face,
            surface,
            whole: false,
            boundary,
            u,
            v,
            local,
            ball: Ball::holding(origin(), origin()),
        };
        certify_pair(
            &piece(&sa, &a, ((9.99, 10.5), (-1.0, 1.0)), &la, face(1)),
            &piece(&sb, &b, cell, &lb, face(2)),
            &Near {
                edges: vec![(1, at)],
                vertices: vec![vertex(2), vertex(3)],
            },
            band(),
        )
    }

    /// **A thin face's end certifies against the face across its short
    /// side, and not against one folded back over it**: the slab's end,
    /// the plane `x = 10` (chart `(y, z)`) beside it at a right angle,
    /// both thin, certifies; a face of the plane `y = 0` itself, outward
    /// `+y`, over the end's last unit — M folded back onto itself along
    /// the shared side — does not. Red on a planar arm that reads the end
    /// as `Local` (the first), or that reads the fold apart (the second).
    #[test]
    fn a_thin_faces_end_certifies_across_its_short_side_and_not_folded() {
        let w = 1e-3;
        let beside = faced(
            [10.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            &[
                (5, (0.0, 0.0), (10.0, 0.0), [2, 5]),
                (6, (10.0, 0.0), (10.0, w), [5, 6]),
                (7, (10.0, w), (0.0, w), [6, 3]),
                (2, (0.0, w), (0.0, 0.0), [3, 2]),
            ],
        );
        let got = against_the_end(beside, ((-0.5, 0.01), (-1.0, 1.0)), &[0, 2, 3], 3);
        assert!(
            matches!(got, Outcome::Certified { .. }),
            "the end and the face beside it certify: {got:?}"
        );
        let folded = faced(
            [0.0; 3],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            &[
                (5, (0.0, 9.0), (w, 9.0), [5, 6]),
                (6, (w, 9.0), (w, 10.0), [6, 3]),
                (2, (w, 10.0), (0.0, 10.0), [3, 2]),
                (7, (0.0, 10.0), (0.0, 9.0), [2, 5]),
            ],
        );
        let got = against_the_end(folded, ((-1.0, 1.0), (9.99, 10.5)), &[1, 2, 3], 2);
        assert!(
            !matches!(got, Outcome::Certified { .. }),
            "the fold is not certified: {got:?}"
        );
    }

    /// **A cell off the frame's face side, across a further edge whose
    /// face lies beyond it, holds the face there**: a slot's two sides (a
    /// cell past its far side), a reflex wedge's (its tip clear of the
    /// cell), and an end whose far corner turns in (a cell past its short
    /// side). None is `Vacuous`. Red where a frame's `Vacuous` is read
    /// before its cap's leans.
    #[test]
    fn a_cell_past_a_far_edge_facing_away_is_not_vacuous() {
        let (l, w) = (10.0, 1e-3);
        let across = ((4.0, 5.0), (2e-4, 3e-3));
        let pair = holding(&[0, 1], &[]);
        let (plane, slot) = segments(&[
            (1, (l, 0.0), (0.0, 0.0), [2, 1]),
            (3, (0.0, w), (l, w), [4, 3]),
        ]);
        for shared in [&[][..], &[0]] {
            assert!(
                !matches!(
                    present_in(&plane, &slot, across, &pair, shared),
                    Ok(Presentation::Vacuous)
                ),
                "a slot, {shared:?} shared: the cell holds the face past its far side"
            );
        }
        let (plane, slit) = segments(&[
            (1, (l, 0.0), (0.0, 0.0), [2, 1]),
            (3, (0.0, 0.0), (l, w), [1, 3]),
        ]);
        assert!(
            !matches!(
                present_in(&plane, &slit, across, &pair, &[0]),
                Ok(Presentation::Vacuous)
            ),
            "a reflex wedge: the cell holds the face past its far side"
        );
        let (plane, notched) = segments(&[
            (1, (0.0, 0.0), (l, 0.0), [1, 2]),
            (2, (l, 0.0), (l, w), [2, 3]),
            (3, (l, w), (l + 1.0, 2.0 * w), [3, 4]),
        ]);
        assert!(
            !matches!(
                present_in(
                    &plane,
                    &notched,
                    ((10.001, 10.5), (-1.0, 1.0)),
                    &holding(&[0, 1, 2], &[2, 3]),
                    &[0]
                ),
                Ok(Presentation::Vacuous)
            ),
            "a reflex end: the cell holds the face past its short side"
        );
    }

    /// `b`'s local boundary in the ball about the cell `u × v` of the
    /// plane `z = 0`, by [`Local::of`] (its vertices filled from its
    /// edges' ends), and that ball.
    fn real_local(b: &mut Boundary, (u, v): ((f64, f64), (f64, f64))) -> (Local, Ball) {
        for e in &b.edges {
            let (c, (t0, t1)) = e.arc.as_ref().expect("a line");
            b.vertices.insert(e.ends[0], c.eval(*t0));
            b.vertices.insert(e.ends[1], c.eval(*t1));
        }
        let centre = Point3::new(iv0(0.5 * (u.0 + u.1)), iv0(0.5 * (v.0 + v.1)), iv0(0.0));
        let patch = Point3::new(
            Interval::from_bounds(u.0, u.1),
            Interval::from_bounds(v.0, v.1),
            iv0(0.0),
        );
        let ball = Ball::holding(centre, patch);
        (Local::of(b, ball, band()), ball)
    }

    /// A cell of a chart, `u × v`.
    type Cell = ((f64, f64), (f64, f64));

    /// [`certify_pair`] of `f` and `g`, faces of the plane `z = 0` in the
    /// cells `cf` and `cg`, each cell's local boundary read by
    /// [`Local::of`] from its own ball, sharing `near`.
    fn certify_in_balls(
        (mut f, cf): (Boundary, Cell),
        (mut g, cg): (Boundary, Cell),
        near: Near,
    ) -> (Outcome, [Local; 2]) {
        let plane = segments(&[]).0;
        let (lf, bf) = real_local(&mut f, cf);
        let (lg, bg) = real_local(&mut g, cg);
        let piece = |boundary, (u, v): ((f64, f64), (f64, f64)), local, ball, n| Piece {
            face: face(n),
            surface: &plane,
            whole: false,
            boundary,
            u,
            v,
            local,
            ball,
        };
        let got = certify_pair(
            &piece(&f, cf, &lf, bf, 1),
            &piece(&g, cg, &lg, bg, 2),
            &near,
            band(),
        );
        (got, [lf, lg])
    }

    /// **A face laid over a slot's far side is not certified**: F, the
    /// plane with a slot `0 < y < w`; G, a face of the same plane sharing
    /// the slot's lower side, over the slot and on over F to `y = 5`, M
    /// folded onto itself past the far side. F's cell lies across the far
    /// side and holds points of F that G covers. Red where a frame reads
    /// `Vacuous` before its cap: the pair then certifies.
    #[test]
    fn a_face_laid_over_a_slots_far_side_is_not_certified() {
        let (l, w) = (10.0, 1e-3);
        let (_, f) = segments(&[
            (1, (l, 0.0), (0.0, 0.0), [2, 1]),
            (3, (0.0, w), (l, w), [4, 3]),
        ]);
        let (_, g) = segments(&[
            (1, (0.0, 0.0), (l, 0.0), [1, 2]),
            (5, (l, 0.0), (l, 5.0), [2, 5]),
            (6, (l, 5.0), (0.0, 5.0), [5, 6]),
            (7, (0.0, 5.0), (0.0, 0.0), [6, 1]),
        ]);
        let (got, [lf, lg]) = certify_in_balls(
            (f, ((4.0, 5.0), (2e-4, 3e-3))),
            (g, ((4.0, 5.0), (-1e-3, 3e-3))),
            Near {
                edges: vec![(0, 0)],
                vertices: vec![],
            },
        );
        assert_eq!(
            (&lf.edges, lf.vertices.len(), &lg.edges, lg.vertices.len()),
            (&vec![0, 1], 0, &vec![0], 0),
            "F's cell holds both slot sides, G's the shared side, no vertex"
        );
        assert!(
            !matches!(got, Outcome::Certified { .. }),
            "the fold past the far side is not certified: {got:?}"
        );
    }

    /// **A strip's end read off a cell's own ball**: a thin face's end,
    /// the cell's local boundary read by [`Local::of`] (its short side and
    /// both neighbours), presents with the short side as the zero; its
    /// middle, both long sides, is `Local`. Red where the end is `Local`.
    #[test]
    fn a_strip_end_read_off_a_real_ball_presents() {
        let (l, w) = (10.0, 1e-3);
        let thin = || {
            segments(&[
                (1, (0.0, 0.0), (l, 0.0), [1, 2]),
                (2, (l, 0.0), (l, w), [2, 3]),
                (3, (l, w), (0.0, w), [3, 4]),
                (4, (0.0, w), (0.0, 0.0), [4, 1]),
            ])
        };
        for (what, cell, shared, held, presented) in [
            (
                "the end",
                ((9.99, 10.5), (-1.0, 1.0)),
                1,
                vec![0, 1, 2],
                true,
            ),
            (
                "the middle",
                ((4.0, 5.0), (-1e-4, 2e-3)),
                0,
                vec![0, 2],
                false,
            ),
        ] {
            let (plane, mut b) = thin();
            let (local, ball) = real_local(&mut b, cell);
            assert_eq!(local.edges, held, "{what}: the cell's own boundary");
            let piece = Piece {
                face: FaceKey::default(),
                surface: &plane,
                whole: false,
                boundary: &b,
                u: cell.0,
                v: cell.1,
                local: &local,
                ball,
            };
            let got = present_plane(&piece, &[shared], &[], band());
            if presented {
                assert!(
                    matches!(&got, Ok(Presentation::Of(q)) if matches!(&q.zero, Zero::Side(_, a) if a.edges == [edge(2)])),
                    "{what}: presented, its short side the zero"
                );
            } else {
                assert!(matches!(got, Err(Unpresented::Local)), "{what}: Local");
            }
        }
    }

    /// **A strip's end turned in its chart presents its short side**, at
    /// five turns, `w` a millimetre and `100Kε`, in square cells about the
    /// short side from 0.2 down to `10w`. Red where the cap's lean picks
    /// up the turn's slack over the reach, read per chart coordinate.
    #[test]
    fn a_strip_end_turned_in_its_chart_presents_its_short_side() {
        let l = 10.0;
        for w in [1e-3_f64, 100.0 * band().escalate()] {
            for turn in [0.0_f64, 0.1, 0.7, 1.3, 2.0] {
                let (c, s) = (turn.cos(), turn.sin());
                let r = |(x, y): (f64, f64)| (c * x - s * y, s * x + c * y);
                let (plane, b) = segments(&[
                    (1, r((0.0, 0.0)), r((l, 0.0)), [1, 2]),
                    (2, r((l, 0.0)), r((l, w)), [2, 3]),
                    (3, r((l, w)), r((0.0, w)), [3, 4]),
                ]);
                let (cx, cy) = r((l, 0.5 * w));
                for half in [0.2, 1e-2, 10.0 * w] {
                    let cell = ((cx - half, cx + half), (cy - half, cy + half));
                    assert!(
                        matches!(
                            present_in(&plane, &b, cell, &holding(&[0, 1, 2], &[2, 3]), &[1]),
                            Ok(Presentation::Of(q)) if matches!(&q.zero, Zero::Side(_, a) if a.edges == [edge(2)])
                        ),
                        "w = {w:e}, turned {turn}, half-size {half:e}: the short side is the zero side"
                    );
                }
            }
        }
    }

    /// **A corner's lean is read at the band**: a strip's end `Kε/2` wide
    /// is not read, its corner's far end standing in the band off the
    /// other edge (it refuses before the cap is read); `2Kε` wide, it is.
    /// Red if the corner's leans are read as signs alone.
    #[test]
    fn an_end_narrower_than_the_band_is_not_read() {
        let k = band().escalate();
        for (w, presented) in [(0.5 * k, false), (2.0 * k, true)] {
            let (plane, b) = segments(&[
                (1, (0.0, 0.0), (10.0, 0.0), [1, 2]),
                (2, (10.0, 0.0), (10.0, w), [2, 3]),
                (3, (10.0, w), (0.0, w), [3, 4]),
            ]);
            let end = holding(&[0, 1, 2], &[2, 3]);
            let got = present_in(&plane, &b, ((9.99, 10.5), (-1.0, 1.0)), &end, &[1]);
            assert_eq!(
                matches!(got, Ok(Presentation::Of(_))),
                presented,
                "an end {w:e} wide: presented = {presented}"
            );
        }
    }

    /// **A cap's lean is read at the band**: a sliver triangle's end in a
    /// cell running along its sides to where they stand `gap` apart, so
    /// the cap's lean over the base comes down to `gap` over the reach
    /// while the corner's own lean, the end's width, is a millimetre: not
    /// read at `0.05Kε`, read at `50Kε`. Red if the cap's lean is read as a
    /// sign alone.
    #[test]
    fn a_cap_lean_inside_the_band_is_not_read() {
        let (l, w) = (10.0, 1e-3);
        let (plane, triangle) = segments(&[
            (1, (0.0, 0.0), (l, 0.0), [1, 2]),
            (2, (l, 0.0), (l, w), [2, 3]),
            (3, (l, w), (0.0, 0.0), [3, 1]),
        ]);
        let end = holding(&[0, 1, 2], &[2, 3]);
        let k = band().escalate();
        for (gap, presented) in [(0.05 * k, false), (50.0 * k, true)] {
            let from = gap * l / w;
            let got = present_in(&plane, &triangle, ((from, 10.5), (-1.0, 1.0)), &end, &[1]);
            assert_eq!(
                matches!(got, Ok(Presentation::Of(_))),
                presented,
                "a cap standing {gap:e} off its base at the cell's far reach: presented = {presented}"
            );
        }
    }

    /// **A cap's lean is read over a reach from the corner**: an end whose
    /// far corner turns in (the cap runs back across the base's line past
    /// the frame's corner), in a cell off that corner, read off the cell's
    /// own ball. The cap's lean at the corner itself refuses it; a reach
    /// starting at the cell's own range misses the corner, presents, and
    /// drops the face point here (the fuzz's seed `0x1234`, effort 15,
    /// case 255). Red if the reach does not start at the corner.
    #[test]
    fn a_caps_reach_starts_at_the_corner() {
        let p1 = (-0.042105173597328_35, 0.280122695629587_14);
        let p2 = (-0.042105241419911_864, 0.280119192265625_37);
        let (_, mut b) = segments(&[
            (1, (334.4823095921917, -942.1069612171361), p1, [1, 2]),
            (2, p1, p2, [2, 3]),
            (3, p2, (-875.6944406777136, -482.6618933154106), [3, 4]),
        ]);
        let cell = (
            (-0.042102506762834536, -0.04209062058628329),
            (0.2801107907952337, 0.2801305986514572),
        );
        let (local, ball) = real_local(&mut b, cell);
        let plane = segments(&[]).0;
        let piece = Piece {
            face: FaceKey::default(),
            surface: &plane,
            whole: false,
            boundary: &b,
            u: cell.0,
            v: cell.1,
            local: &local,
            ball,
        };
        assert_eq!(
            local.edges,
            vec![0, 1, 2],
            "the cell holds the end's three edges"
        );
        match present_plane(&piece, &[0], &[], band()) {
            Ok(Presentation::Of(q)) => {
                let Map::Affine { o, a, b: bv } = q.map else {
                    panic!("a planar presentation is affine")
                };
                let m = |x: Interval| 0.5 * (x.lo() + x.hi());
                let (o, a, bv) = ([m(o[0]), m(o[1])], [m(a[0]), m(a[1])], [m(bv[0]), m(bv[1])]);
                let p = (-0.04210201150547824, 0.28011161612257635);
                let d = [p.0 - o[0], p.1 - o[1]];
                let det = a[0] * bv[1] - a[1] * bv[0];
                let (x, y) = (
                    (d[0] * bv[1] - d[1] * bv[0]) / det,
                    (a[0] * d[1] - a[1] * d[0]) / det,
                );
                assert!(
                    q.x.0 - 1e-7 <= x
                        && x <= q.x.1 + 1e-7
                        && q.y.0 - 1e-7 <= y
                        && y <= q.y.1 + 1e-7,
                    "the face point {p:?} lies in the rectangle: ({x}, {y}) in {:?} × {:?}",
                    q.x,
                    q.y
                );
            }
            Ok(Presentation::Vacuous) => panic!("the cell holds face points"),
            Err(_) => {}
        }
    }

    /// **A structural refusal at a strip's end stands**: an end whose
    /// shared short side is a seam's (two uses), which no planar reading
    /// reads, is `Structural` from either corner, not overwritten by the
    /// second corner's `Local`. Red if the end returns the last corner's
    /// refusal whatever it is.
    #[test]
    fn a_structural_refusal_at_an_end_stands() {
        let (plane, mut b) = thin_face();
        let seam = b.edges[1].uses[0].clone();
        b.edges[1].uses.push(Use {
            plus: !seam.plus,
            ..seam
        });
        let end = holding(&[0, 1, 2], &[2, 3]);
        let got = present_in(&plane, &b, ((9.99, 10.5), (-1.0, 1.0)), &end, &[1]);
        assert!(
            matches!(got, Err(Unpresented::Structural)),
            "a shared seam at the end is structural"
        );
    }

    /// **A presented end holds the face's part of its cell, and a
    /// `Vacuous` cell none** (a counterexample search: the seed varies and
    /// is logged, the count rides `CAD_FUZZ_EFFORT`). Chains of three line
    /// edges `Q0 → P1 → P2 → Q3`, the face on their left, each corner
    /// convex or reflex, turned in the chart, `P1 P2` from `0.3Kε` to a
    /// unit long; cells about either corner, their local boundary read by
    /// [`Local::of`] off each cell's own ball. The truth is the nearest
    /// feature's side, skipping cells that reach an edge's free end or a
    /// convex apex. Every sampled face point of the cell must lie in a
    /// presentation's rectangle, and none in a `Vacuous` cell. The oracle
    /// is live: dropping the end's cap lean, capping at the cell's reach,
    /// reading `Vacuous` before the cap, or the cap's reach starting at
    /// the cell rather than at the corner each fail it.
    #[test]
    fn a_presented_end_holds_the_faces_part_at_random() {
        const CASES: usize = 4_000;
        let k = band().escalate();
        let mut rng = test_utils::fuzz::start("present_plane_end");
        let (mut of3, mut vac3, mut loc3, mut of2, mut vac, mut of1, mut bad) =
            (0, 0, 0, 0, 0, 0, 0);
        let dir = |t: f64| (t.cos(), t.sin());
        for case in 0..test_utils::fuzz::scaled(CASES) {
            let w = rng.log_uniform(0.3 * k, 1.0);
            let (l1, l3) = (rng.log_uniform(w, 20.0), rng.log_uniform(w, 20.0));
            let turn = |r: &mut test_utils::fuzz::Rng| {
                let t = 0.02 + r.unit() * 3.0;
                if r.unit() < 0.75 { t } else { -t }
            };
            let (t1, t2) = (turn(&mut rng), turn(&mut rng));
            let phi = if rng.unit() < 0.2 {
                0.0
            } else {
                rng.unit() * core::f64::consts::TAU
            };
            let p1 = (rng.unit() - 0.5, rng.unit() - 0.5);
            let d2 = dir(phi);
            let p2 = (p1.0 + w * d2.0, p1.1 + w * d2.1);
            let d1 = dir(phi - t1);
            let q0 = (p1.0 - l1 * d1.0, p1.1 - l1 * d1.1);
            let d3 = dir(phi + t2);
            let q3 = (p2.0 + l3 * d3.0, p2.1 + l3 * d3.1);
            let cross = |a: (f64, f64), b: (f64, f64)| a.0 * b.1 - a.1 * b.0;
            let sub = |a: (f64, f64), b: (f64, f64)| (a.0 - b.0, a.1 - b.1);
            // The ground truth runs the first edge back and the last on
            // as rays (a presented or Vacuous cell holds neither's end).
            // Where the rays cross, at X: the triangle X P1 P2 if both
            // corners are convex, else the case is skipped; a cell whose
            // ball reaches X is skipped below.
            let far = 1e3;
            let (r, s) = ((-d1.0, -d1.1), d3);
            let den = cross(r, s);
            let mut apex: Option<(f64, f64)> = None;
            if den.abs() > 1e-300 {
                let t = cross(sub(p2, p1), s) / den;
                let u = cross(sub(p2, p1), r) / den;
                if t > 0.0 && u > 0.0 {
                    if t1 < 0.0 || t2 < 0.0 {
                        continue;
                    }
                    apex = Some((p1.0 + t * r.0, p1.1 + t * r.1));
                }
            }
            let tq0 = apex.unwrap_or((p1.0 - far * d1.0, p1.1 - far * d1.1));
            let tq3 = apex.unwrap_or((p2.0 + far * d3.0, p2.1 + far * d3.1));
            let pts = [tq0, p1, p2, tq3];
            // The edges as the face holds them stop short of the apex.
            if let Some(x) = apex {
                let d = |a: (f64, f64)| ((a.0 - x.0).powi(2) + (a.1 - x.1).powi(2)).sqrt();
                if d(q0) < 1e-6 * l1 || d(q3) < 1e-6 * l3 || l1 > d(p1) || l3 > d(p2) {
                    continue;
                }
            }
            let (plane, mut b) = segments(&[
                (1, q0, p1, [1, 2]),
                (2, p1, p2, [2, 3]),
                (3, p2, q3, [3, 4]),
            ]);
            // The face side of a point: nearest feature's; None at a free end.
            // Each feature from a point at the end: its anchor, its unit
            // direction of travel, and its run from the anchor (signed).
            let norm = |a: (f64, f64)| (a.0 * a.0 + a.1 * a.1).sqrt();
            let feats = [
                (p1, d1, -norm(sub(tq0, p1)), 0.0),
                (p1, d2, 0.0, w),
                (p2, d3, 0.0, norm(sub(tq3, p2))),
            ];
            let in_face = |p: (f64, f64)| -> Option<(bool, f64)> {
                let mut best: Option<(f64, usize, f64)> = None;
                for (i, &(a, dv, lo, hi)) in feats.iter().enumerate() {
                    let r = sub(p, a);
                    let t = (r.0 * dv.0 + r.1 * dv.1).clamp(lo, hi);
                    let d = norm((r.0 - t * dv.0, r.1 - t * dv.1));
                    if best.is_none_or(|(bd, ..)| d < bd) {
                        best = Some((d, i, t));
                    }
                }
                let (d, i, t) = best?;
                let (a, dv, lo, hi) = feats[i];
                if (i == 0 && t == lo) || (i == 2 && t == hi) {
                    return if apex.is_some() {
                        Some((false, d))
                    } else {
                        None
                    };
                }
                Some(if (i == 0 && t == hi) || (i == 1 && t == lo) {
                    (t1 < 0.0, d)
                } else if (i == 1 && t == hi) || (i == 2 && t == lo) {
                    (t2 < 0.0, d)
                } else {
                    (cross(dv, sub(p, a)) > 0.0, d)
                })
            };
            for _ in 0..4 {
                let anchor = pts[1 + (rng.unit() * 2.0) as usize];
                let (hu, hv) = (
                    rng.log_uniform(0.05 * w, 300.0 * w),
                    rng.log_uniform(0.05 * w, 300.0 * w),
                );
                let c = (
                    anchor.0 + (rng.unit() - 0.5) * 3.0 * hu,
                    anchor.1 + (rng.unit() - 0.5) * 3.0 * hv,
                );
                let cell = ((c.0 - hu, c.0 + hu), (c.1 - hv, c.1 + hv));
                let (local, ball) = real_local(&mut b, cell);
                if let Some(x) = apex {
                    let reach = (hu * hu + hv * hv).sqrt();
                    if ((x.0 - c.0).powi(2) + (x.1 - c.1).powi(2)).sqrt() < 1.05 * reach {
                        continue;
                    }
                }
                // The truth is unknown where an edge's real end, or the run
                // past it, comes into the ball: skip those cells.
                {
                    let reach = (hu * hu + hv * hv).sqrt();
                    let seg = |a: (f64, f64), e: (f64, f64)| {
                        let v = sub(e, a);
                        let t = (((c.0 - a.0) * v.0 + (c.1 - a.1) * v.1) / (v.0 * v.0 + v.1 * v.1))
                            .clamp(0.0, 1.0);
                        ((c.0 - a.0 - t * v.0).powi(2) + (c.1 - a.1 - t * v.1).powi(2)).sqrt()
                    };
                    if seg(q0, tq0) < 1.05 * reach || seg(q3, tq3) < 1.05 * reach {
                        continue;
                    }
                }
                let shared: &[usize] = [&[][..], &[0], &[1], &[2]][(rng.unit() * 4.0) as usize];
                if !shared.iter().all(|e| local.edges.contains(e)) {
                    continue;
                }
                let piece = Piece {
                    face: FaceKey::default(),
                    surface: &plane,
                    whole: false,
                    boundary: &b,
                    u: cell.0,
                    v: cell.1,
                    local: &local,
                    ball,
                };
                let got = present_plane(&piece, shared, &[], band());
                let n = local.edges.len();
                let mut samples = Vec::new();
                for i in 0..12 {
                    for j in 0..12 {
                        samples.push((
                            cell.0.0 + (i as f64 + 0.5) / 12.0 * 2.0 * hu,
                            cell.1.0 + (j as f64 + 0.5) / 12.0 * 2.0 * hv,
                        ));
                    }
                }
                let clear = 1e-9 * (hu + hv) + 3e-15;
                let faced: Vec<(f64, f64)> = samples
                    .into_iter()
                    .filter(|&p| in_face(p).is_some_and(|(inside, d)| inside && d > clear))
                    .collect();
                let mut fail = None;
                match &got {
                    Ok(Presentation::Vacuous) => {
                        vac += 1;
                        if n == 3 {
                            vac3 += 1;
                        }
                        if let Some(p) = faced.first() {
                            fail = Some(format!("Vacuous, yet {p:?} is a face point of the cell"));
                        }
                    }
                    Ok(Presentation::Of(q)) => {
                        match n {
                            3 => of3 += 1,
                            2 => of2 += 1,
                            _ => of1 += 1,
                        }
                        if let Map::Affine { o, a, b: bv } = q.map {
                            let m = |x: Interval| 0.5 * (x.lo() + x.hi());
                            let (o, a, bv) =
                                ((m(o[0]), m(o[1])), (m(a[0]), m(a[1])), (m(bv[0]), m(bv[1])));
                            let det = cross(a, bv);
                            for &p in &faced {
                                let d = sub(p, o);
                                let (x, y) = (cross(d, bv) / det, cross(a, d) / det);
                                let (sx, sy) = (1e-7 * (1.0 + x.abs()), 1e-7 * (1.0 + y.abs()));
                                if x < q.x.0 - sx
                                    || x > q.x.1 + sx
                                    || y < q.y.0 - sy
                                    || y > q.y.1 + sy
                                {
                                    fail = Some(format!(
                                        "presented x {:?} y {:?}, yet face point {p:?} is at ({x}, {y})",
                                        q.x, q.y
                                    ));
                                    break;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        if n == 3 {
                            loc3 += 1;
                        }
                    }
                }
                if let Some(why) = fail {
                    bad += 1;
                    if bad <= 8 {
                        println!(
                            "[fuzz] case {case}: {why}\n     pts {pts:?} t1 {t1} t2 {t2} w {w:e} cell {cell:?} local {:?}/{} shared {shared:?}",
                            local.edges,
                            local.vertices.len()
                        );
                    }
                }
            }
        }
        println!(
            "[fuzz] band K eps {k:e}: three-edge cells presented {of3}, vacuous {vac3}, refused {loc3}; \
             two-edge presented {of2}; one-edge presented {of1}; vacuous in all {vac}; FAILURES {bad}"
        );
        assert_eq!(
            bad,
            0,
            "a presentation missed a face point, or Vacuous held one ({})",
            test_utils::fuzz::replay()
        );
    }

    /// **`edge_piece`**: the hull of an edge's parameters whose points the
    /// boxes may hold, narrowed to them from both ends.
    #[test]
    fn edge_piece_narrows_to_the_parameters_the_boxes_hold() {
        let line = Curve3::line(origin(), geom_core::UnitVec3::exact_x());
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
        // A hull reaching the span's end ends at it: over `(−0.4, 1.3)`,
        // `−0.4 + 1.7` rounds a step past `1.3`. Red if the last piece's
        // end is computed from the width.
        let (_, b) = edge_piece(&line, (iv0(-0.4), iv0(1.3)), false, &[bx(1.2, 2.0)]).unwrap();
        assert!(b == 1.3, "the hull ends at the span's own end: {b:e}");
    }

    /// The plane `z = 0`, `u_ref = x`.
    fn floor() -> Surface<Interval> {
        Surface::Plane {
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
        }
    }

    /// **A disk** of radius `r` on [`floor`], its one rim a closed circle
    /// from `u_ref = start` run forward (`plus`) or backward; `sense`
    /// the face's.
    fn disk(r: f64, start: Vec3<Interval>, plus: bool, sense: bool) -> Boundary {
        Boundary {
            sense,
            edges: vec![Rim {
                stated: None,
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
                    curve: None,
                }],
            }],
            vertices: BTreeMap::from([(vertex(1), origin())]),
            extent: None,
            outer: None,
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
    /// certifies.** [`certify_pair_in`] on the cell `(0.69, 0.71) × (0.1,
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
            stated: None,
            key: edge(n),
            // The segment is its chord, its span in metres: the chord's
            // unit direction and its length, both enclosures
            // ([`geom_core::UnitVec3::chord`]), as a built body's line is.
            // Its far end is then an enclosure that holds `to`, a
            // rounding wide, and not a point equal to it. A point box of
            // the rounded quotient is no unit vector at an enclosure
            // scalar: at 45 degrees its squares sum past 1 and the
            // direction is poison.
            arc: Some({
                let (dir, len) = geom_core::UnitVec3::chord(p(to) - p(from));
                (Curve3::line(p(from), dir), (iv0(0.0), len))
            }),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                curve: None,
            }],
        }
    }

    /// [`segment`] from `from` over `len`, its direction poison: what
    /// [`geom_core::UnitVec3::checked`] answers for a box that holds no
    /// unit vector.
    fn poisoned_segment(n: u64, ends: [u64; 2], from: (f64, f64), len: f64) -> Rim {
        let dir = geom_core::UnitVec3::checked(Vec3::new(iv0(0.6), iv0(0.6), iv0(0.0)));
        assert!(
            dir.get().x.lo().is_nan(),
            "the fixture's direction is poison"
        );
        let mut rim = segment(n, ends, from, (from.0 + len, from.1));
        rim.arc = Some((
            Curve3::line(Point3::new(iv0(from.0), iv0(from.1), iv0(0.0)), dir),
            (iv0(0.0), iv0(len)),
        ));
        rim
    }

    /// **Every reader of a line edge refuses one whose direction is
    /// poison.** A `UnitVec3` that its door refused is NaN in every
    /// component and is announced nowhere before a read
    /// (`work/embed/a-poisoned-unit-direction-is-announced-nowhere-between-its-door-and-a-read.md`),
    /// so each reader of the edge's ends or direction has to refuse it
    /// by its own decision. Injected at each: a plane's one line edge
    /// ([`present_plane`]); its corner of two lines; a line shared at a
    /// corner with an arc ([`present_plane_corner`]); the other edge of
    /// a curve's corner ([`leaving`], the run); [`line_onward`]'s own
    /// comparisons; and the edge a graph separator would run on along
    /// ([`graph_run`]). None presents, none is vacuous, none gives a
    /// graph; each fixture with a sound direction in its place is read,
    /// so the refusal is the poison's. Red if a reader's comparison
    /// passes on NaN.
    #[test]
    fn a_poisoned_line_direction_is_refused_by_every_reader() {
        let s = floor();
        let refused = |r: &Read| matches!(r, Read::Local | Read::Structural);
        let ask =
            |b: &Boundary, edges: Vec<usize>, vertices: Vec<VertexKey>, near: &[usize], u, v| {
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
                read(present_plane(&piece, near, &[], band()))
            };
        // One line edge, the cell across it.
        let one = Boundary {
            sense: true,
            edges: vec![poisoned_segment(1, [1, 2], (0.0, 0.0), 4.0)],
            vertices: BTreeMap::new(),
            extent: None,
            outer: None,
        };
        let r = ask(&one, vec![0], Vec::new(), &[0], (1.0, 2.0), (-0.5, 0.5));
        assert!(refused(&r), "a plane's one line edge: {r:?}");
        let mut sound = one.clone();
        sound.edges[0] = segment(1, [1, 2], (0.0, 0.0), (4.0, 0.0));
        let r = ask(&sound, vec![0], Vec::new(), &[0], (1.0, 2.0), (-0.5, 0.5));
        assert!(
            matches!(r, Read::Of(..)),
            "with a direction it is read: {r:?}"
        );
        // A corner of two lines at the origin, one of them poison.
        let two = Boundary {
            sense: true,
            edges: vec![
                segment(2, [2, 1], (0.0, 4.0), (0.0, 0.0)),
                poisoned_segment(3, [1, 3], (0.0, 0.0), 4.0),
            ],
            vertices: BTreeMap::new(),
            extent: None,
            outer: None,
        };
        for near in [&[0][..], &[1][..]] {
            let r = ask(
                &two,
                vec![0, 1],
                vec![vertex(1)],
                near,
                (-0.5, 0.5),
                (-0.5, 0.5),
            );
            assert!(refused(&r), "a corner of two lines, {near:?} shared: {r:?}");
        }
        // The tangent joint, its line poison: read from the arc (the
        // run, `leaving`) and from the line (`present_plane_corner`).
        let good = quarter(segment(20, [3, 1], (1.0, -1.0), (1.0, 0.0)), Vec::new());
        let bad = quarter(poisoned_segment(20, [3, 1], (1.0, -1.0), 1.0), Vec::new());
        let mut sound = two.clone();
        sound.edges[1] = segment(3, [1, 3], (0.0, 0.0), (4.0, 0.0));
        for near in [&[0][..], &[1][..]] {
            let r = ask(
                &sound,
                vec![0, 1],
                vec![vertex(1)],
                near,
                (-0.5, 0.5),
                (-0.5, 0.5),
            );
            assert!(
                matches!(r, Read::Of(..)),
                "two sound lines, {near:?} shared: {r:?}"
            );
            let r = ask(
                &good,
                vec![0, 1],
                vec![vertex(1)],
                near,
                (0.98, 1.02),
                (-0.02, 0.02),
            );
            assert!(
                matches!(r, Read::Of(..)),
                "the sound joint, {near:?} shared: {r:?}"
            );
        }
        for near in [&[0][..], &[1][..]] {
            let r = ask(
                &bad,
                vec![0, 1],
                vec![vertex(1)],
                near,
                (0.98, 1.02),
                (-0.02, 0.02),
            );
            assert!(refused(&r), "the tangent joint, {near:?} shared: {r:?}");
        }
        // `line_onward` on its own, every field of the edge no number.
        let nan = at(f64::NAN);
        let l = Leaving {
            d: [nan; 2],
            near: [nan; 2],
            far: [nan; 2],
            speed: [1.0, 1.0],
            leaves: Leaves::Undecided,
        };
        for onward in [true, false] {
            assert!(
                line_onward(&l, Axis::U, onward, (0.0, 1.0), band()).is_err(),
                "line_onward, onward {onward}"
            );
        }
        // The graph does not run on along a poisoned edge: the run is
        // presented on the good body, then the separator is asked on
        // the body whose line is poison.
        let local = both_arcs();
        let x = piece_on(&s, &good, &local, 1);
        let Ok(Presentation::Of(p)) = present_plane(&x, &[0], &[], band()) else {
            panic!("the run is presented");
        };
        let cyl = cylinder();
        let wall = Presented {
            surface: &cyl,
            map: Map::Chart,
            x: (0.0, 0.02),
            y: (0.0, 0.02),
            zero: Zero::Side(
                Side::YLo,
                Along {
                    edges: vec![edge(10)],
                    vertex: None,
                    whole: true,
                    params: Some(Params {
                        t: (0.0, 0.02),
                        periodic: true,
                    }),
                    on_edges: true,
                    stops: vec![vertex(1)],
                },
            ),
            corner: false,
            cache: Cache::default(),
        };
        let near = Near {
            edges: vec![(0, 0)],
            vertices: Vec::new(),
        };
        assert!(
            graph_of(&x, &x, &near, &p, &wall, band()).is_some(),
            "the fixture reaches the graph on the good body"
        );
        let y = piece_on(&s, &bad, &local, 1);
        assert!(
            graph_of(&y, &y, &near, &p, &wall, band()).is_none(),
            "a graph run on along a poisoned edge"
        );
    }

    /// **A fixture's line is its unit chord as an enclosure, at any
    /// angle.** [`segment`] from `(1.5, 0.5)` to `(1, 0)`, at 45 degrees,
    /// where the rounded quotient's squares sum past 1: its direction is
    /// finite, and its far end is an enclosure that holds `to` without
    /// being the point `to`, as a built body's line ends on its vertex.
    /// Red if the helper states the direction as a point box (poison at
    /// this angle, and every read of the edge then answers nothing), or
    /// states the line so that it ends on its vertex exactly.
    #[test]
    fn a_fixture_segment_ends_on_its_vertex_by_enclosure_at_any_angle() {
        let rim = segment(20, [3, 1], (1.5, 0.5), (1.0, 0.0));
        let (carrier, (_, t1)) = rim.arc.expect("a line");
        let end = carrier.eval(t1);
        for (what, x, to) in [("x", end.x, 1.0), ("y", end.y, 0.0)] {
            assert!(
                x.lo().is_finite() && x.hi().is_finite(),
                "the far end's {what} is a number: {x:?}"
            );
            assert!(x.lo() <= to && to <= x.hi(), "it holds the vertex: {x:?}");
        }
        assert!(
            end.x.lo() < end.x.hi() || end.y.lo() < end.y.hi(),
            "and is not the vertex to the bit: {end:?}"
        );
    }

    /// **A corner of the unit circle's arc on [`floor`]**: the arc from
    /// `(1, 0)` (vertex 1) counter-clockwise to `(0, 1)` (vertex 2), its
    /// face inside the circle, and `other` the edge the loop reaches
    /// `(1, 0)` along, plus `extra` edges. The arc is edge 0, `other`
    /// edge 1.
    fn quarter(other: Rim, extra: Vec<Rim>) -> Boundary {
        let z = Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0));
        let arc = Rim {
            stated: None,
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
                curve: None,
            }],
        };
        let p = |x: f64, y: f64| Point3::new(iv0(x), iv0(y), iv0(0.0));
        let mut vertices = BTreeMap::from([(vertex(1), p(1.0, 0.0)), (vertex(2), p(0.0, 1.0))]);
        for e in core::iter::once(&other).chain(&extra) {
            if let Some((Curve3::Line(geom::Line { origin, dir }), _)) = &e.arc {
                vertices.entry(e.ends[0]).or_insert(*origin);
                vertices.entry(e.ends[1]).or_insert(*origin + dir.get());
            }
        }
        Boundary {
            sense: true,
            edges: [arc, other].into_iter().chain(extra).collect(),
            vertices,
            extent: None,
            outer: None,
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
    /// hold. Red if [`leaving`] admits a decided `Negative`.
    #[test]
    fn a_reflex_corner_of_a_curved_shared_edge_is_structural() {
        let b = quarter(segment(20, [3, 1], (2.0, 0.0), (1.0, 0.0)), Vec::new());
        assert_eq!(
            on_floor(&b, vec![0, 1], vec![vertex(1)], (0.98, 1.02), (-0.02, 0.02)),
            Read::Structural,
            "the reflex corner at (1, 0)"
        );
    }

    /// The run a cell of `b` on [`floor`] about `(1, 0)` is presented by,
    /// the edge `shared` on the shared cell: the side the face is on,
    /// the curve's parameters, the edges its part on the shared cell
    /// lies along, the vertex it runs on through, and the edge it runs
    /// on along. `None` where the cell is not presented by a run.
    #[allow(clippy::type_complexity)]
    fn run_about(
        b: &Boundary,
        shared: usize,
        u: (f64, f64),
        v: (f64, f64),
    ) -> Option<(Side, (f64, f64), Vec<EdgeKey>, Option<VertexKey>, RunOn)> {
        let s = floor();
        let local = both_arcs();
        let piece = Piece {
            u,
            v,
            ..piece_on(&s, b, &local, 1)
        };
        let vertices = if shared == 0 { vec![] } else { vec![vertex(1)] };
        run_of(present_plane(&piece, &[shared], &vertices, band()))
    }

    /// [`run_about`]'s reading of a presentation.
    #[allow(clippy::type_complexity)]
    fn run_of(
        p: Result<Presentation<'_>, Unpresented>,
    ) -> Option<(Side, (f64, f64), Vec<EdgeKey>, Option<VertexKey>, RunOn)> {
        match p {
            Ok(Presentation::Of(q)) => match &q.zero {
                Zero::Curve(side, c) => c
                    .run
                    .map(|run| (*side, c.t, c.along.edges.clone(), c.past, run)),
                _ => None,
            },
            _ => None,
        }
    }

    /// **A line tangent to the arc at its end is read with it as one
    /// run** ([`present_corner`]): it leaves `(1, 0)` along the circle's
    /// tangent, against the arc's run, so it stands outside the circle,
    /// by second order, and the face beside it is inside the line — not
    /// the circle. The cell is presented by the arc up to the vertex and
    /// the line from it, from either edge: the curve's parameters stop
    /// at the vertex's, the part on the shared cell is the shared edge's,
    /// and the side runs on along the other. Red if the corner is read
    /// through by the circle continued (no run), or the curve's
    /// parameters run past the vertex.
    #[test]
    fn a_line_tangent_to_the_arc_at_the_corner_is_read_with_it_as_one_run() {
        let b = quarter(segment(20, [3, 1], (1.0, -1.0), (1.0, 0.0)), Vec::new());
        let cell = ((0.98, 1.02), (-0.02, 0.02));
        let (side, t, along, past, run) =
            run_about(&b, 0, cell.0, cell.1).expect("a run, the arc shared");
        assert_eq!(side, Side::XHi, "the face is at lower u than the run");
        assert!(t.0 == 0.0 && t.1 > 0.0, "the arc from the vertex on: {t:?}");
        assert_eq!(along, [edge(10)], "its shared part is the arc's");
        assert_eq!(past, Some(vertex(1)), "it runs on through the vertex");
        assert_eq!(
            run,
            RunOn {
                edge: edge(20),
                start: true
            },
            "along the line, from the arc's start"
        );
        let (side, t, along, past, run) =
            run_about(&b, 1, cell.0, cell.1).expect("a run, the line shared");
        assert_eq!(side, Side::XHi, "the same side, the line shared");
        assert!(t.0 == 0.0 && t.1 > 0.0, "the arc from the vertex on: {t:?}");
        assert_eq!(along, [edge(20)], "its shared part is the line's");
        assert_eq!(past, Some(vertex(1)), "it runs on through the vertex");
        assert_eq!(
            run,
            RunOn {
                edge: edge(10),
                start: false
            },
            "along the arc, from the line's end"
        );
    }

    /// **A reflex corner whose line runs on across the columns is one
    /// run**: the line reaches `(1, 0)` from `(1.5, −1)`, outside the
    /// circle and below the vertex, so over `v` the arc and the line are
    /// one graph. Where the line reaches it from `(2, 0)`, along `u`, it
    /// is no graph with the arc over either axis, and the corner stays
    /// `Structural` ([`a_reflex_corner_of_a_curved_shared_edge_is_structural`]).
    /// Red if a reflex corner is read through by the circle continued.
    #[test]
    fn a_reflex_corner_whose_line_runs_on_across_the_columns_is_one_run() {
        let b = quarter(segment(20, [3, 1], (1.5, -1.0), (1.0, 0.0)), Vec::new());
        let (side, t, along, past, run) =
            run_about(&b, 0, (0.98, 1.02), (-0.02, 0.02)).expect("a run");
        assert_eq!(side, Side::XHi, "the face is at lower u than the run");
        assert!(t.0 == 0.0 && t.1 > 0.0, "the arc from the vertex on: {t:?}");
        assert_eq!(
            (along.as_slice(), past, run.edge),
            (&[edge(10)][..], Some(vertex(1)), edge(20)),
            "the arc, on through the vertex along the line"
        );
        // A line that leaves the cell's own range before the columns end:
        // from `(2, −0.2)`, it is at `u = 1.1` where `v = −0.02`. `R` is
        // widened to hold it there, so every column of `R` holds a point
        // of the run. Red if `R` is the cell and the curve's reach alone.
        let steep = quarter(segment(20, [3, 1], (2.0, -0.2), (1.0, 0.0)), Vec::new());
        let Read::Of(x, _) = on_floor(
            &steep,
            vec![0, 1],
            vec![vertex(1)],
            (0.98, 1.02),
            (-0.02, 0.02),
        ) else {
            panic!("the steep reflex corner is one run");
        };
        assert!(x.1 >= 1.1, "R holds the line over the columns: {x:?}");
    }

    /// **A run's rectangle holds the line over the columns it crosses,
    /// and nothing else**: the tangent line of the row above, with a
    /// third edge from `(0.99, −0.015)`, between the line and the
    /// circle's continuation, in reach. Red if the run admits an edge
    /// that is neither of the corner's two.
    #[test]
    fn a_run_with_a_third_edge_in_reach_is_not_read() {
        let third = segment(30, [5, 6], (0.99, -0.015), (0.99, -0.5));
        let b = quarter(segment(20, [3, 1], (1.0, -1.0), (1.0, 0.0)), vec![third]);
        assert_eq!(
            on_floor(
                &b,
                vec![0, 1, 2],
                vec![vertex(1)],
                (0.98, 1.02),
                (-0.02, 0.02)
            ),
            Read::Local,
            "a third edge in reach"
        );
        // The line's own far end in the columns: its vertex is in reach.
        let short = quarter(segment(20, [3, 1], (1.0, -0.01), (1.0, 0.0)), Vec::new());
        assert_eq!(
            on_floor(
                &short,
                vec![0, 1],
                vec![vertex(1), vertex(3)],
                (0.98, 1.02),
                (-0.02, 0.02)
            ),
            Read::Local,
            "the line ends inside the columns"
        );
    }

    /// **The graph separator runs on along a run's other edge**
    /// ([`graph_of`], [`graph_run`]): the tangent joint of the rows
    /// above against a wall standing on the arc, the cylinder over the
    /// unit circle from the vertex on. The graph lists the line as its
    /// run, its slopes hold the line's, and the two faces read opposite
    /// signs under it. Red if a run is read zero under a graph that does
    /// not run on along its edge, or under a plane; red if the graph's
    /// own edge is taken past the vertex along its circle, where the
    /// face's boundary is the line.
    #[test]
    fn the_graph_runs_on_along_a_runs_other_edge() {
        let s = floor();
        let local = both_arcs();
        let b = quarter(segment(20, [3, 1], (1.0, -1.0), (1.0, 0.0)), Vec::new());
        let x = piece_on(&s, &b, &local, 1);
        let presented = present_plane(&x, &[0], &[], band());
        // A run is not refused with an unreadable partner: a smaller cell
        // may read the curve alone ([`stuck`]).
        assert!(!stuck(&x, &presented), "a run is not stuck");
        let Ok(Presentation::Of(p)) = presented else {
            panic!("the floor is presented about (1, 0)");
        };
        let cyl = cylinder();
        let wall = |u: (f64, f64), stops: Vec<VertexKey>| Presented {
            surface: &cyl,
            map: Map::Chart,
            x: u,
            y: (0.0, 0.02),
            zero: Zero::Side(
                Side::YLo,
                Along {
                    edges: vec![edge(10)],
                    vertex: None,
                    whole: true,
                    params: Some(Params {
                        t: u,
                        periodic: true,
                    }),
                    on_edges: true,
                    stops,
                },
            ),
            corner: false,
            cache: Cache::default(),
        };
        let near = Near {
            edges: vec![(0, 0)],
            vertices: Vec::new(),
        };
        let on_the_arc = wall((0.0, 0.02), Vec::new());
        let (psi, on) = graph_of(&x, &x, &near, &p, &on_the_arc, band()).expect("a graph");
        assert_eq!(
            (on.edges.as_slice(), on.run),
            (
                &[edge(10)][..],
                Some(RunEnd {
                    edge: edge(20),
                    vertex: vertex(1),
                    at: 0.0,
                    start: true
                })
            ),
            "the arc, run on along the line from its start"
        );
        let Psi::Graph { slope, piece, .. } = psi else {
            panic!("the graph");
        };
        assert!(piece.0 == 0.0, "the piece stops at the vertex: {piece:?}");
        assert!(
            slope.lo() <= 0.0 && 0.0 <= slope.hi(),
            "the slopes hold the tangent line's: {slope:?}"
        );
        let (floor_sign, wall_sign) = (
            sign_on(&psi, Some(&on), &p, Some(&on_the_arc), arm(1.0), band()),
            sign_on(&psi, Some(&on), &on_the_arc, Some(&p), arm(1.0), band()),
        );
        assert!(
            matches!((floor_sign, wall_sign), (Some(a), Some(b)) if a == flip(b)),
            "the floor and the wall part: {floor_sign:?} {wall_sign:?}"
        );
        // A graph that does not run on along the line reads the run
        // nowhere, and a plane does not either.
        let no_run = On {
            run: None,
            ..on.clone()
        };
        assert_eq!(
            sign_on(&psi, Some(&no_run), &p, Some(&on_the_arc), arm(1.0), band()),
            None,
            "a graph that stops at the vertex"
        );
        let own_plane = Psi::Linear {
            n: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            q: origin(),
        };
        let whole = On {
            whole: true,
            ..on.clone()
        };
        assert_eq!(
            sign_on(
                &own_plane,
                Some(&whole),
                &p,
                Some(&on_the_arc),
                arm(1.0),
                band()
            ),
            None,
            "a plane holding the arc's circle"
        );
        // A wall whose side runs past the vertex along the circle: the
        // graph would follow the circle there, off the line, by however
        // little. The rounding a clip's enclosure leaves past the vertex
        // (`4.4e-16`, as a fillet's patch reads) refuses too where the
        // side does not end at that vertex.
        for past in [-0.01, -4.4e-16] {
            assert!(
                graph_of(&x, &x, &near, &p, &wall((past, 0.02), Vec::new()), band()).is_none(),
                "a side {past:e} past a vertex it does not end at"
            );
        }
        // A side clipped by a line whose edge ends at the vertex ends
        // there by key: read to the vertex, it leaves the piece at it and
        // is zero under the graph. One that ends at the arc's other end
        // by key is not read short at this one.
        let ends_here = wall((-4.4e-16, 0.02), vec![vertex(1)]);
        let (psi, on) = graph_of(&x, &x, &near, &p, &ends_here, band()).expect("a graph");
        let Psi::Graph { piece, .. } = psi else {
            panic!("the graph");
        };
        assert!(piece.0 == 0.0, "the piece stops at the vertex: {piece:?}");
        assert!(
            sign_on(&psi, Some(&on), &ends_here, Some(&p), arm(1.0), band()).is_some(),
            "the side that ends at the vertex is zero under the graph"
        );
        assert!(
            graph_of(
                &x,
                &x,
                &near,
                &p,
                &wall((-4.4e-16, 0.02), vec![vertex(2)]),
                band()
            )
            .is_none(),
            "a side that ends at the arc's other vertex"
        );
        // Never in the carrier arm, nor against a partner read past the
        // shared cell.
        assert_eq!(
            sign_on(&psi, Some(&on), &p, None, arm(1.0), band()),
            None,
            "no partner"
        );
        // A reflex joint's line leaves the arc at an angle: the graph's
        // slopes hold the line's, far from the arc's.
        let reflex = quarter(segment(20, [3, 1], (1.5, -1.0), (1.0, 0.0)), Vec::new());
        let y = piece_on(&s, &reflex, &local, 1);
        let Ok(Presentation::Of(q)) = present_plane(&y, &[0], &[], band()) else {
            panic!("the reflex floor is presented about (1, 0)");
        };
        let (psi, _) = graph_of(&y, &y, &near, &q, &on_the_arc, band()).expect("a graph");
        let Psi::Graph { slope: kinked, .. } = psi else {
            panic!("the graph");
        };
        assert!(
            kinked.hi() - kinked.lo() > 0.2 && slope.hi() - slope.lo() < 0.05,
            "the reflex run's slopes hold its line's: {kinked:?} against {slope:?}"
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
    /// leaves the vertex the other way, along `u`, the corner is reflex
    /// and no graph over either axis, which no presentation reads; where
    /// it runs along the arc's tangent, the arc's departure is second
    /// order, and the two are one run, not the line's face side.
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
        assert!(
            run_about(&tangent, 1, (0.98, 1.02), (-0.02, 0.02)).is_some(),
            "the tangent join at (1, 0) is one run: {:?}",
            on_floor_by_line(&tangent, (0.98, 1.02), (-0.02, 0.02))
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
            stated: None,
            key: edge(key),
            arc: Some((carrier, span)),
            ends: [vertex(1), vertex(2)],
            uses: vec![Use {
                plus,
                image: None,
                iso: None,
                curve: None,
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
            outer: None,
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
    /// `(0, 1)`, makes a corner of 135° there, which no reading reads
    /// (a run is read only beside an edge straight in the chart): red if
    /// [`one_conic`] admits two circles that are not one.
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
        let (x, y) = (
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
            run: None,
        };
        assert_eq!(
            sign_on(&graph, Some(&on), &p, Some(&exact), arm(1.0), band()),
            Some(Sign::Negative),
            "admitted"
        );
        let cylinder = Psi::Carrier(Implicit::Cylinder {
            o: origin(),
            axis: geom_core::UnitVec3::exact_z(),
            r: iv0(1.0),
        });
        assert_eq!(
            sign_on(&cylinder, None, &p, None, arm(1.0), band()),
            None,
            "the carrier arm"
        );
        assert_eq!(
            sign_on(&graph, Some(&on), &p, None, arm(1.0), band()),
            None,
            "against no partner"
        );
        let twin = halves(false, 12, None);
        let Ok(Presentation::Of(q)) = on_halves(&s, &twin, &local) else {
            panic!("the outside is presented about (1, 0)");
        };
        assert_eq!(
            sign_on(&graph, Some(&on), &p, Some(&q), arm(1.0), band()),
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
            frame: geom::Frame::new(Point3::new(iv0(1.0), iv0(0.0), iv0(0.0)), normal, u_ref),
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
                    sign_on(&psi, Some(&on), &p, Some(q), arm(1.0), band()),
                    sign_on(&psi, Some(&on), q, Some(&p), arm(1.0), band()),
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
            frame: geom::Frame::new(Point3::new(iv0(1.0), iv0(0.0), iv0(0.0)), normal, u_ref),
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
                    sign_on(&psi, Some(&on), &p, Some(q), arm(1.0), band()),
                    sign_on(&psi, Some(&on), q, Some(&p), arm(1.0), band()),
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
            stated: None,
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
            frame: geom::Frame::new(origin(), z, x),
            radius: iv0(1.0),
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
            outer: None,
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

    /// The octant, its two meridians ending at `top` at the pole: the
    /// lift's end there and the extent's `v` top.
    fn octant_ending_at(top: f64) -> (Surface<Interval>, Boundary) {
        let (s, mut b) = octant();
        let q = core::f64::consts::FRAC_PI_2;
        b.edges[1] = rim(2, [2, 3], Axis::U, q, (q, 0.0), (q, top));
        b.edges[2] = rim(3, [3, 1], Axis::U, 0.0, (0.0, top), (0.0, 0.0));
        b.extent = Some([(0.0, q), (0.0, top)]);
        (s, b)
    }

    /// **A sphere face whose stored extent stops ulps short of its pole
    /// places the pole, in every cell that reaches it.** The octant's
    /// meridians end one ulp under `π/2`'s enclosure, as a meridian's
    /// pcurve evaluated at its edge's parameter does, and one over it.
    /// The face holds a vertex at the pole, so its extent reaches the
    /// polar latitude ([`face_extent`]), and the window's top cell and
    /// each cell under it down to a width of a few ulps is presented
    /// with the pole's collapsed side its zero. Red, every cell
    /// unpresented, where the rectangle is clipped to the stored extent.
    #[test]
    fn a_sphere_face_whose_extent_stops_ulps_short_of_its_pole_places_the_pole() {
        let q = core::f64::consts::FRAC_PI_2;
        let line = pole_latitude(true);
        for (stored, what) in [(q.next_down(), "short"), (q, "on"), (q.next_up(), "past")] {
            let (s, b) = octant_ending_at(stored);
            assert!(
                matches!(
                    face_extent(&s, &b, band()),
                    Ok(Extent::Known([_, (0.0, hi)])) if hi == line.hi()
                ),
                "{what}: the extent ends on the polar latitude"
            );
            let local = Local {
                edges: vec![1, 2],
                vertices: vec![vertex(3)],
            };
            let top = q.next_up();
            let widths =
                core::iter::successors(Some(0.07), |w| Some(w * 0.5)).take_while(|w| *w > 1e-14);
            for w in widths {
                let piece = Piece {
                    face: FaceKey::default(),
                    surface: &s,
                    whole: false,
                    boundary: &b,
                    u: (0.7, 0.8),
                    v: (top - w, top),
                    local: &local,
                    ball: Ball::holding(origin(), origin()),
                };
                let got = match present_chart(&piece, &[], &[vertex(3)], band()) {
                    Ok(Presentation::Of(p)) => Some((p.zero, p.y)),
                    _ => None,
                };
                assert!(
                    matches!(&got, Some((Zero::Side(Side::YHi, a), (_, hi)))
                        if *hi == line.hi() && a.vertex == Some(vertex(3))),
                    "{what}, a cell {w} under the pole: the pole's side is the zero: {got:?}"
                );
            }
        }
    }

    /// **The south pole is the low side of its face's chart**: the
    /// octant under the equator, its meridians ending an ulp over
    /// `-π/2`. The cell at the pole, wholly at negative `v`, is presented
    /// with the side `v = -π/2` its zero. Red where a sphere's cell is
    /// read as a cone's on its other nappe, holding nothing.
    #[test]
    fn a_spheres_south_pole_is_the_low_side_of_its_chart() {
        let (s, mut b) = octant();
        let q = core::f64::consts::FRAC_PI_2;
        let line = pole_latitude(false);
        let end = -q.next_down();
        b.edges = vec![
            rim(1, [2, 1], Axis::V, 0.0, (q, 0.0), (0.0, 0.0)),
            rim(2, [3, 2], Axis::U, q, (q, end), (q, 0.0)),
            rim(3, [1, 3], Axis::U, 0.0, (0.0, 0.0), (0.0, end)),
        ];
        b.vertices
            .insert(vertex(3), Point3::new(iv0(0.0), iv0(0.0), iv0(-1.0)));
        b.extent = Some([(0.0, q), (end, 0.0)]);
        assert_eq!(v_of(&s, &b), Ok((line.lo(), 0.0)));
        let local = Local {
            edges: vec![1, 2],
            vertices: vec![vertex(3)],
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &b,
            u: (0.7, 0.8),
            v: (-q.next_up(), -1.5),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let got = match present_chart(&piece, &[], &[vertex(3)], band()) {
            Ok(Presentation::Of(p)) => Some((p.zero, p.y)),
            _ => None,
        };
        assert!(
            matches!(&got, Some((Zero::Side(Side::YLo, a), (lo, _)))
                if *lo == line.lo() && a.vertex == Some(vertex(3))),
            "the south pole's side is the zero: {got:?}"
        );
    }

    /// **A meridian that ends at the pole's vertex runs on to the pole's
    /// side**: the octant with its meridians' images ending half the
    /// band's zero under the pole, as a vertex in the band and off the
    /// pole puts them. The cell at the pole is presented with the pole's
    /// side its zero. With the same images and the meridians ending at
    /// other vertices, beside the pole's, the cell is `Local`: the gap is
    /// past the cover's `few_ulps`, and no key closes it. Red where the
    /// run-on is granted without the vertex's key, or not at all.
    #[test]
    fn a_meridian_ending_at_the_poles_vertex_runs_on_to_the_poles_side() {
        let q = core::f64::consts::FRAC_PI_2;
        let line = pole_latitude(true);
        let (s, b) = octant_ending_at(line.lo() - 0.5 * band().zero());
        let at_pole = |b: &Boundary| {
            let local = Local {
                edges: vec![1, 2],
                vertices: vec![vertex(3)],
            };
            let piece = Piece {
                face: FaceKey::default(),
                surface: &s,
                whole: false,
                boundary: b,
                u: (0.0, q),
                v: (1.5, q.next_up()),
                local: &local,
                ball: Ball::holding(origin(), origin()),
            };
            match present_chart(&piece, &[], &[vertex(3)], band()) {
                Ok(Presentation::Of(p)) => Ok((p.zero, p.y)),
                Ok(Presentation::Vacuous) => panic!("vacuous"),
                Err(e) => Err(e),
            }
        };
        let got = at_pole(&b);
        assert!(
            matches!(&got, Ok((Zero::Side(Side::YHi, a), (_, hi)))
                if *hi == line.hi() && a.vertex == Some(vertex(3))),
            "the meridians end at the pole's vertex: {got:?}"
        );
        let mut beside = b.clone();
        beside.edges[1].ends[1] = vertex(7);
        beside.edges[2].ends[0] = vertex(8);
        assert!(
            matches!(at_pole(&beside), Err(Unpresented::Local)),
            "the meridians end beside the pole, at other vertices"
        );
    }

    /// **A meridian whose image ends off the pole does not run on to the
    /// pole's side, though the face's extent moved there** (review of
    /// PR 7115): the octant with one meridian's image ending an ulp
    /// under `π/2` and the other's 0.01 under, both ending at the pole's
    /// vertex. The stored extent ends an ulp under, so it moves to the
    /// latitude; the short meridian still covers only its own range, so
    /// the cell across both meridians at the pole is `Local`. Red where
    /// the run-on is granted by the key, or by the moved extent, alone.
    #[test]
    fn a_meridian_ending_off_the_pole_does_not_run_on_where_the_extent_moved() {
        let q = core::f64::consts::FRAC_PI_2;
        let line = pole_latitude(true);
        let (s, mut b) = octant_ending_at(q.next_down());
        b.edges[1] = rim(2, [2, 3], Axis::U, q, (q, 0.0), (q, q - 0.01));
        assert_eq!(v_of(&s, &b), Ok((0.0, line.hi())), "the extent moves");
        let local = Local {
            edges: vec![1, 2],
            vertices: vec![vertex(3)],
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &b,
            u: (0.0, q),
            v: (1.5, q.next_up()),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let got = present_chart(&piece, &[], &[vertex(3)], band());
        assert!(
            matches!(got, Err(Unpresented::Local)),
            "the short meridian does not cover its side: {:?}",
            got.map(|p| matches!(p, Presentation::Vacuous))
        );
    }

    /// **An edge runs on to the pole's side only where its image ends at
    /// the pole by the band**: a polar cap whose loop wraps the sphere
    /// (its extent says nothing, so nothing has checked its rows), with a
    /// meridian spur up to the pole's vertex. The cell beside the spur at
    /// the pole is presented where the spur's image ends half the band's
    /// zero short of the latitude, and is `Local` where it ends twice
    /// the band's escalation short: that edge and its vertex name two
    /// places. Red where the vertex's key alone grants the run-on.
    #[test]
    fn an_edge_runs_on_only_where_its_image_ends_at_the_pole_by_the_band() {
        let (s, _) = octant();
        let top = pole_latitude(true).lo();
        let cap = |end: f64| Boundary {
            sense: true,
            outer: None,
            edges: vec![
                rim(1, [1, 1], Axis::V, 1.0, (0.0, 1.0), (TURN, 1.0)),
                rim(2, [1, 3], Axis::U, 1.5, (1.5, 1.0), (1.5, end)),
            ],
            vertices: BTreeMap::from([
                (
                    vertex(1),
                    Point3::new(iv0(1.0_f64.cos()), iv0(0.0), iv0(1.0_f64.sin())),
                ),
                (vertex(3), Point3::new(iv0(0.0), iv0(0.0), iv0(1.0))),
            ]),
            extent: Some([(0.0, TURN), (1.0, core::f64::consts::FRAC_PI_2)]),
        };
        let at_pole = |b: &Boundary| {
            let local = Local {
                edges: vec![1],
                vertices: vec![vertex(3)],
            };
            let piece = Piece {
                face: FaceKey::default(),
                surface: &s,
                whole: false,
                boundary: b,
                u: (1.2, 1.5),
                v: (1.5, core::f64::consts::FRAC_PI_2.next_up()),
                local: &local,
                ball: Ball::holding(origin(), origin()),
            };
            read(present_chart(&piece, &[], &[vertex(3)], band()))
        };
        assert!(
            matches!(at_pole(&cap(top - 0.5 * band().zero())), Read::Of(..)),
            "in the band: {:?}",
            at_pole(&cap(top - 0.5 * band().zero()))
        );
        assert_eq!(
            at_pole(&cap(top - 2.0 * band().escalate())),
            Read::Local,
            "past the band"
        );
    }

    /// **A rectangle wholly past a singular latitude is not presented**:
    /// a polar cap whose loop wraps the sphere, so its extent says
    /// nothing, read in a cell past `π/2` with the pole's vertex in
    /// reach. The pole's side leaves the rectangle no width, which is
    /// `Local`. Red where the collapse is skipped for such a rectangle
    /// and it keeps its width.
    #[test]
    fn a_rectangle_wholly_past_the_pole_is_not_presented() {
        let (s, _) = octant();
        let b = Boundary {
            sense: true,
            outer: None,
            edges: vec![rim(1, [1, 1], Axis::V, 1.0, (0.0, 1.0), (TURN, 1.0))],
            vertices: BTreeMap::from([
                (
                    vertex(1),
                    Point3::new(iv0(1.0_f64.cos()), iv0(0.0), iv0(1.0_f64.sin())),
                ),
                (vertex(3), Point3::new(iv0(0.0), iv0(0.0), iv0(1.0))),
            ]),
            extent: Some([(0.0, TURN), (1.0, core::f64::consts::FRAC_PI_2)]),
        };
        let local = Local {
            edges: Vec::new(),
            vertices: vec![vertex(3)],
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &b,
            u: (1.2, 1.8),
            v: (1.58, 1.6),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        // No vertex is shared in this cell, so a rectangle that kept its
        // width would be presented with no zero.
        assert!(matches!(
            present_chart(&piece, &[], &[], band()),
            Err(Unpresented::Local)
        ));
    }

    /// **A face that holds no vertex at a pole is given no pole**, and
    /// its extent is the stored one: a vertex a millimetre of arc off
    /// the pole, on a unit sphere, at either end; a face of another
    /// surface with a vertex where a sphere's pole would be; and the
    /// south pole's vertex moves the extent's low end alone. Red where
    /// the extent is widened for a vertex off the pole, or at the wrong
    /// end.
    #[test]
    fn a_face_short_of_its_pole_reads_no_pole() {
        let q = core::f64::consts::FRAC_PI_2;
        let sphere = octant().0;
        let lone = |z: f64, v: (f64, f64)| Boundary {
            sense: true,
            vertices: BTreeMap::from([(
                vertex(3),
                Point3::new(iv0((1.0 - z * z).sqrt()), iv0(0.0), iv0(z)),
            )]),
            extent: Some([(0.0, 1.0), v]),
            ..Boundary::default()
        };
        let v_of = |s: &Surface<Interval>, b: &Boundary| match face_extent(s, b, band()) {
            Ok(Extent::Known([_, v])) => Some(v),
            _ => None,
        };
        let short = q - 1e-3;
        for (z, stored, want, what) in [
            (
                short.sin(),
                (0.5, short),
                (0.5, short),
                "north, off the pole",
            ),
            (
                -short.sin(),
                (-short, -0.5),
                (-short, -0.5),
                "south, off the pole",
            ),
            (
                -1.0,
                (-q.next_down(), -0.5),
                (pole_latitude(false).lo(), -0.5),
                "at the south pole",
            ),
        ] {
            assert_eq!(v_of(&sphere, &lone(z, stored)), Some(want), "{what}");
        }
        let cylinder = cylinder();
        let stored = (0.5, q.next_down());
        assert_eq!(
            v_of(&cylinder, &lone(1.0, stored)),
            Some(stored),
            "a cylinder has no pole"
        );
        // The octant cut off at the latitude `v = 1.5`: no vertex at the
        // pole, so a cell reaching past the latitude is bounded by it
        // and holds no collapsed side.
        let (s, mut b) = octant_ending_at(1.5);
        b.vertices.insert(
            vertex(3),
            Point3::new(iv0(0.0), iv0(1.5_f64.cos()), iv0(1.5_f64.sin())),
        );
        assert_eq!(v_of(&s, &b), Some((0.0, 1.5)), "the cut octant's extent");
        let local = Local {
            edges: vec![1, 2],
            vertices: vec![vertex(3)],
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &b,
            u: (0.7, 0.8),
            v: (1.4, q.next_up()),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let got = present_chart(&piece, &[], &[vertex(3)], band());
        assert!(
            !matches!(&got, Ok(Presentation::Of(p)) if matches!(p.zero, Zero::Side(Side::YHi, _))),
            "the cut octant reads no pole"
        );
    }

    /// A boundary of one vertex, the point `p`, and the extent `v` over
    /// `u ∈ [0, 1]`: what [`face_extent`] reads of a singular point.
    fn lone(p: Point3<Interval>, v: (f64, f64)) -> Boundary {
        Boundary {
            sense: true,
            vertices: BTreeMap::from([(vertex(3), p)]),
            extent: Some([(0.0, 1.0), v]),
            ..Boundary::default()
        }
    }

    /// The extent's `v` as [`face_extent`] returns it: `Ok` where it is
    /// known, `Err` where the face refuses.
    fn v_of(s: &Surface<Interval>, b: &Boundary) -> Result<(f64, f64), Unpresented> {
        match face_extent(s, b, band()) {
            Ok(Extent::Known([_, v])) => Ok(v),
            Ok(_) => panic!("an extent neither wrapping nor unknown"),
            Err(e) => Err(e),
        }
    }

    /// A sphere of radius 1 about `(0, 0, c)`, its axis `z`.
    fn sphere_at(c: f64) -> Surface<Interval> {
        Surface::Sphere {
            frame: geom::Frame::new(
                Point3::new(iv0(0.0), iv0(0.0), iv0(c)),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            radius: iv0(1.0),
        }
    }

    /// A right-angled cone with its apex at `(0, 0, c)`, its axis `z`.
    fn cone_at(c: f64) -> Surface<Interval> {
        Surface::Cone {
            frame: geom::Frame::new(
                Point3::new(iv0(0.0), iv0(0.0), iv0(c)),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
        }
    }

    /// **The extent moves to a singular latitude where its stored end is
    /// at the singular point by the band, and nowhere else**: the one
    /// incidence decision, asked of the row's end. On a unit sphere and a
    /// right-angled cone, at the origin and a hundred units off it, a
    /// face holding the vertex at the pole or apex whose stored extent
    /// ends half the band's zero short of the latitude, in metres along
    /// the surface, reaches it; one whose extent ends twice the band's
    /// escalation short contradicts its vertex and refuses `Structural`.
    /// Both poles, and both nappes. Red where the bound is a scale of
    /// this reader's own, tighter or looser than the band.
    #[test]
    fn an_extent_moves_where_its_stored_end_is_at_the_singular_point_by_the_band() {
        let (north, south) = (pole_latitude(true), pole_latitude(false));
        let (near, far) = (0.5 * band().zero(), 2.0 * band().escalate());
        for c in [0.0, 100.0] {
            let at_z = |z: f64| Point3::new(iv0(0.0), iv0(0.0), iv0(c + z));
            let sphere = sphere_at(c);
            assert_eq!(
                v_of(&sphere, &lone(at_z(1.0), (0.5, north.lo() - near))),
                Ok((0.5, north.hi())),
                "at {c}: in the band under the north pole"
            );
            assert_eq!(
                v_of(&sphere, &lone(at_z(-1.0), (south.hi() + near, -0.5))),
                Ok((south.lo(), -0.5)),
                "at {c}: in the band over the south pole"
            );
            for (z, v, what) in [
                (
                    1.0,
                    (0.5, north.lo() - far),
                    "past the band under the north pole",
                ),
                (1.0, (-1.0, 0.0), "a radian and a half under the north pole"),
                (
                    -1.0,
                    (south.hi() + far, -0.5),
                    "past the band over the south pole",
                ),
            ] {
                assert_eq!(
                    v_of(&sphere, &lone(at_z(z), v)),
                    Err(Unpresented::Structural),
                    "at {c}: {what}"
                );
            }
            // A cone face is on a nappe by a vertex off the apex.
            let cone = cone_at(c);
            let on = |up: bool, v: (f64, f64)| {
                let mut b = lone(at_z(0.0), v);
                let z = if up { 1.0 } else { -1.0 };
                b.vertices
                    .insert(vertex(4), Point3::new(iv0(1.0), iv0(0.0), iv0(c + z)));
                v_of(&cone, &b)
            };
            assert_eq!(
                on(true, (near, 1.0)),
                Ok((0.0, 1.0)),
                "at {c}: over the apex"
            );
            assert_eq!(
                on(false, (-1.0, -near)),
                Ok((-1.0, 0.0)),
                "at {c}: under it"
            );
            for (up, v, what) in [
                (true, (far, 1.0), "past the band over the apex"),
                (false, (-1.0, -far), "past the band under the apex"),
            ] {
                assert_eq!(on(up, v), Err(Unpresented::Structural), "at {c}: {what}");
            }
            // An end past the apex, on the other nappe, is on the face's
            // nappe where it is at the apex by the same decision.
            assert_eq!(
                on(true, (-near, 1.0)),
                Ok((-near, 1.0)),
                "at {c}: the nappe"
            );
            assert_eq!(
                on(true, (-far, 1.0)),
                Err(Unpresented::Structural),
                "at {c}: an end on the other nappe"
            );
            assert_eq!(
                on(false, (-1.0, near)),
                Ok((-1.0, near)),
                "at {c}: the nappe"
            );
            assert_eq!(
                on(false, (-1.0, far)),
                Err(Unpresented::Structural),
                "at {c}: an end on the other nappe"
            );
        }
    }

    /// **The face's lines are read against the stored extent, before it
    /// is moved to a pole**: a face above the latitude a slack and a half
    /// under its north pole, holding the pole's vertex, whose stored
    /// extent ends 0.9 of a slack under the pole. The line has no room
    /// under the stored end, so the extent contradicts it and the face
    /// refuses `Structural`. Red, the extent known, where the line is
    /// read against the extent moved to the pole, which gives it room.
    #[test]
    fn a_line_is_read_against_the_stored_extent_not_the_one_moved_to_the_pole() {
        let (s, mut b) = octant();
        let q = core::f64::consts::FRAC_PI_2;
        let north = pole_latitude(true);
        let slack = chart_slack(&[0.0, north.hi()]);
        let lat = north.lo() - 1.5 * slack;
        b.edges[0] = rim(1, [1, 2], Axis::V, lat, (0.0, lat), (q, lat));
        b.extent = Some([(0.0, q), (0.0, north.lo() - 0.9 * slack)]);
        assert_eq!(v_of(&s, &b), Err(Unpresented::Structural));
        // The same line with room under the stored end agrees with it.
        b.extent = Some([(0.0, q), (0.0, north.lo())]);
        assert_eq!(v_of(&s, &b), Ok((0.0, north.hi())));
    }

    /// **The extent moves to the pole for the vertices the incidence
    /// reads at it, and no others**: on the unit sphere a vertex half the
    /// band's zero of arc off the north pole is at it, and one twice the
    /// band's escalation off is not, so the first face's extent ends on
    /// the latitude and the second's is the stored one. The reading is
    /// `vertex_at_singular_point`'s, in metres, reused; red where this
    /// reader takes a scale of its own.
    #[test]
    fn the_extent_moves_for_a_vertex_the_band_reads_at_the_pole() {
        let sphere = octant().0;
        let north = pole_latitude(true);
        let stored = (0.5, north.lo().next_down());
        let off = |arc: f64| Point3::new(iv0(arc.sin()), iv0(0.0), iv0(arc.cos()));
        assert_eq!(
            v_of(&sphere, &lone(off(0.5 * band().zero()), stored)),
            Ok((0.5, north.hi())),
            "in the band's zero"
        );
        assert_eq!(
            v_of(&sphere, &lone(off(2.0 * band().escalate()), stored)),
            Ok(stored),
            "past the band"
        );
        // The vertex in the band and off the pole, its rows ending at its
        // own latitude: the end is at the pole by the same decision.
        let arc = 0.5 * band().zero();
        assert_eq!(
            v_of(&sphere, &lone(off(arc), (0.5, north.lo() - arc))),
            Ok((0.5, north.hi())),
            "in the band's zero, the rows ending at the vertex's latitude"
        );
    }

    /// **A cone face whose stored extent starts a rounding off its apex
    /// places the apex.** The sector of
    /// [`a_cone_face_at_its_apex_is_read_on_its_own_nappe`], its
    /// meridians ending at the apex vertex, with the extent's low end at
    /// `0`, an ulp of 1 above it (as a meridian line `c + l·t` evaluates
    /// off the origin) and an ulp below. The face holds a vertex at the
    /// apex, so its extent reaches `v = 0` ([`face_extent`]) and the cell
    /// at the apex is presented with the apex's side its zero. Red, the
    /// cell `Local`, where the rectangle is clipped to a stored end above
    /// zero. A stored end off the apex by the band contradicts the vertex
    /// and refuses `Structural`; a face on the negative nappe reaches the
    /// apex at its high end.
    #[test]
    fn a_cone_face_whose_extent_starts_a_rounding_off_its_apex_places_the_apex() {
        let s = cone_at(0.0);
        let sector = |lo: f64| Boundary {
            sense: true,
            outer: None,
            edges: vec![
                rim(1, [9, 3], Axis::U, 2.0, (2.0, lo), (2.0, 1.0)),
                rim(2, [3, 4], Axis::V, 1.0, (2.0, 1.0), (1.0, 1.0)),
                rim(3, [4, 9], Axis::U, 1.0, (1.0, 1.0), (1.0, lo)),
            ],
            vertices: BTreeMap::from([
                (vertex(3), Point3::new(iv0(1.0), iv0(0.0), iv0(1.0))),
                (vertex(9), origin()),
            ]),
            extent: Some([(1.0, 2.0), (lo, 1.0)]),
        };
        // The low end the revolved cone at `y = 1.7` stores, and its mirror;
        // then ends past the cover's own `few_ulps`, as a meridian of a
        // cone 50 long stores (an ulp of 53.85), up to half the band's
        // zero: the meridian ends at the apex's vertex, so it runs on to
        // the collapsed side ([`ends_on_collapse`]).
        let ulp = 2.220_446_049_250_313e-16;
        let past = [
            1.5e-15,
            2.0e-15,
            7.105_427_357_601_002e-15,
            0.5 * band().zero(),
        ];
        for lo in [0.0, ulp, -ulp].into_iter().chain(past) {
            let b = sector(lo);
            assert_eq!(
                v_of(&s, &b),
                Ok((lo.min(0.0), 1.0)),
                "the extent from {lo:e}"
            );
            let local = Local {
                edges: vec![0, 2],
                vertices: vec![vertex(9)],
            };
            let piece = Piece {
                face: FaceKey::default(),
                surface: &s,
                whole: false,
                boundary: &b,
                u: (1.2, 1.8),
                v: (-0.1, 0.3),
                local: &local,
                ball: Ball::holding(origin(), origin()),
            };
            let got = match present_chart(&piece, &[], &[vertex(9)], band()) {
                Ok(Presentation::Of(p)) => Some((p.zero, p.y)),
                _ => None,
            };
            assert!(
                matches!(&got, Some((Zero::Side(Side::YLo, a), (0.0, 0.3)))
                    if a.vertex == Some(vertex(9))),
                "the extent from {lo:e}: the apex's side is the zero: {got:?}"
            );
        }
        assert_eq!(
            v_of(&s, &sector(2.0 * band().escalate())),
            Err(Unpresented::Structural),
            "a stored end off the apex by the band"
        );
        // The same gap under an edge whose end vertex is another: the
        // meridians end at vertices 7 and 8, beside the apex's vertex 9,
        // and nothing runs them on.
        let mut beside = sector(past[2]);
        beside.edges[0].ends[0] = vertex(7);
        beside.edges[2].ends[1] = vertex(8);
        let local = Local {
            edges: vec![0, 2],
            vertices: vec![vertex(9)],
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &beside,
            u: (1.2, 1.8),
            v: (-0.1, 0.3),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        assert!(
            matches!(
                present_chart(&piece, &[], &[vertex(9)], band()),
                Err(Unpresented::Local)
            ),
            "an edge that ends beside the apex, at another vertex"
        );
        let mut below = lone(origin(), (-1.0, -ulp));
        below
            .vertices
            .insert(vertex(4), Point3::new(iv0(1.0), iv0(0.0), iv0(-1.0)));
        assert_eq!(v_of(&s, &below), Ok((-1.0, 0.0)), "the negative nappe");
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

    /// **A curved corner folded on itself is a cusp**: [`certify_pair_in`]
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
            stated: None,
            key: edge(n),
            arc: Some((
                Curve3::line(
                    Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    geom_core::UnitVec3::exact_x(),
                ),
                (iv0(t.0), iv0(t.1)),
            )),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                curve: Some(ChartCurve::Harmonic(h)),
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
            stated: None,
            key: edge(10),
            arc: Some((
                Curve3::line(on_cyl(1.0, 0.5), geom_core::UnitVec3::exact_z()),
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
                curve: None,
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
                vertices.entry(e.ends[1]).or_insert(*origin + dir.get());
            }
        }
        Boundary {
            sense: true,
            edges: [ruling, other].into_iter().chain(extra).collect(),
            vertices,
            extent: Some([(0.0, 1.0), (0.0, 1.0)]),
            outer: None,
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
            stated: None,
            key: edge(n),
            arc: Some((
                Curve3::line(
                    on_cyl(1.0, 0.5),
                    if up {
                        geom_core::UnitVec3::exact_z()
                    } else {
                        -geom_core::UnitVec3::exact_z()
                    },
                ),
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
                curve: None,
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
            outer: None,
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
    /// ([`certify_pair_in`]): the face above the parallel and left of the
    /// ruling against the one below it and right, diagonally opposite
    /// quadrants of one tangent plane, certify through the plane leaning
    /// furthest from both wedges. Against the face below the parallel and LEFT of the
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

    /// **A plane face's wedge at the origin** (vertex 1): the plane the
    /// origin, `d1` and `d2` span, its face the convex wedge from `d1`
    /// round to `d2` about `d1 × d2`, bounded by the lines out to
    /// vertices `n + 1` (along `d1`) and `n + 2` (along `d2`).
    fn plane_wedge(n: u64, d1: [f64; 3], d2: [f64; 3]) -> (Surface<Interval>, Boundary) {
        plane_wedge_at(n, 1, [0.0; 3], d1, d2)
    }

    /// [`plane_wedge`] with its vertex `key` at `q`.
    fn plane_wedge_at(
        n: u64,
        key: u64,
        q: [f64; 3],
        d1: [f64; 3],
        d2: [f64; 3],
    ) -> (Surface<Interval>, Boundary) {
        let v = |d: [f64; 3]| Vec3::new(iv0(d[0]), iv0(d[1]), iv0(d[2]));
        let o = Point3::new(iv0(q[0]), iv0(q[1]), iv0(q[2]));
        let (f1, f2) = (
            Vec3::new(d1[0], d1[1], d1[2]),
            Vec3::new(d2[0], d2[1], d2[2]),
        );
        let normal = unit(f1.cross(f2)).expect("a wedge spans a plane");
        let along = unit(f1).expect("a direction");
        let plane = Surface::Plane {
            frame: geom::Frame::new(o, lift(normal), lift(along)),
        };
        let line = |d: [f64; 3], far: u64, plus: bool| Rim {
            stated: None,
            key: edge(far),
            // The segment is its chord, its span in metres.
            arc: Some({
                let (dir, len) = geom_core::UnitVec3::chord(v(d));
                (Curve3::line(o, dir), (iv0(0.0), len))
            }),
            ends: [vertex(key), vertex(far)],
            uses: vec![Use {
                plus,
                image: None,
                iso: None,
                curve: None,
            }],
        };
        let b = Boundary {
            sense: true,
            edges: vec![line(d2, n + 2, false), line(d1, n + 1, true)],
            vertices: BTreeMap::from([
                (vertex(key), o),
                (vertex(n + 1), o + v(d1)),
                (vertex(n + 2), o + v(d2)),
            ]),
            extent: None,
            outer: None,
        };
        (plane, b)
    }

    /// **A plane wedge at a sphere's pole is parted by the heading plane**
    /// ([`certify_pair`]): [`octant`]'s cell at its pole, presented by the
    /// collapsed side `v = π/2` ([`Zero::Side`], no corner axes), against
    /// a plane wedge at the pole on the far side of the axis, one edge
    /// rising out of the ball and one sinking into it, its plane holding
    /// the cell's own heading. Neither carrier parts them (the wedge
    /// straddles the sphere, the cell its plane), and the wedge plane is
    /// not read; the plane across the two headings does. Red if the
    /// vertex-alone arm offers no plane where a face is not a quadrant.
    #[test]
    fn a_plane_wedge_at_a_spheres_pole_is_parted_by_the_heading_plane() {
        let (sphere, ob) = octant();
        let top = Interval::pi().hi() * 0.5;
        let local = Local {
            edges: vec![1, 2],
            vertices: vec![vertex(3)],
        };
        let near = Near {
            edges: Vec::new(),
            vertices: vec![vertex(3)],
        };
        let cap = Piece {
            face: face(1),
            surface: &sphere,
            whole: false,
            boundary: &ob,
            u: (0.7, 0.8),
            v: (1.5, top),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let (c, s) = (0.75f64.cos(), 0.75f64.sin());
        let (plane, pb) = plane_wedge_at(20, 3, [0.0, 0.0, 1.0], [-c, -s, 0.3], [-c, -s, -0.3]);
        let wedge_local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(3)],
        };
        let wedge = quadrant_piece(&plane, &pb, &wedge_local, 2, ((-0.02, 0.02), (-0.02, 0.02)));
        let zero = |p: &Piece<'_>| match present(
            p,
            &[],
            &near.vertices,
            band(),
            &Poses::of(&FacePoses::default()),
        ) {
            Ok(Presentation::Of(q)) => Some(q.zero.clone()),
            _ => None,
        };
        assert!(
            matches!(zero(&cap), Some(Zero::Side(Side::YHi, _)))
                && matches!(zero(&wedge), Some(Zero::Point(..))),
            "the pole's side against the wedge's quadrant: {:?} {:?}",
            zero(&cap),
            zero(&wedge)
        );
        let got = certify_pair(&cap, &wedge, &near, band());
        assert!(matches!(got, Outcome::Certified { .. }), "{got:?}");
    }

    /// **Two plane wedges at a vertex they share alone** ([`certify_pair`]
    /// through [`wedges_apart`]), each face presented as its quadrant
    /// there, a floor `A` from `a₁` round to `a₂` against a wedge `B`:
    ///
    /// - the keel's joint (`a_deep_keels_joint_is_embedded_in_every_pose`'s
    ///   pair, its cells as long and thin as the keel's): each wedge holds
    ///   an edge in the other's plane, so neither carrier parts them, and
    ///   the plane across the rectangles' middles leaves `B`'s edges on
    ///   `A`'s side;
    /// - a wedge through the floor's plane, each straddling the other's
    ///   plane, its cell long along its first edge: parted by no
    ///   difference of bisectors (`−0.013` on its second edge) and not by
    ///   the rectangles' middles;
    /// - a roof over the floor, its edges `(3, −1, 1)` and `(−1, 3, 1)`
    ///   past the floor's two, parted by no difference of bisectors (the
    ///   floor's carrier parts it);
    /// - a roof folded onto the floor's diagonal, holding its ray
    ///   `(1, 1, 0)`: the faces cross there, and no plane parts them.
    ///
    /// Red on a vertex-alone separator taken from the bisectors or the
    /// rectangles' middles, or if a plane that does not part the wedges
    /// were read as one.
    #[test]
    fn two_plane_wedges_at_a_vertex_alone_certify_only_where_a_plane_parts_them() {
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let near = Near {
            edges: Vec::new(),
            vertices: vec![vertex(1)],
        };
        let square = ((-0.02, 0.02), (-0.02, 0.02));
        type Wedge = ([f64; 3], [f64; 3], ((f64, f64), (f64, f64)));
        let cases: [(Wedge, Wedge, bool, &str); 4] = [
            (
                (
                    [0.351_123_441_588_391_76, 0.936_329_177_569_044_4, 0.0],
                    [0.0, 0.0, -1.0],
                    ((-0.0035, 0.0035), (-0.025, 0.025)),
                ),
                (
                    [0.0, 1.0, 0.0],
                    [0.0, 0.0, 1.0],
                    ((-0.028, 0.028), (-0.018, 0.018)),
                ),
                true,
                "the keel's joint",
            ),
            (
                ([1.0, 0.0, 0.0], [0.122, 0.993, 0.0], square),
                (
                    [-0.662, -0.735, 0.147],
                    [0.496, 0.0, -0.868],
                    ((-0.03, 0.03), (-0.005, 0.005)),
                ),
                true,
                "a wedge through the floor's plane",
            ),
            (
                ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], square),
                ([3.0, -1.0, 1.0], [-1.0, 3.0, 1.0], square),
                true,
                "the roof",
            ),
            (
                ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], square),
                ([1.0, 1.0, 1.0], [1.0, 1.0, -1.0], square),
                false,
                "the folded roof",
            ),
        ];
        for ((a1, a2, ca), (b1, b2, cb), parts, what) in cases {
            let (floor, fb) = plane_wedge(10, a1, a2);
            let (wedge, wb) = plane_wedge(20, b1, b2);
            let a = quadrant_piece(&floor, &fb, &local, 1, ca);
            let b = quadrant_piece(&wedge, &wb, &local, 2, cb);
            let presented = |p: &Piece<'_>| {
                matches!(present(p, &[], &near.vertices, band(), &Poses::of(&FacePoses::default())),
                    Ok(Presentation::Of(q)) if matches!(q.zero, Zero::Point(..)))
            };
            assert!(
                presented(&a) && presented(&b),
                "{what}: both faces are their quadrants at the vertex"
            );
            let got = certify_pair(&a, &b, &near, band());
            assert_eq!(
                matches!(got, Outcome::Certified { .. }),
                parts,
                "{what}: {got:?}"
            );
        }
    }

    /// **The wedge plane turns each corner axis into its rectangle**
    /// ([`wedges_apart`]): on [`floor`]'s own chart, the third quadrant
    /// presented by its rectangle's high corner (`Zero::Point(XHi, YHi)`)
    /// against the first presented by its low one. They are parted by the
    /// plane normal to `(−1, −1, 0)`. Red if an axis is read off the jet's
    /// column unturned: the third quadrant's wedge is then the first's,
    /// and no plane parts them.
    #[test]
    fn the_wedge_plane_turns_each_corner_axis_into_its_rectangle() {
        let s = floor();
        let quadrant = |x: (f64, f64), y: (f64, f64), zero: Zero| Presented {
            surface: &s,
            map: Map::Chart,
            x,
            y,
            zero,
            corner: false,
            cache: Cache::default(),
        };
        let third = quadrant(
            (-0.02, 0.0),
            (-0.02, 0.0),
            Zero::Point(Side::XHi, Side::YHi, vertex(1)),
        );
        let first = quadrant(
            (0.0, 0.02),
            (0.0, 0.02),
            Zero::Point(Side::XLo, Side::YLo, vertex(1)),
        );
        let n = wedges_apart(&third, &first).expect("the quadrants part");
        let want = unit(Vec3::new(-1.0, -1.0, 0.0)).expect("a direction");
        assert!(
            (n - want).dot(n - want) < 1e-24,
            "the plane leans furthest from both: {n:?}"
        );
    }

    /// **The wedge plane reads a curved corner's axes over its
    /// rectangle** ([`Presented::corner_axes`]), as [`sign_on`] decides:
    /// [`cylinder`]'s quadrant at `(1, ½)` over `u ∈ (1, 1.3)`, its `u`
    /// axis turning `0.3` rad across the rectangle, against a plane
    /// wedge at the corner on `(0, −0.6, 0.4)` and `(−0.9, −0.1, 0.8)`.
    /// The plane leaning furthest from both, read over the rectangle,
    /// parts the faces; read at the corner point alone it leaves the far
    /// end of the cylinder's turning axis on the wedge's side. Red if
    /// the axes are read at the corner point.
    #[test]
    fn the_wedge_plane_reads_a_curved_corners_axes_over_its_rectangle() {
        let cyl = cylinder();
        let p0 = on_cyl(1.0, 0.5);
        let quadrant = Presented {
            surface: &cyl,
            map: Map::Chart,
            x: (1.0, 1.3),
            y: (0.5, 0.52),
            zero: Zero::Point(Side::XLo, Side::YLo, vertex(1)),
            corner: false,
            cache: Cache::default(),
        };
        let (d1, d2) = (Vec3::new(0.0, -0.6, 0.4), Vec3::new(-0.9, -0.1, 0.8));
        let (n, u) = (
            unit(d1.cross(d2)).expect("a plane"),
            unit(d1).expect("an axis"),
        );
        let plane = Surface::Plane {
            frame: geom::Frame::new(p0, lift(n), lift(u)),
        };
        let v = n.cross(u);
        let wedge = Presented {
            surface: &plane,
            map: Map::Affine {
                o: [iv0(0.0), iv0(0.0)],
                a: [iv0(d1.dot(u)), iv0(0.0)],
                b: [iv0(d2.dot(u)), iv0(d2.dot(v))],
            },
            x: (0.0, 0.02),
            y: (0.0, 0.02),
            zero: Zero::Point(Side::XLo, Side::YLo, vertex(1)),
            corner: false,
            cache: Cache::default(),
        };
        let on = On {
            edges: Vec::new(),
            vertices: vec![vertex(1)],
            whole: true,
            run: None,
        };
        let n = wedges_apart(&quadrant, &wedge).expect("the wedges part");
        let psi = Psi::Linear { n: lift(n), q: p0 };
        let signs = (
            sign_on(&psi, Some(&on), &quadrant, Some(&wedge), arm(1.0), band()),
            sign_on(&psi, Some(&on), &wedge, Some(&quadrant), arm(1.0), band()),
        );
        assert!(
            matches!(signs, (Some(a), Some(b)) if a == flip(b)),
            "each face strictly on its own side: {signs:?}, {n:?}"
        );
    }

    /// **[`parting_normal`] finds a parting plane exactly where the two
    /// cones share no ray**, wherever the hull's nearest point falls: on
    /// an edge (two quadrants of one plane, a thin wedge below the
    /// other's diagonal) or inside a triangle (the roof, a wide wedge just
    /// above, a thin wedge laid across a thin one). Wedges crossing along
    /// a ray, sharing an edge, holding the other's axis or crossing
    /// through it are parted by none. Each `n` found parts the axes
    /// strictly. Red if the nearest point is not the hull's, or a point is
    /// kept that does not part.
    #[test]
    fn parting_normal_parts_two_cones_exactly_where_they_share_no_ray() {
        let d = |x: f64, y: f64, z: f64| unit(Vec3::new(x, y, z)).expect("a direction");
        let (x, y) = (d(1.0, 0.0, 0.0), d(0.0, 1.0, 0.0));
        for (a, b, parts, what) in [
            (
                [x, y],
                [d(3.0, -1.0, 1.0), d(-1.0, 3.0, 1.0)],
                true,
                "the roof",
            ),
            (
                [x, y],
                [d(-1.0, -1.0, 0.1), d(-1.0, -1.0, -0.1)],
                true,
                "a thin wedge below the diagonal",
            ),
            (
                [x, y],
                [d(-1.0, 0.0, 0.0), d(0.0, -1.0, 0.0)],
                true,
                "the opposite quadrant of one plane",
            ),
            (
                [x, y],
                [d(1.0, -0.01, 1.0), d(-0.01, 1.0, 1.0)],
                true,
                "a wide wedge just above",
            ),
            (
                [x, d(1.0, 1.0, 0.0)],
                [d(-1.0, 1.0, 0.0), d(-1.0, 0.0, 0.0)],
                true,
                "two thin wedges of one plane",
            ),
            (
                [x, y],
                [d(1.0, 1.0, 1.0), d(1.0, 1.0, -1.0)],
                false,
                "across the diagonal",
            ),
            (
                [x, y],
                [y, d(-1.0, 0.0, 0.0)],
                false,
                "the next quadrant, an edge shared",
            ),
            (
                [x, y],
                [d(1.0, 1.0, 0.0), d(0.0, 0.0, 1.0)],
                false,
                "an axis inside the other",
            ),
            (
                [x, d(1.0, 0.2, 0.0)],
                [d(1.0, -0.2, 0.05), d(1.0, 0.4, 0.05)],
                true,
                "a thin wedge laid across a thin one",
            ),
            (
                [x, y],
                [d(1.0, 0.0, 1.0), d(1.0, 0.0, -1.0)],
                false,
                "a wedge through an edge",
            ),
        ] {
            let got = parting_normal(a, b);
            assert_eq!(got.is_some(), parts, "{what}: {got:?}");
            if let Some(n) = got {
                assert!(
                    a.iter().all(|v| n.dot(*v) > 0.0) && b.iter().all(|v| n.dot(*v) < 0.0),
                    "{what}: n parts the axes strictly: {n:?}"
                );
            }
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
            stated: None,
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
                curve: None,
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
            stated: None,
            key: edge(11),
            arc: Some((section, (iv0(0.3), iv0(1.0)))),
            ends: [vertex(3), vertex(1)],
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                curve: Some(ChartCurve::Harmonic(Harmonic {
                    p0: [iv0(0.0), iv0(0.5 - s * 1f64.sin())],
                    pa: [iv0(0.0), iv0(0.0)],
                    pb: [iv0(0.0), iv0(s)],
                    pl: [iv0(1.0), iv0(0.0)],
                })),
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
            outer: None,
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
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            radius: iv0(r),
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
            *c = geom::Circle::new(c.frame(), iv0(r));
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
            stated: None,
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
                curve: None,
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
            outer: None,
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

    /// **An edge that does not stay on the face's side of the continued
    /// curve is not read through by it, but with it as one run** — a
    /// cylinder. The face lies below
    /// the curve `v = −0.4 + sin(u + asin 0.9)`, shared, which ends at the
    /// vertex `(0, ½)`, and below the rim `v = ½`, which leaves the vertex
    /// toward `+u` under the curve's continuation, rising there. The
    /// continuation comes back down across the rim at `u ≈ 0.90`, inside
    /// the cell `(−0.1, 1.0) × (0.4, 0.62)`: the curve up to the vertex
    /// and the rim from it are one graph over `u`. Red if [`leaving`]
    /// decides the tangents at the vertex alone, so the corner is read
    /// through by the curve continued.
    #[test]
    fn a_rim_the_continued_curve_comes_back_across_is_read_as_one_run() {
        let a = phase();
        let mut curve = harmonic_rim(
            10,
            [2, 1],
            harmonic([0.0, -0.4], 1.0, (a.sin(), a.cos(), 0.0)),
            (-1.5, 0.0),
        );
        curve.uses[0].plus = false;
        let rim_line = Rim {
            stated: None,
            key: edge(11),
            arc: Some((
                Curve3::line(
                    Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    geom_core::UnitVec3::exact_x(),
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
                curve: None,
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
            outer: None,
        };
        let s = cylinder();
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &b,
            u: (-0.1, 1.0),
            v: (0.4, 0.62),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let (_, t, along, past, run) =
            run_of(present_chart(&piece, &[0], &[vertex(1)], band())).expect("a run");
        assert!(t.1 == 0.0, "the curve up to the vertex: {t:?}");
        assert_eq!(
            (along.as_slice(), past, run),
            (
                &[edge(10)][..],
                Some(vertex(1)),
                RunOn {
                    edge: edge(11),
                    start: false
                }
            ),
            "the curve, on through the vertex along the rim"
        );
    }

    /// **The same, on a plane, is one run**: the quarter disc's line
    /// runs from `(1, 0)` toward `(0.98, −1)`, inside the continued
    /// circle at first, and crosses back out at `y ≈ −0.04`, inside the
    /// cell `(0.97, 1.02) × (−0.05, 0.05)`. The circle continued does not
    /// bound the face there; the arc and the line as one run do. Red if
    /// [`leaving`] decides the tangents at the vertex alone, so the
    /// corner is read through by the circle continued.
    #[test]
    fn a_line_the_continued_arc_comes_back_across_is_read_as_one_run() {
        let b = quarter(segment(20, [3, 1], (0.98, -1.0), (1.0, 0.0)), Vec::new());
        let (_, t, along, past, run) =
            run_about(&b, 0, (0.97, 1.02), (-0.05, 0.05)).expect("a run");
        assert!(t.0 == 0.0, "the arc from the vertex on: {t:?}");
        assert_eq!(
            (along.as_slice(), past, run.edge),
            (&[edge(10)][..], Some(vertex(1)), edge(20)),
            "the arc, on through the vertex along the line"
        );
    }

    /// **A reflex corner whose curve runs backward along its column is
    /// read by none**: the arc from `(1, 0)` clockwise to `(0, −1)`, its
    /// face inside the circle, the loop reaching the vertex along it and
    /// leaving along the line to `(2, 0)`, outside the circle — the
    /// quarter disc's reflex corner mirrored, its curve falling along
    /// `v`. Red if [`leaving`] drops the sign of the column's rate.
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
            stated: None,
            key: edge(30),
            arc: Some((
                Curve3::line(
                    Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    geom_core::UnitVec3::exact_x(),
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
                curve: None,
            }],
        };
        let ruling = Rim {
            stated: None,
            key: edge(31),
            arc: Some((
                Curve3::line(
                    Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    geom_core::UnitVec3::exact_x(),
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
                curve: None,
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
            outer: None,
        }
    }

    /// [`certify_pair_in`] of a face on [`floor`] in the cell `disc` and
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
    /// M on no shared cell. Red if [`leaving`] reads a decided
    /// `Negative` as leaving onto the face's side: the disc is then
    /// presented by the inside of the circle, the wall's carrier departs
    /// it, and the pair certifies.
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

    /// **A fold through a tangent joint is not certified.** The quarter
    /// disc whose line leaves `(1, 0)` along the circle's tangent holds
    /// the circle past the vertex in its interior, between the circle's
    /// continuation and the line; the wall past the corner stands on it
    /// there, a fold of M on no shared cell. The disc is one run of the
    /// arc and the line. Red if the joint is read through by the circle
    /// continued ([`leaving`] reading an undecided rate as leaving onto
    /// the face's side): the disc is then presented inside the circle,
    /// the wall's carrier departs it, and the pair certifies. (That the
    /// graph's own edge stops at the vertex is
    /// [`the_graph_runs_on_along_a_runs_other_edge`]'s.)
    #[test]
    fn a_fold_through_a_tangent_joint_is_not_certified() {
        let disc = quarter(segment(20, [3, 1], (1.0, -1.0), (1.0, 0.0)), Vec::new());
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
    /// past the corner. Red if [`leaving`] decides the tangents at the
    /// vertex alone.
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
                stated: None,
                key: edge(12),
                arc: Some((
                    Curve3::line(on_cyl(1.015, -0.5), geom_core::UnitVec3::exact_z()),
                    (iv0(0.0), iv0(2.0)),
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
        let Surface::Cylinder { frame, radius, .. } = cyl else {
            unreachable!()
        };
        let (origin, axis) = (frame.origin(), frame.axis_unit());
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
        let [u, v] = ChartCurve::Harmonic(h).at(t);
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
            ends_here: Vec::new(),
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
                stated: None,
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
                    curve: None,
                }],
            }],
            vertices: BTreeMap::new(),
            extent: None,
            outer: None,
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
                stated: None,
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
                    curve: Some(ChartCurve::Harmonic(Harmonic {
                        p0: [iv0(0.0), iv0(0.0)],
                        pa: [iv0(0.0), iv0(0.0)],
                        pb: [iv0(0.0), iv0(0.5)],
                        pl: [iv0(1.0), iv0(0.0)],
                    })),
                }],
            }],
            vertices: BTreeMap::from([(vertex(1), on(t0)), (vertex(2), on(t1))]),
            extent: None,
            outer: None,
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
            stated: None,
            key: edge(2),
            arc: Some((
                Curve3::line(lip(0.0502), geom_core::UnitVec3::exact_x()),
                (iv0(0.0), iv0(1e-4)),
            )),
            ends: [vertex(2), vertex(3)],
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                curve: None,
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
                axis: geom_core::UnitVec3::exact_z(),
                r: iv0(r),
            })
        };
        assert!(
            sign_on(&carrier(0.7), None, &q, None, arm(1.0), band()).is_some(),
            "the rim's own cylinder separates"
        );
        assert_eq!(
            sign_on(&carrier(0.69), None, &q, None, arm(1.0), band()),
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
            run: None,
        };
        assert!(
            sign_on(
                &plane,
                Some(&on(vec![rim], true)),
                &q,
                None,
                arm(1.0),
                band()
            )
            .is_some(),
            "listed"
        );
        assert_eq!(
            sign_on(
                &plane,
                Some(&on(vec![edge(9)], true)),
                &q,
                None,
                arm(1.0),
                band()
            ),
            None,
            "another edge listed"
        );
        assert_eq!(
            sign_on(
                &plane,
                Some(&on(vec![rim], false)),
                &q,
                None,
                arm(1.0),
                band()
            ),
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
                arm(1.0),
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
                arm(1.0),
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
            stops: Vec::new(),
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
            run: None,
        };
        let on = through(vec![edge(1)], true);
        let graph = Psi::Graph {
            m: z,
            t: Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            slope: iv0(0.0),
            piece: (0.0, 1.0),
        };
        assert_eq!(
            sign_on(&plane, Some(&on), &p, Some(&exact), arm(1.0), band()),
            Some(Sign::Positive),
            "admitted"
        );
        assert_eq!(
            sign_on(&plane, None, &p, None, arm(1.0), band()),
            None,
            "the carrier arm"
        );
        assert_eq!(
            sign_on(
                &graph,
                Some(&through(vec![edge(1)], false)),
                &p,
                Some(&exact),
                arm(1.0),
                band()
            ),
            None,
            "the graph separator"
        );
        assert_eq!(
            sign_on(&plane, Some(&on), &p, Some(&other), arm(1.0), band()),
            None,
            "against a past side"
        );
        assert_eq!(
            sign_on(
                &plane,
                Some(&through(vec![edge(2)], true)),
                &p,
                Some(&exact),
                arm(1.0),
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
            sign_on(&plane, Some(&on), &short, Some(&exact), arm(1.0), band()),
            None,
            "a side whose chart line is not its edge's own"
        );
        let patch = sheet();
        let free = Presented {
            surface: &patch,
            ..past(Zero::Past(Side::YLo, line))
        };
        assert_eq!(
            sign_on(&plane, Some(&on), &free, Some(&exact), arm(1.0), band()),
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
        let line = Curve3::line(origin(), geom_core::UnitVec3::chord(d).0);
        let face = |sense: bool| Boundary {
            sense,
            edges: vec![Rim {
                stated: None,
                key: edge(1),
                arc: Some((line.clone(), (iv0(0.0), iv0(4.0)))),
                ends: [vertex(1), vertex(2)],
                uses: vec![Use {
                    plus: true,
                    image: None,
                    iso: None,
                    curve: None,
                }],
            }],
            vertices: BTreeMap::from([
                (vertex(1), origin()),
                (vertex(2), line.eval(iv0(4.0))),
                (vertex(9), Point3::new(iv0(2.0), iv0(1.0), iv0(0.0))),
            ]),
            extent: None,
            outer: None,
        };
        let floor = Surface::Plane {
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
        };
        let wall = Surface::Plane {
            frame: geom::Frame::new(
                origin(),
                Vec3::new(Interval::from_f64(0.0) - s, iv0(1.0), iv0(0.0)),
                d,
            ),
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
            sign_on(psi, Some(on), &px, Some(&py), arm(1.0), band()),
            sign_on(psi, Some(on), &py, Some(&px), arm(1.0), band()),
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

    /// **A point-scalar body's two circles are one where it stores their
    /// frames one up to the signs of their axes, and not where one axis
    /// alone is a rounding off** ([`one_conic`], [`one_stated_conic`],
    /// [`one_stated_plane`]): a ball's two seam meridians in a turned
    /// frame, one origin and one `u`, the second's `v` and `w` turned
    /// over, are one circle though their lifts (the enclosures of the
    /// frames' polar factors) are not one value. Against the first: each
    /// of `u`, `v` and `w` a rounding off with the other two and the
    /// origin kept bit for bit, and the origin an ulp off, are not one
    /// circle; with `u.z` two ulps off, the two lifted `w` are disjoint
    /// enclosures, two planes for certain, which comparing the origin and
    /// `w` alone reads as one; another radius is another circle. Not one
    /// circle where only the lifts are held. Two exactly rigid frames with
    /// one axis and two `u`, whose lifts are points, are one circle by
    /// their lifts. Red if any one of the comparisons is dropped, or
    /// either reading is not read.
    #[test]
    fn two_stored_circles_are_one_only_with_every_axis_one_up_to_its_sign() {
        let o = Point3::new(0.299_999_999_999_999_9, -1.1, 2.9);
        let u = Vec3::new(
            -0.482_929_284_214_212_1,
            0.832_030_133_774_634_8,
            0.272_956_338_888_314_3,
        );
        let w = Vec3::new(
            -0.394_739_798_173_799_8,
            0.071_392_499_417_875_87,
            -0.916_015_066_887_317_3,
        );
        let v = w.cross(u);
        let frame = |o: Point3<f64>, u: Vec3<f64>, v: Vec3<f64>, w: Vec3<f64>| {
            let map = geom_core::Affine3 {
                linear: geom_core::Mat3::from_cols(u, v, w),
                translation: o - Point3::origin(),
            };
            geom::Frame::of_motion(
                &geom_core::RigidMotion::check_rigid(&map).expect("rigid within its rounding"),
            )
        };
        let rim = |f: geom::Frame<f64>, stated: bool| {
            let c = Curve3::Circle(geom::Circle::new(f, 1.0));
            Rim {
                key: edge(1),
                arc: Some((c.map_scalar(Interval::from_f64), (iv0(0.0), iv0(1.0)))),
                stated: stated.then_some(c),
                ends: [vertex(1), vertex(2)],
                uses: Vec::new(),
            }
        };
        let lifted_w = |r: &Rim| match r.arc.as_ref() {
            Some((Curve3::Circle(c), _)) => c.frame().axis(),
            _ => panic!("a circle"),
        };
        let a = rim(frame(o, u, v, w), true);
        let b = rim(frame(o, u, -v, -w), true);
        assert!(
            !one_vector(lifted_w(&a), lifted_w(&b) * at(-1.0)),
            "the lifts are no one value"
        );
        assert!(one_conic(&a, &b), "stored one up to signs");
        assert!(
            !one_conic(
                &rim(frame(o, u, v, w), false),
                &rim(frame(o, u, -v, -w), false)
            ),
            "the lifts alone"
        );
        let up = |x: Vec3<f64>| Vec3::new(x.x.next_up(), x.y, x.z);
        let u_off = rim(
            frame(o, Vec3::new(u.x, u.y, u.z.next_up().next_up()), v, w),
            true,
        );
        let apart = |x: Interval, y: Interval| x.hi() < y.lo() || y.hi() < x.lo();
        let (wa, wu) = (lifted_w(&a), lifted_w(&u_off));
        assert!(
            apart(wa.x, wu.x) || apart(wa.y, wu.y) || apart(wa.z, wu.z),
            "u two ulps off lifts w apart: {wa:?} {wu:?}"
        );
        for (what, other) in [
            ("u two ulps off", u_off),
            ("v an ulp off", rim(frame(o, u, up(v), w), true)),
            ("w an ulp off", rim(frame(o, u, v, up(w)), true)),
            (
                "the origin an ulp off",
                rim(frame(Point3::new(o.x.next_up(), o.y, o.z), u, v, w), true),
            ),
        ] {
            assert!(!one_conic(&a, &other), "{what} is another circle");
        }
        let wider = Rim {
            stated: Some(Curve3::Circle(geom::Circle::new(frame(o, u, -v, -w), 2.0))),
            arc: Some((
                Curve3::Circle(geom::Circle::new(frame(o, u, -v, -w), 2.0))
                    .map_scalar(Interval::from_f64),
                (iv0(0.0), iv0(1.0)),
            )),
            ..b.clone()
        };
        assert!(!one_conic(&a, &wider), "another radius is another circle");
        let (x, y, z) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        assert!(
            one_conic(
                &rim(frame(o, x, y, z), true),
                &rim(frame(o, y, Vec3::new(-1.0, 0.0, 0.0), z), true)
            ),
            "exactly rigid frames with one axis and two u are one circle, by their lifts"
        );
    }

    /// **Two stored ellipses are one only where stated alike**
    /// ([`one_stated_conic`]): an ellipse is one with itself restated;
    /// two held on their arcs, anchored `2⁻⁶⁰` apart with a semi-axis of
    /// 100 toward the centre, derive one centre to the bit and one frame
    /// there, yet are two ellipses (their centres `2⁻⁶⁰` apart), and are
    /// not one; two stated alike but for `v`, which the stated-content
    /// comparison does not read, are not one. Red if the frames at the
    /// derived centre are compared, or `v` is not.
    #[test]
    fn two_stored_ellipses_are_one_only_where_stated_alike() {
        let (x, y, z) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        let rim = |e: geom::Ellipse<f64>| {
            let c = Curve3::Ellipse(e);
            Rim {
                key: edge(1),
                arc: Some((c.map_scalar(Interval::from_f64), (iv0(0.0), iv0(1.0)))),
                stated: Some(c),
                ends: [vertex(1), vertex(2)],
                uses: Vec::new(),
            }
        };
        let near = geom::Ellipse::anchored(Point3::origin(), z, x, 100.0, 1.0);
        let off = 2f64.powi(-60);
        let far = geom::Ellipse::anchored(Point3::new(off, 0.0, 0.0), z, x, 100.0, 1.0);
        assert!(
            one_stated_plane(&near.frame(), &far.frame()),
            "one frame at one derived centre: {:?} {:?}",
            near.frame(),
            far.frame()
        );
        assert!(one_conic(&rim(near), &rim(near)), "restated alike");
        assert!(
            !one_conic(&rim(near), &rim(far)),
            "anchored apart is another ellipse"
        );
        let frame = |v: Vec3<f64>| {
            let map = geom_core::Affine3 {
                linear: geom_core::Mat3::from_cols(x, v, z),
                translation: Vec3::new(0.3, -1.1, 2.9),
            };
            geom::Frame::of_motion(
                &geom_core::RigidMotion::check_rigid(&map).expect("rigid within its rounding"),
            )
        };
        let tilted_v = Vec3::new(y.x, y.y, y.z + 1e-17);
        assert!(
            !one_conic(
                &rim(geom::Ellipse::new(frame(y), 2.0, 1.0)),
                &rim(geom::Ellipse::new(frame(tilted_v), 2.0, 1.0))
            ),
            "v a rounding off is another ellipse"
        );
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
            sign_on(psi, Some(on), &at(vertex(1)), None, arm(1.0), band()).is_some(),
            "at the edge's end, the plane's sign"
        );
        assert_eq!(
            sign_on(psi, Some(on), &at(vertex(9)), None, arm(1.0), band()),
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
            sign_on(&flat, None, &p, None, arm(1.0), band()),
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
            frame: geom::Frame::new(origin(), ez(), ex()),
            major_radius: iv0(major),
            minor_radius: iv0(0.5),
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
            frame: geom::Frame::new(origin(), ez(), ex()),
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
            stated: None,
            key: edge(n),
            arc: Some((spiric(), (iv0(t.0), iv0(t.1)))),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus,
                image: Some([p2((g(t.0), t.0)), p2((g(t.1), t.1))]),
                iso: None,
                curve: Some(ChartCurve::Wall(wall())),
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
            stated: None,
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
                curve: None,
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
            stated: None,
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
                curve: None,
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
            outer: None,
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
        present(
            &piece,
            near.0,
            near.1,
            band(),
            Box::leak(Box::new(Poses::of(Box::leak(Box::default())))),
        )
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
            frame: geom::Frame::new(
                Point3::new(iv0(0.1), iv0(-0.2), iv0(0.05)),
                Vec3::new(iv0(0.0), iv0(0.6), iv0(0.8)),
                ex(),
            ),
            major_radius: iv0(2.03),
            minor_radius: iv0(0.47),
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
            frame: geom::Frame::new(
                Point3::new(iv0(OFF), iv0(0.0), iv0(0.0)),
                ex(),
                Vec3::new(iv0(0.0), iv0(1.0), iv0(0.0)),
            ),
        };
        let mut b = spiric_face(false, true);
        b.edges[0].uses[0].curve = None;
        (plane, b)
    }

    /// [`certify_pair_in`] of a cap cell and a torus cell `cell` of
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
            sign_on(&meridian, None, &p, None, arm(1.0), band()),
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
    /// a corner, [`leaving`]): leaving to lower `u`, inside the
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
    /// other side is presented on the line and read**: the cell under
    /// the parallel, reaching it, clips to a rectangle with no width
    /// across the line, the line its zero side, and the parallel's plane
    /// is signed on it at the faces' reach. Red if the rectangle reads
    /// vacuous, or if the sign is levered at the rectangle's own width.
    #[test]
    fn a_line_corner_cell_reaching_the_line_from_outside_is_read_on_the_line() {
        let b = wall_corner(-0.5, false, 1.0);
        let s = ring(2.0);
        let local = at_the_corner();
        let cell = ((g(1.0) - 0.01, g(1.0) + 0.01), (0.98, 1.0));
        let Ok(Presentation::Of(p)) = on_wall(&s, &b, &local, (&[1], &[vertex(1)]), cell) else {
            panic!("the cell is presented");
        };
        assert!(matches!(p.zero, Zero::Side(..)), "the line is its zero");
        assert_eq!(p.y, (1.0, 1.0), "no width across the line");
        let line = &b.edges[1];
        let plane = Psi::Linear {
            n: ez(),
            q: Point3::new(iv0(0.0), iv0(0.0), iv0(0.5 * 1.0_f64.sin())),
        };
        let on = On {
            run: None,
            edges: vec![line.key],
            vertices: line.ends.to_vec(),
            whole: true,
        };
        assert_eq!(
            sign_on(&plane, Some(&on), &p, None, arm(1.0), band()),
            Some(Sign::Positive),
            "the side of the parallel's plane the face leaves the line to"
        );
        assert_eq!(
            sign_on(&plane, Some(&on), &p, None, None, band()),
            None,
            "no arm decides nothing"
        );
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
                frame: geom::Frame::new(center, ez(), ex()),
                major_radius: iv0(2.0),
                minor_radius: iv0(0.5),
            }
        }

        fn carrier(self) -> Curve3<Interval> {
            let axis = match self {
                Self::Flipped => Vec3::new(iv0(0.0), iv0(0.0), iv0(-1.0)),
                _ => ez(),
            };
            Curve3::Spiric {
                frame: geom::Frame::new(origin(), axis, ex()),
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
                    stated: None,
                    key: edge(1),
                    arc: Some((self.carrier(), (iv0(0.0), iv0(TURN)))),
                    ends: [vertex(1), vertex(1)],
                    uses: vec![Use {
                        plus: true,
                        image: Some([p2((u0, v0)), p2((u1, v1))]),
                        iso: None,
                        curve: Some(ChartCurve::Wall(self.wall())),
                    }],
                }],
                vertices: BTreeMap::from([(vertex(1), p)]),
                extent: None,
                outer: None,
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
    /// parallel onto its non-face side clips to a thin rectangle, but the
    /// corner is reflex, the face beyond the vertex is in the cell, and
    /// the reading refuses `Structural`. Red if the thin rectangle is
    /// presented, or read vacuous.
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
            curve: None,
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

    /// **A strip beside a shared line is presented however thin**
    /// ([`present_chart`]): a cell `1e-13` wide beside [`slot`]'s shared
    /// right line on a cylinder. Red if a width across a shared line
    /// reads the cell as holding none of the face.
    #[test]
    fn a_strip_beside_a_shared_line_is_presented_however_thin() {
        let b = slot();
        let cell = ((1.5, 1.5 + 1e-13), (0.3, 0.7));
        let s = Surface::Cylinder {
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            radius: iv0(1.0),
        };
        assert_eq!(
            chart_near(&s, &b, &[0, 1], &[1], cell),
            Read::Of(cell.0, cell.1)
        );
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
                axis: geom_core::UnitVec3::exact_z(),
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
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
        };
        let along = |end: Interval| Boundary {
            sense: true,
            edges: vec![Rim {
                stated: None,
                key: edge(1),
                arc: Some((
                    Curve3::line(origin(), geom_core::UnitVec3::exact_x()),
                    (iv0(0.0), end),
                )),
                ends: [vertex(1), vertex(2)],
                uses: vec![Use {
                    plus: true,
                    image: None,
                    iso: None,
                    curve: None,
                }],
            }],
            vertices: BTreeMap::new(),
            extent: None,
            outer: None,
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
                run: None,
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
                Box::leak(Box::new(Poses::of(Box::leak(Box::default())))),
            ) {
                Ok(Presentation::Of(q)) => q,
                _ => panic!("presented"),
            }
        }
        let sign = |q: &Presented<'_>, piece| {
            let (psi, on) = rim_graph(piece);
            sign_on(&psi, Some(&on), q, None, arm(1.0), band())
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
            run: None,
        };
        assert!(
            sign_on(
                &rim_plane,
                Some(&holds_rim),
                &corner,
                None,
                arm(1.0),
                band()
            )
            .is_some(),
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
            sign_on(
                &rim_plane,
                Some(&holds_rim),
                &off_line,
                None,
                arm(1.0),
                band()
            ),
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
            frame: geom::Frame::new(Point3::new(iv0(0.0), iv0(0.0), iv0(0.5 + rho)), ez(), ex()),
            major_radius: iv0(1.0),
            minor_radius: iv0(rho),
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
            curve: None,
        }];
        let (s1, c1) = 1.0f64.sin_cos();
        let meridian = Rim {
            stated: None,
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
                curve: None,
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
            outer: None,
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
        let faces = (FacePoses::default(), FacePoses::default());
        let poses = (Poses::of(&faces.0), Poses::of(&faces.1));
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
            sign_on(&psi, Some(&on), &qx, Some(&qy), arm(1.0), band()).is_some(),
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
            (
                a.on_edges,
                sign_on(&psi, Some(&on), &q, None, arm(1.0), band()),
            )
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
            r.uses[0].curve = Some(ChartCurve::Harmonic(h));
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
            outer: None,
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
            run: None,
        };
        assert_eq!(
            sign_on(&graph((0.25, 0.45)), Some(&on), &q, None, arm(1.0), band()),
            Some(Sign::Positive),
            "within the piece"
        );
        assert_eq!(
            sign_on(&graph((0.35, 0.45)), Some(&on), &q, None, arm(1.0), band()),
            None,
            "short of the side"
        );
        assert_eq!(
            sign_on(&graph((0.55, 0.75)), Some(&on), &q, None, arm(1.0), band()),
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
            outer: None,
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
            sign_on(&graph((0.6, 1.0)), Some(&on), &q, None, arm(1.0), band()).is_some(),
            "the shared edge's end at the vertex, its parameter 1"
        );
        assert_eq!(
            sign_on(&graph((0.0, 0.4)), Some(&on), &q, None, arm(1.0), band()),
            None,
            "the shared edge's far end"
        );
    }

    // ------------------------------- rows from the review of the run

    /// A face on [`floor`] outside the unit circle's upper
    /// half (edge 0, key 10, run clockwise to `(1, 0)`, vertex 1), which
    /// a small circle continues tangentially there: centre `(0.8, 0)`,
    /// radius `0.2`, clockwise over `sweep` (edge 1, key 11, to vertex 3),
    /// then a line back to `(-1, 0)` (vertex 2). The face is left of the
    /// loop: outside both circles, below the line.
    fn bulb(sweep: f64) -> Boundary {
        let z = Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0));
        let x = Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0));
        let one = |plus: bool| {
            vec![Use {
                plus,
                image: None,
                iso: None,
                curve: None,
            }]
        };
        let upper = Rim {
            stated: None,
            key: edge(10),
            arc: Some((
                Curve3::circle(origin(), z, iv0(1.0), x),
                (iv0(0.0), iv0(core::f64::consts::PI)),
            )),
            ends: [vertex(1), vertex(2)],
            uses: one(false),
        };
        let small = Rim {
            stated: None,
            key: edge(11),
            arc: Some((
                Curve3::circle(Point3::new(iv0(0.8), iv0(0.0), iv0(0.0)), -z, iv0(0.2), x),
                (iv0(0.0), iv0(sweep)),
            )),
            ends: [vertex(1), vertex(3)],
            uses: one(true),
        };
        let end = (0.8 + 0.2 * sweep.cos(), -0.2 * sweep.sin());
        let back = segment(20, [3, 2], end, (-1.0, 0.0));
        let p = |x: f64, y: f64| Point3::new(iv0(x), iv0(y), iv0(0.0));
        Boundary {
            outer: None,
            sense: true,
            edges: vec![upper, small, back],
            vertices: BTreeMap::from([
                (vertex(1), p(1.0, 0.0)),
                (vertex(2), p(-1.0, 0.0)),
                (vertex(3), p(end.0, end.1)),
            ]),
            extent: None,
        }
    }

    /// **A curved next edge that comes back over the columns inside `R`
    /// is not read as a run.** A ladder that decided the small circle's
    /// advance only up to the first parameter past the columns' far end
    /// (`v = -0.18`) would present this cell; the circle turns at `v = -0.2` and
    /// comes back through the columns at `u = 0.8 - sqrt(0.04 - v^2)`,
    /// inside `R` for `u` in `(0.66, 0.713)`. Its far vertex `(0.6, 0)`
    /// is outside the ball of `R`, so [`alone`] sees only the two edges.
    /// The face point `(0.67, -0.16)` (outside the small circle, below
    /// the line `v = 0`) is in the cell, at lower `u` than the run
    /// (`u = 0.92` there): on the side the presentation names as not the
    /// face.
    #[test]
    fn a_curved_next_edge_that_comes_back_into_r() {
        let s = floor();
        let b = bulb(core::f64::consts::PI);
        let (u, v) = ((0.66, 1.04), (-0.18, 0.1));
        let m = |(l, h): (f64, f64)| geom::mid_param(l, h);
        let ball = Ball::holding(s.point_eval(m(u), m(v)), s.rect_enclosure(u, v));
        let local = Local::of(&b, ball, band());
        let piece = Piece {
            face: face(1),
            surface: &s,
            whole: false,
            boundary: &b,
            u,
            v,
            local: &local,
            ball,
        };
        let r = read(present_plane(&piece, &[0], &[], band()));
        assert!(
            !matches!(r, Read::Of(..)),
            "a cell whose face has points on both sides of the run is presented by one side: {r:?}"
        );
    }

    /// **The graph does not run on along an edge that turns back along
    /// `t`.** The unit circle's arc from 30° (its vertex) to 90°, its
    /// face inside the circle, and a line reaching the
    /// vertex from up-right at 45°: a reflex corner that is one run over
    /// `u` (the arc falls to the vertex, the line rises from it), while
    /// along the arc's tangent the line comes back the way the arc went:
    /// `γ` would be two-valued over `s`. Red if [`graph_run`] does not
    /// decide the other edge's advance along `t`.
    #[test]
    fn the_graph_does_not_run_on_along_an_edge_that_turns_back_along_t() {
        let a = core::f64::consts::FRAC_PI_6;
        let (vx, vy) = (a.cos(), a.sin());
        let mut b = quarter(
            segment(20, [3, 1], (vx + 0.5, vy + 0.5), (vx, vy)),
            Vec::new(),
        );
        let (carrier, _) = b.edges[0].arc.clone().expect("the arc");
        b.edges[0].arc = Some((carrier, (iv0(a), iv0(core::f64::consts::FRAC_PI_2))));
        b.vertices
            .insert(vertex(1), Point3::new(iv0(vx), iv0(vy), iv0(0.0)));
        let s = floor();
        let local = both_arcs();
        let x = Piece {
            u: (vx - 0.02, vx + 0.02),
            v: (vy - 0.02, vy + 0.02),
            ..piece_on(&s, &b, &local, 1)
        };
        let presented = present_plane(&x, &[0], &[], band());
        let Ok(Presentation::Of(p)) = presented else {
            panic!(
                "the kinked reflex corner is not presented: {:?}",
                read(presented)
            );
        };
        let Zero::Curve(_, c) = &p.zero else {
            panic!("not a curve");
        };
        assert!(c.run.is_some(), "the corner is one run over u");
        let cyl = cylinder();
        let wall = Presented {
            surface: &cyl,
            map: Map::Chart,
            x: (a, a + 0.02),
            y: (0.0, 0.02),
            zero: Zero::Side(
                Side::YLo,
                Along {
                    edges: vec![edge(10)],
                    vertex: None,
                    whole: true,
                    params: Some(Params {
                        t: (a, a + 0.02),
                        periodic: true,
                    }),
                    on_edges: true,
                    stops: Vec::new(),
                },
            ),
            corner: false,
            cache: Cache::default(),
        };
        let near = Near {
            edges: vec![(0, 0)],
            vertices: Vec::new(),
        };
        let g = graph_of(&x, &x, &near, &p, &wall, band());
        assert!(g.is_none(), "a graph two-valued over s is no separator");
    }

    /// **A run is not zero under the plane of its conic**: the cylinder face of
    /// [`a_rim_the_continued_curve_comes_back_across_is_read_as_one_run`],
    /// whose shared curve `v = -0.4 + sin(u + a)` is the section of the
    /// unit cylinder by the plane `z = -0.4 + x sin a + y cos a`. That
    /// plane holds the curve continued past the vertex, where it comes
    /// back down across the rim at `u = 0.90` inside the cell: the face
    /// between the rim and the curve continued is on its other side. So
    /// the run is not zero under it. Red if a run is read zero under a
    /// plane holding its curve's whole conic.
    #[test]
    fn a_run_is_not_zero_under_the_plane_of_its_conic() {
        let a = phase();
        let mut curve = harmonic_rim(
            10,
            [2, 1],
            harmonic([0.0, -0.4], 1.0, (a.sin(), a.cos(), 0.0)),
            (-1.5, 0.0),
        );
        curve.uses[0].plus = false;
        let rim_line = Rim {
            stated: None,
            key: edge(11),
            arc: Some((
                Curve3::line(
                    Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    UnitVec3::exact_x(),
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
                curve: None,
            }],
        };
        let b = Boundary {
            outer: None,
            sense: true,
            edges: vec![curve, rim_line],
            vertices: BTreeMap::from([
                (vertex(1), on_cyl(0.0, 0.5)),
                (vertex(2), on_cyl(-1.5, -0.4 + (a - 1.5).sin())),
                (vertex(3), on_cyl(2.5, 0.5)),
            ]),
            extent: Some([(-1.6, 2.6), (-1.5, 0.6)]),
        };
        let s = cylinder();
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &s,
            whole: false,
            boundary: &b,
            u: (-0.1, 1.0),
            v: (0.4, 0.62),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let Ok(Presentation::Of(mut p)) = present_chart(&piece, &[0], &[vertex(1)], band()) else {
            panic!("the run is presented");
        };
        // The fixture's carrier is a stand-in line; the section is an
        // ellipse, whose own plane holds it whole.
        let Zero::Curve(_, c) = &mut p.zero else {
            panic!("a curve");
        };
        assert!(c.run.is_some(), "a run");
        c.along.whole = true;
        let plane = Psi::Linear {
            n: Vec3::new(iv0(-a.sin()), iv0(-a.cos()), iv0(1.0)),
            q: Point3::new(iv0(0.0), iv0(0.0), iv0(-0.4)),
        };
        let on = On {
            edges: vec![edge(10)],
            vertices: Vec::new(),
            whole: true,
            run: None,
        };
        let fl = floor();
        let exact = Presented {
            surface: &fl,
            map: Map::Chart,
            x: (0.9, 1.1),
            y: (-0.1, 0.1),
            zero: Zero::None,
            corner: false,
            cache: Cache::default(),
        };
        let under_run = sign_on(&plane, Some(&on), &p, Some(&exact), arm(1.0), band());
        // The same side with its run forgotten is read: the fixture
        // reaches the arm, so the refusal above is the run's.
        let Zero::Curve(_, c) = &mut p.zero else {
            panic!("a curve");
        };
        c.run = None;
        let p = Presented {
            cache: Cache::default(),
            ..*p
        };
        let without = sign_on(&plane, Some(&on), &p, Some(&exact), arm(1.0), band());
        assert!(
            without.is_some(),
            "the fixture reaches the plane arm: {without:?}"
        );
        assert_eq!(under_run, None, "a run under the plane of its conic");
    }

    /// **a line that runs back over the
    /// columns the arc came across is no run.** The quarter disc's line
    /// reaches `(1, 0)` from `(1.3, 0.5)`: outside the circle (reflex),
    /// and over `v` on the arc's own columns, so the two are two points
    /// to a column. Red if [`line_onward`] takes the way the line runs
    /// for the way the run must go.
    #[test]
    fn a_line_that_runs_back_over_the_columns_is_not_a_run() {
        let b = quarter(segment(20, [3, 1], (1.3, 0.5), (1.0, 0.0)), Vec::new());
        assert_eq!(
            on_floor(&b, vec![0, 1], vec![vertex(1)], (0.98, 1.02), (-0.02, 0.02)),
            Read::Structural,
            "a hairpin reflex corner"
        );
    }

    /// **A side that names no vertex is read past the run's vertex by
    /// no distance.** The tangent joint's floor against a wall whose side
    /// runs past the vertex along the circle by `1e-3` down to `1e-16`,
    /// on no key: no graph, at any of them. Red if a stretch past the
    /// vertex is admitted on its length.
    #[test]
    fn a_side_on_no_key_is_read_past_the_vertex_by_no_distance() {
        let s = floor();
        let local = both_arcs();
        let b = quarter(segment(20, [3, 1], (1.0, -1.0), (1.0, 0.0)), Vec::new());
        let x = piece_on(&s, &b, &local, 1);
        let Ok(Presentation::Of(p)) = present_plane(&x, &[0], &[], band()) else {
            panic!("presented");
        };
        let cyl = cylinder();
        let near = Near {
            edges: vec![(0, 0)],
            vertices: Vec::new(),
        };
        for k in 3..=16 {
            let d = 10f64.powi(-k);
            let wall = Presented {
                surface: &cyl,
                map: Map::Chart,
                x: (-d, 0.02),
                y: (0.0, 0.02),
                zero: Zero::Side(
                    Side::YLo,
                    Along {
                        edges: vec![edge(10)],
                        vertex: None,
                        whole: true,
                        params: Some(Params {
                            t: (-d, 0.02),
                            periodic: true,
                        }),
                        on_edges: true,
                        stops: Vec::new(),
                    },
                ),
                corner: false,
                cache: Cache::default(),
            };
            assert!(
                graph_of(&x, &x, &near, &p, &wall, band()).is_none(),
                "a side {d:e} past the vertex, on no key"
            );
        }
    }

    /// **What a keyed side may be cut by: any distance.** The wall of
    /// [`a_side_on_no_key_is_read_past_the_vertex_by_no_distance`], its
    /// side now stopped at the run's vertex by key
    /// ([`Along::stops`]), its parameters past the vertex by `0.1` down
    /// to `1e-16`: a graph at every one, the side read to the vertex.
    /// [`Along::ending`] compares no length, so nothing here bounds the
    /// stretch: the bound is the body's, enforced where the stop is
    /// recorded ([`present_chart`], on a rectangle
    /// [`sides_are_the_faces`] has passed), and a side built by hand
    /// with a stop, as here, has none. Red if a keyed side is refused
    /// on its length; the row states what the code does, not a bound.
    #[test]
    fn a_keyed_side_is_cut_at_the_vertex_however_far_past_it_reaches() {
        let s = floor();
        let local = both_arcs();
        let b = quarter(segment(20, [3, 1], (1.0, -1.0), (1.0, 0.0)), Vec::new());
        let x = piece_on(&s, &b, &local, 1);
        let Ok(Presentation::Of(p)) = present_plane(&x, &[0], &[], band()) else {
            panic!("presented");
        };
        let cyl = cylinder();
        let near = Near {
            edges: vec![(0, 0)],
            vertices: Vec::new(),
        };
        for k in 1..=16 {
            let d = 10f64.powi(-k);
            let wall = Presented {
                surface: &cyl,
                map: Map::Chart,
                x: (-d, 0.02),
                y: (0.0, 0.02),
                zero: Zero::Side(
                    Side::YLo,
                    Along {
                        edges: vec![edge(10)],
                        vertex: None,
                        whole: true,
                        params: Some(Params {
                            t: (-d, 0.02),
                            periodic: true,
                        }),
                        on_edges: true,
                        stops: vec![vertex(1)],
                    },
                ),
                corner: false,
                cache: Cache::default(),
            };
            assert!(
                graph_of(&x, &x, &near, &p, &wall, band()).is_some(),
                "a keyed side {d:e} past the vertex"
            );
        }
    }

    /// **A chart side records as stops the corners it makes, and no
    /// other end of its edge.** The cylinder's rim `v = ½` clipped at the
    /// ruling `u = 1` through its end, vertex 1
    /// ([`a_chart_side_run_on_past_its_edges_end_answers_nothing_under_the_graph`]'s
    /// corner): the rim's side stops at vertex 1 and not at vertex 2,
    /// the rim's other end, which no line of the other axis clips the
    /// cell at. Red if a side's stops are taken from every clip, its own
    /// included (every end of its edge a stop). On a whole patch, which
    /// keeps its cell and is not asked whether its sides are its edges',
    /// no stop is recorded.
    #[test]
    fn a_chart_side_stops_at_the_corners_it_makes_and_at_no_other_end() {
        let s = cylinder();
        let b = rim_corner(
            rim(11, [3, 1], Axis::U, 1.0, (1.0, 0.0), (1.0, 0.5)),
            (1.0, 0.0),
        );
        let local = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let stops = |whole: bool| {
            // A whole patch's lines are bounds of its exact domain.
            let (u, v) = if whole {
                ((0.97, 1.0), (0.45, 0.5))
            } else {
                ((0.97, 1.01), (0.45, 0.51))
            };
            let piece = Piece {
                face: FaceKey::default(),
                surface: &s,
                whole,
                boundary: &b,
                u,
                v,
                local: &local,
                ball: Ball::holding(origin(), origin()),
            };
            match present_chart(&piece, &[0], &[vertex(1)], band()) {
                Ok(Presentation::Of(q)) => match q.zero {
                    Zero::Side(_, a) => Ok(a.stops),
                    other => Err(format!("{other:?}")),
                },
                other => Err(format!("{:?}", read(other))),
            }
        };
        assert_eq!(stops(false), Ok(vec![vertex(1)]), "the rim's side");
        assert_eq!(stops(true), Ok(Vec::new()), "on a whole patch");
    }

    /// **A wedge plane and a cone face sharing a ruling to the apex.** The
    /// cone `Φ(u, v) = v·(cos u, sin u, 1)/√2` about `z`, its face over
    /// `u ∈ [0, 3π/2]`, `v ∈ [0, 1]`, its apex `A` a collapsed side, its
    /// rulings at `u = 0` (edge 1, shared) and `u = 3π/2` (edge 2). The
    /// plane `y = 0`, charted `(z, x)`, holds the shared ruling, whose
    /// direction there is `(1, 1)`, and a second edge from `A` along
    /// `far`; so it holds the cone's ruling at `u = π`, along `(1, −1)`,
    /// on the cone face. The pair is read in the plane's cell at `A` and
    /// the cone's at `A` about `u = π`, where they share `A` and are near
    /// the shared ruling: [`certify_pair`]'s outcome.
    fn wedge_at_the_apex(far: (f64, f64)) -> Outcome {
        let s2 = core::f64::consts::FRAC_1_SQRT_2;
        let pi = core::f64::consts::PI;
        let p3 = |x: f64, y: f64, z: f64| Point3::new(iv0(x), iv0(y), iv0(z));
        // A segment is its chord, its span in metres, read off the
        // stated points.
        let line = |a: Point3<Interval>, b: Point3<Interval>| {
            let (dir, len) = geom_core::UnitVec3::chord(b - a);
            Some((Curve3::line(a, dir), (iv0(0.0), len)))
        };
        let apex = origin();
        let (p0, p1) = (p3(s2, 0.0, s2), p3(0.0, -s2, s2));
        // The plane's second edge ends at `q`, its chart point `far`.
        let q = p3(far.1, 0.0, far.0);
        let cone = Surface::Cone {
            frame: geom::Frame::new(
                apex,
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
        };
        let plane = Surface::Plane {
            frame: geom::Frame::new(
                apex,
                Vec3::new(iv0(0.0), iv0(1.0), iv0(0.0)),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            ),
        };
        let mut c = Boundary {
            sense: true,
            edges: vec![
                rim(1, [1, 2], Axis::U, 0.0, (0.0, 0.0), (0.0, 1.0)),
                rim(
                    2,
                    [1, 3],
                    Axis::U,
                    1.5 * pi,
                    (1.5 * pi, 0.0),
                    (1.5 * pi, 1.0),
                ),
                rim(3, [2, 3], Axis::V, 1.0, (0.0, 1.0), (1.5 * pi, 1.0)),
            ],
            vertices: BTreeMap::from([(vertex(1), apex), (vertex(2), p0), (vertex(3), p1)]),
            extent: Some([(0.0, 1.5 * pi), (0.0, 1.0)]),
            outer: None,
        };
        c.edges[0].arc = line(apex, p0);
        c.edges[0].uses[0].plus = false;
        c.edges[1].arc = line(apex, p1);
        // The rim: the circle at `v = 1` from `p0` to `p1`, run back.
        c.edges[2].arc = Some((
            Curve3::circle(
                p3(0.0, 0.0, s2),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                iv0(s2),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            (iv0(0.0), iv0(1.5 * pi)),
        ));
        c.edges[2].uses[0].plus = false;
        let unread = |n: u64, ends: [u64; 2], arc, plus| Rim {
            key: edge(n),
            arc,
            stated: None,
            ends: ends.map(vertex),
            uses: vec![Use {
                plus,
                image: None,
                iso: None,
                curve: None,
            }],
        };
        let w = Boundary {
            sense: true,
            edges: vec![
                unread(1, [1, 2], line(apex, p0), false),
                unread(4, [1, 4], line(apex, q), true),
                unread(5, [4, 2], line(q, p0), true),
            ],
            vertices: BTreeMap::from([(vertex(1), apex), (vertex(2), p0), (vertex(4), q)]),
            extent: None,
            outer: None,
        };
        let (lw, lc) = (
            Local {
                edges: vec![0, 1],
                vertices: vec![vertex(1)],
            },
            Local {
                edges: vec![0, 1],
                vertices: vec![vertex(1)],
            },
        );
        let near = Near {
            edges: vec![(0, 0)],
            vertices: vec![vertex(1)],
        };
        let piece = |face: FaceKey, surface, boundary, u, v, local| Piece {
            face,
            surface,
            whole: false,
            boundary,
            u,
            v,
            local,
            ball: Ball::holding(origin(), origin()),
        };
        let a = piece(face(1), &plane, &w, (0.0, 1e-3), (-1e-3, 1e-3), &lw);
        let b = piece(face(2), &cone, &c, (pi - 3e-3, pi + 3e-3), (0.0, 1e-3), &lc);
        certify_pair(&a, &b, &near, band())
    }

    /// **The cone's cell about the second ruling is parted from the wedge
    /// plane at the apex** ([`wedge_at_the_apex`], the plane's second edge
    /// the axis): no plane through the shared ruling signs the cone there,
    /// as the plane's carrier holds the cone's ruling at `u = π`, and the
    /// plane through the apex alone, both faces presented against it,
    /// does. Red while [`departure`] reads no separator but those through
    /// the shared edge, where the pair splits.
    #[test]
    fn a_cone_cell_about_the_wedge_planes_second_ruling_is_parted_at_the_apex() {
        let got = wedge_at_the_apex((1.0, 0.0));
        assert!(
            matches!(got, Outcome::Certified { .. }),
            "the plane through the apex parts the pair: {got:?}"
        );
    }

    /// **A wedge plane holding the cone's second ruling is not parted
    /// from it** ([`wedge_at_the_apex`], the plane's sector run on past
    /// the axis to `(1, −2)`): the plane face holds the cone's ruling at
    /// `u = π`, so the two faces meet along it, off the shared cell, and
    /// no separator through the apex reads them apart. Red if the plane
    /// through the apex certifies a pair that meets.
    #[test]
    fn a_wedge_plane_holding_the_cones_second_ruling_is_not_parted() {
        let got = wedge_at_the_apex((1.0, -2.0));
        assert!(
            !matches!(got, Outcome::Certified { .. }),
            "the faces meet along the ruling at u = π: {got:?}"
        );
    }

    /// **A cone cell across both nappes at its apex is presented on the
    /// face's own**: a whole cone face about `z` (`u` over a whole turn,
    /// its seam at `u = 0`, its rim at `v = 1`), whose extent wraps, so
    /// nothing but the nappe places its `v`. The cell at the apex reaching
    /// `v ∈ (−10⁻³, 10⁻³)` is presented against the apex alone as the
    /// rectangle `v ∈ [0, 10⁻³]` with the apex its collapsed low side, and
    /// the plane `z = 0` through the apex signs it positive. Red if the
    /// rectangle is not clamped to the nappe: it then reaches the other
    /// nappe, where the plane is negative, under the same sign
    /// ([`departure`]'s plane through the one shared vertex reads this).
    #[test]
    fn a_cone_cell_across_both_nappes_at_its_apex_is_presented_on_its_nappe() {
        let s2 = core::f64::consts::FRAC_1_SQRT_2;
        let cone = Surface::Cone {
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
        };
        let p0 = Point3::new(iv0(s2), iv0(0.0), iv0(s2));
        let mut seam = rim(1, [1, 2], Axis::U, 0.0, (0.0, 0.0), (0.0, 1.0));
        seam.arc = Some((
            Curve3::line(
                origin(),
                geom_core::UnitVec3::chord(Vec3::new(iv0(1.0), iv0(0.0), iv0(1.0))).0,
            ),
            (iv0(0.0), iv0(1.0)),
        ));
        let mut back = seam.uses[0].clone();
        back.plus = false;
        back.image = Some([p2((TURN, 0.0)), p2((TURN, 1.0))]);
        back.iso = Some(Iso {
            held: Axis::U,
            at: iv0(TURN),
            full: false,
        });
        seam.uses.push(back);
        let mut ring = rim(2, [2, 2], Axis::V, 1.0, (0.0, 1.0), (TURN, 1.0));
        ring.arc = Some((
            Curve3::circle(
                Point3::new(iv0(0.0), iv0(0.0), iv0(s2)),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                iv0(s2),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            (iv0(0.0), iv0(TURN)),
        ));
        ring.uses[0].plus = false;
        let b = Boundary {
            sense: true,
            edges: vec![seam, ring],
            vertices: BTreeMap::from([(vertex(1), origin()), (vertex(2), p0)]),
            extent: Some([(0.0, TURN), (0.0, 1.0)]),
            outer: None,
        };
        let local = Local {
            edges: vec![0],
            vertices: vec![vertex(1)],
        };
        let piece = Piece {
            face: FaceKey::default(),
            surface: &cone,
            whole: false,
            boundary: &b,
            u: (2.0, 2.1),
            v: (-1e-3, 1e-3),
            local: &local,
            ball: Ball::holding(origin(), origin()),
        };
        let Ok(Presentation::Of(q)) = present_chart(&piece, &[], &[vertex(1)], band()) else {
            panic!("the cell at the apex is presented");
        };
        assert_eq!(q.y, (0.0, 1e-3), "the rectangle is on the face's nappe");
        assert!(
            matches!(&q.zero, Zero::Side(Side::YLo, a) if a.vertex == Some(vertex(1))),
            "the apex is the rectangle's collapsed low side: {:?}",
            q.zero
        );
        let through = On {
            edges: Vec::new(),
            vertices: vec![vertex(1)],
            whole: true,
            run: None,
        };
        let plane = Psi::Linear {
            n: Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            q: origin(),
        };
        assert_eq!(
            sign_on(&plane, Some(&through), &q, None, arm(1.0), band()),
            Some(Sign::Positive),
            "the plane through the apex signs the face's nappe"
        );
    }

    /// [`wedge_at_the_apex`] in a true mirror (`y → −y`), on a re-charted
    /// cone (axis `−z`: `u' = π − u`, `v' = −v`) and a re-charted plane
    /// (normal `−y`), the plane's far edge at `(1, far.1)`, the cone's
    /// cell about `u = π` widened by `du` and reaching `dv` along the
    /// ruling: [`certify_pair`]'s outcome.
    fn mirrored_wedge(
        far: (f64, f64),
        mirror: bool,
        recone: bool,
        replane: bool,
        du: (f64, f64),
        dv: f64,
    ) -> Outcome {
        let s2 = core::f64::consts::FRAC_1_SQRT_2;
        let pi = core::f64::consts::PI;
        let m = if mirror { -1.0 } else { 1.0 };
        let p3 = |x: f64, y: f64, z: f64| Point3::new(iv0(x), iv0(m * y), iv0(z));
        // A segment is its chord, its span in metres, read off the
        // stated points.
        let line = |a: Point3<Interval>, b: Point3<Interval>| {
            let (dir, len) = geom_core::UnitVec3::chord(b - a);
            Some((Curve3::line(a, dir), (iv0(0.0), len)))
        };
        let apex = origin();
        let (p0, p1) = (p3(s2, 0.0, s2), p3(0.0, -s2, s2));
        let q = p3(far.1, 0.0, far.0);
        let cone = Surface::Cone {
            frame: geom::Frame::new(
                apex,
                Vec3::new(iv0(0.0), iv0(0.0), iv0(if recone { -1.0 } else { 1.0 })),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            half_angle: iv0(core::f64::consts::FRAC_PI_4),
        };
        let plane = Surface::Plane {
            frame: geom::Frame::new(
                apex,
                Vec3::new(iv0(0.0), iv0(if replane { -1.0 } else { 1.0 }), iv0(0.0)),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
            ),
        };
        // Chart of a cone point at original (u, v): mirror u -> -u (shift by 2π
        // to stay in [π/2, 2π]); recone (u, v) -> (π − u, −v).
        let ch = |u: f64, v: f64| -> (f64, f64) {
            let u = if mirror {
                if u <= 0.0 { -u } else { 2.0 * pi - u }
            } else {
                u
            };
            if recone { (pi - u, -v) } else { (u, v) }
        };
        let (u_sh, u_far) = (ch(0.0, 0.0).0, ch(1.5 * pi, 0.0).0);
        let vr = if recone { -1.0 } else { 1.0 };
        let mut c = Boundary {
            sense: !mirror,
            edges: vec![
                rim(1, [1, 2], Axis::U, u_sh, (u_sh, 0.0), (u_sh, vr)),
                rim(2, [1, 3], Axis::U, u_far, (u_far, 0.0), (u_far, vr)),
                rim(3, [2, 3], Axis::V, vr, (u_sh, vr), (u_far, vr)),
            ],
            vertices: BTreeMap::from([(vertex(1), apex), (vertex(2), p0), (vertex(3), p1)]),
            extent: Some([
                (u_sh.min(u_far), u_sh.max(u_far)),
                (vr.min(0.0), vr.max(0.0)),
            ]),
            outer: None,
        };
        c.edges[0].arc = line(apex, p0);
        c.edges[0].uses[0].plus = false;
        c.edges[1].arc = line(apex, p1);
        c.edges[2].arc = Some((
            Curve3::circle(
                p3(0.0, 0.0, s2),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(m)),
                iv0(s2),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            (iv0(0.0), iv0(1.5 * pi)),
        ));
        c.edges[2].uses[0].plus = false;
        let unread = |n: u64, ends: [u64; 2], arc, plus| Rim {
            key: edge(n),
            arc,
            stated: None,
            ends: ends.map(vertex),
            uses: vec![Use {
                plus,
                image: None,
                iso: None,
                curve: None,
            }],
        };
        let w = Boundary {
            sense: !(mirror ^ replane),
            edges: vec![
                unread(1, [1, 2], line(apex, p0), false),
                unread(4, [1, 4], line(apex, q), true),
                unread(5, [4, 2], line(q, p0), true),
            ],
            vertices: BTreeMap::from([(vertex(1), apex), (vertex(2), p0), (vertex(4), q)]),
            extent: None,
            outer: None,
        };
        let lw = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let lc = Local {
            edges: vec![0, 1],
            vertices: vec![vertex(1)],
        };
        let near = Near {
            edges: vec![(0, 0)],
            vertices: vec![vertex(1)],
        };
        let piece = |face: FaceKey, surface, boundary, u, v, local| Piece {
            face,
            surface,
            whole: false,
            boundary,
            u,
            v,
            local,
            ball: Ball::holding(origin(), origin()),
        };
        // The plane's chart is (z, ±x): x flips with replane.
        let a = piece(face(1), &plane, &w, (0.0, 1e-3), (-1e-3, 1e-3), &lw);
        let uc = ch(pi, 0.0).0;
        let cv = if recone { (-dv, 0.0) } else { (0.0, dv) };
        let b = piece(face(2), &cone, &c, (uc + du.0, uc + du.1), cv, &lc);
        certify_pair(&a, &b, &near, band())
    }

    /// **A wedge plane holding the cone's second ruling is never parted
    /// from it, in any mirror or chart** ([`mirrored_wedge`]): over both
    /// mirrors, both cone charts, both plane charts, eight far edges
    /// (those with `far.1 < −1` hold the ruling at `u = π`, so the faces
    /// meet along it), five cone cells and three reaches, 960 pairs, none
    /// holding the ruling certifies. Red if the plane through the one
    /// shared vertex certifies where it signs one face only.
    #[test]
    fn mirrored_and_recharted_wedges_never_certify_a_meeting() {
        let mut bad = Vec::new();
        for mirror in [false, true] {
            for recone in [false, true] {
                for replane in [false, true] {
                    for t in [0.9, 0.0, -0.5, -0.99, -1.01, -1.5, -2.0, -5.0] {
                        for du in [
                            (-3e-3, 3e-3),
                            (-1e-4, 2e-3),
                            (-2e-3, 1e-5),
                            (-0.3, 0.3),
                            (-1e-6, 1e-6),
                        ] {
                            for dv in [1e-3, 1e-6, 0.1] {
                                let got = mirrored_wedge((1.0, t), mirror, recone, replane, du, dv);
                                let cert = matches!(got, Outcome::Certified { .. });
                                if t < -1.0 && cert {
                                    bad.push(format!("mirror {mirror} recone {recone} replane {replane} far.1 {t} du {du:?} dv {dv}: {got:?}"));
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(bad.is_empty(), "false certificates: {bad:#?}");
    }
}

/// **`Psi::rate` against a finite difference of `Psi::value`**, member by
/// member, and the carrier arm's negative. The points are a fixed sample
/// (`Lcg`): these rows pin each arm's formula, which any sample of
/// off-axis points exercises; they are not a counterexample search.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod rate_rows {
    use super::*;

    fn iv0(x: f64) -> Interval {
        Interval::from_bounds(x, x)
    }
    fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).unwrap()
    }
    fn pt(q: Vec3<f64>) -> Point3<Interval> {
        Point3::new(iv0(q.x), iv0(q.y), iv0(q.z))
    }
    fn boxed(q: Vec3<f64>, h: f64) -> Point3<Interval> {
        let s = |x: f64| Interval::from_bounds(x - h, x + h);
        Point3::new(s(q.x), s(q.y), s(q.z))
    }
    fn vbox(v: Vec3<f64>, h: f64) -> Vec3<Interval> {
        let s = |x: f64| Interval::from_bounds(x - h, x + h);
        Vec3::new(s(v.x), s(v.y), s(v.z))
    }
    struct Lcg(u64);
    impl Lcg {
        /// Uniform in (-1, 1).
        fn next(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64) * 2.0 - 1.0
        }
        fn vec(&mut self, s: f64) -> Vec3<f64> {
            Vec3::new(self.next() * s, self.next() * s, self.next() * s)
        }
    }
    const D: [f64; 3] = [0.36, -0.48, 0.8];
    const O: [f64; 3] = [1.5, -2.25, 0.75];
    fn d() -> Vec3<f64> {
        Vec3::new(D[0], D[1], D[2])
    }
    fn o() -> Vec3<f64> {
        Vec3::new(O[0], O[1], O[2])
    }
    fn mid(x: Interval) -> f64 {
        0.5 * (x.lo() + x.hi())
    }
    /// A central difference of `psi.value` along `v` at `q`.
    fn fd(psi: &Psi, q: Vec3<f64>, v: Vec3<f64>) -> f64 {
        let h = 1e-5;
        let val = |s: f64| mid(psi.value(pt(q + v * s)).expect("a closed form"));
        (val(h) - val(-h)) / (2.0 * h)
    }
    fn carriers(axis: UnitVec3<Interval>, org: Vec3<f64>) -> Vec<(&'static str, Psi)> {
        let op = pt(org);
        let cone = |nappe: f64| {
            Psi::Carrier(Implicit::Cone {
                apex: op,
                axis,
                cos: iv0(0.8),
                sin: iv0(0.6),
                nappe: iv0(nappe),
            })
        };
        vec![
            (
                "cylinder",
                Psi::Carrier(Implicit::Cylinder {
                    o: op,
                    axis,
                    r: iv0(0.5),
                }),
            ),
            ("cone+", cone(1.0)),
            ("cone-", cone(-1.0)),
            (
                "sphere",
                Psi::Carrier(Implicit::Sphere { c: op, r: iv0(0.5) }),
            ),
            (
                "plane",
                Psi::Carrier(Implicit::Plane {
                    o: op,
                    n: axis.get(),
                }),
            ),
            (
                "linear",
                Psi::Linear {
                    n: axis.get(),
                    q: op,
                },
            ),
        ]
    }
    fn off_axis(q: Vec3<f64>) -> f64 {
        let w = q - o();
        (w - d() * w.dot(d())).norm()
    }

    /// **At a point, each carrier's rate is the derivative of its own
    /// value along `v`**, to 1e-7, and is itself a point to 1e-9: so a
    /// sign, a factor, or a swapped term in any arm goes red.
    #[test]
    fn each_carriers_rate_at_a_point_is_its_values_derivative() {
        let mut g = Lcg(7);
        let mut seen = 0;
        while seen < 200 {
            let q = o() + g.vec(2.0);
            if off_axis(q) < 0.2 {
                continue;
            }
            seen += 1;
            let v = g.vec(1.0);
            for (what, psi) in carriers(geom_core::UnitVec3::chord(d().map(iv0)).0, o()) {
                let rate = psi.rate(pt(q), v.map(iv0));
                let want = fd(&psi, q, v);
                assert!(
                    rate.hi() - rate.lo() < 1e-9
                        && rate.lo() - 1e-7 <= want
                        && want <= rate.hi() + 1e-7,
                    "{what} at {q:?} along {v:?}: rate {rate:?}, derivative {want}"
                );
            }
        }
    }

    /// **Over a box, an axis enclosure and a direction enclosure, the
    /// rate holds every member's derivative**: members sampled, each
    /// with its own axis, point and direction. A member's axis is a unit
    /// direction (a carrier's axis is a [`UnitVec3`]), so a sampled
    /// vector off the axis is read as its direction.
    #[test]
    fn each_carriers_rate_over_a_box_holds_every_sampled_members_derivative() {
        let mut g = Lcg(11);
        let mut seen = 0;
        while seen < 60 {
            let q = o() + g.vec(2.0);
            if off_axis(q) < 0.3 {
                continue;
            }
            seen += 1;
            let v = g.vec(1.0);
            for (hq, ha, hv) in [(1e-2, 0.0, 0.0), (1e-3, 1e-6, 1e-3), (1e-5, 1e-4, 1e-2)] {
                let axis = vbox(d(), ha);
                for ((what, psi), k) in carriers(geom_core::UnitVec3::chord(axis).0, o())
                    .into_iter()
                    .zip(0..)
                {
                    let rate = psi.rate(boxed(q, hq), vbox(v, hv));
                    for _ in 0..12 {
                        let a = d() + g.vec(ha);
                        let (qm, vm) = (q + g.vec(hq), v + g.vec(hv));
                        // The member's axis is its direction, lifted as the
                        // type lifts a stated `f64` direction.
                        let unit_a = geom_core::UnitVec3::stated(a * (1.0 / a.norm()))
                            .expect("unit to rounding")
                            .map_scalar(Interval::from_f64);
                        let (_, member) = carriers(unit_a, o()).swap_remove(k);
                        let want = fd(&member, qm, vm);
                        assert!(
                            rate.lo() - 1e-7 <= want && want <= rate.hi() + 1e-7,
                            "{what} box {hq} axis {ha} dir {hv} at {q:?}: rate {rate:?}, \
                             a member's derivative {want}"
                        );
                    }
                }
            }
        }
    }

    // Retired: a cylinder's rate about a long axis, and a cone's about
    // an axis off unit by the stored convention (`|dir|²` to 1e-10). A
    // carrier's axis is a `UnitVec3<Interval>` (PR 7349,
    // `onehome/carrier-directions`), which no door admits off unit at
    // `Interval` (`UnitVec3::checked` keeps a box only where `v·v − 1`
    // holds zero), so a non-unit axis reaching `rate` is unrepresentable:
    // a stronger guarantee than these rows gave. `rate`'s division by the
    // axis's length and `square_to`'s any length are left without a pin:
    // `work/embed/rate-reads-an-axis-length-the-unit-axis-type-retired.md`.

    /// **Over a box holding a cylinder's or a cone's axis, or the
    /// cone's apex, no sign is read off the rate.**
    #[test]
    fn no_sign_is_read_off_a_rate_over_a_box_holding_the_axis() {
        for (what, psi) in carriers(geom_core::UnitVec3::chord(d().map(iv0)).0, o())
            .into_iter()
            .take(3)
        {
            for (name, q, h) in [
                ("holds the axis", o() + d() * 0.4, 1e-3),
                ("holds the apex", o(), 1e-3),
                (
                    "touches the axis",
                    o() + d() * 0.4 + Vec3::new(1e-3, 0.0, 0.0),
                    1e-3,
                ),
            ] {
                let v = Vec3::new(0.8, 0.6, 0.0);
                let rate = psi.rate(boxed(q, h), v.map(iv0));
                assert!(
                    interval_sign_as(rate, SELF_INTERSECTION_DEPARTURE, band()).is_none(),
                    "{what} {name}: a sign read over a box holding the axis"
                );
            }
        }
    }

    /// **The graph arm's rate holds `m·v − s·(t·v)` for every slope `s`
    /// in its enclosure**: its gradient is `m − γ′(s)·t` by definition.
    #[test]
    fn the_graph_arms_rate_holds_every_slopes_rate() {
        let mut g = Lcg(17);
        for _ in 0..200 {
            let (m, t, v) = (g.vec(1.0), g.vec(1.0), g.vec(1.0));
            let (s0, s1) = {
                let (a, b) = (g.next(), g.next());
                (a.min(b), a.max(b))
            };
            let psi = Psi::Graph {
                m: m.map(iv0),
                t: t.map(iv0),
                slope: Interval::from_bounds(s0, s1),
                piece: (0.0, 1.0),
            };
            let rate = psi.rate(pt(o()), v.map(iv0));
            for k in 0..=4 {
                let s = s0 + (s1 - s0) * f64::from(k) / 4.0;
                let want = m.dot(v) - s * t.dot(v);
                assert!(
                    rate.lo() - 1e-12 <= want && want <= rate.hi() + 1e-12,
                    "slope {s}: rate {rate:?} want {want}"
                );
            }
            // and no wider than the three independent slopes can make it
            let slack = (s1 - s0)
                * (t.x * v.x)
                    .abs()
                    .max((t.y * v.y).abs())
                    .max((t.z * v.z).abs())
                * 3.0;
            let tight = (s1 - s0) * t.dot(v).abs();
            assert!(
                rate.hi() - rate.lo() <= tight + slack + 1e-12,
                "rate {rate:?} wider than its slopes: tight {tight} slack {slack}"
            );
        }
    }

    /// The plane through the ruling `(r, 0, z)` of the cylinder of radius
    /// `r` round `z`, at 45° to the tangent plane there: chart
    /// `(u, v) ↦ (r, 0, 0) + u·e + v·ẑ·(±1)`, `e = (1, 1, 0)/√2`. It
    /// crosses the cylinder along the ruling, `u = 0`.
    fn crossing_plane(r: f64) -> Surface<Interval> {
        let s = core::f64::consts::FRAC_1_SQRT_2;
        Surface::Plane {
            frame: geom::Frame::new(
                Point3::new(iv0(r), iv0(0.0), iv0(0.0)),
                Vec3::new(iv0(s), iv0(-s), iv0(0.0)),
                Vec3::new(iv0(s), iv0(s), iv0(0.0)),
            ),
        }
    }

    /// **The carrier arm's negative.** A plane face's cell with no zero
    /// on its sides, crossing the cylinder's carrier along a ruling's
    /// line in its interior, is not signed against that carrier, at
    /// radius 0.25 and at radius 4; the same cell moved clear of the
    /// line is. Red where the rate's enclosure is too small (a radial
    /// term not normalised at r < 1, a rate read as zero).
    #[test]
    fn a_plane_cell_crossing_a_cylinders_carrier_along_a_ruling_is_not_signed() {
        for r in [0.25, 4.0, 0.006] {
            let s = crossing_plane(r);
            let psi = Psi::Carrier(Implicit::Cylinder {
                o: Point3::new(iv0(0.0), iv0(0.0), iv0(0.0)),
                axis: geom_core::UnitVec3::exact_z(),
                r: iv0(r),
            });
            // Wide enough that a cell clear of the carrier departs it past
            // the band at every ε, and still small against the radius.
            let k = (r * 4e-3).max(100.0 * band().zero());
            let cell = |x: (f64, f64)| Presented {
                surface: &s,
                map: Map::Chart,
                x,
                y: (-k, k),
                zero: Zero::None,
                corner: false,
                cache: Cache::default(),
            };
            let crossing = cell((-0.3 * k, 1.7 * k));
            let value = centred(&psi, &crossing, crossing.x, crossing.y);
            assert_eq!(
                sign_on(
                    &psi,
                    None,
                    &crossing,
                    None,
                    super::presentation_rows::arm(1.0),
                    band()
                ),
                None,
                "r = {r}: a cell crossing the carrier is signed: {value:?}"
            );
            let clear = cell((0.5 * k, 1.5 * k));
            assert_eq!(
                sign_on(
                    &psi,
                    None,
                    &clear,
                    None,
                    super::presentation_rows::arm(1.0),
                    band()
                ),
                Some(Sign::Positive),
                "r = {r}: a cell clear of the carrier, outside it"
            );
            let inside = cell((-1.5 * k, -0.5 * k));
            assert_eq!(
                sign_on(
                    &psi,
                    None,
                    &inside,
                    None,
                    super::presentation_rows::arm(1.0),
                    band()
                ),
                Some(Sign::Negative),
                "r = {r}: a cell clear of the carrier, inside it"
            );
        }
    }

    /// **The graph arm names its slope once**: over a slope `[0.9, 1.1]`
    /// with `t` tilted, `m·v − (t·v)·γ′` is at most a third the width of
    /// `(m − t·γ′)·v`, which names the slope once per component (red
    /// where the rate is spelled that way).
    #[test]
    fn the_graph_arms_rate_names_its_slope_once() {
        let slope = Interval::from_bounds(0.9, 1.1);
        let (m, t) = (Vec3::new(0.6, 0.0, 0.8), Vec3::new(0.48, 0.6, -0.64));
        let v = vbox(Vec3::new(0.6, 0.6, 0.8), 1e-3);
        let psi = Psi::Graph {
            m: m.map(iv0),
            t: t.map(iv0),
            slope,
            piece: (0.0, 1.0),
        };
        let w = |x: Interval| x.hi() - x.lo();
        let rate = psi.rate(pt(o()), v);
        let per_component = (m.map(iv0) - t.map(iv0) * slope).dot(v);
        assert!(
            w(rate) * 3.0 <= w(per_component),
            "rate {:e} against {:e}",
            w(rate),
            w(per_component)
        );
    }

    /// **The sphere arm names `w` once in its dot**: over a cube 1e-2
    /// wide, `(w·v)/|w|` is narrower than `(w/|w|)·v` (red where the rate
    /// is spelled that way).
    #[test]
    fn the_sphere_arms_rate_names_w_once_in_its_dot() {
        let c = o() + d() * 0.7 + Vec3::new(0.8, 0.6, 0.0) * 0.5;
        let p = boxed(c, 1e-2);
        let psi = Psi::Carrier(Implicit::Sphere {
            c: pt(o()),
            r: iv0(0.5),
        });
        let v = Vec3::new(0.3, 0.5, -0.4).map(iv0);
        let w = |x: Interval| x.hi() - x.lo();
        let ws = p - pt(o());
        let unit_first = (ws * (at(1.0) / ws.norm())).dot(v);
        let rate = psi.rate(p, v);
        assert!(
            w(rate) < 0.9 * w(unit_first),
            "rate {:e} against {:e}",
            w(rate),
            w(unit_first)
        );
    }

    /// **`|∇ψ|` holds each arm's gradient norm**: a central difference
    /// along the gradient's own direction over its norm is 1 for the
    /// distance carriers and `|n|` for a plane.
    #[test]
    fn each_carriers_gradient_norm_is_its_rate_along_its_steepest_direction() {
        let mut g = Lcg(19);
        let mut seen = 0;
        while seen < 50 {
            let q = o() + g.vec(2.0);
            if off_axis(q) < 0.2 {
                continue;
            }
            seen += 1;
            for (what, psi) in carriers(geom_core::UnitVec3::chord(d().map(iv0)).0, o()) {
                let e = 1e-6;
                let grad = Vec3::new(
                    fd(&psi, q, Vec3::new(1.0, 0.0, 0.0)),
                    fd(&psi, q, Vec3::new(0.0, 1.0, 0.0)),
                    fd(&psi, q, Vec3::new(0.0, 0.0, 1.0)),
                );
                let norm = psi.gradient_norm();
                assert!(
                    norm.lo() - e <= grad.norm() && grad.norm() <= norm.hi() + e,
                    "{what} at {q:?}: |grad| {} against {norm:?}",
                    grad.norm()
                );
            }
        }
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
            curve: None,
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
            stated: None,
            ends: ends.map(vertex),
            uses: vec![u],
        }
    }

    /// The unit sphere about the origin, charted about `z` from `x`.
    fn sphere() -> Surface<Interval> {
        Surface::Sphere {
            frame: geom::Frame::new(pt(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0)),
            radius: iv0(1.0),
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
            outer: Some(vec![(0, true), (1, true), (2, true)]),
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
        let face_poses = FacePoses::default();
        let poses = Poses::of(&face_poses);
        match present(&piece, near.0, near.1, band(), &poses)? {
            Presentation::Vacuous => panic!("the cell holds the face"),
            Presentation::Of(q) => {
                let Surface::Sphere { frame, .. } = *q.surface else {
                    panic!("a sphere is presented on a sphere")
                };
                Ok((q.zero.clone(), q.x, q.y, vmid(frame.axis())))
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
        let face_poses = FacePoses::default();
        let set = poses_of(&piece, near.0, near.1, band(), &face_poses);
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
            outer: Some(vec![(0, true)]),
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
            frame: geom::Frame::new(pt(c, 0.0, 0.0), v3(-1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0)),
            half_angle: iv0(alpha),
        };
        let on = |u: f64, v: f64| {
            let r = v * alpha.sin();
            pt(c - v * alpha.cos(), r * u.cos(), -r * u.sin())
        };
        let line = |n: u64, ends: [u64; 2], u: f64, (v0, v1): (f64, f64)| {
            let (p, q) = (vmid_point(on(u, v0)), vmid_point(on(u, v1)));
            Rim {
                key: edge(n),
                arc: Some(chord_segment(lift3(p), lift3(q))),
                stated: None,
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
            outer: None,
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
        b.edges[1].arc = Some(chord_segment(p, q));
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
        let face_poses = FacePoses::default();
        let poses = Poses::of(&face_poses);
        assert_eq!(
            present(&piece, &[1], &[], band(), &poses).err(),
            Some(Unpresented::Structural),
            "the shared line is unread in every pose"
        );
        let face_poses = FacePoses::default();
        let set = poses_of(&piece, &[1], &[], band(), &face_poses);
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
            stated: None,
            ends: [vertex(1), vertex(4)],
            uses: vec![curve_use(true, e1, ps)],
        };
        b.edges.push(Rim {
            key: edge(4),
            arc: Some((c, (iv0(tm), t1))),
            stated: None,
            ends: [vertex(4), vertex(3)],
            uses: vec![curve_use(true, ps, top)],
        });
        b.vertices.insert(vertex(4), sp);
        b.outer = Some(vec![(0, true), (3, true), (1, true), (2, true)]);
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

    /// `w` scaled onto the unit sphere.
    fn on_sphere(w: [f64; 3]) -> Point3<Interval> {
        let n = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
        pt(w[0] / n, w[1] / n, w[2] / n)
    }

    /// The extents the triangle's two poses grant, read at `p`.
    fn holding(b: &Boundary, p: Point3<Interval>) -> Vec<Option<bool>> {
        posed_extents_holding(&sphere(), b, p, band())
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
            outer: Some(vec![(0, true)]),
        }
    }

    /// **A cap holding its pose's pole is granted no extent**
    /// ([`posed_extent`]): the cap `x ≥ A`, and the larger cap `x ≤ A`,
    /// each holding a pole of the pose about `x`. Its circle lifts to a
    /// whole parallel, which wraps the azimuth: unwrapped, the lift does
    /// not close. Red if the wrap goes unchecked.
    #[test]
    fn a_cap_holding_its_poses_pole_is_granted_no_extent() {
        // Each cap with a point of the hemisphere it holds, stated per
        // case: the cap `x ≥ A` towards `+x`, the larger cap `x ≤ A`
        // towards `−x`. A fixture's coordinate, no face normal.
        for (sense, towards) in [(true, 1.0), (false, -1.0)] {
            let p = on_sphere([towards, 0.1, 0.1]);
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
            outer: Some(vec![(0, forward), (1, !forward)]),
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
        let face_poses = FacePoses::default();
        let poses = Poses::of(&face_poses);
        let got = present(&piece, &[0], &[], band(), &poses);
        assert!(
            matches!(got, Ok(Presentation::Vacuous)),
            "outside the face, beside the parallel: {:?} (local {:?})",
            got.as_ref().err(),
            local.edges
        );
    }

    /// [`x_and_y_cut`] walked as the face, its long arc of `x = A` split in
    /// two at a vertex `eps` past `t = π` in the direction it is walked:
    /// the face's lowest point in the pose about `y`, `(A, −s, 0)` at
    /// `t = π`, then lies inside the first part's last piece, not at its
    /// end.
    fn x_and_y_cut_split(eps: f64) -> Boundary {
        let p = on_sphere([0.95, 0.1, 0.1]);
        let forward = [true, false]
            .into_iter()
            .find(|&f| holding(&x_and_y_cut(f), p) == vec![None, Some(true)])
            .expect("one way round is the face");
        let mut b = x_and_y_cut(forward);
        let (c, (t0, t1)) = b.edges[0].arc.clone().expect("the long arc");
        let at_t = if forward {
            core::f64::consts::PI + eps
        } else {
            core::f64::consts::PI - eps
        };
        let split = c.eval(iv0(at_t));
        let u = b.edges[0].uses[0].clone();
        b.edges[0] = Rim {
            key: edge(1),
            arc: Some((c.clone(), (t0, iv0(at_t)))),
            stated: None,
            ends: [vertex(1), vertex(3)],
            uses: vec![u.clone()],
        };
        b.edges.push(Rim {
            key: edge(3),
            arc: Some((c, (iv0(at_t), t1))),
            stated: None,
            ends: [vertex(3), vertex(2)],
            uses: vec![u],
        });
        b.vertices.insert(vertex(3), split);
        b.outer = Some(if forward {
            vec![(0, true), (2, true), (1, false)]
        } else {
            vec![(1, true), (2, false), (0, false)]
        });
        b
    }

    /// **A pose's hull holds a row's last piece** ([`posed_extent`]): the
    /// face `x ≥ A`, `y ≤ B` with its long arc split just past its lowest
    /// point in the pose about `y` ([`x_and_y_cut_split`]), so that point
    /// lies inside a row's last piece. The pose about `y` grants an extent
    /// that holds it. Red if a row's last piece is left out of the hull.
    #[test]
    fn a_poses_hull_holds_a_rows_last_piece() {
        let b = x_and_y_cut_split(5e-3);
        let low = pt(A, -across(A), 0.0);
        let got = holding(&b, low);
        assert_eq!(got.len(), 2, "two poses: {got:?}");
        assert!(
            got.contains(&Some(true)),
            "the pose about y holds the lowest point: {got:?}"
        );
        assert!(
            !got.contains(&Some(false)),
            "no extent leaves it out: {got:?}"
        );
    }

    /// The band `h ≤ z ≤ h + d`, `x ≥ A`, `y ≥ 0` of the unit sphere: the
    /// parallels `z = h` and `z = h + d` (the stored chart's), closed by the
    /// circle `x = A` and the meridian `y = 0`. `sense` picks the band or
    /// its complement.
    fn band_between_parallels(h: f64, d: f64, sense: bool) -> Boundary {
        let r = |z: f64| (1.0 - z * z).sqrt();
        let (r0, r1) = (r(h), r(h + d));
        let (y2, y3) = ((r0 * r0 - A * A).sqrt(), (r1 * r1 - A * A).sqrt());
        let (f2, f3) = (y2.atan2(A), y3.atan2(A));
        let (t2, t3) = (h.atan2(y2), (h + d).atan2(y3));
        let parallel =
            |z: f64, rr: f64| (pt(0.0, 0.0, z), v3(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0), rr);
        let flat = |plus: bool| curve_use(plus, (0.0, 0.0), (0.0, 0.0));
        let line = |plus: bool, held: Axis, at: f64| Use {
            iso: Some(Iso {
                held,
                at: iv0(at),
                full: false,
            }),
            ..flat(plus)
        };
        Boundary {
            sense,
            edges: vec![
                circle_rim(
                    1,
                    [1, 2],
                    parallel(h, r0),
                    (0.0, f2),
                    line(true, Axis::V, h.asin()),
                ),
                circle_rim(
                    2,
                    [2, 3],
                    x_circle(),
                    (t2.min(t3), t2.max(t3)),
                    flat(t3 > t2),
                ),
                circle_rim(
                    3,
                    [4, 3],
                    parallel(h + d, r1),
                    (0.0, f3),
                    line(false, Axis::V, (h + d).asin()),
                ),
                circle_rim(
                    4,
                    [4, 1],
                    (pt(0.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), v3(1.0, 0.0, 0.0), 1.0),
                    (-(h + d).asin(), -h.asin()),
                    line(true, Axis::U, 0.0),
                ),
            ],
            vertices: BTreeMap::from([
                (vertex(1), pt(r0, 0.0, h)),
                (vertex(2), pt(A, y2, h)),
                (vertex(3), pt(A, y3, h + d)),
                (vertex(4), pt(r1, 0.0, h + d)),
            ]),
            extent: None,
            outer: Some(vec![(0, true), (1, t3 > t2), (2, false), (3, true)]),
        }
    }

    /// **A face thinner than its rows' chords is granted no extent, either
    /// way round** ([`posed_extent`]'s margin): the band between the
    /// parallels `z = 0.3` and `z = 0.3 + 1e-6`, closed by the circle
    /// `x = A` and a meridian, and its complement, in the pose about `x`.
    /// The parallels are curves there, and a piece's chord strays from its
    /// arc by far more than the band is wide, so the polygon through the
    /// pieces' starts may wind either way: its area is within the margin
    /// of the pieces' rectangles, its sign undecided, and neither the band
    /// nor the rest of the sphere is granted an extent. (A face bounded by
    /// the pose's own circle and one other is a lens or a segment, convex,
    /// and its polygon winds its way; the margin is for a face like this.)
    /// Red if the area's sign is read without its margin: the polygon's
    /// sign then grants one of the two a hull of the band.
    #[test]
    fn a_face_thinner_than_its_rows_chords_is_granted_no_extent() {
        for sense in [true, false] {
            let b = band_between_parallels(0.3, 1e-6, sense);
            assert_eq!(
                holding(&b, on_sphere([0.9, 0.1, 0.3])),
                vec![None],
                "sense {sense}"
            );
        }
    }

    /// **A face with a row stored off its sphere is granted no extent**
    /// ([`on_sphere`]): the triangle with its circle `y = A` stored with a
    /// radius 1e-9 too large, an edge's tolerance off the sphere, not a
    /// rounding: its points' latitudes `asin(Z/R)` would place it where it
    /// is not. Neither pose grants an extent; the triangle as stored grants
    /// both ([`a_poses_extent_holds_its_face`]). Red if a row off the
    /// sphere is lifted into a granted extent.
    #[test]
    fn a_face_with_a_row_off_its_sphere_is_granted_no_extent() {
        let mut b = triangle();
        let (c, span) = b.edges[1].arc.clone().expect("the circle y = A");
        let c = *c.as_circle().expect("a circle");
        b.edges[1].arc = Some((
            Curve3::circle(
                c.frame().origin(),
                c.frame().axis(),
                c.radius() + iv0(1e-9),
                c.frame().u_ref(),
            ),
            span,
        ));
        assert_eq!(holding(&b, on_sphere([1.0, 1.0, 0.5])), vec![None, None]);
    }

    /// The half cap `x ≥ A`, `z ≥ 0`: the circle `x = A` over `z ≥ 0` and
    /// the equator arc through the pose's pole `(1, 0, 0)`.
    fn half_cap() -> Boundary {
        let s = across(A);
        let e = s.atan2(A);
        Boundary {
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
            outer: Some(vec![(0, true), (1, true)]),
        }
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
        assert_eq!(holding(&half_cap(), on_sphere([1.0, 0.0, 0.5])), vec![None]);
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
            arc: Some(chord_segment(lift3(corner), lift3(tip))),
            stated: None,
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod margin_rows {
    //! The departure's margin: a sine levered at the faces' reach.
    use super::presentation_rows::arm;
    use super::*;

    pub(super) fn iv0(x: f64) -> Interval {
        Interval::from_bounds(x, x)
    }

    pub(super) fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).unwrap()
    }

    pub(super) fn key<K: From<slotmap::KeyData>>(n: u64) -> K {
        slotmap::KeyData::from_ffi((1u64 << 32) | n).into()
    }

    pub(super) fn v3(x: f64, y: f64, z: f64) -> Vec3<Interval> {
        Vec3::new(iv0(x), iv0(y), iv0(z))
    }

    pub(super) fn p3(x: f64, y: f64, z: f64) -> Point3<Interval> {
        Point3::new(iv0(x), iv0(y), iv0(z))
    }

    /// A side on the one edge `e`, the whole of its line.
    pub(super) fn along(e: u64) -> Along {
        Along {
            stops: Vec::new(),
            edges: vec![key(e)],
            vertex: None,
            whole: true,
            params: None,
            on_edges: true,
        }
    }

    /// A separator that vanishes on the edges `edges` and the vertices
    /// `vertices`, whole.
    pub(super) fn on(edges: &[u64], vertices: &[u64]) -> On {
        On {
            run: None,
            edges: edges.iter().map(|&e| key(e)).collect(),
            vertices: vertices.iter().map(|&v| key(v)).collect(),
            whole: true,
        }
    }

    fn presented(
        surface: &Surface<Interval>,
        x: (f64, f64),
        y: (f64, f64),
        zero: Zero,
    ) -> Presented<'_> {
        Presented {
            surface,
            map: Map::Chart,
            x,
            y,
            zero,
            corner: false,
            cache: Cache::default(),
        }
    }

    /// The plane `z = 0`, charted `(u, v) = (x, y)`.
    fn floor() -> Surface<Interval> {
        Surface::Plane {
            frame: geom::Frame::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0)),
        }
    }

    /// The plane through the `x` axis that the floor leaves at the angle
    /// whose sine is `sine`, the floor's `+y` side on its positive side.
    fn wall(sine: f64) -> Psi {
        Psi::Linear {
            n: v3(0.0, sine, -(1.0 - sine * sine).sqrt()),
            q: p3(0.0, 0.0, 0.0),
        }
    }

    /// **A departure is levered at its arm, never at its rectangle.** The
    /// floor beside its edge on the `x` axis, against a wall through that
    /// edge:
    /// - a rectangle `1e-15` across, and one of no width at all, leave a
    ///   square wall decidedly at a metre's arm;
    /// - a rectangle a metre across decides nothing at an arm inside the
    ///   band, and nothing with no arm;
    /// - a wall the floor leaves at a sine three times the zero band is
    ///   undecided at a metre and decided at a hundred.
    ///
    /// Red if the margin reads the rectangle's extent across the side,
    /// if it is not levered at all, or if a missing arm reads as one.
    #[test]
    fn a_departure_is_levered_at_its_arm_never_at_its_rectangle() {
        let s = floor();
        let read = |across: f64, sine: f64, arm: Option<Lever<Interval>>| {
            let p = presented(
                &s,
                (0.0, 1.0),
                (0.0, across),
                Zero::Side(Side::YLo, along(1)),
            );
            sign_on(&wall(sine), Some(&on(&[1], &[])), &p, None, arm, band())
        };
        for across in [1e-15, 0.0] {
            assert_eq!(
                read(across, 1.0, arm(1.0)),
                Some(Sign::Positive),
                "a rectangle {across} across is read"
            );
        }
        // The sign is the side the face lies on, however thin: a
        // rectangle on the edge's other side reads the other sign, so two
        // faces folded onto one side of the wall are never read apart.
        let folded = presented(
            &s,
            (0.0, 1.0),
            (-1e-15, 0.0),
            Zero::Side(Side::YHi, along(1)),
        );
        assert_eq!(
            sign_on(
                &wall(1.0),
                Some(&on(&[1], &[])),
                &folded,
                None,
                arm(1.0),
                band()
            ),
            Some(Sign::Negative),
            "the other side of the edge"
        );
        let inside = 0.5 * (band().zero() + band().escalate());
        assert_eq!(read(1.0, 1.0, arm(inside)), None, "an arm inside the band");
        assert_eq!(read(1.0, 1.0, None), None, "no arm");
        let shallow = 3.0 * band().zero();
        assert_eq!(
            read(1.0, shallow, arm(1.0)),
            None,
            "a shallow wall at a metre"
        );
        assert_eq!(
            read(1.0, shallow, arm(100.0)),
            Some(Sign::Positive),
            "the same wall at a hundred metres"
        );
    }

    /// **A quadrant of no width departs at its arm.** A plane face read
    /// as its quadrant at a vertex its pair shares alone, in a cell that
    /// holds it only along one of its edges: the rectangle is `2.4e-15`
    /// across one axis. Under a plane through the vertex that both edges
    /// leave to one side, at sines of about `0.05` and `0.24`, the
    /// quadrant is signed at a metre's arm; under a plane one edge lies
    /// on the other side of, it is not.
    ///
    /// Red if either axis's departure is levered at the rectangle's
    /// extent along it, or if the two axes' signs are not both asked.
    #[test]
    fn a_quadrant_of_no_width_departs_at_its_arm() {
        let s = floor();
        let read = |n: Vec3<Interval>| {
            let p = presented(
                &s,
                (0.0, 2.4e-15),
                (0.0, 0.7727),
                Zero::Point(Side::XLo, Side::YLo, key(7)),
            );
            let psi = Psi::Linear {
                n,
                q: p3(0.0, 0.0, 0.0),
            };
            sign_on(&psi, Some(&on(&[], &[7])), &p, None, arm(1.0), band())
        };
        assert_eq!(
            read(v3(0.0528, 0.2377, 0.9699)),
            Some(Sign::Positive),
            "both edges leave the plane to one side"
        );
        assert_eq!(
            read(v3(-0.0528, 0.2377, 0.9699)),
            None,
            "the edges leave it to opposite sides"
        );
    }

    /// **The rate is read as a sine, whatever the chart's speed.** A
    /// cylinder of radius `r` on its ruling at `u = 0`, a rectangle of no
    /// width, against a plane through the ruling that the cylinder
    /// leaves at a sine `s`; the rate across the ruling is `r·s`.
    /// - `r = 1e-3`, `s` a hundred times the band: decided, where the
    ///   bare rate at a metre is a tenth of the band;
    /// - `r = 1e3`, `s` a hundredth of the band's zero: undecided, where
    ///   the bare rate at a metre is ten times it.
    ///
    /// Red if the rate is levered without its chart speed divided out.
    #[test]
    fn a_departures_rate_is_read_as_a_sine_whatever_the_charts_speed() {
        let read = |radius: f64, sine: f64| {
            let s = Surface::Cylinder {
                frame: geom::Frame::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0)),
                radius: iv0(radius),
            };
            let p = presented(&s, (0.0, 0.0), (0.0, 1.0), Zero::Side(Side::XLo, along(1)));
            // The ruling at `(r, 0, ·)`; the cylinder's direction across
            // it is `+y`.
            let plane = Psi::Linear {
                n: v3(-(1.0 - sine * sine).sqrt(), sine, 0.0),
                q: p3(radius, 0.0, 0.0),
            };
            sign_on(&plane, Some(&on(&[1], &[])), &p, None, arm(1.0), band())
        };
        assert_eq!(
            read(1e-3, 100.0 * band().escalate()),
            Some(Sign::Positive),
            "a slow chart"
        );
        assert_eq!(read(1e3, 0.01 * band().zero()), None, "a fast chart");
    }

    /// **At a pole the sine is read by its limit, and only there.** A
    /// sphere's cell at its north pole beside the meridian `u = 0`,
    /// against that meridian's plane: the rate across the meridian and
    /// the chart's speed both vanish at the pole, and the lune leaves the
    /// plane squarely all the way to it.
    /// - with the pole's side collapsed, the plane is signed;
    /// - with that side stated as a regular one on an edge, the same
    ///   rectangle answers nothing: between two regular sides the face is
    ///   tangent to the separator at the corner, and the mixed rate over
    ///   its own size does not bound the sine.
    ///
    /// Red if the collapsed side's reading divides by the chart's own
    /// speeds (`0/0` at the pole, and past one beside it), or if two
    /// regular sides are read by the mixed rate.
    #[test]
    fn a_pole_is_read_by_the_sines_limit_and_two_regular_sides_are_not() {
        let s = Surface::Sphere {
            frame: geom::Frame::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0)),
            radius: iv0(1.0),
        };
        let pole = core::f64::consts::FRAC_PI_2;
        let meridian = Psi::Linear {
            n: v3(0.0, 1.0, 0.0),
            q: p3(0.0, 0.0, 0.0),
        };
        let read_at = |top: Along, arm: Option<Lever<Interval>>| {
            let zero = Zero::Corner((Side::XLo, along(1)), (Side::YHi, top));
            let p = presented(&s, (0.0, 0.1), (pole - 0.1, pole), zero);
            sign_on(&meridian, Some(&on(&[1, 2], &[7])), &p, None, arm, band())
        };
        let read = |top: Along| read_at(top, arm(1.0));
        assert_eq!(read(Along::at(key(7))), Some(Sign::Positive), "the pole");
        assert_eq!(read(along(2)), None, "two regular sides");
        // The sine is at most one: at an arm inside the band it decides
        // nothing, where the mixed rate over the chart's speeds, ten and
        // more this near the pole, would.
        let inside = 0.3 * band().escalate();
        assert_eq!(
            read_at(Along::at(key(7)), arm(inside)),
            None,
            "the pole at an arm inside the band"
        );
        // The lune on the meridian's other side reads the other sign.
        let west = Zero::Corner((Side::XHi, along(1)), (Side::YHi, Along::at(key(7))));
        let p = presented(&s, (-0.1, 0.0), (pole - 0.1, pole), west);
        assert_eq!(
            sign_on(
                &meridian,
                Some(&on(&[1, 2], &[7])),
                &p,
                None,
                arm(1.0),
                band()
            ),
            Some(Sign::Negative),
            "the other side of the meridian"
        );
    }

    /// A line edge: its key, its ends' points (`None` for null
    /// scaffolding) and its ends' keys.
    type LineEdge = (u64, Option<([f64; 3], [f64; 3])>, [u64; 2]);

    /// A boundary of line edges over the vertices `vertices`.
    fn boundary(edges: &[LineEdge], vertices: &[(u64, [f64; 3])]) -> Boundary {
        Boundary {
            sense: true,
            edges: edges
                .iter()
                .map(|&(k, arc, ends)| Rim {
                    key: key(k),
                    arc: arc
                        .map(|(a, b)| chord_segment(p3(a[0], a[1], a[2]), p3(b[0], b[1], b[2]))),
                    stated: None,
                    ends: ends.map(key),
                    uses: Vec::new(),
                })
                .collect(),
            vertices: vertices
                .iter()
                .map(|&(k, p)| (key(k), p3(p[0], p[1], p[2])))
                .collect(),
            extent: None,
            outer: None,
        }
    }

    /// **The arm is the faces' reach, the least over what the pair is
    /// near.** Two faces sharing the edges `1` (along `x`, a metre) and
    /// `2` (along `z`, ten metres) at the origin, the second face
    /// reaching twenty metres along `y`:
    /// - near edge `1` alone, its reach from its middle: the second
    ///   face's far vertex, `√400.25`;
    /// - near edge `2` alone: that vertex again, `√425`;
    /// - near both, in either order: the lesser;
    /// - near the origin alone: the farthest vertex of either face from
    ///   it, the second face's, twenty;
    /// - one face against itself reads its own vertices alone, `√26`;
    /// - near a null edge: no arm.
    ///
    /// Red if the arm is one face's, the first edge's or the greater, if
    /// a vertex pair is levered at an edge, or if a null edge lends one.
    ///
    /// The vertex-alone arm is pinned here by its number only: no row
    /// takes it through `certify_pair` to a decision, as
    /// `a_fold_certifies_by_its_sine_at_the_least_reach` takes the edge
    /// arm. A fixture for it needs two faces presented at a vertex they
    /// share alone, which `embed/vertex-bisector` and
    /// `embed/corner-pieces` are re-shaping; the row is theirs to add
    /// with that presentation.
    #[test]
    fn the_arm_is_the_faces_reach_the_least_over_what_the_pair_is_near() {
        let o = [0.0; 3];
        let edges = [
            (1, Some((o, [1.0, 0.0, 0.0])), [1, 2]),
            (2, Some((o, [0.0, 0.0, 10.0])), [1, 3]),
            (9, None, [1, 1]),
        ];
        let x = boundary(
            &edges,
            &[(1, o), (2, [1.0, 0.0, 0.0]), (3, [0.0, 0.0, 10.0])],
        );
        let y = boundary(
            &edges,
            &[
                (1, o),
                (2, [1.0, 0.0, 0.0]),
                (3, [0.0, 0.0, 10.0]),
                (4, [0.0, 20.0, 0.0]),
            ],
        );
        let s = floor();
        let local = Local::default();
        fn piece<'a>(
            face: u64,
            surface: &'a Surface<Interval>,
            boundary: &'a Boundary,
            local: &'a Local,
        ) -> Piece<'a> {
            Piece {
                face: key(face),
                surface,
                whole: false,
                boundary,
                u: (0.0, 1.0),
                v: (0.0, 1.0),
                local,
                ball: Ball::holding(p3(0.0, 0.0, 0.0), p3(0.0, 0.0, 0.0)),
            }
        }
        let read = |other: (u64, &Boundary), edges: &[usize], vertices: &[u64]| {
            let near = Near {
                edges: edges.iter().map(|&i| (i, i)).collect(),
                vertices: vertices.iter().map(|&v| key(v)).collect(),
            };
            arm_of(
                &piece(1, &s, &x, &local),
                &piece(other.0, &s, other.1, &local),
                &near,
            )
            .map(|arm| arm.length().hi())
        };
        let close = |got: Option<f64>, want: f64, what: &str| {
            let got = got.unwrap_or_else(|| panic!("{what}: an arm"));
            assert!((got - want).abs() < 1e-12, "{what}: {got}, not {want}");
        };
        close(read((2, &y), &[0], &[]), 400.25_f64.sqrt(), "edge 1");
        close(read((2, &y), &[1], &[]), 425.0_f64.sqrt(), "edge 2");
        close(
            read((2, &y), &[0, 1], &[1]),
            400.25_f64.sqrt(),
            "both edges",
        );
        close(
            read((2, &y), &[1, 0], &[1]),
            400.25_f64.sqrt(),
            "both edges, the other way round",
        );
        close(read((2, &y), &[], &[1]), 20.0, "the vertex alone");
        close(
            read((1, &y), &[1], &[]),
            26.0_f64.sqrt(),
            "one face against itself",
        );
        assert_eq!(read((2, &y), &[0, 2], &[]), None, "a null edge");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod margin_reach_rows {
    //! Where the departure's arm is longer than the run it stands for,
    //! pinned as the code reads it today, and the arm felt in a decision.
    use super::margin_rows::{along, band, iv0, key, on, p3, v3};
    use super::presentation_rows::arm;
    use super::*;
    use geom_core::test_support::fixture_lever;

    /// An array into the shared [`v3`].
    fn va(v: [f64; 3]) -> Vec3<Interval> {
        v3(v[0], v[1], v[2])
    }

    /// An array into the shared [`p3`].
    fn pa(v: [f64; 3]) -> Point3<Interval> {
        p3(v[0], v[1], v[2])
    }

    /// A line edge from `a` to `b`, used once, forward.
    fn line(k: u64, ends: [u64; 2], a: [f64; 3], b: [f64; 3]) -> Rim {
        Rim {
            key: key(k),
            arc: Some(chord_segment(pa(a), pa(b))),
            stated: None,
            ends: ends.map(key),
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                curve: None,
            }],
        }
    }

    fn boundary(edges: Vec<Rim>, vertices: &[(u64, [f64; 3])]) -> Boundary {
        Boundary {
            sense: true,
            edges,
            vertices: vertices.iter().map(|&(k, p)| (key(k), pa(p))).collect(),
            extent: None,
            outer: None,
        }
    }

    fn piece<'a>(
        face: u64,
        surface: &'a Surface<Interval>,
        boundary: &'a Boundary,
        local: &'a Local,
        (u, v): ((f64, f64), (f64, f64)),
    ) -> Piece<'a> {
        Piece {
            face: key(face),
            surface,
            whole: false,
            boundary,
            u,
            v,
            local,
            ball: Ball::holding(pa([0.0; 3]), pa([0.0; 3])),
        }
    }

    /// The plane through the `x` axis whose normal is `(0, −sin φ, cos φ)`:
    /// the floor `z = 0` folded up by `φ` about that axis.
    fn folded(phi: f64) -> Surface<Interval> {
        Surface::Plane {
            frame: geom::Frame::new(
                pa([0.0; 3]),
                va([0.0, -phi.sin(), phi.cos()]),
                va([1.0, 0.0, 0.0]),
            ),
        }
    }

    fn floor() -> Surface<Interval> {
        folded(0.0)
    }

    /// **The arm counts a strip's run along its edge**
    /// (`work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md`,
    /// which owes the fix). Two plane strips 4 along their shared edge
    /// and 0.05 across, folded by nine tenths of the band's zero over
    /// that width, so their far edges part by less than the zero
    /// everywhere. The reach, read from the edge's middle, is 2.0006, the
    /// strips' run along the edge; the sine times it is well past the
    /// band, and the strip's whole width is signed apart.
    ///
    /// This row pins what the code reads, not what it should: a reach
    /// read as the faces' run away from the edge is 0.05, and at it the
    /// strip decides nothing. Red when the lever becomes the narrower
    /// face's run across the edge, read from the verdict
    /// (`work/embed/the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict.md`).
    #[test]
    fn the_arm_counts_a_strips_run_along_its_edge() {
        let phi: f64 = 0.9 * band().zero() / 0.05;
        let (o, e) = ([0.0; 3], [4.0, 0.0, 0.0]);
        let strip = |far: f64, up: f64, ks: [u64; 2]| {
            boundary(
                vec![line(1, [1, 2], o, e)],
                &[
                    (1, o),
                    (2, e),
                    (ks[0], [4.0, far, up]),
                    (ks[1], [0.0, far, up]),
                ],
            )
        };
        let (x, y) = (
            strip(0.05, 0.0, [3, 4]),
            strip(0.05 * phi.cos(), 0.05 * phi.sin(), [5, 6]),
        );
        let s = floor();
        let local = Local::default();
        let cell = ((0.0, 1.0), (0.0, 1.0));
        let near = Near {
            edges: vec![(0, 0)],
            vertices: Vec::new(),
        };
        let arm = arm_of(
            &piece(1, &s, &x, &local, cell),
            &piece(2, &s, &y, &local, cell),
            &near,
        )
        .expect("an arm");
        assert!(
            (arm.length().hi() - 4.002_5_f64.sqrt()).abs() < 1e-9,
            "the reach is the strips' run along the edge: {:?}",
            arm.length()
        );
        assert!(
            0.05 * phi.sin() < band().zero(),
            "the strips never part by the band's zero"
        );
        let psi = Psi::Linear {
            n: va([0.0, -phi.sin(), phi.cos()]),
            q: pa(o),
        };
        let read = |arm: Lever<Interval>| {
            let p = Presented {
                surface: &s,
                map: Map::Chart,
                x: (0.0, 4.0),
                y: (0.0, 0.05),
                zero: Zero::Side(Side::YLo, along(1)),
                corner: false,
                cache: Cache::default(),
            };
            sign_on(&psi, Some(&on(&[1], &[])), &p, None, Some(arm), band())
        };
        assert_eq!(
            read(arm),
            Some(Sign::Negative),
            "signed apart at the run along the edge"
        );
        assert_eq!(
            read(fixture_lever(iv0(0.05))),
            None,
            "undecided at the strips' run away from it"
        );
    }

    /// **The reach alone signs an edge tier 3's wedge check classes
    /// smooth**
    /// (`work/embed/the-departure-is-levered-at-the-reach-alone-where-check-4-folds-the-faces-curvature-arms.md`).
    /// A cylinder of radius `1e-3` and a plane through its ruling that it
    /// leaves at the sine `s`, both levered at one metre:
    /// - at `s` fifty times the band's escalation both read a corner;
    /// - at `s` five hundred times the band's zero, check 4's folded arm
    ///   (the radius) leaves it in band, and the carrier arm signs it;
    /// - at `s` fifty times the band's escalation over a thousand, check
    ///   4 reads the smooth class, and the carrier arm still signs it.
    ///
    /// This row pins the difference as it is. Red when the question
    /// reads the edge's class from tier 3's verdict in place of deciding
    /// it at the reach
    /// (`work/embed/the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict.md`).
    #[test]
    fn the_reach_alone_signs_an_edge_check_4_classes_smooth() {
        let r = 1e-3;
        let read = |s: f64| {
            let cyl = Surface::Cylinder {
                frame: geom::Frame::new(pa([0.0; 3]), va([0.0, 0.0, 1.0]), va([1.0, 0.0, 0.0])),
                radius: iv0(r),
            };
            let n = va([-(1.0 - s * s).sqrt(), s, 0.0]);
            let plane = Surface::Plane {
                frame: geom::Frame::new(pa([r, 0.0, 0.0]), n, va([0.0, 0.0, 1.0])),
            };
            let check4 = match geom_brep::classify_dihedral(
                &cyl,
                &plane,
                pa([r, 0.0, 0.5]),
                fixture_lever(iv0(1.0)),
                band(),
            ) {
                Ok(geom_brep::Dihedral::Corner(_)) => "corner",
                Ok(geom_brep::Dihedral::Tangent(_)) => "smooth",
                Err(_) => "in band",
            };
            let p = Presented {
                surface: &cyl,
                map: Map::Chart,
                x: (0.0, 0.0),
                y: (0.0, 1.0),
                zero: Zero::Side(Side::XLo, along(1)),
                corner: false,
                cache: Cache::default(),
            };
            let psi = Psi::Carrier(Implicit::Plane {
                o: pa([r, 0.0, 0.0]),
                n,
            });
            let engine = sign_on(&psi, None, &p, None, arm(1.0), band());
            (check4, engine)
        };
        let (zero, escalate) = (band().zero(), band().escalate());
        assert_eq!(
            read(50.0 * escalate / r),
            ("corner", Some(Sign::Positive)),
            "a plain corner"
        );
        assert_eq!(
            read(0.5 * (zero + escalate) / r),
            ("in band", Some(Sign::Positive)),
            "check 4 in band at the radius, the engine definite at the reach"
        );
        assert_eq!(
            read(0.05 * zero / r),
            ("smooth", Some(Sign::Positive)),
            "check 4 smooth at the radius, the engine definite at the reach"
        );
    }

    /// **A fold between two faces certifies by its sine times the least
    /// reach, and not by any other arm.** The floor's square beside its
    /// edge on the `x` axis, against a face folded up about that axis by
    /// `φ`. The pair is near that edge (reach 3, the folded face's far
    /// vertex) and a second edge (reach 1.58, the least), and the folded
    /// face runs farther than the floor (the floor's own reach from the
    /// first edge is 1.118).
    /// - `sin φ` times 1.58 at 1.2 times the band: certified;
    /// - at 0.7 times the band: not certified.
    ///
    /// Red if the arm that reaches the decision is doubled, is the first
    /// edge's or the greatest (0.7 becomes 1.33 or 1.4 and certifies), or
    /// is one face's (1.2 becomes 0.85 and does not).
    #[test]
    fn a_fold_certifies_by_its_sine_at_the_least_reach() {
        let least = 2.5_f64.sqrt();
        let certified = |times: f64| {
            let phi = (times * band().escalate() / least).asin();
            let o = [0.0; 3];
            // The two shared edges: the fold's, and a short one whose
            // middle is (0.5, 1.5, 0) on the floor.
            let edges = || {
                vec![
                    line(1, [1, 2], o, [1.0, 0.0, 0.0]),
                    line(2, [7, 8], [0.4, 1.5, 0.0], [0.6, 1.5, 0.0]),
                ]
            };
            let x = boundary(
                edges(),
                &[
                    (1, o),
                    (2, [1.0, 0.0, 0.0]),
                    (3, [1.0, 1.0, 0.0]),
                    (4, [0.0, 1.0, 0.0]),
                ],
            );
            let y = boundary(
                edges(),
                &[
                    (1, o),
                    (2, [1.0, 0.0, 0.0]),
                    (5, [0.5, 3.0 * phi.cos(), 3.0 * phi.sin()]),
                ],
            );
            let (sx, sy) = (floor(), folded(phi));
            let here = Local {
                edges: vec![0],
                vertices: Vec::new(),
            };
            let away = Local::default();
            let near = Near {
                edges: vec![(0, 0), (1, 1)],
                vertices: Vec::new(),
            };
            let cell = ((0.4, 0.6), (0.0, 0.1));
            let arm = arm_of(
                &piece(1, &sx, &x, &here, cell),
                &piece(2, &sy, &y, &away, cell),
                &near,
            )
            .expect("an arm");
            assert!(
                (arm.length().hi() - least).abs() < 1e-9,
                "the least reach: {:?}",
                arm.length()
            );
            let got = certify_pair(
                &piece(1, &sx, &x, &here, cell),
                &piece(2, &sy, &y, &away, cell),
                &near,
                band(),
            );
            matches!(got, Outcome::Certified { .. })
        };
        assert!(certified(1.2), "above the band at the least reach");
        assert!(!certified(0.7), "inside the band at the least reach");
    }

    /// **A sine that changes sign over the rectangle decides nothing.** A
    /// cylinder beside its ruling at `u = 0`, half a radian wide, against
    /// a plane through that ruling it has already crossed a tenth of a
    /// radian in: the sine runs from `−0.1` to `0.39` across the
    /// rectangle. Red if the enclosure is read at its middle.
    #[test]
    fn a_sine_that_changes_sign_over_the_rectangle_decides_nothing() {
        let cyl = Surface::Cylinder {
            frame: geom::Frame::new(pa([0.0; 3]), va([0.0, 0.0, 1.0]), va([1.0, 0.0, 0.0])),
            radius: iv0(1.0),
        };
        let a: f64 = 0.1;
        // The plane through the ruling at (1, 0, ·) whose trace in the
        // cross-section is tangent to the circle at the angle `a`.
        let psi = Psi::Linear {
            n: va([-a.cos(), -a.sin(), 0.0]),
            q: pa([1.0, 0.0, 0.0]),
        };
        let read = |width: f64| {
            let p = Presented {
                surface: &cyl,
                map: Map::Chart,
                x: (0.0, width),
                y: (0.0, 1.0),
                zero: Zero::Side(Side::XLo, along(1)),
                corner: false,
                cache: Cache::default(),
            };
            sign_on(&psi, Some(&on(&[1], &[])), &p, None, arm(1.0), band())
        };
        assert_eq!(
            read(0.5),
            None,
            "the sine changes sign across the rectangle"
        );
        assert!(read(0.05).is_some(), "the control: before the crossing");
    }

    /// **A zero arm and an arm that does not certify decide nothing.**
    /// Two faces each bounded by a closed edge on the one vertex they
    /// share have no other vertex to read a reach from: the arm is zero.
    /// Red if a zero or uncertified arm is replaced by a default.
    #[test]
    fn a_zero_arm_and_an_uncertified_arm_decide_nothing() {
        let o = [0.0; 3];
        let rim = || boundary(Vec::new(), &[(1, o)]);
        let (x, y) = (rim(), rim());
        let s = floor();
        let local = Local::default();
        let cell = ((0.0, 1.0), (0.0, 1.0));
        let near = Near {
            edges: Vec::new(),
            vertices: vec![key(1)],
        };
        let arm = arm_of(
            &piece(1, &s, &x, &local, cell),
            &piece(2, &s, &y, &local, cell),
            &near,
        )
        .expect("an arm, of no length");
        assert_eq!(arm.length().hi(), 0.0, "two rims on one vertex");
        let psi = Psi::Linear {
            n: va([0.0, 1.0, 0.0]),
            q: pa(o),
        };
        let read = |arm: Lever<Interval>| {
            let p = Presented {
                surface: &s,
                map: Map::Chart,
                x: (0.0, 1.0),
                y: (0.0, 1.0),
                zero: Zero::Side(Side::YLo, along(1)),
                corner: false,
                cache: Cache::default(),
            };
            sign_on(&psi, Some(&on(&[1], &[])), &p, None, Some(arm), band())
        };
        assert!(read(fixture_lever(iv0(1.0))).is_some(), "the control");
        assert_eq!(read(arm), None, "a zero arm");
        assert_eq!(
            read(fixture_lever(crate::test_support_fixtures::trv(1.0))),
            None,
            "an arm that does not certify"
        );
    }
}

/// **A cone's chart reads a section ellipse's focal image**: the image's
/// enclosures against the stored row's own evaluator, the guards on what
/// builds one and where a cone's chart is read, and the cells beside a
/// section arc and at the corners it makes with a ruling and a rim.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod focal_rows {
    use super::*;
    use geom_brep::{FocalImage, Pcurve};

    fn iv0(x: f64) -> Interval {
        Interval::from_bounds(x, x)
    }
    fn p2((u, v): (f64, f64)) -> Point2<Interval> {
        Point2::new(iv0(u), iv0(v))
    }
    fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).unwrap()
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

    const ALPHA: f64 = 0.5;

    /// The cone of half-angle [`ALPHA`] about `z`, its apex the origin.
    fn cone() -> Surface<Interval> {
        Surface::Cone {
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            half_angle: iv0(ALPHA),
        }
    }

    /// The cone's point at the chart point `(u, v)`.
    fn on_cone(u: f64, v: f64) -> Point3<Interval> {
        let (s, c) = ALPHA.sin_cos();
        Point3::new(iv0(v * s * u.cos()), iv0(v * s * u.sin()), iv0(v * c))
    }

    /// **The section of [`cone`] by the plane `n·p = 1`**, `n` the axis
    /// tilted `0.3` toward the azimuth `u0`, as its stored row: the
    /// ellipse's vertex nearer the axis is at `u0`, where `E = 0`.
    fn section(u0: f64, with: bool) -> FocalImage<f64> {
        let (a, b) = (ALPHA.cos() * 0.3f64.cos(), ALPHA.sin() * 0.3f64.sin());
        let e = b / a;
        let v0 = 1.0 / (a * (1.0 - e * e));
        FocalImage {
            u0,
            t0: 0.0,
            v0,
            va: -e * v0,
            vb: 0.0,
            vl: 0.0,
            beta: e / (1.0 + (1.0 - e * e).sqrt()),
            sense: if with { 1.0 } else { -1.0 },
        }
    }

    /// The row's chart point at `t`, by the stored row's own evaluator.
    fn stored(image: &FocalImage<f64>, t: f64) -> (f64, f64) {
        let q = Pcurve::FocalSection(*image).eval(t);
        (q.x, q.y)
    }

    /// The row's parameter where its azimuth is `u0 ± nu`.
    fn anomaly_of(image: &FocalImage<f64>, nu: f64) -> f64 {
        let b = image.beta;
        2.0 * (((1.0 - b) / (1.0 + b)) * (nu / 2.0).tan()).atan()
    }

    /// The image lifted, each number widened by `w`.
    fn lifted(image: &FocalImage<f64>, w: f64) -> FocalImage<Interval> {
        let x = |v: f64| Interval::from_bounds(v - w, v + w);
        FocalImage {
            u0: x(image.u0),
            t0: iv0(image.t0),
            v0: x(image.v0),
            va: x(image.va),
            vb: x(image.vb),
            vl: iv0(image.vl),
            beta: x(image.beta),
            sense: iv0(image.sense),
        }
    }

    fn focal(image: &FocalImage<f64>) -> Focal {
        Focal::of(&lifted(image, 0.0)).expect("the section's image builds")
    }

    /// **The enclosures hold the stored row, member by member, at any
    /// width of the parameter**, and at a point image the azimuth's is no
    /// wider than the row's own advance. Each of `β`, `u0` and the second
    /// channel's constant and cosine terms is widened by `1e-3`, wider
    /// than the narrow pieces read (`1e-4`; the others are `1.5` and
    /// `7.0`), so the two end evaluations of a narrow piece overlap; `t0`
    /// is a point here, and bracketed in
    /// `a_focal_images_enclosures_hold_random_members_of_random_brackets`.
    /// Read for both senses, a `β` of either sign, and `β = 1 − 1e-9`.
    /// Red if the azimuth's hull takes the wrong end of an end
    /// evaluation, drops the sense, or the rate misses the derivative.
    #[test]
    fn a_focal_images_enclosures_hold_its_stored_row_member_by_member() {
        let near_parabolic = FocalImage {
            beta: 1.0 - 1e-9,
            ..section(0.4, true)
        };
        let reversed = FocalImage {
            beta: -section(0.0, true).beta,
            ..section(2.0, false)
        };
        for (what, image, w) in [
            ("with the frame", section(0.4, true), 1e-3),
            ("against the frame", section(5.9, false), 1e-3),
            ("a negative β against the frame", reversed, 1e-3),
            ("a point image", section(0.4, true), 0.0),
            ("near-parabolic", near_parabolic, 0.0),
        ] {
            let f = Focal::of(&lifted(&image, w)).expect("builds");
            let members: Vec<FocalImage<f64>> = [-1.0, 0.0, 1.0]
                .into_iter()
                .flat_map(|sb| {
                    [-1.0, 1.0].into_iter().map(move |su| FocalImage {
                        beta: image.beta + sb * w,
                        u0: image.u0 + su * w,
                        v0: image.v0 + su * w,
                        va: image.va - sb * w,
                        ..image
                    })
                })
                .collect();
            for (t0, width) in [
                (-2.9, 1e-4),
                (-0.3, 1e-4),
                (0.0, 1.5),
                (1.1, 1e-4),
                (2.0, 7.0),
            ] {
                let t = Interval::from_bounds(t0, t0 + width);
                let (u, v) = (f.on(Axis::U, t), f.on(Axis::V, t));
                let (ru, rv) = (f.rate(Axis::U, t), f.rate(Axis::V, t));
                assert!(u.is_certified() && ru.is_certified(), "{what}: bounded");
                for m in &members {
                    for k in 0..=8 {
                        let s = t0 + width * f64::from(k) / 8.0;
                        let (su, sv) = stored(m, s);
                        assert!(
                            u.lo() <= su && su <= u.hi() && v.lo() <= sv && sv <= v.hi(),
                            "{what}: the row's point at {s} is held: {su} in {u:?}, {sv} in {v:?}"
                        );
                        let h = 1e-6;
                        let (a, b) = (stored(m, s - h), stored(m, s + h));
                        let (du, dv) = ((b.0 - a.0) / (2.0 * h), (b.1 - a.1) / (2.0 * h));
                        let slack = |d: f64| 1e-4 * (1.0 + d.abs());
                        if (t0 + h..=t0 + width - h).contains(&s) {
                            assert!(
                                ru.lo() - slack(du) <= du && du <= ru.hi() + slack(du),
                                "{what}: the azimuth's rate at {s}: {du} in {ru:?}"
                            );
                            assert!(
                                rv.lo() - slack(dv) <= dv && dv <= rv.hi() + slack(dv),
                                "{what}: the slant's rate at {s}: {dv} in {rv:?}"
                            );
                        }
                    }
                }
                if w == 0.0 {
                    let (a, b) = (stored(&image, t0).0, stored(&image, t0 + width).0);
                    assert!(
                        u.hi() - u.lo() <= (b - a).abs() + 1e-9 * (1.0 + a.abs()),
                        "{what}: the azimuth over {width} is its two ends' hull: {u:?}"
                    );
                }
            }
        }
    }

    /// A member of a bracketed image, lifted to points.
    fn point(m: &FocalImage<f64>) -> FocalImage<Interval> {
        lifted(m, 0.0)
    }

    /// **The enclosures hold random members of random brackets** (a
    /// counterexample search: the seed varies and is logged, the count
    /// rides `CAD_FUZZ_EFFORT`). `β` (to `1 − 1e-9`, either sign), `u0`,
    /// `t0` and the slant's constant are each bracketed at widths from
    /// none to wide, both senses, over pieces from `1e-12` to several
    /// turns. Each corner member's own evaluation, at the interval
    /// scalar, and its own derivative meet the enclosure at seventeen
    /// parameters of the piece: one that does not is a member whose true
    /// value the enclosure misses.
    #[test]
    fn a_focal_images_enclosures_hold_random_members_of_random_brackets() {
        let mut rng = test_utils::fuzz::start("focal_enclosures_members");
        let (mut asked, mut missed) = (0_u64, Vec::new());
        for case in 0..test_utils::fuzz::scaled(400) {
            let reach = [0.3, 0.9, 0.999, 1.0 - 1e-9][case % 4];
            let beta = rng.range(-reach, reach);
            let wb = [0.0, 1e-12, 1e-6, 1e-3, 0.05][case % 5].min((1.0 - beta.abs()) * 0.5);
            let wu = [0.0, 1e-9, 1e-3, 0.5, 3.0][(case / 5) % 5];
            let wt = [0.0, 1e-9, 1e-3, 0.3][(case / 25) % 4];
            let centre = FocalImage {
                u0: rng.range(-20.0, 20.0),
                t0: rng.range(-10.0, 10.0),
                v0: rng.range(0.5, 3.0),
                va: rng.range(-1.0, 1.0),
                vb: rng.range(-1.0, 1.0),
                vl: 0.0,
                beta,
                sense: if rng.unit() < 0.5 { 1.0 } else { -1.0 },
            };
            let x = |v: f64, w: f64| Interval::from_bounds(v - w, v + w);
            let bracket = FocalImage {
                u0: x(centre.u0, wu),
                t0: x(centre.t0, wt),
                v0: x(centre.v0, wu * 0.1),
                beta: x(centre.beta, wb),
                ..point(&centre)
            };
            let Some(f) = Focal::of(&bracket) else {
                continue;
            };
            let lo = rng.range(-60.0, 60.0);
            let width = [1e-12, 1e-6, 1e-3, 0.5, 3.0, TURN, 7.0, 40.0][case % 8];
            let t = Interval::from_bounds(lo, lo + width);
            let (u, v) = (f.on(Axis::U, t), f.on(Axis::V, t));
            let ru = f.rate(Axis::U, t);
            assert!(
                u.is_certified() && v.is_certified() && ru.is_certified(),
                "case {case}: bounded ({})",
                test_utils::fuzz::replay()
            );
            let apart = |a: Interval, b: Interval| a.hi() < b.lo() || a.lo() > b.hi();
            for (sb, su, st) in [
                (-1.0, -1.0, -1.0),
                (-1.0, 1.0, 1.0),
                (0.0, -1.0, 1.0),
                (0.0, 1.0, -1.0),
                (1.0, -1.0, -1.0),
                (1.0, 1.0, 1.0),
                (1.0, -1.0, 1.0),
                (-1.0, 1.0, -1.0),
            ] {
                let m = FocalImage {
                    beta: centre.beta + sb * wb,
                    u0: centre.u0 + su * wu,
                    t0: centre.t0 + st * wt,
                    v0: centre.v0 + su * wu * 0.1,
                    ..centre
                };
                for k in 0..=16 {
                    let s = (lo + width * f64::from(k) / 16.0).min(lo + width);
                    asked += 1;
                    let e = Pcurve::FocalSection(point(&m)).eval(iv0(s));
                    let c = (iv0(s) - iv0(m.t0)).sin_cos().1;
                    let b = iv0(m.beta);
                    let d = (iv0(1.0) - b.powi(2)) / (iv0(1.0) + b.powi(2) - b * c * iv0(2.0))
                        * iv0(m.sense);
                    if apart(e.x, u) || apart(e.y, v) || apart(d, ru) {
                        missed.push(format!(
                            "case {case} member {m:?} at {s}: {e:?}, rate {d:?}, \
                             against {u:?} {v:?} {ru:?}"
                        ));
                    }
                }
            }
        }
        assert!(
            missed.is_empty() && asked > 0,
            "{} of {asked} missed, first {:#?} ({})",
            missed.len(),
            &missed[..missed.len().min(3)],
            test_utils::fuzz::replay()
        );
    }

    /// **A random arc's image holds the arc at a fine grid of its
    /// parameters** (a counterexample search, as the row above): the
    /// extent a face is given ([`ChartCurve::at`] over a whole span),
    /// over spans from nothing to several turns, both senses.
    #[test]
    fn a_focal_images_enclosure_over_a_random_arc_holds_the_arc() {
        let mut rng = test_utils::fuzz::start("focal_enclosures_arcs");
        let mut missed = Vec::new();
        for case in 0..test_utils::fuzz::scaled(200) {
            let image = FocalImage {
                u0: rng.range(-7.0, 7.0),
                t0: rng.range(-4.0, 4.0),
                v0: rng.range(0.5, 3.0),
                va: rng.range(-1.0, 1.0),
                vb: rng.range(-1.0, 1.0),
                vl: 0.0,
                beta: rng.range(-0.95, 0.95),
                sense: if case % 2 == 0 { 1.0 } else { -1.0 },
            };
            let (lo, w) = (rng.range(-10.0, 10.0), rng.range(0.0, 15.0));
            let [u, v] = ChartCurve::Focal(focal(&image)).at(Interval::from_bounds(lo, lo + w));
            for k in 0..=400 {
                let s = (lo + w * f64::from(k) / 400.0).min(lo + w);
                let e = Pcurve::FocalSection(point(&image)).eval(iv0(s));
                if e.x.hi() < u.lo() || e.x.lo() > u.hi() || e.y.hi() < v.lo() || e.y.lo() > v.hi()
                {
                    missed.push(format!(
                        "case {case} {image:?} at {s}: {e:?} against {u:?} {v:?}"
                    ));
                }
            }
        }
        assert!(
            missed.is_empty(),
            "{} missed, first {:#?} ({})",
            missed.len(),
            &missed[..missed.len().min(3)],
            test_utils::fuzz::replay()
        );
    }

    /// **The image over a whole arc holds the arc: both its ends, entire,
    /// and its slant's extreme between them.** A face's extent is read
    /// off this enclosure (`clearance`'s `row_of`, which says why it must
    /// hold the arc and is pinned at its own row). The arc runs through
    /// the section's far vertex
    /// (`E = π`), where the slant is greatest and neither end is; the
    /// enclosure holds the stored row there, and holds each end's own
    /// evaluation whole, by the row's evaluator at the interval scalar.
    /// Red if the slant is read at the span's two ends alone, or the
    /// azimuth's hull is drawn inside an end evaluation.
    #[test]
    fn a_focal_images_enclosure_over_an_arc_holds_its_ends_and_its_slants_extreme() {
        use core::f64::consts::PI;
        for (what, image) in [
            ("with the frame", section(0.4, true)),
            ("against the frame", section(5.9, false)),
        ] {
            let (t0, t1) = (PI - 1.2, PI + 0.9);
            let lifted = lifted(&image, 0.0);
            let [u, v] =
                ChartCurve::Focal(Focal::of(&lifted).unwrap()).at(Interval::from_bounds(t0, t1));
            let (far_u, far_v) = stored(&image, PI);
            let ends = [stored(&image, t0), stored(&image, t1)];
            assert!(
                far_v > ends[0].1.max(ends[1].1) + 0.01,
                "{what}: the slant's extreme is interior: {far_v} against {ends:?}"
            );
            assert!(
                v.lo() <= far_v && far_v <= v.hi() && u.lo() <= far_u && far_u <= u.hi(),
                "{what}: the far vertex ({far_u}, {far_v}) is held: {u:?} {v:?}"
            );
            for t in [t0, t1] {
                let end = Pcurve::FocalSection(lifted).eval(iv0(t));
                assert!(
                    u.lo() <= end.x.lo()
                        && end.x.hi() <= u.hi()
                        && v.lo() <= end.y.lo()
                        && end.y.hi() <= v.hi(),
                    "{what}: the end at {t} is held whole: {end:?} in {u:?} {v:?}"
                );
            }
        }
    }

    /// **A focal image builds only where its azimuth is monotone for
    /// every member**: a `β` whose enclosure touches `±1`, or a sense
    /// that is not exactly `±1`, builds none; `β = 1 − 1e-9` builds. Red
    /// if [`Focal::of`] admits a bracket that holds a parabola's `β`.
    #[test]
    fn a_focal_image_builds_only_with_its_kepler_parameter_strictly_inside_one() {
        let image = section(0.4, true);
        let with = |beta: Interval, sense: Interval| {
            Focal::of(&FocalImage {
                beta,
                sense,
                ..lifted(&image, 0.0)
            })
            .is_some()
        };
        let one = iv0(1.0);
        assert!(with(iv0(1.0 - 1e-9), one), "near-parabolic builds");
        assert!(with(iv0(-(1.0 - 1e-9)), iv0(-1.0)), "and against the frame");
        for (what, beta) in [
            ("a bracket reaching 1", Interval::from_bounds(0.9, 1.0)),
            ("a bracket past 1", Interval::from_bounds(0.9, 1.1)),
            ("the point 1", iv0(1.0)),
            ("a bracket reaching −1", Interval::from_bounds(-1.0, -0.9)),
            ("an unbounded β", whole_line()),
        ] {
            assert!(!with(beta, one), "{what} builds no image");
        }
        for (what, sense) in [
            ("a sense of ½", iv0(0.5)),
            ("a sense of 0", iv0(0.0)),
            (
                "a bracket about 1",
                Interval::from_bounds(1.0 - 1e-12, 1.0 + 1e-12),
            ),
        ] {
            assert!(!with(iv0(image.beta), sense), "{what} builds no image");
        }
    }

    /// **One period of the parameter winds the azimuth one turn, by the
    /// image's sense, and returns the slant**: [`ChartCurve::wind`]
    /// against the stored row's own advance over a period. Red if the
    /// turn count a closed section is continued by is not the row's.
    #[test]
    fn a_closed_sections_period_winds_its_azimuth_once_by_its_sense() {
        for with in [true, false] {
            let image = section(1.3, with);
            let c = ChartCurve::Focal(focal(&image));
            let (a, b) = (stored(&image, -1.0), stored(&image, -1.0 + TURN));
            let turns = ((b.0 - a.0) / TURN).round();
            assert!((b.0 - a.0 - turns * TURN).abs() < 1e-12 && (b.1 - a.1).abs() < 1e-12);
            #[allow(clippy::cast_possible_truncation)]
            let turns = turns as i64;
            assert_eq!(turns, if with { 1 } else { -1 });
            assert_eq!(c.wind(Axis::U), Some(turns), "the azimuth's turns");
            assert_eq!(c.wind(Axis::V), Some(0), "the slant returns");
        }
        let drifting = Focal::of(&FocalImage {
            vl: iv0(1.0),
            ..lifted(&section(1.3, true), 0.0)
        })
        .unwrap();
        assert_eq!(
            ChartCurve::Focal(drifting).wind(Axis::V),
            None,
            "a slant that does not return is not continued"
        );
    }

    /// A cone face's edge of one use whose chart image is `image` over
    /// `t`, from vertex `ends[0]` to `ends[1]`; its carrier a stand-in
    /// out of every cell's reach, as the cylinder rows' is.
    fn section_rim(n: u64, ends: [u64; 2], image: &FocalImage<f64>, t: (f64, f64)) -> Rim {
        Rim {
            stated: None,
            key: edge(n),
            arc: Some((
                Curve3::line(
                    Point3::new(iv0(10.0), iv0(10.0), iv0(10.0)),
                    geom_core::UnitVec3::exact_x(),
                ),
                (iv0(t.0), iv0(t.1)),
            )),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus: true,
                image: None,
                iso: None,
                curve: Some(ChartCurve::Focal(focal(image))),
            }],
        }
    }

    /// A coordinate-line edge of [`cone`] its loop runs from the chart
    /// point `a` (vertex `ends[0]`) to `b`: a ruling where `held` is `U`,
    /// a rim's arc where it is `V`. Its carrier runs toward the greater
    /// moving coordinate, so the loop runs it forward only where `b`'s is.
    fn line_rim(n: u64, ends: [u64; 2], held: Axis, a: (f64, f64), b: (f64, f64)) -> Rim {
        let (s, c) = ALPHA.sin_cos();
        let plus = match held {
            Axis::U => b.1 > a.1,
            Axis::V => b.0 > a.0,
        };
        let (lo, hi, ends) = if plus {
            (a, b, ends)
        } else {
            (b, a, [ends[1], ends[0]])
        };
        let (carrier, span) = match held {
            Axis::U => (
                Curve3::line(
                    on_cone(lo.0, lo.1),
                    geom_core::UnitVec3::checked(Vec3::new(
                        iv0(s * lo.0.cos()),
                        iv0(s * lo.0.sin()),
                        iv0(c),
                    )),
                ),
                (iv0(0.0), iv0(hi.1 - lo.1)),
            ),
            Axis::V => (
                Curve3::circle(
                    Point3::new(iv0(0.0), iv0(0.0), iv0(lo.1 * c)),
                    Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                    iv0(lo.1 * s),
                    Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
                ),
                (iv0(lo.0), iv0(hi.0)),
            ),
        };
        Rim {
            stated: None,
            key: edge(n),
            arc: Some((carrier, span)),
            ends: ends.map(vertex),
            uses: vec![Use {
                plus,
                image: Some([p2(lo), p2(hi)]),
                iso: Some(Iso {
                    held,
                    at: iv0(match held {
                        Axis::U => a.0,
                        Axis::V => a.1,
                    }),
                    full: false,
                }),
                curve: None,
            }],
        }
    }

    /// **A corner of a cone face where a ruling meets a section arc**, at
    /// the section's point of azimuth `1`: the ruling `u = 1` (edge 0)
    /// runs down the wall to the vertex 1, and the arc (edge 1) leaves it
    /// toward `nu_far`, an azimuth past the section's near vertex. With
    /// `nu_far > 1` the face is at `u > 1`, beyond the arc from the apex,
    /// and the corner is convex; with `nu_far < 1` the arc leaves into the
    /// ruling's other side.
    fn ruling_corner(u0: f64, nu_far: f64) -> (Boundary, (f64, f64)) {
        gapped_corner(u0, nu_far, 0.0)
    }

    /// [`ruling_corner`] with the arc's image `gap` radians on in azimuth
    /// from the ruling and the vertex: the two edges' chart points at the
    /// vertex are that far apart.
    fn gapped_corner(u0: f64, nu_far: f64, gap: f64) -> (Boundary, (f64, f64)) {
        let image = section(u0, true);
        let moved = FocalImage {
            u0: u0 + gap,
            ..image
        };
        let (ta, tb) = (anomaly_of(&image, 1.0), anomaly_of(&image, nu_far));
        let (corner, far) = (stored(&image, ta), stored(&image, tb));
        let top = (corner.0, corner.1 + 1.0);
        let b = Boundary {
            sense: true,
            edges: vec![
                line_rim(10, [2, 1], Axis::U, top, corner),
                section_rim(11, [1, 3], &moved, (ta, tb)),
            ],
            vertices: BTreeMap::from([
                (vertex(1), on_cone(corner.0, corner.1)),
                (vertex(2), on_cone(top.0, top.1)),
                (vertex(3), on_cone(far.0, far.1)),
            ]),
            extent: Some([(corner.0, corner.0 + 1.5), (corner.1 - 0.2, corner.1 + 1.0)]),
            outer: None,
        };
        (b, corner)
    }

    /// [`present_chart`] of `b` in the cell `u × v`, edges `near` shared,
    /// with the corner's vertex 1 near where `at_vertex`.
    fn on_wall(
        b: &Boundary,
        near: &[usize],
        at_vertex: bool,
        u: (f64, f64),
        v: (f64, f64),
    ) -> Read {
        let s = cone();
        let local = Local {
            edges: (0..b.edges.len()).collect(),
            vertices: if at_vertex {
                vec![vertex(1)]
            } else {
                Vec::new()
            },
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
        let vertices = if at_vertex {
            vec![vertex(1)]
        } else {
            Vec::new()
        };
        match present_chart(&piece, near, &vertices, band()) {
            Ok(Presentation::Vacuous) => Read::Vacuous,
            Ok(Presentation::Of(q)) => Read::Of(Zeroed::of(&q.zero), q.corner),
            Err(Unpresented::Local) => Read::Local,
            Err(Unpresented::Structural) => Read::Structural,
            Err(Unpresented::Collapsed(_)) => Read::Collapsed,
        }
    }

    /// What a presentation came to: for one presented, its zero and
    /// whether it was read through a corner.
    #[derive(Debug, PartialEq)]
    enum Read {
        Vacuous,
        Of(Zeroed, bool),
        Local,
        Structural,
        Collapsed,
    }

    /// A presentation's zero and the side of it the face is on: the
    /// rectangle's side the zero is, so `XLo` and `YLo` have the face
    /// above the zero on that axis.
    #[derive(Debug, PartialEq)]
    enum Zeroed {
        /// A curve across the rectangle.
        Curve(Side),
        /// One side of the rectangle, a coordinate line.
        Line(Side),
        Other,
    }

    impl Zeroed {
        fn of(zero: &Zero) -> Self {
            match zero {
                Zero::Curve(side, _) => Self::Curve(*side),
                Zero::Side(side, _) => Self::Line(*side),
                _ => Self::Other,
            }
        }
    }

    /// **A ruling's corner with a section arc on a cone is read by the
    /// ruling where the arc leaves into the face** ([`reading`]'s line
    /// arm, [`present_line_corner`]), in the stored sheet and with the
    /// corner a turn on; a reflex corner is read by none. The zero is
    /// the ruling, the rectangle's low side in `u`: the face is at the
    /// greater azimuths. Red where a cone has no line reading (the corner
    /// is `Structural`: two halves of a cut cone refuse at depth 0), if
    /// [`leaves_line`] admits a decided `Negative`, or if the face is
    /// presented on the ruling's other side.
    #[test]
    fn a_rulings_corner_with_a_section_arc_on_a_cone_is_read_by_the_ruling() {
        for u0 in [0.0, TURN - 1.0 - 1e-3, -TURN] {
            let (convex, (cu, cv)) = ruling_corner(u0, 2.0);
            let cell = ((cu - 0.01, cu + 0.03), (cv - 0.02, cv + 0.03));
            assert_eq!(
                on_wall(&convex, &[0], true, cell.0, cell.1),
                Read::Of(Zeroed::Line(Side::XLo), true),
                "the convex corner at u0 = {u0} is presented by the ruling"
            );
            let (reflex, _) = ruling_corner(u0, 0.4);
            let mut reflex = reflex;
            reflex.extent = Some([(cu - 1.5, cu), (cv - 0.3, cv + 1.0)]);
            assert_eq!(
                on_wall(&reflex, &[0], true, cell.0, cell.1),
                Read::Structural,
                "the reflex corner at u0 = {u0} is read by none"
            );
        }
    }

    /// **The same corner with the section arc shared is read by the arc,
    /// through the vertex** ([`Reading::Curve`], [`present_curved`]), and
    /// a cell beside the arc alone by the arc's graph. The face is beyond
    /// the arc from the apex, so the zero is the curve with the face above
    /// it in `v` (`YLo`), and the rectangle is the cell widened to hold
    /// the arc over its columns. The rim's corner, its arc
    /// shared and run the other way along the loop, has the face above
    /// its arc too. Red where a focal image is no curve
    /// [`present_curved`] reads on a cone, or if the face is presented on
    /// the arc's other side.
    #[test]
    fn a_cone_cell_beside_a_section_arc_is_presented_by_the_arcs_graph() {
        let (b, (cu, cv)) = ruling_corner(0.0, 2.0);
        let cell = ((cu - 0.01, cu + 0.03), (cv - 0.02, cv + 0.03));
        assert_eq!(
            on_wall(&b, &[1], true, cell.0, cell.1),
            Read::Of(Zeroed::Curve(Side::YLo), true),
            "the corner, the arc shared"
        );
        let image = section(0.0, true);
        let (mu, mv) = stored(&image, anomaly_of(&image, 1.5));
        let mut alone = b.clone();
        alone.vertices.remove(&vertex(1));
        let got = {
            let s = cone();
            let local = Local {
                edges: vec![1],
                vertices: Vec::new(),
            };
            let piece = Piece {
                face: FaceKey::default(),
                surface: &s,
                whole: false,
                boundary: &b,
                u: (mu - 0.02, mu + 0.02),
                v: (mv - 0.02, mv + 0.02),
                local: &local,
                ball: Ball::holding(origin(), origin()),
            };
            let Ok(Presentation::Of(q)) = present_chart(&piece, &[1], &[], band()) else {
                panic!("a cell beside the arc alone is presented")
            };
            (Zeroed::of(&q.zero), q.corner, q.x, q.y)
        };
        assert_eq!(
            (&got.0, got.1),
            (&Zeroed::Curve(Side::YLo), false),
            "a cell beside the arc alone is presented by its graph, the face above it"
        );
        let (x, y) = (got.2, got.3);
        let arc = [x.0, x.1].map(|u| stored(&image, anomaly_of(&image, u)).1);
        assert!(
            x.0 <= mu - 0.02 && mu + 0.02 <= x.1 && x.1 - x.0 < 0.1,
            "the rectangle's columns are the cell's: {x:?}"
        );
        assert!(
            y.0 <= (mv - 0.02).min(arc[0]) && y.1 >= (mv + 0.02).max(arc[1]) && y.1 - y.0 < 0.1,
            "the rectangle holds the cell and the arc over its columns ({arc:?}): {y:?}"
        );
        let (rim, corner) = rim_corner();
        assert_eq!(
            on_wall(
                &rim,
                &[1],
                true,
                (corner.0 - 0.03, corner.0 + 0.01),
                (corner.1 - 0.01, corner.1 + 0.005)
            ),
            Read::Of(Zeroed::Curve(Side::YLo), true),
            "the rim's corner, the arc shared"
        );
    }

    /// **A closed section is read through its join, a turn on from its
    /// start**: the whole ellipse as one edge over `E ∈ [−π, π]`, its
    /// vertex at the section's far point, in a cell about that point on
    /// the sheet the span's end lands on and on the one a turn back. The
    /// curve continues past its span by its closed form, its azimuth a
    /// turn on by its sense ([`ChartCurve::wind`]). Red if the join is
    /// continued by any other turn count: the run stops at the vertex,
    /// inside the columns.
    ///
    /// The join is an ε concession (the module docs' list): the span's
    /// end, the rounded period, may stop an ulp of `τ` short of the
    /// vertex in the parameter. Here that is under `2e-15` m: the curve's
    /// speed is no more than `|v|·sin α·(1 + |β|)/(1 − |β|)` along the
    /// azimuth plus `|va|` along the slant, about `1.03` m per unit of
    /// parameter.
    #[test]
    fn a_closed_section_is_read_through_its_join() {
        use core::f64::consts::PI;
        {
            let image = section(0.4, true);
            let b = image.beta.abs();
            let speed =
                (image.v0 + image.va.abs()) * ALPHA.sin() * (1.0 + b) / (1.0 - b) + image.va.abs();
            let gap = TURN * f64::EPSILON * speed;
            assert!(
                speed < 1.04 && gap < 2e-15,
                "the join's gap in metres: {gap} at speed {speed}"
            );
        }
        for with in [true, false] {
            let image = section(0.4, with);
            let (ju, jv) = stored(&image, PI);
            let mut rim = section_rim(11, [1, 1], &image, (-PI, PI));
            rim.uses[0].plus = with;
            let b = Boundary {
                sense: true,
                edges: vec![rim],
                vertices: BTreeMap::from([(vertex(1), on_cone(ju, jv))]),
                extent: None,
                outer: None,
            };
            let s = cone();
            let local = Local {
                edges: vec![0],
                vertices: vec![vertex(1)],
            };
            for turn in [0.0, if with { -TURN } else { TURN }] {
                let piece = Piece {
                    face: FaceKey::default(),
                    surface: &s,
                    whole: true,
                    boundary: &b,
                    u: (ju + turn - 0.05, ju + turn + 0.05),
                    v: (jv - 0.05, jv + 0.05),
                    local: &local,
                    ball: Ball::holding(origin(), origin()),
                };
                let got = present_curved(&piece, 0, None, &[0], &[vertex(1)], band());
                // The loop runs the section toward the greater azimuths
                // either way (`plus` follows the sense), its face on its
                // left: beyond the arc from the apex.
                assert!(
                    matches!(
                        &got,
                        Ok(Presentation::Of(q))
                            if Zeroed::of(&q.zero) == Zeroed::Curve(Side::YLo)
                    ),
                    "the join, sense with = {with}, {turn} on: {:?}",
                    got.as_ref()
                        .map(|p| match p {
                            Presentation::Of(q) => format!("{:?}", Zeroed::of(&q.zero)),
                            Presentation::Vacuous => "vacuous".into(),
                        })
                        .map_err(|e| format!("{e:?}"))
                );
            }
        }
    }

    /// **A rim's corner with a section arc is read by the rim**: the rim
    /// `v = c` through the section's point of azimuth `1` runs from the
    /// vertex toward the section's near vertex, the face between it and
    /// the arc: the zero is the rim, the rectangle's high side in `v`.
    /// Red where the line reading takes a ruling only, or if the face is
    /// presented above the rim.
    #[test]
    fn a_rims_corner_with_a_section_arc_on_a_cone_is_read_by_the_rim() {
        let (b, corner) = rim_corner();
        let cell = (
            (corner.0 - 0.03, corner.0 + 0.01),
            (corner.1 - 0.01, corner.1 + 0.005),
        );
        assert_eq!(
            on_wall(&b, &[0], true, cell.0, cell.1),
            Read::Of(Zeroed::Line(Side::YHi), true),
            "the corner is presented by the rim, the face below it"
        );
    }

    /// **A corner of a cone face where a rim's arc meets a section arc**:
    /// the rim `v = c` (edge 0) through the section's point of azimuth
    /// `1`, vertex 1, runs from it toward the section's near vertex, and
    /// the arc (edge 1) comes to it from the azimuth `0.3`. The face is
    /// between them: below the rim, above the arc.
    fn rim_corner() -> (Boundary, (f64, f64)) {
        let image = section(0.0, true);
        let (ta, tb) = (anomaly_of(&image, 0.3), anomaly_of(&image, 1.0));
        let (far, corner) = (stored(&image, ta), stored(&image, tb));
        let end = (0.3, corner.1);
        let b = Boundary {
            sense: true,
            edges: vec![
                line_rim(10, [1, 2], Axis::V, corner, end),
                section_rim(11, [3, 1], &image, (ta, tb)),
            ],
            vertices: BTreeMap::from([
                (vertex(1), on_cone(corner.0, corner.1)),
                (vertex(2), on_cone(end.0, end.1)),
                (vertex(3), on_cone(far.0, far.1)),
            ]),
            extent: Some([(0.3, corner.0), (far.1, corner.1)]),
            outer: None,
        };
        (b, corner)
    }

    /// **No reading reads a rectangle of a cone's chart that holds the
    /// apex or is a turn wide** ([`cone_regular`]): the ruling's corner in
    /// a cell that runs down through `v = 0`, where every azimuth is one
    /// point and the cell's far side of the ruling is no other place, is
    /// left to a smaller cell; the same cell clear of the apex is
    /// presented. Red, presented, with the guard off: the fixture has no
    /// apex vertex for [`only`] to refuse.
    #[test]
    fn a_cone_rectangle_holding_its_apex_is_read_by_no_curved_reading() {
        let (b, (cu, cv)) = ruling_corner(0.0, 2.0);
        let u = (cu - 0.01, cu + 0.03);
        assert_eq!(
            on_wall(&b, &[0], true, u, (0.05, cv + 0.03)),
            Read::Of(Zeroed::Line(Side::XLo), true),
            "clear of the apex"
        );
        for (what, v) in [
            ("through the apex", (-0.05, cv + 0.03)),
            ("down to the apex", (0.0, cv + 0.03)),
        ] {
            assert_eq!(on_wall(&b, &[0], true, u, v), Read::Local, "{what}");
            assert_eq!(
                on_wall(&b, &[1], true, u, v),
                Read::Local,
                "{what}, the arc shared"
            );
        }
        let s = cone();
        assert!(cone_regular(&s, (0.0, 6.0), (0.5, 1.0)));
        assert!(!cone_regular(&s, (0.0, TURN), (0.5, 1.0)), "a turn wide");
        assert!(
            !cone_regular(&s, (0.0, 1.0), (-1.0, 1.0)),
            "across the apex"
        );
        assert!(
            cone_regular(&s, (0.0, 1.0), (-1.0, -0.5)),
            "the other nappe"
        );
    }

    /// **A corner's two edges a gap apart in azimuth are one point only
    /// where the gap is within the band at the rectangle's FAR slant**:
    /// on a cone a chart length along `u` is `|v|·sin α` metres, so over
    /// `v ∈ [1e-10, 1]` a gap of `1e-3` rad is `4.8e-4` m at the far end
    /// and `4.8e-14` m at the near one. [`on_vertex`] is handed the
    /// chart's speed bound over the rectangle ([`speeds`]), an upper
    /// bound, and reads two points. Red if [`speeds`] takes the speed at
    /// the near end, where the vertex concession would read them as one.
    /// This is the helper's row; that each corner reading hands the
    /// rectangle's own bound to the concession is
    /// `a_cones_corner_at_a_gap_is_two_points_at_each_reading_of_a_tall_rectangle`.
    #[test]
    fn a_cones_corner_gap_is_metred_at_the_rectangles_far_slant() {
        let s = cone();
        let (x, y) = ((1.0, 1.001), (1e-10, 1.0));
        let speed = speeds(&s, x, y);
        assert!(
            speed[0] >= ALPHA.sin() && speed[1] >= 1.0,
            "an upper bound over the rectangle: {speed:?}"
        );
        let at = |u: f64| [iv0(u), iv0(0.5)];
        assert!(
            !on_vertex(at(1.0), at(1.001), speed, band()),
            "a gap of 4.8e-4 m at the far slant is two points"
        );
        assert!(
            on_vertex(at(1.0), at(1.0 + 1e-13), speed, band()),
            "a gap within the band at the far slant is one"
        );
    }

    /// **Each corner reading on a cone metres its vertex at the far slant
    /// of the rectangle it reads.** The ruling's corner with the arc's
    /// image a hundred ε of azimuth off the ruling at the vertex, in a
    /// cell whose slant runs from a thousandth of the corner's up past
    /// it: the gap is 55 ε of length at the rectangle's far slant, two
    /// points, and a twentieth of ε at its near one, where [`on_vertex`]
    /// would grant them as one (both asserted, the premise; at the
    /// default ε, `1e-7` rad, `5e-8` m and `5e-11` m). Neither reading presents the cell:
    /// the ruling's ([`present_line_corner`]) and, the arc shared, the
    /// arc's through the vertex ([`present_curved`], [`leaving`]); with no
    /// gap both do, in the same cell. Red, at the reading it is taken in,
    /// if either hands the concession the chart's speed at the
    /// rectangle's near slant.
    #[test]
    fn a_cones_corner_at_a_gap_is_two_points_at_each_reading_of_a_tall_rectangle() {
        let s = cone();
        let gap = 100.0 * geom_core::Tol::witness().eps();
        let (closed, (cu, cv)) = gapped_corner(0.0, 2.0, 0.0);
        let (open, _) = gapped_corner(0.0, 2.0, gap);
        let (u, tall) = ((cu - 0.01, cu + 0.03), (cv * 1e-3, cv + 0.03));
        let (far, near) = (
            speeds(&s, (cu, u.1), tall),
            speeds(&s, (cu, u.1), (tall.0, tall.0)),
        );
        let at = |u: f64| [iv0(u), iv0(cv)];
        assert!(
            !on_vertex(at(cu), at(cu + gap), far, band())
                && on_vertex(at(cu), at(cu + gap), near, band()),
            "the gap is two points at the far slant and one at the near: {far:?} {near:?}"
        );
        for (what, shared, read) in [
            ("the ruling's reading", [0], Zeroed::Line(Side::XLo)),
            ("the arc's reading", [1], Zeroed::Curve(Side::YLo)),
        ] {
            assert_eq!(
                on_wall(&closed, &shared, true, u, tall),
                Read::Of(read, true),
                "{what}: the corner with no gap is presented in the tall cell"
            );
            let got = on_wall(&open, &shared, true, u, tall);
            assert!(
                !matches!(got, Read::Of(..)),
                "{what}: the gapped corner is not one point: {got:?}"
            );
        }
    }

    /// **A focal image on a torus (a Villarceau circle) is read by no
    /// curve reading**: it has no form [`present_curved`] reads, and its
    /// edge is no curve a corner is read by. Admitting it is a decision
    /// (`work/embed/curved-presentation-reads-plane-and-cylinder-images-only.md`).
    #[test]
    fn a_villarceau_row_on_a_torus_is_still_unread() {
        let torus = Surface::Torus {
            frame: geom::Frame::new(
                origin(),
                Vec3::new(iv0(0.0), iv0(0.0), iv0(1.0)),
                Vec3::new(iv0(1.0), iv0(0.0), iv0(0.0)),
            ),
            major_radius: iv0(2.0),
            minor_radius: iv0(0.5),
        };
        let image = FocalImage {
            va: 0.0,
            vl: 1.0,
            ..section(0.0, true)
        };
        let curve = ChartCurve::Focal(focal(&image));
        assert!(Form::of(&torus, curve).is_none(), "no form on a torus");
        assert!(
            Form::of(&cone(), curve).is_some(),
            "the same image on a cone"
        );
        let rim = section_rim(11, [1, 3], &image, (0.0, 1.0));
        assert!(is_curved(&torus, &rim) && !reads_curve(&torus, &rim));
        assert!(reads_curve(&cone(), &rim));
    }
}
