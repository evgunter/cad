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
//! Skinning needs curves AND surfaces in one place, and `geom-surfaces`
//! deliberately does not depend on `geom-curves` (its crate docs: the
//! two are peer evaluators; machinery spanning both belongs one layer
//! up). `sweep` is that layer for constructions — it already depends on
//! both, and §10.3/§10.4 are sweep operations by name. The produced
//! payload is `geom_surfaces::NurbsSurface`, so nothing about the
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

use geom_brep::SketchSegment;
use geom_core::spline::{KnotAlgebraError, KnotVector, SplineError};
use geom_core::{Affine3, COINCIDENCE_RECOURSE, Point2, Point3, Real, Vec3};
use geom_curves::NurbsCurve3;
use geom_curves::fit::{FitError, interpolate_columns};
use geom_surfaces::NurbsSurface;

/// The quarter-turn ceiling on one rational-quadratic arc span: every
/// sub-arc of a converted profile arc is at most this wide, so the
/// tangent-intersection control point stays finite and well
/// conditioned (`cos(Δ/2) ≥ cos(π/4)`).
const MAX_SUB_ARC: f64 = core::f64::consts::FRAC_PI_2;

/// Typed refusal of the §10.3/§10.4 constructions (closed enum, D4 ¶3).
///
/// Every arm is a STRUCTURAL statement about the inputs — section
/// counts, loop counts, degrees, weights, solvability. Structure
/// selection is exact-`f64` (C6), so none of these arms has an in-band
/// twin and the two-tolerance discipline does not apply to them; the
/// one arm that DOES describe a geometric near-degeneracy
/// ([`SkinError::DegenerateSection`]) carries the shared recourse, and
/// so does its in-band neighbour [`SkinError::Escalated`].
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
    /// Open and closed sections were mixed. A closed loop's skin is a
    /// tube, an open one's is a strip; the two have different topology
    /// and no correspondence between them.
    OpenClosedMixed {
        /// The first section whose closedness differs from section 0.
        section: usize,
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
    /// The DEFINITE half of a two-tolerance pair whose in-band twin is
    /// [`SkinError::Escalated`] — one user situation, one recourse.
    DegenerateSection {
        /// Where it sits.
        section: usize,
        /// What was degenerate.
        what: &'static str,
    },
    /// A degeneracy decision landed in the ambiguity band (the in-band
    /// half of [`SkinError::DegenerateSection`]'s pair).
    Escalated {
        /// The escalated decision.
        diag: geom_core::Indeterminate,
    },
    /// The requested v-degree is 0, or ≥ the section count.
    BadDegree {
        /// The requested degree.
        degree: usize,
        /// The sections available.
        sections: usize,
    },
    /// The path's tangent reverses (or vanishes) between consecutive
    /// stations, so no rigid frame carries the profile through it.
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
            Self::OpenClosedMixed { section } => write!(
                f,
                "skin: section {section} is open where section 0 is closed (or the reverse) \
                 — a tube and a strip have no correspondence"
            ),
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
            Self::Escalated { diag } => write!(f, "skin: {diag}"),
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
            let b2 = bulge * bulge;
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

/// The interior knots of `kv` as `(value, multiplicity)`, ascending —
/// an exact-`f64` structure scan (C6), never a tolerance question.
fn interior_multiplicities(kv: &KnotVector) -> Vec<(f64, usize)> {
    let p = kv.degree();
    let knots = kv.knots();
    let mut out: Vec<(f64, usize)> = Vec::new();
    for u in &knots[p + 1..knots.len() - p - 1] {
        match out.last_mut() {
            Some((v, m)) if *v == *u => *m += 1,
            _ => out.push((*u, 1)),
        }
    }
    out
}

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
        for (value, mult) in interior_multiplicities(c.knots()) {
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
            let own = interior_multiplicities(c.knots());
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
    // Rows of homogeneous data: one row per section, `4·n` wide.
    let rows: Vec<Vec<f64>> = sections
        .iter()
        .map(|c| {
            let mut row = Vec::with_capacity(4 * n);
            for (p, w) in c.control().iter().zip(c.weights()) {
                row.extend([p.x * w, p.y * w, p.z * w, *w]);
            }
            row
        })
        .collect();
    let (knots_v, solved) = interpolate_columns(params, v_degree, &rows)?;
    // Un-homogenize into the surface's row-major (u-major) layout:
    // `idx = iu · nv + iv`, `nv = solved.len()`.
    let nv = solved.len();
    let mut control = Vec::with_capacity(n * nv);
    let mut weights = Vec::with_capacity(n * nv);
    for i in 0..n {
        for row in &solved {
            let (x, y, z, w) = (row[4 * i], row[4 * i + 1], row[4 * i + 2], row[4 * i + 3]);
            control.push(Point3::new(x / w, y / w, z / w));
            weights.push(w);
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

/// One section of a loft, as this module needs it: the segments of
/// each loop, already in world space through the section's placement.
pub type SectionSegments = Vec<Vec<SketchSegment<f64>>>;

/// Builds the definitional walls of a loft (§10.3) from section
/// segment chains and their placements.
///
/// `sections[k][l][j]` is section `k`, loop `l`, segment `j`, in the
/// section's own sketch coordinates; `places[k]` is that section's
/// rigid placement. Every section must present the same loop and
/// segment counts — the correspondence is BY INDEX, and there is no
/// honest way to guess one that was not given.
///
/// # Errors
///
/// [`SkinError::TooFewSections`], [`SkinError::SectionShapeMismatch`]
/// naming the disagreeing section and count, [`SkinError::BadDegree`],
/// and every refusal [`segment_curve`], [`make_compatible`] and
/// [`skin`] carry.
pub fn loft_geometry(
    sections: &[SectionSegments],
    places: &[Affine3<f64>],
    v_degree: usize,
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
    let loops = sections[0].len();
    for (i, s) in sections.iter().enumerate().skip(1) {
        if s.len() != loops {
            return Err(SkinError::SectionShapeMismatch {
                section: i,
                expected: loops,
                found: s.len(),
                what: "loops",
            });
        }
        for (l, chain) in s.iter().enumerate() {
            if chain.len() != sections[0][l].len() {
                return Err(SkinError::SectionShapeMismatch {
                    section: i,
                    expected: sections[0][l].len(),
                    found: chain.len(),
                    what: "segments",
                });
            }
        }
    }
    let mut walls = Vec::with_capacity(loops);
    let mut kept = Vec::with_capacity(loops);
    let mut params: Option<Vec<f64>> = None;
    for l in 0..loops {
        let mut loop_walls = Vec::with_capacity(sections[0][l].len());
        let mut loop_sections = Vec::with_capacity(sections[0][l].len());
        for j in 0..sections[0][l].len() {
            let raw: Vec<NurbsCurve3<f64>> = sections
                .iter()
                .zip(places)
                .enumerate()
                .map(|(i, (s, place))| segment_curve(i, s[l][j], *place))
                .collect::<Result<_, _>>()?;
            let compat = make_compatible(&raw)?;
            // The v-parameterization is the SURFACE's, not one strip's:
            // taken from the first strip and reused, so every wall of
            // the body agrees on where its sections sit. (Book §10.3
            // averages across control rows for exactly this reason;
            // averaging across strips too would make the sections
            // planar cross-sections of nothing in particular.)
            let p = match &params {
                Some(p) => p.clone(),
                None => {
                    let p = skin_parameters(&compat)?;
                    params = Some(p.clone());
                    p
                }
            };
            loop_walls.push(Arc::new(skin_on(&compat, v_degree, &p)?));
            loop_sections.push(compat);
        }
        walls.push(loop_walls);
        kept.push(loop_sections);
    }
    Ok(LoftGeometry {
        walls,
        section_params: params.unwrap_or_default(),
        sections: kept,
    })
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
/// its tangent there, then translated onto the path. A tangent that
/// vanishes or reverses has no such rotation and refuses typed rather
/// than picking one.
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
    profile: &SectionSegments,
    place: Affine3<f64>,
    path: &NurbsCurve3<f64>,
    stations: usize,
    v_degree: usize,
) -> Result<LoftGeometry, SkinError> {
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
            return Err(SkinError::PathTangentReversal { station: i });
        };
        places.push(Affine3::translation(path.eval(t) - base_point) * turn * place);
    }
    let sections: Vec<SectionSegments> = core::iter::repeat_n(profile.clone(), stations).collect();
    loft_geometry(&sections, &places, v_degree)
}
