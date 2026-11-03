//! The vertex-neighborhood **sector-shape** predicates: one metering
//! arm, one wideness verdict, one subdivision direction — shared by the
//! two lanes that ask this question.
//!
//! # What the question is
//!
//! Both the splitting lane (`splitting::neighborhood`) and the
//! boolean lane (`topo::boolean::sectors`) walk a vertex orbit and,
//! for the sector CW-after orbit half-edge `he`, must answer three
//! things about the corner bounded by `he`'s own outgoing chord and the
//! next orbit chord:
//!
//! 1. **Is the corner metered at all?** The lever arm is the shorter of
//!    the two bounds' lengths (metres: a line edge's chord, a curved
//!    edge's extent from below, [`crate::edge_extent_inf`], through
//!    [`crate::ConicDeparture`], on both sides); a non-positive arm
//!    is a degenerate neighborhood, refused typed.
//! 2. **Is the corner convex (< 180°)?** `sin θ = (b̂ × â)·n` metered at
//!    the arm. A face with a tangent plane at the vertex is
//!    planar-or-charted there, so a sector's interior is the positive
//!    cone of its bounds **iff** the angle is < 180°; only then does one
//!    entry suffice. At a cone's apex there is no tangent plane, and the
//!    interior is the fan of rulings between the bounds at every width,
//!    so none of this holds there: that sector is read by
//!    `topo::sector_shape::apex_corner_shape`, against a plane through the apex: the one
//!    the split cuts with, or the face a Boolean's vertex pierces.
//! 3. **If not definitely convex, where does it get subdivided?** Any
//!    interior direction with both sub-angles < 180° is valid. Definite
//!    reflex ⇒ `−normalize(â + b̂)` (the true bisector of the reflex
//!    span); the straight band (θ ≈ π, where `â + b̂` collapses) ⇒
//!    `n × b̂`, 90° into the interior. θ ≈ 0 or ≈ 2π between two
//!    DISTINCT edges is a spike — ill-conditioned, refused rather than
//!    guessed — unless the structure says which: at a slit's tip
//!    (`topo::sector_shape::SlitTips`) the face wraps round the two coincident edges, and
//!    `topo::sector_shape::corner_shape` reads the full turn and subdivides it at its three
//!    quarters (`topo::sector_shape::FullTurn`). At a cone's apex the window between them
//!    reads no width, and `topo::sector_shape::apex_corner_shape` reads the whole turn round
//!    the apex the same way. Both take the turn by one rule
//!    (`topo::sector_shape::reads_slit_tip_turn`). A wrap edge's tip is the one-edge form
//!    of the same structure: its face's loop walks the edge out and
//!    straight back, the face wraps the full turn round it, and
//!    `topo::sector_shape::SlitTips` holds it beside the slits' tips (the name covers
//!    both), so its corner reads the full turn by the same rule. Any
//!    other one-edge orbit (a strut vertex) is the legitimate
//!    full-circle sector and takes the 90° device.
//!
//! The three items above are the CONTRACT — what each rung decides and
//! what it returns. The **derivations** — why convex subdivision beats
//! the paper's complement-and-negate, why the wideness trilean has no
//! escalation cliff, why the duplicate entry is what makes dangling
//! null edges fall out of the generic run scan — stay in
//! `splitting::neighborhood`'s module docs, which own them, and are not
//! restated here.
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
//!   margins calls [`sector_shape`]; the guard row beside the corner
//!   readings (`topo::sector_shape`) reads every `.rs` file under
//!   `geom-brep/src` and `topo/src` for the quoted form of these three
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

use crate::OutwardNormal;
use geom_core::k_stats::{decide, decide_negative, decide_positive};
use geom_core::{
    Band, Decide, Indeterminate, Lever, Margin, Point3, Real, Sign, Vec3, is_finite_length,
    is_underflowed_length,
};

/// Rung 1's K name: the metering arm is positive.
///
/// This and its two siblings are PRIVATE to this module. They are rows
/// of the census in `docs/K-REPORT.md`, and no other file can IMPORT
/// one and re-implement the rung under it. Privacy stops the import,
/// not a bare string literal; what stops that is the guard row
/// `the_rungs_are_decided_in_one_place` in `topo::sector_shape`.
const SECTOR_ARM: &str = "sector_arm";

/// Rung 2's K name: the corner is convex (`sin θ` levered at the arm).
const SECTOR_REFLEX: &str = "sector_reflex";

/// Rung 3's K name: the straight/spike disambiguation (`cos θ` levered
/// at the arm, so a straight corner reads NEGATIVE), reached only when
/// rung 2 is not definitely signed. `topo::boolean::refusal_routes`' sized
/// recourse reads that sign.
const SECTOR_STRAIGHT: &str = "sector_straight";

/// What each rung decides, in the words a flip report states
/// (`topo::boolean::decision_words`).
pub fn rung_words(predicate: &str) -> Option<&'static str> {
    Some(match predicate {
        SECTOR_ARM => SectorRung::Arm.subject(),
        SECTOR_REFLEX => "whether a corner is convex or reflex",
        SECTOR_STRAIGHT => SectorRung::Straight { full_circle: false }.subject(),
        _ => return None,
    })
}

/// Which rung of [`sector_shape`] refused: the refusal's words and what
/// the rung passes on. Rung 2 (convexity) never refuses, since an
/// undecided reading of it falls through to rung 3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    any(test, feature = "test-support"),
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(SectorRungKind), derive(strum::EnumIter), doc(hidden))
)]
pub enum SectorRung {
    /// Rung 1: the metering arm is a positive length.
    Arm,
    /// Rung 3: the corner is straight, not a spike.
    Straight {
        /// The two bounds are one orbit half-edge (a strut vertex),
        /// where a reading of θ ≈ 0 or ≈ 2π is legitimate too.
        full_circle: bool,
    },
}

impl SectorRung {
    /// What the rung decides, as a clause with no colon or dash of its
    /// own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Arm => "whether a corner's edges are long enough to measure its angle over",
            Self::Straight { .. } => "whether a corner is straight or folds back on itself",
        }
    }
}

/// Why [`sector_shape`] refused.
///
/// Two causes, kept apart because their recourses are different
/// sentences: a bounding chord whose length is not a NUMBER is not a
/// coincidence at any tolerance and no tolerance lever reaches it,
/// while everything the rungs themselves refuse is a band question
/// carrying a classifier payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SectorFault {
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
    /// A rung refused or escalated.
    Rung {
        /// Which rung.
        rung: SectorRung,
        /// The rung's diagnostics, named by the rung.
        diag: Indeterminate,
    },
}

/// What the sector-shape rungs decided about one corner.
#[derive(Clone, Copy, Debug)]
pub struct SectorShape<T: Real> {
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
/// `dir_own` and `dir_next` are the bounds **scaled to their edges'
/// lengths** (a line's chord, a curved edge's extent from below) — their
/// norms are the arm's raw material and their directions the bounds. `normal` is the sector face's
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
/// [`Vec3::norm`]'s ~1e154 overflow band has an infinite norm;
/// hidden under the `min`, rung 1 sees a healthy arm and passes on a
/// length that is not a number.
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
/// same reason: a chord below `Vec3::norm`'s ~1e-162 underflow
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
/// named by the rung that produced it — the funnel's escalation, which
/// for a definite verdict the rung does not admit (non-positive arm; a
/// spike between distinct edges) is the gate's rejection from
/// [`decide_positive`] or [`decide_negative`], carrying the decided
/// margin, on the frame's log. Each lane
/// wraps this in its own error type — the two wrappings are the only
/// thing that was ever genuinely per-lane here.
pub fn sector_shape<T: Decide>(
    dir_own: Vec3<T>,
    dir_next: Vec3<T>,
    normal: OutwardNormal<T>,
    full_circle: bool,
    band: Band,
) -> Result<SectorShape<T>, SectorFault> {
    let arm = sector_arm(dir_own, dir_next, band)?;
    let (unit_own, unit_next) = (dir_own.normalize(), dir_next.normalize());
    // Wideness: sin θ = (b̂ × â)·n metered at the arm. Positive ⇒
    // convex; negative ⇒ reflex; zero-band ⇒ disambiguate by cosine
    // (for unit bounds sin and cos cannot both vanish, so the second
    // margin is definite whenever the first is not).
    let n = normal.vec();
    let reflex_margin = Margin::levered(unit_next.cross(unit_own).dot(n), metering_arm(arm));
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
            let straight_margin = Margin::levered(unit_own.dot(unit_next), metering_arm(arm));
            let rung = SectorRung::Straight { full_circle };
            if full_circle {
                // A one-edge orbit admits every definite reading: θ ≈ π,
                // or θ ≈ 0 / ≈ 2π, the legitimate full-circle sector.
                decide(SECTOR_STRAIGHT, straight_margin, band).map_err(refused(rung))?;
            } else {
                // Between two distinct edges only θ ≈ π (a negative
                // `cos θ`) is a corner; a spike has no interior
                // direction to guess.
                decide_negative(SECTOR_STRAIGHT, straight_margin, band).map_err(refused(rung))?;
            }
            // 90° into the interior, valid throughout the band.
            Some(n.cross(unit_next))
        }
    };
    Ok(SectorShape {
        arm,
        unit_own,
        unit_next,
        bisector,
    })
}

/// Rungs 0 and 1 of [`sector_shape`]: both bounds' lengths are numbers,
/// and the shorter, the corner's metering arm, is positive.
pub fn sector_arm<T: Decide>(
    dir_own: Vec3<T>,
    dir_next: Vec3<T>,
    band: Band,
) -> Result<T, SectorFault> {
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
    decide_positive(SECTOR_ARM, Margin::of(arm), band).map_err(refused(SectorRung::Arm))?;
    Ok(arm)
}

/// **A corner's metering arm** ([`sector_shape`]'s, the shorter of its
/// bounds' departure vectors' norms) as the lever its sectors read at.
pub fn metering_arm<T: Real>(arm: T) -> Lever<T> {
    // Unsourced lever: the norm of a departure vector, a curved bound's
    // read back from its extent times a unit tangent (or a line's chord),
    // which differs from that extent by rounding: the edges' `Lever::min`
    // would round differently.
    Lever::unsourced(arm)
}

/// [`sector_shape`] for the wedge square to an edge: the sector CCW
/// about the edge's own direction `axis` from `dir_next` to `dir_own`,
/// both square to it. The orientation is the edge's (its plus half runs
/// along `axis`), not a face's, so there is no sense bit to fold:
/// `axis` is the sector plane's normal as it stands.
///
/// # Errors
///
/// As [`sector_shape`].
pub fn edge_wedge_shape<T: Decide>(
    dir_own: Vec3<T>,
    dir_next: Vec3<T>,
    axis: Vec3<T>,
    band: Band,
) -> Result<SectorShape<T>, SectorFault> {
    sector_shape(
        dir_own,
        dir_next,
        OutwardNormal::from_chart(axis, true),
        false,
        band,
    )
}

/// A rung's escalation, as the fault naming that rung. The escalation
/// is the funnel's — an in-band margin, or a definite verdict the
/// rung's gate does not admit — so it is on the frame's log already.
fn refused(rung: SectorRung) -> impl FnOnce(Indeterminate) -> SectorFault {
    move |diag| SectorFault::Rung { rung, diag }
}

/// **A point's signed distance from a plane, in metres**: `q` against
/// the plane through `origin` with unit normal `normal`. The one reading
/// both lanes decide a real vertex's side with: the splitting lane's
/// vertex classes against the split plane, and the boolean's
/// line-bound side codes, where `q` is the edge's far vertex and
/// `origin` its base vertex. A point needs no lever: its distance is
/// what a margin measures.
pub fn plane_offset<T: Real>(origin: Point3<T>, normal: Vec3<T>, q: Point3<T>) -> T {
    (q - origin).dot(normal)
}

/// [`plane_offset`] with a first-order bound on its own rounding, its
/// inputs exact ([`geom_core::running`]): the value is bit-identical to
/// [`plane_offset`]'s, so a door that decides on one charges the other.
pub fn plane_offset_rounded<T: Real>(
    origin: Point3<T>,
    normal: Vec3<T>,
    q: Point3<T>,
) -> geom_core::running::Rounded<T> {
    use geom_core::running::{self, Rounded};
    let d = [(q.x, origin.x), (q.y, origin.y), (q.z, origin.z)]
        .map(|(a, b)| Rounded::exact(a) - Rounded::exact(b));
    running::dot(d, running::exact_vec(normal))
}

/// **A point's distance from the closed segment `a → b`, in metres**:
/// to the foot of the perpendicular where it falls on the segment, to
/// the nearer end otherwise (the foot parameter clamped by `min`/`max`,
/// no comparison). The segment has positive length.
pub fn point_segment_distance<T: Real>(a: Point3<T>, b: Point3<T>, q: Point3<T>) -> T {
    let (d, w) = (b - a, q - a);
    let t = (w.dot(d) / d.norm_squared()).max(T::zero()).min(T::one());
    (w - d * t).norm()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::{ErrorTextReading, Tol};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A rung refusal as `(rung, predicate, the margin's reading, the
    /// sign its gate rejected)`, so a row compares against the payload
    /// rather than against the code under test; `None` for any other
    /// fault.
    fn gate_refusal(
        fault: Option<SectorFault>,
    ) -> Option<(
        SectorRung,
        Option<&'static str>,
        ErrorTextReading,
        Option<Sign>,
    )> {
        match fault? {
            SectorFault::Rung { rung, diag } => Some((
                rung,
                diag.predicate,
                diag.margin.diagnostic_f64_for_error_text(),
                diag.margin.rejected_sign(),
            )),
            _ => None,
        }
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
        let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        sector_shape(own, next, n, full_circle, band())
    }

    /// A right-angle corner (θ = 90°, `sin θ` = +1) is convex: no
    /// subdivision, arm = the shorter bounding chord.
    #[test]
    fn convex_corner_needs_no_subdivision() {
        // own = +y (CCW-last), next = +x (CCW-first): the sector sweeps
        // +x → +y CCW around +z, so (b̂ × â)·n = (x̂ × ŷ)·ẑ = +1.
        let s = shape(Vec3::new(0.0, 3.0, 0.0), Vec3::new(2.0, 0.0, 0.0), false).unwrap();
        assert!(s.bisector.is_none());
        assert_eq!(s.arm, 2.0);
        assert_dir(s.unit_own, Vec3::new(0.0, 1.0, 0.0));
        assert_dir(s.unit_next, Vec3::new(1.0, 0.0, 0.0));
    }
    /// A definitely reflex corner (θ = 270°) subdivides at
    /// `−normalize(â + b̂)`, which points INTO the reflex span.
    #[test]
    fn reflex_corner_subdivides_at_the_reflex_bisector() {
        // own = +x, next = +y: (ŷ × x̂)·ẑ = −1 ⇒ reflex.
        let s = shape(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), false).unwrap();
        let b = s.bisector.expect("a reflex corner subdivides");
        assert!(bits_eq(b, -(Vec3::new(1.0, 1.0, 0.0).normalize())));
        // The sector sweeps +y → +x the LONG way; the bisector is
        // strictly interior to it, i.e. on the far side from (+1, +1).
        assert!(b.dot(Vec3::new(-1.0, -1.0, 0.0)) > 0.0);
    }
    /// A straight corner (θ = 180°, where `â + b̂` collapses) falls to
    /// the cosine rung and subdivides 90° into the interior.
    #[test]
    fn straight_corner_subdivides_ninety_degrees_in() {
        let s = shape(Vec3::new(1.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), false).unwrap();
        // n × b̂ = ẑ × (−x̂) = −ŷ.
        assert_dir(
            s.bisector.expect("a straight corner subdivides"),
            Vec3::new(0.0, -1.0, 0.0),
        );
    }
    /// A spike between two DISTINCT edges (θ ≈ 0) refuses, named by the
    /// cosine rung — it is ill-conditioned, not a full circle.
    #[test]
    fn spike_between_distinct_edges_refuses_named() {
        let e = shape(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), false)
            .expect_err("a spike has no valid interior direction");
        let SectorFault::Rung { diag: e, .. } = e else {
            panic!("a spike is a rung refusal, not a chord-length one: {e:?}");
        };
        assert_eq!(e.predicate, Some("sector_straight"));
        assert_eq!(
            (
                e.margin.diagnostic_f64_for_error_text(),
                e.margin.rejected_sign()
            ),
            (ErrorTextReading::Value(1.0), Some(Sign::Positive)),
            "the decided cos θ · arm"
        );
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
    /// Some("sector_arm") })` — the arm gate's refusal, which says the
    /// arm is non-positive and offers the
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
        assert_eq!(Vec3::new(0.0, tiny, 0.0).norm(), 0.0);
        assert_eq!(Vec3::new(0.0, tiny, 0.0).norm_witness(), tiny);
        for full_circle in [false, true] {
            for (own, next) in [
                (Vec3::new(0.0, tiny, 0.0), Vec3::new(0.0, tiny, 0.0)),
                (Vec3::new(0.0, 3.0, 0.0), Vec3::new(tiny, 0.0, 0.0)),
                (Vec3::new(tiny, 0.0, 0.0), Vec3::new(0.0, 3.0, 0.0)),
                (Vec3::new(tiny, tiny, tiny), Vec3::new(0.0, 3.0, 0.0)),
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
            gate_refusal(shape(Vec3::new(0.0, 1e-12, 0.0), Vec3::new(0.0, 3.0, 0.0), false).err()),
            Some((
                SectorRung::Arm,
                Some(SECTOR_ARM),
                ErrorTextReading::Value(1e-12),
                Some(Sign::Zero)
            ))
        );
        // And the zero chord is not an underflowed one: it has no
        // direction to recover, so its witness is zero too and the
        // predicate's two ratios are both poison.
        assert_eq!(
            gate_refusal(shape(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 3.0, 0.0), false).err()),
            Some((
                SectorRung::Arm,
                Some(SECTOR_ARM),
                ErrorTextReading::Value(0.0),
                Some(Sign::Zero)
            ))
        );
    }
    /// **Rung 0: a chord whose length is not a NUMBER.** A chord past
    /// `Vec3::norm`'s ~1e154 overflow band has an infinite norm,
    /// which is maximally definite to the arm rung.
    ///
    /// **Measured on this body at the merge base (`5fbdda206`), all
    /// EIGHT shapes** — four chord pairs × `full_circle`. Every line
    /// below was executed, not argued:
    ///
    /// | chords (own / next) | `full_circle` clear | `full_circle` set |
    /// |---|---|---|
    /// | `1e200` / `1e200` | rung 3 escalation | rung 3 escalation |
    /// | finite / `1e200` | rung 3 spike refusal | **`Ok`, bisector `(0,0,0)`** |
    /// | `1e200` / finite | rung 3 spike refusal | **`Ok`, bisector `(-1,0,0)`** |
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
    /// ESCALATIONS (a poisoned margin, `MarginKind::Invalid`), while
    /// the two single-overflow rows with `full_circle` clear are this
    /// straightness gate's spike refusal. Those two OUTCOMES were
    /// indistinguishable by value — both carried `MarginKind::Invalid`
    /// under the rung's name.
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
                (Vec3::new(big, 0.0, 0.0), Vec3::new(big, 0.0, 0.0)),
                (Vec3::new(0.0, 3.0, 0.0), Vec3::new(big, 0.0, 0.0)),
                (Vec3::new(big, 0.0, 0.0), Vec3::new(0.0, 3.0, 0.0)),
                (Vec3::new(f64::NAN, 0.0, 0.0), Vec3::new(0.0, 3.0, 0.0)),
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
        assert!(shape(Vec3::new(0.0, 3.0, 0.0), Vec3::new(2.0, 0.0, 0.0), false).is_ok());
    }
    /// The SAME reading on a one-edge orbit is the legitimate strut
    /// full-circle sector and subdivides instead of refusing — the row
    /// that separates the two, so the spike row above cannot be passing
    /// for the wrong reason.
    #[test]
    fn strut_full_circle_subdivides() {
        let s = shape(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), true).unwrap();
        assert_dir(
            s.bisector.expect("a strut vertex subdivides"),
            Vec3::new(0.0, 1.0, 0.0),
        );
    }
    /// A collapsed bounding chord fails the arm rung, named.
    #[test]
    fn degenerate_arm_refuses_named() {
        let e = shape(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), false)
            .expect_err("a collapsed chord cannot meter the corner");
        // Literal pins, for the reason spelled out on the spike row.
        let SectorFault::Rung { diag: e, .. } = e else {
            panic!("a collapsed chord has a finite length: {e:?}");
        };
        assert_eq!(e.predicate, Some("sector_arm"));
        assert_eq!(
            (
                e.margin.diagnostic_f64_for_error_text(),
                e.margin.rejected_sign()
            ),
            (ErrorTextReading::Value(0.0), Some(Sign::Zero)),
            "the decided arm"
        );
        assert_eq!(e.band, band());
    }
    #[test]
    fn a_gate_refusal_is_the_funnels_escalation() {
        for (own, next, rung) in [
            (
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                SECTOR_ARM,
            ),
            (
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                SECTOR_STRAIGHT,
            ),
        ] {
            let bracket = geom_core::k_stats::Bracket::open();
            let got = shape(own, next, false);
            let recorded = bracket.finish();
            let Err(SectorFault::Rung { diag, .. }) = got else {
                panic!("{rung}: a gate refusal, not {got:?}");
            };
            assert_eq!(diag.predicate, Some(rung), "{rung}: named by its rung");
            let logged: Vec<Indeterminate> =
                recorded.escalations.iter().map(|e| e.source).collect();
            assert_eq!(
                logged,
                vec![diag],
                "{rung}: the refusal is the logged escalation"
            );
        }
    }
}
