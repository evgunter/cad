//! Certified-conservative boxes for candidate generation (M5 PR 8,
//! C10) and its siblings.
//!
//! **The contract, in one sentence: every box this module returns
//! contains the entity's whole locus, or is the poison box.** A box
//! that is not a superset silently loses whatever its door would
//! otherwise have examined; a poison box overlaps everything, which
//! is the honest answer when no cheap superset is known. No Q1
//! predicate runs on a box; classification is untouched (`reduce`
//! module docs).
//!
//! **The contract holds on every scalar, and the arithmetic that
//! makes it hold is [`Span`]'s.** A description whose coordinates are
//! brackets is ENCLOSED — the axial projection ranges over the whole
//! bracket rather than one endpoint of it, and a reference direction
//! contributes the largest magnitude its bracket admits rather than
//! its upper end's. (Both were live under-enclosures at the
//! `Interval` scalar, issue #862; the `f64` lane was never affected,
//! because there a bracket is a point.) So the sentence above may be
//! cited plainly — `separation`'s door does, and it is the one where
//! a box-level non-overlap is a GRANT.
//!
//! **A box is a SUPERSET, so overlap between two of them is a MAY,
//! not a DOES.** Every door that reads non-overlap as a certificate
//! is entitled to it; no door may read overlap as evidence that two
//! loci meet. The boolean operand gate states that in its refusal
//! text, because it is the door where a reader is most likely to
//! believe otherwise.
//!
//! # Which way LOOSENESS runs is the door's property, not the box's
//!
//! A box bigger than it needs to be is free only where the box
//! PRUNES. That is **four** of the thirteen doors that read a box from
//! here; at five of the other nine, box NON-overlap is the answer being
//! sought, so a bigger box is a REFUSAL, and at the other four it is
//! more exact work AND can be a refusal:
//!
//! - `boolean::reduce`'s C10 tree PRUNES. Loose costs a candidate
//!   pair's worth of exact work and can never change a verdict; a
//!   curved candidate the narrow phase parts is pruned behind it.
//! - `census`'s pre-filter (`census::Trees`) PRUNES on the same
//!   terms: the at-rest sweeps examine the C10 tree's candidates over
//!   these boxes, and a loose box only admits more pairs to the exact
//!   sweeps. The backstop's two classes — its reach boxes over
//!   `face_reach` and its instance extents — read the same direction
//!   through the same door (`census::Candidates::class`); they are
//!   built from `face_reach`, not from these constructors, so the
//!   inventory below does not count them. Pass 2′'s curved-edge class
//!   (`census::Trees::curved`: [`edge_box`] of each curved edge, a
//!   spline's control hull) reads the same direction: a loose box
//!   admits a vertex to `census::curved::may_rest_on`, whose walk passes
//!   a vertex every piece clears with no decision pushed. It costs that
//!   walk, and a refusal only where the walk runs past its budget, which
//!   reads the vertex as one that may rest on the span and hands it to
//!   the reading. `census::collinear_overlaps`, the edge-edge pass's
//!   candidates lifted out for ∖'s reading of coincident edges, boxes
//!   the line edges the same way ([`edge_box`]): a loose box hands the
//!   parallel screen and the overlap more pairs, which they part.
//!   Its every-face class (`census::Trees::every_face`, [`face_box`] of
//!   each face) reads the same direction for the same-solid tangent-line
//!   pass: a loose box hands that reading a pair whose carriers it parts.
//! - `validate`'s check 9 (`ring_pairs`) PRUNES the same way: two
//!   rings of one face whose hulled edge boxes clear are never handed
//!   to the contact arms, and whether two rings meet is decided by those
//!   arms through `Decide`. A bigger box costs a pair the exact arms,
//!   never an answer.
//! - `boolean::reduce`'s operand GATE grants on non-overlap: an
//!   unsupported-kind face whose box clears the other operand cannot
//!   enter a pair, so the operation runs. A bigger box refuses an
//!   operation whose faces never meet, unless the narrow phase behind
//!   the overlap (`boolean::separating`, reaches along directions that
//!   turn with the operands) parts the pair.
//! - the glue door (`boolean::glue`) only PRUNES: a face pair whose
//!   boxes clear is never asked, and whether two faces lie on one
//!   carrier is decided by the carrier ladder through `Decide`, so a
//!   bigger box costs exact work there and decides nothing.
//! - `separation` GRANTS on non-overlap — `Ok(())` IS the
//!   disjointness certificate — so a bigger box refuses a placement
//!   pair that is genuinely separated.
//! - `boolean::ops`'s sphere-extent fallback refuses typed unless the
//!   ball's certified extent CLEARS the face's box (the same
//!   `face_rows` box), so a bigger box
//!   turns a separated sphere × approximated-face pair into
//!   `CurvedBooleanUnsupported`, and a plane face's boundary-edge box
//!   met by a section circle's box ([`circle_box`], the circle's own
//!   extent) into `FallbackExtentUnsupported` unless the narrow phase
//!   parts the edge and the circle.
//! - `boolean::ops`'s section certificate (`walk_pairs` over the boxes
//!   `face_rows` builds once per face, taken by `section_pairs` on both
//!   paths and by `sphere_faces_apart`, the sphere-extent fallback's
//!   reading of a crossing sphere pair's faces) EXAMINES every pair
//!   whose two face boxes overlap and the narrow phase does not part,
//!   and builds from the overlap the pair's reach, which pivots and levers
//!   its angular margins (`section_cert`'s module docs). A bigger box
//!   sends a separated pair through the exact classification, which
//!   certifies it apart; and it widens the reach, which lengthens the
//!   lever, so a nearly parallel pair decides its tilt `Zero` less
//!   readily — onto the exact tilted arm where there is one, and into
//!   a REFUSAL on reach where there is not (a torus pair with a
//!   near-parallel wall). A parallel reading that survives a bigger box
//!   is bounded by the band over a region at least as large as the
//!   section's. A box TIGHTER than its face is the unsound direction:
//!   its reach could miss the section.
//! - `census`'s arm 2 clears an instance pair at its gate on a
//!   definitely negative margin against a CONTAINING box and sends
//!   every other pair to the material test, so over-width would cost
//!   a genuinely-outside instance a point-in-solid probe of its
//!   vertices. On planar-only pairs the reach box IS the vertex hull,
//!   so the cost is not reached there; a reach box a curved face
//!   inflates reaches it once arm 1 has proved that face's pairs apart
//!   (a box, direction or depth gap), and costs probes, never a wrong
//!   clear (`bool4r1_probes::probe_d`).
//! - `census::depth_gap`, arm 1's clear on a gap in depth, GRANTS: it
//!   reads [`face_box_rule`]'s construction and each boundary edge's
//!   carrier, never an extent, and covers each edge with the piece its
//!   carrier names (a chord, the ellipse's ring, the spiric's torus). A cover
//!   that holds more than its edge — no claim where one
//!   exists, a ring wider than the arc — can only raise a face's
//!   greatest depth and lower its least, so a looser reading REFUSES
//!   a pair that is apart and can never clear one that meets. A cover
//!   that holds LESS than its edge (a chord named for a curved
//!   carrier) is the unsound direction: it could clear a pair that
//!   meets.
//! - `census::in_plane`, arm 1's clear in a plane, GRANTS: it reads
//!   carrier's kind only to tell a line edge, whose
//!   ends decide which side of a plane it lies on, from any other,
//!   whose reach must lie definitely off the plane. A looser read (a
//!   line read as curved) only asks more of the edge and REFUSES
//!   a pair that is apart; a chord named for a curved carrier is the
//!   unsound direction, reading a curve by its ends.
//! - `pieces`'s screen (`pieces::Screen`, its face boxes built by the
//!   boolean's exit in `boolean/ops.rs` and passed in) PRUNES: two
//!   shells of one result whose hulled boxes clear are never probed
//!   against each other, and where each shell stands is decided by the
//!   point-in-solid walk through `Decide`. A bigger box costs a probe
//!   (and a recorded verdict), never an answer.
//! - `boolean::carrier_touch`'s ball reading (`edge_clear_of_ball`)
//!   PRUNES: an edge whose box clears a touch's ball is clear of it,
//!   and any other is decided on a lower bound on its distance through
//!   `Decide`. A bigger box costs that bound, never an answer — except
//!   for an edge with no bound (a spline or a spiric), which is not
//!   clear, so there a bigger box REFUSES a touch the edge stays clear
//!   of: the crossing layer keeps its typed door.
//! - the held-pierce read (`vtxfac::faces_holding_pierce`, its curved
//!   faces' boxes built in `boolean/mod.rs`) PRUNES: a curved face whose
//!   box clears a pierce point is not asked whether it holds it, and any
//!   other is decided on its carrier's residual and its trim through
//!   `Decide`. A bigger box costs that read, never an answer — except
//!   for a face on the point's carrier whose trim the chart door cannot
//!   read, which is not cleared, so there a bigger box REFUSES
//!   (`HeldPierceUnbuilt`) a point the face stays clear of.
//!
//! So nothing here may say "loose is free" about a BOX. It is a claim
//! about a door, and the door has to be named. The thirteen are not
//! recited: `every_door_that_reads_a_box_is_inventoried` below walks
//! `topo/src` and pins them per file — both rules, face and edge — so
//! a new door cannot land unargued. **It pins WHERE the doors are
//! and not which way each reads**, which is the column that carries
//! the argument above; that gap is `S234` and has an owner rather
//! than a disclosure.
//!
//! [`sweep_pad`] is sized so the padding can never lose an accepted
//! pair at the sweep's door (its derivation below).
//!
//! [`FaceBoxRule`] is the ONE statement of which surface kinds have a
//! cheap sound box and by what construction; [`face_box`] is its
//! `f64`-bracket instantiation and `census`'s `face_reach_in` is its
//! instantiation at the census's own scalar. **Neither re-derives an
//! extent**: the per-kind arithmetic lives once, in [`slab_extent`],
//! [`cone_frustum_extent`], [`ball_extent`], [`torus_extent`],
//! [`ring_window_extent`], [`torus_rect_extent`] and [`conic_extent`],
//! written against [`Span`] so a lane on the [`Bounds`] allowlist and a
//! lane off it can both enter it — the first with `[lo(), hi()]`
//! brackets at `f64`, the second with degenerate spans at its own `T`.
//! An EDGE has no rule here: the region it occupies is its cover
//! ([`edge_cover`]), whose one kind match is `geom`'s
//! `Curve3::cover_over`, and every edge door reads that cover's methods
//! ([`edge_box`] its bracketed world box, the census its box in a frame,
//! the slab arms its support along an axis, [`cover_axial`]). What the
//! census still owns is its arena WALK and its answer for an edge with
//! no cover (`None`, versus the poison box here); neither is arithmetic.
//!
//! An allowlisted [`geom_core::Bounds`] seam (ratified 2026-07-29 —
//! see geom-core `real.rs`, Bounds scope rule; the C10 tree is the
//! subdivision driver): coordinates enter as `[lo(), hi()]` brackets,
//! and poison flows to the poison box, which each door reads in its
//! own fail-loud direction — never prunes at the sweep, refuses at
//! the other three.

use bvh::Aabb;
use geom::Surface;
use geom::cover::{Cover, ParamWindow};
use geom::surfaces::nurbs::NurbsSurface;
use geom_core::spline::SpanLocate;
use geom_core::{Band, Bounds, Decide, Point3, Real, UnitVec3, Vec3};

use super::BooleanError;
use crate::body::Body;
use crate::entity::{Edge, EdgeKey, EntityId, FaceKey, HalfEdgeKey};
use crate::live::{BoundaryMember, linked, proven};

/// The sweep's box pad in meters — what candidate generation must add
/// so pruning can never lose an accepted pair. Derivation (each term
/// against the sweep's accept conditions in `reduce`):
///
/// - an event point classifies ON the face plane / boundary only
///   within `band.zero()` (`bool_vertex_face_side`, `contfp`'s
///   `bool_contact_*` sites) — one `zero` for the point-to-face gap;
/// - vertices sit on their carriers only up to attachment-time
///   certification (`Certificate::max_residual ≤ ε`, the same run
///   tolerance the linear band's `zero` is built from) — one more
///   `zero` per side for vertex-extent honesty;
/// - `band.escalate()` on top dominates every remaining f64 slop
///   (crossing-point interpolation and `eval` rounding are
///   session-box-scale ulps, orders below it) and keeps near-boundary
///   escalation zones inside candidate range.
///
/// The sum is deliberately generous, and what that buys differs by
/// door (module docs). At the sweep's it only admits more candidate
/// pairs and never changes an answer (the differential suite pins
/// that). At `separation`'s and at `boolean::ops`'s fallback it can
/// only make a certificate harder to obtain, never wrongly grant one
/// — the same direction, but a cost rather than a free one.
pub(crate) fn sweep_pad(band: Band) -> f64 {
    band.escalate() + 2.0 * band.zero()
}

/// The K funnel name of a cylinder face's axis-length decision, taken
/// where [`face_box_rule`] reads the carrier: the slab reads the axis
/// as a unit direction, and a carrier's axis is unit only by the
/// surfaces' at-rest convention. Its comparand is the axis's norm, a
/// pure number, levered by the cylinder's radius
/// ([`UnitVec3::levered`]).
pub(crate) const BOX_CYLINDER_AXIS: &str = "bool_box_cylinder_axis";

/// One coordinate's ENCLOSURE, in whatever scalar the reading lane
/// works in — **the form the per-kind extents below are written
/// against, so that exactly one derivation of them exists.**
///
/// Two lanes read those extents and they cannot share a scalar: the
/// `f64`-bracket lane ([`face_box`], [`edge_box`]) folds a
/// `T: Bounds` description down to `f64` through `[lo(), hi()]`, and
/// the census's lane stays in its own `T` (a `Dual` body's box has to
/// be compared by that lane's `Decide`). A span is what both can
/// spell: the bracket lane instantiates `Span<f64>` from the
/// bracket, the census lane instantiates `Span<T>` with `lo == hi`,
/// and neither takes a bound the other cannot.
///
/// The arithmetic here is interval arithmetic, so a description whose
/// coordinates are themselves enclosures (an `Interval` scalar's) is
/// enclosed rather than sampled at one arbitrary endpoint. Rounding
/// is NOT directed. No extent written here takes the root of a
/// cancelling difference, so the arithmetic's own error is a few ulps
/// of the box's scale. The `f64` lane's last step is [`Aabb::padded`],
/// whose outward ulp plus [`sweep_pad`] dominates those few ulps. The
/// census lane adds no pad, so at `f64` its boxes may fall short of the
/// locus by them: that this costs nothing rests on what each census
/// reader compares the box against, which nothing here can check
/// (`census-lane-span-extents-are-not-self-enclosing`).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Span<T> {
    /// The enclosure's lower end.
    pub lo: T,
    /// The enclosure's upper end.
    pub hi: T,
}

impl<T: Real> Span<T> {
    /// A degenerate span — one exact value.
    pub(crate) fn exact(v: T) -> Self {
        Self { lo: v, hi: v }
    }

    fn add(self, o: Self) -> Self {
        Self {
            lo: self.lo + o.lo,
            hi: self.hi + o.hi,
        }
    }

    /// The hull of two spans.
    pub(crate) fn hull(self, o: Self) -> Self {
        Self {
            lo: self.lo.min(o.lo),
            hi: self.hi.max(o.hi),
        }
    }

    /// Interval product.
    ///
    /// **Poison survives a single poisoned END**, which a naive
    /// min/max fold over the four corner products would drop:
    /// [`Real::min`] propagates poison, so folding `NaN` against a
    /// finite corner yields `NaN` — but only because THIS fold is the
    /// `Real` one. `f64::min` returns the non-NaN operand, and a fold
    /// written with it would quietly return a finite product for a
    /// description with a poisoned bracket end. The
    /// `a_half_poisoned_span_poisons_its_product` row pins it.
    fn mul(self, o: Self) -> Self {
        let (a, b, c, d) = (
            self.lo * o.lo,
            self.lo * o.hi,
            self.hi * o.lo,
            self.hi * o.hi,
        );
        Self {
            lo: Real::min(Real::min(a, b), Real::min(c, d)),
            hi: Real::max(Real::max(a, b), Real::max(c, d)),
        }
    }

    /// Outward by `w` on both ends.
    fn widen(self, w: T) -> Self {
        Self {
            lo: self.lo - w,
            hi: self.hi + w,
        }
    }

    /// An UPPER bound on `|x|` over the span.
    fn abs_max(self) -> T {
        self.hi.max(-self.lo)
    }
}

/// Three coordinate spans: a box, a point, or a direction, depending
/// on what the reader wants of it ([`Span`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct SpanBox<T> {
    /// The x span.
    pub x: Span<T>,
    /// The y span.
    pub y: Span<T>,
    /// The z span.
    pub z: Span<T>,
}

impl<T: Real> SpanBox<T> {
    /// An exact point or vector as three degenerate spans.
    pub(crate) fn point(p: Point3<T>) -> Self {
        Self {
            x: Span::exact(p.x),
            y: Span::exact(p.y),
            z: Span::exact(p.z),
        }
    }

    /// An exact direction as three degenerate spans.
    pub(crate) fn vector(v: Vec3<T>) -> Self {
        Self {
            x: Span::exact(v.x),
            y: Span::exact(v.y),
            z: Span::exact(v.z),
        }
    }
}

/// The enclosure of a DECIDED unit direction: the [`SpanBox`] of a
/// [`UnitVec3`], minted only from one, so an extent that takes it
/// reads a unit axis by type. Two mints: the bracket lane takes the
/// witness's `f64` bracket ([`UnitSpanBox::bracketed`]); the census
/// lane takes it at its own scalar, rotated into the frame it reads
/// ([`BoxFrame::unit`]), which is the witness's components along
/// orthonormal rows — unit to the rounding of a dot product, as the
/// witness itself is unit to the rounding of its normalize.
#[derive(Clone, Copy, Debug)]
pub(crate) struct UnitSpanBox<T>(SpanBox<T>);

impl<T: Real> UnitSpanBox<T> {
    /// The enclosure itself.
    pub(crate) fn get(&self) -> &SpanBox<T> {
        &self.0
    }
}

/// **The frame a box is read in**: the world's axes, or a rotation of
/// them whose first axis is a decided unit direction.
///
/// Every per-kind extent in this module computes coordinate `i` of its
/// box from coordinate `i` of its inputs alone, save one reading: a
/// unit axis's room perpendicular to row `i` ([`perp_room`]), which is
/// the norm of the axis's part off that row, `√(a_j² + a_k²)`, and
/// which every orthonormal frame keeps as it keeps `a_i`
/// (`every_extent_reads_each_coordinate_from_that_coordinate_alone`
/// holds each of them to it, bit for bit), and its soundness argument
/// reads coordinate `i` as the component along a UNIT direction —
/// never as anything particular to the world's `x̂`, `ŷ` or `ẑ`. An
/// extent added here owes the same, and a row in that test. So the same extents, fed the components of their inputs
/// along the rows of any orthonormal frame, box the face in that
/// frame, and the first coordinate of an [`Aimed`](Self::Aimed) frame's
/// box is the face's support along its aim. That is a reading that
/// turns with the body: the world box of a turned face widens by how
/// it is turned, and the support along a direction fixed to the face
/// does not.
///
/// The aimed frame is the aim with [`UnitVec3::orthonormal_basis`],
/// in the cyclic order `(aim, b1, b2)` of the right-handed
/// `(b1, b2, aim)`, so a cross product of two mapped vectors is the
/// mapped cross product.
///
/// **Rounding.** A point is turned about the WORLD origin, so its
/// coordinate along a row is a dot product rounded by at most
/// `γ₃·Σ|nᵢpᵢ| ≤ 3u·|p|`, about one and a half ulps of `|p|`
/// (`the_aimed_frames_rounding_is_a_few_ulps_of_the_point` measures it
/// out to `|p| = 10⁸` m). The pad every reader adds is `12 ε`, so it
/// covers that wherever a coordinate of magnitude `|p|` resolves `ε` at
/// all — and a body placed where `ulp(|p|) > ε` has vertices the band
/// cannot place either.
#[derive(Clone, Copy, Debug)]
pub(crate) enum BoxFrame<T: Real> {
    /// The world's axes: every point and vector read as it is stored.
    World,
    /// The rows of a right-handed orthonormal frame whose first row is
    /// the aim.
    Aimed([Vec3<T>; 3]),
}

impl<T: Real> BoxFrame<T> {
    /// The frame whose first axis is `aim`. Its other two rows are
    /// `orthonormal_basis`'s: a box read in it is scratch that no output
    /// keeps, and every reading of it is the face's support along the
    /// aim, which those rows do not move.
    pub(crate) fn aimed(aim: UnitVec3<T>) -> Self {
        let (b1, b2) = aim.orthonormal_basis();
        Self::Aimed([aim.get(), b1, b2])
    }

    /// The frame's rows: the world's axes, or the aimed frame's.
    pub(crate) fn rows(&self) -> [Vec3<T>; 3] {
        match self {
            Self::World => [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()],
            Self::Aimed(rows) => *rows,
        }
    }

    /// A direction's components along the frame's rows.
    pub(crate) fn vector(&self, v: Vec3<T>) -> Vec3<T> {
        match self {
            Self::World => v,
            Self::Aimed([a, b, c]) => Vec3::new(a.dot(v), b.dot(v), c.dot(v)),
        }
    }

    /// A point's coordinates in the frame, about the world origin.
    pub(crate) fn point(&self, p: Point3<T>) -> Point3<T> {
        match self {
            Self::World => p,
            Self::Aimed(_) => {
                let v = self.vector(Vec3::new(p.x, p.y, p.z));
                Point3::new(v.x, v.y, v.z)
            }
        }
    }

    /// A decided unit direction's components in the frame: a rotation
    /// of a unit vector is one, so the witness carries.
    pub(crate) fn unit(&self, u: UnitVec3<T>) -> UnitSpanBox<T> {
        UnitSpanBox(SpanBox::vector(self.vector(u.get())))
    }
}

impl UnitSpanBox<f64> {
    /// The witness's `f64` bracket ([`bracket_vector`]).
    pub(crate) fn bracketed<T: Bounds>(u: UnitVec3<T>) -> Self {
        Self(bracket_vector(u.get()))
    }
}

/// A description's coordinates as `f64` spans — the bracket lane's
/// entry into [`SpanBox`]. Poison surfaces as NaN ends rather than
/// narrowing away, per [`Bounds`].
fn bracket_point<T: Bounds>(p: Point3<T>) -> SpanBox<f64> {
    SpanBox {
        x: Span {
            lo: p.x.lo(),
            hi: p.x.hi(),
        },
        y: Span {
            lo: p.y.lo(),
            hi: p.y.hi(),
        },
        z: Span {
            lo: p.z.lo(),
            hi: p.z.hi(),
        },
    }
}

/// [`bracket_point`] for a direction.
fn bracket_vector<T: Bounds>(v: Vec3<T>) -> SpanBox<f64> {
    bracket_point(Point3::new(v.x, v.y, v.z))
}

/// The AXIAL SLAB: the axis segment over `h`, widened by `radius`
/// **perpendicular to the axis and only there**.
///
/// A surface point is `origin + h·axis + ρ·û` with `û ⊥ axis`,
/// `|û| = 1` and `ρ ≤ radius`, so coordinate `i` is
/// `origin_i + h·axis_i + ρ·û_i` and `|û_i| ≤ √(1 − axis_i²)` — the
/// most a unit vector perpendicular to a UNIT axis can spend on one
/// coordinate. The axial coordinate of an axis-aligned cylinder
/// therefore takes no widening at all, which is the whole content of
/// the arm: the boundary already bounds the axial extent, and adding
/// `radius` there claimed a slab longer than the face.
///
/// The room is read off the axis's other two components, each bounded
/// ABOVE ([`Span::abs_max`]), so the perpendicular factor is bounded
/// above, which is the sound direction ([`perp_room`]); an axis
/// bracket that does not pin the direction reads as more
/// perpendicular room, never less, and a poisoned one poisons the box.
///
/// The axis is a [`UnitSpanBox`], so `h` is a length along it: the
/// reader decides the carrier axis's length ([`BOX_CYLINDER_AXIS`]).
pub(crate) fn slab_extent<T: Real>(
    origin: &SpanBox<T>,
    axis: &UnitSpanBox<T>,
    h: Span<T>,
    radius: T,
) -> SpanBox<T> {
    let axis = axis.get();
    let room = |i| perp_room(others([axis.x, axis.y, axis.z], i));
    SpanBox {
        x: origin.x.add(h.mul(axis.x)).widen(radius * room(0)),
        y: origin.y.add(h.mul(axis.y)).widen(radius * room(1)),
        z: origin.z.add(h.mul(axis.z)).widen(radius * room(2)),
    }
}

/// The most one coordinate of a UNIT vector perpendicular to a unit
/// `axis` can be: `√(1 − axis_i²)`, spelled `√(axis_j² + axis_k²)`
/// over the OTHER two coordinates `(j, k)` of the axis, each bounded
/// ABOVE so the room is bounded above — the sound direction
/// ([`slab_extent`]).
///
/// The two spellings agree for a unit axis, but `1 − axis_i²` cancels
/// as the axis nears row `i`: `axis_i²` rounds by `u` absolute, so the
/// room it leaves is off by about `u/(2·room)` and is exactly zero
/// below `room ≈ 1e-8`, dropping the widening `radius·room` the axis's
/// tilt owes. The other two components carry the room to a few ulps of
/// itself.
fn perp_room<T: Real>((j, k): (Span<T>, Span<T>)) -> T {
    (j.abs_max().powi(2) + k.abs_max().powi(2)).sqrt()
}

/// The coordinates of `v` other than `i`, in cyclic order — the pair
/// [`perp_room`] reads for coordinate `i`.
fn others<T: Copy>(v: [T; 3], i: usize) -> (T, T) {
    (v[(i + 1) % 3], v[(i + 2) % 3])
}

/// The norm of `v`'s components other than `k`: a unit `v`'s share of
/// a direction perpendicular to it along row `k`, without the
/// cancellation of `√(1 − v_k²)` ([`perp_room`]).
fn pick_other<T: Real>(v: Vec3<T>, k: usize) -> T {
    let (j, l) = others([v.x, v.y, v.z], k);
    (j.powi(2) + l.powi(2)).sqrt()
}

/// The CONE FRUSTUM over the axial window `h` — the slab whose radius
/// TRACKS `h` instead of being pinned at the window's widest end.
///
/// A cone point at axial coordinate `t` sits at radial offset
/// `|t|·tan α` EXACTLY, so coordinate `i` of the trimmed face lies in
/// `apex_i + [min g, max g]` over `t ∈ h`, where
/// `g(t) = t·axis_i ± |t|·tan α·√(1 − axis_i²)`. That `g` is
/// piecewise linear with one kink, at `t = 0` (the apex), so its
/// extremes over a closed window are attained at the two ends or at
/// the kink — three candidates, hulled, and the result is the
/// frustum's own box rather than a superset of it.
///
/// **Why the radius may not be pinned at the wide end.** A pucker,
/// a chamfer, a lamp shade: the face occupies a window far from the
/// apex, where the near and far radii differ by little, but the
/// window's own `|t|·tan α` at the far end is what a constant-radius
/// slab claims along the WHOLE window — and out at the near end that
/// is the widest part of the cone applied where the face is
/// narrowest. Doors that read box overlap as "may meet" pay that
/// difference in refusals: it is what named a (Cone, Sphere) germ
/// pair for a lily tepal seam whose exact frustum clears the carving
/// ball entirely.
///
/// `h0` is the point of the window nearest the apex —
/// `clamp(0, h.lo, h.hi)`, which IS an end when the window does not
/// straddle the apex, so the kink candidate costs nothing and needs
/// no sign decision. The axis is read as UNIT, `h` a length along it:
/// a premise of the cone carrier's at-rest convention, undecided here.
pub(crate) fn cone_frustum_extent<T: Real>(
    apex: &SpanBox<T>,
    axis: &SpanBox<T>,
    h: Span<T>,
    tan_half_angle: T,
) -> SpanBox<T> {
    let h0 = h.lo.max(T::zero()).min(h.hi);
    let rows = [axis.x, axis.y, axis.z];
    let coord = |o: Span<T>, i: usize| {
        let a = rows[i];
        let k = tan_half_angle * perp_room(others(rows, i));
        let at = |t: T| Span::exact(t).mul(a).widen(t.abs() * k);
        o.add(at(h.lo).hull(at(h.hi)).hull(at(h0)))
    };
    SpanBox {
        x: coord(apex.x, 0),
        y: coord(apex.y, 1),
        z: coord(apex.z, 2),
    }
}

/// The WHOLE BALL `center ± radius` — every surface point is within
/// `radius` of the centre in every coordinate.
pub(crate) fn ball_extent<T: Real>(center: &SpanBox<T>, radius: T) -> SpanBox<T> {
    SpanBox {
        x: center.x.widen(radius),
        y: center.y.widen(radius),
        z: center.z.widen(radius),
    }
}

/// The WHOLE TORUS about `center`. A torus point is
/// `center + (R + r·cos φ)·û + r·sin φ·axis` with `û ⊥ axis` a unit
/// vector, so coordinate `i` reaches at most
/// `(R + r)·√(1 − axis_i²) + r·|axis_i|` from the centre — the
/// perpendicular room of [`slab_extent`] for the in-plane part plus
/// the tube's own reach along the axis. For an axis-aligned torus
/// that is exactly the true box. The axis is read as UNIT — a premise
/// of the torus carrier's at-rest convention, undecided here — and it
/// is the widening's own: `|axis| > 1` would claim less perpendicular
/// room than a unit `û` can take.
pub(crate) fn torus_extent<T: Real>(
    center: &SpanBox<T>,
    axis: &SpanBox<T>,
    major: T,
    minor: T,
) -> SpanBox<T> {
    let rows = [axis.x, axis.y, axis.z];
    let reach = |i: usize| (major + minor) * perp_room(others(rows, i)) + minor * rows[i].abs_max();
    SpanBox {
        x: center.x.widen(reach(0)),
        y: center.y.widen(reach(1)),
        z: center.z.widen(reach(2)),
    }
}

/// **A RING TORUS over a chart rectangle, in closed form**: per
/// coordinate, the least and greatest of `e·S(u, v)` over the window
/// `u × v`, enclosed over every torus the description's spans admit
/// ([`torus_rect_extent`]).
///
/// # Why the rectangle and not a hull of the boundary
///
/// [`clip_to_boundary`] is sound on a cylinder because no coordinate
/// has an interior critical point on a cylinder chart. On a torus every
/// direction `e` has four, a max, a min and two saddles, so a face
/// holding one bulges past its own boundary and the boundary hull is
/// not a bound. The chart rectangle's extent is: the face's region lies
/// in the rectangle its boundary's certified pcurves span
/// ([`TorusChartWindow`]), and over a rectangle the extremes have a
/// closed form.
///
/// The closed form reads `R + r·cos v ≥ 0` over the window: every torus
/// the spans admit is a ring, `R ≥ r ≥ 0`. The box lanes decide that
/// before they enter here ([`FaceBoxRule`]). The rows `u_ref`,
/// `v_ref` and `axis` enter as given, so the extent is exact for
/// `S = c + (R + r·cos v)·(cos u·u_ref + sin u·v_ref) + r·sin v·axis`
/// whatever their lengths; the meet with [`torus_extent`] every reader
/// takes inherits that extent's unit-axis premise.
pub(crate) fn ring_window_extent<T: Decide>(
    center: &SpanBox<T>,
    (u_ref, v_ref, axis): (&SpanBox<T>, &SpanBox<T>, &SpanBox<T>),
    radii: (Span<T>, Span<T>),
    window: TorusWindowPair<T>,
) -> SpanBox<T> {
    let at = |c, p, q, a| torus_rect_extent(c, (p, q, a), radii, window);
    SpanBox {
        x: at(center.x, u_ref.x, v_ref.x, axis.x),
        y: at(center.y, u_ref.y, v_ref.y, axis.y),
        z: at(center.z, u_ref.z, v_ref.z, axis.z),
    }
}

/// Two boxes' INTERSECTION, coordinate by coordinate — the fold that
/// makes a windowed torus box "never looser than the whole tube"
/// structural rather than a claim about the window.
///
/// Both arguments are supersets of the same locus, so their
/// intersection is one too, and it inherits [`torus_extent`]'s
/// UNIT-AXIS premise from the argument it is folded against.
///
/// Poison propagates: [`Real::min`] and [`Real::max`] carry it. **A
/// poisoned WINDOW never reaches here** — [`TorusChartWindow::finish`]
/// turns a non-finite window end into `None`, and the arm then folds
/// nothing but the whole tube — so the poison this fold carries is a
/// poisoned DESCRIPTION's, which poisons both arguments alike.
pub(crate) fn meet<T: Real>(a: SpanBox<T>, b: SpanBox<T>) -> SpanBox<T> {
    let one = |x: Span<T>, y: Span<T>| Span {
        lo: x.lo.max(y.lo),
        hi: x.hi.min(y.hi),
    };
    SpanBox {
        x: one(a.x, b.x),
        y: one(a.y, b.y),
        z: one(a.z, b.z),
    }
}

/// A torus face's chart window: the `u` (major azimuth) and `v`
/// (minor angle) channels' spans, in that order.
pub(crate) type TorusWindowPair<T> = (Span<T>, Span<T>);

/// One HALF-EDGE, as the window walk reads it: its stored certified
/// pcurve cache, its image as the loop's lift places it
/// ([`crate::Body::loop_lift`]), and whether the loop traverses it
/// FORWARD (the `he_plus` side, so the certified span runs `t₀ → t₁`).
/// `None` for a half-edge of a loop with no lift.
pub(crate) type WindowStep<'a, T> =
    Option<(&'a geom_brep::PcurveCache<T>, geom_brep::Pcurve<T>, bool)>;

/// **A torus face's chart window, from its boundary's stored certified
/// pcurves — the ONE walk, for every lane.**
///
/// The caller hands over its own arena traversal as loops of
/// [`WindowStep`]s and nothing else: `boxes.rs`'s `face_box`, the
/// census's `face_reach` and the construction rows all enter here, so
/// the fail modes cannot drift between them the way two hand-written
/// walks did.
///
/// # Why the window is a superset of the face's chart region
///
/// The face's region is the planar region its loops' chart images
/// bound, in the lift the pcurve mint certified closed, and a region
/// lies inside the bounding rectangle of its own boundary.
///
/// **That argument needs the boundary to SAY which side is material,
/// and two guards are what make it hold:
///
/// - **A face with a RING gets no window.** Two loops can be pinned on
///   different branches, and their hull is then a rectangle neither
///   bounds.
/// - **A loop whose walk WRAPS a channel gets no window.** A loop of
///   lone full circles closes by going once round a channel rather
///   than by returning to where it started, and the two complementary
///   annuli it can denote have the SAME pcurves and the same hull —
///   so the hull cannot contain both, and nothing in the caches says
///   which one this face is. Measured: a face bounded by two lone
///   full meridians at `u = 0` and `u = 22°` windows to
///   `[0, 22°] × [0, 2π]`, and in the orientation whose material is
///   the 338° complement, 1591 of 1681 sampled surface points lie
///   OUTSIDE that box. No kernel door mints such a face today — every
///   torus constructor seams its walls — so this is a latent hole
///   rather than a live one, and it is closed by refusing the class.
///
/// The wrap test is the loop's NET travel per channel: a loop that
/// closes in the chart returns to its entry point, so the signed sum
/// of `P(exit) − P(entry)` over its half-edges is zero, while a loop
/// that wraps sums to `± τ`. Half a period separates the two, so the
/// test needs no `Band` and no `decide`.
///
/// # The slack, and the bound it holds under
///
/// A cache certifies `sup |S(P(t)) − C(t)| ≤ envelope` in METRES
/// ([`geom_brep::PcurveCertificate::envelope`]), so the carrier can
/// sit up to `envelope` off the chart image's surface points. Pulling
/// that back through the chart costs the inverse's Lipschitz
/// constants: one radian of `u` moves a surface point by at least
/// `R − r` (the tube's nearest approach to the axis) and one radian of
/// `v` by exactly `r`.
///
/// **The factor of two is not "one for each end"** — [`Span::widen`]
/// already touches both. It is the price of the segment between
/// `S(P(t))` and `C(t)` LEAVING the surface: the chart inverse is
/// Lipschitz on the surface, and a point `envelope` off it is pulled
/// back only after being projected there, which can cost as much
/// again. That argument holds while the displacement stays inside the
/// tube's own scale — `envelope ≤ (R − r)/2` and `envelope ≤ r/2` —
/// so [`torus_window_slack`] GUARDS both, and a cache whose envelope
/// exceeds either gets no window at all.
pub(crate) struct TorusChartWindow<T: Real> {
    u: Option<Span<T>>,
    v: Option<Span<T>>,
    envelope: T,
    /// The current loop's signed travel per channel; `None` once a
    /// loop has been abandoned or has wrapped.
    net: Option<(T, T)>,
    loops: usize,
    ok: bool,
}

/// The certified slack a torus chart window owes an envelope, or
/// `None` when the envelope is too large for the argument that
/// derives it ([`TorusChartWindow`], "the slack").
///
/// `envelope` is in metres and both radii are lengths, so the guard is
/// a comparison of lengths and reads no angle.
pub(crate) fn torus_window_slack<T: Real>(
    envelope: T,
    major: T,
    minor: T,
) -> Option<TorusWindowPair<T>> {
    let half = T::from_f64(0.5);
    let (tube, ring) = (minor, major - minor);
    // **The comparison, in the terms [`Real`] has.** There is no
    // ordering on a generic scalar here — an interval scalar cannot
    // answer `<=` — so the guard is spelled through the totality
    // policy the module already builds [`geom_core::poison`] with:
    // `√(bound − envelope)` is poison exactly when the envelope
    // exceeds the bound, and at a bracketed scalar it is poison as
    // soon as it MIGHT, which refuses in the direction that costs the
    // whole tube rather than a wrong window.
    let exceeded = |bound: T| (bound * half - envelope).sqrt().is_poison();
    if exceeded(tube) || exceeded(ring) {
        return None;
    }
    let two = T::from_f64(2.0);
    Some((
        Span {
            lo: T::zero() - two * envelope / ring,
            hi: two * envelope / ring,
        },
        Span {
            lo: T::zero() - two * envelope / tube,
            hi: two * envelope / tube,
        },
    ))
}

/// A value that is not FINITE, in the terms [`Real`] has: `x − x` is
/// zero for a finite `x`, poison for `±∞` and poison for poison. The
/// window ends are tested with this rather than with
/// [`Real::is_poison`] alone, which an infinity walks straight past on
/// its way to a NaN sample.
// `x − x` is the test, not a typo: it is the one expression that is
// zero for every finite value and poison for `±∞` and for poison
// itself, in a trait with no ordering and no `is_finite`.
#[allow(clippy::eq_op)]
fn not_finite<T: Real>(x: T) -> bool {
    (x - x).is_poison()
}

impl<T: Real> TorusChartWindow<T> {
    /// An empty accumulator.
    pub(crate) fn new() -> Self {
        Self {
            u: None,
            v: None,
            envelope: T::zero(),
            net: Some((T::zero(), T::zero())),
            loops: 0,
            ok: true,
        }
    }

    /// Open a loop. The FIRST loop is the face's outer one; every
    /// later one is a ring, and a ring abandons the window.
    pub(crate) fn open_loop(&mut self) {
        self.loops += 1;
        if self.loops > 1 {
            self.ok = false;
        }
        self.net = Some((T::zero(), T::zero()));
    }

    /// One half-edge of the open loop. The window reads a torus chart's
    /// harmonic images; any other image (a Villarceau circle's focal
    /// section among them) abandons it — which widens the box to the
    /// whole tube, never narrows it.
    pub(crate) fn step(&mut self, step: &WindowStep<'_, T>) {
        let Some((cache, image, forward)) = step else {
            self.ok = false;
            return;
        };
        let (t0, t1) = cache.params();
        let Some(b) = image.closed_form_span_box(t0, t1) else {
            self.ok = false;
            return;
        };
        let u = Span {
            lo: b.u_min,
            hi: b.u_max,
        };
        let v = Span {
            lo: b.v_min,
            hi: b.v_max,
        };
        let Some(travel) = harmonic_travel(image, t0, t1, *forward) else {
            self.ok = false;
            return;
        };
        self.u = Some(self.u.map_or(u, |a: Span<T>| a.hull(u)));
        self.v = Some(self.v.map_or(v, |a: Span<T>| a.hull(v)));
        self.envelope = self.envelope.max(cache.certificate().envelope);
        self.net = self.net.map(|(a, b)| (a + travel.0, b + travel.1));
    }

    /// Close the open loop: it must return to where it entered the
    /// chart rather than wrap a channel (the type docs).
    pub(crate) fn close_loop(&mut self) {
        let Some((nu, nv)) = self.net else {
            self.ok = false;
            return;
        };
        // Half a period separates a loop that closed (net zero) from
        // one that wrapped (net `± τ`), and the test is the same
        // poison spelling [`torus_window_slack`] uses — no ordering,
        // no `Band`, no `decide`.
        for n in [nu, nv] {
            if not_finite(n) || (T::pi() - n.abs()).sqrt().is_poison() {
                self.ok = false;
            }
        }
        self.net = None;
    }

    /// Abandon the window outright — the caller's own arm for a loop
    /// it cannot read at all (a lone vertex carries no chart image).
    pub(crate) fn abandon(&mut self) {
        self.ok = false;
    }

    /// The window, widened by the certificate slack — see the type
    /// docs. `None` whenever any guard fired, and the arm then keeps
    /// [`torus_extent`], the box every torus face had before this
    /// window existed. **The only discrete step in the arm is "window
    /// or no window", and its cost is not discrete**: no window widens
    /// to the whole tube, never narrows.
    pub(crate) fn finish(self, major: T, minor: T) -> Option<TorusWindowPair<T>> {
        if !self.ok {
            return None;
        }
        let (u, v) = (self.u?, self.v?);
        let (su, sv) = torus_window_slack(self.envelope, major, minor)?;
        let u = Span {
            lo: u.lo + su.lo,
            hi: u.hi + su.hi,
        };
        let v = Span {
            lo: v.lo + sv.lo,
            hi: v.hi + sv.hi,
        };
        if [u.lo, u.hi, v.lo, v.hi].into_iter().any(not_finite) {
            return None;
        }
        Some((u, v))
    }
}

/// One half-edge's SIGNED travel in each chart channel, in the
/// direction the loop traverses it — `P(exit) − P(entry)`, the datum
/// [`TorusChartWindow`]'s wrap guard sums. `None` for a pcurve outside
/// the harmonic family.
pub(crate) fn harmonic_travel<T: Real>(
    pcurve: &geom_brep::Pcurve<T>,
    t0: T,
    t1: T,
    forward: bool,
) -> Option<(T, T)> {
    let (p0, pa, pb, pl) = pcurve.harmonic_form()?;
    let at = |t: T| {
        let (s, c) = t.sin_cos();
        (
            p0.x + pa.x * c + pb.x * s + pl.x * t,
            p0.y + pa.y * c + pb.y * s + pl.y * t,
        )
    };
    let (entry, exit) = if forward {
        (at(t0), at(t1))
    } else {
        (at(t1), at(t0))
    };
    Some((exit.0 - entry.0, exit.1 - entry.1))
}

/// **The face's loops, as [`WindowStep`]s**, read a loop at a time
/// through [`crate::body::Body::face_boundary_by_loop`]: the window
/// steps the boolean lane, the census lane and the construction rows
/// all read.
///
/// A lone-vertex loop yields an EMPTY loop, which [`torus_chart_window`]
/// abandons the window on: it carries no chart image.
///
/// # Panics
///
/// Where `face`, which every caller resolved in `body`, or a record on
/// the walk from it does not resolve, or a loop walk does not close
/// (D2 row 4). The bodies are at rest (the census's) or the boolean's
/// working copies, whose links hold by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn face_window_steps<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Vec<Vec<WindowStep<'_, T>>> {
    let f = proven(&body.faces, face, EntityId::Face);
    body.face_boundary_by_loop(face, f)
        .map(|(_, members, _)| {
            let cycle: Vec<(HalfEdgeKey, &Edge)> = members
                .into_iter()
                .filter_map(|member| match member {
                    BoundaryMember::Edge { he, edge, .. } => Some((he, edge)),
                    BoundaryMember::Isolated { .. } => None,
                })
                .collect();
            let halves: Vec<HalfEdgeKey> = cycle.iter().map(|&(he, _)| he).collect();
            let lifted = crate::pcurves::lifted_images(body, &halves);
            cycle
                .into_iter()
                .zip(lifted)
                .map(|((he, edge), image)| {
                    body.pcurve(he)
                        .zip(image)
                        .map(|(c, image)| (c, image, edge.he_plus == he))
                })
                .collect()
        })
        .collect()
}

/// **The ONE walk**, over a face's loops of [`WindowStep`]s — see
/// [`TorusChartWindow`]. A loop that yields NO half-edge (a lone
/// vertex) abandons the window: it carries no chart image, so the walk
/// cannot see what bounds the face's chart region.
pub(crate) fn torus_chart_window<T: Real>(
    loops: &[Vec<WindowStep<'_, T>>],
    major: T,
    minor: T,
) -> Option<TorusWindowPair<T>> {
    let mut acc = TorusChartWindow::new();
    for lp in loops {
        acc.open_loop();
        for step in lp {
            acc.step(step);
        }
        if lp.is_empty() {
            acc.abandon();
        } else {
            acc.close_loop();
        }
    }
    acc.finish(major, minor)
}

/// The full conic's centre-±-amplitude box: a conic point's
/// coordinate `i` is `center_i + a·û_i·cos t + b·v̂_i·sin t`, which
/// over a full turn reaches exactly `√((a·û_i)² + (b·v̂_i)²)` from the
/// centre. That is a function of the LOCUS: two descriptions of one
/// circle whose `u_ref` differ by an in-plane rotation give the same
/// number, where the triangle-inequality bound `|û_i|·a + |v̂_i|·b`
/// gives `r` at an axis-aligned `u_ref` and `r√2` at 45°.
///
/// The reference directions are bounded by [`Span::abs_max`] — the
/// largest magnitude the bracket admits, not one endpoint's, which is
/// what makes the bound hold for a bracket that straddles zero.
pub(crate) fn conic_extent<T: Real>(
    center: &SpanBox<T>,
    u_ref: &SpanBox<T>,
    v_ref: &SpanBox<T>,
    semi_u: T,
    semi_v: T,
) -> SpanBox<T> {
    let reach = |u: Span<T>, v: Span<T>| {
        let (a, b) = (u.abs_max() * semi_u, v.abs_max() * semi_v);
        (a.powi(2) + b.powi(2)).sqrt()
    };
    SpanBox {
        x: center.x.widen(reach(u_ref.x, v_ref.x)),
        y: center.y.widen(reach(u_ref.y, v_ref.y)),
        z: center.z.widen(reach(u_ref.z, v_ref.z)),
    }
}

/// A vector's three brackets: the key the narrow phase's axis set
/// compares two axes by ([`super::separating::AxisKey`]). Two equal keys
/// are one direction to the bracket, and a candidate dropped as one
/// only costs the reading along it.
pub(crate) fn axis_key<T: Bounds>(v: Vec3<T>) -> [(f64, f64); 3] {
    [v.x, v.y, v.z].map(|c| (c.lo(), c.hi()))
}

/// The padded box of a full circle ([`super::separating::Circle`]):
/// [`conic_extent`] at the bracket lane, so coordinate `i` reaches
/// `radius·√(u_i² + v_i²) = radius·√(1 − n_i²)` from the centre, where
/// `n` is the plane's normal — the circle's own extent, which turns with
/// it, not the `2·radius` cube about the centre.
pub(crate) fn circle_box<T: Bounds>(
    &super::separating::Circle {
        center,
        u_ref,
        v_ref,
        radius,
    }: &super::separating::Circle<T>,
    pad: f64,
) -> Aabb {
    let r = radius.hi();
    aabb_of(conic_extent(
        &bracket_point(center),
        &bracket_vector(u_ref),
        &bracket_vector(v_ref),
        r,
        r,
    ))
    .padded(pad)
}

/// **The one soundness rule for a face's box**, stated per surface
/// kind: which cheap construction yields a genuine SUPERSET of the
/// face's locus. Every consumer that bounds a face reads its arm from
/// here, so no two of them can quietly disagree about which kinds are
/// boxable.
///
/// The variants carry the surface payload the construction needs, so
/// the kind is matched ONCE and each lane only performs its own
/// arithmetic. The soundness argument per arm:
///
/// - [`BoundaryHull`](Self::BoundaryHull) — **Plane, and a cylinder
///   whose axis the band does not decide unit** ([`face_box_rule`]).
///   Either face lies in the convex hull of its boundary: a planar one
///   trivially, a cylindrical one because the cylinder is ruled along
///   its axis — the ruling through any point of the face meets the
///   face in a segment whose two ends lie on its boundary (an edge, a
///   seam, or an isolated vertex), whatever the radius, origin or
///   frame. So the hull of the boundary's own certified boxes contains
///   the face whatever the boundary curves are. The hull of the
///   boundary VERTICES alone does not: a circular rim bulges past its
///   endpoints, and this engine's plane×cylinder lane mints exactly
///   that face.
/// - [`CylinderSlab`](Self::CylinderSlab) — **Cylinder.** The wall's
///   belly bulges past its chords, so the box is the whole cylinder
///   slab over the face's axial range (the axial coordinate is linear
///   along the surface, so the face's axial extremes lie on its
///   boundary), widened by the radius **perpendicular to the axis**
///   ([`slab_extent`] carries the derivation), then clipped to the
///   boundary hull, which contains the face by the ruled argument
///   above ([`clip_to_boundary`]).
/// - [`ConeSlab`](Self::ConeSlab) — **Cone.** The FRUSTUM over the
///   face's axial window, not a slab of constant radius: a cone point
///   is `apex + v·(axis·cos α + û·sin α)`, so at axial coordinate
///   `t = v·cos α` its radial offset is `|t|·tan α` EXACTLY. `t` is
///   linear in the chart, so its extremes over a trimmed face lie on
///   the face's boundary exactly as the cylinder's do
///   ([`cover_axial`]), and [`cone_frustum_extent`] boxes the frustum
///   that window cuts. Pinning the radius at the window's widest end
///   instead is what makes a pucker read as if it were the whole
///   cone, and doors that read overlap as "may meet" pay it in
///   refusals.
/// - [`SphereWindow`](Self::SphereWindow) — **Sphere.** A band's belly
///   bulges past its poles and seam arcs, so the boundary does not
///   bound the face. Where the face is the chart rectangle its boundary
///   spans, or lies in the latitude zone its rims bound, the box is that
///   rectangle's or zone's own support along each coordinate
///   ([`sphere_reach`], read off the certified trim and its side);
///   otherwise it is the whole ball `center ± r`, within which every
///   surface point lies.
/// - [`ControlNet`](Self::ControlNet) — **NURBS.** The patch bulges
///   past the hull of its boundary exactly as the sphere does, but it
///   lies in the hull of its CONTROL NET (nonnegative basis, strictly
///   positive weights — `geom::surfaces::boxes::nurbs_surface_aabb`
///   carries the citation), over the whole KNOT DOMAIN and a fortiori
///   over any trim inside it.
///
///   **The premise that carries: the face's trim lies inside the knot
///   domain.** The convex-hull property is a statement about the
///   domain the basis is defined on; this kernel's evaluator
///   EXTRAPOLATES outside it, and an extrapolated point can leave the
///   control hull by any amount. Nothing in the type system enforces
///   trim ⊆ domain today — `pcurves.rs`'s chart window is built from
///   the boundary's own chart boxes and is never compared against
///   `knot_values_u().domain()`. What holds it up is construction: every
///   kernel-minted NURBS wall is iso-parameter bounded at the domain
///   edges. State the premise when you add a constructor that is not.
/// - [`TorusWindow`](Self::TorusWindow) — **Torus.** The CHART
///   RECTANGLE the face's boundary occupies, boxed in closed form
///   ([`ring_window_extent`]) and intersected with the whole tube
///   ([`torus_extent`]); the whole tube alone when no window can be
///   read or the ring convention is not decided.
///
///   A torus coordinate is not a chart coordinate the way a
///   cylinder's azimuth is: `e·S` has four interior critical points
///   on the chart (a max, a min and two saddles), so
///   [`clip_to_boundary`]'s argument does NOT port and a torus face's
///   box is the boundary hull only when no critical point lies in the
///   face. That is a statement about the face's chart REGION, so the
///   arm needs the region — and it reads it from the boundary's
///   STORED CERTIFIED pcurves ([`TorusChartWindow`], filled by each
///   lane's own arena walk), which is a read of certified data
///   exactly as `EdgeCurve::params` is for a conic edge's arc.
///
///   The closed form needs `R + r·cos v ≥ 0`, the ring convention.
///   Both lanes decide it at the body's scalar under the band through
///   its one home (`geom::is_ring_torus`), so they take the same arm;
///   at a bracketed scalar a decided convention holds over the whole
///   bracket, so every torus the bracket lane's spans admit is a
///   ring. Where it is not decided, the face keeps the whole tube,
///   the coarser box the sphere arm's ball is to its rectangle.
///
/// **Every surface kind has an arm**, and that is a statement this
/// enum makes at the type level: there is no `NoSoundBox` on the face
/// side, so a kind added to [`Surface`] cannot acquire a box by
/// falling through a wildcard, and none can be silently left without
/// one either. A face whose surface key does not RESOLVE is a
/// different answer — a torn body, on which the callers here panic
/// rather than folding it in here. A box can still
/// come out POISON (an unboxable boundary edge, a poisoned
/// description); that is the value, not the rule.
///
/// A ball-falling [`SphereWindow`](Self::SphereWindow), a windowless
/// [`TorusWindow`](Self::TorusWindow) and the conic-fed
/// [`BoundaryHull`](Self::BoundaryHull) claim more than the trimmed
/// face occupies on purpose — a cheap SUPERSET is what the contract
/// asks for, and no cheaper one is known per kind. That looseness is
/// not free (module docs: three of four doors read it as a refusal),
/// so it is bounded rather than open-ended:
/// the six `the_*_arms_box_is_exactly_the_construction_its_rule_states`
/// rows below pin every arm to exactly the construction stated here.
pub(crate) enum FaceBoxRule<'a, T: Real> {
    /// Hull the boundary's certified loci — see the type docs.
    BoundaryHull,
    /// The axial slab widened perpendicular to the axis by the radius
    /// — see the type docs.
    CylinderSlab {
        /// The `v = 0` point on the axis.
        origin: Point3<T>,
        /// The carrier's axis, its length decided at the read.
        axis: UnitVec3<T>,
        /// The cylinder's radius.
        radius: T,
    },
    /// The same slab with the generator's own radius — see the type
    /// docs.
    ///
    /// **No boundary clip, and that is a fence rather than an
    /// oversight.** The chart argument [`clip_to_boundary`] rests on
    /// holds here verbatim — azimuth is a coordinate on a cone chart
    /// too, so a cone face's perpendicular footprint is likewise
    /// contained in its boundary's, and the same intersection would be
    /// sound. What stops it is jurisdiction: the cone lanes are a
    /// different unit's territory, every door that reads a cone box
    /// today refuses on the KIND before the box's width can matter, and
    /// tightening a box under a lane nobody is measuring buys nothing
    /// and moves baselines that unit will have to re-derive. Apply it
    /// with the cone lane, not before.
    ConeSlab {
        /// The apex (`v = 0`).
        apex: Point3<T>,
        /// The unit axis direction.
        axis: Vec3<T>,
        /// The half-angle α ∈ (0, π/2).
        half_angle: T,
    },
    /// The face's chart rectangle or latitude zone, or the whole ball
    /// — see the type docs.
    SphereWindow {
        /// The sphere's centre.
        center: Point3<T>,
        /// Its radius.
        radius: T,
    },
    /// The boundary's chart rectangle in closed form, meeting the
    /// whole tube — see the type docs.
    TorusWindow {
        /// The torus centre.
        center: Point3<T>,
        /// The unit axis direction.
        axis: Vec3<T>,
        /// The major radius `R`.
        major_radius: T,
        /// The minor radius `r`.
        minor_radius: T,
        /// The chart's `u = 0` direction — the seam meridian. The
        /// window is read in THIS frame, so the arm carries it
        /// rather than re-deriving a basis of its own.
        u_ref: Vec3<T>,
    },
    /// The control net's hull — see the type docs.
    ControlNet(&'a NurbsSurface<T>),
}

/// The [`FaceBoxRule`] for a surface — the single place a surface (and,
/// for a cylinder, the band its axis is decided under) picks its rule. A
/// kind added to [`Surface`] gets its arm by being written here, never
/// by falling through a wildcard in some consumer. Takes a RESOLVED
/// surface: a missing one is a torn body, which is the caller's to
/// meet and not a rule.
///
/// The cylinder's axis is decided here, under [`BOX_CYLINDER_AXIS`]:
/// [`slab_extent`] reads it as a unit direction and a carrier's axis is
/// unit only by the surfaces' at-rest convention, which no tier
/// certifies. Its length is a pure number, levered by the radius, the
/// arm the slab swings the axis by.
///
/// **Where that decision does not land, the face takes the boundary
/// hull**, which contains a cylinder face by the ruled argument in the
/// [`FaceBoxRule`] docs. The slab only tightens that hull (or stands in
/// for a poisoned one), so the fallback gives up tightness, never
/// soundness. It needs the axis to
/// be a direction and nothing more — no length, no lever — so a radius
/// the band reads as zero or in band, which tier 3 admits (it asks
/// `radius > 0` exactly), boxes rather than refuses. The lever's
/// escalation is then spliced superseded: the box does not rest on it.
///
/// # Errors
///
/// The [`LeveredUnitError`](geom_core::LeveredUnitError) of a cylinder
/// axis that is no direction at all (zero or non-finite) — a broken
/// cylinder carrier tier 3 refuses at rest, which each lane answers in
/// its own fail-loud direction.
pub(crate) fn face_box_rule<T: Decide>(
    surface: &Surface<T>,
    band: Band,
) -> Result<FaceBoxRule<'_, T>, geom_core::LeveredUnitError> {
    Ok(match surface {
        Surface::Plane { .. } => FaceBoxRule::BoundaryHull,
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (read, log) = geom_core::k_stats::detached(|| {
                UnitVec3::levered(
                    *axis,
                    BOX_CYLINDER_AXIS,
                    band,
                    geom_core::Lever::curvature_radius(*radius),
                )
            });
            match read {
                Ok(unit) => {
                    geom_core::k_stats::splice(log);
                    FaceBoxRule::CylinderSlab {
                        origin: *origin,
                        axis: unit,
                        radius: *radius,
                    }
                }
                Err(_) if crate::validate::is_direction(axis) => {
                    geom_core::k_stats::splice_superseded(log);
                    FaceBoxRule::BoundaryHull
                }
                Err(e) => {
                    geom_core::k_stats::splice(log);
                    return Err(e);
                }
            }
        }
        Surface::Sphere { center, radius, .. } => FaceBoxRule::SphereWindow {
            center: *center,
            radius: *radius,
        },
        Surface::Nurbs(patch) => FaceBoxRule::ControlNet(patch),
        // The fit IS the face's locus, so the fit's control hull is a
        // genuine superset of it — the same rule, on the same net. The
        // certificate bounds the fit's distance from the DESCRIPTION,
        // which is a statement about intent and has no business
        // widening a box around the geometry that is actually there.
        Surface::Approx(a) => FaceBoxRule::ControlNet(a.fit()),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => FaceBoxRule::ConeSlab {
            apex: *apex,
            axis: *axis,
            half_angle: *half_angle,
        },
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => FaceBoxRule::TorusWindow {
            center: *center,
            axis: *axis,
            major_radius: *major_radius,
            minor_radius: *minor_radius,
            u_ref: *u_ref,
        },
    })
}

/// The K funnel name of a sphere's polar axis read as a unit direction,
/// before its chart window is read in closed form ([`sphere_window`]).
pub(crate) const BOX_SPHERE_AXIS: &str = "bool_box_sphere_axis";

/// The K funnel name of a sphere's seam direction against its polar
/// axis: their cosine, levered by the radius, decided zero before the
/// azimuth window is read in the frame they span ([`sphere_reach`]).
pub(crate) const BOX_SPHERE_SEAM: &str = "bool_box_sphere_seam";

/// What [`sphere_window`] reads a sphere face's chart window as:
/// latitudes in radians on `[−π/2, π/2]`, azimuths measured from
/// `u_ref` about the axis.
enum SphereWindow<T: Real> {
    /// The face is the chart rectangle `az × lat` its boundary spans.
    Rect {
        axis: UnitVec3<T>,
        /// The azimuth window, with the chart's `u_ref` and
        /// `axis × u_ref`.
        az: (Span<T>, Vec3<T>, Vec3<T>),
        lat: Span<T>,
    },
    /// The face lies in the latitude zone `lat`: it attains every
    /// azimuth of its latitudes, or its chart frame is not decided.
    Zone { axis: UnitVec3<T>, lat: Span<T> },
    /// No window is read, and the face lies in the whole ball.
    Ball,
}

/// The K funnel name of a sphere's seam direction read as a unit
/// direction, before the azimuth window is read in the frame it spans
/// ([`sphere_window`]).
pub(crate) const BOX_SPHERE_SEAM_UNIT: &str = "bool_box_sphere_seam_unit";

/// **A sphere face's reach, in `frame`**: the support of its chart
/// rectangle along each coordinate ([`torus_rect_extent`] with `R = 0`),
/// or of its latitude zone, where [`sphere_window`] reads one, and the
/// whole ball `center ± r` otherwise. Both box lanes read it: the census
/// at its own scalar, and [`face_box`] at the body's scalar before it
/// brackets out to `f64`.
///
/// A sphere point is `c + r·(cos v·ê(u) + sin v·â)`, so along a unit `e`
/// the factor `r·cos v` of the azimuth term is non-negative on the
/// chart's `[−π/2, π/2]`: the best azimuth does not depend on the
/// latitude, which is the ring torus's argument with `R = 0`. A zone
/// reads the whole turn there.
///
/// The window's ends come through `atan2`, `cos` and `sqrt`, which a
/// scalar without outward rounding (`f64`, whose `lo()`/`hi()` are
/// identities) leaves a few ulps of `|c| + r` either side of the exact
/// support. Each end is therefore charged [`SPHERE_REACH_ULPS`] ulps of
/// `|c| + r` outward, which bounds every step's rounding only because
/// no step cancels: the zone's share of the axis's normal plane is the
/// norm of the axis's other two components, not `√(1 − a²)`. The ball is not: it is the one-rounding `c ± r`
/// every kind's whole-carrier arm reads.
///
/// # Panics
///
/// As [`sphere_window`].
pub(crate) fn sphere_reach<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
    frame: &BoxFrame<T>,
) -> (Point3<T>, Point3<T>) {
    let f = proven(&body.faces, face, EntityId::Face);
    let surface = body.face_surface_linked(face, f);
    let Surface::Sphere { center, radius, .. } = surface else {
        unreachable!("sphere_reach reads a sphere face")
    };
    let c = frame.point(*center);
    let (axis, az, lat) = match sphere_window(body, face, f, surface, band) {
        SphereWindow::Ball => {
            let ball = ball_extent(&SpanBox::point(c), *radius);
            return (
                Point3::new(ball.x.lo, ball.y.lo, ball.z.lo),
                Point3::new(ball.x.hi, ball.y.hi, ball.z.hi),
            );
        }
        SphereWindow::Rect { axis, az, lat } => (axis, Some(az), lat),
        SphereWindow::Zone { axis, lat } => (axis, None, lat),
    };
    let a = frame.vector(axis.get());
    let at = |c: T, k: usize| {
        let pick = |v: Vec3<T>| [v.x, v.y, v.z][k];
        let ax = pick(a);
        let (p, q, u) = match az {
            Some((u, u_ref, v_ref)) => (pick(frame.vector(u_ref)), pick(frame.vector(v_ref)), u),
            // The whole turn: any frame of the axis's normal plane, of
            // which the coordinate's share is `√(1 − a²)`, read as the
            // axis's other two components' norm. `1 − a²` cancels as
            // the row nears the axis and loses the share outright below
            // `p ≈ 1e-8`, so the radial term `r·p·cos v` went missing
            // from the box: the term this reach exists to keep.
            None => (
                pick_other(a, k),
                T::zero(),
                Span {
                    lo: T::zero(),
                    hi: T::tau(),
                },
            ),
        };
        let s = torus_rect_extent(
            Span::exact(c),
            (Span::exact(p), Span::exact(q), Span::exact(ax)),
            (Span::exact(T::zero()), Span::exact(*radius)),
            (u, lat),
        );
        let slack = (c.abs() + *radius) * T::from_f64(SPHERE_REACH_ULPS * f64::EPSILON);
        (s.lo - slack, s.hi + slack)
    };
    let ((xl, xh), (yl, yh), (zl, zh)) = (at(c.x, 0), at(c.y, 1), at(c.z, 2));
    (Point3::new(xl, yl, zl), Point3::new(xh, yh, zh))
}

/// The ulps of `|c| + r` [`sphere_reach`] charges each end of a window's
/// support.
///
/// The steps, to first order, in ulps of `|c| + r` (an angle's error
/// moves `r·cos(…)` by at most `r` times it): the axis and seam
/// components mapped into the frame, about 1.5 each; the normal-plane
/// share and `B = √(M² + a²)`, 1.5 each (both are norms of components,
/// so neither cancels); the crest's `atan2` and each latitude end's
/// (`atan2` of a dot-product pair), up to 3.5 between them; the angle
/// difference and its `cos`, 1.5; the products by `r`, 1; the sum with
/// `c`, 0.5. That is under 12, and the charge is 16. The zone arm once
/// read its share as `√(1 − a²)`, which no ulp count bounds: it lost the
/// share outright with the axis `1e-8` off a row.
/// `sphere_rect_rows::a_zone_reach_charge_covers_random_poses_near_a_box_row`
/// measures the worst at under 2.5 over 12 000 random caps with axes
/// near a row, at ε 1e-9, 1e-6 and 1e-12.
const SPHERE_REACH_ULPS: f64 = 16.0;

/// **A sphere face's chart window**, from the window
/// `solid_contain::sphere_chart_trim` pins: its latitude extremes and
/// the azimuth window of its boundary walk.
///
/// The trim reads the window off the boundary, and the boundary of a
/// rectangle is also the boundary of its complement. The rectangle is
/// therefore taken only when the side the outer loop encodes
/// (`boundary_material_sign`: the side of the loop that holds no chart
/// pole, the reading tier 3's curved sense check runs) agrees with the
/// face's `sense`, which places the face on the rectangle's side of its
/// boundary. The trim's class then
/// holds no pole inside the face, so the face lies in the rectangle its
/// boundary spans. The side is read without the tips the chart image
/// drops: a wrap-tip cap holds the pole at its tip, outside that class,
/// and its rim alone encodes no side. A face whose side is unencoded, refused, or in
/// disagreement, one whose outer loop is a lone vertex or carries null
/// scaffolding, one outside the trim's class, one whose trim escalates,
/// and one whose axis is not decided unit all keep the
/// [`Ball`](SphereWindow::Ball). A face that wraps the azimuth alone,
/// or whose seam direction is not decided a unit normal of the axis,
/// keeps its latitude [`Zone`](SphereWindow::Zone).
///
/// `face` resolved to `f` in the caller, so its outer loop and that
/// loop's members are links ([`Body::loop_members_linked`]), and a
/// record the flattening of that loop names is one too: a flattening
/// that refuses past a cycle loop is a torn body and panics
/// ([`crate::live::OPERATORS_KEEP_LINKS`]). The trim's `CorruptFace` is
/// a record miss past `face` too, and panics the same way.
fn sphere_window<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    f: &crate::entity::Face,
    surface: &Surface<T>,
    band: Band,
) -> SphereWindow<T> {
    use super::solid_contain::PointInSolidError;
    use crate::props::LoopEdgesError;
    use geom_core::{Margin, Sign};
    let Surface::Sphere {
        center,
        radius,
        axis,
        u_ref,
    } = surface
    else {
        return SphereWindow::Ball;
    };
    let outer = linked(
        &body.loops,
        f.outer,
        EntityId::Loop,
        EntityId::Face(face),
        "outer",
    );
    if matches!(
        body.loop_members_linked(f.outer, outer).as_slice(),
        [BoundaryMember::Isolated { .. }]
    ) {
        return SphereWindow::Ball;
    }
    let outer = match crate::props::chart_image_edges(body, f.outer) {
        Ok((outer, _)) if outer.is_empty() => return SphereWindow::Ball,
        Ok((outer, _)) => outer,
        Err(LoopEdgesError::NullScaffoldEdge { .. }) => return SphereWindow::Ball,
        Err(refusal @ LoopEdgesError::Corrupt { .. }) => torn_outer_loop(body, face, f, refusal),
    };
    let side = if f.sense {
        Sign::Positive
    } else {
        Sign::Negative
    };
    if geom_brep::props::boundary_material_sign(surface, &outer, band).ok()
        != Some(geom_brep::props::MaterialSign::Encoded(side))
    {
        return SphereWindow::Ball;
    }
    let trim =
        match super::solid_contain::sphere_chart_trim(body, face, *center, *radius, *axis, band) {
            Ok(Some(trim)) => trim,
            Ok(None) | Err(PointInSolidError::Escalated { .. }) => return SphereWindow::Ball,
            Err(refusal) => torn_outer_loop(body, face, f, refusal),
        };
    let Ok(unit) = UnitVec3::new(*axis, BOX_SPHERE_AXIS, band) else {
        return SphereWindow::Ball;
    };
    let pole = |sign: T| (sign * *radius, T::zero());
    let latitude = |(h, rho): (T, T)| h.atan2(rho);
    let lat = Span {
        lo: latitude(trim.south.unwrap_or_else(|| pole(T::zero() - T::one()))),
        hi: latitude(trim.north.unwrap_or_else(|| pole(T::one()))),
    };
    let seam = UnitVec3::new(*u_ref, BOX_SPHERE_SEAM_UNIT, band)
        .ok()
        .filter(|seam| {
            matches!(
                crate::validate::decide(
                    BOX_SPHERE_SEAM,
                    Margin::levered(
                        seam.get().dot(unit.get()),
                        geom_brep::ExtentBall::of_sphere(*center, *radius).lever()
                    ),
                    band
                ),
                Ok(Sign::Zero)
            )
        });
    match trim.az.zip(seam) {
        Some(((lo, hi), seam)) => {
            let u_ref = seam.get();
            SphereWindow::Rect {
                axis: unit,
                az: (Span { lo, hi }, u_ref, unit.get().cross(u_ref)),
                lat,
            }
        }
        None => SphereWindow::Zone { axis: unit, lat },
    }
}

/// The torn hop under `refusal`, a record-miss refusal of a reading of
/// `face`'s outer loop ([`crate::props::loop_edges`],
/// `sphere_chart_trim`) past `face`, which resolved to `f`. Each hop
/// those readings take is re-read as a link, so the one that does not
/// resolve panics naming its holder, its field and both premises
/// ([`crate::live::dangling_link`]).
#[track_caller]
fn torn_outer_loop<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    f: &crate::entity::Face,
    refusal: impl core::fmt::Debug,
) -> ! {
    body.face_surface_linked(face, f);
    let outer = linked(
        &body.loops,
        f.outer,
        EntityId::Loop,
        EntityId::Face(face),
        "outer",
    );
    for member in body.loop_members_linked(f.outer, outer) {
        if let BoundaryMember::Edge { he, half, ek, edge } = member {
            body.edge_curve_linked(ek, edge);
            body.linked_vertex_point(half.start, EntityId::HalfEdge(he), "start");
            body.proven_half_edge_end(he);
        }
    }
    unreachable!(
        "the outer loop of {} refused {refusal:?} with every record on it resolved",
        EntityId::Face(face)
    )
}

/// **One coordinate of a ring torus's chart rectangle, in closed
/// form**: the least and greatest `e·S(u, v)` over `u ∈ U`, `v ∈ V`,
/// where `e` is a direction, `c` its component of the centre and
/// `(p, q, a)` its components of `u_ref`, `axis × u_ref` and the axis,
/// each a span, enclosed over every torus whose components and radii
/// lie in them.
///
/// `e·S − c = (R + r·cos v)·(p·cos u + q·sin u) + r·a·sin v`. The
/// factor `R + r·cos v` is non-negative on a ring torus (`R ≥ r ≥ 0`)
/// and on a sphere's chart (`R = 0`, `cos v ≥ 0`), so for every `v` the
/// best `u` is the one that maximizes `p·cos u + q·sin u = A·cos(u − u*)`
/// over `U`, whatever `v` is: call that greatest value `M`
/// ([`most_cos`]). What is left is
/// `R·M + r·(M·cos v + a·sin v) = R·M + r·B·cos(v − v*)` with
/// `B = √(M² + a²)`, maximized over `V` the same way. The least value is
/// the same reading of `−e`. At a point every step is an equality, so
/// the extent is the rectangle's own and turns with it.
///
/// **Over spans, by three facts and no pad.** `M` is a maximum of
/// functions linear in `(p, q)`, so it is convex there and greatest at
/// one of the span box's four corners, which are all read. The greatest
/// `e·S − c` is `max_v [M·(R + r·cos v) + r·a·sin v]`: non-decreasing in
/// `M`, each function's slope being the non-negative factor, so it is
/// read at the greatest `M`; and a maximum of functions linear in `a`,
/// so it is greatest at one of `a`'s two ends. It is then
/// `R·M + r·B`, linear in each radius, which the span product reads at
/// their ends, `r ≥ 0` letting `B` be taken at its greater end first.
/// The rounding is the module's undirected arithmetic ([`Span`]).
pub(crate) fn torus_rect_extent<T: Decide>(
    c: Span<T>,
    (p, q, a): (Span<T>, Span<T>, Span<T>),
    (major, minor): (Span<T>, Span<T>),
    (u, v): TorusWindowPair<T>,
) -> Span<T> {
    let most = |p: Span<T>, q: Span<T>, a: Span<T>| {
        let along = |p: T, q: T| (p.powi(2) + q.powi(2)).sqrt() * most_cos(q.atan2(p), u);
        let m = along(p.lo, q.lo)
            .max(along(p.lo, q.hi))
            .max(along(p.hi, q.lo).max(along(p.hi, q.hi)));
        let lift = |a: T| (m.powi(2) + a.powi(2)).sqrt() * most_cos(a.atan2(m), v);
        let b = lift(a.lo).max(lift(a.hi));
        major.mul(Span::exact(m)).hi + minor.mul(Span::exact(b)).hi
    };
    let neg = |s: Span<T>| Span {
        lo: T::zero() - s.hi,
        hi: T::zero() - s.lo,
    };
    Span {
        lo: c.lo - most(neg(p), neg(q), neg(a)),
        hi: c.hi + most(p, q, a),
    }
}

/// The greatest `cos(t − t*)` over `t ∈ w`: one when `t*` lies in the
/// window modulo a turn, otherwise the better of the two ends, since a
/// window that misses the crest is monotone between them. The
/// membership is read branch-free (`select_le_zero` on the crest's
/// offset into the window less its width), and a crest a rounding off
/// an end gives either reading to within that rounding.
fn most_cos<T: Decide>(crest: T, w: Span<T>) -> T {
    let into = (crest - w.lo).reduce_periodic(T::tau());
    let ends = (w.lo - crest).cos().max((w.hi - crest).cos());
    (into - (w.hi - w.lo)).select_le_zero(T::one(), ends)
}

/// The face's certified box, padded — [`FaceBoxRule`]'s
/// `f64`-bracket instantiation, and therefore a genuine superset of
/// the face's locus.
///
/// The per-kind extents are [`slab_extent`] and friends, in
/// [`Span`] arithmetic: a description whose coordinates are
/// themselves brackets is ENCLOSED, never sampled at one endpoint.
/// Poison rides through as NaN ends and comes out the poison box,
/// which every door reads in its own fail-loud direction.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a cylinder whose axis
/// is no direction ([`face_box_rule`]): a broken carrier.
///
/// # Panics
///
/// Where `face`, which the caller read out of `body`, or a record on
/// the walk from it does not resolve, or a loop walk does not close
/// (D2 row 4); a torn curve is not an uncertified one. The bodies are
/// at rest or the reduction's working copies, whose links hold by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn face_box<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    pad: f64,
    band: Band,
) -> Result<Aabb, BooleanError> {
    let f = proven(&body.faces, face, EntityId::Face);
    let surface = body.face_surface_linked(face, f);
    // The axial window from the boundary's OWN locus, edge by edge
    // (each edge's cover's support either way, `cover_axial`), not from
    // the corners of a box around it: at a tilted axis a box's spread in
    // the other two coordinates leaks into the window, and the frustum's
    // radius with it. Each end is bracketed outward, and an edge with no
    // cover claims nothing: a poisoned window, which reaches the poison
    // box, since the hull here is `Real`'s and carries NaN.
    let axial_window = |axis: Vec3<T>, origin: Point3<T>| -> Option<Span<f64>> {
        let bracketed = |(lo, hi): (T, T)| Span {
            lo: lo.lo(),
            hi: hi.hi(),
        };
        let mut acc: Option<Span<f64>> = None;
        let mut grow = |s: Span<f64>| acc = Some(acc.map_or(s, |a: Span<f64>| a.hull(s)));
        for member in body.face_boundary_linked(face, f) {
            grow(match member {
                BoundaryMember::Isolated { point: p, .. } => {
                    let h = (p - origin).dot(axis);
                    bracketed((h, h))
                }
                BoundaryMember::Edge { ek, edge: e, .. } => match edge_cover_linked(body, ek, e) {
                    Ok(cover) => bracketed(cover_axial(&cover, origin, axis)),
                    Err(_) => Span::exact(geom_core::poison()),
                },
            });
        }
        acc
    };
    // The torus arm's chart window, read through the ONE walk
    // (`face_window_steps` -> `torus_chart_window`) the census lane and
    // the construction rows also enter, so no fail mode can drift
    // between them.
    //
    // The window is accumulated at the BODY's scalar and read out at
    // this lane's `f64` brackets, each end taken outward
    // (`lo()`/`hi()`), so a bracketed cache widens the window rather
    // than narrowing it.
    let chart_window = |major: T, minor: T| -> Option<TorusWindowPair<f64>> {
        let steps = face_window_steps(body, face);
        torus_chart_window(&steps, major, minor).map(|(u, v)| {
            (
                Span {
                    lo: u.lo.lo(),
                    hi: u.hi.hi(),
                },
                Span {
                    lo: v.lo.lo(),
                    hi: v.hi.hi(),
                },
            )
        })
    };
    let rule = face_box_rule(surface, band).map_err(|_| BooleanError::ClassificationInvariant {
        what: "face box: a cylinder's axis is no direction (a broken cylinder carrier)",
    })?;
    let boxed = match rule {
        FaceBoxRule::ControlNet(patch) => geom::surfaces::boxes::nurbs_surface_aabb(patch),
        // The reach is read at the body's scalar and bracketed out, end
        // by end, as the torus window is.
        FaceBoxRule::SphereWindow { .. } => {
            let (lo, hi) = sphere_reach(body, face, band, &BoxFrame::World);
            Aabb {
                min_x: lo.x.lo(),
                min_y: lo.y.lo(),
                min_z: lo.z.lo(),
                max_x: hi.x.hi(),
                max_y: hi.y.hi(),
                max_z: hi.z.hi(),
            }
        }
        FaceBoxRule::TorusWindow {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let (c, ax) = (bracket_point(center), bracket_vector(axis));
            let whole = torus_extent(&c, &ax, major_radius.hi(), minor_radius.hi());
            // BOTH ends of each radius bracket: the extent encloses the
            // whole family of tori they admit, each a ring once the
            // convention is decided at the body's scalar.
            let radii = (
                Span {
                    lo: major_radius.lo(),
                    hi: major_radius.hi(),
                },
                Span {
                    lo: minor_radius.lo(),
                    hi: minor_radius.hi(),
                },
            );
            match geom::is_ring_torus(major_radius, minor_radius, band)
                .then(|| chart_window(major_radius, minor_radius))
                .flatten()
            {
                None => aabb_of(whole),
                Some(window) => aabb_of(meet(
                    ring_window_extent(
                        &c,
                        (
                            &bracket_vector(u_ref),
                            &bracket_vector(axis.cross(u_ref)),
                            &ax,
                        ),
                        radii,
                        window,
                    ),
                    whole,
                )),
            }
        }
        FaceBoxRule::CylinderSlab {
            origin,
            axis,
            radius,
        } => {
            let Some(h) = axial_window(axis.get(), origin) else {
                return Ok(Aabb::poison());
            };
            let slab = aabb_of(slab_extent(
                &bracket_point(origin),
                &UnitSpanBox::bracketed(axis),
                h,
                radius.hi(),
            ));
            // The slab is the full 2π ring at every height in the
            // window; the face is a PATCH of it. Clipping the ring to
            // the boundary's own box is what makes the box trim-scoped
            // in azimuth as well as axially — see [`clip_to_boundary`]
            // for why that is sound and why it needs no chart work.
            clip_to_boundary(slab, boundary_hull(body, face, f))
        }
        FaceBoxRule::ConeSlab {
            apex,
            axis,
            half_angle,
        } => {
            let Some(h) = axial_window(axis, apex) else {
                return Ok(Aabb::poison());
            };
            let axis_span = bracket_vector(axis);
            let apex = bracket_point(apex);
            // An UPPER bound on tan α over the half-angle's bracket:
            // α ∈ (0, π/2), where sin increases and cos decreases, so
            // `sin.hi / cos.lo` dominates. A bracket reaching π/2 has
            // `cos.lo ≤ 0`, and the quotient then poisons the box —
            // the honest answer for a description that may be a plane.
            let (sin, cos) = half_angle.sin_cos();
            aabb_of(cone_frustum_extent(
                &apex,
                &axis_span,
                h,
                sin.hi() / cos.lo(),
            ))
        }
        FaceBoxRule::BoundaryHull => boundary_hull(body, face, f).unwrap_or_else(Aabb::poison),
    };
    Ok(boxed.padded(pad))
}

/// A [`SpanBox`] of `f64` spans as the [`Aabb`] every door reads.
/// **Clip a cylinder wall's ring-shaped slab to its boundary's box.**
///
/// The slab arm covers the WHOLE turn at every height in the face's
/// axial window, because the axial coordinate is what the boundary
/// pins exactly. The azimuth is not pinned there at all, so a corner
/// round — a quarter turn — is boxed as the whole carrier cylinder,
/// and every door that reads the box pays for three quarters of a ring
/// the face does not occupy. That is the mechanism behind #347's
/// measured bound: a pocket wall 2 mm clear of a corner ARC became a
/// candidate against the round because it entered the round's
/// CARRIER.
///
/// **Why the boundary's box may be intersected in**, on every
/// coordinate: a cylinder face lies in its boundary's hull because the
/// cylinder is ruled ([`FaceBoxRule`]'s `BoundaryHull` arm carries the
/// argument), so slab ∩ hull is still a superset of the FACE's locus,
/// which is the module contract. It is derived from boxes this arm
/// already walks: no chart image, no azimuth window, no `Band`, and no
/// new way for a box to escalate.
///
/// A face with no boundary, or one whose hull came out POISON, keeps
/// the slab: poison overlaps everything by design, and quietly
/// intersecting it away would turn "no cheap superset is known" into a
/// silent claim.
///
/// **The two lanes diverge here, and only here.** This lane TESTS for
/// poison and keeps the slab; the census lane's mirror clamps with
/// `min`/`max` against its own `boundary_reach`, and a NaN end there
/// propagates through the clamp rather than being stepped around. Both
/// are sound — one keeps the wider box, the other keeps the poison —
/// and neither can narrow a real locus away. It is recorded rather
/// than unified because the two lanes answer differently for a face
/// with no claim in it at all (`Aabb::poison()` versus `None`), which
/// is a difference that predates this clip and belongs to the census's
/// own contract.
fn clip_to_boundary(slab: Aabb, hull: Option<Aabb>) -> Aabb {
    let Some(h) = hull else { return slab };
    let finite = [h.min_x, h.min_y, h.min_z, h.max_x, h.max_y, h.max_z]
        .into_iter()
        .all(f64::is_finite);
    if !finite {
        return slab;
    }
    Aabb {
        min_x: slab.min_x.max(h.min_x),
        min_y: slab.min_y.max(h.min_y),
        min_z: slab.min_z.max(h.min_z),
        max_x: slab.max_x.min(h.max_x),
        max_y: slab.max_y.min(h.max_y),
        max_z: slab.max_z.min(h.max_z),
    }
}

fn aabb_of(s: SpanBox<f64>) -> Aabb {
    Aabb {
        min_x: s.x.lo,
        min_y: s.y.lo,
        min_z: s.z.lo,
        max_x: s.x.hi,
        max_y: s.y.hi,
        max_z: s.z.hi,
    }
}

/// The hull of the face boundary's own certified boxes — every
/// boundary edge's [`edge_box`], plus the isolated-vertex loops, which
/// have no edge to speak for them. `None` for a face with no boundary
/// at all.
///
/// Poison propagates: [`Aabb::hull`] carries NaN by construction, so
/// one unboxable boundary edge poisons the face rather than quietly
/// contributing nothing.
///
/// This is the [`FaceBoxRule::BoundaryHull`] arm, and it is also what
/// the [`FaceBoxRule::CylinderSlab`] arm reads its axial range from —
/// the axial coordinate is linear along the surface, so the face's
/// axial extremes lie on the boundary, but not necessarily at a
/// boundary VERTEX.
fn boundary_hull<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    f: &crate::entity::Face,
) -> Option<Aabb> {
    let mut acc: Option<Aabb> = None;
    let mut grow = |x: Aabb| acc = Some(acc.map_or(x, |a: Aabb| a.hull(&x)));
    for member in body.face_boundary_linked(face, f) {
        grow(match member {
            BoundaryMember::Isolated { point: p, .. } => {
                Aabb::from_points([p]).unwrap_or_else(Aabb::poison)
            }
            BoundaryMember::Edge { ek, .. } => edge_box(body, ek, 0.0),
        });
    }
    acc
}

/// **Topo's reading of the region an edge can occupy** (DESIGN
/// `[one-way.places]`, "The region a carrier, an edge or a face can
/// occupy"): its certified carrier's cover over its span
/// ([`geom::Curve3::cover_over`], which holds the one kind match),
/// hulled with its two end vertices, which sit on the carrier only to
/// its certification residual. Every bound on where an edge is reads
/// this ([`edge_box`], the census's `edge_reach_in`, the containment,
/// shell and reduction doors) through the cover's methods, and none
/// matches on the carrier's kind.
///
/// # Errors
///
/// [`EdgeUncovered`]: null scaffolding, whose chord is not a bound; a
/// carrier its own reading cannot cover; an end vertex that is not a
/// finite point.
pub(crate) fn edge_cover_of<T: SpanLocate>(
    certified: Option<&geom_brep::EdgeCurve<T>>,
    ends: (Point3<T>, Point3<T>),
) -> Result<Cover<T>, EdgeUncovered> {
    let curve = certified.ok_or(EdgeUncovered::Scaffold)?;
    edge_cover_over(curve.carrier(), curve.params(), ends)
}

/// **[`edge_cover_of`]'s construction**, from a carrier, its span and
/// the edge's two ends: the carrier's cover over the span hulled with
/// the ends. Read alone where there is no certified edge record to read
/// it from: rows whose near-fold sector `mev`'s witness certification
/// refuses.
///
/// # Errors
///
/// As [`edge_cover_of`], save [`EdgeUncovered::Scaffold`].
pub(crate) fn edge_cover_over<T: SpanLocate>(
    carrier: &geom::Curve3<T>,
    span: (T, T),
    (a, b): (Point3<T>, Point3<T>),
) -> Result<Cover<T>, EdgeUncovered> {
    let arc = arc_cover(carrier, span)?;
    let ends = Cover::points(vec![a, b]).ok_or(EdgeUncovered::Ends)?;
    Ok(arc.hull(ends))
}

/// **The carrier's half of an edge's cover**: its cover over the span
/// ([`geom::Curve3::cover_over`]), with no ends. The one place topo
/// reads a carrier over an edge's span; [`edge_cover_over`] hulls it
/// with the ends. Read alone by a reader that holds a lifted carrier and
/// span with no edge record behind them (`cell_region`'s boundary arcs,
/// through `splitting::containment::arc_distance_floor`), which reads
/// its ends as vertices of its own.
///
/// # Errors
///
/// [`EdgeUncovered::Carrier`].
pub(crate) fn arc_cover<T: SpanLocate>(
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
) -> Result<Cover<T>, EdgeUncovered> {
    carrier
        .cover_over(ParamWindow::new(t0, t1))
        .map_err(EdgeUncovered::Carrier)
}

/// **How far `edge` reaches from `pivot`, bounded from both sides**
/// ([`geom_brep::edge_reach_bounds`]), its bound from above the far
/// distance of the edge's cover ([`edge_cover_linked`]) from `pivot`,
/// poison where the edge has no cover, which the reading levered at it
/// escalates. `None` for null scaffolding.
pub(crate) fn edge_reach_bounds<T: SpanLocate>(
    body: &Body<T>,
    edge: EdgeKey,
    e: &Edge,
    pivot: Point3<T>,
) -> Option<geom_brep::LeverBounds<T>> {
    let curve = body.edge_curve_linked(edge, e).certified()?;
    let cover = edge_cover_linked(body, edge, e).ok();
    Some(reach_bounds_in(
        curve.carrier(),
        curve.params(),
        cover.as_ref(),
        pivot,
    ))
}

/// **How far a piece of an edge reaches from `pivot`, bounded from both
/// sides**: the piece of `carrier` over `span` from a point inside the
/// edge to one of its vertices (`overlap`'s half-turns), its region the
/// edge cover's construction over the piece ([`edge_cover_over`]), read
/// as [`edge_reach_bounds`] reads the whole edge's.
pub(crate) fn piece_reach_bounds<T: SpanLocate>(
    carrier: &geom::Curve3<T>,
    span: (T, T),
    ends: (Point3<T>, Point3<T>),
    pivot: Point3<T>,
) -> geom_brep::LeverBounds<T> {
    let cover = edge_cover_over(carrier, span, ends).ok();
    reach_bounds_in(carrier, span, cover.as_ref(), pivot)
}

/// The reach bounds of an arc whose region is `cover`: from above its
/// far distance from `pivot` ([`geom_brep::ExtentBall::of_cover_from`]).
fn reach_bounds_in<T: SpanLocate>(
    carrier: &geom::Curve3<T>,
    span: (T, T),
    cover: Option<&Cover<T>>,
    pivot: Point3<T>,
) -> geom_brep::LeverBounds<T> {
    let above = geom_brep::ExtentBall::of_cover_from(cover, pivot).lever_from(pivot);
    geom_brep::edge_reach_bounds(carrier, span, pivot, above)
}

/// [`edge_cover_of`] for `edge`, its ends read through its half-edges.
///
/// # Errors
///
/// As [`edge_cover_of`].
///
/// # Panics
///
/// Where `edge`, which the caller read out of `body`, its curve or a
/// record on the way to its ends does not resolve (D2 row 4): a torn
/// curve is not an uncertified one.
pub(crate) fn edge_cover<T: SpanLocate>(
    body: &Body<T>,
    edge: EdgeKey,
) -> Result<Cover<T>, EdgeUncovered> {
    edge_cover_linked(body, edge, proven(&body.edges, edge, EntityId::Edge))
}

impl<T: SpanLocate> Body<T> {
    /// **The region `edge` can occupy** (`[one-way.places]`): its
    /// certified carrier's cover over its span hulled with its two end
    /// vertices, topo's one reading of an edge ([`edge_cover_of`]), for
    /// a reader outside topo.
    ///
    /// # Errors
    ///
    /// [`EdgeUncovered`], as [`edge_cover_of`].
    ///
    /// # Panics
    ///
    /// Where `edge`, a live edge of `self`, or a record on the way to its
    /// ends does not resolve (D2 row 4).
    pub fn edge_cover(&self, edge: EdgeKey) -> Result<Cover<T>, EdgeUncovered> {
        edge_cover(self, edge)
    }
}

/// [`edge_cover`] past its edge's lookup: `e` is `edge`'s record.
///
/// # Errors
///
/// As [`edge_cover_of`].
pub(crate) fn edge_cover_linked<T: SpanLocate>(
    body: &Body<T>,
    edge: EdgeKey,
    e: &Edge,
) -> Result<Cover<T>, EdgeUncovered> {
    let ends = (
        edge_end_point(body, edge, e.he_plus, "he_plus"),
        edge_end_point(body, edge, e.he_minus, "he_minus"),
    );
    edge_cover_of(body.edge_curve_linked(edge, e).certified(), ends)
}

/// **Why an edge has no cover** ([`edge_cover_of`]). A reader states its
/// own direction on it: a pruning reader takes the box that overlaps
/// everything, a granting reader refuses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EdgeUncovered {
    /// Null scaffolding: nothing is certified about the locus, and the
    /// chord between its ends is not a bound.
    Scaffold,
    /// The certified carrier's own reading refused.
    Carrier(geom::cover::Uncovered),
    /// An end vertex's point is not a finite number.
    Ends,
}

impl core::fmt::Display for EdgeUncovered {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Scaffold => f.write_str("the edge is null scaffolding"),
            Self::Carrier(why) => write!(f, "the edge's carrier has {why}"),
            Self::Ends => f.write_str("an end vertex of the edge is not a finite point"),
        }
    }
}

/// The cover's frame for a [`BoxFrame`]: the world's, or its rows. An
/// adapter for the face lanes, which still read their boxes in a
/// [`BoxFrame`].
pub(crate) fn cover_frame<T: Real>(frame: &BoxFrame<T>) -> geom::cover::Frame<T> {
    match frame {
        BoxFrame::World => geom::cover::Frame::World,
        BoxFrame::Aimed(rows) => geom::cover::Frame::Rows(*rows),
    }
}

/// An edge's range along `axis` about `origin`, from its cover's support
/// either way: the axial window a slab face reads off its boundary. An
/// adapter for the face lanes' slab arms.
pub(crate) fn cover_axial<T: Real>(cover: &Cover<T>, origin: Point3<T>, axis: Vec3<T>) -> (T, T) {
    let base = (origin - Point3::origin()).dot(axis);
    (
        T::zero() - cover.support(-axis) - base,
        cover.support(axis) - base,
    )
}

/// The box of the ball about `c` of radius `r`, from their enclosures,
/// padded by `pad`.
pub(crate) fn centred_box<T: Bounds>(c: Point3<T>, r: T, pad: f64) -> Aabb {
    let r = r.hi();
    Aabb {
        min_x: c.x.lo() - r,
        min_y: c.y.lo() - r,
        min_z: c.z.lo() - r,
        max_x: c.x.hi() + r,
        max_y: c.y.hi() + r,
        max_z: c.z.hi() + r,
    }
    .padded(pad)
}

/// The edge's box, padded: its cover's bracketed world box
/// ([`edge_cover`], [`Cover::aabb`]), or the poison box where it has no
/// cover.
///
/// # Panics
///
/// Where `edge`, which the caller read out of `body`, its curve or a
/// record on the way to its ends does not resolve (D2 row 4): a torn
/// curve is not an uncertified one. The bodies are at rest (the census,
/// the separation and the operand gate) or the reduction's working
/// copies, whose links hold by [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn edge_box<T: Decide + Bounds>(body: &Body<T>, edge: EdgeKey, pad: f64) -> Aabb {
    edge_cover(body, edge).map_or_else(|_| Aabb::poison(), |c| c.aabb(pad))
}

/// The start point of `edge`'s half `he`, which `edge`'s field `field`
/// names: every hop is a link.
#[track_caller]
pub(crate) fn edge_end_point<T: Real>(
    body: &Body<T>,
    edge: EdgeKey,
    he: HalfEdgeKey,
    field: &'static str,
) -> Point3<T> {
    let start = linked(
        &body.half_edges,
        he,
        EntityId::HalfEdge,
        EntityId::Edge(edge),
        field,
    )
    .start;
    body.linked_vertex_point(start, EntityId::HalfEdge(he), "start")
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub(crate) mod tests {
    //! **Two contracts, and they need opposite assertions.**
    //!
    //! The **locus** rows sample the face's true locus and assert the
    //! box contains every sample. The spans are swept rather than
    //! chosen, so a rule that drops part of the bulge reds and not
    //! only one that drops all of it, and no single fixture can be
    //! the reason a row passes.
    //!
    //! **That family degrades in ONE direction only, and saying
    //! otherwise is the thing S66 exists to refute.** `holds(&b, p)`
    //! is satisfied by any bigger box, so a `face_box` returning
    //! `[-1e300, 1e300]` on every arm that has one passes every locus
    //! row here — and passes the whole `topo` lib suite. Over-width
    //! is not slower work at three of this module's four doors
    //! (module docs); it is a refusal, and therefore an answer. A
    //! suite of locus rows is **not** an adequate guard for this
    //! module, and a reader who leaves this header believing it is
    //! has been told the wrong thing.
    //!
    //! So the **ceiling** rows —
    //! `the_*_arms_box_is_exactly_the_construction_its_rule_states` —
    //! state each arm's box as a formula in the fixture's own
    //! parameters and pin it on all six faces in BOTH directions.
    //! Neither family subsumes the other: the locus rows check the
    //! rule against the geometry, the ceiling rows check the code
    //! against the rule.
    //!
    //! `every_door_that_reads_a_box_is_inventoried` is neither — it
    //! reads source, not geometry, and guards the module docs' door
    //! list rather than any box.

    use super::*;
    use crate::boolean::conic_oracle::exact::Dd;

    /// **A spline edge's axial span from its cover holds the curve**
    /// ([`cover_axial`], about an origin off the curve): random clamped nets of degree one
    /// to three, rational with weights from 0.02 to 50, a random axis and
    /// sub-interval, the curve's height along the axis sampled inside the
    /// span to 1e-12 (a counterexample search; seed logged).
    #[test]
    fn a_hull_axial_span_holds_the_curve() {
        use geom_core::spline::KnotVector;
        let mut rng = test_utils::fuzz::start("boxes::tests::a_hull_axial_span_holds_the_curve");
        for _ in 0..test_utils::fuzz::scaled(400) {
            let p = 1 + rng.below(3);
            let n = p + 1 + rng.below(5);
            let mut k = vec![0.0; p + 1];
            let mut ks: Vec<f64> = (0..n - p - 1).map(|_| rng.range(0.05, 0.95)).collect();
            ks.sort_by(f64::total_cmp);
            k.extend(ks);
            k.extend(vec![1.0; p + 1]);
            let ctl: Vec<_> = (0..n)
                .map(|_| Point3::new(rng.range(-1., 1.), rng.range(-1., 1.), rng.range(-1., 1.)))
                .collect();
            let w: Vec<f64> = (0..n).map(|_| rng.log_uniform(0.02, 50.0)).collect();
            let Ok(net) = geom::NurbsCurve3::new(KnotVector::clamped(k, p).unwrap(), ctl, w) else {
                continue;
            };
            let ax =
                Vec3::new(rng.range(-1., 1.), rng.range(-1., 1.), rng.range(-1., 1.)).normalize();
            let o = Point3::new(0.3, -0.7, 0.2);
            let (a, b) = (rng.range(0., 0.5), rng.range(0.5, 1.));
            let ends = (net.eval(a), net.eval(b));
            let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(net.clone()));
            let cover = carrier
                .cover_over(ParamWindow::new(a, b))
                .unwrap()
                .hull(Cover::points(vec![ends.0, ends.1]).unwrap());
            let (lo, hi) = cover_axial(&cover, o, ax);
            let span = Span { lo, hi };
            for i in 0..=200 {
                let h = (net.eval(a + (b - a) * f64::from(i) / 200.) - o).dot(ax);
                assert!(
                    span.lo - 1e-12 <= h && h <= span.hi + 1e-12,
                    "{h} outside [{}, {}]; {}",
                    span.lo,
                    span.hi,
                    test_utils::fuzz::replay()
                );
            }
        }
    }
    use crate::entity::LoopBoundary;
    use crate::euler::{FaceSurface, MefSite, MevSite};
    use geom::Curve3;
    use geom::Surface;
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
    use geom_core::{Interval, Tol};
    use geom_core::{Point3, Vec3};

    /// The pad every row boxes with — the sweep's own, so a row that
    /// only passes because of a generous pad would have to say so.
    fn witness_band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn pad() -> f64 {
        sweep_pad(witness_band())
    }

    /// `p` is inside `b` — the containment the contract promises.
    fn holds(b: &Aabb, p: Point3<f64>) -> bool {
        p.x >= b.min_x
            && p.x <= b.max_x
            && p.y >= b.min_y
            && p.y <= b.max_y
            && p.z >= b.min_z
            && p.z <= b.max_z
    }

    fn plane_z0() -> Surface<f64> {
        Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        }
    }

    fn cyl_r(r: f64) -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        }
    }

    /// A PLANAR face whose rim is a circular arc: the sector of radius
    /// `r` spanning `[0, span]` in azimuth, closed by two radii. This
    /// is the shape the plane×cylinder lane mints as a cylinder's cap,
    /// and the shape whose locus leaves its boundary-vertex hull.
    ///
    /// Returns the body and the sector face.
    fn arc_sector(r: f64, span: f64) -> (Body<f64>, FaceKey) {
        arc_sector_from(r, span, 0.0)
    }

    /// [`arc_sector`] with the rim carrier's `u_ref` rotated by `phi`
    /// in the plane: the SAME circle and the same arc, named from a
    /// different reference direction, with the parameter range shifted
    /// so the endpoints are unchanged. That is not a contrivance — the
    /// split lane mints rims this way, and `cyl_wall`'s descending rim
    /// below is one — and a per-coordinate triangle bound
    /// `|û_i|·a + |v̂_i|·b` is not invariant under it; the arc cover's
    /// amplitude is.
    fn arc_sector_from(r: f64, span: f64, phi: f64) -> (Body<f64>, FaceKey) {
        circle_sector(r, 0.0, span, phi)
    }

    /// [`arc_sector_from`] with the arc's world run `[start, start +
    /// span]` rather than `[0, span]`: the extrema a run crosses are
    /// chosen by where it starts, and the adopted-arm rows below need
    /// runs that cross one strictly inside.
    fn circle_sector(r: f64, start: f64, span: f64, phi: f64) -> (Body<f64>, FaceKey) {
        let on = |t: f64| Point3::new(r * t.cos(), r * t.sin(), 0.0);
        let (a, b, c) = (on(start), on(start + span), Point3::origin());
        let plane_surface = plane_z0();
        let cyl_surface = cyl_r(r);
        let carrier = Curve3::circle(
            Point3::origin(),
            Vec3::unit_z(),
            r,
            Vec3::new(phi.cos(), phi.sin(), 0.0),
        );
        conic_sector(
            plane_surface,
            cyl_surface,
            carrier,
            (start - phi, start + span - phi),
            on(start + span * 0.5),
            (a, b, c),
        )
    }

    /// A planar face whose rim is an ELLIPSE arc: the radius-`r`
    /// cylinder about `z` cut by the plane through the origin whose
    /// normal is tilted by `alpha` from `z` towards `x`. The section is
    /// the ellipse with semi-major `r / cos α` along the plane's line
    /// of steepest descent and semi-minor `r` along `y`, so the axis
    /// ratio grows without bound as `alpha → π/2` — the adversarial
    /// case for a per-coordinate amplitude. Parameters `[t0, t1]` are
    /// the eccentric anomaly; the face is closed by two chords to the
    /// origin, which lies on the plane.
    fn ellipse_sector(r: f64, alpha: f64, t0: f64, t1: f64) -> (Body<f64>, FaceKey) {
        let normal = Vec3::new(alpha.sin(), 0.0, alpha.cos());
        let u_ref = Vec3::new(alpha.cos(), 0.0, -alpha.sin());
        let carrier = Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(Point3::origin(), normal, u_ref),
            r / alpha.cos(),
            r,
        ));
        let plane_surface = Surface::Plane {
            origin: Point3::origin(),
            normal,
            u_ref,
        };
        conic_sector(
            plane_surface,
            cyl_r(r),
            carrier.clone(),
            (t0, t1),
            carrier.eval(0.5 * (t0 + t1)),
            (carrier.eval(t0), carrier.eval(t1), Point3::origin()),
        )
    }

    /// The sector body under [`circle_sector`] and [`ellipse_sector`]:
    /// one conic edge `a → b` described as the plane×cylinder
    /// intersection with the given carrier and parameter run, closed
    /// by the chords `b → c → a` on the plane.
    fn conic_sector(
        plane_surface: Surface<f64>,
        cyl_surface: Surface<f64>,
        carrier: Curve3<f64>,
        (param_start, param_end): (f64, f64),
        witness: Point3<f64>,
        (a, b, c): (Point3<f64>, Point3<f64>, Point3<f64>),
    ) -> (Body<f64>, FaceKey) {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(a, true).unwrap();
        let plane = body.add_surface(plane_surface);
        let cyl = body.add_surface(cyl_surface);
        let arc = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(plane, cyl),
                selector: geom_brep::CurveSelector::Witness(witness),
            },
            carrier,
            param_start,
            param_end,
        };
        let e_ab = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                b,
                arc,
                Tol::witness(),
            )
            .unwrap();
        let e_bc = body
            .mev_line(
                MevSite::Fan {
                    he1: e_ab.he_minus,
                    he2: e_ab.he_minus,
                },
                c,
                Tol::witness(),
            )
            .unwrap();
        let he = body
            .find_half_edge(seed.face, e_bc.vertex, e_ab.vertex)
            .unwrap();
        let face = body
            .mef(
                MefSite::Chords {
                    he1: he,
                    he2: e_ab.he_plus,
                },
                EdgeCurveSpec::line_between(c, a),
                FaceSurface::Shared {
                    key: plane,
                    sense: true,
                },
                Tol::witness(),
            )
            .unwrap()
            .face;
        (body, face)
    }

    /// **The reported defect.** A planar face's rim bulges past its
    /// boundary VERTICES, so a vertex-hull box is not a superset and
    /// `Bvh::overlapping` can prune a pair the exact predicates would
    /// have accepted.
    ///
    /// The span is swept from a shallow arc to a reflex one, and the
    /// radius with it: the miss grows with the sagitta, so a rule that
    /// covers only part of the bulge fails at the larger spans while
    /// passing the small ones. A single fixture cannot be the reason
    /// this row is green.
    #[test]
    fn a_planar_faces_circular_rim_is_inside_its_box() {
        for &r in &[0.001, 1.0, 250.0] {
            for span_deg in [10.0_f64, 90.0, 179.0, 181.0, 300.0, 359.0] {
                for phi_deg in [0.0_f64, 45.0, 137.0] {
                    let span = span_deg.to_radians();
                    let (body, face) = arc_sector_from(r, span, phi_deg.to_radians());
                    let b = face_box(&body, face, pad(), witness_band()).unwrap();
                    // The locus is the convex hull of its boundary and
                    // the box is convex, so sampling the boundary
                    // settles it.
                    for i in 0..=512 {
                        let t = span * f64::from(i) / 512.0;
                        let p = Point3::new(r * t.cos(), r * t.sin(), 0.0);
                        assert!(
                            holds(&b, p),
                            "rim point at {t} rad left the box (r = {r}, \
                             span = {span_deg}°, u_ref at {phi_deg}°): {b:?}"
                        );
                    }
                }
            }
        }
    }

    /// The same claim stated as a margin: the box must reach the arc's
    /// extreme in the direction the vertex hull cannot see. A
    /// half-turn sector's rim tops out at `y = r` while both its
    /// vertices sit at `y ≤ 0`, so a vertex-hull box misses by `r` —
    /// this row measures that gap rather than trusting a sample to
    /// land on it.
    #[test]
    fn the_boxs_reach_beyond_the_vertex_hull_is_the_whole_bulge() {
        let r = 2.0;
        let (body, face) = arc_sector(r, core::f64::consts::PI);
        let b = face_box(&body, face, pad(), witness_band()).unwrap();
        // Both boundary vertices and the sector's centre are at y ≤ 0;
        // the rim reaches y = r.
        assert!(
            b.max_y >= r,
            "the box must reach the rim's extreme y = {r}, got {}",
            b.max_y
        );
    }

    /// `face` re-labelled onto `surface`, its edges' descriptions as
    /// they stand.
    fn relabel(body: &mut Body<f64>, face: FaceKey, surface: Surface<f64>) {
        // Lifts RechartUnvouched: each box arm reads the surface a face
        // is labelled with, and the fixture's edges name the chart it
        // was built on.
        body.lifting_rechart_refusals_for_tests(|body| {
            body.set_face_surfaces_describing(
                vec![crate::construction::Rechart::new(surface, face, true)],
                &[],
                Tol::witness(),
            )
        })
        .unwrap();
    }

    /// A cylinder WALL face: the patch `u ∈ [u0, u1] × z ∈ [z0, z1]` on
    /// the radius-`r` cylinder about the z axis, bounded below and
    /// above by circular rims and on the sides by axial lines.
    fn cyl_wall(r: f64, u0: f64, u1: f64, z0: f64, z1: f64) -> (Body<f64>, FaceKey) {
        revolved_wall(&|_| r, u0, u1, z0, z1)
    }

    /// A wall of revolution about `z`, of radius `rho(z)`, over
    /// `u ∈ [u0, u1] × z ∈ [z0, z1]`: circular rims top and bottom,
    /// generator edges at the sides. The face's own surface is the
    /// CYLINDER through the bottom rim; a caller wanting another
    /// surface of revolution through the same boundary re-labels it
    /// ([`Body::set_face_surfaces_describing`](crate::construction::Construct::set_face_surfaces_describing)) — the boundary is
    /// genuinely on that surface whenever `rho` is the surface's own
    /// radius profile, which is what makes the re-label honest rather
    /// than a fixture trick.
    ///
    /// The rim carriers are described as the cylinder-of-that-radius
    /// cut by the plane at that height, which is the same circle
    /// whatever surface the face ends up carrying: the description
    /// certifies the CARRIER, and a cone's own rim is that circle.
    ///
    /// The face's loop runs clockwise about the chart normal and its
    /// sense is `true`, so the side its loop bounds is the sense-false
    /// one: a row that reads which points the face holds gives it a side
    /// with [`crate::test_support_fixtures::set_sheet_sense`].
    fn revolved_wall(
        rho: &dyn Fn(f64) -> f64,
        u0: f64,
        u1: f64,
        z0: f64,
        z1: f64,
    ) -> (Body<f64>, FaceKey) {
        let on = |u: f64, z: f64| Point3::new(rho(z) * u.cos(), rho(z) * u.sin(), z);
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(on(u0, z0), true).unwrap();
        // A rim at height `z`: the cylinder cut by the plane there.
        // The descending rim runs on the reversed axis so its own
        // parameters increase, exactly as the split lane mints them.
        let rim = |body: &mut Body<f64>, z: f64, ccw: bool| {
            let r = rho(z);
            let plane = body.add_surface(Surface::Plane {
                origin: Point3::new(0.0, 0.0, z),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            });
            let wall = body.add_surface(cyl_r(r));
            let (carrier, t0, t1) = if ccw {
                (
                    Curve3::circle(Point3::new(0.0, 0.0, z), Vec3::unit_z(), r, Vec3::unit_x()),
                    u0,
                    u1,
                )
            } else {
                (
                    Curve3::circle(
                        Point3::new(0.0, 0.0, z),
                        Vec3::new(0.0, 0.0, -1.0),
                        r,
                        Vec3::new(u1.cos(), u1.sin(), 0.0),
                    ),
                    0.0,
                    u1 - u0,
                )
            };
            (
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::Intersection {
                        pair: SurfacePair::new(wall, plane),
                        selector: geom_brep::CurveSelector::Witness(on((u0 + u1) * 0.5, z)),
                    },
                    carrier,
                    param_start: t0,
                    param_end: t1,
                },
                wall,
            )
        };
        let (bottom, cyl) = rim(&mut body, z0, true);
        let e_b = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                on(u1, z0),
                bottom,
                Tol::witness(),
            )
            .unwrap();
        let e_r = body
            .mev_line(
                MevSite::Fan {
                    he1: e_b.he_minus,
                    he2: e_b.he_minus,
                },
                on(u1, z1),
                Tol::witness(),
            )
            .unwrap();
        let (top, _) = rim(&mut body, z1, false);
        let e_t = body
            .mev(
                MevSite::Fan {
                    he1: e_r.he_minus,
                    he2: e_r.he_minus,
                },
                on(u0, z1),
                top,
                Tol::witness(),
            )
            .unwrap();
        let he = body
            .find_half_edge(seed.face, e_t.vertex, e_r.vertex)
            .unwrap();
        // Lifts RechartUnvouched: the top rim names the wall at its own height, which callers re-label the face onto.
        let face = body
            .lifting_rechart_refusals_for_tests(|body| {
                body.mef(
                    MefSite::Chords {
                        he1: he,
                        he2: e_b.he_plus,
                    },
                    EdgeCurveSpec::line_between(on(u0, z1), on(u0, z0)),
                    FaceSurface::Shared {
                        key: cyl,
                        sense: true,
                    },
                    Tol::witness(),
                )
            })
            .unwrap()
            .face;
        (body, face)
    }

    /// The cylinder arm, against the wall it bounds. The belly bulges
    /// past every chord of the boundary, and the axial range must cover
    /// the whole patch — both swept over radii and over azimuth spans
    /// including a reflex one, so a rule that recovers the extent only
    /// for short spans goes red.
    #[test]
    fn a_cylinder_walls_locus_is_inside_its_box() {
        for &r in &[0.002, 1.0, 40.0] {
            for span_deg in [30.0_f64, 170.0, 200.0, 350.0] {
                let span = span_deg.to_radians();
                let (z0, z1) = (-0.25 * r, 0.75 * r);
                let (body, face) = cyl_wall(r, 0.0, span, z0, z1);
                let b = face_box(&body, face, pad(), witness_band()).unwrap();
                for i in 0..=64 {
                    let u = span * f64::from(i) / 64.0;
                    for j in 0..=8 {
                        let z = z0 + (z1 - z0) * f64::from(j) / 8.0;
                        let p = Point3::new(r * u.cos(), r * u.sin(), z);
                        assert!(
                            holds(&b, p),
                            "wall point (u = {u}, z = {z}) left the box \
                             (r = {r}, span = {span_deg}°): {b:?}"
                        );
                    }
                }
            }
        }
    }

    /// The sphere arm, against the sphere. This arm claims the WHOLE
    /// ball and reads nothing from the boundary, so the honest locus to
    /// sample is the whole sphere — every point of it, at any trim, is
    /// what the box promises to contain.
    #[test]
    fn a_spheres_whole_locus_is_inside_its_box() {
        for &r in &[0.002, 1.0, 40.0] {
            let center = Point3::new(0.3 * r, -0.2 * r, 0.1 * r);
            let (mut body, face) = arc_sector(r, core::f64::consts::PI);
            relabel(
                &mut body,
                face,
                Surface::Sphere {
                    center,
                    radius: r,
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                },
            );
            let b = face_box(&body, face, pad(), witness_band()).unwrap();
            for i in 0..=32 {
                let theta = core::f64::consts::PI * f64::from(i) / 32.0;
                for j in 0..=32 {
                    let phi = 2.0 * core::f64::consts::PI * f64::from(j) / 32.0;
                    let p = Point3::new(
                        center.x + r * theta.sin() * phi.cos(),
                        center.y + r * theta.sin() * phi.sin(),
                        center.z + r * theta.cos(),
                    );
                    assert!(holds(&b, p), "sphere point left the box (r = {r}): {b:?}");
                }
            }
        }
    }

    /// A biquadratic patch whose boundary lies entirely in `z = 0`
    /// while its centre control point lifts the surface to `z = 1/4`,
    /// carried on an arc sector's topology. Returns the body, the
    /// face, and the control net's own hull — which is the unit cube,
    /// and is what [`FaceBoxRule::ControlNet`] claims.
    fn nurbs_bulge_face() -> (Body<f64>, FaceKey, (Point3<f64>, Point3<f64>)) {
        use geom::surfaces::nurbs::NurbsSurface;
        use geom_core::spline::KnotVector;
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::new(2).unwrap());
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.5, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.5, 0.0, 0.0),
            Point3::new(0.5, 0.5, 1.0),
            Point3::new(0.5, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.5, 0.0),
            Point3::new(1.0, 1.0, 0.0),
        ];
        let patch = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap();
        let surface = Surface::Nurbs(std::sync::Arc::new(patch));
        let (mut body, face) = arc_sector(1.0, core::f64::consts::PI);
        relabel(&mut body, face, surface);
        (
            body,
            face,
            (Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)),
        )
    }

    /// **The NURBS half of the same defect.** A patch's interior
    /// bulges past the hull of its boundary — here the biquadratic
    /// whose boundary lies entirely in `z = 0` while its centre
    /// control point lifts the surface to `z = 1/4`. The control-net
    /// hull contains it; the boundary hull does not.
    #[test]
    fn a_nurbs_patchs_interior_bulge_is_inside_its_box() {
        let (body, face, _) = nurbs_bulge_face();
        let surface = body.face_surface(face).unwrap().clone();
        // The lifted interior, on the surface itself — its boundary
        // curves all lie in `z = 0`, so no hull of the BOUNDARY can
        // contain this point.
        let mid = surface.eval(0.5, 0.5);
        assert!(mid.z > 0.2, "the fixture must actually bulge, got {mid:?}");
        assert!(
            surface.eval(0.0, 0.5).z.abs() < 1e-15,
            "the fixture's boundary must lie in z = 0"
        );
        let b = face_box(&body, face, pad(), witness_band()).unwrap();
        assert!(
            holds(&b, mid),
            "the patch's own interior point left its box: {b:?}"
        );
    }

    /// **The ceiling side of the contract, shared by the four rows
    /// below: an arm's box is EXACTLY the construction its rule
    /// states.**
    ///
    /// Each row states its arm's box as a FORMULA in the fixture's
    /// own parameters and pins [`face_box`] to it on all six faces in
    /// **both** directions — widening beyond the stated construction
    /// reds it, and so does narrowing, which the locus rows see only
    /// once it crosses the locus. One row per arm, so a red names its
    /// arm before the message does.
    ///
    /// **Every formula below is the rule and nothing else.** No term
    /// here stands for a known over-claim: an arm that claims more
    /// than its rule states reds its row rather than acquiring a
    /// named deviation term, because a term the row tolerates is a
    /// ratification.
    ///
    /// The six faces of `got` agree with `want` to within the
    /// arithmetic's own rounding. [`Aabb::padded`] alone moves each
    /// face by an ulp, and the fixture's trigonometry by a few more;
    /// the tolerance is relative to the fixture's scale and is orders
    /// below every term it has to separate — the pad, and the radius
    /// each arm might wrongly add.
    fn agrees_with_the_rule(got: &Aabb, want: &Aabb, scale: f64, what: &str) {
        let tol = 1e-12 * (1.0 + scale);
        for (name, g, w) in [
            ("min_x", got.min_x, want.min_x),
            ("min_y", got.min_y, want.min_y),
            ("min_z", got.min_z, want.min_z),
            ("max_x", got.max_x, want.max_x),
            ("max_y", got.max_y, want.max_y),
            ("max_z", got.max_z, want.max_z),
        ] {
            assert!(
                (g - w).abs() <= tol,
                "{what}: {name} is {g}, the construction its rule states gives {w} \
                 (off by {}, tolerance {tol})",
                g - w
            );
        }
    }

    /// **`BoundaryHull`, conic-fed** — the sector's box is the hull of
    /// its rim ARC's own box and its two radius chords, which run from
    /// the centre out to the arc's ends. So it is the arc's exact
    /// extremes hulled with the ORIGIN, and it is flat in z.
    ///
    /// The arc's extremes, not the full turn's: a 10° sector no longer
    /// claims `±r` on both axes. That is the same trim-scoping the
    /// cylinder arm gets, arriving here through the boundary — a planar
    /// face bounded by arcs was paying for the whole circle exactly as
    /// a wall was.
    ///
    /// **The φ sweep is what makes this row an invariance claim**: at
    /// each start angle the answer must be the arc's own geometry, not
    /// a bound read off `u_ref`. It is what would catch a triangle
    /// inequality claiming `r√2` at 45°, and now also an arc scoping
    /// that forgot to rotate with the run.
    #[test]
    fn the_planar_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        for &r in &[0.001, 1.0, 250.0] {
            for span_deg in [10.0_f64, 90.0, 179.0, 181.0, 300.0, 359.0] {
                for phi_deg in [0.0_f64, 45.0, 137.0] {
                    let phi = phi_deg.to_radians();
                    let span = span_deg.to_radians();
                    let (body, face) = arc_sector_from(r, span, phi);
                    let b = face_box(&body, face, pad, witness_band()).unwrap();
                    // World angles, NOT `u_ref`-relative: the fixture's
                    // φ renames the carrier's reference direction and
                    // leaves the sector where it is, which is the whole
                    // point of sweeping it. The run is `[0, span]` at
                    // every φ, and so is the expectation.
                    let (cos_lo, cos_hi) = arc_extremes(span, &f64::cos, &[core::f64::consts::PI]);
                    let (sin_lo, sin_hi) = arc_extremes(
                        span,
                        &f64::sin,
                        &[
                            core::f64::consts::FRAC_PI_2,
                            3.0 * core::f64::consts::FRAC_PI_2,
                        ],
                    );
                    // The EXACT construction: the arc's own extremes,
                    // hulled with the centre the two radius chords end
                    // at. The box must contain it — that is soundness —
                    // and must not exceed it by more than the pad and
                    // outward rounding — that is the trim scoping, and
                    // the exact arm's whole claim.
                    let exact = Aabb {
                        min_x: (r * cos_lo).min(0.0),
                        min_y: (r * sin_lo).min(0.0),
                        min_z: 0.0,
                        max_x: (r * cos_hi).max(0.0),
                        max_y: (r * sin_hi).max(0.0),
                        max_z: 0.0,
                    };
                    let slack = pad + 1e-9 * r;
                    let who = format!(
                        "the planar arm (r = {r}, span = {span_deg}°, u_ref at {phi_deg}°)"
                    );
                    for (name, got, want) in [
                        ("min_x", b.min_x, exact.min_x),
                        ("min_y", b.min_y, exact.min_y),
                        ("min_z", b.min_z, exact.min_z),
                    ] {
                        assert!(
                            got <= want && got >= want - slack,
                            "{who}: {name} is {got}, outside [{}, {want}]",
                            want - slack
                        );
                    }
                    for (name, got, want) in [
                        ("max_x", b.max_x, exact.max_x),
                        ("max_y", b.max_y, exact.max_y),
                        ("max_z", b.max_z, exact.max_z),
                    ] {
                        assert!(
                            got >= want && got <= want + slack,
                            "{who}: {name} is {got}, outside [{want}, {}]",
                            want + slack
                        );
                    }
                }
            }
        }
    }

    /// **`CylinderSlab`** — the axial range is the boundary's own, and
    /// the perpendicular extent is the ring of that radius CLIPPED to
    /// the boundary's own footprint. The fixture's axis is `z`, so:
    ///
    /// - the z face is the trim's own `[z0, z1]` plus the pad and
    ///   NOTHING else — the radius does not appear there, and neither
    ///   does a second application of the pad. That is the whole of
    ///   #862's measured case (a radius-`r` cylinder over
    ///   `z ∈ [z0, z1]` claimed over `z ∈ [z0 − r, z1 + r]`), stated
    ///   as an equality this row cannot pass with the width restored.
    /// - the x and y faces are the ARC's own extremes over the trim's
    ///   azimuth run, not `±r`. A quarter-turn corner round is boxed
    ///   as a quarter ring; only a run that actually reaches an axis
    ///   crossing gets `±r` on that axis. The spans below straddle
    ///   every case: `30°` reaches neither, `170°` reaches `+y` only,
    ///   `200°` reaches `+y` and `−x`, `350°` reaches all four.
    ///
    /// The clip is taken BEFORE the pad, so the pad is restored on
    /// every face afterwards — the box keeps its tolerance slack.
    #[test]
    fn the_cylinder_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        for &r in &[0.002, 1.0, 40.0] {
            for span_deg in [30.0_f64, 170.0, 200.0, 350.0] {
                let (z0, z1) = (-0.25 * r, 0.75 * r);
                let span = span_deg.to_radians();
                let (body, face) = cyl_wall(r, 0.0, span, z0, z1);
                let b = face_box(&body, face, pad, witness_band()).unwrap();
                let (cos_lo, cos_hi) = arc_extremes(span, &f64::cos, &[core::f64::consts::PI]);
                let (sin_lo, sin_hi) = arc_extremes(
                    span,
                    &f64::sin,
                    &[
                        core::f64::consts::FRAC_PI_2,
                        3.0 * core::f64::consts::FRAC_PI_2,
                    ],
                );
                // The rim arcs' own boxes are the exact extremes, so
                // the clipped ring is exactly the arc's footprint.
                agrees_with_the_rule(
                    &b,
                    &Aabb {
                        min_x: r * cos_lo - pad,
                        min_y: r * sin_lo - pad,
                        min_z: z0 - pad,
                        max_x: r * cos_hi + pad,
                        max_y: r * sin_hi + pad,
                        max_z: z1 + pad,
                    },
                    r,
                    &format!("the cylinder arm (r = {r}, span = {span_deg}°)"),
                );
            }
        }
    }

    /// **The slab reads a decided axis, never the carrier's bare one.**
    /// A cylinder carrier whose axis is `2ẑ` describes the same point
    /// set as the unit one; read bare, the axial window would be twice
    /// the face's and the slab would place it twice again along the
    /// axis — a box four times the wall's height. Both lanes box it as
    /// the unit-axis wall. An axis that is no direction is a broken
    /// carrier: the bracket lane refuses it, the census lane claims
    /// nothing.
    #[test]
    fn a_cylinder_axis_is_decided_unit_before_the_slab_reads_it() {
        let (z0, z1) = (-0.25, 0.75);
        let (unit, face) = cyl_wall(1.0, 0.0, 2.0, z0, z1);
        let with_axis = |axis: Vec3<f64>| {
            let mut body = unit.clone();
            let key = body.get_face(face).unwrap().surface;
            let Surface::Cylinder { axis: a, .. } = &mut body.surfaces[key] else {
                panic!("cyl_wall's face carries a cylinder")
            };
            *a = axis;
            body
        };
        let want = face_box(&unit, face, pad(), witness_band()).unwrap();
        let long = with_axis(Vec3::new(0.0, 0.0, 2.0));
        let got = face_box(&long, face, pad(), witness_band()).unwrap();
        for (what, w, g) in [
            ("min z", want.min_z, got.min_z),
            ("max z", want.max_z, got.max_z),
            ("min x", want.min_x, got.min_x),
            ("max x", want.max_x, got.max_x),
        ] {
            assert!(
                (w - g).abs() <= 1e-12,
                "the bracket lane must box a 2ẑ-axis wall as the unit one ({what}): \
                 {g} vs {w}"
            );
        }
        let (lo, hi) = crate::census::face_reach(&long, face, witness_band())
            .expect("the census lane claims the 2ẑ-axis wall");
        assert!(
            (lo.z - z0).abs() <= 1e-12 && (hi.z - z1).abs() <= 1e-12,
            "the census lane must box a 2ẑ-axis wall's height as [{z0}, {z1}], got \
             [{}, {}]",
            lo.z,
            hi.z
        );
        let broken = with_axis(Vec3::new(0.0, 0.0, 0.0));
        assert!(
            matches!(
                face_box(&broken, face, pad(), witness_band()),
                Err(BooleanError::ClassificationInvariant { .. })
            ),
            "a zero cylinder axis must refuse the bracket lane's box"
        );
        assert!(
            crate::census::face_reach(&broken, face, witness_band()).is_none(),
            "a zero cylinder axis must claim nothing in the census lane"
        );
    }

    /// **A radius the band does not read as positive boxes by the
    /// boundary hull, never refuses.** Tier 3 admits any cylinder whose
    /// radius is exactly positive, and the slab's levered axis decision
    /// refuses one the band reads as zero or in band. Walls of radius 1
    /// whose arcs bulge past their own vertices (an azimuth extremum
    /// strictly inside the arc, a near-full turn) are read under a band
    /// that puts 1 in band, then under one that decides it zero: both
    /// lanes box each as its boundary's hull, that box holds the wall,
    /// and the lever's escalation, which the box does not rest on, is
    /// not left on the log.
    #[test]
    fn a_cylinder_whose_radius_the_band_does_not_decide_boxes_by_its_boundary() {
        use geom_core::k_stats::{Bracket, Escalation};
        let (z0, z1) = (-0.25, 0.75);
        let pi = core::f64::consts::PI;
        for (u0, u1) in [
            (pi / 4.0, 3.0 * pi / 4.0),
            (0.3, 0.3 + 6.0),
            (-pi / 2.0 + 0.1, pi / 2.0 - 0.1),
        ] {
            let (body, face) = cyl_wall(1.0, u0, u1, z0, z1);
            let f = body.get_face(face).unwrap();
            let Surface::Cylinder { axis, radius, .. } = body.face_surface_linked(face, f) else {
                panic!("the fixture's face carries a cylinder");
            };
            for (what, band) in [
                ("in band", Band::new(0.5, 5.0).unwrap()),
                ("decided zero", Band::new(2.0, 20.0).unwrap()),
            ] {
                let what = format!("[{u0}, {u1}] {what}");
                assert!(
                    UnitVec3::levered(
                        *axis,
                        BOX_CYLINDER_AXIS,
                        band,
                        geom_core::Lever::curvature_radius(*radius)
                    )
                    .is_err(),
                    "{what}: the levered axis must not decide, or the row reads the slab"
                );
                // Not the band's sweep pad: at these bands it is wider
                // than the bulge a vertex-only hull would miss.
                let pad = 1e-9;
                let hull = boundary_hull(&body, face, f).expect("the wall's boundary boxes");
                let bracket = Bracket::open();
                let b = face_box(&body, face, pad, band);
                let c = crate::census::face_reach(&body, face, band);
                let escalated: Vec<_> = bracket
                    .finish()
                    .escalations
                    .iter()
                    .map(Escalation::predicate)
                    .collect();
                assert!(
                    escalated.is_empty(),
                    "{what}: a definite hull box left escalations on the log: {escalated:?}"
                );
                let b = b.unwrap_or_else(|e| panic!("{what}: the wall must box, got {e:?}"));
                let (lo, hi) =
                    c.unwrap_or_else(|| panic!("{what}: the census must claim the wall"));
                let h = hull.padded(pad);
                assert_eq!(
                    [b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z],
                    [h.min_x, h.min_y, h.min_z, h.max_x, h.max_y, h.max_z],
                    "{what}: the bracket lane boxes the wall as its padded boundary hull"
                );
                let (blo, bhi) = ([b.min_x, b.min_y, b.min_z], [b.max_x, b.max_y, b.max_z]);
                let (clo, chi) = ([lo.x, lo.y, lo.z], [hi.x, hi.y, hi.z]);
                for i in 0..=64 {
                    for j in 0..=4 {
                        let u = u0 + (u1 - u0) * f64::from(i) / 64.0;
                        let z = z0 + (z1 - z0) * f64::from(j) / 4.0;
                        let p = [u.cos(), u.sin(), z];
                        for k in 0..3 {
                            assert!(
                                blo[k] <= p[k] && p[k] <= bhi[k],
                                "{what}: the bracket box must hold the wall at u {u}, z {z}"
                            );
                            assert!(
                                clo[k] <= p[k] && p[k] <= chi[k],
                                "{what}: the census reach must hold the wall at u {u}, z {z}"
                            );
                        }
                    }
                }
            }
        }
    }

    /// **A tilted wall's slab is clipped to its boundary hull on every
    /// world coordinate**, the axis-parallel one included. The wall's
    /// axis is tilted about `y`, so the slab's `z` extent carries the
    /// radius widening `r·sin θ`; the face lies in its boundary's hull
    /// (the ruled argument in [`FaceBoxRule`]'s docs). Over
    /// `u ∈ [1, 3]` a point's height is `v·cos θ − r·cos u·sin θ`, so
    /// the floor is the bottom rim's `u = 1` corner, `(1 − cos 1)·sin θ`
    /// above the slab's. Both lanes box the wall to that floor and
    /// still hold it.
    #[test]
    fn a_tilted_cylinder_walls_box_is_clipped_to_its_boundary_on_every_axis() {
        use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
        let ((u0, u1), (v0, v1)) = ((1.0, 3.0), (-0.5, 1.25));
        let frame = CylFrame::tilted(1.0, 0.7);
        let mut body = Body::<f64>::new();
        let face = cyl_wall_sheet(&mut body, frame, (u0, u1), (v0, v1), Tol::witness());
        let floor = frame.at::<f64>(u0, v0).z;
        // The slab's floor sits this far below; the census lane's arc
        // reach carries its subdivision charge, far inside it.
        let gap = (1.0 - u0.cos()) * 0.7_f64.sin();
        let b = face_box(&body, face, 0.0, witness_band()).unwrap();
        let (lo, hi) = crate::census::face_reach(&body, face, witness_band()).unwrap();
        for (what, min_z) in [("bracket", b.min_z), ("census", lo.z)] {
            assert!(
                min_z <= floor + 1e-12 && floor - min_z <= gap / 10.0,
                "the {what} lane's z floor {min_z} is the boundary's {floor}, not the slab's \
                 {}",
                floor - gap
            );
        }
        let (blo, bhi) = ([b.min_x, b.min_y, b.min_z], [b.max_x, b.max_y, b.max_z]);
        let (clo, chi) = ([lo.x, lo.y, lo.z], [hi.x, hi.y, hi.z]);
        for i in 0..=64 {
            for j in 0..=4 {
                let u = u0 + (u1 - u0) * f64::from(i) / 64.0;
                let v = v0 + (v1 - v0) * f64::from(j) / 4.0;
                let p = frame.at::<f64>(u, v);
                let p = [p.x, p.y, p.z];
                for k in 0..3 {
                    assert!(
                        blo[k] - 1e-12 <= p[k] && p[k] <= bhi[k] + 1e-12,
                        "the bracket box must hold the wall at u {u}, v {v}"
                    );
                    assert!(
                        clo[k] - 1e-12 <= p[k] && p[k] <= chi[k] + 1e-12,
                        "the census reach must hold the wall at u {u}, v {v}"
                    );
                }
            }
        }
    }

    /// The exact range of `f` over the arc `[0, span]`: the endpoints,
    /// plus each listed critical angle the run actually reaches. Stated
    /// here rather than folded into the row so the expectation is a
    /// construction the reader can check against the unit circle, not a
    /// number copied out of a failing run.
    fn arc_extremes(span: f64, f: &dyn Fn(f64) -> f64, criticals: &[f64]) -> (f64, f64) {
        arc_extremes_from(0.0, span, f, criticals)
    }

    /// The same over `[t0, t0 + span]`: a critical angle counts when
    /// some `2π`-shift of it lands in the run.
    fn arc_extremes_from(
        t0: f64,
        span: f64,
        f: &dyn Fn(f64) -> f64,
        criticals: &[f64],
    ) -> (f64, f64) {
        let mut lo = f(t0).min(f(t0 + span));
        let mut hi = f(t0).max(f(t0 + span));
        for &c in criticals {
            for k in -2..=2 {
                let t = c + f64::from(k) * core::f64::consts::TAU;
                if t >= t0 && t <= t0 + span {
                    lo = lo.min(f(t));
                    hi = hi.max(f(t));
                }
            }
        }
        (lo, hi)
    }

    /// **The issue's own measured case, in its own numbers**: a
    /// radius-0.5 cylinder over `z ∈ [0, 1]` was claimed over
    /// `z ∈ [−0.5, 1.5]` — a 2.0-long slab where the face is 1.0
    /// long, and the containing extent `census`'s arm 2 reads, so a
    /// probe below the cylinder lost its definitely-negative margin
    /// and came back `CensusUndecidable`.
    ///
    /// Stated as the CONSUMER's question rather than as six faces:
    /// does a point that sits below the cylinder's own trim, by more
    /// than the pad and less than the radius, fall outside the box?
    /// It has to, and the whole spread between `pad` and `r` is
    /// swept, so a partial restoration of the width reds this too.
    #[test]
    fn the_measured_axial_over_claim_is_gone_at_the_issues_own_numbers() {
        let (r, z0, z1) = (0.5, 0.0, 1.0);
        let pad = pad();
        let (body, face) = cyl_wall(r, 0.0, core::f64::consts::PI, z0, z1);
        let b = face_box(&body, face, pad, witness_band()).unwrap();
        assert!(
            b.min_z > z0 - r && b.max_z < z1 + r,
            "the slab must not claim the radius along its own axis: {b:?}"
        );
        for k in 1..=16 {
            let below = z0 - pad - (r - pad) * f64::from(k) / 16.0;
            assert!(
                below < b.min_z,
                "a probe at z = {below}, below the trim by more than the pad, is \
                 still inside the box [{}, {}] — the axial over-claim is back",
                b.min_z,
                b.max_z
            );
        }
    }

    /// **`SphereWindow` outside the rectangle class** — `center ± r`: the
    /// fixture's boundary is a planar sector's, no chart rectangle of
    /// the sphere, so its trim must not appear in the box at all. No
    /// deviation terms: this arm claims exactly what its rule states.
    /// The rectangle itself is `sphere_rect_rows`'.
    #[test]
    fn the_sphere_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        for &r in &[0.002, 1.0, 40.0] {
            let c = Point3::new(0.3 * r, -0.2 * r, 0.1 * r);
            let (mut body, face) = arc_sector(r, core::f64::consts::PI);
            relabel(
                &mut body,
                face,
                Surface::Sphere {
                    center: c,
                    radius: r,
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                },
            );
            let b = face_box(&body, face, pad, witness_band()).unwrap();
            agrees_with_the_rule(
                &b,
                &Aabb {
                    min_x: c.x - r - pad,
                    min_y: c.y - r - pad,
                    min_z: c.z - r - pad,
                    max_x: c.x + r + pad,
                    max_y: c.y + r + pad,
                    max_z: c.z + r + pad,
                },
                r,
                &format!("the sphere arm (r = {r})"),
            );
        }
    }

    /// **`ControlNet`** — the hull of the net and nothing else. The
    /// fixture's net spans the unit cube while its own boundary lies
    /// in `z = 0`, so a box read off the BOUNDARY would be visibly
    /// different from one read off the net. No deviation terms.
    #[test]
    fn the_nurbs_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        let (body, face, net) = nurbs_bulge_face();
        let b = face_box(&body, face, pad, witness_band()).unwrap();
        agrees_with_the_rule(
            &b,
            &Aabb {
                min_x: net.0.x - pad,
                min_y: net.0.y - pad,
                min_z: net.0.z - pad,
                max_x: net.1.x + pad,
                max_y: net.1.y + pad,
                max_z: net.1.z + pad,
            },
            1.0,
            "the NURBS arm",
        );
    }

    /// **The door inventory** — the module docs' door list, computed
    /// rather than recited.
    ///
    /// The header states which direction each consumer reads a loose
    /// box in. That is a roster, and a roster is right only until the
    /// next door lands. This row walks `topo/src` and counts every
    /// CALL of [`face_box`], [`face_box_rule`], [`edge_box`] and the
    /// edge's cover ([`edge_cover`], [`edge_cover_linked`],
    /// [`edge_cover_of`]) in code — comments and
    /// literals blanked by
    /// [`crate::source_walk::CodeOnly`], the more competent of this
    /// crate's two readers — pinned per file.
    ///
    /// **Faces and edges, not just faces.** The header's claim is
    /// about *a box*, and every face box that hulls a boundary is the
    /// edges' covers' answer one dimension down. A walk that
    /// matched only `face_box` would attribute that arm's cost to a
    /// door list computed for a different function, and an
    /// edge-box-only door would land green.
    ///
    /// - `boolean/reduce.rs` — the C10 candidate tree, face and edge.
    ///   **Prunes**: loose is slower work, never a different answer.
    ///   And the maximal-faces gate's lever, an edge's cover's farthest
    ///   distance from its start. **Refuses**: a longer lever reads a
    ///   near-coplanar pair as undecided, never as one plane.
    /// - `boolean/glue.rs` — the glue door's box sweep
    ///   (`overlapping_pairs`), face boxes of both operands, which its
    ///   pair scan and its coaxial scan both read. **Prunes**: a loose box
    ///   only adds pairs the carrier ladder reads and leaves apart,
    ///   which is slower work and the same answer. A box TIGHTER than
    ///   its face would miss a pair, which then reaches its site
    ///   unglued and refuses there — loud, never a different body.
    /// - `boolean/ops.rs` — the curved-extent fallback, face and
    ///   edge: the cylinder-face arm clears a [`face_box`] against
    ///   the ball's extent, the scan's near-boundary test walks the
    ///   face's [`edge_box`]es against the germ circle's box, the
    ///   cone/torus arm consults a [`face_box`] before refusing by
    ///   kind — reach first, kind second, as at the operand gate —
    ///   and the section certificate's pair scan (`section_pairs`,
    ///   both paths) reads one [`face_box`] per face of each operand:
    ///   two that overlap put the pair through the certificate, whose
    ///   reach — the ball about the overlap — pivots and levers its
    ///   angular margins. **Refuses** at the extent scan: a box that
    ///   fails to clear turns the pair into `FallbackExtentUnsupported`.
    ///   **Examines, and can refuse,** at the pair scan: a loose box
    ///   sends a separated pair through the classification, which
    ///   certifies it apart, and lengthens the lever, so a near-parallel
    ///   tilt stops deciding `Zero` and the pair takes the tilted arm or
    ///   refuses on reach. A box TIGHTER than its face would be the
    ///   unsound direction: its reach could miss the section.
    /// - `separation.rs` — the two separation certificates, the
    ///   placement one and the solid-pair one, on one rule.
    ///   **Refuses**, both of them and for the same reason:
    ///   non-overlap IS the grant, so a loose box withholds a
    ///   certificate it should have given and can never manufacture
    ///   one. The second door added no direction to weigh — it is the
    ///   first read in the body's own frame instead of through a
    ///   placement's affine image, which drops the image step and
    ///   changes nothing about what looseness costs.
    /// - `face_boxes.rs` — [`crate::FaceBoxes`], every face's box and
    ///   a C10 tree over them, which the blend's reach meter
    ///   (`sweep::blend::reach`) reads twice: to prune the faces a band
    ///   can reach, and to bound the cells it meters a survivor on.
    ///   **Prunes, then can refuse**: a loose box only adds candidates
    ///   and cells, which is slower work until the meter's per-face
    ///   cell budget runs out, and then a refusal as uncertified. A
    ///   box TIGHTER than its face would be the unsound direction: a
    ///   pruned face is never metered.
    /// - `census.rs` — `face_reach_in` and `edge_reach_in`, this module's
    ///   extents entered at the census's own scalar. **Refuses**:
    ///   arm 2 clears for free only on a definitely negative margin
    ///   against a CONTAINING box, so over-width would send a separated
    ///   pair to the material test instead of clearing it — a case the
    ///   planar-only corpus does not reach (the hull is the box) and a
    ///   curved reach box reaches once arm 1 has proved its face's
    ///   pairs apart, where it costs probes, never a wrong clear.
    ///   And `flush_at_rest`'s `face_box` read, a tree over the planar,
    ///   cylinder and sphere faces whose overlapping boxes are the flush
    ///   candidates.
    ///   **Prunes**: a loose box only adds candidates, each read
    ///   exactly; a box tighter than its face would drop a flush pair.
    /// - `census/depth_gap.rs` — arm 1's depth-gap clear: one
    ///   [`face_box_rule`] read, which picks a face's cover and its
    ///   carrier's depth. **Refuses**: it grants on a gap, and every
    ///   cover it builds from the rule only holds MORE than its face
    ///   when loose, which narrows the gap. A rule that named a cover
    ///   holding less would be the unsound direction. A looser
    ///   proposal (an axis not offered) only withholds a depth.
    ///
    /// `boolean/boxes.rs` is excluded by path: it is the definition
    /// site, every call in it is this suite's own or one arm calling
    /// another, and its count would churn on each row added here
    /// while pinning nothing.
    ///
    /// **This row is a member of S117's population** — a
    /// source-text guard over `.rs` in this workspace. It reuses the
    /// shared reader rather than minting a sixth, which answers the
    /// half of S117 about how they lex; it does not answer the half
    /// about how many there are, and it makes that number larger.
    ///
    /// **What this cannot match**, stated rather than implied:
    ///
    /// 1. A door in another CRATE. The walk is `topo/src` because
    ///    that is the tree this crate can see. All six functions are
    ///    `pub(crate)`, so no such door can exist today — that, and
    ///    not a survey, is what carries the claim out of crate.
    /// 2. A door reading a box through a helper defined here: a
    ///    wrapper's own callers are invisible to a textual walk.
    /// 3. A door under another name: a `use … as` rename, or the
    ///    function used as a value (`.map(edge_box)`, a fn pointer).
    ///    A module alias or a re-export still spells the name and its
    ///    `(` or `::<`, so it is counted; a call a macro assembles is
    ///    not.
    ///
    /// **What it cannot match that is a FINDING rather than a
    /// disclosure: the direction itself.** This pins where the doors
    /// are, not what each does with looseness — and the direction
    /// column is the whole content of the header's argument, so a
    /// roster without it is a grep result. A door that changes its
    /// reading without moving leaves the dispositions above stale and
    /// this row green. That is this module's own defect one level up,
    /// recorded as **S232**'s sibling `S234` rather than left in the
    /// list above, because a disclosure with no owner is how the
    /// fifth instance gets found by accident.
    #[test]
    fn every_door_that_reads_a_box_is_inventoried() {
        // `census.rs` counts TEN: the pre-filter's `face_box` read (one
        // box per face, read by the planar and every-face classes) and
        // its `edge_box` read, the flush door's `face_box` read (the
        // pruning door over its candidates), pass 2′'s curved-edge `edge_box` read
        // (`census::Candidates::build`, the pruning door) and
        // `collinear_overlaps`' `edge_box` read (the same pruning door),
        // a `face_box_rule` read of its own and two edge-cover reads (the
        // boundary's axial window and the edge reach), and two that are
        // not doors — two rows in its test module call `face_box` to
        // pin what a described net carrying poison answers there. The
        // number is stated with that content rather than filtered,
        // because this pin's protection is that an
        // occurrence cannot arrive, leave or move unnoticed — which it
        // still gives — while the module docs' DOOR list above stays a
        // list of doors and gains nothing from the two.
        //
        // `boolean/glue.rs`'s two are one door, the sweep its pair scan
        // and its coaxial scan share, boxing both operands' faces. One of
        // `boolean/ops.rs`'s four and `pieces.rs`'s one are one door:
        // the boolean's exit builds the face-box closure the piece
        // sort's screen calls. Another of `ops.rs`'s four is not a door:
        // `the_approx_arm_asks_whether_the_ball_reaches_the_face` boxes
        // its bricks' faces to hand the arm's question what the scan
        // hands it. `boolean/torn_hop_rows.rs`' four are not
        // doors either: its torn-body witnesses call `face_box` and
        // `edge_box` to show a torn link panics. Nor is
        // `boolean/carrier_touch_rows.rs`' one: a spiric-edge witness
        // calls `edge_box` to show its ball lies inside the edge's box,
        // so that only the edge's own floor can clear it.
        const PINNED: [(&str, usize); 13] = [
            ("boolean/carrier_touch.rs", 1),
            ("boolean/carrier_touch_rows.rs", 1),
            ("boolean/glue.rs", 2),
            ("boolean/mod.rs", 1),
            ("boolean/ops.rs", 4),
            ("boolean/reduce.rs", 6),
            ("boolean/torn_hop_rows.rs", 4),
            ("census.rs", 10),
            ("census/depth_gap.rs", 1),
            ("face_boxes.rs", 1),
            ("pieces.rs", 1),
            ("separation.rs", 2),
            ("validate.rs", 1),
        ];
        const HOME: &str = "boolean/boxes.rs";
        const DOORS: [&str; 6] = [
            "face_box",
            "face_box_rule",
            "edge_box",
            "edge_cover",
            "edge_cover_linked",
            "edge_cover_of",
        ];
        let root = crate::source_walk::src_root();
        let mut found: Vec<(String, usize)> = Vec::new();
        for path in crate::source_walk::crate_sources() {
            let rel = path
                .strip_prefix(&root)
                .expect("a walked file lies under topo/src")
                .to_string_lossy()
                .replace('\\', "/");
            if rel == HOME {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("a readable source file");
            let code = crate::source_walk::CodeOnly::of(&text);
            // A call is the name then its argument list, or then a
            // turbofish (`edge_cover::<f64>(`).
            let calls: usize = DOORS
                .iter()
                .flat_map(|d| [format!("{d}("), format!("{d}::<")])
                .map(|call| code.as_str().matches(call.as_str()).count())
                .sum();
            if calls > 0 || PINNED.iter().any(|(pinned, _)| *pinned == rel) {
                found.push((rel, calls));
            }
        }
        found.sort();
        let pinned: Vec<(String, usize)> = PINNED
            .iter()
            .map(|(path, calls)| ((*path).to_string(), *calls))
            .collect();
        assert_eq!(
            found, pinned,
            "a door that reads a box from this module arrived, left or moved. What the \
             module docs owe is not this count but the DIRECTION the new door reads \
             looseness in — pruning, or refusing — and nothing computes that (S234). \
             Update both, and read S234 before trusting the list you are updating."
        );
    }

    /// **A tilted section circle is boxed by its own extent**, in both
    /// directions: coordinate `i` of [`circle_box`] spans exactly
    /// `ρ·√(1 − nᵢ²)` about the centre, and every sampled point of the
    /// circle lies inside it. A `2ρ` cube about the centre passes the
    /// locus half and reds the ceiling half on every axis at the
    /// `(1, 2, 3)` normal, which leans off all three. The oracle reads
    /// `√(1 − nᵢ²)` as `√(nⱼ² + nₖ²)`, the norm of `n`'s other two
    /// components, which does not cancel where `nᵢ → 1`: the normal a
    /// few nanoradians off `ẑ` boxes its circle a few nanometres thick
    /// in `z`, and `1 − n_z²` rounds that to nothing.
    #[test]
    fn a_tilted_circles_box_is_its_own_extent() {
        let tilted = Vec3::new(1.0, 2.0, 3.0).normalize();
        let near_z = Vec3::new(1e-9, -2e-9, 1.0).normalize();
        for (n, u) in [
            (tilted, Vec3::new(2.0, -1.0, 0.0).normalize()),
            (
                near_z,
                Vec3::new(1.0, 0.0, -near_z.x / near_z.z).normalize(),
            ),
        ] {
            let v = n.cross(u);
            let (c, rho) = (Point3::new(0.3, -0.2, 1.1), 0.7);
            let b = circle_box(
                &super::super::separating::Circle {
                    center: c,
                    u_ref: u,
                    v_ref: v,
                    radius: rho,
                },
                0.0,
            );
            let (lo, hi) = ([b.min_x, b.min_y, b.min_z], [b.max_x, b.max_y, b.max_z]);
            let across = [
                (n.y, n.z), // x: the norm of n's y and z
                (n.z, n.x),
                (n.x, n.y),
            ];
            for (i, (centre, (nj, nk))) in [c.x, c.y, c.z].into_iter().zip(across).enumerate() {
                let half = rho * nj.hypot(nk);
                assert!(
                    (lo[i] - (centre - half)).abs() <= 1e-15
                        && (hi[i] - (centre + half)).abs() <= 1e-15,
                    "n {n:?}, axis {i}: [{}, {}] against the circle's own [{}, {}]",
                    lo[i],
                    hi[i],
                    centre - half,
                    centre + half
                );
            }
            for k in 0..720 {
                let t = f64::from(k) * core::f64::consts::TAU / 720.0;
                let p = c + u * (rho * t.cos()) + v * (rho * t.sin());
                assert!(
                    holds(&b, p),
                    "n {n:?}: the circle at t = {t} lies outside its box"
                );
            }
        }
    }

    /// **A DISPATCH row, not a locus row.** Every surface kind has a
    /// box, and none of them claims the world: a face far from the
    /// origin must be DEFINITELY disjoint from a box out at 1e6.
    /// That is the property the boolean operand gate rests on — a
    /// kind whose box were poison would overlap everything and the
    /// gate could never admit a pair — so it is asserted per kind
    /// rather than left to the arms' individual rows.
    #[test]
    fn every_surface_kind_has_a_sound_box_and_none_claims_the_world() {
        let far = Aabb {
            min_x: 1e6,
            min_y: 1e6,
            min_z: 1e6,
            max_x: 2e6,
            max_y: 2e6,
            max_z: 2e6,
        };
        let kinds = [
            plane_z0(),
            cyl_r(1.0),
            Surface::Sphere {
                center: Point3::origin(),
                radius: 1.0,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Cone {
                apex: Point3::origin(),
                axis: Vec3::unit_z(),
                half_angle: 0.5,
                u_ref: Vec3::unit_x(),
            },
            Surface::Torus {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                major_radius: 2.0,
                minor_radius: 0.5,
                u_ref: Vec3::unit_x(),
            },
        ];
        for s in kinds {
            let kind = s.kind();
            let (mut body, face) = arc_sector(1.0, core::f64::consts::PI);
            relabel(&mut body, face, s);
            let b = face_box(&body, face, pad(), witness_band()).unwrap();
            assert!(
                !b.min_x.is_nan(),
                "{kind:?} must have a box, got poison: {b:?}"
            );
            assert!(
                !b.overlaps(&far),
                "{kind:?}'s box reaches 1e6 away from a unit-scale face: {b:?}"
            );
        }
    }

    /// A real TORUS patch face over `u ∈ [u0, u1] × v ∈ [v0, v1]`,
    /// with the boundary the chart's own two families — parallels at
    /// `v0`/`v1` (circles ⊥ the axis, centred on it) and meridians at
    /// `u0`/`u1` (circles in a plane through the axis, centred on the
    /// spine) — and its certified pcurves minted.
    ///
    /// Both families are exactly what `geom_brep::chart_pcurve`'s
    /// torus arm has closed forms for, so the caches this fixture
    /// stores are the ones a revolve or a tube mints; the box arm
    /// reads them and nothing else.
    ///
    /// The descending sides run on a reversed carrier axis so their
    /// own parameters increase, exactly as the split lane mints them.
    pub(crate) fn torus_wall(
        center: Point3<f64>,
        axis: Vec3<f64>,
        u_ref: Vec3<f64>,
        major: f64,
        minor: f64,
        us: (f64, f64),
        vs: (f64, f64),
    ) -> (Body<f64>, FaceKey) {
        torus_wall_below(0.0, center, axis, u_ref, major, minor, us, vs)
    }

    /// [`torus_wall`] with its bottom parallel's carrier parameter run
    /// `periods` whole periods below the azimuth it covers.
    #[allow(clippy::too_many_arguments)] // `torus_wall`'s, and the offset
    pub(crate) fn torus_wall_below(
        periods: f64,
        center: Point3<f64>,
        axis: Vec3<f64>,
        u_ref: Vec3<f64>,
        major: f64,
        minor: f64,
        us: (f64, f64),
        vs: (f64, f64),
    ) -> (Body<f64>, FaceKey) {
        torus_wall_sensed(false, periods, center, axis, u_ref, major, minor, us, vs)
    }

    /// [`torus_wall`]'s loop under the chart's own normal (`sense: true`),
    /// which it runs CLOCKWISE: the face is then the rest of the torus,
    /// the patch's complement.
    #[allow(clippy::too_many_arguments)] // `torus_wall`'s
    pub(crate) fn torus_wall_clockwise(
        center: Point3<f64>,
        axis: Vec3<f64>,
        u_ref: Vec3<f64>,
        major: f64,
        minor: f64,
        us: (f64, f64),
        vs: (f64, f64),
    ) -> (Body<f64>, FaceKey) {
        torus_wall_sensed(true, 0.0, center, axis, u_ref, major, minor, us, vs)
    }

    /// The wall, its face carrying `sense`. The `mef` leaves the face a
    /// loop that runs clockwise in the chart `(u, v)`, whose normal is the
    /// torus's outward one; under `sense: false` the face's outward normal
    /// points into the tube, the loop runs counterclockwise under it, and
    /// the face is the patch.
    #[allow(clippy::too_many_arguments)] // `torus_wall`'s, the offset and the sense
    fn torus_wall_sensed(
        sense: bool,
        periods: f64,
        center: Point3<f64>,
        axis: Vec3<f64>,
        u_ref: Vec3<f64>,
        major: f64,
        minor: f64,
        (u0, u1): (f64, f64),
        (v0, v1): (f64, f64),
    ) -> (Body<f64>, FaceKey) {
        let shift = core::f64::consts::TAU * periods;
        let v_ref = axis.cross(u_ref);
        let e = |u: f64| u_ref * u.cos() + v_ref * u.sin();
        let on =
            |u: f64, v: f64| center + e(u) * (major + minor * v.cos()) + axis * (minor * v.sin());
        let mut body = Body::<f64>::new();
        // The seed FIRST: a surface added before it has a face is
        // orphan geometry, which `mvfs`'s tier-1 postcondition
        // rejects.
        let seed = body.mvfs(on(u0, v0), true).unwrap();
        let torus = body.add_surface(Surface::Torus {
            center,
            axis,
            major_radius: major,
            minor_radius: minor,
            u_ref,
        });
        // A PARALLEL at `v`: the torus cut by the plane ⊥ the axis
        // there. Ascending in `u` when `up`, descending otherwise.
        let parallel = |body: &mut Body<f64>, v: f64, up: bool| {
            let c = center + axis * (minor * v.sin());
            let radius = major + minor * v.cos();
            let plane = body.add_surface(Surface::Plane {
                origin: c,
                normal: axis,
                u_ref,
            });
            let (carrier, t0, t1) = if up {
                (
                    Curve3::circle(c, axis, radius, u_ref),
                    u0 - shift,
                    u1 - shift,
                )
            } else {
                (Curve3::circle(c, -axis, radius, e(u1)), 0.0, u1 - u0)
            };
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(torus, plane),
                    selector: geom_brep::CurveSelector::Witness(on((u0 + u1) * 0.5, v)),
                },
                carrier,
                param_start: t0,
                param_end: t1,
            }
        };
        // A MERIDIAN at `u`: the torus cut by the plane through the
        // axis at that azimuth. Ascending in `v` when `up`. The
        // descending one is the chord `mef` closes the wall with, whose
        // two faces are the seed and the wall: described as an image in
        // the torus's chart, which the wall wears, since no face wears
        // the cutting plane.
        let meridian = |body: &mut Body<f64>, u: f64, up: bool| {
            let spine = center + e(u) * major;
            let description = if up {
                EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(
                        torus,
                        body.add_surface(Surface::Plane {
                            origin: center,
                            normal: axis.cross(e(u)),
                            u_ref: e(u),
                        }),
                    ),
                    selector: geom_brep::CurveSelector::Witness(on(u, (v0 + v1) * 0.5)),
                }
            } else {
                EdgeDescriptionSpec::chart(torus)
            };
            let (carrier, t0, t1) = if up {
                (Curve3::circle(spine, e(u).cross(axis), minor, e(u)), v0, v1)
            } else {
                (
                    Curve3::circle(
                        spine,
                        axis.cross(e(u)),
                        minor,
                        e(u) * v1.cos() + axis * v1.sin(),
                    ),
                    0.0,
                    v1 - v0,
                )
            };
            EdgeCurveSpec {
                description,
                carrier,
                param_start: t0,
                param_end: t1,
            }
        };
        let bottom = parallel(&mut body, v0, true);
        let e_b = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                on(u1, v0),
                bottom,
                Tol::witness(),
            )
            .unwrap();
        let right = meridian(&mut body, u1, true);
        let e_r = body
            .mev(
                MevSite::Fan {
                    he1: e_b.he_minus,
                    he2: e_b.he_minus,
                },
                on(u1, v1),
                right,
                Tol::witness(),
            )
            .unwrap();
        let top = parallel(&mut body, v1, false);
        let e_t = body
            .mev(
                MevSite::Fan {
                    he1: e_r.he_minus,
                    he2: e_r.he_minus,
                },
                on(u0, v1),
                top,
                Tol::witness(),
            )
            .unwrap();
        let he = body
            .find_half_edge(seed.face, e_t.vertex, e_r.vertex)
            .unwrap();
        let left = meridian(&mut body, u0, false);
        let face = body
            .mef(
                MefSite::Chords {
                    he1: he,
                    he2: e_b.he_plus,
                },
                left,
                FaceSurface::Shared { key: torus, sense },
                Tol::witness(),
            )
            .unwrap()
            .face;
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        (body, face)
    }

    /// A CONE wall face: the patch `u ∈ [u0, u1] × z ∈ [z0, z1]` on
    /// the cone of half-angle `alpha` about `z` with its apex at the
    /// origin. Rims are the cone's own circles, sides its generators.
    pub(crate) fn cone_wall(
        alpha: f64,
        u0: f64,
        u1: f64,
        z0: f64,
        z1: f64,
    ) -> (Body<f64>, FaceKey) {
        let (mut body, face) = revolved_wall(&|z| z * alpha.tan(), u0, u1, z0, z1);
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: alpha,
            u_ref: Vec3::unit_x(),
        };
        relabel(&mut body, face, cone);
        (body, face)
    }

    /// **The face-level containment door keeps its two kinds of `Out`
    /// apart on a cone** (it lives here because this module's tests
    /// own the cone-wall fixture). A point on the MIRROR nappe is on
    /// the double-cone carrier the face's surface states, so
    /// `curved_face_placement` answers `Trim(Some(Out))` — a sibling
    /// face's incidence — and never `OffCarrier`, which the crossing
    /// layer reads as a contradiction of its own on-carrier
    /// certificate. A point off the carrier is `OffCarrier`, and a
    /// carrier point outside the azimuth window is `Trim(Some(Out))`.
    #[test]
    fn a_cone_band_places_the_mirror_nappe_on_its_carrier() {
        use crate::boolean::contain::{CurvedPlacement, FaceContainment, curved_face_placement};
        let alpha = 30.0_f64.to_radians();
        let (mut body, face) = cone_wall(alpha, 0.0, 1.0, 0.5, 1.0);
        let band = Band::linear(Tol::witness()).unwrap();
        crate::test_support_fixtures::set_sheet_sense(&mut body, face, true, Tol::witness());
        let at = |u: f64, z: f64, off: f64| {
            let rho = z.abs() * alpha.tan() + off;
            Point3::new(rho * u.cos(), rho * u.sin(), z)
        };
        let place = |p| curved_face_placement(&body, face, p, band).unwrap();
        assert_eq!(
            place(at(0.5, 0.75, 0.0)),
            CurvedPlacement::Trim(Some(FaceContainment::In))
        );
        assert_eq!(
            place(at(0.5, -0.75, 0.0)),
            CurvedPlacement::Trim(Some(FaceContainment::Out)),
            "the mirror nappe is on the carrier"
        );
        assert_eq!(place(at(0.5, 0.75, 1e-3)), CurvedPlacement::OffCarrier);
        assert_eq!(
            place(at(2.0, 0.75, 0.0)),
            CurvedPlacement::Trim(Some(FaceContainment::Out)),
            "outside the azimuth window"
        );
    }

    /// The cone arm, against the wall it bounds — the same claim the
    /// cylinder's locus row makes, and it needs the same sweep: the
    /// belly bulges past every chord of the boundary, and the axial
    /// range must cover the whole patch.
    #[test]
    fn a_cone_walls_locus_is_inside_its_box() {
        for &alpha_deg in &[10.0_f64, 30.0, 70.0] {
            let alpha = alpha_deg.to_radians();
            for &scale in &[0.002, 1.0, 40.0] {
                for span_deg in [30.0_f64, 170.0, 200.0, 350.0] {
                    let span = span_deg.to_radians();
                    let (z0, z1) = (0.4 * scale, 1.0 * scale);
                    let (body, face) = cone_wall(alpha, 0.0, span, z0, z1);
                    let b = face_box(&body, face, pad(), witness_band()).unwrap();
                    for i in 0..=64 {
                        let u = span * f64::from(i) / 64.0;
                        for j in 0..=8 {
                            let z = z0 + (z1 - z0) * f64::from(j) / 8.0;
                            let rho = z * alpha.tan();
                            let p = Point3::new(rho * u.cos(), rho * u.sin(), z);
                            assert!(
                                holds(&b, p),
                                "cone point (u = {u}, z = {z}) left the box \
                                 (α = {alpha_deg}°, span = {span_deg}°): {b:?}"
                            );
                        }
                    }
                }
            }
        }
    }

    /// **`ConeSlab`** — the FRUSTUM the face's axial window cuts. The
    /// axial coordinate takes no widening at all, and the radial
    /// half-width is the window's own far radius `z₁·tan α`, not the
    /// widest ring the whole cone reaches.
    ///
    /// Both rims are circles PERPENDICULAR to the axis, so their
    /// axial image is a point ([`cover_axial`]) and the window is
    /// exactly `[z0, z1]` whatever the azimuth span — which is what
    /// makes this row a formula in the fixture's own parameters
    /// rather than in a box around its boundary.
    #[test]
    fn the_cone_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        for &alpha_deg in &[10.0_f64, 30.0, 70.0] {
            let alpha = alpha_deg.to_radians();
            for &scale in &[0.002, 1.0, 40.0] {
                for span_deg in [30.0_f64, 170.0, 200.0, 350.0] {
                    let (z0, z1) = (0.4 * scale, 1.0 * scale);
                    let radius = z1 * alpha.tan();
                    let (body, face) = cone_wall(alpha, 0.0, span_deg.to_radians(), z0, z1);
                    let b = face_box(&body, face, pad, witness_band()).unwrap();
                    agrees_with_the_rule(
                        &b,
                        &Aabb {
                            min_x: -radius - pad,
                            min_y: -radius - pad,
                            min_z: z0 - pad,
                            max_x: radius + pad,
                            max_y: radius + pad,
                            max_z: z1 + pad,
                        },
                        scale,
                        &format!("the cone arm (α = {alpha_deg}°, span = {span_deg}°)"),
                    );
                }
            }
        }
    }

    /// The torus arm, against the whole torus. Like the ball's, this
    /// arm reads nothing from the boundary, so the honest locus to
    /// sample is every point of the tube — at a TILTED axis too,
    /// which is where the perpendicular half-extent
    /// `(R + r)·√(1 − aᵢ²) + r·|aᵢ|` is doing work rather than
    /// collapsing to `R + r`.
    #[test]
    fn a_toruss_whole_locus_is_inside_its_box() {
        for &(major, minor) in &[(2.0, 0.5), (0.01, 0.004), (60.0, 12.0)] {
            for axis in [Vec3::unit_z(), Vec3::new(1.0, 2.0, 3.0).normalize()] {
                let center = Point3::new(0.3 * major, -0.2 * major, 0.1 * major);
                let (u_ref, _) = axis.orthonormal_basis();
                let v_ref = axis.cross(u_ref);
                let (mut body, face) = arc_sector(major, core::f64::consts::PI);
                relabel(
                    &mut body,
                    face,
                    Surface::Torus {
                        center,
                        axis,
                        major_radius: major,
                        minor_radius: minor,
                        u_ref,
                    },
                );
                let b = face_box(&body, face, pad(), witness_band()).unwrap();
                for i in 0..=48 {
                    let theta = 2.0 * core::f64::consts::PI * f64::from(i) / 48.0;
                    let radial = u_ref * theta.cos() + v_ref * theta.sin();
                    for j in 0..=48 {
                        let phi = 2.0 * core::f64::consts::PI * f64::from(j) / 48.0;
                        let p = center
                            + radial * (major + minor * phi.cos())
                            + axis * (minor * phi.sin());
                        assert!(
                            holds(&b, p),
                            "torus point (θ = {theta}, φ = {phi}) left the box \
                             (R = {major}, r = {minor}, axis {axis:?}): {b:?}"
                        );
                    }
                }
            }
        }
    }

    /// The chart window a face's own stored caches state — the SAME
    /// walk `face_box` runs, entered here so a row can see the window
    /// itself. Not a second spelling: both go through
    /// [`face_window_steps`] and [`torus_chart_window`].
    fn read_window(
        body: &Body<f64>,
        face: FaceKey,
        major: f64,
        minor: f64,
    ) -> Option<TorusWindowPair<f64>> {
        torus_chart_window(&face_window_steps(body, face), major, minor)
    }

    /// A torus face bounded by two LONE full-meridian circles
    /// (outer + ring, no seam parallel) — **the R1 review arm's
    /// P6 fixture**, adopted as the row for the class it found; the
    /// seed takes the torus first, so each circle `mef` mints names
    /// the two faces it bounds.
    ///
    /// Both orientations give the SAME stored pcurves and the SAME
    /// hull, yet they denote complementary annuli, so that hull
    /// cannot contain both. Tier-1/2 validate and the mint accept
    /// the body; no kernel door mints one.
    fn lone_circle_annulus(forward: bool) -> (Body<f64>, FaceKey) {
        let (center, axis, u_ref, major, minor) =
            (Point3::origin(), Vec3::unit_z(), Vec3::unit_x(), 2.0, 0.5);
        let (u0, u1) = (0.0, 22.0_f64.to_radians());
        let v_ref = axis.cross(u_ref);
        let e = |u: f64| u_ref * u.cos() + v_ref * u.sin();
        let on =
            |u: f64, v: f64| center + e(u) * (major + minor * v.cos()) + axis * (minor * v.sin());
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(on(u0, 0.0), true).unwrap();
        let torus = body.add_surface(Surface::Torus {
            center,
            axis,
            major_radius: major,
            minor_radius: minor,
            u_ref,
        });
        // The seed wears the torus before any circle is minted on it:
        // each circle's description names the torus and its cap's plane,
        // the two faces `mef` gives it.
        body.set_face_surface(
            seed.face,
            FaceSurface::Shared {
                key: torus,
                sense: true,
            },
        )
        .expect("the seed face is the torus annulus");
        let meridian = |body: &mut Body<f64>,
                        u: f64|
         -> (EdgeCurveSpec<f64>, crate::geometry::SurfaceKey) {
            let spine = center + e(u) * major;
            let plane = Surface::Plane {
                origin: center,
                normal: axis.cross(e(u)),
                u_ref: e(u),
            };
            let pk = body.add_surface(plane);
            let ax = if forward {
                e(u).cross(axis)
            } else {
                axis.cross(e(u))
            };
            (
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::Intersection {
                        pair: SurfacePair::new(torus, pk),
                        selector: geom_brep::CurveSelector::Witness(on(u, core::f64::consts::PI)),
                    },
                    carrier: Curve3::circle(spine, ax, minor, e(u)),
                    param_start: 0.0,
                    param_end: core::f64::consts::TAU,
                },
                pk,
            )
        };
        let (m0, cap0_plane) = meridian(&mut body, u0);
        let cap0 = body
            .mef(
                MefSite::Lone {
                    r#loop: seed.r#loop,
                },
                m0,
                FaceSurface::Shared {
                    key: cap0_plane,
                    sense: true,
                },
                Tol::witness(),
            )
            .expect("mef Lone: the circular edge at u0");
        let eq_plane = body.add_surface(Surface::Plane {
            origin: center,
            normal: axis,
            u_ref,
        });
        let strut_spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(torus, eq_plane),
                selector: geom_brep::CurveSelector::Witness(on((u0 + u1) * 0.5, 0.0)),
            },
            carrier: Curve3::circle(center, axis, major + minor, u_ref),
            param_start: u0,
            param_end: u1,
        };
        let strut = body
            .mev(
                MevSite::Fan {
                    he1: cap0.he_plus,
                    he2: cap0.he_plus,
                },
                on(u1, 0.0),
                strut_spec,
                Tol::witness(),
            )
            .expect("mev strut along the outer equator to u1");
        let (m1, cap1_plane) = meridian(&mut body, u1);
        let _cap1 = body
            .mef(
                MefSite::Chords {
                    he1: strut.he_minus,
                    he2: strut.he_minus,
                },
                m1,
                FaceSurface::Shared {
                    key: cap1_plane,
                    sense: true,
                },
                Tol::witness(),
            )
            .expect("mef Chords self-loop: the circular edge at u1");
        body.kemr(strut.he_plus, strut.he_minus)
            .expect("kemr: the strut dies and the u1 circle becomes a ring");
        (body, seed.face)
    }

    /// **The two guards, on the fixture that needs them.** A face of
    /// lone full circles gets NO window in either orientation, and its
    /// box is exactly the whole tube — which contains both annuli, as
    /// no windowed box could.
    ///
    /// The fixture and its measurement are the R1 review arm's (P6);
    /// what it found is that the hull `[0, 22°] × [0, 2π]` leaves 1591
    /// of 1681 sampled surface points outside the box in the
    /// complementary orientation. Both guards fire here — the face
    /// carries a RING, and each loop WRAPS the `v` channel — so either
    /// alone would close it, and the row asserts the outcome rather
    /// than which guard spoke.
    #[test]
    fn a_torus_face_of_lone_circle_loops_gets_no_window() {
        let (major, minor) = (2.0, 0.5);
        let (center, axis, u_ref) = (Point3::origin(), Vec3::unit_z(), Vec3::unit_x());
        let v_ref = axis.cross(u_ref);
        for forward in [true, false] {
            let (mut body, face) = lone_circle_annulus(forward);
            crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
            assert!(
                body.get_face(face).unwrap().rings.len() == 1,
                "the fixture's face carries the ring the guard is about"
            );
            assert!(
                read_window(&body, face, major, minor).is_none(),
                "a face of lone full circles must get no window (forward={forward})"
            );
            // **Each guard, separately.** On this face both fire, so
            // the assertion above outlives either one; these two pin
            // them one at a time, on real caches.
            let steps = face_window_steps(&body, face);
            assert!(
                torus_chart_window(&[steps[0].clone()], major, minor).is_none(),
                "the WRAP guard alone must refuse the outer loop, which closes by \
                 going once round `v` (forward={forward})"
            );
            let (wall, wall_face) =
                torus_wall(center, axis, u_ref, major, minor, (0.3, 1.9), (-0.7, 0.8));
            let ok = face_window_steps(&wall, wall_face);
            assert!(
                torus_chart_window(&[ok[0].clone()], major, minor).is_some(),
                "that same wall windows as ONE loop"
            );
            assert!(
                torus_chart_window(&[ok[0].clone(), ok[0].clone()], major, minor).is_none(),
                "the RING guard alone must refuse it as TWO — a second loop can be \
                 pinned on another branch, and the hull would bound neither"
            );
            let b = face_box(&body, face, pad(), witness_band()).unwrap();
            let reach = |a: f64| (major + minor) * (1.0 - a * a).sqrt() + minor * a.abs();
            agrees_with_the_rule(
                &b,
                &Aabb {
                    min_x: center.x - reach(axis.x) - pad(),
                    min_y: center.y - reach(axis.y) - pad(),
                    min_z: center.z - reach(axis.z) - pad(),
                    max_x: center.x + reach(axis.x) + pad(),
                    max_y: center.y + reach(axis.y) + pad(),
                    max_z: center.z + reach(axis.z) + pad(),
                },
                major,
                &format!("the lone-circle face's box (forward={forward})"),
            );
            // Both annuli, and the box holds them both — which is the
            // whole reason no window may be claimed here.
            for (lo, hi) in [
                (0.0, 22.0_f64.to_radians()),
                (22.0_f64.to_radians(), core::f64::consts::TAU),
            ] {
                for i in 0..=40 {
                    let u = lo + (hi - lo) * f64::from(i) / 40.0;
                    let radial = u_ref * u.cos() + v_ref * u.sin();
                    for j in 0..=40 {
                        let v = core::f64::consts::TAU * f64::from(j) / 40.0;
                        let p =
                            center + radial * (major + minor * v.cos()) + axis * (minor * v.sin());
                        assert!(holds(&b, p), "the whole tube must hold both annuli: {p:?}");
                    }
                }
            }
        }
    }

    /// **A face whose caches are a MIX abandons the window** rather
    /// than narrowing to the half it can read. One half-edge's cache
    /// is dropped from an otherwise fully cached fixture; the window
    /// must go, not shrink.
    #[test]
    fn a_partly_cached_torus_face_gets_no_window() {
        let (major, minor) = (2.0, 0.5);
        let (u, v) = ((0.3, 1.9), (-0.7, 0.8));
        let (mut body, face) = torus_wall(
            Point3::origin(),
            Vec3::unit_z(),
            Vec3::unit_x(),
            major,
            minor,
            u,
            v,
        );
        let full = read_window(&body, face, major, minor).expect("the intact fixture windows");
        let LoopBoundary::Cycle { first } = body
            .get_loop(body.get_face(face).unwrap().outer)
            .unwrap()
            .boundary
        else {
            panic!("the fixture's face has a cycle")
        };
        let dropped = body.loop_cycle(first).unwrap()[1];
        body.pcurves.remove(dropped);
        assert!(
            read_window(&body, face, major, minor).is_none(),
            "a face with a MIX of cached and uncached half-edges must abandon the window, \
             not narrow to {full:?}"
        );
        let b = face_box(&body, face, pad(), witness_band()).unwrap();
        let reach = |a: f64| (major + minor) * (1.0 - a * a).sqrt() + minor * a.abs();
        agrees_with_the_rule(
            &b,
            &Aabb {
                min_x: -reach(0.0) - pad(),
                min_y: -reach(0.0) - pad(),
                min_z: -reach(1.0) - pad(),
                max_x: reach(0.0) + pad(),
                max_y: reach(0.0) + pad(),
                max_z: reach(1.0) + pad(),
            },
            major,
            "the partly cached face's box",
        );
    }

    /// **The certificate slack, and the bound it holds under.** The
    /// slack is `2·envelope/(R − r)` in `u` and `2·envelope/r` in `v`,
    /// and it is claimed only while the envelope stays inside the
    /// tube's own scale — an envelope past `r/2` or `(R − r)/2` gets
    /// NO window.
    ///
    /// The guard is unobservable on any minted fixture (an exact
    /// harmonic image certifies at ~1e-16 m), so the row plants the
    /// large envelope directly on the function that owns it.
    #[test]
    fn a_torus_windows_slack_is_guarded_by_the_bound_it_rests_on() {
        let (major, minor) = (2.0, 0.5);
        let (su, sv) = torus_window_slack(1e-9, major, minor).expect("a certified envelope");
        assert!(
            (su.hi - 2.0 * 1e-9 / (major - minor)).abs() < 1e-24,
            "{su:?}"
        );
        assert!((sv.hi - 2.0 * 1e-9 / minor).abs() < 1e-24, "{sv:?}");
        assert!((su.lo + su.hi).abs() < 1e-24 && (sv.lo + sv.hi).abs() < 1e-24);
        // Past the tube bound `r/2`, and past the ring bound
        // `(R − r)/2`, in turn — each alone must refuse.
        assert!(torus_window_slack(minor * 0.5 + 1e-9, major, minor).is_none());
        assert!(
            torus_window_slack((major - minor) * 0.5 + 1e-9, major, major - 1e-3).is_none(),
            "the ring bound refuses on its own"
        );
    }

    /// **The COMPOSED answer against ground truth** — the window read
    /// AND the extent, end to end on a real Euler-built body, checked
    /// against the face's own locus and the §Geometry oracle rather
    /// than against the read that produced it: the box holds the locus,
    /// and each side IS the oracle's extreme to within the window's
    /// certified slack.
    ///
    /// The ceiling rows restate the construction; this one does not
    /// look at it at all.
    #[test]
    fn a_real_torus_faces_box_holds_its_locus_and_stays_near_the_oracle() {
        // Pinned to limb 2's kernel limit at ε = 1e-12: the torus composite's conditioning
        // (`work/shell/the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding.md`).
        test_utils::kernel_limit::pinned_row(
            geom_core::Tol::witness().eps(),
            &[1e-12],
            "a_real_torus_faces_box_holds_its_locus_and_stays_near_the_oracle",
            || {
                let (major, minor) = (5.0, 0.06);
                let (center, axis, u_ref) = (
                    Point3::new(-major, 0.0, 0.0),
                    Vec3::new(0.0, -1.0, 0.0),
                    Vec3::unit_x(),
                );
                let v_ref = axis.cross(u_ref);
                let (u, v) = ((0.0, 22.0_f64.to_radians()), (-1.1, 2.4));
                let (body, face) = torus_wall(center, axis, u_ref, major, minor, u, v);
                let b = face_box(&body, face, 0.0, witness_band()).unwrap();
                for i in 0..=80 {
                    let uu = u.0 + (u.1 - u.0) * f64::from(i) / 80.0;
                    let radial = u_ref * uu.cos() + v_ref * uu.sin();
                    for j in 0..=80 {
                        let vv = v.0 + (v.1 - v.0) * f64::from(j) / 80.0;
                        let p = center
                            + radial * (major + minor * vv.cos())
                            + axis * (minor * vv.sin());
                        assert!(
                            holds(&b, p),
                            "the face's own locus left its box at ({uu}, {vv})"
                        );
                    }
                }
                // The window's certified slack and the extent's rounding.
                let tight = 1e-9;
                for (e, lo, hi) in [
                    (Vec3::unit_x(), b.min_x, b.max_x),
                    (Vec3::unit_y(), b.min_y, b.max_y),
                    (Vec3::unit_z(), b.min_z, b.max_z),
                ] {
                    let (o_lo, o_hi) = torus_oracle(center, axis, u_ref, major, minor, (u, v), e);
                    assert!(
                        lo <= o_lo + 1e-12 && hi >= o_hi - 1e-12,
                        "the box must contain the exact extremes along {e:?}"
                    );
                    assert!(
                        (lo - o_lo).abs() <= tight && (hi - o_hi).abs() <= tight,
                        "and be them to within {tight} along {e:?}: [{lo}, {hi}] against \
                         [{o_lo}, {o_hi}]"
                    );
                }
            },
        );
    }

    /// The chart windows the torus rows sweep: `u` spans of 22°,
    /// 100°, 200°, 350° and the full turn, against `v` spans of 60°,
    /// 200° and the full turn, each placed off the seam so no row
    /// passes by symmetry.
    fn torus_windows() -> Vec<((f64, f64), (f64, f64))> {
        let d = |x: f64| x.to_radians();
        let mut out = Vec::new();
        for us in [
            d(22.0),
            d(100.0),
            d(200.0),
            d(350.0),
            core::f64::consts::TAU,
        ] {
            for vs in [d(60.0), d(200.0), core::f64::consts::TAU] {
                out.push(((d(37.0), d(37.0) + us), (d(-71.0), d(-71.0) + vs)));
            }
        }
        out
    }

    /// The periodic distance from `x` to the window `[lo, hi]` —
    /// zero when `x` is inside it, and otherwise the shorter way round
    /// to an end. The oracle's `δ_u`, `δ_v`.
    fn dist_tau(x: f64, lo: f64, hi: f64) -> f64 {
        let tau = core::f64::consts::TAU;
        if (x - lo).rem_euclid(tau) <= hi - lo {
            return 0.0;
        }
        let to = |e: f64| {
            let d = (x - e).rem_euclid(tau);
            d.min(tau - d)
        };
        to(lo).min(to(hi))
    }

    /// **The §Geometry ORACLE**: the exact extremes of `e·S` over the
    /// chart rectangle, in closed form, spelled from the geometry
    /// rather than from [`torus_rect_extent`]: what the rows measure
    /// the lanes against, and not what any lane computes.
    ///
    /// At fixed `v`, `f = e·c + (R + r cos v)·a(u) + r sin v·e_n` is
    /// affine in `a(u) = |e_⊥|·cos(u − u*)` with POSITIVE slope
    /// (`R + r cos v > 0` on a ring torus), so `max_u f` sits at
    /// `a_max = |e_⊥|·cos δ_u`; then `g(v) = R·a_max + r·A·cos(v − φ)`
    /// with `A = hypot(a_max, e_n)` and `φ = atan2(e_n, a_max)`, whose
    /// window extreme is at the `v` nearest `φ` (nearest `φ + π` for
    /// the minimum, with `a_min` in place of `a_max`).
    fn torus_oracle(
        center: Point3<f64>,
        axis: Vec3<f64>,
        u_ref: Vec3<f64>,
        major: f64,
        minor: f64,
        ((u0, u1), (v0, v1)): ((f64, f64), (f64, f64)),
        e: Vec3<f64>,
    ) -> (f64, f64) {
        let v_ref = axis.cross(u_ref);
        let e_n = e.dot(axis);
        let perp = (e - axis * e_n).norm();
        let u_star = e.dot(v_ref).atan2(e.dot(u_ref));
        let base = (center - Point3::origin()).dot(e);
        let extreme = |a: f64, want_max: bool| {
            let amp = a.hypot(e_n);
            let phi = e_n.atan2(a);
            let near = if want_max {
                phi
            } else {
                phi + core::f64::consts::PI
            };
            let c = dist_tau(near, v0, v1).cos();
            if want_max {
                base + major * a + minor * amp * c
            } else {
                base + major * a - minor * amp * c
            }
        };
        let a_max = perp * dist_tau(u_star, u0, u1).cos();
        let a_min = -perp * dist_tau(u_star + core::f64::consts::PI, u0, u1).cos();
        (extreme(a_min, false), extreme(a_max, true))
    }

    /// The arm's construction over a chart window, at an `f64` point
    /// description: the closed form met with the whole tube, as both
    /// lanes read it.
    fn window_box(
        (center, axis, u_ref): (Point3<f64>, Vec3<f64>, Vec3<f64>),
        (major, minor): (f64, f64),
        (u, v): ((f64, f64), (f64, f64)),
    ) -> Aabb {
        let (c, ax) = (SpanBox::point(center), SpanBox::vector(axis));
        aabb_of(meet(
            ring_window_extent(
                &c,
                (
                    &SpanBox::vector(u_ref),
                    &SpanBox::vector(axis.cross(u_ref)),
                    &ax,
                ),
                (Span::exact(major), Span::exact(minor)),
                (Span { lo: u.0, hi: u.1 }, Span { lo: v.0, hi: v.1 }),
            ),
            torus_extent(&c, &ax, major, minor),
        ))
    }

    /// The rounding a closed-form side may carry: a few ulps of the
    /// largest term it sums, the centre's coordinate and the two radii.
    fn rounding(center: Point3<f64>, major: f64, minor: f64) -> f64 {
        let c = center.x.abs().max(center.y.abs()).max(center.z.abs());
        16.0 * f64::EPSILON * (c + major + minor)
    }

    /// **The locus row.** Every point of a dense grid over the chart
    /// rectangle, its edges and corners included, is inside the box the
    /// arm's construction gives it, padded by rounding alone, swept
    /// over the window shapes, three `(R, r)` and two axes.
    ///
    /// The closed form IS the support, so the grid's extreme points sit
    /// on the box's faces and nothing but rounding separates them: a
    /// boundary-hull-only box (a window holding a pole of some
    /// coordinate loses the bulge), a best azimuth read at the wrong
    /// latitude, a dropped radius term or a crest read on the wrong
    /// side of a window end each put grid points outside.
    #[test]
    fn a_torus_windows_locus_is_inside_its_box() {
        for &(major, minor) in &[(2.0, 0.5), (0.01, 0.004), (60.0, 12.0)] {
            for axis in [Vec3::unit_z(), Vec3::new(1.0, 2.0, 3.0).normalize()] {
                let center = Point3::new(0.3 * major, -0.2 * major, 0.1 * major);
                let (u_ref, _) = axis.orthonormal_basis();
                let v_ref = axis.cross(u_ref);
                for (u, v) in torus_windows() {
                    let b = window_box((center, axis, u_ref), (major, minor), (u, v))
                        .padded(rounding(center, major, minor));
                    let n = 64;
                    for i in 0..=n {
                        let uu = u.0 + (u.1 - u.0) * f64::from(i) / f64::from(n);
                        let radial = u_ref * uu.cos() + v_ref * uu.sin();
                        for j in 0..=n {
                            let vv = v.0 + (v.1 - v.0) * f64::from(j) / f64::from(n);
                            let p = center
                                + radial * (major + minor * vv.cos())
                                + axis * (minor * vv.sin());
                            assert!(
                                holds(&b, p),
                                "the chart point (u = {uu}, v = {vv}) of the window \
                                 {u:?}×{v:?} left the box (R = {major}, r = {minor}, \
                                 axis {axis:?}): {b:?}"
                            );
                        }
                    }
                }
            }
        }
    }

    /// **The window is TIGHT**: each side of the box is attained, to
    /// rounding, at a chart point known to be extremal, at the lily's
    /// own numbers — the 22° arc of a 5 m ring with a 0.06 m tube about
    /// `−ŷ` with `u_ref = x̂`, so `axis × u_ref = ẑ` and
    /// `S = c + (R + r·cos v)·(cos u, 0, sin u) − r·sin v·ŷ`.
    ///
    /// Over `u ∈ [0, 22°]` and every `v`: `x` is greatest at `(0, 0)`
    /// and least at `(22°, π)`, `y` is greatest at `v = −π/2` and least
    /// at `v = π/2`, and `z` is greatest at `(22°, 0)` and least, zero,
    /// all along `u = 0`. Each side is checked against `S` at that point
    /// and against the closed-form oracle, which agree; and the whole
    /// ring's box must be metres off on two sides, or the fixture is
    /// not the one this arm exists for.
    #[test]
    fn the_torus_window_box_is_attained_at_its_extremal_points() {
        use core::f64::consts::{FRAC_PI_2, PI, TAU};
        let (major, minor) = (5.0, 0.06);
        let center = Point3::new(-major, 0.0, 0.0);
        let axis = Vec3::new(0.0, -1.0, 0.0);
        let u_ref = Vec3::new(1.0, 0.0, 0.0);
        let v_ref = axis.cross(u_ref);
        let (u, v) = ((0.0, 22.0_f64.to_radians()), (-PI, PI));
        let b = window_box((center, axis, u_ref), (major, minor), (u, v));
        let whole = aabb_of(torus_extent(
            &SpanBox::point(center),
            &SpanBox::vector(axis),
            major,
            minor,
        ));
        let at = |uu: f64, vv: f64| {
            center
                + (u_ref * uu.cos() + v_ref * uu.sin()) * (major + minor * vv.cos())
                + axis * (minor * vv.sin())
        };
        let tol = rounding(center, major, minor);
        let sides = [
            ("min_x", b.min_x, at(u.1, PI).x, Vec3::unit_x(), false),
            ("max_x", b.max_x, at(0.0, 0.0).x, Vec3::unit_x(), true),
            (
                "min_y",
                b.min_y,
                at(0.0, FRAC_PI_2).y,
                Vec3::unit_y(),
                false,
            ),
            (
                "max_y",
                b.max_y,
                at(0.0, -FRAC_PI_2).y,
                Vec3::unit_y(),
                true,
            ),
            ("min_z", b.min_z, at(0.0, 1.3).z, Vec3::unit_z(), false),
            ("max_z", b.max_z, at(u.1, 0.0).z, Vec3::unit_z(), true),
        ];
        for (name, got, attained, e, max) in sides {
            let (o_lo, o_hi) = torus_oracle(center, axis, u_ref, major, minor, (u, v), e);
            let oracle = if max { o_hi } else { o_lo };
            assert!(
                (got - attained).abs() <= tol,
                "{name} is {got}, the extremal point's {attained}: off by {} (rounding {tol:e})",
                got - attained
            );
            assert!(
                (got - oracle).abs() <= tol,
                "{name} is {got}, the oracle's extreme {oracle}"
            );
        }
        assert!(
            (b.min_x - whole.min_x).abs() > 3.0 && (b.max_z - whole.max_z).abs() > 3.0,
            "the whole-ring box must be ≥ 3 m off on two sides, or this fixture is not the \
             artifact the unit exists for: {b:?} vs {whole:?}"
        );
        // The full turn of `v` as the window's own span, not by symmetry.
        let full = window_box((center, axis, u_ref), (major, minor), (u, (0.0, TAU)));
        assert!(
            (full.max_y - b.max_y).abs() <= tol && (full.min_y - b.min_y).abs() <= tol,
            "a full turn of v read from either start: {full:?} vs {b:?}"
        );
    }

    /// **A bracketed description is ENCLOSED, and no looser than its
    /// corners**: [`torus_rect_extent`] over spans of the frame's
    /// components and both radii holds every torus they admit, a grid
    /// varying all five independently through their interiors, and
    /// each end IS the most extreme corner's, since the family's
    /// extremes are attained at a corner ([`torus_rect_extent`]'s three
    /// facts). The corners are read by an independent oracle, a dense
    /// grid of `e·S` over the window, so the row does not grade the
    /// function with itself. A span product read at one end, or a
    /// corner of `(p, q)` not read, puts a member outside; a pad or a
    /// sampled charge leaves the ends off the corners.
    #[test]
    fn a_bracketed_torus_window_encloses_every_torus_it_admits() {
        let sp = |lo: f64, hi: f64| Span { lo, hi };
        let cases = [
            // p, q, a, R, r, u window, v window
            (
                sp(0.55, 0.62),
                sp(-0.31, -0.2),
                sp(0.7, 0.78),
                sp(1.9, 2.1),
                sp(0.45, 0.55),
                sp(0.3, 1.9),
                sp(-0.7, 0.8),
            ),
            (
                sp(-0.1, 0.12),
                sp(0.9, 0.99),
                sp(-0.2, 0.05),
                sp(4.99, 5.01),
                sp(0.05, 0.07),
                sp(-1.0, 3.0),
                sp(2.5, 9.0),
            ),
            (
                sp(-0.8, -0.6),
                sp(-0.5, 0.4),
                sp(-0.3, -0.25),
                sp(0.75, 0.8),
                sp(0.2, 0.25),
                sp(5.5, 7.0),
                sp(0.0, core::f64::consts::TAU),
            ),
        ];
        let ends = |s: Span<f64>| [s.lo, s.hi];
        let x = Span::exact;
        // `e·S − c` over the window on an `n × n` grid: a lower bound on
        // the greatest and an upper bound on the least, short of them by
        // the grid's chord error `(R + r)·h²/8` at most.
        let n = 160;
        let sampled =
            |(p, q, a, big, small): (f64, f64, f64, f64, f64), u: Span<f64>, v: Span<f64>| {
                let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
                for i in 0..=n {
                    let uu = u.lo + (u.hi - u.lo) * f64::from(i) / f64::from(n);
                    let along = p * uu.cos() + q * uu.sin();
                    for j in 0..=n {
                        let vv = v.lo + (v.hi - v.lo) * f64::from(j) / f64::from(n);
                        let f = (big + small * vv.cos()) * along + small * a * vv.sin();
                        lo = lo.min(f);
                        hi = hi.max(f);
                    }
                }
                (lo, hi)
            };
        for (k, &(p, q, a, big, small, u, v)) in cases.iter().enumerate() {
            let got = torus_rect_extent(x(0.0), (p, q, a), (big, small), (u, v));
            let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
            for p in ends(p) {
                for q in ends(q) {
                    for a in ends(a) {
                        for big in ends(big) {
                            for small in ends(small) {
                                let (slo, shi) = sampled((p, q, a, big, small), u, v);
                                lo = lo.min(slo);
                                hi = hi.max(shi);
                            }
                        }
                    }
                }
            }
            let h = (u.hi - u.lo).max(v.hi - v.lo) / f64::from(n);
            let chord = (big.hi + small.hi) * 2.0 * h * h / 8.0;
            assert!(
                got.lo <= lo && got.hi >= hi,
                "case {k}: [{}, {}] must enclose the corners' sampled [{lo}, {hi}]",
                got.lo,
                got.hi
            );
            assert!(
                lo - got.lo <= chord && got.hi - hi <= chord,
                "case {k}: [{}, {}] is looser than the corners' sampled [{lo}, {hi}] by more \
                 than the grid's chord error {chord:e}",
                got.lo,
                got.hi
            );
            let tol = 16.0 * f64::EPSILON * (big.hi + small.hi);
            let inner = |s: Span<f64>, i: u32| s.lo + (s.hi - s.lo) * f64::from(i) / 4.0;
            for i in 0..=4 {
                for j in 0..=4 {
                    for l in 0..=4 {
                        for m in 0..=4 {
                            for o in 0..=4 {
                                let one = torus_rect_extent(
                                    x(0.0),
                                    (x(inner(p, i)), x(inner(q, j)), x(inner(a, l))),
                                    (x(inner(big, m)), x(inner(small, o))),
                                    (u, v),
                                );
                                assert!(
                                    one.lo >= got.lo - tol && one.hi <= got.hi + tol,
                                    "case {k}: a member's [{}, {}] left the enclosure [{}, {}]",
                                    one.lo,
                                    one.hi,
                                    got.lo,
                                    got.hi
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    /// **The ring convention holds the closed form back, in both lanes
    /// alike.** A spindle (`R < r`) breaks the closed form's premise:
    /// `R + r·cos v` goes negative, and the best azimuth then depends on
    /// the latitude. Along `u_ref` over `u ∈ [−0.1, 0.1]` and every `v`,
    /// a spindle of `R = 0.3`, `r = 1` reaches `−0.700`, and the closed
    /// form claims only `−0.6965`, so it must never be read there.
    ///
    /// On a real face both lanes refuse it twice over: the chart window's
    /// own slack guard reads `R − r` as a length and gives no window for
    /// a spindle (held here on a ring face's caches read under a
    /// spindle's radii), and the convention (`geom::is_ring_torus`) is
    /// not decided. The window guard shadows the convention on a
    /// spindle, so the near-horn face, `R − r` inside the band where the
    /// window still reads, is what holds the convention alone: each lane
    /// must take the whole tube there, and a lane whose guard read the
    /// bracket or skipped the decision takes the rectangle.
    #[test]
    fn a_torus_outside_the_decided_ring_convention_keeps_the_whole_tube() {
        use core::f64::consts::PI;
        let (u, v) = ((-0.1, 0.1), (-PI, PI));
        // The premise is load-bearing: the closed form under-claims a
        // spindle, so the guard is what keeps it out.
        let (big, small) = (0.3, 1.0);
        let at = |uu: f64, vv: f64| (big + small * vv.cos()) * uu.cos();
        let mut least = f64::INFINITY;
        for i in 0..=200 {
            for j in 0..=2000 {
                let uu = u.0 + (u.1 - u.0) * f64::from(i) / 200.0;
                let vv = v.0 + (v.1 - v.0) * f64::from(j) / 2000.0;
                least = least.min(at(uu, vv));
            }
        }
        let x = Span::exact;
        let closed = torus_rect_extent(
            x(0.0),
            (x(1.0), x(0.0), x(0.0)),
            (x(big), x(small)),
            (Span { lo: u.0, hi: u.1 }, Span { lo: v.0, hi: v.1 }),
        );
        assert!(
            closed.lo > least + 1e-3,
            "the closed form's least {} must under-claim the spindle's {least}, or this row \
             no longer witnesses why the convention is decided",
            closed.lo
        );

        let band = witness_band();
        let pose = (
            Point3::new(0.4, -0.3, 0.2),
            Vec3::new(1.0, 2.0, 3.0).normalize(),
        );
        let u_ref = pose.1.orthonormal_basis().0;
        let window = ((0.3, 1.9), (-0.7, 0.8));
        let whole = |major: f64, minor: f64| {
            aabb_of(torus_extent(
                &SpanBox::point(pose.0),
                &SpanBox::vector(pose.1),
                major,
                minor,
            ))
        };
        // A ring face's certified caches, read under a spindle's radii:
        // the window's slack guard refuses, so no lane reaches the
        // closed form there whatever its convention reads.
        let (body, face) = torus_wall(pose.0, pose.1, u_ref, 2.0, 0.5, window.0, window.1);
        assert!(
            read_window(&body, face, 2.0, 0.5).is_some(),
            "the ring face's own window reads"
        );
        assert!(
            read_window(&body, face, big, small).is_none(),
            "a spindle's radii must give no window"
        );

        // The near-horn face: `R − r` inside the band, its window read.
        let minor = 0.5;
        let major = minor + 0.25 * band.zero();
        let (body, face) = torus_wall(pose.0, pose.1, u_ref, major, minor, window.0, window.1);
        assert!(
            read_window(&body, face, major, minor).is_some(),
            "the near-horn face's window must read, or this row does not hold the convention"
        );
        assert!(
            !geom::is_ring_torus(major, minor, band),
            "R − r = {:e} must lie inside the band",
            major - minor
        );
        let want = whole(major, minor);
        let boxed = face_box(&body, face, 0.0, band).unwrap();
        let (lo, hi) = crate::census::face_reach(&body, face, band).unwrap();
        agrees_with_the_rule(&boxed, &want, major, "face_box on the near-horn face");
        agrees_with_the_rule(
            &Aabb {
                min_x: lo.x,
                min_y: lo.y,
                min_z: lo.z,
                max_x: hi.x,
                max_y: hi.y,
                max_z: hi.z,
            },
            &want,
            major,
            "face_reach on the near-horn face",
        );
    }

    /// **The window is the EDGE'S OWN SPAN, and it is the stored rows'
    /// `chart_box` hull** — the window reads the one harmonic span box,
    /// so a 22° patch's `u` window is 22° wide and sits where the face
    /// does, and so is the hull the mint's check 5 measures the same
    /// rows by.
    #[test]
    fn the_torus_window_is_the_edges_own_span_and_its_chart_box_hull() {
        // Pinned to limb 2's kernel limit at ε = 1e-12: the torus composite's conditioning
        // (`work/shell/the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding.md`).
        test_utils::kernel_limit::pinned_row(
            geom_core::Tol::witness().eps(),
            &[1e-12],
            "the_torus_window_is_the_edges_own_span_and_its_chart_box_hull",
            || {
                let (major, minor) = (5.0, 0.06);
                let (u0, u1) = (0.0, 22.0_f64.to_radians());
                let (v0, v1) = (-0.4, 0.9);
                let (body, face) = torus_wall(
                    Point3::origin(),
                    Vec3::unit_z(),
                    Vec3::unit_x(),
                    major,
                    minor,
                    (u0, u1),
                    (v0, v1),
                );
                let (u, v) = read_window(&body, face, major, minor)
                    .expect("every half-edge stores a certified cache and the window reads");
                let hull = face_window_steps(&body, face)
                    .into_iter()
                    .flatten()
                    .map(|step| {
                        let (cache, image, _) =
                            step.expect("every half-edge stores a certified cache");
                        let (t0, t1) = cache.params();
                        image.chart_box(t0, t1)
                    })
                    .reduce(geom_brep::ChartWindow::hull)
                    .expect("the face has half-edges");
                // The slack is `2·envelope/(R − r)` and `2·envelope/r` on a
                // fixture whose images are exact, so it is at the mint's own
                // residual scale — orders below the spans themselves.
                let slack = 1e-6;
                for (got, span_end, boxed, what) in [
                    (u.lo, u0, hull.u_min, "u lo"),
                    (u.hi, u1, hull.u_max, "u hi"),
                    (v.lo, v0, hull.v_min, "v lo"),
                    (v.hi, v1, hull.v_max, "v hi"),
                ] {
                    assert!(
                        (got - span_end).abs() < slack,
                        "the {what} window end is {got}, the edge's own span end {span_end}"
                    );
                    assert!(
                        (got - boxed).abs() < slack,
                        "the {what} window end is {got}, the rows' chart_box hull end {boxed}"
                    );
                }
            },
        );
    }

    /// **`TorusWindow`**, both halves of the rule.
    ///
    /// *Windowless* — a face whose boundary stores no certified
    /// pcurve gets `(R + r)` perpendicular to the axis and `r` along
    /// it, reading nothing from the boundary, so the trim such a
    /// fixture carries must not appear in the box at all.
    ///
    /// *Windowed* — a face whose boundary DOES store them gets the
    /// chart rectangle's closed-form extremes met with that whole tube,
    /// and nothing else: the box is the construction in the fixture's
    /// own parameters, on all six faces in both directions.
    #[test]
    fn the_torus_arms_box_is_exactly_the_construction_its_rule_states() {
        // Pinned to limb 2's kernel limit at ε = 1e-12: the torus composite's conditioning
        // (`work/shell/the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding.md`).
        test_utils::kernel_limit::pinned_row(
            geom_core::Tol::witness().eps(),
            &[1e-12],
            "the_torus_arms_box_is_exactly_the_construction_its_rule_states",
            || {
                let pad = pad();
                for &(major, minor) in &[(2.0, 0.5), (0.01, 0.004), (60.0, 12.0)] {
                    for axis in [Vec3::unit_z(), Vec3::new(1.0, 2.0, 3.0).normalize()] {
                        let c = Point3::new(0.3 * major, -0.2 * major, 0.1 * major);
                        let u_ref = axis.orthonormal_basis().0;
                        let (mut body, face) = arc_sector(major, core::f64::consts::PI);
                        relabel(
                            &mut body,
                            face,
                            Surface::Torus {
                                center: c,
                                axis,
                                major_radius: major,
                                minor_radius: minor,
                                u_ref,
                            },
                        );
                        let b = face_box(&body, face, pad, witness_band()).unwrap();
                        // The room off each row as the norm of the axis's other
                        // two components, which does not cancel near a row.
                        let reach =
                            |a: f64, j: f64, k: f64| (major + minor) * j.hypot(k) + minor * a.abs();
                        let (rx, ry, rz) = (
                            reach(axis.x, axis.y, axis.z),
                            reach(axis.y, axis.z, axis.x),
                            reach(axis.z, axis.x, axis.y),
                        );
                        agrees_with_the_rule(
                            &b,
                            &Aabb {
                                min_x: c.x - rx - pad,
                                min_y: c.y - ry - pad,
                                min_z: c.z - rz - pad,
                                max_x: c.x + rx + pad,
                                max_y: c.y + ry + pad,
                                max_z: c.z + rz + pad,
                            },
                            major,
                            &format!(
                                "the windowless torus arm (R = {major}, r = {minor}, axis {axis:?})"
                            ),
                        );

                        let (u, v) = ((0.3, 1.9), (-0.7, 0.8));
                        let (body, face) = torus_wall(c, axis, u_ref, major, minor, u, v);
                        let b = face_box(&body, face, pad, witness_band()).unwrap();
                        // The window comes from the fixture's own caches (the
                        // fidelity row is what pins it to `[u0, u1] × [v0, v1]`);
                        // the CONSTRUCTION is restated here rather than called,
                        // through the oracle's own closed form, so this row can
                        // see a term go missing. The meet with the tube is
                        // restated too but cannot bind here: at a point
                        // description the rectangle lies inside the tube, and
                        // the meet binds only where a bracket's corners leave
                        // unit norm.
                        let (wu, wv) = read_window(&body, face, major, minor).unwrap();
                        let window = ((wu.lo, wu.hi), (wv.lo, wv.hi));
                        let side =
                            |e: Vec3<f64>| torus_oracle(c, axis, u_ref, major, minor, window, e);
                        let ((xl, xh), (yl, yh), (zl, zh)) = (
                            side(Vec3::unit_x()),
                            side(Vec3::unit_y()),
                            side(Vec3::unit_z()),
                        );
                        let want = Aabb {
                            min_x: xl.max(c.x - rx),
                            min_y: yl.max(c.y - ry),
                            min_z: zl.max(c.z - rz),
                            max_x: xh.min(c.x + rx),
                            max_y: yh.min(c.y + ry),
                            max_z: zh.min(c.z + rz),
                        }
                        .padded(pad);
                        agrees_with_the_rule(
                            &b,
                            &want,
                            major,
                            &format!(
                                "the windowed torus arm (R = {major}, r = {minor}, axis {axis:?})"
                            ),
                        );
                    }
                }
            },
        );
    }

    /// **The two box lanes, side by side on one body** — what #700
    /// asked for and nothing did.
    ///
    /// The face extents are shared, so on every arm but one this row
    /// is not guarding arithmetic: it guards what is NOT shared, the
    /// census's own arena walk of a face's boundary against this
    /// module's. Those two walks can drift — a loop order, an
    /// isolated-vertex loop, a half-edge's edge — and a divergence
    /// between the census's boxes and the boolean sweep's is exactly
    /// the shape that produces a wrong census verdict with both
    /// halves looking correct on their own.
    ///
    /// The conic EDGE box is one closed form on both lanes,
    /// `geom::curves::boxes`'s directional extremum, run certified at
    /// [`Interval`](geom_core::Interval) here and at the arena's own
    /// scalar in the census, so the row pins equality to rounding on
    /// every arm, the arcs' included. What that catches: a conic edge
    /// dropped from either lane's walk, and any widening of either
    /// box past a few ulps. What it does NOT catch: a dropped chord
    /// that the hull already covers (the sector's two radii both end
    /// at the origin, which the arc's box carries), so a walk drift
    /// is loud only where the edge it loses is load-bearing for the
    /// hull.
    ///
    /// Compared at `pad = 0`, where the only other difference the
    /// module admits is [`Aabb::padded`]'s outward ulp and the
    /// certified lane's outward rounding.
    #[test]
    fn the_two_box_lanes_agree_face_for_face() {
        let sphere = Surface::Sphere {
            center: Point3::new(0.45, -0.3, 0.15),
            radius: 1.5,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let tilt = Vec3::new(1.0, 2.0, 3.0).normalize();
        let torus = Surface::Torus {
            center: Point3::new(0.4, -0.3, 0.2),
            axis: tilt,
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: tilt.orthonormal_basis().0,
        };
        let relabelled = |s: Surface<f64>| {
            let (mut body, face) = arc_sector(1.0, core::f64::consts::PI);
            relabel(&mut body, face, s);
            (body, face)
        };
        let (nurbs_body, nurbs_face, _) = nurbs_bulge_face();
        // Each case carries its largest radius: a crest `c ± A` rounds at
        // the scale of its terms, `|c| + A ≤ |coordinate| + 2A`, whatever
        // the coordinate itself is. The NURBS net has no crest; its terms
        // are its control points, within a unit of the origin.
        type Case = (&'static str, f64, (Body<f64>, FaceKey));
        let cases: Vec<Case> = vec![
            ("plane", 2.0, arc_sector(2.0, 2.3)),
            ("cylinder", 1.5, cyl_wall(1.5, 0.0, 2.4, -0.5, 1.25)),
            // The top rim, `z = 1`, is the widest.
            ("cone", 0.5_f64.tan(), cone_wall(0.5, 0.0, 2.4, 0.4, 1.0)),
            ("sphere", 1.5, relabelled(sphere)),
            ("torus", 2.5, relabelled(torus)),
            // A face whose boundary DOES store certified pcurves: the
            // window read is a per-lane arena walk, so this case is
            // what holds the census's walk of it against this module's.
            (
                "torus window",
                2.5,
                torus_wall(
                    Point3::new(0.4, -0.3, 0.2),
                    tilt,
                    tilt.orthonormal_basis().0,
                    2.0,
                    0.5,
                    (0.3, 1.9),
                    (-0.7, 0.8),
                ),
            ),
            // `R − r` inside the band: both lanes must take the same
            // arm, the whole tube, though the window reads.
            (
                "near-horn torus window",
                2.5,
                torus_wall(
                    Point3::new(0.4, -0.3, 0.2),
                    tilt,
                    tilt.orthonormal_basis().0,
                    0.5 + 0.25 * witness_band().zero(),
                    0.5,
                    (0.3, 1.9),
                    (-0.7, 0.8),
                ),
            ),
            ("nurbs", 1.0, (nurbs_body, nurbs_face)),
        ];
        for (what, radius, (body, face)) in cases {
            let boxed = face_box(&body, face, 0.0, witness_band()).unwrap();
            let (lo, hi) = crate::census::face_reach(&body, face, witness_band())
                .unwrap_or_else(|| panic!("{what}: the census lane claims nothing"));
            // `census − boolean` outward per side: equal to rounding.
            for (name, gap, scale) in [
                ("min_x", boxed.min_x - lo.x, lo.x),
                ("min_y", boxed.min_y - lo.y, lo.y),
                ("min_z", boxed.min_z - lo.z, lo.z),
                ("max_x", hi.x - boxed.max_x, hi.x),
                ("max_y", hi.y - boxed.max_y, hi.y),
                ("max_z", hi.z - boxed.max_z, hi.z),
            ] {
                let tol = 4.0 * f64::EPSILON * (scale.abs() + 2.0 * radius);
                assert!(
                    gap >= -tol,
                    "{what}: the census box falls inside the boolean box at {name} by {} \
                     (past rounding, {tol:e})",
                    -gap
                );
                assert!(
                    gap <= tol,
                    "{what}: the census box exceeds the boolean box at {name} by {gap} \
                     (past rounding, {tol:e})"
                );
            }
        }
    }

    /// **A slab face with an unclaimable boundary edge has no census
    /// reach**: a cylinder or cone wall one of whose side generators is
    /// null scaffolding (no sound box) answers `None` from
    /// [`crate::census::face_reach`], never a box with a NaN end.
    #[test]
    fn a_slab_face_with_an_unclaimable_boundary_edge_has_no_census_reach() {
        for (what, (mut body, face)) in [
            ("cylinder", cyl_wall(1.5, 0.0, 2.4, -0.5, 1.25)),
            ("cone", cone_wall(0.5, 0.0, 2.4, 0.4, 1.0)),
        ] {
            assert!(
                crate::census::face_reach(&body, face, witness_band()).is_some(),
                "{what}: the described wall claims a reach"
            );
            let (curve, he) = body
                .edges
                .iter()
                .find_map(|(_, e)| {
                    let line = matches!(
                        body.get_curve_geom(e.curve)
                            .and_then(crate::null::CurveGeom::certified)
                            .map(|c| c.carrier()),
                        Some(Curve3::Line(_))
                    );
                    line.then_some((e.curve, e.he_plus))
                })
                .unwrap_or_else(|| panic!("{what}: a wall has a straight generator"));
            let ends = crate::null::NullEdge {
                below_end: body.half_edges[he].start,
                above_end: body.half_edge_end(he).unwrap(),
            };
            body.curves[curve] = crate::null::CurveGeom::NullScaffold(ends);
            let reach = crate::census::face_reach(&body, face, witness_band());
            assert!(
                reach.is_none(),
                "{what}: a null generator leaves the window unclaimed, got {reach:?}"
            );
        }
    }

    /// **Poison survives one poisoned END.** [`Span::mul`] folds four
    /// corner products; whether a `NaN` corner survives depends
    /// entirely on WHICH min/max the fold uses. `Real::min`
    /// propagates poison; `f64`'s inherent `min` RETURNS the non-NaN
    /// operand, so the same fold written against it would hand a
    /// finite product back for a description with a poisoned bracket
    /// end — and the poison box, which every door reads in its own
    /// fail-loud direction, would never be reached.
    ///
    /// Planted at the arithmetic because no body can carry a
    /// half-poisoned bracket at `f64`, where a bracket is a point.
    #[test]
    fn a_half_poisoned_span_poisons_its_product() {
        let poisoned = Span {
            lo: f64::NAN,
            hi: 1.0,
        };
        let finite = Span { lo: 2.0, hi: 3.0 };
        for (a, b, what) in [
            (poisoned, finite, "poisoned lo on the left"),
            (finite, poisoned, "poisoned lo on the right"),
            (
                Span {
                    lo: -1.0,
                    hi: f64::NAN,
                },
                finite,
                "poisoned hi on the left",
            ),
        ] {
            let p = a.mul(b);
            assert!(
                p.lo.is_nan() && p.hi.is_nan(),
                "{what}: a poisoned end must poison the product, got {p:?}"
            );
        }
        // And it reaches the box: a slab over a poisoned axial range
        // is the poison box, not a finite claim.
        let poison_h = Span {
            lo: f64::NAN,
            hi: 1.0,
        };
        let slab = slab_extent(
            &SpanBox::point(Point3::<f64>::origin()),
            &BoxFrame::World.unit(
                UnitVec3::new(Vec3::<f64>::unit_z(), BOX_CYLINDER_AXIS, witness_band()).unwrap(),
            ),
            poison_h,
            1.0,
        );
        assert!(
            slab.z.lo.is_nan() && slab.z.hi.is_nan(),
            "a poisoned axial range must poison the slab: {slab:?}"
        );
    }

    /// **The frustum tracks its window.** A cone face far from the
    /// apex must be boxed as the frustum that window cuts, not as the
    /// widest ring the cone reaches inside it: the radial half-extent
    /// at the near end is the NEAR radius, and pinning it at the far
    /// one is what named a germ pair for a lily tepal seam whose
    /// exact frustum cleared the carving ball.
    ///
    /// Stated as an inequality in the fixture's own numbers so it
    /// cannot pass by being loose: the box's radial half-width must
    /// not exceed the far radius, and it must be strictly less than
    /// what a constant-radius slab over the same window would claim.
    #[test]
    fn the_cone_arm_boxes_the_frustum_not_the_widest_ring() {
        let alpha = 0.4_f64;
        let (z0, z1) = (2.0, 2.5);
        let apex = SpanBox::point(Point3::<f64>::origin());
        let axis = SpanBox::vector(Vec3::<f64>::unit_z());
        let h = Span { lo: z0, hi: z1 };
        let b = cone_frustum_extent(&apex, &axis, h, alpha.tan());
        let far = z1 * alpha.tan();
        assert!(
            b.x.hi <= far * (1.0 + 1e-12) && b.x.hi >= far * (1.0 - 1e-12),
            "the widest coordinate reach is the FAR radius {far}, got {}",
            b.x.hi
        );
        assert!(
            b.z.lo >= z0 - 1e-12 && b.z.hi <= z1 + 1e-12,
            "the axial coordinate takes no widening at all: {b:?}"
        );
        // A window whose far end is ten times out: the frustum box
        // must NOT be the ten-times ring everywhere.
        let near = cone_frustum_extent(&apex, &axis, Span { lo: 0.1, hi: 0.2 }, alpha.tan());
        assert!(
            near.x.hi < 0.2 * alpha.tan() * 1.000_001,
            "a window near the apex must claim the near radius: {near:?}"
        );
    }

    /// **The bracket defects, planted at the arithmetic.** Both of
    /// #862's under-enclosures are properties of a DESCRIPTION whose
    /// coordinates are brackets — invisible at `f64`, where a bracket
    /// is a point — so they are planted here, on the extents the
    /// census lane reads (the boolean lane's conic edge box is `geom`'s
    /// exact door, pinned in that crate), rather than through a body
    /// at a scalar the row cannot pick.
    ///
    /// 1. The axial projection: an axis whose bracket spans two
    ///    directions must give an axial range that ENCLOSES what
    ///    either endpoint alone would give, never sit at one of them.
    ///    (It is [`cover_axial`] that does the projecting now —
    ///    per boundary edge, not over a box — and the bracket
    ///    question is the same one.)
    /// 2. The reference direction: a bracket that straddles zero, or
    ///    whose lower end is the larger in magnitude, must contribute
    ///    that larger magnitude — `hi()` alone under-claims.
    ///
    /// The slab's perpendicular room reads the same way round: an
    /// axis coordinate that is not CERTAINLY ±1 gets the room a
    /// perpendicular unit vector could take, never zero on the
    /// strength of one endpoint.
    #[test]
    fn a_bracketed_description_is_enclosed_not_sampled_at_one_endpoint() {
        let origin = SpanBox::point(Point3::<f64>::origin());
        // An axis known only to lie between (0, 0, 1) and (0.6, 0, 0.8).
        let iv = geom_core::Interval::from_bounds;
        let at =
            Cover::points(vec![Point3::new(iv(1.0, 1.0), iv(0.0, 0.0), iv(0.0, 0.0))]).unwrap();
        let (lo, hi) = cover_axial(
            &at,
            Point3::origin(),
            Vec3::new(iv(0.0, 0.6), iv(0.0, 0.0), iv(0.8, 1.0)),
        );
        assert!(
            lo.lo() <= 0.0 && hi.hi() >= 0.6,
            "the axial projection must enclose both endpoints' answers, got [{}, {}]",
            lo.lo(),
            hi.hi()
        );
        // Perpendicular room: `axis.z` is not certainly ±1, so the z
        // coordinate takes at least the room `√(1 − 0.8²) = 0.6` of the
        // radius. The slab reads the bracket's decided direction, which
        // the normalizing divide only widens.
        let unit = UnitVec3::new(
            Vec3::new(iv(0.0, 0.6), iv(0.0, 0.0), iv(0.8, 1.0)),
            BOX_CYLINDER_AXIS,
            witness_band(),
        )
        .unwrap();
        let slab = slab_extent(
            &origin,
            &UnitSpanBox::bracketed(unit),
            Span::exact(0.0),
            1.0,
        );
        assert!(
            slab.z.hi >= 0.6 - 1e-12,
            "an axis coordinate bracketed away from ±1 must keep its perpendicular \
             room, got {slab:?}"
        );
        // A reference direction whose lower end is the larger in
        // magnitude, and one straddling zero.
        let conic = conic_extent(
            &origin,
            &SpanBox {
                x: Span { lo: -1.0, hi: -0.9 },
                y: Span { lo: -0.4, hi: 0.4 },
                z: Span { lo: 0.0, hi: 0.0 },
            },
            &SpanBox::vector(Vec3::<f64>::unit_y()),
            2.0,
            0.0,
        );
        assert!(
            conic.x.hi >= 2.0 - 1e-12,
            "the reference direction's largest magnitude must be the one that \
             counts, got {conic:?}"
        );
        assert!(
            conic.y.hi >= 0.8 - 1e-12,
            "a bracket straddling zero must contribute its largest magnitude, \
             got {conic:?}"
        );
    }

    // ------------------------------------------------------------------
    // The adopted conic arm: `edge_box` reads the exact arc box.
    // ------------------------------------------------------------------

    /// The one conic edge of a sector body, with its certified curve.
    fn conic_edge(body: &Body<f64>) -> (EdgeKey, &geom_brep::EdgeCurve<f64>) {
        body.edges()
            .find_map(|(k, e)| {
                let c = body
                    .get_curve_geom(e.curve)
                    .and_then(crate::null::CurveGeom::certified)?;
                matches!(c.carrier(), Curve3::Circle(..) | Curve3::Ellipse(..)).then_some((k, c))
            })
            .unwrap()
    }

    /// A hand-built SPIRIC sector: one spiric arc `a → b` on the
    /// elbow's numbers (`R = 1.2`, `r = 0.225`, stand-off `d = 0.05`,
    /// the run `[0.4, 2.9]`), described as the plane × torus
    /// intersection and closed by two chords in the cap plane. The
    /// carrier is minted through the kind's deciding door.
    fn spiric_sector() -> (Body<f64>, FaceKey, EdgeKey) {
        spiric_sector_at(
            Point3::origin(),
            Vec3::unit_z(),
            Vec3::unit_x(),
            (1.2, 0.225, 0.05),
        )
    }

    /// [`spiric_sector`] about `center`, on the torus of `axis` whose
    /// cutting plane stands `d` off along `u_ref`.
    fn spiric_sector_at(
        center: Point3<f64>,
        axis: Vec3<f64>,
        u_ref: Vec3<f64>,
        (big_r, r, d): (f64, f64, f64),
    ) -> (Body<f64>, FaceKey, EdgeKey) {
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let carrier = Curve3::spiric(center, axis, u_ref, big_r, r, d, band).unwrap();
        let foot = center + u_ref * d;
        let plane_surface = Surface::Plane {
            origin: foot,
            normal: u_ref,
            u_ref: axis.cross(u_ref),
        };
        let torus_surface = Surface::Torus {
            center,
            axis,
            major_radius: big_r,
            minor_radius: r,
            u_ref,
        };
        let (t0, t1) = (0.4, 2.9);
        let (body, face) = conic_sector(
            plane_surface,
            torus_surface,
            carrier.clone(),
            (t0, t1),
            carrier.eval(0.5 * (t0 + t1)),
            (carrier.eval(t0), carrier.eval(t1), foot),
        );
        let edge = body
            .edges()
            .find_map(|(k, e)| {
                let c = body
                    .get_curve_geom(e.curve)
                    .and_then(crate::null::CurveGeom::certified)?;
                matches!(c.carrier(), Curve3::Spiric { .. }).then_some(k)
            })
            .unwrap();
        (body, face, edge)
    }

    /// A spiric's `f = √(ρ² − d²)` at `ρ = R + r·cos_v`, in
    /// double-double (the conic oracle's [`Dd`]) with the radicand
    /// factored as `(ρ − d)·(ρ + d)`: accurate far below an ulp of `f`.
    /// The reference the spiric rows hold the boxes to, because it does
    /// not round the radicand the way `Curve3::eval` and the box
    /// construction both do — near the fold that rounding moves `f` by
    /// far more than an ulp.
    ///
    /// [`Dd`]: crate::boolean::conic_oracle::exact::Dd
    fn spiric_f_dd(big_r: f64, r: f64, d: f64, cos_v: f64) -> Dd {
        let rho = Dd::of(big_r).add(Dd::of(r).mul(Dd::of(cos_v)));
        rho.sub(Dd::of(d)).mul(rho.add(Dd::of(d))).sqrt()
    }

    /// [`spiric_f_dd`] rounded to `f64`.
    fn spiric_f_reference(big_r: f64, r: f64, d: f64, cos_v: f64) -> f64 {
        spiric_f_dd(big_r, r, d, cos_v).to_f64()
    }

    /// **An edge's cover holds its end vertices where they stand off its
    /// carrier.** A vertex sits on its edge's carrier only to the edge's
    /// certification residual, so the edge's region is the carrier's
    /// cover over its span hulled with its two ends ([`edge_cover_of`]).
    /// Here the ends of a unit segment and of a quarter circle stand
    /// `3e-11` off their carriers, across their plane: the box read off
    /// the edge's cover holds both, where the carrier's cover alone
    /// ([`arc_cover`]) holds neither.
    #[test]
    fn an_edges_cover_holds_its_vertices_off_its_carrier() {
        let off = 3e-11;
        let n = Vec3::new(0.0, 0.0, 1.0);
        let at = Point3::new(1.0, 2.0, 3.0);
        let line = Curve3::Line(geom::Line {
            origin: at,
            dir: Vec3::new(1.0, 0.0, 0.0),
        });
        let circle = Curve3::circle(at, n, 2.0, Vec3::new(1.0, 0.0, 0.0));
        for (what, carrier, span) in [
            ("the segment", line, (0.0, 1.0)),
            (
                "the quarter circle",
                circle,
                (0.0, core::f64::consts::FRAC_PI_2),
            ),
        ] {
            let ends = (
                carrier.eval(span.0) + n * off,
                carrier.eval(span.1) - n * off,
            );
            let inside = |b: &Aabb, p: Point3<f64>| {
                (b.min_x..=b.max_x).contains(&p.x)
                    && (b.min_y..=b.max_y).contains(&p.y)
                    && (b.min_z..=b.max_z).contains(&p.z)
            };
            let edge = edge_cover_over(&carrier, span, ends).unwrap().aabb(0.0);
            let bare = arc_cover(&carrier, span).unwrap().aabb(0.0);
            for p in [ends.0, ends.1] {
                assert!(
                    inside(&edge, p),
                    "{what}: the edge's box {edge:?} misses its end {p:?}"
                );
                assert!(
                    !inside(&bare, p),
                    "{what}: the carrier's box holds {p:?} already"
                );
            }
        }
    }

    /// **The spiric arm of `edge_box`, and the census's reach twin,
    /// contain the arc** — the two readers of a spiric edge's cover,
    /// each executed on the hand-built sector over its window
    /// `[0.4, 2.9]`, which is what the cover claims, with no slack on
    /// either: each sample is the elbow's point
    /// `(d, f(v), r·sin v)` with `f` from [`spiric_f_reference`], not
    /// `Curve3::eval`, whose rounding of the radicand is the box's own.
    #[test]
    fn the_spiric_edge_box_and_reach_contain_a_dense_sample() {
        let (body, face, edge) = spiric_sector();
        let (big_r, r, d) = (1.2, 0.225, 0.05);
        let b = edge_box(&body, edge, 0.0);
        let (lo, hi) = crate::census::face_reach(&body, face, witness_band()).unwrap();
        let (t0, t1) = (0.4, 2.9);
        for i in 0..=20_000 {
            let t = t0 + (t1 - t0) * f64::from(i) / 20_000.0;
            let (sin, cos) = t.sin_cos();
            let p = Point3::new(d, spiric_f_reference(big_r, r, d, cos), r * sin);
            assert!(
                p.x >= b.min_x
                    && p.x <= b.max_x
                    && p.y >= b.min_y
                    && p.y <= b.max_y
                    && p.z >= b.min_z
                    && p.z <= b.max_z,
                "the spiric point at v = {t} ({p:?}) left the edge box {b:?}"
            );
            assert!(
                p.x >= lo.x
                    && p.x <= hi.x
                    && p.y >= lo.y
                    && p.y <= hi.y
                    && p.z >= lo.z
                    && p.z <= hi.z,
                "the spiric point at v = {t} ({p:?}) left the face reach {lo:?}..{hi:?}"
            );
        }
    }

    /// The admitted spirics the near-fold rows box: the elbow, the
    /// review's fold and its fat torus (`r/R = 0.9` at `R = 10⁴`), and a
    /// grid at scales 10 and 100 m, `r/R` from 0.02 to the fat 0.5 and
    /// 0.9, whose inner
    /// equator stands `k·escalate` inside the cutting plane for `k` from
    /// just past the deciding door's floor up to far from it. Each as
    /// `(what, R, r, d)`, on the torus of axis `ẑ` about the origin cut
    /// along `x̂`, so the exact box is `x = d`, `y ∈ [f_min, f_max]`,
    /// `z ∈ [−r, r]`. A grid point the door does not admit at this ε is
    /// left out; the row that reads this asserts how many are left.
    fn admitted_spirics(band: Band) -> Vec<(String, f64, f64, f64)> {
        let mut out = vec![
            ("the elbow".to_string(), 1.2, 0.225, 0.05),
            (
                "the review's fold".to_string(),
                788.172_981_855_614_4,
                5.350_502_372_890_727,
                782.822_479_472_187_4,
            ),
            ("the review's fat torus".to_string(), 1e4, 9e3, 1e3 - 1e-8),
        ];
        for &scale in &[10.0, 100.0] {
            for &ratio in &[0.02, 0.3, 0.5, 0.9] {
                for &k in &[1.01, 1.1, 2.0, 1e2, 1e4] {
                    let (big_r, r) = (scale, scale * ratio);
                    let d = big_r - r - k * band.escalate();
                    out.push((format!("R = {scale}, r = {r}, k = {k}"), big_r, r, d));
                }
            }
        }
        out.into_iter()
            .filter(|&(_, big_r, r, d)| {
                Curve3::spiric(
                    Point3::origin(),
                    Vec3::unit_z(),
                    Vec3::unit_x(),
                    big_r,
                    r,
                    d,
                    band,
                )
                .is_ok()
            })
            .collect()
    }

    /// **The spiric box is its exact extremes at every admitted spiric,
    /// near the fold included, on both lanes and in `geom`'s door.** The
    /// edge's cover ([`edge_cover_of`]) read as the box lane reads it
    /// ([`Cover::aabb`]), as the census reads it ([`Cover::box_in`]), and
    /// `geom::curves::boxes::spiric_arc_aabb` are each held, with no pad,
    /// to the exact box of [`admitted_spirics`] (`f` from
    /// [`spiric_f_reference`]) within a few ulps either way
    /// (`[simple.conditioned]`: the `f64` box is the real-number box): the
    /// two cover lanes to the edge's window's box, a window holding the
    /// fold's `v = π` and the far side's `v = 0`, and `geom`'s door, which boxes the whole period, to the
    /// period's. A root of the inner radicand spelled as a difference of
    /// squares, or of `fl(R − r) − |d|`, misses `f_min` near the fold by
    /// far more.
    #[test]
    fn the_spiric_box_is_its_exact_extremes_near_the_fold() {
        let band = witness_band();
        let fixtures = admitted_spirics(band);
        let near = fixtures
            .iter()
            .filter(|(_, big_r, r, d)| big_r - r - d < 3.0 * band.escalate())
            .count();
        assert!(
            near >= 5,
            "at least five admitted spirics stand within 3·escalate of the fold, got {near}"
        );
        let u = f64::EPSILON / 2.0;
        for (what, big_r, r, d) in fixtures {
            let carrier = Curve3::spiric(
                Point3::origin(),
                Vec3::unit_z(),
                Vec3::unit_x(),
                big_r,
                r,
                d,
                band,
            )
            .unwrap();
            let (t0, t1) = (-0.3, 3.3);
            let ends = (carrier.eval(t0), carrier.eval(t1));
            let cover = edge_cover_over(&carrier, (t0, t1), ends).unwrap();
            let bracketed = cover.aabb(0.0);
            let (lo, hi) = cover.box_in(&geom::cover::Frame::World);
            let census = Aabb {
                min_x: lo.x,
                min_y: lo.y,
                min_z: lo.z,
                max_x: hi.x,
                max_y: hi.y,
                max_z: hi.z,
            };
            let door = geom::curves::boxes::spiric_arc_aabb(&carrier, ends.0, ends.1).unwrap();

            // The window `[−0.3, 3.3]` holds `v = 0`, `π/2` and `π`, so
            // `f` turns at its crests inside it, not at an end, whose point
            // `Curve3::eval` reads with its own rounding of the radicand:
            // `cos v` runs over `[−1, 1]` and `sin v` from `sin(−0.3)` to
            // 1. Over the period, `[−1, 1]` each.
            let f = |cos: f64| spiric_f_reference(big_r, r, d, cos);
            let window = [d, f(-1.0), r * t0.sin(), d, f(1.0), r];
            let period = [d, f(-1.0), -r, d, f(1.0), r];
            for (lane, b, exact) in [
                ("the box lane", bracketed, window),
                ("the census lane", census, window),
                ("geom's door", door, period),
            ] {
                let got = [b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z];
                for k in 0..6 {
                    let out = if k < 3 {
                        exact[k] - got[k]
                    } else {
                        got[k] - exact[k]
                    };
                    let slop = 4.0 * u * exact[k].abs();
                    assert!(
                        out.abs() <= slop,
                        "{what}, {lane}, end {k}: the box is the exact extreme {:e} to a few \
                         ulps, got {:e} ({out:e} outside)",
                        exact[k],
                        got[k]
                    );
                }
            }
        }
    }

    /// **In an aimed frame, about a centre far from the origin, the
    /// spiric box holds the arc at `Interval` and is its support to a few
    /// ulps at `f64`.** The census lane reads an edge's cover in the frame
    /// it boxes in ([`Cover::box_in`]); this row runs that read at `f64`
    /// and with `Interval` rows and data (the frame's turn enclosed rather
    /// than rounded), on tori placed `|c| ≈ 10³–10⁴` m out, near the fold
    /// and fat, under two aims that lean off every world axis. Each
    /// coordinate is held, with no pad, to the arc's own support along
    /// that row: the point `c + (d, f(v), r·sin v)` (axis `ẑ`, cut along
    /// `x̂`) dotted with the row in double-double over a dense run of `v`
    /// across the edge's window and the one extremal angle in it
    /// (`v = π`), `f` from [`spiric_f_dd`].
    /// The `Interval` box contains each; the `f64` box falls short of
    /// none by more than a few ulps of the row's scale
    /// (`[simple.conditioned]`). A construction fed the world centre
    /// rather than the frame's reds this row by the centre's whole offset.
    #[test]
    fn the_spiric_box_holds_the_arc_in_an_aimed_frame_about_a_placed_centre() {
        let band = witness_band();
        let esc = band.escalate();
        let centres = [
            Point3::new(1234.5, -2718.25, 3141.0),
            Point3::new(-9000.125, 4000.5, 7500.75),
        ];
        let aims = [Vec3::new(1.0, 2.0, 3.0), Vec3::new(-0.3, 0.8, 0.52)];
        let tori = [
            (10.0, 3.0, 7.0 - 1.01 * esc),
            (100.0, 50.0, 30.0),
            (1e3, 900.0, 100.0 - 1.1 * esc),
            (1.2, 0.225, 0.05),
        ];
        let iv = |x: f64| Interval::from_bounds(x, x);
        let ivp = |p: Point3<f64>| Point3::new(iv(p.x), iv(p.y), iv(p.z));
        let ivv = |v: Vec3<f64>| Vec3::new(iv(v.x), iv(v.y), iv(v.z));
        let mut boxed = 0;
        for &c in &centres {
            for &aim in &aims {
                let frame = BoxFrame::aimed(UnitVec3::new(aim, "aim", band).unwrap());
                let BoxFrame::Aimed(rows) = frame else {
                    unreachable!("an aimed frame has rows")
                };
                let iframe: BoxFrame<Interval> =
                    BoxFrame::Aimed([ivv(rows[0]), ivv(rows[1]), ivv(rows[2])]);
                for &(big_r, r, d) in &tori {
                    let Ok(carrier) =
                        Curve3::spiric(c, Vec3::unit_z(), Vec3::unit_x(), big_r, r, d, band)
                    else {
                        continue;
                    };
                    boxed += 1;
                    let (t0, t1) = (2.0, 4.2);
                    let ends = (carrier.eval(t0), carrier.eval(t1));
                    let (lo, hi) = edge_cover_over(&carrier, (t0, t1), ends)
                        .unwrap()
                        .box_in(&cover_frame(&frame));
                    let at_f64 = Aabb {
                        min_x: lo.x,
                        min_y: lo.y,
                        min_z: lo.z,
                        max_x: hi.x,
                        max_y: hi.y,
                        max_z: hi.z,
                    };
                    let icarrier = carrier.map_scalar(iv);
                    let (lo, hi) =
                        edge_cover_over(&icarrier, (iv(t0), iv(t1)), (ivp(ends.0), ivp(ends.1)))
                            .unwrap()
                            .box_in(&cover_frame(&iframe));
                    let at_interval = Aabb {
                        min_x: lo.x.lo(),
                        min_y: lo.y.lo(),
                        min_z: lo.z.lo(),
                        max_x: hi.x.hi(),
                        max_y: hi.y.hi(),
                        max_z: hi.z.hi(),
                    };
                    let scale = (c - Point3::origin()).norm() + big_r + r;
                    let ulps = 16.0 * f64::EPSILON * scale;
                    let n = 4096;
                    let angles = (0..=n)
                        .map(|k| {
                            let (sin, cos) =
                                (t0 + (t1 - t0) * f64::from(k) / f64::from(n)).sin_cos();
                            (cos, sin)
                        })
                        .chain([(-1.0, 0.0)]);
                    for (cos, sin) in angles {
                        let f = spiric_f_dd(big_r, r, d, cos);
                        let p = [
                            Dd::of(c.x).add(Dd::of(d)),
                            Dd::of(c.y).add(f),
                            Dd::of(c.z).add(Dd::of(r).mul(Dd::of(sin))),
                        ];
                        let along = |row: Vec3<f64>| {
                            Dd::of(row.x)
                                .mul(p[0])
                                .add(Dd::of(row.y).mul(p[1]))
                                .add(Dd::of(row.z).mul(p[2]))
                                .to_f64()
                        };
                        let q = [along(rows[0]), along(rows[1]), along(rows[2])];
                        for (lane, b, slop) in
                            [("f64", at_f64, ulps), ("Interval", at_interval, 0.0)]
                        {
                            let (lo, hi) =
                                ([b.min_x, b.min_y, b.min_z], [b.max_x, b.max_y, b.max_z]);
                            for i in 0..3 {
                                assert!(
                                    lo[i] - slop <= q[i] && q[i] <= hi[i] + slop,
                                    "c = {c:?}, aim {aim:?}, torus ({big_r}, {r}, {d}), \
                                     {lane}: the arc's point at cos v = {cos} reads {:e} along \
                                     row {i}, outside the box's [{:e}, {:e}]",
                                    q[i],
                                    lo[i],
                                    hi[i]
                                );
                            }
                        }
                    }
                }
            }
        }
        assert!(
            boxed >= 12,
            "at least 12 placed, aimed spirics are boxed, got {boxed}"
        );
    }

    /// **The box lane's spiric box holds the arc near the fold, and so
    /// does `geom`'s door.** `f_min = √(ρ_min² − d²)` is the root of a
    /// radicand that vanishes at the fold. Spelled as a difference of
    /// squares, its rounding at `ρ²` scale reaches `f_min` amplified by
    /// `ρ²·u / (2·f_min)`, far more than any pad; posed as the product
    /// `(ρ − |d|)·(ρ + |d|)` with `ρ` carried as the unevaluated two-sum
    /// of `R − r` (so `s − |d|` is exact by Sterbenz), it keeps its
    /// digits. On a spiric the deciding door admits (`ρ_min − |d|` is
    /// 1.05·`escalate` at the default ε, far above it at 1e-12; every
    /// coordinate inside the session box), the arc's inner-equator point
    /// `center + u_ref·d + m·f_min` (`v = π`, inside the run) lies inside
    /// the box of the edge's cover padded by the sweep pad
    /// ([`edge_cover_over`], the construction [`edge_box`] reads: a
    /// near-fold sector does not certify), and `geom`'s unpadded door,
    /// which boxes the whole period, reaches it to a few ulps.
    ///
    /// The reference `f_min` is [`spiric_f_reference`]'s.
    #[test]
    fn the_spiric_box_holds_the_arc_near_the_fold() {
        let (big_r, r, d) = (
            788.172_981_855_614_4,
            5.350_502_372_890_727,
            782.822_479_472_187_4,
        );
        let band = witness_band();
        // Its fold margin is 1.054·escalate at the default ε: at a
        // coarser ε (the 1e-6 row) the deciding door refuses it as not
        // two ovals, and there is no spiric to box.
        let carrier = match Curve3::spiric(
            Point3::origin(),
            Vec3::unit_z(),
            Vec3::unit_x(),
            big_r,
            r,
            d,
            band,
        ) {
            Ok(carrier) => carrier,
            Err(refusal) => {
                assert!(
                    matches!(refusal, geom::SpiricInvalid::NotTwoOvals)
                        && big_r - r - d < 1.1 * band.escalate(),
                    "only the fold margin may refuse the review's spiric, got {refusal:?}"
                );
                test_utils::vacuity::stood_down(
                    "review fold refused at this ε",
                    "the near-fold box at this ε (the_spiric_box_is_its_exact_extremes_\
                     near_the_fold grids its folds from this ε's escalate)",
                );
                return;
            }
        };
        let (t0, t1) = (2.0, 4.2);
        let ends = (carrier.eval(t0), carrier.eval(t1));
        let boxed = edge_cover_over(&carrier, (t0, t1), ends)
            .unwrap()
            .aabb(pad());

        let f_min = spiric_f_reference(big_r, r, d, -1.0);

        let door = geom::curves::boxes::spiric_arc_aabb(&carrier, ends.0, ends.1).unwrap();
        assert!(
            door.min_y <= f_min * (1.0 + 4.0 * f64::EPSILON),
            "geom's door reaches the arc's inner equator to a few ulps: {:e} vs {f_min:e}",
            door.min_y
        );
        assert!(
            boxed.min_y <= f_min,
            "edge_box(…, sweep_pad) must hold the arc's inner-equator point (v = π, y = \
             {f_min:e}); its min_y is {:e}, {:e} m inside it (sweep_pad {:e})",
            boxed.min_y,
            boxed.min_y - f_min,
            pad()
        );
    }

    /// The conic's frame as `(centre, a·û, b·v̂)`: coordinate `i` of a
    /// point at parameter `t` is `centre_i + (a·û)_i·cos t + (b·v̂)_i·sin t`.
    fn conic_frame(carrier: &Curve3<f64>) -> (Point3<f64>, Vec3<f64>, Vec3<f64>) {
        match carrier {
            Curve3::Circle(geom::Circle {
                frame:
                    geom::Frame {
                        center,
                        axis,
                        u_ref,
                    },
                radius,
            }) => (*center, *u_ref * *radius, axis.cross(*u_ref) * *radius),
            Curve3::Ellipse(geom::Ellipse {
                frame:
                    geom::Frame {
                        center,
                        axis,
                        u_ref,
                    },
                semi_u,
                semi_v,
            }) => (*center, *u_ref * *semi_u, axis.cross(*u_ref) * *semi_v),
            other => panic!("a conic sector's rim is a conic, got {other:?}"),
        }
    }

    /// The exact range of `c + p·cos t + q·sin t` over `[t0, t1]`: the
    /// endpoints plus the extremal angle `atan2(q, p)` and its
    /// antipode wherever a `2π`-shift of one lands in the run.
    fn conic_coordinate_extremes(c: f64, p: f64, q: f64, t0: f64, t1: f64) -> (f64, f64) {
        let phi = q.atan2(p);
        arc_extremes_from(
            t0,
            t1 - t0,
            &|t: f64| c + p * t.cos() + q * t.sin(),
            &[phi, phi + core::f64::consts::PI],
        )
    }

    /// **The adversarial conic edges**: circles whose run crosses one
    /// extremal angle strictly mid-run, a near-full turn at an odd
    /// phase, a degenerate run, every one at a swept `u_ref`; and
    /// ellipses of axis ratio up to `1 / cos 89° ≈ 57` in a tilted
    /// plane, with runs crossing one, two and all four extrema. Each
    /// with the scale its tolerance is read at.
    fn adversarial_conic_sectors() -> Vec<(String, Body<f64>, f64)> {
        let tau = core::f64::consts::TAU;
        let deg = f64::to_radians;
        let mut out = Vec::new();
        for &r in &[0.001, 1.0, 250.0] {
            for &(what, start, span) in &[
                ("short", 0.0, deg(10.0)),
                // Short of a full turn by 0.02 rad: the span-winding
                // certificate's margin is `r` times the shortfall, and at
                // r = 0.001 that must clear the widest escalate band the
                // suite runs (1e-5 at ε = 1e-6) — 2e-5 does. The fixture
                // refuses from ε ≈ 3e-6 (escalate 3e-5), the decade in
                // which three of this module's older rows refuse too.
                ("near-full turn", 0.1, tau - 0.02),
                ("crosses +y mid-run", 1.4, 0.35),
                ("crosses -x mid-run", 2.9, 0.5),
                ("crosses -y mid-run", 4.5, 0.4),
                ("reflex", 0.7, deg(300.0)),
            ] {
                for phi_deg in [0.0_f64, 45.0, 137.0, 200.0] {
                    let (body, _) = circle_sector(r, start, span, deg(phi_deg));
                    out.push((format!("circle r={r} {what} u_ref at {phi_deg}°"), body, r));
                }
            }
        }
        // A degenerate run: a chord well above the band at every ε row
        // the suite runs, and an arc that moves by its sagitta only.
        for &r in &[1.0, 250.0] {
            let (body, _) = circle_sector(r, 0.3, 1e-4, deg(45.0));
            out.push((format!("circle r={r} degenerate run"), body, r));
        }
        for alpha_deg in [20.0_f64, 60.0, 80.0, 89.0] {
            let alpha = deg(alpha_deg);
            let scale = 1.0 / alpha.cos();
            for &(what, t0, t1) in &[
                ("no extremum", 0.2, 1.4),
                ("crosses +y and -x", 1.0, 4.0),
                ("crosses all four", 0.1, 6.2),
                ("degenerate run", 3.0, 3.0002),
            ] {
                let (body, _) = ellipse_sector(1.0, alpha, t0, t1);
                out.push((format!("ellipse alpha={alpha_deg}° {what}"), body, scale));
            }
        }
        out
    }

    /// **The adopted arm, soundness.** Every one of 20 000 samples of
    /// each adversarial arc lies inside its `edge_box` at zero pad, to
    /// a slack that is rounding and nothing else. This is the row a
    /// too-tight box fails, and a too-tight edge box is a soundness
    /// hole in every door that prunes on one.
    #[test]
    fn the_conic_edge_box_contains_a_dense_sample_of_adversarial_arcs() {
        // Pinned to limb 2's kernel limit at ε = 1e-12: a part near 1 km rounds at its own
        // extent past the band (`work/shell/eps-min-leaves-no-certification-headroom-at-the-box-edge.md`).
        test_utils::kernel_limit::pinned_row(
            geom_core::Tol::witness().eps(),
            &[1e-12],
            "the_conic_edge_box_contains_a_dense_sample_of_adversarial_arcs",
            || {
                for (what, body, scale) in adversarial_conic_sectors() {
                    let (edge, curve) = conic_edge(&body);
                    let b = edge_box(&body, edge, 0.0);
                    let (t0, t1) = curve.params();
                    let slack = 1e-12 * (1.0 + scale);
                    for i in 0..=20_000 {
                        let t = t0 + (t1 - t0) * f64::from(i) / 20_000.0;
                        let p = curve.carrier().eval(t);
                        assert!(
                            p.x >= b.min_x - slack
                                && p.x <= b.max_x + slack
                                && p.y >= b.min_y - slack
                                && p.y <= b.max_y + slack
                                && p.z >= b.min_z - slack
                                && p.z <= b.max_z + slack,
                            "{what}: the arc point at t = {t} ({p:?}) left the edge box {b:?}"
                        );
                    }
                }
            },
        );
    }

    /// **The adopted arm, ceiling.** The edge box at zero pad IS the
    /// exact extremal construction, per coordinate and per side: the
    /// endpoints, plus the coordinate's own extremum `c_i ± A_i`
    /// exactly when its angle lies in the run. No subdivision charge,
    /// no full-turn amplitude, no triangle bound — a box wider than
    /// this by more than rounding is a refusal at three of the four
    /// doors that read it.
    #[test]
    fn the_conic_edge_box_is_the_exact_extremal_construction() {
        // Pinned to limb 2's kernel limit at ε = 1e-12: a part near 1 km rounds at its own
        // extent past the band (`work/shell/eps-min-leaves-no-certification-headroom-at-the-box-edge.md`).
        test_utils::kernel_limit::pinned_row(
            geom_core::Tol::witness().eps(),
            &[1e-12],
            "the_conic_edge_box_is_the_exact_extremal_construction",
            || {
                for (what, body, scale) in adversarial_conic_sectors() {
                    let (edge, curve) = conic_edge(&body);
                    let b = edge_box(&body, edge, 0.0);
                    let (t0, t1) = curve.params();
                    let (c, p, q) = conic_frame(curve.carrier());
                    let tol = 1e-12 * (1.0 + scale);
                    for (name, got_lo, got_hi, ci, pi, qi) in [
                        ("x", b.min_x, b.max_x, c.x, p.x, q.x),
                        ("y", b.min_y, b.max_y, c.y, p.y, q.y),
                        ("z", b.min_z, b.max_z, c.z, p.z, q.z),
                    ] {
                        let (lo, hi) = conic_coordinate_extremes(ci, pi, qi, t0, t1);
                        assert!(
                            (got_lo - lo).abs() <= tol && (got_hi - hi).abs() <= tol,
                            "{what}: {name} is [{got_lo}, {got_hi}], the exact construction gives \
                         [{lo}, {hi}] (tolerance {tol})"
                        );
                    }
                }
            },
        );
    }

    // ------------------------------------------------------------------
    // Blinded-review probe rows (VERBS-CYLCYL PR-B, ordinal 80) — probe
    // branch only. An adversarial containment attack on the boundary
    // clip's wrap case.
    // ------------------------------------------------------------------

    /// **The clip's wrap case.** A wall window crossing azimuth 0
    /// (`u ∈ [5.9, 6.7]`, through `2π`): the boundary rims' arc boxes
    /// and the clipped face box must still contain the whole wall —
    /// the azimuth chart coordinate wraps here, which is the exact
    /// posture the "no interior extremum" argument has to survive. The
    /// census lane must agree face-for-face at the same window
    /// (pad = 0), and the box must actually BE clipped: the window
    /// reaches `+x` (cos hits 1 at `t = 2π`) but not `−x`/`±y` fully,
    /// so `min_x` must sit near `r·cos(5.9)`… clipped, not at `−r`.
    #[test]
    fn probe_wrap_window_wall_clip_is_sound_and_tight() {
        let (r, u0, u1, z0, z1) = (2.0, 5.9, 6.7, -0.25, 0.75);
        let (body, face) = cyl_wall(r, u0, u1, z0, z1);
        let pad = pad();
        let b = face_box(&body, face, pad, witness_band()).unwrap();
        // Soundness: dense wall samples inside.
        for i in 0..=400 {
            for j in 0..=40 {
                let u = u0 + (u1 - u0) * f64::from(i) / 400.0;
                let z = z0 + (z1 - z0) * f64::from(j) / 40.0;
                let p = Point3::new(r * u.cos(), r * u.sin(), z);
                assert!(
                    holds(&b, p),
                    "wall point (u = {u}, z = {z}) left the box: {b:?}"
                );
            }
        }
        // Tightness: the window never reaches −x (cos min there is
        // cos(5.9) ≈ 0.927), so a clip that silently kept the slab
        // (min_x = −r) fails here — and the rims' boxes are exact, so
        // the clipped face is the arc's own footprint to rounding.
        assert!(
            b.min_x >= r * (u0.cos().min(u1.cos())) - pad - 1e-9,
            "min_x {} is looser than the clipped construction",
            b.min_x
        );
        // The census mirror at the same wrap window reads the same
        // closed form for its rims, so it is this lane's box to
        // rounding (the two-lanes row's contract).
        let (lo, hi) =
            crate::census::face_reach(&body, face, witness_band()).expect("census claims the wall");
        let b0 = face_box(&body, face, 0.0, witness_band()).unwrap();
        for (name, gap, scale) in [
            ("min_x", b0.min_x - lo.x, lo.x),
            ("min_y", b0.min_y - lo.y, lo.y),
            ("min_z", b0.min_z - lo.z, lo.z),
            ("max_x", hi.x - b0.max_x, hi.x),
            ("max_y", hi.y - b0.max_y, hi.y),
            ("max_z", hi.z - b0.max_z, hi.z),
        ] {
            // A crest `c ± A` rounds at the scale of its terms,
            // `|c| + A ≤ |coordinate| + 2r`.
            let tol = 4.0 * f64::EPSILON * (scale.abs() + 2.0 * r);
            assert!(
                gap >= -tol && gap <= tol,
                "wrap window: the census box must be this lane's box to rounding at {name}, \
                 gap {gap}"
            );
        }
    }

    /// **Every extent reads coordinate `i` from coordinate `i` alone** —
    /// the premise [`BoxFrame`] rests on: an extent that mixed a second
    /// coordinate into the first would be exact in the world frame and
    /// unsound in an aimed one, which no world row could see. Each
    /// extent is run twice on inputs that agree in `x` and disagree in
    /// `y` and `z`, and its `x` span must not move by a bit. The axis's
    /// `y` and `z` are the one exception the premise names: they are
    /// turned and swapped, which keeps the norm of the axis off `x`
    /// ([`perp_room`]) to the bit.
    #[test]
    fn every_extent_reads_each_coordinate_from_that_coordinate_alone() {
        let v = |x: f64, y: f64, z: f64| SpanBox::vector(Vec3::new(x, y, z));
        let pt = |x: f64, y: f64, z: f64| SpanBox::point(Point3::new(x, y, z));
        let same = |what: &str, a: SpanBox<f64>, b: SpanBox<f64>| {
            assert!(
                a.x.lo.to_bits() == b.x.lo.to_bits() && a.x.hi.to_bits() == b.x.hi.to_bits(),
                "{what}: the x span moved with y and z: {:?} vs {:?}",
                a.x,
                b.x
            );
        };
        let h = Span { lo: -0.4, hi: 0.9 };
        // Two readings of the same x components: (0.3, …) etc.
        for (oy, oz, ay, az) in [(5.0, -2.0, 0.82, -0.5), (-7.0, 3.5, -0.5, -0.82)] {
            let (o0, o1) = (pt(0.3, 0.2, -0.1), pt(0.3, oy, oz));
            let (a0, a1) = (v(0.28, 0.5, 0.82), v(0.28, ay, az));
            let (u0, u1) = (v(0.6, -0.7, 0.2), v(0.6, az, oy));
            let (w0, w1) = (v(-0.3, 0.1, 0.9), v(-0.3, oz, ay));
            same(
                "slab_extent",
                slab_extent(&o0, &UnitSpanBox(a0), h, 0.7),
                slab_extent(&o1, &UnitSpanBox(a1), h, 0.7),
            );
            same(
                "cone_frustum_extent",
                cone_frustum_extent(&o0, &a0, h, 0.4),
                cone_frustum_extent(&o1, &a1, h, 0.4),
            );
            same("ball_extent", ball_extent(&o0, 0.7), ball_extent(&o1, 0.7));
            same(
                "torus_extent",
                torus_extent(&o0, &a0, 0.75, 0.25),
                torus_extent(&o1, &a1, 0.75, 0.25),
            );
            let window = (Span { lo: 0.3, hi: 2.9 }, Span { lo: -1.0, hi: 1.4 });
            same(
                "ring_window_extent",
                ring_window_extent(
                    &o0,
                    (&u0, &w0, &a0),
                    (Span::exact(0.75), Span::exact(0.25)),
                    window,
                ),
                ring_window_extent(
                    &o1,
                    (&u1, &w1, &a1),
                    (Span::exact(0.75), Span::exact(0.25)),
                    window,
                ),
            );
            same(
                "conic_extent",
                conic_extent(&o0, &u0, &w0, 0.9, 0.4),
                conic_extent(&o1, &u1, &w1, 0.9, 0.4),
            );
        }
    }

    /// **An axis near a row keeps its perpendicular room.** A
    /// cylinder, cone or torus (whole, or over a full chart window)
    /// whose axis is `t` off `ẑ` reaches
    /// `radius·sin t` along `z` from its axis point: the room a unit
    /// direction perpendicular to the axis has along `ẑ`. Spelled
    /// `√(1 − a_z²)`, the room cancelled and was exactly zero below
    /// `t ≈ 1e-8`, so a slab of radius `1e3` at `t = 1e-8` sat `1e-5`
    /// inside its circle along `z`, past any pad its readers take.
    #[test]
    fn an_axis_near_a_row_keeps_its_perpendicular_room() {
        let r = 1e3;
        let origin = SpanBox::point(Point3::origin());
        for t in [1e-12f64, 1e-9, 1e-8, 1e-7, 1e-4, 1e-2] {
            let tilted = Vec3::new(t.sin(), 0.0, t.cos());
            let axis = SpanBox::vector(tilted);
            let (u_ref, _) = tilted.orthonormal_basis();
            let v_ref = tilted.cross(u_ref);
            let turn = Span {
                lo: 0.0,
                hi: core::f64::consts::TAU,
            };
            let room = r * t.sin();
            // The cone's axis point at `h = r` is `r·cos t` up, read to
            // an ulp of `r`.
            let cone = cone_frustum_extent(&origin, &axis, Span::exact(r), 1.0)
                .z
                .hi;
            for (what, reach, slack) in [
                (
                    "slab_extent",
                    slab_extent(&origin, &UnitSpanBox(axis), Span::exact(0.0), r)
                        .z
                        .hi,
                    0.0,
                ),
                (
                    "cone_frustum_extent",
                    cone - r * t.cos(),
                    2.0 * r * f64::EPSILON,
                ),
                (
                    "torus_extent",
                    torus_extent(&origin, &axis, r, 0.0).z.hi,
                    0.0,
                ),
                (
                    "ring_window_extent",
                    ring_window_extent(
                        &origin,
                        (&SpanBox::vector(u_ref), &SpanBox::vector(v_ref), &axis),
                        (Span::exact(r), Span::exact(0.0)),
                        (turn, turn),
                    )
                    .z
                    .hi,
                    0.0,
                ),
            ] {
                assert!(
                    reach >= room * (1.0 - 4.0 * f64::EPSILON) - slack,
                    "{what}, tilt {t:e}: reaches {reach:e} along z from its axis point, short \
                     of the room {room:e}"
                );
            }
        }
    }

    /// **The aimed frame's rounding, measured.** A point's first
    /// coordinate is the dot product `n·p`, whose rounding is at most
    /// `γ₃·Σ|nᵢpᵢ| ≤ 3u·|p|` (about one and a half ulps of `|p|`): against
    /// an error-free reference (each product split exactly by `mul_add`,
    /// the sum carried in two parts), at `|p|` from a millimetre to
    /// `10⁸` m. The pad, `12 ε`, covers it wherever a coordinate of
    /// magnitude `|p|` resolves `ε` at all (`ulp(|p|) ≤ ε`), which is
    /// what [`BoxFrame`]'s docs lean on.
    #[test]
    fn the_aimed_frames_rounding_is_a_few_ulps_of_the_point() {
        let two_sum = |a: f64, b: f64| {
            let s = a + b;
            let bb = s - a;
            (s, (a - (s - bb)) + (b - bb))
        };
        let exact_dot = |n: Vec3<f64>, p: Point3<f64>| {
            let (mut hi, mut lo) = (0.0f64, 0.0f64);
            for (a, b) in [(n.x, p.x), (n.y, p.y), (n.z, p.z)] {
                let prod = a * b;
                let err = a.mul_add(b, -prod);
                let (s, e) = two_sum(hi, prod);
                hi = s;
                lo += e + err;
            }
            (hi, lo)
        };
        let band = witness_band();
        let mut worst = 0.0f64;
        for (k, scale) in [1e-3, 1.0, 1e3, 1e6, 1e8].into_iter().enumerate() {
            for i in 0..200u32 {
                let t = f64::from(i) * 0.731 + k as f64;
                let n = UnitVec3::new(
                    Vec3::new(t.sin(), (1.7 * t).cos(), (0.3 * t).sin() + 0.2),
                    BOX_CYLINDER_AXIS,
                    band,
                )
                .unwrap();
                let p = Point3::new(
                    scale * (2.3 * t).cos(),
                    scale * (1.1 * t).sin(),
                    scale * (0.7 * t + 1.0).cos(),
                );
                let got = BoxFrame::aimed(n).point(p).x;
                let (hi, lo) = exact_dot(n.get(), p);
                let norm = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt();
                let err = ((got - hi) - lo).abs();
                assert!(
                    err <= 3.0 * f64::EPSILON / 2.0 * norm,
                    "the aimed coordinate of {p:?} along {n:?} is off by {err}, over 3u·|p| = {}",
                    3.0 * f64::EPSILON / 2.0 * norm
                );
                worst = worst.max(err / norm);
            }
        }
        println!("the aimed frame's worst rounding: {worst:e}·|p|");
    }

    /// **The spiric edge's reach turns with the frame**: read in frames
    /// aimed off every world axis, the first coordinate of
    /// `census::edge_reach_in` holds `n·p` for a dense sample of the
    /// spiric edge over its span — the arm the split gate reads a spiric edge through. A
    /// world-frame row cannot see a component left unrotated here.
    #[test]
    fn the_spiric_edge_reach_holds_the_curve_in_an_aimed_frame() {
        let (body, _, edge) = spiric_sector();
        let curve = body
            .get_curve_geom(body.get_edge(edge).unwrap().curve)
            .and_then(crate::null::CurveGeom::certified)
            .unwrap();
        let band = witness_band();
        for aim in [
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.3, -0.4, 0.87),
            Vec3::new(-0.8, 0.2, 0.55),
            Vec3::new(0.6, 0.7, -0.4),
        ] {
            let n = UnitVec3::new(aim, BOX_CYLINDER_AXIS, band).unwrap();
            let (lo, hi) = crate::census::edge_reach_in(&body, edge, &BoxFrame::aimed(n)).unwrap();
            let (t0, t1) = curve.params();
            for i in 0..=20_000 {
                let t = t0 + (t1 - t0) * f64::from(i) / 20_000.0;
                let p = curve.carrier().eval(t);
                let along = n.get().dot(Vec3::new(p.x, p.y, p.z));
                assert!(
                    along >= lo.x - 1e-15 && along <= hi.x + 1e-15,
                    "along {aim:?}, the spiric point at v = {t} reads {along}, \
                     outside the reach [{}, {}]",
                    lo.x,
                    hi.x
                );
            }
        }
    }

    /// **The census's conic edge reach is the arc's extent in an aimed
    /// frame**: along each of a [`BoxFrame::aimed`] frame's rows, it
    /// contains 40 001 samples of the arc and sits within their spacing's
    /// bulge of them, for runs short of a half-turn to nearly a full
    /// one, from several reference directions.
    #[test]
    fn the_census_conic_edge_reach_is_the_arc_in_an_aimed_frame() {
        let band = witness_band();
        for (start, span, phi) in [
            (0.3, 2.3, 0.0),
            (-1.0, 3.0, 0.7),
            (2.0, 4.5, -1.3),
            (0.1, 6.0, 2.0),
        ] {
            let (body, _) = circle_sector(1.7, start, span, phi);
            let (edge, curve) = conic_edge(&body);
            let (t0, t1) = curve.params();
            for aim in [
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.3, -0.4, 0.87),
                Vec3::new(-0.8, 0.2, 0.55),
                Vec3::new(0.6, 0.7, -0.4),
                Vec3::new(1.0, 0.0, 0.0),
            ] {
                let n = UnitVec3::new(aim, BOX_CYLINDER_AXIS, band).unwrap();
                let frame = BoxFrame::aimed(n);
                let rows = frame.rows();
                let (lo, hi) = crate::census::edge_reach_in(&body, edge, &frame).unwrap();
                let (lo, hi) = ([lo.x, lo.y, lo.z], [hi.x, hi.y, hi.z]);
                let mut slo = [f64::INFINITY; 3];
                let mut shi = [f64::NEG_INFINITY; 3];
                for i in 0..=40_000 {
                    let t = t0 + (t1 - t0) * f64::from(i) / 40_000.0;
                    let p = curve.carrier().eval(t);
                    for k in 0..3 {
                        let a = rows[k].dot(Vec3::new(p.x, p.y, p.z));
                        slo[k] = slo[k].min(a);
                        shi[k] = shi[k].max(a);
                    }
                }
                for k in 0..3 {
                    assert!(
                        lo[k] <= slo[k] + 1e-14 && hi[k] >= shi[k] - 1e-14,
                        "[{start},{span},{phi}] aim {aim:?} row {k}: [{}, {}] vs samples [{}, {}]",
                        lo[k],
                        hi[k],
                        slo[k],
                        shi[k]
                    );
                    assert!(
                        slo[k] - lo[k] <= 1e-8 && hi[k] - shi[k] <= 1e-8,
                        "[{start},{span},{phi}] aim {aim:?} row {k}: loose [{}, {}] vs samples [{}, {}]",
                        lo[k],
                        hi[k],
                        slo[k],
                        shi[k]
                    );
                }
            }
        }
    }
}

/// **The chart rectangles' closed forms are their supports**:
/// [`torus_rect_extent`] on a ring torus and on a sphere (`R = 0`),
/// each against a dense grid of the rectangle's own points.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod rect_extent_rows {
    use core::f64::consts::PI;

    use geom_core::Vec3;

    use super::{Span, torus_rect_extent};

    /// The greatest and least of `f` over an `n × n` grid of
    /// `[a0, a1] × [b0, b1]`.
    fn sampled(
        n: usize,
        (a0, a1): (f64, f64),
        (b0, b1): (f64, f64),
        f: impl Fn(f64, f64) -> f64,
    ) -> (f64, f64) {
        let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
        for i in 0..=n {
            for j in 0..=n {
                let x = f(
                    a0 + (a1 - a0) * i as f64 / n as f64,
                    b0 + (b1 - b0) * j as f64 / n as f64,
                );
                lo = lo.min(x);
                hi = hi.max(x);
            }
        }
        (lo, hi)
    }

    /// [`torus_rect_extent`] of a point description about the origin.
    fn point_extent(
        (p, q, a): (f64, f64, f64),
        (major, minor): (f64, f64),
        window: (Span<f64>, Span<f64>),
    ) -> (f64, f64) {
        let x = Span::exact;
        let s = torus_rect_extent(x(0.0), (x(p), x(q), x(a)), (x(major), x(minor)), window);
        (s.lo, s.hi)
    }

    /// `(lo, hi)` encloses the sampled `(slo, shi)` and exceeds it by no
    /// more than the grid's own chord error `slack`.
    fn assert_support(what: &str, (lo, hi): (f64, f64), (slo, shi): (f64, f64), slack: f64) {
        assert!(
            lo <= slo + 1e-12 && hi >= shi - 1e-12,
            "{what}: [{lo}, {hi}] must enclose the samples [{slo}, {shi}]"
        );
        assert!(
            slo - lo <= slack && hi - shi <= slack,
            "{what}: [{lo}, {hi}] is looser than the samples [{slo}, {shi}] by more than {slack}"
        );
    }

    /// **A sphere's chart rectangle has its own extent along a
    /// direction**: [`torus_rect_extent`] with `R = 0`, against a dense
    /// grid of the rectangle's own points, for azimuth windows from a
    /// sliver to a full turn and across the seam's period, and latitude
    /// windows that reach a pole, hold the equator, and miss it on
    /// either side, along directions from the axis to across it.
    #[test]
    fn the_sphere_rect_extent_is_the_rectangles_support_along_every_direction() {
        let r = 1.25;
        // Axis `ŷ`, `u_ref = x̂`, so `axis × u_ref = −ẑ`.
        let point =
            |u: f64, v: f64| Vec3::new(r * v.cos() * u.cos(), r * v.sin(), -r * v.cos() * u.sin());
        for u in [
            (0.0, 2.0 * PI),
            (0.2, 1.4),
            (5.5, 7.0),
            (2.0, 2.05),
            (-1.0, 3.0),
        ] {
            for v in [
                (0.6435, PI / 2.0),
                (-PI / 2.0, -0.3),
                (-0.4, 0.9),
                (0.2, 0.5),
            ] {
                for e in [
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.3, 0.9, -0.2),
                    Vec3::new(-0.8, 0.1, 0.5),
                    Vec3::new(0.2, -0.7, 0.6),
                    Vec3::new(0.0, 0.0, -1.0),
                ] {
                    let e = e * (1.0 / e.norm());
                    let samples = sampled(300, u, v, |a, b| e.dot(point(a, b)));
                    let span = |(lo, hi)| Span { lo, hi };
                    let got = point_extent((e.x, -e.z, e.y), (0.0, r), (span(u), span(v)));
                    assert_support(
                        &format!("window {u:?} × {v:?}, e = {e:?}"),
                        got,
                        samples,
                        1e-4,
                    );
                }
            }
        }
    }

    /// **A ring torus's chart rectangle has its own extent along a
    /// direction**: against a dense grid of the rectangle's own points,
    /// for a full turn, a window that holds no crest, one across the
    /// seam's period and one wider than a turn, along tilted directions
    /// on both sides of the axis.
    #[test]
    fn the_torus_rect_extent_is_the_rectangles_support_along_every_direction() {
        let (major, minor) = (0.75, 0.25);
        // Axis `ŷ`, `u_ref = x̂`, so `axis × u_ref = −ẑ`.
        let point = |u: f64, v: f64| {
            let rho = major + minor * v.cos();
            Vec3::new(rho * u.cos(), minor * v.sin(), -rho * u.sin())
        };
        for (u, v) in [
            ((0.0, 2.0 * PI), (0.0, PI / 2.0)),
            ((1.0, 2.5), (-0.5, 2.0)),
            ((5.5, 7.0), (3.0, 4.0)),
            ((-1.0, 6.0), (2.5, 9.0)),
        ] {
            for e in [
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.3, 0.9, -0.2),
                Vec3::new(-0.8, 0.1, 0.5),
                Vec3::new(0.2, -0.7, 0.6),
                Vec3::new(0.0, 0.0, -1.0),
            ] {
                let e = e * (1.0 / e.norm());
                let samples = sampled(600, u, v, |a, b| e.dot(point(a, b)));
                let span = |(lo, hi)| Span { lo, hi };
                let got = point_extent((e.x, -e.z, e.y), (major, minor), (span(u), span(v)));
                assert_support(
                    &format!("window {u:?} × {v:?}, e = {e:?}"),
                    got,
                    samples,
                    1e-4,
                );
            }
        }
    }
}

/// **A sphere face's box is its chart rectangle's support**, read
/// alike by both box lanes, and **a torn record under it is a torn
/// body**, on a sphere zone sheet whose boundary certifies a side.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub(crate) mod sphere_rect_rows {
    use geom::{Curve3, Surface};
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{BoxFrame, face_box, sphere_reach};
    use crate::body::Body;
    use crate::construction::{FaceSurface, MefSite, MevSite};
    use crate::entity::{EntityId, FaceKey, GeomRef, LoopBoundary};
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};

    /// The unit sphere's zone over azimuths `[u0, u1]` and heights
    /// `[h0, h1]`: two latitude rims and two meridian arcs, each on its
    /// exact circle, described as the sphere cut by the circle's plane.
    pub(crate) fn sphere_zone_sheet(
        (u0, u1): (f64, f64),
        (h0, h1): (f64, f64),
        tol: Tol,
    ) -> (Body<f64>, FaceKey) {
        let radial = |u: f64| Vec3::new(u.cos(), u.sin(), 0.0);
        let at = |u: f64, h: f64| {
            let rho = (1.0 - h * h).sqrt();
            Point3::origin() + radial(u) * rho + Vec3::unit_z() * h
        };
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(at(u0, h0), true).unwrap();
        let sphere = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: Surface::Sphere {
                        center: Point3::origin(),
                        radius: 1.0,
                        axis: Vec3::unit_z(),
                        u_ref: Vec3::unit_x(),
                    },
                    sense: true,
                },
            )
            .unwrap();
        let arc = |body: &mut Body<f64>, carrier: Curve3<f64>, (t0, t1): (f64, f64)| {
            let Curve3::Circle(geom::Circle {
                frame: geom::Frame { center, axis, .. },
                ..
            }) = carrier
            else {
                unreachable!("a sheet edge is a circle");
            };
            let plane = body.add_surface(Surface::Plane {
                origin: center,
                normal: axis,
                // A rim's plane is horizontal and a meridian's vertical.
                u_ref: if axis.z.abs() > 0.5 {
                    Vec3::unit_x()
                } else {
                    Vec3::unit_z()
                },
            });
            let witness = carrier.mid_point(t0, t1);
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: geom_brep::SurfacePair::new(sphere, plane),
                    selector: geom_brep::CurveSelector::Witness(witness),
                },
                carrier,
                param_start: t0,
                param_end: t1,
            }
        };
        let rim = |h: f64, ascending: bool| {
            let rho = (1.0 - h * h).sqrt();
            let centre = Point3::new(0.0, 0.0, h);
            if ascending {
                let c = Curve3::circle(centre, Vec3::unit_z(), rho, Vec3::unit_x());
                (c, (u0, u1))
            } else {
                let c = Curve3::circle(centre, -Vec3::unit_z(), rho, radial(u1));
                (c, (0.0, u1 - u0))
            }
        };
        // A meridian at azimuth `u`, rising (its parameter is the
        // latitude) or falling (the negated latitude).
        let meridian = |u: f64, rising: bool| {
            let up = radial(u).cross(Vec3::unit_z());
            if rising {
                let c = Curve3::circle(Point3::origin(), up, 1.0, radial(u));
                (c, (h0.asin(), h1.asin()))
            } else {
                let c = Curve3::circle(Point3::origin(), -up, 1.0, radial(u));
                (c, (-h1.asin(), -h0.asin()))
            }
        };
        let (c, t) = rim(h0, true);
        let bottom = arc(&mut body, c, t);
        let e_b = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                at(u1, h0),
                bottom,
                tol,
            )
            .unwrap();
        let (c, t) = meridian(u1, true);
        let right = arc(&mut body, c, t);
        let e_r = body
            .mev(
                MevSite::Fan {
                    he1: e_b.he_minus,
                    he2: e_b.he_minus,
                },
                at(u1, h1),
                right,
                tol,
            )
            .unwrap();
        let (c, t) = rim(h1, false);
        let top = arc(&mut body, c, t);
        let e_t = body
            .mev(
                MevSite::Fan {
                    he1: e_r.he_minus,
                    he2: e_r.he_minus,
                },
                at(u0, h1),
                top,
                tol,
            )
            .unwrap();
        let he = body
            .find_half_edge(seed.face, e_t.vertex, e_r.vertex)
            .unwrap();
        let (c, t) = meridian(u0, false);
        let left = arc(&mut body, c, t);
        let face = body
            .mef(
                MefSite::Chords {
                    he1: he,
                    he2: e_b.he_plus,
                },
                left,
                FaceSurface::Shared {
                    key: sphere,
                    sense: true,
                },
                tol,
            )
            .unwrap()
            .face;
        // The boundary runs clockwise about the outward normal, so the
        // face's side of it is the sense-false one.
        body.set_face_sense(face, false).unwrap();
        crate::pcurves::mint_pcurves(&mut body, tol).unwrap();
        (body, face)
    }

    /// The sheet's support along unit `e`, by a dense grid of its own
    /// points: the unit sphere about the origin, polar axis `ẑ`,
    /// `u_ref = x̂`, over `u ∈ [u0, u1]` and `z ∈ [h0, h1]`.
    fn sampled_support((u0, u1): (f64, f64), (h0, h1): (f64, f64), e: Vec3<f64>) -> (f64, f64) {
        let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
        let n = 600;
        for i in 0..=n {
            let u = u0 + (u1 - u0) * f64::from(i) / f64::from(n);
            for j in 0..=n {
                let v = h0.asin() + (h1.asin() - h0.asin()) * f64::from(j) / f64::from(n);
                let x = e.dot(Vec3::new(v.cos() * u.cos(), v.cos() * u.sin(), v.sin()));
                lo = lo.min(x);
                hi = hi.max(x);
            }
        }
        (lo, hi)
    }

    /// **The sheet's box is its rectangle's own support, in both
    /// lanes and in a turned frame**: along each world axis (the
    /// boolean lane's box, at no pad, and the census's reach) and along
    /// tilted aims (the census's reach in the aimed frame), the box
    /// encloses a dense grid of the sheet and exceeds it by no more than
    /// the grid's chord error. The ring of the sheet's latitudes, which
    /// the zone alone would claim, reaches `x = −ρ` and `y = −ρ`, about
    /// `0.95` past the sheet's least `x` and `y`.
    #[test]
    fn a_sphere_faces_box_is_its_chart_rectangles_support() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (u, h) = ((0.2, 1.4), (-0.3, 0.4));
        let (body, face) = sphere_zone_sheet(u, h, tol);
        let close = |what: &str, (lo, hi): (f64, f64), e: Vec3<f64>| {
            let (slo, shi) = sampled_support(u, h, e);
            assert!(
                lo <= slo + 1e-12 && hi >= shi - 1e-12,
                "{what}: [{lo}, {hi}] must enclose the sheet's [{slo}, {shi}]"
            );
            assert!(
                slo - lo <= 1e-5 && hi - shi <= 1e-5,
                "{what}: [{lo}, {hi}] is looser than the sheet's [{slo}, {shi}]"
            );
        };
        let boxed = face_box(&body, face, 0.0, band).unwrap();
        let (lo, hi) = crate::census::face_reach(&body, face, band).unwrap();
        for (k, e) in [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()]
            .into_iter()
            .enumerate()
        {
            let pick = |p: Point3<f64>| [p.x, p.y, p.z][k];
            let b = [
                (boxed.min_x, boxed.max_x),
                (boxed.min_y, boxed.max_y),
                (boxed.min_z, boxed.max_z),
            ][k];
            close(&format!("the boolean lane's box, axis {k}"), b, e);
            close(
                &format!("the census lane's reach, axis {k}"),
                (pick(lo), pick(hi)),
                e,
            );
        }
        for aim in [
            Vec3::new(0.3, 0.9, -0.2),
            Vec3::new(-0.8, 0.1, 0.5),
            Vec3::new(0.2, -0.7, 0.6),
        ] {
            let aim = aim * (1.0 / aim.norm());
            let frame =
                BoxFrame::aimed(geom_core::UnitVec3::new(aim, "sphere_rect_rows", band).unwrap());
            let (lo, _) = sphere_reach(&body, face, band, &frame);
            let (_, hi) = crate::census::face_reach_in(&body, face, band, &frame).unwrap();
            close(&format!("the reach along {aim:?}"), (lo.x, hi.x), aim);
        }
    }

    /// The unit ball cut by the rim `z = h0` into a north cap and a
    /// south remainder, the north cap carrying a strut meridian at
    /// azimuth 0 from the rim up to `z = hc`, each face on `senses`'
    /// side. `(body, north, south)`.
    fn strut_cap_ball(h0: f64, hc: f64, senses: (bool, bool)) -> (Body<f64>, FaceKey, FaceKey) {
        let world = (Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z());
        strut_cap(
            Point3::origin(),
            1.0,
            world,
            (h0, hc),
            senses,
            false,
            Tol::witness(),
        )
    }

    /// [`strut_cap_ball`] on the sphere of radius `r` about `c`, in the
    /// right-handed orthonormal `frame` `(x̂, ŷ, ẑ)`: the polar axis is
    /// `ẑ`, the seam `x̂`, and the heights `(h0, hc)` are latitude
    /// sines. The seed face carries the strut; which side of the rim it
    /// is, is `senses`' and the boundary's to say.
    ///
    /// `zone` closes the strut's tip with a second rim at `hc`: the
    /// seed face is then the zone `[h0, hc]` on its seam, sensed from its
    /// base rim's stored traversal, and a third face caps the tip's rim.
    fn strut_cap(
        c: Point3<f64>,
        r: f64,
        (x, y, z): (Vec3<f64>, Vec3<f64>, Vec3<f64>),
        (h0, hc): (f64, f64),
        senses: (bool, bool),
        zone: bool,
        tol: Tol,
    ) -> (Body<f64>, FaceKey, FaceKey) {
        let at = |h: f64| c + x * (r * (1.0 - h * h).sqrt()) + z * (r * h);
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(at(h0), true).unwrap();
        let sphere = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: Surface::Sphere {
                        center: c,
                        radius: r,
                        axis: z,
                        u_ref: x,
                    },
                    sense: true,
                },
            )
            .unwrap();
        // A rim's plane is normal to the axis and takes the seam as its
        // reference; the strut's contains the axis and takes it.
        let arc =
            |body: &mut Body<f64>, carrier: Curve3<f64>, u_ref: Vec3<f64>, (t0, t1): (f64, f64)| {
                let Curve3::Circle(geom::Circle {
                    frame: geom::Frame { center, axis, .. },
                    ..
                }) = carrier
                else {
                    unreachable!("a cap edge is a circle");
                };
                let plane = body.add_surface(Surface::Plane {
                    origin: center,
                    normal: axis,
                    u_ref,
                });
                let witness = carrier.mid_point(t0, t1);
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::Intersection {
                        pair: geom_brep::SurfacePair::new(sphere, plane),
                        selector: geom_brep::CurveSelector::Witness(witness),
                    },
                    carrier,
                    param_start: t0,
                    param_end: t1,
                }
            };
        // The strut runs from the rim to `hc`: about `−ŷ` its parameter
        // is the latitude, about `ŷ` the negated one, so either way the
        // span ascends from the rim's end.
        let (s0, s1) = (h0.asin(), hc.asin());
        let (axis, span) = if hc >= h0 {
            (-y, (s0, s1))
        } else {
            (y, (-s0, -s1))
        };
        let meridian = Curve3::circle(c, axis, r, x);
        let strut = arc(&mut body, meridian, z, span);
        let e_s = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                at(hc),
                strut,
                tol,
            )
            .unwrap();
        let rim = Curve3::circle(c + z * (r * h0), z, r * (1.0 - h0 * h0).sqrt(), x);
        let rim = arc(&mut body, rim, x, (0.0, core::f64::consts::TAU));
        let made = body
            .mef(
                MefSite::Chords {
                    he1: e_s.he_plus,
                    he2: e_s.he_plus,
                },
                rim,
                FaceSurface::Shared {
                    key: sphere,
                    sense: true,
                },
                tol,
            )
            .unwrap();
        let (north, south) = (seed.face, made.face);
        body.set_face_sense(north, senses.0).unwrap();
        body.set_face_sense(south, senses.1).unwrap();
        if zone {
            // Against the base rim's direction, so the seed's loop runs
            // round the zone once.
            let tip = Curve3::circle(c + z * (r * hc), -z, r * (1.0 - hc * hc).sqrt(), x);
            let tip = arc(&mut body, tip, x, (0.0, core::f64::consts::TAU));
            let cap = |sense| FaceSurface::Shared { key: sphere, sense };
            let site = MefSite::Chords {
                he1: e_s.he_minus,
                he2: e_s.he_minus,
            };
            match body.mef(site, tip.clone(), cap(true), tol) {
                Ok(_) => {}
                Err(crate::EulerOpError::SenseContradictsChart { derived, .. }) => {
                    body.mef(site, tip, cap(derived), tol).unwrap();
                }
                Err(e) => panic!("the tip's rim: {e:?}"),
            }
            // The zone lies on the side of the base rim toward the tip,
            // which is left of the rim's traversal under the chart normal
            // exactly when the rim runs about `+ẑ` and the tip is above
            // it, or about `−ẑ` and below: set from the stored traversal,
            // not from the side reader these rows test.
            let (outer, _) = crate::props::loop_edges(&body, body.faces[north].outer).unwrap();
            let base = outer
                .iter()
                .find(|e| matches!(e.carrier, Curve3::Circle(geom::Circle { frame: geom::Frame { axis, .. }, .. }) if axis.dot(z) > 0.5))
                .expect("the base rim runs about +ẑ");
            body.set_face_sense(north, base.forward == (hc > h0))
                .unwrap();
        }
        crate::pcurves::mint_pcurves(&mut body, tol).unwrap();
        (body, north, south)
    }

    /// **A strut never stands in for a rim.** A cap that wraps the
    /// azimuth alone is bounded by its one rim, `z = −0.5`; a strut
    /// meridian it carries up to `z = 0.3` is a point set of the face,
    /// not a bound on it, and the face reaches the pole `z = 1`. Read
    /// as a level, the strut's tip would end the latitude window at
    /// `z = 0.3`. Every lane's box reaches the pole, under every sense.
    /// A strut's tip is a cusp of the face's loop, which the sphere's
    /// side reader refuses (`props_sphere_loop_cusp`), so no lane reads
    /// a latitude window off this face at all: every box is the ball's.
    #[test]
    fn a_caps_strut_does_not_bound_its_latitude_window() {
        let band = Band::linear(Tol::witness()).unwrap();
        for hc in [0.3, 1.0] {
            for senses in [(true, true), (true, false), (false, true), (false, false)] {
                let (body, north, _) = strut_cap_ball(-0.5, hc, senses);
                let (lo, hi) = sphere_reach(&body, north, band, &BoxFrame::World);
                let boxed = face_box(&body, north, 0.0, band).unwrap();
                let (clo, chi) = crate::census::face_reach(&body, north, band).unwrap();
                // Every extreme is the ball's: a window read off either
                // side of the rim `z = −0.5` stops short of one pole,
                // and a seam's azimuth window short of `x = −1` or of
                // `y = ±1`.
                for (lane, lo, hi) in [
                    ("sphere_reach", [lo.x, lo.y, lo.z], [hi.x, hi.y, hi.z]),
                    (
                        "face_box",
                        [boxed.min_x, boxed.min_y, boxed.min_z],
                        [boxed.max_x, boxed.max_y, boxed.max_z],
                    ),
                    (
                        "census::face_reach",
                        [clo.x, clo.y, clo.z],
                        [chi.x, chi.y, chi.z],
                    ),
                ] {
                    assert!(
                        lo.iter().all(|&l| l <= -1.0) && hi.iter().all(|&h| h >= 1.0),
                        "strut tip z = {hc}, senses {senses:?}: {lane}'s box {lo:?}..{hi:?} \
                         is not the ball's"
                    );
                }
            }
        }
    }

    /// The right-handed orthonormal frame whose `ẑ` is `axis`: `x̂` is
    /// the world row least along `axis`, made square to it.
    fn frame_about(axis: Vec3<f64>) -> (Vec3<f64>, Vec3<f64>, Vec3<f64>) {
        let z = axis * (1.0 / axis.norm());
        let rows = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
        let least = rows
            .into_iter()
            .min_by(|a, b| a.dot(z).abs().total_cmp(&b.dot(z).abs()))
            .unwrap();
        let x = least - z * least.dot(z);
        let x = x * (1.0 / x.norm());
        (x, z.cross(x), z)
    }

    /// **The exact support of a latitude zone** `[s_lo, s_hi]` (latitude
    /// sines) of the sphere `(c, r)` with polar axis `z`, along world
    /// row `k`: `c_k ± r·max cos(v − β)` over the zone, `β` the row's
    /// latitude (`±β` for the two ends). Its cosine is the norm of the
    /// axis's other two components, which carries it without
    /// cancellation, as the reach under test must.
    fn zone_support(
        c: Point3<f64>,
        r: f64,
        z: Vec3<f64>,
        (s_lo, s_hi): (f64, f64),
        k: usize,
    ) -> (f64, f64) {
        let z = z * (1.0 / z.norm());
        let comps = [z.x, z.y, z.z];
        let sin_b = comps[k];
        let cos_b = super::pick_other(z, k);
        let best = |sin_b: f64| {
            let (lo, hi) = (s_lo.asin(), s_hi.asin());
            let b = sin_b.atan2(cos_b);
            if (lo..=hi).contains(&b) {
                1.0
            } else {
                let s = if b < lo { s_lo } else { s_hi };
                (1.0 - s * s).sqrt() * cos_b + s * sin_b
            }
        };
        let ck = [c.x, c.y, c.z][k];
        (ck - r * best(-sin_b), ck + r * best(sin_b))
    }

    /// **A zone whose axis nears a box row keeps its radial share**
    /// (the verifier's repro). The zone below the rim `h0 = −0.3` of a
    /// sphere of radius `1e3`, down to `−0.9` on its seam, wraps the
    /// azimuth alone, so the reach reads its latitude zone. Its axis is
    /// `t` off `ẑ`, and its top along `z` is on the rim,
    /// `c_z + r·(sin v₀·cos t + cos v₀·sin t)`. The share `√(1 − a_z²)`
    /// of the axis's normal plane cancelled to exactly zero below
    /// `t ≈ 1e-8`, so the box dropped `r·t·cos v₀`: at `ε = 1e-9` it
    /// sat ~1e-5 m inside the face, about 800 times the pad its readers
    /// take. The box, padded as its readers pad it, must reach the rim
    /// at every tilt and every ε.
    #[test]
    fn a_zone_axis_near_a_box_row_keeps_its_radial_share() {
        // Pinned to limb 2's kernel limit at ε = 1e-12: a part near 1 km rounds at its own
        // extent past the band (`work/shell/eps-min-leaves-no-certification-headroom-at-the-box-edge.md`).
        test_utils::kernel_limit::pinned_row(
            geom_core::Tol::witness().eps(),
            &[1e-12],
            "a_zone_axis_near_a_box_row_keeps_its_radial_share",
            || {
                let tol = Tol::witness();
                let band = Band::linear(tol).unwrap();
                let (r, h0) = (1e3, -0.3);
                let c = Point3::new(120.0, -40.0, 75.0);
                for t in [1e-9f64, 1e-8, 1e-7, 1e-4, 1e-3] {
                    let axis = Vec3::new(t.sin(), 0.0, t.cos());
                    let (body, cap, _) = strut_cap(
                        c,
                        r,
                        frame_about(axis),
                        (h0, -0.9),
                        (false, true),
                        true,
                        tol,
                    );
                    let boxed = face_box(&body, cap, super::sweep_pad(band), band).unwrap();
                    let top = c.z + r * (h0 * t.cos() + (1.0 - h0 * h0).sqrt() * t.sin());
                    assert!(
                        boxed.max_z >= top,
                        "tilt {t:e}: the padded box's top {} is {:e} m inside the cap's rim {top}",
                        boxed.max_z,
                        top - boxed.max_z
                    );
                    // The window was read: the box is the zone, not the ball.
                    assert!(
                        boxed.max_z < c.z,
                        "tilt {t:e}: the cap's box {} reads the ball, not its zone",
                        boxed.max_z
                    );
                }
            },
        );
    }

    /// **The reach's outward charge covers its rounding, measured.**
    /// Random zones on a seam (the zone arm), with axes within
    /// `10⁻¹⁰` to `10⁻¹` of a random box row, at scales `1e-3`, `1` and
    /// `1e3` (those the run's ε resolves), read at pad 0 by all three lanes along every row against
    /// the exact support ([`zone_support`]). Each lane must enclose the
    /// face, and `sphere_reach`'s uncharged support must fall short of
    /// the exact one by no more than the 4 ulps of `|c_k| + r` measured,
    /// a quarter of the [`super::SPHERE_REACH_ULPS`] it is charged.
    #[test]
    fn a_zone_reach_charge_covers_random_poses_near_a_box_row() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut rng =
            test_utils::fuzz::start("a_zone_reach_charge_covers_random_poses_near_a_box_row");
        let rows = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
        let (mut faces, mut worst) = (0usize, 0.0f64);
        let wanted = test_utils::fuzz::scaled(300).max(250);
        while faces < wanted {
            let s = [1e-3, 1.0, 1e3][rng.below(3)];
            let r = s * rng.range(0.5, 2.0);
            // A body the run's ε cannot resolve is not built: one under
            // `10⁴ ε`, or one whose coordinates' rounding (64 ulps of the
            // farthest point) reaches ε, which the pcurve envelopes then
            // refuse to certify.
            if r < 1e4 * tol.eps() || (4.0 * s + r) * 64.0 * f64::EPSILON > tol.eps() {
                continue;
            }
            let c = Point3::new(
                s * rng.range(-2.0, 2.0),
                s * rng.range(-2.0, 2.0),
                s * rng.range(-2.0, 2.0),
            );
            let row = rows[rng.below(3)];
            let (side, _, _) = frame_about(row);
            let side = {
                let phi = rng.range(0.0, core::f64::consts::TAU);
                side * phi.cos() + row.cross(side) * phi.sin()
            };
            let t = 10f64.powf(rng.range(-10.0, -1.0));
            let sign = if rng.below(2) == 0 { 1.0 } else { -1.0 };
            let axis = row * (sign * t.cos()) + side * t.sin();
            let h0 = rng.range(-0.9, 0.9);
            let pole = if rng.below(2) == 0 { -1.0 } else { 1.0 };
            let hc = h0 + (pole - h0) * rng.range(0.2, 0.9);
            let window = if hc < h0 { (hc, h0) } else { (h0, hc) };
            let frame = frame_about(axis);
            let (body, cap, _) = strut_cap(c, r, frame, (h0, hc), (true, true), true, tol);
            let (lo, hi) = sphere_reach(&body, cap, band, &BoxFrame::World);
            let boxed = face_box(&body, cap, 0.0, band).unwrap();
            let (clo, chi) = crate::census::face_reach(&body, cap, band).unwrap();
            assert!(
                hi.x - lo.x < 2.0 * r || hi.y - lo.y < 2.0 * r || hi.z - lo.z < 2.0 * r,
                "a zone on its seam reads its zone, not the ball"
            );
            for k in 0..3 {
                let (slo, shi) = zone_support(c, r, frame.2, window, k);
                let pick = |p: Point3<f64>| [p.x, p.y, p.z][k];
                let b = [
                    (boxed.min_x, boxed.max_x),
                    (boxed.min_y, boxed.max_y),
                    (boxed.min_z, boxed.max_z),
                ][k];
                for (lane, (l, h)) in [
                    ("sphere_reach", (pick(lo), pick(hi))),
                    ("face_box", b),
                    ("census::face_reach", (pick(clo), pick(chi))),
                ] {
                    assert!(
                        l <= slo && h >= shi,
                        "{lane}, row {k}, axis {axis:?}, c {c:?}, r {r}, window {window:?}: \
                         [{l}, {h}] does not enclose the face's [{slo}, {shi}]"
                    );
                }
                let ulp = (pick(c).abs() + r) * f64::EPSILON;
                let slack = ulp * super::SPHERE_REACH_ULPS;
                let short = ((pick(lo) + slack) - slo).max(shi - (pick(hi) - slack)) / ulp;
                worst = worst.max(short);
            }
            faces += 1;
        }
        eprintln!(
            "{faces} faces: the uncharged reach falls short by at most {worst:.2} ulps of |c| + r"
        );
        // Measured under 2.5 at every ε (12 000 faces at effort 20); a
        // step that starts to cancel again shows here long before it
        // eats the charge.
        const MEASURED_WORST_ULPS: f64 = 4.0;
        assert!(
            worst <= MEASURED_WORST_ULPS && MEASURED_WORST_ULPS <= super::SPHERE_REACH_ULPS,
            "the uncharged reach falls short by {worst} ulps, past the {MEASURED_WORST_ULPS} \
             measured (the charge is {})",
            super::SPHERE_REACH_ULPS
        );
    }

    #[test]
    fn a_torn_record_under_the_sphere_rect_panics() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let frame = BoxFrame::World;
        let (body, face) = sphere_zone_sheet((0.2, 1.4), (-0.3, 0.4), tol);
        let reach = |b: &Body<f64>| sphere_reach(b, face, band, &frame);
        let (lo, hi) = reach(&body);
        assert!(
            (lo.z + 0.3).abs() < 1e-9 && (hi.z - 0.4).abs() < 1e-9,
            "the sound sheet's reach along its axis is its latitude window, not the ball: \
             [{}, {}]",
            lo.z,
            hi.z
        );
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the zone's outer loop is a cycle");
        };

        // The flattening's read: a member's curve.
        let mut torn = body.clone();
        let edge = torn.get_half_edge(first).unwrap().edge;
        let curve = torn.get_edge(edge).unwrap().curve;
        torn.curves.remove(curve);
        let named = format!(
            "{}'s curve names {}",
            EntityId::Edge(edge),
            GeomRef::Curve(curve)
        );
        assert_torn_op_panics(
            "sphere_reach (flattening)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| reach(b),
        );

        // The trim's read alone: a boundary vertex's point.
        let mut torn = body.clone();
        let v = torn.get_half_edge(first).unwrap().start;
        let point = torn.get_vertex(v).unwrap().point;
        torn.points.remove(point);
        let named = format!(
            "{}'s point names {}",
            EntityId::Vertex(v),
            GeomRef::Point(point)
        );
        assert_torn_op_panics(
            "sphere_reach (trim)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| reach(b),
        );
    }
}
