//! The vertex-neighborhood **sector-shape** predicates: one metering
//! arm, one wideness verdict, one subdivision direction — shared by the
//! two lanes that ask this question.
//!
//! # What the question is
//!
//! Both the splitting lane (`splitting::neighborhood`) and the
//! boolean lane ([`crate::boolean::sectors`]) walk a vertex orbit and,
//! for the sector CW-after orbit half-edge `he`, must answer three
//! things about the corner bounded by `he`'s own outgoing chord and the
//! next orbit chord:
//!
//! 1. **Is the corner metered at all?** The lever arm is the shorter of
//!    the two bounding chords (metres, from the same
//!    [`geom_brep::edge_extent`] machinery on both sides); a
//!    non-positive arm is a degenerate neighborhood, refused typed.
//! 2. **Is the corner convex (< 180°)?** `sin θ = (b̂ × â)·n` metered at
//!    the arm. Every face is planar-or-charted, so a sector's interior
//!    is the positive cone of its bounds **iff** the angle is < 180°;
//!    only then does one entry suffice.
//! 3. **If not definitely convex, where does it get subdivided?** Any
//!    interior direction with both sub-angles < 180° is valid. Definite
//!    reflex ⇒ `−normalize(â + b̂)` (the true bisector of the reflex
//!    span); the straight band (θ ≈ π, where `â + b̂` collapses) ⇒
//!    `n × b̂`, 90° into the interior. θ ≈ 0 or ≈ 2π between two
//!    DISTINCT edges is a spike — ill-conditioned, refused rather than
//!    guessed; the same reading on a one-edge orbit (a strut vertex) is
//!    the legitimate full-circle sector and takes the same 90° device.
//!
//! The three items above are the CONTRACT — what each rung decides and
//! what it returns. The **derivations** — why convex subdivision beats
//! the paper's complement-and-negate, why the wideness trilean has no
//! escalation cliff, why the duplicate entry is what makes dangling
//! null edges fall out of the generic run scan — stay in
//! `splitting::neighborhood`'s module docs, which own them, and are not
//! restated here.
//!
//! # Why the code is here and not in either lane
//!
//! These three rungs used to exist **twice**, once per lane,
//! dimensionally identical line-for-line under the correspondence
//! `dir_a ↔ dir_end`, `dir_b ↔ dir_start` — the same `min` of the same
//! two chord norms, the same `Margin::levered(sin, arm)`, the same
//! `.dot()` fallback, the same bisectors, the same spike guard. This
//! module is a **top-level sibling** of `boolean/` and `splitting/`
//! precisely so neither half hosts the other's core: both lanes already
//! depend on this scope (`crate::body`, `crate::entity`,
//! `crate::validate::decide`), so sharing here adds no dependency edge
//! between the halves and no public API.
//!
//! **The alternative considered and rejected: `geom-brep`.** The body
//! is pure vector algebra over [`Vec3`], [`Band`] and a name — no
//! `Body`, no arena keys — and `geom-brep` already hosts exactly this
//! shape (`enters_material` is a named K predicate consumed by both
//! lanes, and [`geom_brep::edge_extent`], the arm's source, is already
//! there). It sits BELOW both halves, so it is strictly more neutral
//! than the crate root. Two things ruled it out, and only one of them
//! is permanent:
//!
//! - **Permanent:** it would promote an internal algebra to public API
//!   in a crate re-exported into four others, for no consumer outside
//!   `topo`.
//! - **Expired, and recorded as having expired:** `crates/geom-brep/src/`
//!   was held by an in-flight lane (#639) this module's unit was told
//!   not to collide with. #639 has since landed, so that reason is
//!   spent and carries no weight here any more.
//!
//! **Re-open trigger — FIRED, and it resolves the other way.** The
//! `sector_face` twins are now unified as [`crate::sector_face`], the
//! second consumer this paragraph was waiting for. That consumer takes
//! a [`Body`](crate::body::Body) and three arena keys, so `geom-brep` —
//! which has neither — cannot host IT at any price. That settles where
//! `sector_face` goes; it does not by itself settle this module, whose
//! own body is still pure [`Vec3`]/[`Band`] algebra and would compile
//! in `geom-brep` today. What keeps it here is the PERMANENT bullet
//! above (public API in a crate re-exported into four others, for no
//! consumer outside `topo`) plus the weaker preference that the two
//! shared sector modules be read together. The expired bullet is not
//! load-bearing, and the second consumer's arrival strengthened the
//! case for `geom-brep` for THIS module rather than weakening it — so
//! whoever re-opens it should weigh the public-API cost, which is the
//! only argument still standing.
//!
//! # Which scalars the argument-order equality is proven for
//!
//! Two of the three rungs changed argument order when the lanes
//! merged:
//! the reflex bisector's `â + b̂` (the boolean wrote `b̂ + â`) and the
//! straight rung's `â · b̂` (the boolean wrote `b̂ · â`). Bit-identity
//! under that swap is what makes the merge K-neutral, so it is worth
//! being exact about which scalars it is PROVEN for — the kernel is
//! generic over [`Real`], and the proof is not.
//!
//! - **`f64`, and the recording scalar `geom_core::k_stats::Probe`
//!   (an `f64` newtype): proven.** [`Vec3::dot`] is the fixed
//!   association `((x·x′) + (y·y′)) + (z·z′)`, so swapping the
//!   arguments commutes each product and permutes nothing in the sum;
//!   IEEE `*` and `+` are commutative on `f64` at every finite input,
//!   ±0 included. `geom_core`'s `dot_symmetry_bit_exact` proptest pins
//!   it (over `1.0e-3..1.0e3`, so 0, −0, inf and subnormals are argued
//!   rather than sampled), and the 26541-sample K stream reproduced in
//!   the unit's PR runs at `Probe`. The one f64 gap is NaN, whose
//!   payload propagation is not specified bitwise — and a NaN chord
//!   cannot reach these two rungs, since it fails the arm rung above.
//! - **`geom_core::Interval`: NOT proven.** It is also a [`Decide`]
//!   scalar and it is live for both lanes (`topo/tests/interval_body.rs`,
//!   `m3_pr3_split.rs`, `m3_pr4_boolean.rs`), and its `Add`/`Mul`
//!   delegate to the enclosure backend, whose bit-level commutativity
//!   is asserted NOWHERE in-tree. The equality is expected — a rounded
//!   endpoint of a symmetric operation is a symmetric function of the
//!   operands — but expected is not pinned. Closing it needs a
//!   `Vec3<Interval>` analogue of `dot_symmetry_bit_exact` in
//!   `geom-core`; until that exists, the interval lane's sector margins
//!   are equal by argument, not by proof. Nothing here is broken by
//!   that: a divergence would be an enclosure differing in its last
//!   bit, not a verdict changing.
//!
//! # One K name per rung
//!
//! The three rungs emit `sector_arm`, `sector_reflex` and
//! `sector_straight` — **one name each, spelled here, for both
//! lanes**. Until #652 they were six, `bool_sector_*` and
//! `split_sector_*`, handed in as a `SectorPredicates` parameter so
//! that #647's merge of the two bodies could be K-neutral; Ev ruled
//! the two populations one (2026-08-19, #652), so the parameter is
//! gone — there is nothing left for it to vary. **The evidence for
//! that ruling, and the two-way precedent it weighed, live in
//! `docs/K-REPORT.md`'s census note (2026-08-19)**, which is the
//! decision's home and is not restated here. What binds code is two
//! rules:
//!
//! - **Spelled here, nowhere else.** A lane that wants one of these
//!   margins calls [`sector_shape`]; the guard row below reads every
//!   `.rs` file under `topo/src` for the quoted form of these three
//!   names and of the six retired ones.
//! - **The pooled names are NEW spellings, not either lane's old one.**
//!   Keeping the 29:1 majority `bool_sector_*` was the cheap choice and
//!   fails on its own terms: post-merge, every splitting-lane decision
//!   would be filed under a prefix that says `bool_` — an actively
//!   FALSE lane label on rows carrying nothing else to distinguish
//!   them. A fresh spelling also self-dates a K row, which matters
//!   because the K report compares snapshots across milestones. Both
//!   arguments are quantified in the census note; the CSVs under
//!   `docs/k-report-data/` are left exactly as the sweep wrote them.
//!
//! # Naming
//!
//! The two lanes name the bounds oppositely (`dir_a`/`dir_b` in
//! splitting, `dir_end`/`dir_start` in the boolean). This module uses
//! neither: `own` is the sector's own orbit chord (the CCW-**last**
//! bound, splitting's `dir_a`, the boolean's `dir_end`) and `next` is
//! the next orbit chord (the CCW-**first** bound, splitting's `dir_b`,
//! the boolean's `dir_start`).

use geom_brep::OutwardNormal;
use geom_core::{
    Band, Decide, Indeterminate, Margin, MarginDiag, Real, Sign, Vec3, is_finite_length,
    is_underflowed_length,
};

use crate::validate::decide;

/// Rung 1's K name: the metering arm is positive.
///
/// This and its two siblings are PRIVATE to this module. They are rows
/// of the census in `docs/K-REPORT.md`, and no other file can IMPORT
/// one and re-implement the rung under it. Privacy stops the import,
/// not a bare string literal; what stops that is the guard row
/// `tests::the_rungs_are_decided_in_one_place` below.
const SECTOR_ARM: &str = "sector_arm";

/// Rung 2's K name: the corner is convex (`sin θ` levered at the arm).
const SECTOR_REFLEX: &str = "sector_reflex";

/// Rung 3's K name: the straight/spike disambiguation (`cos θ` levered
/// at the arm), reached only when rung 2 is not definitely signed.
const SECTOR_STRAIGHT: &str = "sector_straight";

/// Why [`sector_shape`] refused.
///
/// Two causes, kept apart because their recourses are different
/// sentences: a bounding chord whose length is not a NUMBER is not a
/// coincidence at any tolerance and no tolerance lever reaches it,
/// while everything the rungs themselves refuse is a band question
/// carrying a classifier payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SectorFault {
    /// A bounding chord's length is not a finite number: its
    /// components overflow the norm (past ~1e154), or one of them is
    /// not a number. Carries nothing — which of the two chords it was
    /// is not a distinction a caller can act on, and the recourse is
    /// the same either way.
    NonFiniteChord,
    /// A bounding chord's length **underflowed out of the format**:
    /// its components are small enough (below ~1e-162 at `f64`) that
    /// `norm_squared` flushed to zero, so the norm is exactly zero for
    /// a chord that has a perfectly good direction. Carries nothing,
    /// for the reason its overflow sibling carries nothing.
    ///
    /// Its own arm because no tolerance lever reaches it either: the
    /// squared norm is zero at every eps, so the arm rung below decides
    /// `Zero` definitely and the corner is reported as an invalid
    /// (non-positive) arm. That is true of the arithmetic and false of
    /// the input, whose recourse is the overflow end's — scale.
    UnderflowedChord,
    /// A rung refused or escalated, named by the rung inside.
    Rung(Indeterminate),
}

/// What the sector-shape rungs decided about one corner.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SectorShape<T: Real> {
    /// The metering arm: the shorter of the two bounding chords, in
    /// metres. Every downstream margin of this sector is levered at it.
    pub arm: T,
    /// Unit direction of the sector's own orbit chord (CCW-last bound).
    pub unit_own: Vec3<T>,
    /// Unit direction of the next orbit chord (CCW-first bound).
    pub unit_next: Vec3<T>,
    /// `None` when the corner is definitely convex (one entry
    /// suffices); otherwise the interior direction the sector is
    /// convexly subdivided at.
    pub bisector: Option<Vec3<T>>,
}

/// Climbs the three rungs for one sector (module docs).
///
/// `dir_own` and `dir_next` are the bounding chords **scaled to their
/// edges' honest extents** — their norms are the arm's raw material and
/// their directions the bounds. `normal` is the sector face's
/// [`OutwardNormal`] at the base vertex, minted by each lane's
/// `sector_face`; the type carries the obligation that used to be a
/// sentence here. This function must not re-apply the sense — the
/// bounds come from the STORED orbit order, which `revert` reverses in
/// the same breath as the sense bit, so a second factor would
/// double-count and read every convex corner as reflex.
/// `full_circle` says the two bounds are the SAME orbit half-edge (a
/// strut vertex), which is what makes a θ ≈ 0 / ≈ 2π reading
/// legitimate rather than a spike.
///
/// **Rung 0: are both chord lengths NUMBERS?** Asked before rung 1,
/// and asked of each chord SEPARATELY, because rung 1's arm is their
/// `min` and [`Real::min`] propagates `NaN` but not infinity — so
/// `min` hides an OVERFLOWED chord behind a finite one (`min(3, ∞) =
/// 3`) while a poisoned one stops rung 1 on its own. A chord past
/// [`Vec3::normalize`]'s ~1e154 overflow band has an infinite norm;
/// hidden under the `min`, rung 1 sees a healthy arm, passes, and the
/// chord normalizes to the ZERO vector.
///
/// Measured on this body before the question went first, over all
/// eight shapes (four chord pairs × `full_circle`): **two** of them
/// returned `Ok`, both with `full_circle` set and one chord
/// overflowed, once in each order. One produced an all-zero bisector;
/// the other produced `(-1,0,0)` — a unit direction built by `n ×
/// unit_next` from the chord that survived, with nothing about it to
/// smell. The other six refused, but never for this reason and never
/// with a recourse that could work: the two poisoned shapes escalated
/// at rung 1, the two doubly-overflowed ones at rung 3, and the two
/// single-overflow shapes with `full_circle` clear were refused as
/// SPIKES. The tests carry the full table.
///
/// **Rung 0 asks the other end of the format too**, per chord for the
/// same reason: a chord below `Vec3::normalize`'s ~1e-162 underflow
/// band squares to zero, so `norm` is EXACTLY zero, and `min` puts that
/// zero straight into the arm — an underflowed chord hides behind a
/// good one exactly as an overflowed one did, and it hides HARDER,
/// because zero is what `min` prefers. The arm rung then decides `Zero`
/// definitely and the corner is refused as a non-positive arm: true of
/// the arithmetic, false of the chord, and offering a band that cannot
/// reach a squared norm of zero. `is_underflowed_length` is asked
/// against each chord's own largest `|component|`
/// (`Vec3::norm_witness`), which is that predicate's contract, and it
/// is asked AFTER the finiteness question because an overflowed or
/// poisoned length makes its two ratios non-finite for an unrelated
/// reason.
///
/// **Point-scalar gates.** Both are value questions, and at
/// `T = Interval` neither bites: `Interval::is_poison` is
/// `is_nai() || is_empty()`, and `[1e200, ∞] − [1e200, ∞]` is
/// `[−∞, ∞]`, which answers finite; and a norm whose lower end
/// underflowed still ENCLOSES the true length, so the underflow ratio
/// is an unbounded enclosure rather than poison and the question
/// answers `false`. So rung 0 bites at `f64` and `Probe` and waves an
/// interval chord through to the rungs below. No live caller
/// instantiates this body at `Interval` today; see
/// `geom_core::is_finite_length` for the general statement.
///
/// **K consequence.** Rung 0 refuses before the funnel, so a
/// non-finite chord contributes no sample under `sector_arm` (nor,
/// where it used to reach them, under `sector_reflex` or
/// `sector_straight`). Those samples were a `+∞` margin recorded as a
/// definite `Positive` and two NaN margins recorded as escalations.
/// Rung 0 emits no sample of its own: it is a value-channel question,
/// not a classification, so it adds no row to `docs/K-REPORT.md`.
///
/// # Errors
///
/// [`SectorFault::NonFiniteChord`] when either bounding chord's
/// length is not a finite number;
/// [`SectorFault::UnderflowedChord`] when either underflowed out of
/// the format; otherwise [`SectorFault::Rung`]
/// named by the rung that produced it — a `decide` escalation passed
/// through unchanged, or a [`MarginDiag::Invalid`] diagnostic when a
/// definite verdict is one this predicate does not admit
/// (non-positive arm; a spike between distinct edges). Each lane
/// wraps this in its own error type — the two wrappings are the only
/// thing that was ever genuinely per-lane here.
pub(crate) fn sector_shape<T: Decide>(
    dir_own: Vec3<T>,
    dir_next: Vec3<T>,
    normal: OutwardNormal<T>,
    full_circle: bool,
    band: Band,
) -> Result<SectorShape<T>, SectorFault> {
    let (norm_own, norm_next) = (dir_own.norm(), dir_next.norm());
    if !is_finite_length(norm_own) || !is_finite_length(norm_next) {
        return Err(SectorFault::NonFiniteChord);
    }
    // Per CHORD, against that chord's own witness — the arm below is
    // their `min`, which has no witness of its own and would hide the
    // underflowed one behind the good one.
    if is_underflowed_length(norm_own, dir_own.norm_witness())
        || is_underflowed_length(norm_next, dir_next.norm_witness())
    {
        return Err(SectorFault::UnderflowedChord);
    }
    let arm = norm_own.min(norm_next);
    match decide(SECTOR_ARM, Margin::of(arm), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(invalid(band, SECTOR_ARM)),
        Err(diag) => return Err(SectorFault::Rung(diag)),
    }
    let (unit_own, unit_next) = (dir_own.normalize(), dir_next.normalize());
    // Wideness: sin θ = (b̂ × â)·n metered at the arm. Positive ⇒
    // convex; negative ⇒ reflex; zero-band ⇒ disambiguate by cosine
    // (for unit bounds sin and cos cannot both vanish, so the second
    // margin is definite whenever the first is not).
    let n = normal.vec();
    let reflex_margin = Margin::levered(unit_next.cross(unit_own).dot(n), arm);
    let bisector = match decide(SECTOR_REFLEX, reflex_margin, band) {
        Ok(Sign::Positive) => None,
        // Definite reflex, θ ∈ (π, 2π). `unit_own + unit_next` is the
        // splitting lane's spelling and `unit_next + unit_own` the
        // boolean's; `Add` is componentwise, so this is bit-identical
        // to both at every scalar whose addition is bitwise commutative
        // — see the module docs on WHICH scalars that is proven for
        // (`f64`/`Probe` yes, `Interval` expected but unpinned). (The
        // collapse â + b̂ → 0 happens only at θ → π, which lands in the
        // Zero band below, never here.)
        Ok(Sign::Negative) => Some(-((unit_own + unit_next).normalize())),
        Ok(Sign::Zero) | Err(_) => {
            // Likewise `unit_own.dot(unit_next)` vs the boolean's
            // `unit_next.dot(unit_own)`: componentwise products, same
            // summation order — bit-identical under the same scalar
            // scope as above, not for every `T: Real` unconditionally.
            let straight_margin = Margin::levered(unit_own.dot(unit_next), arm);
            match decide(SECTOR_STRAIGHT, straight_margin, band) {
                // θ ≈ π: 90° into the interior is valid throughout the
                // band.
                Ok(Sign::Negative) => Some(n.cross(unit_next)),
                // θ ≈ 0 or ≈ 2π on a one-edge orbit: the legitimate
                // full-circle sector, same device.
                Ok(Sign::Positive | Sign::Zero) if full_circle => Some(n.cross(unit_next)),
                // A spike corner between two distinct edges: refuse,
                // never guess an interior direction.
                Ok(Sign::Positive | Sign::Zero) => return Err(invalid(band, SECTOR_STRAIGHT)),
                Err(diag) => return Err(SectorFault::Rung(diag)),
            }
        }
    };
    Ok(SectorShape {
        arm,
        unit_own,
        unit_next,
        bisector,
    })
}

/// The diagnostic for a definite verdict this predicate does not admit
/// — spelled identically in both lanes before the merge.
///
/// The home is `impl Indeterminate` in
/// `geom-core/src/predicate.rs:724`. Unifying it is a public-API
/// addition plus a four-crate sweep — deliberately not folded into the
/// unit that shared these rungs, and recorded here so the next pass
/// finds the home rather than the method.
fn invalid(band: Band, predicate: &'static str) -> SectorFault {
    SectorFault::Rung(Indeterminate {
        margin: MarginDiag::Invalid,
        band,
        predicate: Some(predicate),
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
        Vec3::new(x, y, z)
    }

    /// `Vec3` carries no `PartialEq`; these rows want BIT equality, not
    /// a tolerance.
    fn bits_eq(a: Vec3<f64>, b: Vec3<f64>) -> bool {
        a.x.to_bits() == b.x.to_bits()
            && a.y.to_bits() == b.y.to_bits()
            && a.z.to_bits() == b.z.to_bits()
    }

    fn assert_dir(got: Vec3<f64>, want: Vec3<f64>) {
        assert!(
            (got.x - want.x).abs() < 1e-15
                && (got.y - want.y).abs() < 1e-15
                && (got.z - want.z).abs() < 1e-15,
            "direction {got:?} is not {want:?}"
        );
    }

    fn shape(
        own: Vec3<f64>,
        next: Vec3<f64>,
        full_circle: bool,
    ) -> Result<SectorShape<f64>, SectorFault> {
        let n = OutwardNormal::from_chart(v(0.0, 0.0, 1.0), true);
        sector_shape(own, next, n, full_circle, band())
    }

    /// A right-angle corner (θ = 90°, `sin θ` = +1) is convex: no
    /// subdivision, arm = the shorter bounding chord.
    #[test]
    fn convex_corner_needs_no_subdivision() {
        // own = +y (CCW-last), next = +x (CCW-first): the sector sweeps
        // +x → +y CCW around +z, so (b̂ × â)·n = (x̂ × ŷ)·ẑ = +1.
        let s = shape(v(0.0, 3.0, 0.0), v(2.0, 0.0, 0.0), false).unwrap();
        assert!(s.bisector.is_none());
        assert_eq!(s.arm, 2.0);
        assert_dir(s.unit_own, v(0.0, 1.0, 0.0));
        assert_dir(s.unit_next, v(1.0, 0.0, 0.0));
    }

    /// A definitely reflex corner (θ = 270°) subdivides at
    /// `−normalize(â + b̂)`, which points INTO the reflex span.
    #[test]
    fn reflex_corner_subdivides_at_the_reflex_bisector() {
        // own = +x, next = +y: (ŷ × x̂)·ẑ = −1 ⇒ reflex.
        let s = shape(v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), false).unwrap();
        let b = s.bisector.expect("a reflex corner subdivides");
        assert!(bits_eq(b, -(v(1.0, 1.0, 0.0).normalize())));
        // The sector sweeps +y → +x the LONG way; the bisector is
        // strictly interior to it, i.e. on the far side from (+1, +1).
        assert!(b.dot(v(-1.0, -1.0, 0.0)) > 0.0);
    }

    /// A straight corner (θ = 180°, where `â + b̂` collapses) falls to
    /// the cosine rung and subdivides 90° into the interior.
    #[test]
    fn straight_corner_subdivides_ninety_degrees_in() {
        let s = shape(v(1.0, 0.0, 0.0), v(-1.0, 0.0, 0.0), false).unwrap();
        // n × b̂ = ẑ × (−x̂) = −ŷ.
        assert_dir(
            s.bisector.expect("a straight corner subdivides"),
            v(0.0, -1.0, 0.0),
        );
    }

    /// A spike between two DISTINCT edges (θ ≈ 0) refuses, named by the
    /// cosine rung — it is ill-conditioned, not a full circle.
    #[test]
    fn spike_between_distinct_edges_refuses_named() {
        let e = shape(v(1.0, 0.0, 0.0), v(1.0, 0.0, 0.0), false)
            .expect_err("a spike has no valid interior direction");
        // LITERAL pins, restored across the `SectorFault` boundary.
        // `assert_eq!(e, invalid(band(), "sector_straight"))` reads
        // shorter and says less: `invalid` is production code thirty
        // lines up, so that form compares the thing under test against
        // itself and stays green if it starts emitting a different
        // `MarginDiag`.
        let SectorFault::Rung(e) = e else {
            panic!("a spike is a rung refusal, not a chord-length one: {e:?}");
        };
        assert_eq!(e.predicate, Some("sector_straight"));
        assert_eq!(e.margin, MarginDiag::Invalid);
        assert_eq!(e.band, band());
    }

    /// **Rung 0, the other end: a chord whose length UNDERFLOWED.**
    /// Components below ~1e-162 square to zero, so `norm` is exactly
    /// zero for a chord that names its direction perfectly well.
    ///
    /// **Measured on this body without the underflow arm, all eight
    /// shapes** — four chord pairs × `full_circle`. Every row was
    /// executed: all eight refused identically, as
    /// `Rung(Indeterminate { margin: Invalid, band, predicate:
    /// Some("sector_arm") })` — this body's own [`invalid`] spike
    /// refusal, which says the arm is non-positive and offers the
    /// band. Both halves are false of the input: the chord is not
    /// short, it is unmeasurable, and no band reaches a squared norm
    /// of zero.
    ///
    /// **`min` hides this end HARDER than the overflow end.** There,
    /// `min(3.0, ∞) = 3.0` let a healthy-looking arm through to rung
    /// 3, where two of eight shapes returned `Ok`. Here
    /// `min(3.0, 0.0) = 0.0`, so the underflowed chord always WINS the
    /// `min` and the refusal is never silent — but it is never about
    /// the chord either, and the arm it names is the good chord's
    /// neighbour rather than the one that failed. Asking each chord
    /// separately is what recovers the cause; asking the arm could not,
    /// because the arm is a `min` of two chords and has no witness of
    /// its own.
    ///
    /// `full_circle` is carried through every row for the same reason
    /// the overflow rows carry it: it is the flag that separated silent
    /// from loud there, and a row that dropped it would be testing less
    /// than its sibling.
    #[test]
    fn a_chord_whose_length_underflowed_refuses_at_rung_zero() {
        let tiny = 1e-200;
        // The premise: the norm flushed, and the direction survives in
        // the witness the underflow question is asked against.
        assert_eq!(v(0.0, tiny, 0.0).norm(), 0.0);
        assert_eq!(v(0.0, tiny, 0.0).norm_witness(), tiny);
        for full_circle in [false, true] {
            for (own, next) in [
                (v(0.0, tiny, 0.0), v(0.0, tiny, 0.0)),
                (v(0.0, 3.0, 0.0), v(tiny, 0.0, 0.0)),
                (v(tiny, 0.0, 0.0), v(0.0, 3.0, 0.0)),
                (v(tiny, tiny, tiny), v(0.0, 3.0, 0.0)),
            ] {
                assert_eq!(
                    shape(own, next, full_circle).err(),
                    Some(SectorFault::UnderflowedChord),
                    "{own:?} / {next:?} / full_circle={full_circle}"
                );
            }
        }
        // A chord that is genuinely SHORT rather than unmeasurable is
        // still the arm rung's, not this one's: 1e-12 squares to 1e-24,
        // which the format holds.
        assert_eq!(
            shape(v(0.0, 1e-12, 0.0), v(0.0, 3.0, 0.0), false).err(),
            Some(invalid(band(), SECTOR_ARM))
        );
        // And the zero chord is not an underflowed one: it has no
        // direction to recover, so its witness is zero too and the
        // predicate's two ratios are both poison.
        assert_eq!(
            shape(v(0.0, 0.0, 0.0), v(0.0, 3.0, 0.0), false).err(),
            Some(invalid(band(), SECTOR_ARM))
        );
    }

    /// **Rung 0: a chord whose length is not a NUMBER.** A chord past
    /// `Vec3::normalize`'s ~1e154 overflow band has an infinite norm,
    /// which is maximally definite to the arm rung, and then
    /// normalizes to the zero vector.
    ///
    /// **Measured on this body at the merge base (`5fbdda206`), all
    /// EIGHT shapes** — four chord pairs × `full_circle`. Every line
    /// below was executed, not argued:
    ///
    /// | chords (own / next) | `full_circle` clear | `full_circle` set |
    /// |---|---|---|
    /// | `1e200` / `1e200` | rung 3 escalation | rung 3 escalation |
    /// | finite / `1e200` | rung 3 `invalid` | **`Ok`, bisector `(0,0,0)`** |
    /// | `1e200` / finite | rung 3 `invalid` | **`Ok`, bisector `(-1,0,0)`** |
    /// | `NaN` / finite | rung 1 escalation | rung 1 escalation |
    ///
    /// **Two shapes returned `Ok`, not one, and the second is the
    /// worse of the pair.** With `full_circle` set and ONE chord
    /// overflowed, the door reported success either way round:
    ///
    /// - own finite, next overflowed — `arm: 3.0, unit_own: (0,1,0),
    ///   unit_next: (0,0,0), bisector: Some((0,0,0))`. A subdivision
    ///   direction of nothing.
    /// - own overflowed, next finite — `arm: 3.0, unit_own: (0,0,0),
    ///   unit_next: (0,1,0), bisector: Some((-1,0,0))`. The bisector
    ///   is `n × unit_next`, built from the chord that SURVIVED, so
    ///   it is a unit vector in a plausible direction with nothing
    ///   about it to smell. The all-zero bisector above at least
    ///   looks wrong to a downstream reader; this one does not.
    ///
    /// **Why `min` hides exactly one of the two ends.** [`Real::min`]
    /// propagates `NaN` but not infinity, so `min(3.0, ∞) = 3.0`
    /// (the arm looks healthy and rung 1 passes) while
    /// `min(3.0, NaN) = NaN` (rung 1 escalates immediately). That is
    /// the whole of the split above: the OVERFLOW end is hidden by
    /// `min` and reaches rung 3 or slips through, the POISON end
    /// never gets past rung 1. It is also why rung 0 asks each chord
    /// separately rather than asking the arm — asking the arm would
    /// have left both silent shapes open.
    ///
    /// The six loud shapes were loud in two different ways, neither
    /// of which names the chord or a recourse that could work: the
    /// two `NaN` rows and the two `1e200`/`1e200` rows are `decide`
    /// ESCALATIONS (a poisoned margin, `MarginDiag::Invalid`), while
    /// the two single-overflow rows with `full_circle` clear are this
    /// body's own [`invalid`] spike refusal. Those two OUTCOMES are
    /// indistinguishable by value — an escalation carrying
    /// `MarginDiag::Invalid` and an `invalid(band, p)` compare equal —
    /// which is why the rows below pin the two fields literally
    /// instead of comparing against [`invalid`], which is production
    /// code in this same file.
    ///
    /// The `full_circle` flag is carried through every row because it
    /// is what separates the silent shapes from the loud ones: a row
    /// that only tested the loud shapes would have gone green on the
    /// merge base.
    #[test]
    fn a_chord_with_no_finite_length_refuses_at_rung_zero() {
        let big = 1e200;
        for full_circle in [false, true] {
            for (own, next) in [
                (v(big, 0.0, 0.0), v(big, 0.0, 0.0)),
                (v(0.0, 3.0, 0.0), v(big, 0.0, 0.0)),
                (v(big, 0.0, 0.0), v(0.0, 3.0, 0.0)),
                (v(f64::NAN, 0.0, 0.0), v(0.0, 3.0, 0.0)),
            ] {
                assert_eq!(
                    shape(own, next, full_circle).err(),
                    Some(SectorFault::NonFiniteChord),
                    "{own:?} / {next:?} / full_circle={full_circle}"
                );
            }
        }
        // A finite pair still climbs the rungs — the row above cannot
        // be passing because rung 0 refuses everything.
        assert!(shape(v(0.0, 3.0, 0.0), v(2.0, 0.0, 0.0), false).is_ok());
    }

    /// The SAME reading on a one-edge orbit is the legitimate strut
    /// full-circle sector and subdivides instead of refusing — the row
    /// that separates the two, so the spike row above cannot be passing
    /// for the wrong reason.
    #[test]
    fn strut_full_circle_subdivides() {
        let s = shape(v(1.0, 0.0, 0.0), v(1.0, 0.0, 0.0), true).unwrap();
        assert_dir(
            s.bisector.expect("a strut vertex subdivides"),
            v(0.0, 1.0, 0.0),
        );
    }

    /// A collapsed bounding chord fails the arm rung, named.
    #[test]
    fn degenerate_arm_refuses_named() {
        let e = shape(v(1.0, 0.0, 0.0), v(0.0, 0.0, 0.0), false)
            .expect_err("a collapsed chord cannot meter the corner");
        // Literal pins, for the reason spelled out on the spike row.
        let SectorFault::Rung(e) = e else {
            panic!("a collapsed chord has a finite length: {e:?}");
        };
        assert_eq!(e.predicate, Some("sector_arm"));
        assert_eq!(e.margin, MarginDiag::Invalid);
        assert_eq!(e.band, band());
    }

    /// **The anti-re-fork row.** The three sector-shape K names are
    /// decided HERE and nowhere else in this crate, and the six
    /// per-lane names they replaced (#652) are decided NOWHERE at all:
    /// no file under `topo/src` may spell either set as a string
    /// literal. Re-forking the rungs means re-introducing a
    /// `decide("…_sector_arm", …)` somewhere, and that is exactly what
    /// this reads for.
    ///
    /// The retired names are BUILT from the pooled ones rather than
    /// written out, so the check covers this file like any other — a
    /// literal roster would be its own first counter-example, which is
    /// exactly how this row went red on its own module while being
    /// written.
    ///
    /// **What the derivation does NOT buy, said plainly because the
    /// obvious reading is the reverse of the truth.** It does not keep
    /// the two rosters in step across a future rename; it is what
    /// breaks them. Rename `SECTOR_ARM` and `retired` becomes
    /// `"bool_<newname>"` / `"split_<newname>"`, strings that have
    /// never existed anywhere, so the six genuinely retired spellings
    /// stop being guarded — silently, with nothing going red. #652 is
    /// itself the proof that renames happen. **Nothing in the tree
    /// protects the retired set across one.** Those six spellings
    /// survive only in prose (`docs/K-REPORT.md`'s census note) and in
    /// the committed CSVs they date. Whoever renames a rung next has to carry the old
    /// spellings forward by hand — and has to assemble them from parts
    /// the way this does, or this file becomes the counter-example
    /// again.
    ///
    /// It walks the whole of `src/` at RUNTIME rather than
    /// `include_str!`ing the two lane files it happens to know about,
    /// so a re-fork that grows in a third file — a new module under
    /// `boolean/` or `splitting/`, or a fresh crate-root sibling — is
    /// caught too, and the guard does not need editing when a lane is
    /// split across more files.
    ///
    /// **What replaced the second guard.** Before #652 this row had a
    /// sibling, `both_lanes_decide_the_same_shape`, which ran every
    /// input shape under both lanes' name sets and demanded
    /// bit-identical results — the guard against a re-fork from INSIDE
    /// the shared body (`names.arm == "…"`, a lane flag, a per-lane
    /// band). Pooling deleted its subject: [`sector_shape`] no longer
    /// takes a lane parameter, so there is nothing for the body to
    /// branch on and nothing for that row to compare. It is gone
    /// rather than trivially green.
    ///
    /// **The residue #647 shipped is closed.** Its third uncovered
    /// case — a lane re-implementing the rungs while IMPORTING the
    /// names, `decide(BOOL_SECTOR_PREDICATES.arm, …)`, which no string
    /// search can see — cannot happen: the three consts are private to
    /// this module, so no other file can name them at all.
    ///
    /// **What this still does not cover** (stated plainly rather than
    /// as one careful caveat that implies the rest is covered — four
    /// shapes, two of them found in review of #661):
    ///
    /// 1. **A re-fork under FRESH names.** Out of reach by
    ///    construction; that one surfaces as new rows in the
    ///    `docs/K-REPORT.md` census, which is the mechanism that
    ///    already exists for it.
    /// 2. **These names spelled outside `topo/src`.** The walk is
    ///    scoped to this crate. `crate::validate::decide` is
    ///    `pub(crate)`, so a foreign crate would have to bypass it and
    ///    call `geom_core::k_stats::decide` directly — unlikely, but
    ///    not excluded by anything here.
    /// 3. **A lane that CALLS [`sector_shape`] and then re-derives the
    ///    verdict from what comes back.** `arm`, `unit_own` and
    ///    `unit_next` are `pub(crate)` fields of [`SectorShape`], so a
    ///    lane can recompute `sin θ` or its own bisector and act on
    ///    that instead of on `bisector`. The rungs still fire exactly
    ///    once each, from here, so the K stream stays byte-identical,
    ///    the census shows nothing new, and no string search can see
    ///    it. It is the closed residue's shape one level in: the names
    ///    are no longer importable, but the rungs' INPUTS are.
    /// 4. **A re-fork that decides with NO K name at all** — a raw
    ///    comparison where a `decide` call belongs. Residue 1's
    ///    mitigation cannot reach this one: new census rows are how a
    ///    freshly-named re-fork surfaces, and this emits no row. What
    ///    catches it is review, or a hole noticed later in the census.
    ///
    /// **A permanent constraint on prose, not just on code.** The check
    /// is textual and matches the QUOTED forms, so it is not only a
    /// `decide` call that reds it: any doc comment anywhere under
    /// `topo/src` that spells a retired name inside real quote marks —
    /// documenting what was retired, say — fails this row. Backticks
    /// are fine, and are what the module docs above use throughout.
    /// That constraint is the price of the check being able to cover
    /// its own home file, and it does not expire.
    #[test]
    fn the_rungs_are_decided_in_one_place() {
        let pooled = [SECTOR_ARM, SECTOR_REFLEX, SECTOR_STRAIGHT];
        // The retired six, each WITH its surrounding quotes — assembled
        // rather than spelled, so this file is subject to the check like
        // any other.
        let retired: Vec<String> = ["bool", "split"]
            .iter()
            .flat_map(|lane| pooled.iter().map(move |rung| format!("\"{lane}_{rung}\"")))
            .collect();
        let home = crate::source_walk::src_root().join("sector_shape.rs");
        let files = crate::source_walk::crate_sources();
        assert!(
            files.contains(&home),
            "the walk found {} file(s) but not the home module — it is not reading topo/src",
            files.len()
        );
        for path in &files {
            let text = std::fs::read_to_string(path).expect("a readable source file");
            for name in &retired {
                assert!(
                    !text.contains(name.as_str()),
                    "{} decides the RETIRED per-lane sector predicate {name} — #652 \
                     pooled the six lane names into three. Call `sector_shape`, which \
                     spells the pooled name itself.",
                    path.display()
                );
            }
            if path == &home {
                continue;
            }
            for name in pooled {
                assert!(
                    !text.contains(&format!("\"{name}\"")),
                    "{} names the sector-shape predicate `{name}` again — the rungs \
                     have been re-forked out of sector_shape.rs, which must hold the \
                     only implementation. Call `sector_shape` instead.",
                    path.display()
                );
            }
        }
        // The guard earns its line only if the strings it looks for are
        // reachable at all: they must all be spelled HERE.
        let here = std::fs::read_to_string(&home).expect("the home module is readable");
        for name in pooled {
            assert!(
                here.contains(&format!("\"{name}\"")),
                "`{name}` is not spelled in this module"
            );
        }
    }
}
