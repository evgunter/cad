//! §10.3 skinned (lofted) and §10.4 swept NURBS surfaces — the
//! DEFINITIONAL geometry behind the `Loft` and `Sweep` feature nodes
//! (M5 PR 10; C11, The NURBS Book ch. 10).
//!
//! # Q8, stated where the geometry is made
//!
//! **The produced NURBS *is* the definition.** The recipe (the
//! sections, the path, the degrees) is PROVENANCE: it says how the
//! surface was chosen, not what it approximates. There is no residual
//! obligation to some "true" swept locus, no approximating-surface
//! machinery, and no tolerance attached to the construction itself.
//! Only DERIVED items — intersections with these walls, pcurves of
//! non-iso edges — carry certificates, because only they make a claim
//! about something other than themselves.
//!
//! That is why §10.4's general sweep is honestly implemented as
//! *instantiate and skin*: placing rigid copies of the profile along
//! the path and skinning them does not approximate a swept surface,
//! it DEFINES one. The Book presents the same construction; we simply
//! decline to pretend the result is anything but itself.
//!
//! # Placement in this crate (M5 PR 10, numbered deviation 1)
//!
//! Not "constructions live here" — `geom` builds NURBS payloads too
//! (`geom::curves::fit`, which this module calls). The binding reason
//! is the layering: skinning reads a `profile::ValidatedProfile`
//! through a `SketchPlane` and a `geom_brep::SketchSegment`, so it
//! depends on two layers ABOVE `geom` and cannot live below them. Of
//! the crates that sit above both, `sweep` is the one whose subject
//! this is — §10.3/§10.4 are sweep operations by name. The import list
//! below is the check: nothing in it may be pushed down.
//!
//! The produced payload is `geom::NurbsSurface`, so nothing about the
//! surface type moved.
//!
//! # Structure selection is f64 (C6/D9)
//!
//! Degrees, knot vectors, sub-arc counts, and the interpolation
//! parameters are chosen in `f64`, deterministically: same sections in,
//! same knots and same control bits out. The produced surface then
//! evaluates generically over [`geom_core::Real`] (the substrate's
//! rule) — [`lift_surface`] carries a chosen structure to any scalar.
//! No topology decision is made here, so nothing routes through
//! `k_stats`; the raw `f64` comparisons below are structure selection
//! under C6.

use std::sync::Arc;

use geom::NurbsCurve3;
use geom::NurbsSurface;
use geom::curves::fit::{FitError, interpolate_columns};
use geom_brep::SketchSegment;
use geom_core::Tol;
use geom_core::spline::{KnotAlgebraError, KnotVector, SplineError};
use geom_core::{Affine3, COINCIDENCE_RECOURSE, Point2, Point3, Real, Vec3};
use profile::{Profile, ProfileError, ProfileLoop, SketchPlane, ValidatedProfile};

/// The quarter-turn ceiling on one rational-quadratic arc span: every
/// sub-arc of a converted profile arc is at most this wide, so the
/// tangent-intersection control point stays finite and well
/// conditioned (`cos(Δ/2) ≥ cos(π/4)`).
const MAX_SUB_ARC: f64 = core::f64::consts::FRAC_PI_2;

/// Typed refusal of the §10.3/§10.4 constructions (closed enum, D4 ¶3).
///
/// Every arm is a STRUCTURAL statement about the inputs — section
/// counts, loop counts, degrees, weights, solvability. Structure
/// selection is exact-`f64` (C6), so NO arm here has an in-band twin
/// and the two-tolerance discipline does not apply within this module;
/// [`SkinError::DegenerateSection`] documents where the banded half of
/// its user situation actually lives and carries that door's recourse.
#[derive(Clone, Debug, PartialEq)]
pub enum SkinError {
    /// Fewer sections than a skin needs (≥ 2 always; ≥ `degree + 1`
    /// for the requested v-degree).
    TooFewSections {
        /// Sections supplied.
        have: usize,
        /// Sections required.
        need: usize,
    },
    /// The sections do not describe the same thing: their loop counts,
    /// or one loop's segment counts, differ. A skin matches segment
    /// `j` of every section to segment `j` of every other; there is no
    /// honest way to guess a correspondence that was not given.
    SectionShapeMismatch {
        /// The section that disagrees with section 0.
        section: usize,
        /// What section 0 had.
        expected: usize,
        /// What this section had.
        found: usize,
        /// Which count disagreed (`"loops"` or `"segments"`).
        what: &'static str,
    },
    /// A section failed the profile crate's validation door — the
    /// same gate every extruded/revolved profile passes (one
    /// profile vocabulary for all four body ops, so a section that
    /// would not extrude does not skin either). Open chains are not a
    /// case this door can meet: [`ProfileLoop`] closes by
    /// construction, so the former open/closed-mixed refusal has
    /// nothing left to refuse and the mixed skin is unrepresentable.
    SectionProfile {
        /// The offending section.
        section: usize,
        /// The profile door's typed refusal.
        source: ProfileError,
    },
    /// A section curve does not live on the unit parameter domain the
    /// compatibility pass requires (`[0, 1]`, clamped).
    DomainNotUnit {
        /// The offending section.
        section: usize,
        /// Its domain.
        domain: (f64, f64),
    },
    /// A section, or the path, is degenerate: a zero-length chord, a
    /// zero-radius arc, or two consecutive sections that coincide
    /// (chord-length parameterization has no step there).
    ///
    /// **Where the in-band twin lives** (M5 PR 10 fix pass, review
    /// MIN-1): not here. This module makes no banded decisions — every
    /// comparison in it is exact-`f64` structure selection (C6), so
    /// there is no `Escalated` arm to pair with, and inventing one
    /// would advertise a refusal nothing can produce. The banded half
    /// of the user's situation sits one layer UP, in
    /// `profile::validate`'s `vertex_separation` /
    /// `segment_straightness` / `arc_diameter_clearance` predicates,
    /// which every section passes before it reaches this module. This
    /// arm catches the exactly-zero residue those doors let through by
    /// construction, and it names the SAME recourse they do, so both
    /// halves of one user situation still read alike across the layer
    /// boundary (D4 ¶1's intent, honoured where the decision actually
    /// is).
    DegenerateSection {
        /// Where it sits.
        section: usize,
        /// What was degenerate.
        what: &'static str,
    },
    /// The requested v-degree is 0, or ≥ the section count.
    BadDegree {
        /// The requested degree.
        degree: usize,
        /// The sections available.
        sections: usize,
    },
    /// The path has no usable tangent at a station, so no rigid frame
    /// carries the profile through it.
    ///
    /// **What this actually catches** (M5 PR 10 fix pass, review
    /// MIN-2): a VANISHING tangent — `|C'(t)|` not finite and positive
    /// — plus the exactly-anti-parallel case, where the minimal
    /// rotation is not unique because every axis perpendicular to the
    /// tangent turns one into the other. The second case is a `f64`
    /// knife edge and essentially unreachable: an exact half-turn path
    /// evaluates `|t₀ × t₁| ≈ 1.2e-16 > 0`, so the frame is BUILT from
    /// a numerically ill-conditioned axis rather than refused (pinned,
    /// executed, in `tests/review_m5_pr10.rs`). Under Q8 that surface
    /// is still the definition — it is whatever that frame produced,
    /// not an approximation of something else — but it is not the
    /// surface a reader of "reversing paths refuse" would expect, so
    /// the claim is stated as it is rather than as it reads best. See
    /// [`sweep_geometry`]'s C6 note for why no band is asserted.
    PathTangentReversal {
        /// The station where the frame failed.
        station: usize,
    },
    /// The v-direction interpolation refused.
    Fit(FitError),
    /// A knot-algebra operation (degree elevation, refinement) refused.
    KnotAlgebra(KnotAlgebraError),
    /// The produced spline structure refused validated construction —
    /// notably a non-positive interpolated weight, which would break
    /// the convex-hull invariant every C9 bound stands on.
    Structure(SplineError),
}

impl core::fmt::Display for SkinError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooFewSections { have, need } => {
                write!(f, "skin: {have} sections, need at least {need}")
            }
            Self::SectionShapeMismatch {
                section,
                expected,
                found,
                what,
            } => write!(
                f,
                "skin: section {section} has {found} {what}, section 0 has {expected} — a \
                 skin matches like to like by index; supply sections with the same shape"
            ),
            Self::SectionProfile { section, source } => {
                write!(
                    f,
                    "skin: section {section} failed profile validation: {source}"
                )
            }
            Self::DomainNotUnit { section, domain } => write!(
                f,
                "skin: section {section} lives on [{}, {}], not the unit domain the \
                 compatibility pass requires",
                domain.0, domain.1
            ),
            Self::DegenerateSection { section, what } => write!(
                f,
                "skin: section {section} is degenerate ({what}) — {COINCIDENCE_RECOURSE}"
            ),
            Self::BadDegree { degree, sections } => write!(
                f,
                "skin: v-degree {degree} is not usable for {sections} sections (need \
                 1 ≤ degree ≤ sections − 1)"
            ),
            Self::PathTangentReversal { station } => write!(
                f,
                "skin: the path's tangent reverses or vanishes at station {station} — no \
                 rigid frame carries the profile through it"
            ),
            Self::Fit(e) => write!(f, "skin: {e}"),
            Self::KnotAlgebra(e) => write!(f, "skin: {e}"),
            Self::Structure(e) => write!(f, "skin: {e}"),
        }
    }
}

impl core::error::Error for SkinError {}

impl From<KnotAlgebraError> for SkinError {
    fn from(e: KnotAlgebraError) -> Self {
        Self::KnotAlgebra(e)
    }
}

impl From<SplineError> for SkinError {
    fn from(e: SplineError) -> Self {
        Self::Structure(e)
    }
}

impl From<FitError> for SkinError {
    fn from(e: FitError) -> Self {
        Self::Fit(e)
    }
}

// ---------------------------------------------------------------------
// Analytic → NURBS: the section curves
// ---------------------------------------------------------------------

/// One profile segment as a NURBS curve in WORLD space, exactly.
///
/// - **Line** → degree 1, two control points, unit weights: the chord,
///   with the segment's own affine parameter.
/// - **Arc** → the standard rational-quadratic circular representation
///   (Book §7.5 / A7.1), split into `⌈|θ| / (π/2)⌉` equal sub-arcs so
///   every tangent-intersection control point stays finite and the
///   interior weights `cos(Δ/2)` stay ≥ `cos(π/4)`. Exact in ℝ: the
///   locus IS the carrier circle, not a fit of it.
///
/// The 2-D control points are mapped through `place` — an affine map,
/// under which a NURBS is exactly a NURBS with the same weights and
/// knots (control points transform, nothing else moves).
///
/// # Errors
///
/// [`SkinError::DegenerateSection`] for a zero-length chord, a
/// non-finite bulge, or an arc whose derived radius is not finite and
/// positive; [`SkinError::Structure`] if validated construction
/// refuses.
// `!(x > 0)` and `!(a < b)` are deliberate NaN-catching (the
// geom-core::spline::algebra note): a poisoned coordinate must take
// the refusal arm, not slip through a negated comparison.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn segment_curve(
    section: usize,
    seg: SketchSegment<f64>,
    place: Affine3<f64>,
) -> Result<NurbsCurve3<f64>, SkinError> {
    let degenerate = |what: &'static str| SkinError::DegenerateSection { section, what };
    let world = |p: Point2<f64>| place.transform_point(Point3::new(p.x, p.y, 0.0));
    match seg {
        SketchSegment::Line { a, b } => {
            if !(a.distance(b) > 0.0) {
                return Err(degenerate("zero-length chord"));
            }
            let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1)?;
            Ok(NurbsCurve3::new(
                knots,
                vec![world(a), world(b)],
                vec![1.0, 1.0],
            )?)
        }
        SketchSegment::Arc { a, b, bulge } => {
            let len = a.distance(b);
            if !(len > 0.0) {
                return Err(degenerate("zero-length chord"));
            }
            if !bulge.is_finite() || bulge == 0.0 {
                return Err(degenerate("zero or non-finite bulge"));
            }
            // The profile crate's ratified bulge closed forms, verbatim
            // (`profile::seg::build_seg`): the chord's LEFT normal, the
            // apothem, and the signed radius.
            let ux = (b.x - a.x) / len;
            let uy = (b.y - a.y) / len;
            let (nx, ny) = (-uy, ux);
            let b2 = bulge.powi(2);
            let four_bulge = 4.0 * bulge;
            let apothem = len * (1.0 - b2) / four_bulge;
            let radius = (len * (1.0 + b2) / four_bulge).abs();
            if !(radius > 0.0) || !radius.is_finite() {
                return Err(degenerate("arc radius is not finite and positive"));
            }
            let center = Point2::new(
                0.5f64.mul_add(b.x - a.x, a.x) + nx * apothem,
                0.5f64.mul_add(b.y - a.y, a.y) + ny * apothem,
            );
            // θ = 4·atan(bulge), signed (the sanctioned bulge
            // re-inspection — never endpoint `atan2`).
            let theta = 4.0 * bulge.atan();
            let start = (a.y - center.y).atan2(a.x - center.x);
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let m = ((theta.abs() / MAX_SUB_ARC).ceil() as usize).max(1);
            #[allow(clippy::cast_precision_loss)]
            let steps = m as f64;
            let delta = theta / steps;
            let half_w = (delta / 2.0).cos();
            let on = |ang: f64| {
                let (s, c) = ang.sin_cos();
                world(Point2::new(
                    radius.mul_add(c, center.x),
                    radius.mul_add(s, center.y),
                ))
            };
            let tangent_point = |ang: f64| {
                let (s, c) = ang.sin_cos();
                let r = radius / half_w;
                world(Point2::new(r.mul_add(c, center.x), r.mul_add(s, center.y)))
            };
            let mut control = vec![on(start)];
            let mut weights = vec![1.0f64];
            for k in 0..m {
                #[allow(clippy::cast_precision_loss)]
                let base = delta.mul_add(k as f64, start);
                control.push(tangent_point(base + delta / 2.0));
                weights.push(half_w);
                control.push(on(base + delta));
                weights.push(1.0);
            }
            let mut knots = vec![0.0, 0.0, 0.0];
            for k in 1..m {
                #[allow(clippy::cast_precision_loss)]
                let t = k as f64 / steps;
                knots.push(t);
                knots.push(t);
            }
            knots.extend([1.0, 1.0, 1.0]);
            Ok(NurbsCurve3::new(
                KnotVector::clamped(knots, 2)?,
                control,
                weights,
            )?)
        }
    }
}

// ---------------------------------------------------------------------
// §5.3 + §5.5: compatibility
// ---------------------------------------------------------------------

/// Makes section curves COMPATIBLE (Book §10.3's precondition): one
/// common degree (§5.5 degree elevation) on one common knot vector
/// (§5.3 knot merging — refine each section by whatever the union
/// needs).
///
/// Both operations are **exact in ℝ**: they change the representation,
/// never the locus. Afterwards every section has the same degree, the
/// same knots, and therefore the same control-point count — which is
/// what makes a tensor-product skin possible at all.
///
/// # Errors
///
/// [`SkinError::TooFewSections`], [`SkinError::DomainNotUnit`] for a
/// section off the unit domain, and the propagated knot-algebra /
/// structure refusals.
pub fn make_compatible(sections: &[NurbsCurve3<f64>]) -> Result<Vec<NurbsCurve3<f64>>, SkinError> {
    if sections.len() < 2 {
        return Err(SkinError::TooFewSections {
            have: sections.len(),
            need: 2,
        });
    }
    for (i, c) in sections.iter().enumerate() {
        let domain = c.domain();
        if domain.0 != 0.0 || domain.1 != 1.0 {
            return Err(SkinError::DomainNotUnit { section: i, domain });
        }
    }
    // §5.5: one common degree.
    let degree = sections.iter().map(NurbsCurve3::degree).max().unwrap_or(0);
    let elevated: Vec<NurbsCurve3<f64>> = sections
        .iter()
        .map(|c| {
            let raise = degree - c.degree();
            if raise == 0 {
                Ok(c.clone())
            } else {
                c.elevate_degree(raise).map_err(SkinError::KnotAlgebra)
            }
        })
        .collect::<Result<_, _>>()?;
    // §5.3: the union knot vector — every distinct interior value at
    // the greatest multiplicity any section gives it.
    let mut union: Vec<(f64, usize)> = Vec::new();
    for c in &elevated {
        for (value, mult) in c.knots().interior_knots() {
            match union.iter_mut().find(|(v, _)| *v == value) {
                Some((_, m)) => *m = (*m).max(mult),
                None => union.push((value, mult)),
            }
        }
    }
    union.sort_by(|a, b| a.0.total_cmp(&b.0));
    let merged: Vec<NurbsCurve3<f64>> = elevated
        .iter()
        .map(|c| {
            let own: Vec<(f64, usize)> = c.knots().interior_knots().collect();
            let mut add: Vec<f64> = Vec::new();
            for (value, want) in &union {
                let have = own
                    .iter()
                    .find(|(v, _)| *v == *value)
                    .map_or(0, |(_, m)| *m);
                for _ in have..*want {
                    add.push(*value);
                }
            }
            if add.is_empty() {
                Ok(c.clone())
            } else {
                c.refine_knots(&add).map_err(SkinError::KnotAlgebra)
            }
        })
        .collect::<Result<_, _>>()?;
    Ok(merged)
}

// ---------------------------------------------------------------------
// §10.3: the skinned surface
// ---------------------------------------------------------------------

/// The v-direction parameters Book Eq. 10.8 chooses for a set of
/// COMPATIBLE sections: chord lengths accumulated down each control
/// row, then **averaged across the rows** — one parameterization for
/// the whole surface, which is what keeps the u-columns' solves
/// consistent.
///
/// # Errors
///
/// [`SkinError::TooFewSections`], [`SkinError::SectionShapeMismatch`]
/// for sections that did not come out of [`make_compatible`], and
/// [`SkinError::DegenerateSection`] when two consecutive sections
/// coincide at every control point (no chord step exists there).
// `!(x > 0)` and `!(a < b)` are deliberate NaN-catching (the
// geom-core::spline::algebra note): a poisoned coordinate must take
// the refusal arm, not slip through a negated comparison.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn skin_parameters(sections: &[NurbsCurve3<f64>]) -> Result<Vec<f64>, SkinError> {
    let k = sections.len();
    if k < 2 {
        return Err(SkinError::TooFewSections { have: k, need: 2 });
    }
    let n = sections[0].control().len();
    // Every section must present the same control row count — the
    // caller's `make_compatible` obligation, checked rather than
    // assumed (an out-of-range index would be a panic, and this
    // entry is public).
    for (i, c) in sections.iter().enumerate().skip(1) {
        if c.control().len() != n {
            return Err(SkinError::SectionShapeMismatch {
                section: i,
                expected: n,
                found: c.control().len(),
                what: "compatible control points",
            });
        }
    }
    let mut totals = vec![0.0f64; k];
    let mut rows = 0usize;
    for i in 0..n {
        let mut chords = vec![0.0f64; k];
        let mut total = 0.0f64;
        for j in 1..k {
            let a = sections[j - 1].control()[i];
            let b = sections[j].control()[i];
            chords[j] = a.distance(b);
            total += chords[j];
        }
        // A row that never moves (a corner pinned across every
        // section) carries no parameterization information — it is not
        // an error, it simply abstains from the average.
        if !(total > 0.0) {
            continue;
        }
        rows += 1;
        let mut acc = 0.0f64;
        for j in 1..k {
            acc += chords[j];
            totals[j] += acc / total;
        }
    }
    if rows == 0 {
        return Err(SkinError::DegenerateSection {
            section: 0,
            what: "every control row is pinned — the sections coincide",
        });
    }
    #[allow(clippy::cast_precision_loss)]
    let denom = rows as f64;
    let mut params: Vec<f64> = totals.iter().map(|t| t / denom).collect();
    // The averaged accumulation pins 0 and 1 in exact arithmetic; make
    // the ends EXACT so the averaged-knot construction's clamped
    // `0 → 1` precondition holds bit-for-bit (D9 structure selection).
    params[0] = 0.0;
    params[k - 1] = 1.0;
    for j in 1..k {
        if !(params[j - 1] < params[j]) {
            return Err(SkinError::DegenerateSection {
                section: j,
                what: "sections coincide (no chord step between them)",
            });
        }
    }
    Ok(params)
}

/// **§10.3, the skinned surface.** Interpolates COMPATIBLE section
/// curves v-directionally at `v_degree`, in homogeneous ℝ⁴ — the
/// Book's rational treatment: the weighted control points are the data,
/// one shared collocation solve carries every column, and the
/// projective divide happens once at the end.
///
/// # No synthesized weight channel (#207)
///
/// The homogeneous treatment above is the RATIONAL lane, and it runs
/// only when the input is rational. When every section is **integral**
/// — every weight bit-exactly `1.0`, the kernel's own definition of
/// non-rational (`NurbsCurve::speed_lower_bound`'s test) — the
/// sections are interpolated in plain Cartesian ℝ³ and the result
/// carries exactly `1.0` weights.
///
/// The two lanes agree exactly in ℝ: with `w ≡ 1` the homogeneous data
/// IS the Cartesian data, and the weight column's interpolant is the
/// all-ones control vector by partition of unity (`Σⱼ Nⱼ(u) = 1` at
/// every parameter, so the collocation system's right-hand side of
/// ones has the vector of ones as its exact solution). They do NOT
/// agree in `f64`: solving that column by LU and then dividing every
/// coordinate through by the result is a normalization ROUND-TRIP, and
/// it lands off `1.0` by an ulp for most parameterizations — measured
/// `1.0000000000000002` and `0.9999999999999998` on plain polyline
/// sections at unevenly spaced stations.
///
/// That ulp is not a small error, it is a **change of kind**: a wall
/// with a non-unit weight is bitwise rational, so the seam carrier
/// `geom_brep::boundary_iso_u` reads back rational,
/// `NurbsCurve::speed_lower_bound` returns its documented poison (the
/// derivative of a rational B-spline is not a convex combination of
/// any control net), the `nurbs_span_meter` margin is `Invalid`, and
/// the whole body refuses at assembly. Before this lane existed, that
/// was the fate of every `sweep_body` with a curved path and every
/// `loft_body` whose sections were not evenly spaced.
///
/// So the fix is a fit-contract fix, not a repair: an integral input
/// must not be answered with a rational output, and the way to
/// guarantee that is to never manufacture the weight channel in the
/// first place — NOT to solve for it and then snap the answer back to
/// the value it was supposed to have (a silent heal, which would also
/// leave the control points divided by the pre-snap denominator).
/// `integral` is decided by `==` on the input's own bits: C6 structure
/// selection about what the caller handed in, never a predicate, never
/// a tolerance.
///
/// The Cartesian lane is bitwise conservative for the cases that
/// already worked. `solve_square` factors the matrix once from `a`
/// alone and substitutes each right-hand-side column independently, so
/// dropping the weight column cannot move the `x`/`y`/`z` columns; the
/// row entries themselves are unchanged because `p.x * 1.0` is `p.x`
/// bit-for-bit; and the final divide it removes was a division by
/// exactly `1.0` in precisely the cases whose weights came out exact.
/// A uniformly spaced loft therefore skins to bit-identical walls
/// (pinned in `tests/m7_skin_integral.rs`).
///
/// # Numbered note 5 (spec §2): the solve is DENSE, and where that lands
///
/// The collocation system is `k × k` in the SECTION count `k` — not in
/// the control count — solved by fixed-order LU with `4·n` (integral
/// input: `3·n`) simultaneous right-hand sides (`n` = control points
/// per section). Cost is
/// `O(k³)` for the factorization plus `O(k²·n)` for the substitutions;
/// memory is `O(k² + k·n)`.
///
/// Spec §2 permits a banded solver only if PR 4's stack already
/// provides one. It does not — `geom_core::linalg::lsq` offers
/// `solve_square` and `solve_normal`, both dense — so this is the
/// dense small-system lane, and the size limit is worth stating rather
/// than discovering. Realistic lofts put `k` in the single digits to
/// low tens (the Book's §10.3 examples; a twenty-section loft is
/// already an unusual part), where `k³ ≤ 8·10³` is unmeasurable next
/// to the knot-algebra work that precedes it. A banded solver starts
/// to matter around `k` in the high hundreds, where `k³` reaches
/// `10⁸` — scattered-data fitting territory (M7), not feature
/// modelling. The matrix IS banded (half-bandwidth `q` for v-degree
/// `q`, since only `q + 1` basis functions are nonzero per row), so
/// the upgrade is a solver swap behind this same entry when a use case
/// asks for it; nothing about the structure selection would change.
///
/// The u-direction structure is the sections' shared one, untouched.
/// The result therefore passes through every section exactly (in ℝ) at
/// its own v-parameter — the property the acceptance suite verifies by
/// evaluation, not by assertion.
///
/// # Errors
///
/// [`SkinError::TooFewSections`], [`SkinError::BadDegree`],
/// [`SkinError::SectionShapeMismatch`] for sections that did not come
/// out of [`make_compatible`], [`SkinError::DegenerateSection`],
/// [`SkinError::Fit`] for a degenerate collocation system, and
/// [`SkinError::Structure`] — notably a non-positive interpolated
/// weight, refused at the door because the convex-hull invariant every
/// C9 bound stands on requires `w > 0`.
pub fn skin(
    sections: &[NurbsCurve3<f64>],
    v_degree: usize,
) -> Result<NurbsSurface<f64>, SkinError> {
    let k = sections.len();
    if k < 2 {
        return Err(SkinError::TooFewSections { have: k, need: 2 });
    }
    if v_degree == 0 || v_degree >= k {
        return Err(SkinError::BadDegree {
            degree: v_degree,
            sections: k,
        });
    }
    let knots_u = sections[0].knots().clone();
    let n = sections[0].control().len();
    for (i, c) in sections.iter().enumerate().skip(1) {
        if c.control().len() != n || c.knots().knots() != knots_u.knots() {
            return Err(SkinError::SectionShapeMismatch {
                section: i,
                expected: n,
                found: c.control().len(),
                what: "compatible control points",
            });
        }
    }
    let params = skin_parameters(sections)?;
    skin_on(sections, v_degree, &params)
}

/// [`skin`] on an EXPLICIT v-parameterization — the entry a multi-wall
/// body needs, so every wall of one loft agrees on where its sections
/// sit (see [`loft_geometry`]).
///
/// # Errors
///
/// Exactly [`skin`]'s, plus [`SkinError::Fit`] for a parameterization
/// that is not clamped `0 → 1` and strictly ascending.
pub fn skin_on(
    sections: &[NurbsCurve3<f64>],
    v_degree: usize,
    params: &[f64],
) -> Result<NurbsSurface<f64>, SkinError> {
    let k = sections.len();
    if k < 2 {
        return Err(SkinError::TooFewSections { have: k, need: 2 });
    }
    if v_degree == 0 || v_degree >= k {
        return Err(SkinError::BadDegree {
            degree: v_degree,
            sections: k,
        });
    }
    let knots_u = sections[0].knots().clone();
    let n = sections[0].control().len();
    for (i, c) in sections.iter().enumerate().skip(1) {
        if c.control().len() != n || c.knots().knots() != knots_u.knots() {
            return Err(SkinError::SectionShapeMismatch {
                section: i,
                expected: n,
                found: c.control().len(),
                what: "compatible control points",
            });
        }
    }
    // INTEGRAL INPUT ⇒ INTEGRAL OUTPUT, by structure (see the
    // "no synthesized weight channel" section of these docs): the
    // kernel's own definition of non-rational is bit-exact unit
    // weights (`NurbsCurve::speed_lower_bound`'s test), so this is a
    // structure question about the input, decided by `==` — C6
    // structure selection, never a predicate.
    let integral = sections
        .iter()
        .all(|c| c.weights().iter().all(|w| *w == 1.0));
    // Rows of data: one row per section. Homogeneous `(w·x, w·y, w·z,
    // w)` quadruples (`4·n` wide) for a rational input; plain
    // Cartesian triples (`3·n` wide) when the input is integral, where
    // the weight channel carries no information the output does not
    // already have.
    let stride = if integral { 3 } else { 4 };
    let rows: Vec<Vec<f64>> = sections
        .iter()
        .map(|c| {
            let mut row = Vec::with_capacity(stride * n);
            for (p, w) in c.control().iter().zip(c.weights()) {
                if integral {
                    row.extend([p.x, p.y, p.z]);
                } else {
                    row.extend([p.x * w, p.y * w, p.z * w, *w]);
                }
            }
            row
        })
        .collect();
    let (knots_v, solved) = interpolate_columns(params, v_degree, &rows)?;
    // Un-homogenize into the surface's row-major (u-major) layout:
    // `idx = iu · nv + iv`, `nv = solved.len()`. The integral lane has
    // nothing to un-homogenize: the solved columns ARE the control
    // coordinates, and every weight is the exact `1.0` the input had.
    let nv = solved.len();
    let mut control = Vec::with_capacity(n * nv);
    let mut weights = Vec::with_capacity(n * nv);
    for i in 0..n {
        for row in &solved {
            if integral {
                control.push(Point3::new(row[3 * i], row[3 * i + 1], row[3 * i + 2]));
                weights.push(1.0);
            } else {
                let (x, y, z, w) = (row[4 * i], row[4 * i + 1], row[4 * i + 2], row[4 * i + 3]);
                control.push(Point3::new(x / w, y / w, z / w));
                weights.push(w);
            }
        }
    }
    // `NurbsSurface::new` is the `w > 0` door (and the count door).
    NurbsSurface::new(knots_u, knots_v, control, weights).map_err(SkinError::Structure)
}

/// Carries an `f64`-chosen surface structure to any scalar for
/// evaluation (crate docs' C6 note): the control bits and weights are
/// DATA — the Q8 definition — so lifting is `from_f64`, exact at every
/// scalar, and the interval lane encloses the very same surface.
///
/// # Errors
///
/// [`SplineError`] — unreachable for a surface that already validated,
/// surfaced rather than swallowed.
pub fn lift_surface<T: Real>(s: &NurbsSurface<f64>) -> Result<NurbsSurface<T>, SplineError> {
    let control = s
        .control()
        .iter()
        .map(|p| Point3::new(T::from_f64(p.x), T::from_f64(p.y), T::from_f64(p.z)))
        .collect();
    NurbsSurface::new(
        s.knots_u().clone(),
        s.knots_v().clone(),
        control,
        s.weights().to_vec(),
    )
}

// ---------------------------------------------------------------------
// §10.3/§10.4 as FEATURE geometry: the walls of a lofted body
// ---------------------------------------------------------------------

/// The definitional geometry of a loft or a path sweep: one skinned
/// wall per profile segment, plus the v-parameters the sections sit
/// at.
///
/// The **segment structure is the u-direction** (spec §3): wall `[l][j]`
/// spans profile loop `l`'s segment `j` across every section, so a
/// profile corner is a wall–wall seam by construction and every
/// cap–wall and wall–wall edge is an ISO-PARAMETER curve of the wall
/// it bounds — `v = 0` / `v = 1` for the caps, `u = 0` / `u = 1` for
/// the seams. That is the definitional payoff: those edges' pcurves are
/// exact straight lines in UV, no fit anywhere.
#[derive(Clone, Debug)]
pub struct LoftGeometry {
    /// Skinned walls, per loop (outer first, then holes in canonical
    /// order), per segment in canonical order.
    pub walls: Vec<Vec<Arc<NurbsSurface<f64>>>>,
    /// The v-parameter each section sits at (`params[0] = 0`,
    /// `params[k - 1] = 1`), shared by every wall — the sections are
    /// planar cross-sections of the whole loft at these values.
    pub section_params: Vec<f64>,
    /// The compatible section curves per (loop, segment), kept because
    /// the cap and rim geometry is exactly section 0's and section
    /// `k − 1`'s — no re-derivation, no drift.
    pub sections: Vec<Vec<Vec<NurbsCurve3<f64>>>>,
}

/// One section of a loft or sweep: its loops in the profile
/// vocabulary — outer boundary first, holes after, each a
/// [`ProfileLoop`] (closed by construction; segment `j` runs from
/// vertex `j` to vertex `(j + 1) mod n`, carrying vertex `j`'s
/// bulge) — the same vocabulary extrude and revolve speak. Each
/// interior joint has exactly one naming, so a walls-vs-caps
/// disagreement is unrepresentable.
pub type Section = Vec<ProfileLoop<f64>>;

/// The section's world-space traversal data for segment `j`: the
/// `(a, b, bulge)` triple as a [`SketchSegment`], exactly the lowered
/// form [`segment_curve`] consumes (`segment_curve` stays public as
/// the exact-path-leg door, and routing every wall through it keeps
/// the produced NURBS on one code path).
fn vertex_segment(lp: &profile::ValidatedLoop<f64>, j: usize) -> SketchSegment<f64> {
    let vs = lp.vertices();
    let a = vs[j];
    let b = vs[(j + 1) % vs.len()];
    if a.bulge() == 0.0 {
        SketchSegment::Line {
            a: a.pos(),
            b: b.pos(),
        }
    } else {
        SketchSegment::Arc {
            a: a.pos(),
            b: b.pos(),
            bulge: a.bulge(),
        }
    }
}

/// Validates every section at the door: each section runs
/// through [`Profile::validate`] against its own placement — the
/// same gate extrude and revolve profiles pass — and the CANONICAL
/// loops (outer counterclockwise first, holes clockwise, canonical
/// start vertex) are what the skin consumes, so the walls and the
/// body assembly's caps read the same traversal by construction.
fn validate_sections(
    sections: &[Section],
    places: &[Affine3<f64>],
    tol: Tol,
) -> Result<Vec<ValidatedProfile<f64>>, SkinError> {
    sections
        .iter()
        .zip(places)
        .enumerate()
        .map(|(i, (loops, place))| {
            Profile::new(SketchPlane::new(*place), loops.clone())
                .validate(tol)
                .map_err(|source| SkinError::SectionProfile { section: i, source })
        })
        .collect()
}

/// Builds the definitional walls of a loft (§10.3) from profile-loop
/// sections and their placements.
///
/// `sections[k][l]` is section `k`, loop `l` (a [`ProfileLoop`]) in
/// the section's own sketch coordinates; `places[k]` is that
/// section's rigid placement. Every section must present the same
/// loop and vertex counts — the correspondence is BY INDEX, and there
/// is no honest way to guess one that was not given. Each section
/// passes [`Profile::validate`] at the door (fail loud — a section
/// that would not extrude does not skin either), and the canonical
/// loops are what get skinned.
///
/// # Errors
///
/// [`SkinError::TooFewSections`], [`SkinError::SectionShapeMismatch`]
/// naming the disagreeing section and count,
/// [`SkinError::SectionProfile`] for a section the profile door
/// refuses, [`SkinError::BadDegree`], and every refusal
/// [`segment_curve`], [`make_compatible`] and [`skin`] carry.
pub fn loft_geometry(
    sections: &[Section],
    places: &[Affine3<f64>],
    v_degree: usize,
    tol: Tol,
) -> Result<LoftGeometry, SkinError> {
    let k = sections.len();
    if k < 2 {
        return Err(SkinError::TooFewSections { have: k, need: 2 });
    }
    if places.len() != k {
        return Err(SkinError::SectionShapeMismatch {
            section: places.len().min(k),
            expected: k,
            found: places.len(),
            what: "placements",
        });
    }
    if v_degree == 0 || v_degree >= k {
        return Err(SkinError::BadDegree {
            degree: v_degree,
            sections: k,
        });
    }
    let validated = validate_sections(sections, places, tol)?;
    let loops = validated[0].loops().len();
    for (i, s) in validated.iter().enumerate().skip(1) {
        if s.loops().len() != loops {
            return Err(SkinError::SectionShapeMismatch {
                section: i,
                expected: loops,
                found: s.loops().len(),
                what: "loops",
            });
        }
        for (l, lp) in s.loops().iter().enumerate() {
            if lp.vertices().len() != validated[0].loops()[l].vertices().len() {
                return Err(SkinError::SectionShapeMismatch {
                    section: i,
                    expected: validated[0].loops()[l].vertices().len(),
                    found: lp.vertices().len(),
                    what: "segments",
                });
            }
        }
    }
    // The v-parameterization is the SURFACE's, not one strip's: taken
    // from the first strip and reused, so every wall of the body
    // agrees on where its sections sit. (Book §10.3 averages across
    // control rows for exactly this reason; averaging across strips
    // too would make the sections planar cross-sections of nothing in
    // particular.) It is computed through the same helper
    // [`loft_parameters`] answers with — ONE code path, so the query
    // can never drift from the construction it reports.
    let params = first_strip_parameters(&validated, places)?;
    let mut walls = Vec::with_capacity(loops);
    let mut kept = Vec::with_capacity(loops);
    for l in 0..loops {
        let n_segments = validated[0].loops()[l].vertices().len();
        let mut loop_walls = Vec::with_capacity(n_segments);
        let mut loop_sections = Vec::with_capacity(n_segments);
        for j in 0..n_segments {
            let raw: Vec<NurbsCurve3<f64>> = validated
                .iter()
                .zip(places)
                .enumerate()
                .map(|(i, (s, place))| segment_curve(i, vertex_segment(&s.loops()[l], j), *place))
                .collect::<Result<_, _>>()?;
            let compat = make_compatible(&raw)?;
            loop_walls.push(Arc::new(skin_on(&compat, v_degree, &params)?));
            loop_sections.push(compat);
        }
        walls.push(loop_walls);
        kept.push(loop_sections);
    }
    Ok(LoftGeometry {
        walls,
        section_params: params,
        sections: kept,
    })
}

/// **What v-parameters will the skin put my sections at?** — the
/// read-back door for [`LoftGeometry::section_params`], answerable
/// from what a section author already holds ([`Section`]s and their
/// placements), before any surface is built.
///
/// [`skin_parameters`] answers the same question one layer down, but
/// only for COMPATIBLE `NurbsCurve3` sections — a caller holding
/// [`ProfileLoop`]s cannot reach it without redoing `segment_curve` /
/// [`make_compatible`] by hand. This door is that path, and it is
/// literally the path [`loft_geometry`] takes (one shared helper), so
/// the answer is the construction's, not a re-derivation of it.
///
/// Chord-length averaging (Book Eq. 10.8) is what makes this worth
/// asking: the answer is NOT the z-spacing, and hand-deriving it is
/// how demos and fixtures drifted.
///
/// `v_degree` is not used to place the sections — it is validated, so
/// this door refuses exactly where [`loft_body`](crate::loft_body)
/// would rather than answering for a loft that cannot be built.
///
/// # Errors
///
/// [`SkinError::TooFewSections`], [`SkinError::SectionShapeMismatch`]
/// (placement count), [`SkinError::BadDegree`], and every refusal
/// [`segment_curve`], [`make_compatible`] and [`skin_parameters`]
/// carry.
///
/// ```
/// use geom_core::{Affine3, Point2, Tol, Vec3};
/// use profile::RawLoop;
/// use sweep::{ProfileLoop, Section, loft_parameters};
///
/// let tol = Tol::witness();
/// let quad = |pts: [(f64, f64); 4]| -> Section {
///     vec![ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)))]
/// };
/// // The corpus's non-uniform prism: square, flared trapezoid,
/// // square — placed at z = 0, 1, 3.
/// let sections = vec![
///     quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
///     quad([(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
///     quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
/// ];
/// let places: Vec<Affine3<f64>> = [0.0, 1.0, 3.0]
///     .iter()
///     .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
///     .collect();
///
/// let params = loft_parameters(&sections, &places, 2, tol).expect("the sections skin");
/// // Ends pinned; the middle is the CHORD-length share, not the
/// // z-share (which would be 1/3): the flare lengthens the first
/// // chord, giving t = √73 / (√73 + √265).
/// assert_eq!(params[0], 0.0);
/// assert_eq!(params[2], 1.0);
/// assert_eq!(params[1], 0.34419950074181277);
/// ```
pub fn loft_parameters(
    sections: &[Section],
    places: &[Affine3<f64>],
    v_degree: usize,
    tol: Tol,
) -> Result<Vec<f64>, SkinError> {
    let k = sections.len();
    if k < 2 {
        return Err(SkinError::TooFewSections { have: k, need: 2 });
    }
    if places.len() != k {
        return Err(SkinError::SectionShapeMismatch {
            section: places.len().min(k),
            expected: k,
            found: places.len(),
            what: "placements",
        });
    }
    if v_degree == 0 || v_degree >= k {
        return Err(SkinError::BadDegree {
            degree: v_degree,
            sections: k,
        });
    }
    first_strip_parameters(&validate_sections(sections, places, tol)?, places)
}

/// The first strip's v-parameters — the whole loft's, by the
/// construction rule above. Loop-less sections have no strip and so
/// no parameterization: the empty list, exactly what the lazy form
/// this replaced produced.
fn first_strip_parameters(
    validated: &[ValidatedProfile<f64>],
    places: &[Affine3<f64>],
) -> Result<Vec<f64>, SkinError> {
    if validated.is_empty() || validated[0].loops().is_empty() {
        return Ok(Vec::new());
    }
    let raw: Vec<NurbsCurve3<f64>> = validated
        .iter()
        .zip(places)
        .enumerate()
        .map(|(i, (s, place))| segment_curve(i, vertex_segment(&s.loops()[0], 0), *place))
        .collect::<Result<_, _>>()?;
    skin_parameters(&make_compatible(&raw)?)
}

// ---------------------------------------------------------------------
// §10.4: the swept surface, by instantiate-and-skin
// ---------------------------------------------------------------------

/// **§10.4, the path sweep.** Places rigid copies of `profile` at
/// `stations` points along `path` and skins them (Book §10.4's
/// instantiate-and-skin; C11's scope box, verbatim: rigid profile,
/// path-following, **no** variable sections and **no** scaling laws).
///
/// The frame is path-FOLLOWING: at each station the profile is turned
/// by the minimal rotation carrying the path's tangent at station 0 to
/// its tangent there, then translated onto the path. A VANISHING
/// tangent has no such rotation and refuses typed
/// ([`SkinError::PathTangentReversal`]) rather than picking one.
///
/// # C6: the anti-parallel knife edge, stated honestly
///
/// The turn is selected from `sin = |t₀ × tᵢ|` and `cos = t₀ · tᵢ` by
/// EXACT `f64` comparison — structure selection, not a predicate: no
/// topology depends on it, so it never routes through `k_stats`. The
/// consequence is that `sin > 0.0` separates only the exactly-zero
/// case. At a genuine half-turn `sin` evaluates to ≈ `1.2e-16`, not
/// `0`, so the anti-parallel arm does not fire and the frame is built
/// from an axis whose DIRECTION is decided by cancellation.
///
/// No band is asserted here, deliberately (D4 ¶1: a band needs a
/// margin with a stated meaning and a stated lever arm). `sin` is a
/// dimensionless sine, not a length, so the linear band does not apply
/// to it; converting it to a length would need a lever arm this
/// construction does not have — the profile's extent is not the
/// conditioning scale of the cross product, and asserting one would be
/// a derived threshold nobody derived. Banding this correctly means
/// giving the frame choice a named angular predicate with a real
/// margin, which is a design change, not a fix-pass edit.
///
/// Under Q8 the produced skin is not an approximation of a swept
/// locus — it IS the sweep (crate-module docs).
///
/// # Errors
///
/// [`SkinError::TooFewSections`] for `stations < 2`,
/// [`SkinError::PathTangentReversal`] naming the station, plus every
/// refusal [`loft_geometry`] carries.
// `!(x > 0)` and `!(a < b)` are deliberate NaN-catching (the
// geom-core::spline::algebra note): a poisoned coordinate must take
// the refusal arm, not slip through a negated comparison.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn sweep_geometry(
    profile: &[ProfileLoop<f64>],
    place: Affine3<f64>,
    path: &NurbsCurve3<f64>,
    stations: usize,
    v_degree: usize,
    tol: Tol,
) -> Result<LoftGeometry, SkinError> {
    let places = sweep_places(place, path, stations)?;
    let sections: Vec<Section> = core::iter::repeat_n(profile.to_vec(), stations).collect();
    loft_geometry(&sections, &places, v_degree, tol)
}

/// The rigid section placements of a §10.4 path sweep — the
/// path-following frame of [`sweep_geometry`], factored so the BODY
/// assembly (M6-3, `crate::loft`) rides the exact same machinery with
/// no semantic fork. All the docs (and the C6 anti-parallel knife
/// edge) above apply verbatim: this IS that function's frame, moved.
///
/// # Errors
///
/// [`SkinError::TooFewSections`] for `stations < 2`,
/// [`SkinError::PathTangentReversal`] naming the station.
// `!(x > 0)` and `!(a < b)` are deliberate NaN-catching (the
// geom-core::spline::algebra note).
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn sweep_places(
    place: Affine3<f64>,
    path: &NurbsCurve3<f64>,
    stations: usize,
) -> Result<Vec<Affine3<f64>>, SkinError> {
    if stations < 2 {
        return Err(SkinError::TooFewSections {
            have: stations,
            need: 2,
        });
    }
    #[allow(clippy::cast_precision_loss)]
    let last = (stations - 1) as f64;
    let (lo, hi) = path.domain();
    let origin = Point3::new(
        place.translation.x,
        place.translation.y,
        place.translation.z,
    );
    let unit_tangent = |station: usize, t: f64| -> Result<Vec3<f64>, SkinError> {
        let d = path.deriv(t);
        let n = d.norm();
        if !(n > 0.0) || !n.is_finite() {
            return Err(SkinError::PathTangentReversal { station });
        }
        Ok(d / n)
    };
    let t_of = |i: usize| {
        #[allow(clippy::cast_precision_loss)]
        let s = i as f64 / last;
        (hi - lo).mul_add(s, lo)
    };
    let base_tangent = unit_tangent(0, t_of(0))?;
    let base_point = path.eval(t_of(0));
    let mut places = Vec::with_capacity(stations);
    for i in 0..stations {
        let t = t_of(i);
        let tangent = unit_tangent(i, t)?;
        let axis = base_tangent.cross(tangent);
        let sin = axis.norm();
        let cos = base_tangent.dot(tangent);
        let turn = if sin > 0.0 {
            Affine3::rotation_about_axis(origin, axis / sin, sin.atan2(cos))
        } else if cos > 0.0 {
            Affine3::identity()
        } else {
            // Anti-parallel: the minimal rotation is not unique (every
            // axis perpendicular to the tangent turns one into the
            // other). Refuse rather than pick one — a sweep whose path
            // doubles back needs a stated frame, not a guess.
            //
            // C6 knife edge (see the fn docs): `sin > 0.0` is an exact
            // structure comparison, so this arm fires only on an
            // EXACT zero. A float half-turn gives sin ≈ 1.2e-16 and
            // takes the branch above instead, building from an
            // ill-conditioned axis. Pinned as executed behaviour, not
            // asserted as intent.
            return Err(SkinError::PathTangentReversal { station: i });
        };
        places.push(Affine3::translation(path.eval(t) - base_point) * turn * place);
    }
    Ok(places)
}
