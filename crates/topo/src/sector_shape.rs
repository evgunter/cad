//! The vertex-neighborhood **sector-shape** predicates: one metering
//! arm, one wideness verdict, one subdivision direction — shared by the
//! two lanes that ask this question.
//!
//! # What the question is
//!
//! Both the splitting lane ([`crate::splitting::neighborhood`]) and the
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
//! Full derivations — why convex subdivision beats the paper's
//! complement-and-negate, why the wideness trilean has no escalation
//! cliff, why the duplicate entry is what makes dangling null edges fall
//! out of the generic run scan — stay in
//! [`crate::splitting::neighborhood`]'s module docs, which own them.
//!
//! # Why the code is here and not in either lane
//!
//! Until S5's first fix these three rungs existed **twice**, once per
//! lane, dimensionally identical line-for-line under the correspondence
//! `dir_a ↔ dir_end`, `dir_b ↔ dir_start` — the same `min` of the same
//! two chord norms, the same `Margin::levered(sin, arm)`, the same
//! `.dot()` fallback, the same bisectors, the same spike guard. This
//! module is a **top-level sibling** of `boolean/` and `splitting/`
//! precisely so neither half hosts the other's core: both lanes already
//! depend on this scope (`crate::body`, `crate::entity`,
//! `crate::validate::decide`), so sharing here adds no dependency edge
//! between the halves and no public API.
//!
//! # Two K names for one margin
//!
//! The predicate NAMES stay per-lane and bit-for-bit unchanged —
//! `docs/K-REPORT.md` is an append-only census and merging names is a
//! schema break, not this module's call. The lane hands its
//! [`SectorPredicates`] in; the funnel
//! ([`crate::validate::decide`] → [`geom_core::k_stats::decide`]) takes
//! the name as a parameter, so the emitted stream is exactly what each
//! lane emitted before. The same device already carries
//! `bool_planar_chord_spec` and `chord_spec` under one shared name.
//! Whether the two populations SHOULD be one is a live open question,
//! recorded with its evidence in the PR that created this module.
//!
//! # Naming
//!
//! The two lanes name the bounds oppositely (`dir_a`/`dir_b` in
//! splitting, `dir_end`/`dir_start` in the boolean). This module uses
//! neither: `own` is the sector's own orbit chord (the CCW-**last**
//! bound, splitting's `dir_a`, the boolean's `dir_end`) and `next` is
//! the next orbit chord (the CCW-**first** bound, splitting's `dir_b`,
//! the boolean's `dir_start`).

use geom_core::{Band, Decide, Indeterminate, Margin, MarginDiag, Real, Sign, Vec3};

use crate::validate::decide;

/// The three K predicate names one lane spells for the sector-shape
/// question — the rungs of [`sector_shape`], in the order it climbs
/// them.
///
/// Names are per-lane and load-bearing: they are rows of the
/// append-only census in `docs/K-REPORT.md`. A new lane must add new
/// names, never reuse another lane's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SectorPredicates {
    /// Rung 1: the metering arm is positive.
    pub arm: &'static str,
    /// Rung 2: the corner is convex (`sin θ` levered at the arm).
    pub reflex: &'static str,
    /// Rung 3: the straight/spike disambiguation (`cos θ` levered at
    /// the arm), reached only when rung 2 is not definitely signed.
    pub straight: &'static str,
}

/// The boolean lane's names (`boolean/sectors.rs`).
pub(crate) const BOOL_SECTOR_PREDICATES: SectorPredicates = SectorPredicates {
    arm: "bool_sector_arm",
    reflex: "bool_sector_reflex",
    straight: "bool_sector_straight",
};

/// The splitting lane's names (`splitting/neighborhood.rs`).
pub(crate) const SPLIT_SECTOR_PREDICATES: SectorPredicates = SectorPredicates {
    arm: "split_sector_arm",
    reflex: "split_sector_reflex",
    straight: "split_sector_straight",
};

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
/// their directions the bounds. `normal` is the sector face's OUTWARD
/// normal at the base vertex (chart normal × `sense_sign`, applied once
/// by each lane's `sector_face`; this function must not re-apply it —
/// the bounds come from the STORED orbit order, which `revert` reverses
/// in the same breath as the sense bit, so a second factor would
/// double-count and read every convex corner as reflex).
/// `full_circle` says the two bounds are the SAME orbit half-edge (a
/// strut vertex), which is what makes a θ ≈ 0 / ≈ 2π reading
/// legitimate rather than a spike.
///
/// # Errors
///
/// [`Indeterminate`], named by the rung that produced it: a `decide`
/// escalation verbatim, or a [`MarginDiag::Invalid`] diagnostic when a
/// definite verdict is one this predicate does not admit (non-positive
/// arm; a spike between distinct edges). Each lane wraps this in its
/// own error type — the two wrappings are the only thing that was ever
/// genuinely per-lane here.
pub(crate) fn sector_shape<T: Decide>(
    dir_own: Vec3<T>,
    dir_next: Vec3<T>,
    normal: Vec3<T>,
    full_circle: bool,
    names: SectorPredicates,
    band: Band,
) -> Result<SectorShape<T>, Indeterminate> {
    let arm = dir_own.norm().min(dir_next.norm());
    match decide(names.arm, Margin::of(arm), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(invalid(band, names.arm)),
        Err(diag) => return Err(diag),
    }
    let (unit_own, unit_next) = (dir_own.normalize(), dir_next.normalize());
    // Wideness: sin θ = (b̂ × â)·n metered at the arm. Positive ⇒
    // convex; negative ⇒ reflex; zero-band ⇒ disambiguate by cosine
    // (for unit bounds sin and cos cannot both vanish, so the second
    // margin is definite whenever the first is not).
    let reflex_margin = Margin::levered(unit_next.cross(unit_own).dot(normal), arm);
    let bisector = match decide(names.reflex, reflex_margin, band) {
        Ok(Sign::Positive) => None,
        // Definite reflex, θ ∈ (π, 2π). `unit_own + unit_next` is the
        // splitting lane's spelling and `unit_next + unit_own` the
        // boolean's; IEEE addition is commutative, so this is
        // bit-identical to both. (The collapse â + b̂ → 0 happens only
        // at θ → π, which lands in the Zero band below, never here.)
        Ok(Sign::Negative) => Some(-((unit_own + unit_next).normalize())),
        Ok(Sign::Zero) | Err(_) => {
            // Likewise `unit_own.dot(unit_next)` vs the boolean's
            // `unit_next.dot(unit_own)`: componentwise products, same
            // summation order, bit-identical.
            let straight_margin = Margin::levered(unit_own.dot(unit_next), arm);
            match decide(names.straight, straight_margin, band) {
                // θ ≈ π: 90° into the interior is valid throughout the
                // band.
                Ok(Sign::Negative) => Some(normal.cross(unit_next)),
                // θ ≈ 0 or ≈ 2π on a one-edge orbit: the legitimate
                // full-circle sector, same device.
                Ok(Sign::Positive | Sign::Zero) if full_circle => Some(normal.cross(unit_next)),
                // A spike corner between two distinct edges: refuse,
                // never guess an interior direction.
                Ok(Sign::Positive | Sign::Zero) => return Err(invalid(band, names.straight)),
                Err(diag) => return Err(diag),
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
fn invalid(band: Band, predicate: &'static str) -> Indeterminate {
    Indeterminate {
        margin: MarginDiag::Invalid,
        band,
        predicate: Some(predicate),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    fn band() -> Band {
        Band::linear().unwrap()
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
        names: SectorPredicates,
    ) -> Result<SectorShape<f64>, Indeterminate> {
        sector_shape(own, next, v(0.0, 0.0, 1.0), full_circle, names, band())
    }

    /// A right-angle corner (θ = 90°, `sin θ` = +1) is convex: no
    /// subdivision, arm = the shorter bounding chord.
    #[test]
    fn convex_corner_needs_no_subdivision() {
        // own = +y (CCW-last), next = +x (CCW-first): the sector sweeps
        // +x → +y CCW around +z, so (b̂ × â)·n = (x̂ × ŷ)·ẑ = +1.
        let s = shape(
            v(0.0, 3.0, 0.0),
            v(2.0, 0.0, 0.0),
            false,
            BOOL_SECTOR_PREDICATES,
        )
        .unwrap();
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
        let s = shape(
            v(1.0, 0.0, 0.0),
            v(0.0, 1.0, 0.0),
            false,
            SPLIT_SECTOR_PREDICATES,
        )
        .unwrap();
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
        let s = shape(
            v(1.0, 0.0, 0.0),
            v(-1.0, 0.0, 0.0),
            false,
            BOOL_SECTOR_PREDICATES,
        )
        .unwrap();
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
        let e = shape(
            v(1.0, 0.0, 0.0),
            v(1.0, 0.0, 0.0),
            false,
            SPLIT_SECTOR_PREDICATES,
        )
        .expect_err("a spike has no valid interior direction");
        assert_eq!(e.predicate, Some("split_sector_straight"));
        assert_eq!(e.margin, MarginDiag::Invalid);
    }

    /// The SAME reading on a one-edge orbit is the legitimate strut
    /// full-circle sector and subdivides instead of refusing — the row
    /// that separates the two, so the spike row above cannot be passing
    /// for the wrong reason.
    #[test]
    fn strut_full_circle_subdivides() {
        let s = shape(
            v(1.0, 0.0, 0.0),
            v(1.0, 0.0, 0.0),
            true,
            SPLIT_SECTOR_PREDICATES,
        )
        .unwrap();
        assert_dir(
            s.bisector.expect("a strut vertex subdivides"),
            v(0.0, 1.0, 0.0),
        );
    }

    /// A collapsed bounding chord fails the arm rung, named.
    #[test]
    fn degenerate_arm_refuses_named() {
        let e = shape(
            v(1.0, 0.0, 0.0),
            v(0.0, 0.0, 0.0),
            false,
            BOOL_SECTOR_PREDICATES,
        )
        .expect_err("a collapsed chord cannot meter the corner");
        assert_eq!(e.predicate, Some("bool_sector_arm"));
        assert_eq!(e.margin, MarginDiag::Invalid);
    }

    /// **The anti-fork row.** One geometric question, two K names: the
    /// two lanes' name sets must differ ONLY in the strings they emit.
    /// Every input shape — convex, reflex, straight, spike, strut,
    /// degenerate arm — runs under both name sets and the results must
    /// be BIT-identical, with escalations differing only in
    /// `predicate`.
    ///
    /// It goes red the moment anything inside [`sector_shape`] branches
    /// on WHICH lane is asking (a `names.arm == "…"` test, a lane flag,
    /// a per-lane band or arm) — the shape a re-fork takes from the
    /// inside. The complementary guard against a re-fork from the
    /// OUTSIDE is `the_rungs_are_decided_in_one_place`.
    #[test]
    fn both_lanes_decide_the_same_shape() {
        let cases: [(Vec3<f64>, Vec3<f64>, bool); 6] = [
            (v(0.0, 3.0, 0.0), v(2.0, 0.0, 0.0), false),
            (v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), false),
            (v(1.0, 0.0, 0.0), v(-1.0, 0.0, 0.0), false),
            (v(1.0, 0.0, 0.0), v(1.0, 0.0, 0.0), false),
            (v(1.0, 0.0, 0.0), v(1.0, 0.0, 0.0), true),
            (v(1.0, 0.0, 0.0), v(0.0, 0.0, 0.0), false),
        ];
        for (own, next, full_circle) in cases {
            match (
                shape(own, next, full_circle, BOOL_SECTOR_PREDICATES),
                shape(own, next, full_circle, SPLIT_SECTOR_PREDICATES),
            ) {
                (Ok(b), Ok(s)) => {
                    assert_eq!(b.arm.to_bits(), s.arm.to_bits());
                    assert!(bits_eq(b.unit_own, s.unit_own));
                    assert!(bits_eq(b.unit_next, s.unit_next));
                    match (b.bisector, s.bisector) {
                        (None, None) => {}
                        (Some(bb), Some(sb)) => assert!(bits_eq(bb, sb)),
                        other => panic!("wideness diverged between the lanes: {other:?}"),
                    }
                }
                (Err(b), Err(s)) => {
                    assert_eq!(b.margin, s.margin);
                    assert_eq!(b.band, s.band);
                    // The names are the ONE sanctioned difference.
                    assert_ne!(b.predicate, s.predicate);
                }
                other => panic!("outcome kind diverged between the lanes: {other:?}"),
            }
        }
    }

    /// **The anti-re-fork row.** The six sector-shape K names are
    /// decided HERE and nowhere else: neither lane's neighborhood file
    /// may spell one as a string literal again. Re-forking the rungs
    /// means re-introducing a `decide("…_sector_arm", …)` into one of
    /// those files, and that is exactly what this reads for. (It looks
    /// for the QUOTED form, so prose that mentions a predicate in
    /// backticks is untouched.)
    ///
    /// It goes red on a re-fork that keeps the names — the case where
    /// the K stream still looks plausible, so nothing else would catch
    /// it. A re-fork under FRESH names is out of its reach by
    /// construction; that one surfaces as new rows in the
    /// `docs/K-REPORT.md` census, which is the mechanism that already
    /// exists for it.
    #[test]
    fn the_rungs_are_decided_in_one_place() {
        let names = [
            BOOL_SECTOR_PREDICATES.arm,
            BOOL_SECTOR_PREDICATES.reflex,
            BOOL_SECTOR_PREDICATES.straight,
            SPLIT_SECTOR_PREDICATES.arm,
            SPLIT_SECTOR_PREDICATES.reflex,
            SPLIT_SECTOR_PREDICATES.straight,
        ];
        for (file, src) in [
            ("boolean/sectors.rs", include_str!("boolean/sectors.rs")),
            (
                "splitting/neighborhood.rs",
                include_str!("splitting/neighborhood.rs"),
            ),
        ] {
            for name in names {
                assert!(
                    !src.contains(&format!("\"{name}\"")),
                    "{file} names the sector-shape predicate `{name}` again — the rungs \
                     have been re-forked out of sector_shape.rs (smell scan S5). Call \
                     `sector_shape` with the lane's `SectorPredicates` instead."
                );
            }
        }
        // The guard earns its line only if the strings it looks for are
        // reachable at all: they must all be spelled HERE.
        let here = include_str!("sector_shape.rs");
        for name in names {
            assert!(
                here.contains(&format!("\"{name}\"")),
                "`{name}` is not spelled in this module"
            );
        }
    }
}
