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
//! - **Transient, and recorded as such:** `crates/geom-brep/src/` was
//!   held by an in-flight lane (#639) that the S5 sector unit was told
//!   not to collide with. That is a scheduling fact with an expiry, not
//!   an architectural argument, and it is not a reason to keep the code
//!   here once #639 has landed.
//!
//! **Re-open trigger.** If the `sector_face` twins (still forked in
//! both lanes — the rest of S5) are ever unified and want the same
//! treatment, that is a real second consumer, and `geom-brep` is where
//! this placement should be re-opened. Absent that consumer, the crate
//! root holds: it is defensible on the merits above, not merely on the
//! transient reason.
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
//! lanes**. Until #652 they were six: `bool_sector_*` from the boolean
//! lane and `split_sector_*` from the splitting lane, handed in as a
//! `SectorPredicates` parameter so that #647's merge of the two bodies
//! could be K-neutral and the census question deferred rather than
//! taken by an implementation unit. Evan ruled the two populations one
//! (2026-08-19, #652), so the parameter is gone: there is nothing left
//! for it to vary.
//!
//! Why pooling rather than lane attribution. Since #647 these are
//! literally one implementation of one quantity, and the corpus shows
//! the fork was costing COVERAGE rather than buying attribution: every
//! one of the 64 `split_sector_reflex` samples in
//! `docs/k-report-data/m7-eps-1e-6.csv.gz` is exactly zero, so the
//! splitting lane's wideness name had no coverage of a definite
//! convex-or-reflex verdict at all, while `bool_sector_reflex` had 426
//! definite of 1880. Pooled, the rung is one population carrying those
//! 426 instead of two of which one is entirely degenerate. The
//! counter-precedent runs the other way and is real — `M3-LOG.md:264`
//! records PR #55's review MINOR-1 forcing two margins that shared one
//! K name to be SPLIT — but the in-tree precedent for sharing is in
//! this very module pair: `bool_planar_chord_spec` and `chord_spec`
//! deliberately share the one name `split_arc_window`.
//!
//! The pooled names are **new spellings, not either lane's old one**,
//! and that is deliberate: it makes the era of a K row self-evident.
//! A row reading `bool_sector_arm` or `split_sector_arm` anywhere in
//! `docs/k-report-data/` is pre-#652 data; a row reading `sector_arm`
//! is post. Those committed CSVs are dated snapshots and are left
//! exactly as the sweep wrote them — see `docs/K-REPORT.md`'s census
//! note (2026-08-19).
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

/// Rung 1's K name: the metering arm is positive.
///
/// This and its two siblings are PRIVATE to this module. They are rows
/// of the census in `docs/K-REPORT.md`, and the only way to emit one is
/// to call [`sector_shape`] — a lane cannot import the name and
/// re-implement the rung under it.
const SECTOR_ARM: &str = "sector_arm";

/// Rung 2's K name: the corner is convex (`sin θ` levered at the arm).
const SECTOR_REFLEX: &str = "sector_reflex";

/// Rung 3's K name: the straight/spike disambiguation (`cos θ` levered
/// at the arm), reached only when rung 2 is not definitely signed.
const SECTOR_STRAIGHT: &str = "sector_straight";

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
/// escalation passed through unchanged, or a [`MarginDiag::Invalid`]
/// diagnostic when a definite verdict is one this predicate does not
/// admit (non-positive arm; a spike between distinct edges). Each lane
/// wraps this in its own error type — the two wrappings are the only
/// thing that was ever genuinely per-lane here.
pub(crate) fn sector_shape<T: Decide>(
    dir_own: Vec3<T>,
    dir_next: Vec3<T>,
    normal: Vec3<T>,
    full_circle: bool,
    band: Band,
) -> Result<SectorShape<T>, Indeterminate> {
    let arm = dir_own.norm().min(dir_next.norm());
    match decide(SECTOR_ARM, Margin::of(arm), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(invalid(band, SECTOR_ARM)),
        Err(diag) => return Err(diag),
    }
    let (unit_own, unit_next) = (dir_own.normalize(), dir_next.normalize());
    // Wideness: sin θ = (b̂ × â)·n metered at the arm. Positive ⇒
    // convex; negative ⇒ reflex; zero-band ⇒ disambiguate by cosine
    // (for unit bounds sin and cos cannot both vanish, so the second
    // margin is definite whenever the first is not).
    let reflex_margin = Margin::levered(unit_next.cross(unit_own).dot(normal), arm);
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
                Ok(Sign::Negative) => Some(normal.cross(unit_next)),
                // θ ≈ 0 or ≈ 2π on a one-edge orbit: the legitimate
                // full-circle sector, same device.
                Ok(Sign::Positive | Sign::Zero) if full_circle => Some(normal.cross(unit_next)),
                // A spike corner between two distinct edges: refuse,
                // never guess an interior direction.
                Ok(Sign::Positive | Sign::Zero) => return Err(invalid(band, SECTOR_STRAIGHT)),
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
///
/// **This is a THIRD byte-identical copy in `topo/src`**, and naming
/// that is the point of this comment. `census.rs:377` and
/// `boolean/contain.rs:155` already carry the same four-line body;
/// `boolean/sectors.rs`'s `invalid_escalation` is a fourth spelling
/// that wraps the same value into `BooleanError`; and roughly sixty
/// further sites across four crates construct `Indeterminate {
/// margin: MarginDiag::Invalid, .. }` inline. The home is
/// `impl Indeterminate` in `geom-core/src/predicate.rs:724`. Unifying
/// it is a public-API addition plus a four-crate sweep — deliberately
/// not folded into the S5 sector unit, and recorded here so the next
/// pass finds the home rather than the method (smell scan C12).
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
    ) -> Result<SectorShape<f64>, Indeterminate> {
        sector_shape(own, next, v(0.0, 0.0, 1.0), full_circle, band())
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
        assert_eq!(e.predicate, Some("sector_straight"));
        assert_eq!(e.margin, MarginDiag::Invalid);
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
        assert_eq!(e.predicate, Some("sector_arm"));
        assert_eq!(e.margin, MarginDiag::Invalid);
    }

    /// **The anti-re-fork row.** The three sector-shape K names are
    /// decided HERE and nowhere else in this crate, and the six
    /// per-lane names they replaced (#652) are decided NOWHERE at all:
    /// no file under `topo/src` may spell either set as a string
    /// literal. Re-forking the rungs means re-introducing a
    /// `decide("…_sector_arm", …)` somewhere, and that is exactly what
    /// this reads for. (It looks for the QUOTED form, so prose that
    /// mentions a predicate in backticks — including this module's own
    /// docs, which name the retired spellings — is untouched.)
    ///
    /// The retired names are BUILT from the pooled ones rather than
    /// written out, for two reasons: the check can then cover this file
    /// too (a literal roster would be its own counter-example), and the
    /// two sets cannot drift apart if a rung is ever renamed again.
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
    /// **What this still does not cover** (stated plainly rather than
    /// as one careful caveat that implies the rest is covered):
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
    ///
    /// The third residue #647 shipped — a lane re-implementing the
    /// rungs while IMPORTING the names, `decide(BOOL_SECTOR_PREDICATES.arm,
    /// …)`, which no string search can see — is **closed**: the three
    /// consts are private to this module, so no other file can name
    /// them at all.
    #[test]
    fn the_rungs_are_decided_in_one_place() {
        let pooled = [SECTOR_ARM, SECTOR_REFLEX, SECTOR_STRAIGHT];
        // The retired six, each WITH its surrounding quotes — assembled
        // rather than spelled, so this file is subject to the check like
        // any other. (Writing them out here would make this comment the
        // guard's own first counter-example, which it briefly was.)
        let retired: Vec<String> = ["bool", "split"]
            .iter()
            .flat_map(|lane| pooled.iter().map(move |rung| format!("\"{lane}_{rung}\"")))
            .collect();
        let src = src_root();
        let home = src.join("sector_shape.rs");
        let mut files = Vec::new();
        collect_rs(&src, &mut files);
        // The walk must be reading what it thinks it is: a broken or
        // empty walk would otherwise pass by finding nothing.
        assert!(
            files.contains(&home) && files.len() > 20,
            "the walk of {src:?} found {} file(s) and {}the home module \
             — it is not reading topo/src",
            files.len(),
            if files.contains(&home) { "" } else { "not " }
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
                     have been re-forked out of sector_shape.rs (smell scan S5). Call \
                     `sector_shape` instead.",
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

    /// This crate's `src/`, resolved for both ways the suite runs: a
    /// plain `cargo test` (where the baked-in `CARGO_MANIFEST_DIR` is
    /// the tree that is here) and a nextest ARCHIVE replayed on a
    /// different runner (where that absolute path need not exist, but
    /// `--workspace-remap` has pointed the per-test cwd at the crate
    /// root).
    fn src_root() -> std::path::PathBuf {
        let baked = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        if baked.is_dir() {
            return baked;
        }
        let cwd = std::env::current_dir()
            .expect("a working directory")
            .join("src");
        assert!(cwd.is_dir(), "neither {baked:?} nor {cwd:?} is topo's src/");
        cwd
    }

    /// Every `.rs` file under `dir`, recursively.
    fn collect_rs(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).expect("a readable source directory") {
            let path = entry.expect("a readable directory entry").path();
            if path.is_dir() {
                collect_rs(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
}
