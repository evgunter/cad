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
//! PRUNES. That is **one** of the five doors that read a box from
//! here; at the other four, box NON-overlap is the answer being
//! sought, so a bigger box is a REFUSAL:
//!
//! - `boolean::reduce`'s C10 tree PRUNES. Loose costs a candidate
//!   pair's worth of exact work and can never change a verdict.
//! - `boolean::reduce`'s operand GATE grants on non-overlap: an
//!   unsupported-kind face whose box clears the other operand cannot
//!   enter a pair, so the operation runs. A bigger box refuses an
//!   operation whose faces never meet.
//! - `separation` GRANTS on non-overlap — `Ok(())` IS the
//!   disjointness certificate — so a bigger box refuses a placement
//!   pair that is genuinely separated.
//! - `boolean::ops`'s sphere-extent fallback refuses typed unless the
//!   ball's certified extent CLEARS the face's box, so a bigger box
//!   turns a separated cyl×sphere pair into
//!   `FallbackExtentUnsupported`.
//! - `census`'s arm 2 clears an instance pair only on a definitely
//!   negative margin against a CONTAINING box, so a bigger box turns
//!   a genuinely-outside instance into `CensusUndecidable` — the
//!   interference class.
//!
//! So nothing here may say "loose is free" about a BOX. It is a claim
//! about a door, and the door has to be named. The five are not
//! recited: `every_door_that_reads_a_box_is_inventoried` below walks
//! `topo/src` and pins them per file — both rules, face and edge — so
//! a fifth door cannot land unargued. **It pins WHERE the doors are
//! and not which way each reads**, which is the column that carries
//! the argument above; that gap is `S234` and has an owner rather
//! than a disclosure.
//!
//! [`sweep_pad`] is sized so the padding can never lose an accepted
//! pair at the sweep's door (its derivation below).
//!
//! [`FaceBoxRule`] is the ONE statement of which surface kinds have a
//! cheap sound box and by what construction; [`face_box`] is its
//! `f64`-bracket instantiation and `census`'s `reach_box` is its
//! instantiation at the census's own scalar. **Neither re-derives an
//! extent**: the per-kind arithmetic lives once, in [`slab_extent`],
//! [`ball_extent`], [`torus_extent`] and [`conic_extent`], written
//! against [`Span`] so a lane on the [`Bounds`] allowlist and a lane
//! off it can both enter it — the first with `[lo(), hi()]`
//! brackets at `f64`, the second with degenerate spans at its own
//! `T`. What the census still owns is its arena WALK and its answer
//! for a description with no claim in it (`None`, versus the poison
//! box here); neither is arithmetic, and the census comment states
//! both.
//!
//! An allowlisted [`geom_core::Bounds`] seam (ratified 2026-07-29 —
//! see geom-core `real.rs`, Bounds scope rule; the C10 tree is the
//! subdivision driver): coordinates enter as `[lo(), hi()]` brackets,
//! and poison flows to the poison box, which each door reads in its
//! own fail-loud direction — never prunes at the sweep, refuses at
//! the other three.

use bvh::Aabb;
use geom::Surface;
use geom::surfaces::nurbs::NurbsSurface;
use geom_core::{Band, Bounds, Decide, Point3, Real, Vec3};

use super::BooleanError;
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, LoopKey, VertexKey};

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

fn corrupt(what: &'static str) -> BooleanError {
    BooleanError::ClassificationInvariant { what }
}

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
/// is NOT directed: the `f64` lane's last step is
/// [`Aabb::padded`], whose outward ulp plus [`sweep_pad`] dominates
/// the arithmetic's own error, and the census lane's scalar carries
/// its own enclosure.
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

    fn sub(self, o: Self) -> Self {
        Self {
            lo: self.lo - o.hi,
            hi: self.hi - o.lo,
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

    /// A LOWER bound on `|x|` over the span — zero as soon as the
    /// span straddles zero, branch-free (`max(0, max(lo, −hi))`), so
    /// no scalar is asked to decide a sign it may not know.
    fn abs_min(self) -> T {
        T::zero().max(self.lo.max(-self.hi))
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

/// The scalar's POISON, built through the totality policy rather than
/// spelled per type: `Real::sqrt` of a negative is poison on every
/// implementor (`geom_core::real` module docs — NaN at `f64`, the
/// empty enclosure at the interval scalar, a poisoned value channel
/// at the dual).
fn poison_value<T: Real>() -> T {
    (T::zero() - T::one()).sqrt()
}

/// ONE boundary edge's axial range about `(origin, axis)` — **the
/// edge's own locus projected on the axis, not the corners of a box
/// around it.**
///
/// Projecting a boundary's AABB corners instead is exact when the
/// axis is a coordinate direction: the projection then reads one coordinate and
/// the box's spread in the other two contributes nothing. **At a
/// TILTED axis it is not**, and the error is the box's spread times
/// the axis's other components — for a rim circle of radius `r`, up
/// to `r` of axial window that the rim, which is an axial iso-line,
/// does not occupy at all. That inflation feeds straight back into
/// the frustum's radius and re-widens the very box
/// [`cone_frustum_extent`] exists to tighten.
///
/// Per rule:
///
/// - **Chord** — the hull of the two endpoints' projections. Exact:
///   the projection is linear, so a segment's image is the segment
///   between the images.
/// - **ConicAmplitude** — the conic's axial image is
///   `(centre − origin)·axis ± √((a·(û·axis))² + (b·(v̂·axis))²)`, the
///   same full-turn amplitude [`conic_extent`] takes per coordinate,
///   taken along the axis instead. A rim PERPENDICULAR to the axis
///   has `û·axis = v̂·axis = 0` and collapses to a point, which is
///   what it geometrically is. Hulled with the chord so a poisoned
///   amplitude cannot narrow the answer.
/// - **NoSoundBox** — nothing is claimed: the poison span.
pub(crate) enum AxialCarrier<T> {
    /// Nothing is certified about the locus — claim nothing.
    Unclaimable,
    /// The locus IS the chord between the two ends.
    Chord,
    /// A full conic: its centre, the two in-plane reference
    /// directions and the matching semi-axes, in the reading lane's
    /// own spans.
    Conic {
        /// The conic's centre.
        center: SpanBox<T>,
        /// The `t = 0` reference direction.
        u_ref: SpanBox<T>,
        /// `axis × u_ref`.
        v_ref: SpanBox<T>,
        /// The semi-axis along `u_ref`.
        semi_u: T,
        /// The semi-axis along `v_ref`.
        semi_v: T,
    },
}

pub(crate) fn edge_axial_span<T: Real>(
    origin: &SpanBox<T>,
    axis: &SpanBox<T>,
    carrier: &AxialCarrier<T>,
    ends: (&SpanBox<T>, &SpanBox<T>),
) -> Span<T> {
    let along = |p: &SpanBox<T>, o: &SpanBox<T>| {
        p.x.sub(o.x)
            .mul(axis.x)
            .add(p.y.sub(o.y).mul(axis.y))
            .add(p.z.sub(o.z).mul(axis.z))
    };
    let zero = SpanBox {
        x: Span::exact(T::zero()),
        y: Span::exact(T::zero()),
        z: Span::exact(T::zero()),
    };
    let chord = along(ends.0, origin).hull(along(ends.1, origin));
    match carrier {
        // Claim nothing, in the one form every door already reads.
        AxialCarrier::Unclaimable => chord.widen(poison_value::<T>()),
        AxialCarrier::Chord => chord,
        AxialCarrier::Conic {
            center,
            u_ref,
            v_ref,
            semi_u,
            semi_v,
        } => {
            let du = along(u_ref, &zero).abs_max();
            let dv = along(v_ref, &zero).abs_max();
            let amp = ((du * *semi_u).powi(2) + (dv * *semi_v).powi(2)).sqrt();
            along(center, origin).widen(amp).hull(chord)
        }
    }
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
/// `axis_i²` is bounded BELOW ([`Span::abs_min`]) so the perpendicular
/// factor is bounded above, which is the sound direction; an axis
/// bracket that does not pin the direction reads as more
/// perpendicular room, never less, and a poisoned one poisons the box.
///
/// **The premise is a UNIT axis**, which is what [`Surface`]'s own
/// conic descriptions promise and what `h` is a length in. It is a
/// premise of the CONSTRUCTION, not of the widening: with `|axis| ≠ 1`
/// the axis segment `origin + h·axis` is off by `|axis|²` before any
/// radius is added, so nothing here could rescue such a description
/// and this arm does not pretend to. State the premise if you add a
/// constructor that does not hold it.
pub(crate) fn slab_extent<T: Real>(
    origin: &SpanBox<T>,
    axis: &SpanBox<T>,
    h: Span<T>,
    radius: T,
) -> SpanBox<T> {
    SpanBox {
        x: origin
            .x
            .add(h.mul(axis.x))
            .widen(radius * perp_room(axis.x)),
        y: origin
            .y
            .add(h.mul(axis.y))
            .widen(radius * perp_room(axis.y)),
        z: origin
            .z
            .add(h.mul(axis.z))
            .widen(radius * perp_room(axis.z)),
    }
}

/// The most one coordinate of a UNIT vector perpendicular to a unit
/// `axis` can be: `√(1 − axis_i²)`, with `axis_i²` bounded BELOW so
/// the room is bounded above — the sound direction ([`slab_extent`]).
fn perp_room<T: Real>(a: Span<T>) -> T {
    (T::one() - a.abs_min().powi(2)).max(T::zero()).sqrt()
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
/// no sign decision. Same UNIT-axis premise as [`slab_extent`].
pub(crate) fn cone_frustum_extent<T: Real>(
    apex: &SpanBox<T>,
    axis: &SpanBox<T>,
    h: Span<T>,
    tan_half_angle: T,
) -> SpanBox<T> {
    let h0 = h.lo.max(T::zero()).min(h.hi);
    let coord = |o: Span<T>, a: Span<T>| {
        let k = tan_half_angle * perp_room(a);
        let at = |t: T| Span::exact(t).mul(a).widen(t.abs() * k);
        o.add(at(h.lo).hull(at(h.hi)).hull(at(h0)))
    };
    SpanBox {
        x: coord(apex.x, axis.x),
        y: coord(apex.y, axis.y),
        z: coord(apex.z, axis.z),
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
/// that is exactly the true box. Same UNIT-axis premise as
/// [`slab_extent`], and here it is the widening's own: `|axis| > 1`
/// would claim less perpendicular room than a unit `û` can take.
pub(crate) fn torus_extent<T: Real>(
    center: &SpanBox<T>,
    axis: &SpanBox<T>,
    major: T,
    minor: T,
) -> SpanBox<T> {
    let reach = |a: Span<T>| (major + minor) * perp_room(a) + minor * a.abs_max();
    SpanBox {
        x: center.x.widen(reach(axis.x)),
        y: center.y.widen(reach(axis.y)),
        z: center.z.widen(reach(axis.z)),
    }
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
/// - [`BoundaryHull`](Self::BoundaryHull) — **Plane.** A planar face
///   lies in the convex hull of its boundary, so the hull of the
///   boundary's own certified boxes contains it whatever the boundary
///   curves are. The hull of the boundary VERTICES alone does not: a
///   circular rim bulges past its endpoints, and this engine's
///   plane×cylinder lane mints exactly that face.
/// - [`CylinderSlab`](Self::CylinderSlab) — **Cylinder.** The wall's
///   belly bulges past its chords, so the box is the whole cylinder
///   slab over the face's axial range (the axial coordinate is linear
///   along the surface, so the face's axial extremes lie on its
///   boundary), widened by the radius **perpendicular to the axis**
///   ([`slab_extent`] carries the derivation). The axial coordinate
///   takes no widening at all: the boundary bounds it exactly.
/// - [`ConeSlab`](Self::ConeSlab) — **Cone.** The FRUSTUM over the
///   face's axial window, not a slab of constant radius: a cone point
///   is `apex + v·(axis·cos α + û·sin α)`, so at axial coordinate
///   `t = v·cos α` its radial offset is `|t|·tan α` EXACTLY. `t` is
///   linear in the chart, so its extremes over a trimmed face lie on
///   the face's boundary exactly as the cylinder's do
///   ([`edge_axial_span`]), and [`cone_frustum_extent`] boxes the frustum
///   that window cuts. Pinning the radius at the window's widest end
///   instead is what makes a pucker read as if it were the whole
///   cone, and doors that read overlap as "may meet" pay it in
///   refusals.
/// - [`WholeBall`](Self::WholeBall) — **Sphere.** A band's belly
///   bulges past its poles and seam arcs, so the box is the whole ball
///   `center ± r`; every surface point is within `r` of the center.
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
///   `knots_u().domain()`. What holds it up is construction: every
///   kernel-minted NURBS wall is iso-parameter bounded at the domain
///   edges. State the premise when you add a constructor that is not.
/// - [`WholeTorus`](Self::WholeTorus) — **Torus.** The whole tube
///   about the centre, `(R + r)` perpendicular to the axis and `r`
///   along it ([`torus_extent`]) — reading nothing from the boundary,
///   as the ball does.
///
/// **Every surface kind has an arm**, and that is a statement this
/// enum makes at the type level: there is no `NoSoundBox` on the face
/// side, so a kind added to [`Surface`] cannot acquire a box by
/// falling through a wildcard, and none can be silently left without
/// one either. A face whose surface key does not RESOLVE is a
/// different answer — that is arena corruption, and the callers here
/// report it as such rather than folding it in here. A box can still
/// come out POISON (an unboxable boundary edge, a poisoned
/// description); that is the value, not the rule.
///
/// [`WholeBall`](Self::WholeBall), [`WholeTorus`](Self::WholeTorus)
/// and the conic-fed
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
        /// The unit axis direction.
        axis: Vec3<T>,
        /// The cylinder's radius.
        radius: T,
    },
    /// The same slab with the generator's own radius — see the type
    /// docs.
    ConeSlab {
        /// The apex (`v = 0`).
        apex: Point3<T>,
        /// The unit axis direction.
        axis: Vec3<T>,
        /// The half-angle α ∈ (0, π/2).
        half_angle: T,
    },
    /// The whole ball `center ± r` — see the type docs.
    WholeBall {
        /// The sphere's center.
        center: Point3<T>,
        /// The sphere's radius.
        radius: T,
    },
    /// The whole tube about the centre — see the type docs.
    WholeTorus {
        /// The torus centre.
        center: Point3<T>,
        /// The unit axis direction.
        axis: Vec3<T>,
        /// The major radius `R`.
        major_radius: T,
        /// The minor radius `r`.
        minor_radius: T,
    },
    /// The control net's hull — see the type docs.
    ControlNet(&'a NurbsSurface<T>),
}

/// The [`FaceBoxRule`] for a surface — the single kind→rule mapping. A
/// kind added to [`Surface`] gets its arm by being written here, never
/// by falling through a wildcard in some consumer. Takes a RESOLVED
/// surface: a missing one is corruption, which is the caller's to
/// report and not a rule.
pub(crate) fn face_box_rule<T: Real>(surface: &Surface<T>) -> FaceBoxRule<'_, T> {
    match surface {
        Surface::Plane { .. } => FaceBoxRule::BoundaryHull,
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => FaceBoxRule::CylinderSlab {
            origin: *origin,
            axis: *axis,
            radius: *radius,
        },
        Surface::Sphere { center, radius, .. } => FaceBoxRule::WholeBall {
            center: *center,
            radius: *radius,
        },
        Surface::Nurbs(patch) => FaceBoxRule::ControlNet(patch),
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
            ..
        } => FaceBoxRule::WholeTorus {
            center: *center,
            axis: *axis,
            major_radius: *major_radius,
            minor_radius: *minor_radius,
        },
    }
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
/// [`BooleanError::ClassificationInvariant`] when the face's topology
/// is corrupt (a lost entity, an unwalkable loop). A face whose
/// surface key does not resolve is corruption, NOT a kind without a
/// box — the two are separate answers here.
pub(crate) fn face_box<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    pad: f64,
) -> Result<Aabb, BooleanError> {
    let f = body.get_face(face).ok_or(corrupt("face box: face lost"))?;
    let surface = body
        .get_surface(f.surface)
        .ok_or(corrupt("face box: surface lost"))?;
    // The axial range over the boundary's own hull. Taking the hull
    // first (rather than each edge box separately) is a superset of
    // every per-edge range because the projection is linear — looser,
    // the conservative direction — and it is what makes poison
    // PROPAGATE: `Aabb::hull` carries NaN, and a NaN projection
    // reaches the poison box rather than being dropped by an
    // `f64::min` that ignores it.
    // The axial window from the boundary's OWN locus (per edge), not
    // from the corners of a box around it — see `edge_axial_span`.
    let axial_window =
        |axis: Vec3<T>, origin: Point3<T>| -> Result<Option<Span<f64>>, BooleanError> {
            let (origin, axis) = (bracket_point(origin), bracket_vector(axis));
            let mut acc: Option<Span<f64>> = None;
            let mut grow = |s: Span<f64>| acc = Some(acc.map_or(s, |a: Span<f64>| a.hull(s)));
            for lk in loops_of(f) {
                let l = body.get_loop(lk).ok_or(corrupt("face box: loop lost"))?;
                match l.boundary {
                    LoopBoundary::Empty { vertex } => {
                        let p = bracket_point(vertex_point(body, vertex)?);
                        grow(edge_axial_span(
                            &origin,
                            &axis,
                            &AxialCarrier::Chord,
                            (&p, &p),
                        ));
                    }
                    LoopBoundary::Cycle { first } => {
                        for he in body
                            .loop_cycle(first)
                            .ok_or(corrupt("face box: unwalkable loop"))?
                        {
                            let ek = body
                                .get_half_edge(he)
                                .ok_or(corrupt("face box: half-edge lost"))?
                                .edge;
                            let e = body.get_edge(ek).ok_or(corrupt("face box: edge lost"))?;
                            let end = |h| -> Result<SpanBox<f64>, BooleanError> {
                                let vk = body
                                    .get_half_edge(h)
                                    .ok_or(corrupt("face box: half-edge lost"))?
                                    .start;
                                Ok(bracket_point(vertex_point(body, vk)?))
                            };
                            let carrier = body
                                .get_curve_geom(e.curve)
                                .and_then(crate::null::CurveGeom::certified)
                                .map(geom_brep::EdgeCurve::carrier);
                            let axial = match edge_box_rule(carrier) {
                                EdgeBoxRule::NoSoundBox => AxialCarrier::Unclaimable,
                                EdgeBoxRule::Chord => AxialCarrier::Chord,
                                EdgeBoxRule::ConicAmplitude {
                                    center,
                                    axis: c_axis,
                                    semi_u,
                                    semi_v,
                                    u_ref,
                                } => AxialCarrier::Conic {
                                    center: bracket_point(center),
                                    u_ref: bracket_vector(u_ref),
                                    v_ref: bracket_vector(c_axis.cross(u_ref)),
                                    semi_u: semi_u.hi(),
                                    semi_v: semi_v.hi(),
                                },
                            };
                            grow(edge_axial_span(
                                &origin,
                                &axis,
                                &axial,
                                (&end(e.he_plus)?, &end(e.he_minus)?),
                            ));
                        }
                    }
                }
            }
            Ok(acc)
        };
    let boxed = match face_box_rule(surface) {
        FaceBoxRule::ControlNet(patch) => geom::surfaces::boxes::nurbs_surface_aabb(patch),
        FaceBoxRule::WholeBall { center, radius } => {
            aabb_of(ball_extent(&bracket_point(center), radius.hi()))
        }
        FaceBoxRule::WholeTorus {
            center,
            axis,
            major_radius,
            minor_radius,
        } => aabb_of(torus_extent(
            &bracket_point(center),
            &bracket_vector(axis),
            major_radius.hi(),
            minor_radius.hi(),
        )),
        FaceBoxRule::CylinderSlab {
            origin,
            axis,
            radius,
        } => {
            let Some(h) = axial_window(axis, origin)? else {
                return Ok(Aabb::poison());
            };
            aabb_of(slab_extent(
                &bracket_point(origin),
                &bracket_vector(axis),
                h,
                radius.hi(),
            ))
        }
        FaceBoxRule::ConeSlab {
            apex,
            axis,
            half_angle,
        } => {
            let Some(h) = axial_window(axis, apex)? else {
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
        FaceBoxRule::BoundaryHull => boundary_hull(body, f)?.unwrap_or_else(Aabb::poison),
    };
    Ok(boxed.padded(pad))
}

/// A [`SpanBox`] of `f64` spans as the [`Aabb`] every door reads.
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

/// A face's loop keys, outer first — the walk order every arm here
/// shares (D9: fixed, so two boxes of one face fold identically).
fn loops_of(f: &crate::entity::Face) -> impl Iterator<Item = LoopKey> + '_ {
    core::iter::once(f.outer).chain(f.rings.iter().copied())
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
    f: &crate::entity::Face,
) -> Result<Option<Aabb>, BooleanError> {
    let mut acc: Option<Aabb> = None;
    let mut grow = |x: Aabb| acc = Some(acc.map_or(x, |a: Aabb| a.hull(&x)));
    for lk in loops_of(f) {
        let l = body.get_loop(lk).ok_or(corrupt("face box: loop lost"))?;
        match l.boundary {
            LoopBoundary::Empty { vertex } => {
                let p = vertex_point(body, vertex)?;
                grow(Aabb::from_points([p]).unwrap_or_else(Aabb::poison));
            }
            LoopBoundary::Cycle { first } => {
                for he in body
                    .loop_cycle(first)
                    .ok_or(corrupt("face box: unwalkable loop"))?
                {
                    let ek = body
                        .get_half_edge(he)
                        .ok_or(corrupt("face box: half-edge lost"))?
                        .edge;
                    grow(edge_box(body, ek, 0.0)?);
                }
            }
        }
    }
    Ok(acc)
}

/// **The one soundness rule for an edge's box** — [`FaceBoxRule`]'s
/// curve-side twin, and read by the same consumers for the same
/// reason: which cheap construction yields a genuine SUPERSET of the
/// edge's locus over its span.
///
/// - [`Chord`](Self::Chord) — **Line.** The locus IS the chord between
///   the endpoints, up to the certification residual the pad covers.
/// - [`ConicAmplitude`](Self::ConicAmplitude) — **Circle, Ellipse.**
///   The FULL conic's centre-±-amplitude box (per coordinate
///   `√((û_i·a)² + (v̂_i·b)²)`, with `v̂ = axis × û`) hulled with the
///   chord. A superset of any arc of the conic, reflex spans included
///   — an arc's belly bulges past its chord, so the chord alone is
///   not a bound. The full turn is deliberately loose: a superset is
///   what the contract asks for and the arc's own extremes are not
///   cheap. What that looseness costs is the door's, not the box's —
///   the module docs list the four and their directions.
///
///   **The half-extent is a function of the LOCUS**, which is a
///   sharper requirement than being sound: `a·û_i·cos t + b·v̂_i·sin t`
///   tops out at exactly `√((û_i·a)² + (v̂_i·b)²)` over a full turn,
///   so two `Curve3::Circle` values describing the SAME circle — same
///   centre, axis and radius, `u_ref` rotated within the plane — get
///   the same box. The triangle-inequality bound `|û_i|·a + |v̂_i|·b`
///   does not: it returns `r` at an axis-aligned `u_ref` and `r√2` at
///   45°, and three of the four doors pay that difference in
///   refusals rather than in work. In-tree bodies take the rotated
///   branch routinely — the plane×cylinder rim inherits the cylinder
///   surface's own `u_ref`, the plane×sphere circle derives one from
///   the seam or polar candidate, and an extruded arc profile mints
///   rotated ones directly — so this is exercised, not latent.
///   [`conic_extent`] is the one derivation, and the
///   `the_planar_arms_box_is_exactly_…` row sweeps `u_ref` in the
///   plane and pins the box's invariance under it.
///
///   **A tighter box still exists one crate down and is unused.**
///   `geom::curves::boxes::circle_arc_aabb` (and its ellipse twin)
///   computes the same amplitude outward-bracketed AND restricts to
///   the certified span, so it is tighter on the span; it takes the
///   two params [`geom_brep::EdgeCurve::params`] already has here,
///   and today its only callers are `geom`'s own tests. Taking it is
///   a TIGHTENING that would start pruning pairs examined today —
///   **`S235`**, and a deliberate looseness rather than a defect
///   while it waits.
/// - [`NoSoundBox`](Self::NoSoundBox) — **NURBS carriers**, and an
///   edge whose carrier is null scaffolding. Nothing is certified
///   about the locus, so nothing is claimed; the chord is NOT a bound
///   for either.
///
///   The NURBS arm is the one place a sound cheap box exists and is
///   deliberately not taken: `geom::curves::boxes::nurbs_curve_aabb`
///   would give the control-net hull, exactly as
///   [`FaceBoxRule::ControlNet`] does one dimension up. Taking it
///   would TIGHTEN this box — it would start pruning pairs that are
///   examined today — and tightening is a different obligation from
///   soundness: a rung-3 operand gate has to admit the kind first.
///   Claiming nothing is already the conservative answer, so nothing
///   is unsound while it waits. (It also carries the same trim ⊆ knot
///   domain premise the surface arm states.)
pub(crate) enum EdgeBoxRule<T: Real> {
    /// The chord between the endpoints — see the type docs.
    Chord,
    /// The full conic's amplitude box, hulled with the chord — see the
    /// type docs.
    ConicAmplitude {
        /// The conic's centre.
        center: Point3<T>,
        /// The plane normal of the conic.
        axis: Vec3<T>,
        /// The semi-axis along `u_ref`.
        semi_u: T,
        /// The semi-axis along `axis × u_ref`.
        semi_v: T,
        /// The in-plane reference direction.
        u_ref: Vec3<T>,
    },
    /// No cheap superset exists — see the type docs.
    NoSoundBox,
}

/// The [`EdgeBoxRule`] for a carrier — the single kind→rule mapping,
/// with `None` standing for the null-scaffolding state (no carrier by
/// type). A kind added to [`geom::Curve3`] lands on
/// [`EdgeBoxRule::NoSoundBox`] only by being written here.
pub(crate) fn edge_box_rule<T: Real>(carrier: Option<&geom::Curve3<T>>) -> EdgeBoxRule<T> {
    match carrier {
        Some(geom::Curve3::Line { .. }) => EdgeBoxRule::Chord,
        Some(geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        }) => EdgeBoxRule::ConicAmplitude {
            center: *center,
            axis: *axis,
            semi_u: *radius,
            semi_v: *radius,
            u_ref: *u_ref,
        },
        Some(geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        }) => EdgeBoxRule::ConicAmplitude {
            center: *center,
            axis: *axis,
            semi_u: *major,
            semi_v: *minor,
            u_ref: *u_ref,
        },
        Some(geom::Curve3::Nurbs(_)) | None => EdgeBoxRule::NoSoundBox,
    }
}

/// The edge's certified box, padded — [`EdgeBoxRule`]'s `f64`-bracket
/// instantiation, and therefore a superset of the edge's locus or the
/// poison box.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when the edge's topology
/// is corrupt.
pub(crate) fn edge_box<T: Decide + Bounds>(
    body: &Body<T>,
    edge: EdgeKey,
    pad: f64,
) -> Result<Aabb, BooleanError> {
    let e = body.get_edge(edge).ok_or(corrupt("edge box: edge lost"))?;
    let start_of = |he| -> Result<Point3<T>, BooleanError> {
        let vk = body
            .get_half_edge(he)
            .ok_or(corrupt("edge box: half-edge lost"))?
            .start;
        vertex_point(body, vk)
    };
    let (a, b) = (start_of(e.he_plus)?, start_of(e.he_minus)?);
    let chord = Aabb::from_points([a, b]).unwrap_or_else(Aabb::poison);
    let carrier = body
        .get_curve_geom(e.curve)
        .and_then(crate::null::CurveGeom::certified)
        .map(geom_brep::EdgeCurve::carrier);
    let boxed = match edge_box_rule(carrier) {
        EdgeBoxRule::NoSoundBox => return Ok(Aabb::poison()),
        EdgeBoxRule::Chord => chord,
        EdgeBoxRule::ConicAmplitude {
            center,
            axis,
            semi_u,
            semi_v,
            u_ref,
        } => {
            let v_ref = axis.cross(u_ref);
            let full = aabb_of(conic_extent(
                &bracket_point(center),
                &bracket_vector(u_ref),
                &bracket_vector(v_ref),
                semi_u.hi(),
                semi_v.hi(),
            ));
            // `Aabb::hull`, not a raw min/max fold: a poisoned centre
            // or semi-axis must survive the hull, and `f64::min`
            // RETURNS the non-NaN operand.
            full.hull(&chord)
        }
    };
    Ok(boxed.padded(pad))
}

/// Either empty lookup means the same thing here — a corrupt body —
/// so the read-back door's discriminated reference collapses to one
/// verdict.
fn vertex_point<T: Decide + Bounds>(
    body: &Body<T>,
    v: VertexKey,
) -> Result<Point3<T>, BooleanError> {
    crate::readback::vertex_point_ref(body, v)
        .map_err(|_| corrupt("face/edge box: vertex point lost"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
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
    use crate::euler::{FaceSurface, MefSite, MevSite};
    use geom::Curve3;
    use geom::Surface;
    use geom_brep::{EdgeCurveSpec, EdgeGeometry};
    use geom_core::Tol;
    use geom_core::{Point3, Vec3};

    /// The pad every row boxes with — the sweep's own, so a row that
    /// only passes because of a generous pad would have to say so.
    fn pad() -> f64 {
        sweep_pad(Band::linear(Tol::witness()).unwrap())
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
    /// below is one — and [`EdgeBoxRule::ConicAmplitude`]'s
    /// per-coordinate `|û_i|·a + |v̂_i|·b` is not invariant under it.
    fn arc_sector_from(r: f64, span: f64, phi: f64) -> (Body<f64>, FaceKey) {
        let on = |t: f64| Point3::new(r * t.cos(), r * t.sin(), 0.0);
        let (a, b, c) = (on(0.0), on(span), Point3::origin());
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(a).unwrap();
        let plane = body.add_surface(plane_z0());
        let cyl = body.add_surface(cyl_r(r));
        let arc = EdgeCurveSpec {
            description: EdgeGeometry::Intersection {
                s1: plane,
                s2: cyl,
                witness: on(span * 0.5),
            },
            carrier: Curve3::Circle {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: r,
                u_ref: Vec3::new(phi.cos(), phi.sin(), 0.0),
            },
            param_start: -phi,
            param_end: span - phi,
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
                FaceSurface::Shared(plane),
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
                    let b = face_box(&body, face, pad()).unwrap();
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
        let b = face_box(&body, face, pad()).unwrap();
        // Both boundary vertices and the sector's centre are at y ≤ 0;
        // the rim reaches y = r.
        assert!(
            b.max_y >= r,
            "the box must reach the rim's extreme y = {r}, got {}",
            b.max_y
        );
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
    /// ([`FaceSurface::New`]) — the boundary is genuinely on that
    /// surface whenever `rho` is the surface's own radius profile,
    /// which is what makes the re-label honest rather than a fixture
    /// trick.
    ///
    /// The rim carriers are described as the cylinder-of-that-radius
    /// cut by the plane at that height, which is the same circle
    /// whatever surface the face ends up carrying: the description
    /// certifies the CARRIER, and a cone's own rim is that circle.
    fn revolved_wall(
        rho: &dyn Fn(f64) -> f64,
        u0: f64,
        u1: f64,
        z0: f64,
        z1: f64,
    ) -> (Body<f64>, FaceKey) {
        let on = |u: f64, z: f64| Point3::new(rho(z) * u.cos(), rho(z) * u.sin(), z);
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(on(u0, z0)).unwrap();
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
                    Curve3::Circle {
                        center: Point3::new(0.0, 0.0, z),
                        axis: Vec3::unit_z(),
                        radius: r,
                        u_ref: Vec3::unit_x(),
                    },
                    u0,
                    u1,
                )
            } else {
                (
                    Curve3::Circle {
                        center: Point3::new(0.0, 0.0, z),
                        axis: Vec3::new(0.0, 0.0, -1.0),
                        radius: r,
                        u_ref: Vec3::new(u1.cos(), u1.sin(), 0.0),
                    },
                    0.0,
                    u1 - u0,
                )
            };
            (
                EdgeCurveSpec {
                    description: EdgeGeometry::Intersection {
                        s1: wall,
                        s2: plane,
                        witness: on((u0 + u1) * 0.5, z),
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
        let face = body
            .mef(
                MefSite::Chords {
                    he1: he,
                    he2: e_b.he_plus,
                },
                EdgeCurveSpec::line_between(on(u0, z1), on(u0, z0)),
                FaceSurface::Shared(cyl),
                Tol::witness(),
            )
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
                let b = face_box(&body, face, pad()).unwrap();
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
            body.set_face_surface(
                face,
                FaceSurface::New(Surface::Sphere {
                    center,
                    radius: r,
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                }),
            )
            .unwrap();
            let b = face_box(&body, face, pad()).unwrap();
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
        let kv = KnotVector::unit_segment(2);
        let p = |x: f64, y: f64, z: f64| Point3::new(x, y, z);
        let control = vec![
            p(0.0, 0.0, 0.0),
            p(0.0, 0.5, 0.0),
            p(0.0, 1.0, 0.0),
            p(0.5, 0.0, 0.0),
            p(0.5, 0.5, 1.0),
            p(0.5, 1.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(1.0, 0.5, 0.0),
            p(1.0, 1.0, 0.0),
        ];
        let patch = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap();
        let surface = Surface::Nurbs(std::sync::Arc::new(patch));
        let (mut body, face) = arc_sector(1.0, core::f64::consts::PI);
        body.set_face_surface(face, FaceSurface::New(surface))
            .unwrap();
        (body, face, (p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)))
    }

    /// **The NURBS half of the same defect.** A patch's interior
    /// bulges past the hull of its boundary — here the biquadratic
    /// whose boundary lies entirely in `z = 0` while its centre
    /// control point lifts the surface to `z = 1/4`. The control-net
    /// hull contains it; the boundary hull does not.
    #[test]
    fn a_nurbs_patchs_interior_bulge_is_inside_its_box() {
        let (body, face, _) = nurbs_bulge_face();
        let surface = body
            .get_surface(body.get_face(face).unwrap().surface)
            .unwrap()
            .clone();
        // The lifted interior, on the surface itself — its boundary
        // curves all lie in `z = 0`, so no hull of the BOUNDARY can
        // contain this point.
        let mid = surface.eval(0.5, 0.5);
        assert!(mid.z > 0.2, "the fixture must actually bulge, got {mid:?}");
        assert!(
            surface.eval(0.0, 0.5).z.abs() < 1e-15,
            "the fixture's boundary must lie in z = 0"
        );
        let b = face_box(&body, face, pad()).unwrap();
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

    /// **`BoundaryHull`, conic-fed** — the rim is a circle, so its
    /// [`EdgeBoxRule::ConicAmplitude`] box is the FULL turn's
    /// whatever the span (the arc's own extremes are deliberately not
    /// recovered) and the two radii chords lie inside it. Flat in z.
    ///
    /// The circle's half extent is `r` in every in-plane coordinate,
    /// whatever `u_ref` the carrier is NAMED from. **The φ sweep is
    /// what makes this row an invariance claim**: the box must be the
    /// same at 45°, where the triangle-inequality bound this arm used
    /// to compute would claim `r√2`.
    #[test]
    fn the_planar_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        for &r in &[0.001, 1.0, 250.0] {
            for span_deg in [10.0_f64, 90.0, 179.0, 181.0, 300.0, 359.0] {
                for phi_deg in [0.0_f64, 45.0, 137.0] {
                    let phi = phi_deg.to_radians();
                    let (body, face) = arc_sector_from(r, span_deg.to_radians(), phi);
                    let b = face_box(&body, face, pad).unwrap();
                    agrees_with_the_rule(
                        &b,
                        &Aabb {
                            min_x: -r - pad,
                            min_y: -r - pad,
                            min_z: -pad,
                            max_x: r + pad,
                            max_y: r + pad,
                            max_z: pad,
                        },
                        r,
                        &format!(
                            "the planar arm (r = {r}, span = {span_deg}°, u_ref at {phi_deg}°)"
                        ),
                    );
                }
            }
        }
    }

    /// **`CylinderSlab`** — the axial range is the boundary's own and
    /// the radial half-width is the radius, **perpendicular to the
    /// axis only**. The fixture's axis is `z`, so the z face of the
    /// box is the trim's own `[z0, z1]` plus the pad and NOTHING
    /// else: the radius does not appear there, and neither does a
    /// second application of the pad. That is the whole of #862's
    /// measured case — a radius-`r` cylinder over `z ∈ [z0, z1]`
    /// claimed over `z ∈ [z0 − r, z1 + r]` — stated as an equality
    /// this row cannot pass with the width restored.
    #[test]
    fn the_cylinder_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        for &r in &[0.002, 1.0, 40.0] {
            for span_deg in [30.0_f64, 170.0, 200.0, 350.0] {
                let (z0, z1) = (-0.25 * r, 0.75 * r);
                let (body, face) = cyl_wall(r, 0.0, span_deg.to_radians(), z0, z1);
                let b = face_box(&body, face, pad).unwrap();
                agrees_with_the_rule(
                    &b,
                    &Aabb {
                        min_x: -r - pad,
                        min_y: -r - pad,
                        min_z: z0 - pad,
                        max_x: r + pad,
                        max_y: r + pad,
                        max_z: z1 + pad,
                    },
                    r,
                    &format!("the cylinder arm (r = {r}, span = {span_deg}°)"),
                );
            }
        }
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
        let b = face_box(&body, face, pad).unwrap();
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

    /// **`WholeBall`** — `center ± r`, reading nothing from the
    /// boundary, so the trim the fixture carries must not appear in
    /// the box at all. No deviation terms: this arm claims exactly
    /// what its rule states.
    #[test]
    fn the_sphere_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        for &r in &[0.002, 1.0, 40.0] {
            let c = Point3::new(0.3 * r, -0.2 * r, 0.1 * r);
            let (mut body, face) = arc_sector(r, core::f64::consts::PI);
            body.set_face_surface(
                face,
                FaceSurface::New(Surface::Sphere {
                    center: c,
                    radius: r,
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                }),
            )
            .unwrap();
            let b = face_box(&body, face, pad).unwrap();
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
        let b = face_box(&body, face, pad).unwrap();
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
    /// CALL of [`face_box`], [`face_box_rule`], [`edge_box`] and
    /// [`edge_box_rule`] in code — comments and literals blanked by
    /// [`crate::source_walk::CodeOnly`], the more competent of this
    /// crate's two readers — pinned per file.
    ///
    /// **Both rules, not just the face one.** The header's claim is
    /// about *a box*, and every face box that hulls a boundary is
    /// [`EdgeBoxRule`]'s answer one dimension down. A walk that
    /// matched only `face_box` would attribute that arm's cost to a
    /// door list computed for a different function, and an
    /// edge-box-only door would land green.
    ///
    /// - `boolean/reduce.rs` — the C10 candidate tree, face and edge.
    ///   **Prunes**: loose is slower work, never a different answer.
    ///   The only door for which that is true.
    /// - `boolean/ops.rs` — the sphere-extent fallback, face and
    ///   edge: the cylinder-face arm clears a [`face_box`] against
    ///   the ball's extent, the scan's near-boundary test walks the
    ///   face's [`edge_box`]es against the germ circle's box, and the
    ///   cone/torus arm consults a [`face_box`] before refusing by
    ///   kind — reach first, kind second, as at the operand gate.
    ///   **Refuses**: whichever box fails to clear turns the pair
    ///   into `FallbackExtentUnsupported`.
    /// - `separation.rs` — the placement certificate. **Refuses**:
    ///   non-overlap IS the grant.
    /// - `census.rs` — `reach_box` and `edge_reach`, this module's
    ///   extents entered at the census's own scalar. **Refuses**:
    ///   arm 2 clears only on a definitely negative margin against a
    ///   CONTAINING box, so over-width is a false `CensusUndecidable`.
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
    ///    that is the tree this crate can see. All four functions are
    ///    `pub(crate)`, so no such door can exist today — that, and
    ///    not a survey, is what carries the claim out of crate.
    /// 2. A door reading a box through a helper defined here: a
    ///    wrapper's own callers are invisible to a textual walk.
    /// 3. A call spelled through an alias or a re-export, or one a
    ///    macro assembles.
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
        const PINNED: [(&str, usize); 4] = [
            ("boolean/ops.rs", 3),
            ("boolean/reduce.rs", 5),
            ("census.rs", 3),
            ("separation.rs", 1),
        ];
        const HOME: &str = "boolean/boxes.rs";
        const DOORS: [&str; 4] = ["face_box(", "face_box_rule(", "edge_box(", "edge_box_rule("];
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
            let calls: usize = DOORS.iter().map(|d| code.as_str().matches(d).count()).sum();
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
            let kind = geom_brep::SurfaceKind::of(&s);
            let (mut body, face) = arc_sector(1.0, core::f64::consts::PI);
            body.set_face_surface(face, FaceSurface::New(s)).unwrap();
            let b = face_box(&body, face, pad()).unwrap();
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

    /// A CONE wall face: the patch `u ∈ [u0, u1] × z ∈ [z0, z1]` on
    /// the cone of half-angle `alpha` about `z` with its apex at the
    /// origin. Rims are the cone's own circles, sides its generators.
    fn cone_wall(alpha: f64, u0: f64, u1: f64, z0: f64, z1: f64) -> (Body<f64>, FaceKey) {
        let (mut body, face) = revolved_wall(&|z| z * alpha.tan(), u0, u1, z0, z1);
        body.set_face_surface(
            face,
            FaceSurface::New(Surface::Cone {
                apex: Point3::origin(),
                axis: Vec3::unit_z(),
                half_angle: alpha,
                u_ref: Vec3::unit_x(),
            }),
        )
        .unwrap();
        (body, face)
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
                    let b = face_box(&body, face, pad()).unwrap();
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
    /// axial image is a point ([`edge_axial_span`]) and the window is
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
                    let b = face_box(&body, face, pad).unwrap();
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
                body.set_face_surface(
                    face,
                    FaceSurface::New(Surface::Torus {
                        center,
                        axis,
                        major_radius: major,
                        minor_radius: minor,
                        u_ref,
                    }),
                )
                .unwrap();
                let b = face_box(&body, face, pad()).unwrap();
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

    /// **`WholeTorus`** — `(R + r)` perpendicular to the axis and `r`
    /// along it, reading nothing from the boundary, so the trim the
    /// fixture carries must not appear in the box at all.
    #[test]
    fn the_torus_arms_box_is_exactly_the_construction_its_rule_states() {
        let pad = pad();
        for &(major, minor) in &[(2.0, 0.5), (0.01, 0.004), (60.0, 12.0)] {
            for axis in [Vec3::unit_z(), Vec3::new(1.0, 2.0, 3.0).normalize()] {
                let c = Point3::new(0.3 * major, -0.2 * major, 0.1 * major);
                let (mut body, face) = arc_sector(major, core::f64::consts::PI);
                body.set_face_surface(
                    face,
                    FaceSurface::New(Surface::Torus {
                        center: c,
                        axis,
                        major_radius: major,
                        minor_radius: minor,
                        u_ref: axis.orthonormal_basis().0,
                    }),
                )
                .unwrap();
                let b = face_box(&body, face, pad).unwrap();
                let reach = |a: f64| (major + minor) * (1.0 - a * a).sqrt() + minor * a.abs();
                let (rx, ry, rz) = (reach(axis.x), reach(axis.y), reach(axis.z));
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
                    &format!("the torus arm (R = {major}, r = {minor}, axis {axis:?})"),
                );
            }
        }
    }

    /// **The two box lanes, side by side on one body** — what #700
    /// asked for and nothing did.
    ///
    /// The extents are shared now, so this row is not guarding
    /// arithmetic: it guards what is NOT shared, the census's own
    /// arena walk of a face's boundary against this module's. Those
    /// two walks can drift — a loop order, an isolated-vertex loop, a
    /// half-edge's edge — and a divergence between the census's boxes
    /// and the boolean sweep's is exactly the shape that produces a
    /// wrong census verdict with both halves looking correct on their
    /// own.
    ///
    /// Compared at `pad = 0`, where the only difference the module
    /// admits is [`Aabb::padded`]'s outward ulp.
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
            body.set_face_surface(face, FaceSurface::New(s)).unwrap();
            (body, face)
        };
        let (nurbs_body, nurbs_face, _) = nurbs_bulge_face();
        let cases: Vec<(&str, (Body<f64>, FaceKey))> = vec![
            ("plane", arc_sector(2.0, 2.3)),
            ("cylinder", cyl_wall(1.5, 0.0, 2.4, -0.5, 1.25)),
            ("cone", cone_wall(0.5, 0.0, 2.4, 0.4, 1.0)),
            ("sphere", relabelled(sphere)),
            ("torus", relabelled(torus)),
            ("nurbs", (nurbs_body, nurbs_face)),
        ];
        for (what, (body, face)) in cases {
            let boxed = face_box(&body, face, 0.0).unwrap();
            let (lo, hi) = crate::census::face_reach(&body, face)
                .unwrap_or_else(|| panic!("{what}: the census lane claims nothing"));
            for (name, g, w) in [
                ("min_x", boxed.min_x, lo.x),
                ("min_y", boxed.min_y, lo.y),
                ("min_z", boxed.min_z, lo.z),
                ("max_x", boxed.max_x, hi.x),
                ("max_y", boxed.max_y, hi.y),
                ("max_z", boxed.max_z, hi.z),
            ] {
                assert!(
                    (g - w).abs() <= 4.0 * f64::EPSILON * (1.0 + w.abs()),
                    "{what}: the two lanes disagree at {name}: boolean {g}, census {w}"
                );
            }
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
            &SpanBox::vector(Vec3::<f64>::unit_z()),
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
    /// is a point — so they are planted here, on the shared extents,
    /// rather than through a body at a scalar the row cannot pick.
    ///
    /// 1. The axial projection: an axis whose bracket spans two
    ///    directions must give an axial range that ENCLOSES what
    ///    either endpoint alone would give, never sit at one of them.
    ///    (It is [`edge_axial_span`] that does the projecting now —
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
        let axis = SpanBox {
            x: Span { lo: 0.0, hi: 0.6 },
            y: Span { lo: 0.0, hi: 0.0 },
            z: Span { lo: 0.8, hi: 1.0 },
        };
        let at = SpanBox::point(Point3::new(1.0, 0.0, 0.0));
        let h = edge_axial_span(&origin, &axis, &AxialCarrier::Chord, (&at, &at));
        assert!(
            h.lo <= 0.0 && h.hi >= 0.6,
            "the axial projection must enclose both endpoints' answers, got [{}, {}]",
            h.lo,
            h.hi
        );
        // Perpendicular room: `axis.z` is not certainly ±1, so the z
        // coordinate takes the room `√(1 − 0.8²) = 0.6` of the radius.
        let slab = slab_extent(&origin, &axis, Span::exact(0.0), 1.0);
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
}
