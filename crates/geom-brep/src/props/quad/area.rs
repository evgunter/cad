//! **The rectangle lanes' area**: the shared midpoint–Taylor rule
//! ([`area_midpoint_taylor`]) over a UV rectangle, and the A2 area gauge
//! with its certified chord-perimeter lower bound
//! ([`boundary_chord_perimeter_lo`]).

use geom_core::interval::certification::Certification;
use geom_core::interval::{Interval, certified_hi};
use geom_core::spline::ParamRange;

use super::cells::knot_aligned_cuts;
use super::net::{PatchRect, RVec3, widen};
use crate::props::bracket::{lo_or_refuse, pt};

/// Cells per knot span, per axis, of BOTH rectangle lanes' area pass
/// ([`knot_aligned_cuts`]). The shared [`area_midpoint_taylor`] rule is
/// O(h), so the resolution sets the area's honest width directly.
pub(super) const AREA_CELLS_PER_SPAN: usize = 32;
/// Chords per knot span of a rectangle side in the area gauge's
/// certified perimeter lower bound ([`boundary_chord_perimeter_lo`]).
/// Sixteen is enough that a wrapped patch's collapsed corner chords
/// stop dominating the bound.
pub(super) const PERIMETER_CHORDS_PER_SPAN: usize = 16;
/// The A2 area gauge's ceiling: the mean edge displacement that
/// explains the certified area bracket, as a multiple of the face's
/// own certified perimeter ([`area_gauge_ok`] carries the calibration
/// and the margin). A TRIPWIRE, deliberately far above the corpus.
const AREA_GAUGE_CEILING: f64 = 1.0;
/// The fallback relative gauge's ceiling — `area.width()/area.lo()`,
/// read only where no positive certified perimeter is reachable
/// ([`area_gauge_ok`]).
const AREA_GAUGE_REL_CEILING: f64 = 1.0e3;

/// A certified LOWER bound on the length of the closed traversal of a
/// UV rectangle's image boundary: a chord polygon inscribed in it.
///
/// A curve is at least as long as any polygon inscribed in it, and
/// ADDING a vertex can only lengthen that polygon (triangle
/// inequality) — so the bound is sound at any schedule and monotone
/// in refinement. Every difference and root is formed in certification arithmetic, so
/// `lo()` is outward-rounded and the bound survives its own
/// arithmetic.
///
/// **The schedule is knot-aware, and that is a soundness property,
/// not a tightness one.** A uniform-in-parameter schedule can ALIAS:
/// a boundary oscillating commensurately with the sample count is
/// sampled only at its zero crossings and the polygon reads a
/// straight line. Measured on an analytic `sin(2πku)` edge at 16
/// uniform chords per side: 1.16x..1.37x understatement at k = 5..7,
/// and a COLLAPSE at k ≡ 0 mod 8 (k = 8 reads 4.000 against a true
/// 66.07 — 16.5x under). The gauge divides by this number, so an
/// understatement shrinks the ceiling as its SQUARE — 273x at k = 8,
/// which is more than the whole calibrated margin. **An understated
/// bound makes the tripwire fire on honest geometry, and
/// `debug-assertions` are on in release — so this direction is a
/// release panic, not a missed catch.**
///
/// Three things remove it, in order of what they cover:
///
/// 1. **The knot lines are in the schedule** ([`knot_aligned_cuts`],
///    the same cut rule the area grid uses). Through the patch lanes
///    the evaluator is a spline over these knots, and a spline cannot
///    oscillate faster than its own spans: with every span cut, the
///    geometry's structure can no longer align with the sampling
///    grid. This is what makes the reachable case safe.
/// 2. **Two uniform grids at coprime counts** are unioned in. A
///    grid of `n` collapses on a `sin` boundary when `k ≡ 0 mod n/2`;
///    two counts with no common factor collapse together only at the
///    product. This covers an arbitrary evaluator — including one
///    with no knots at all, which is how the probes drive it.
/// 3. **The coarse block edges**, which [`knot_aligned_cuts`] already
///    contributes.
///
/// **What it bounds, exactly.** The traversal walks the rectangle's
/// four sides in order, so on a WRAPPED patch (whose `u0` and `u1`
/// edges are one physical curve) it crosses the seam TWICE and the
/// result exceeds the face's own perimeter. That is the numbing
/// direction — an inflated denominator only makes the ceiling more
/// generous and the tripwire quieter — never the firing one. It is a
/// lower bound on the FACE's perimeter only where that boundary is
/// embedded; it is a lower bound on the traversal always.
///
/// It bounds the image of the RECTANGLE, not of the true trim
/// boundary; the departure is the caller's certified
/// `boundary_defect`.
///
/// **The result is clamped at zero.** Interval arithmetic sum's lower endpoint
/// rounds outward, so a face whose chords are all sub-ulp returns a
/// small NEGATIVE number (measured: −3.16e-322 on a 1e-12 face at
/// offset 1e6). Zero is an equally valid, weaker certified lower
/// bound on a length, and it routes the gauge to its relative arm
/// rather than dividing by a negative.
pub fn boundary_chord_perimeter_lo(
    rect: (f64, f64, f64, f64),
    samples: usize,
    knots: (&[f64], &[f64]),
    eval: impl Fn(f64, f64) -> RVec3,
) -> f64 {
    let (u0, u1, v0, v1) = rect;
    let n = samples.max(1);
    // Coprime by construction: consecutive integers share no factor.
    let schedule = |lo: f64, hi: f64, kv: &[f64]| -> Vec<f64> {
        let mut c = knot_aligned_cuts(lo, hi, n, kv);
        c.extend(knot_aligned_cuts(lo, hi, n + 1, kv));
        c.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));
        c.dedup();
        c
    };
    let cu = schedule(u0, u1, knots.0);
    let cv = schedule(v0, v1, knots.1);
    if cu.len() < 2 || cv.len() < 2 {
        return 0.0;
    }
    // Each side contributes its vertices EXCEPT its last, which is the
    // next side's first — so the closed walk visits each corner once.
    let mut pts: Vec<RVec3> = Vec::with_capacity(2 * (cu.len() + cv.len()));
    for &u in &cu[..cu.len() - 1] {
        pts.push(eval(u, v0));
    }
    for &v in &cv[..cv.len() - 1] {
        pts.push(eval(u1, v));
    }
    for &u in cu[1..].iter().rev() {
        pts.push(eval(u, v1));
    }
    for &v in cv[1..].iter().rev() {
        pts.push(eval(u0, v));
    }
    let mut p = Interval::zero();
    for i in 0..pts.len() {
        let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
        let d = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
        p = p + (d[0].sqr() + d[1].sqr() + d[2].sqr()).sqrt();
    }
    lo_or_refuse(p).max(0.0)
}

/// **The A2 area gauge** — the certified area bracket's width read as
/// a MEAN EDGE DISPLACEMENT, against a generous ceiling.
///
/// `A2 = area.width() / perimeter_lo` is a LENGTH: how far the face's
/// boundary would have to move, on average, for the area enclosure's
/// width to be geometry rather than quadrature slack. It is the
/// direct analogue of the flux funnel's
/// [`mean_boundary_displacement`](super::run::mean_boundary_displacement), which reads `flux.width()` over
/// the area lever; here the area's own width is read over a certified
/// perimeter LOWER bound ([`boundary_chord_perimeter_lo`]), so the
/// quotient never understates the displacement.
///
/// The ceiling is `AREA_GAUGE_CEILING · perimeter_lo`, i.e. the
/// dimensionless test `area.width() ≤ AREA_GAUGE_CEILING ·
/// perimeter_lo²`. The statement it makes is
///
/// > the area uncertainty, expressed as a displacement of this face's
/// > boundary, does not exceed the face's own perimeter.
///
/// A face that fails that is not making a geometry statement, which
/// is why this is a `debug_assert` of the D2 addendum's ROW-5
/// BOUNDARY class (expensive checks whose failure PROBABLY indicates
/// a bug), not a refusal: no typed behaviour rides on it, an
/// input-reachable width is reported with the measurement's stop, and
/// the honest answer to
/// a wide-but-sound bracket is that it is sound (S-CERT Q1, ruled
/// 2026-08-29: no always-on area metering, no funnel target).
///
/// # Why it is not scale-free, quite
///
/// A width is an area and the perimeter a length, so the ratio is
/// dimensionless and a body remodelled at another scale reads the
/// same gauge — down to the point where interval arithmetic's own rounding
/// stops being scale-free. Below that, a sub-ulp face's chords
/// vanish, `perimeter_lo` clamps to zero and the face exits to the
/// relative arm. So: scale-free over the range where the enclosure
/// arithmetic is, with a documented exit below it — not scale-free by
/// construction.
///
/// # THE CALIBRATION RECORD — the one home for these figures
///
/// Every other site that discusses this calibration — the two
/// acceptance rows in `sweep/tests/` and the chart probe in
/// `geom-brep/tests/` — points HERE rather than restating a digit. CERT-5 moved these
/// numbers once and the restatements went stale; the fix pass's
/// re-measurement moved them again.
///
/// Re-measured on the FIX-PASS head by tracing every patch-lane
/// certified return across the `geom-brep`, `sweep`, `topo`,
/// `editor-core` and `step-import` suites (instrumentation reverted).
/// **344 certified returns**, gauged as `area.width()/denominator²`:
///
/// | | min | p50 | p90 | p99 | max |
/// |---|---|---|---|---|---|
/// | 343 on the perimeter arm | 2.60e-14 | 1.77e-3 | 3.31e-3 | 1.26e-2 | **1.906e-2** |
///
/// With `AREA_GAUGE_CEILING = 1.0` that is **52.5x** headroom. The
/// margin is then stated at three harder places than the corpus
/// median, because a corpus statistic is not a margin:
///
/// - **Door-authored geometry.** A five-section, non-affine,
///   8x-scale-ramped loft authored through the public `loft_body`
///   door — nobody's tuned fixture — reads **1.26e-2**, i.e. the
///   corpus p99, for **79x** headroom. Under the OLD chord-only
///   denominator the same body read 3.94e-2, above the whole corpus;
///   absorbing that is what the denominator change bought, and even
///   there the headroom was 25x.
/// - **The widest honest bracket the file produces**, which does not
///   certify: dm1's wall (`stepcode/dm1-id-214.stp`, `FaceKey(3v3)`)
///   exits at the round budget carrying width 5.2477e-4 over a
///   denominator of 8.2832e-2 — **7.65e-2**, for **13.1x**. One more
///   schedule round certifies it (issue 1315), so a ceiling that hugged
///   the certified population would turn that round into a landmine.
/// - **The relative arm** below is no longer uncovered: a patch whose
///   whole rectangle boundary collapses to a point certifies with real
///   interior area and a zero-length traversal, reading
///   `width/lo = 56.8` against `AREA_GAUGE_REL_CEILING = 1e3` — **17.6x**,
///   and one constructed witness is the whole of that arm's coverage.
///
/// Both arms are ε-invariant in VALUE: the patch lanes build the area
/// enclosure once, before the round loop, and neither gauge consults a
/// tolerance. What ε moves is which faces REACH the assert, and a
/// looser ε converges a superset of a tighter one.
///
/// The cautionary half of the `closing_column` precedent
/// (`mesh::walk`) is why the margin is stated at those three places
/// rather than at the corpus: its recorded estimate was nine orders
/// off on issue 723's input. Nine orders is also the scale this
/// ceiling is FOR — a corruption that size clears it by 10⁷ even from
/// the widest honest row.
///
/// # What the gauge cannot see
///
/// - It reads a bracket's WIDTH, never its correctness. A
///   wrong-but-narrow enclosure is invisible; containment is the test
///   suites' row, and issue 873's coupling row on the cylinder arm is
///   what pins the two brackets to each other.
/// - The denominator bounds the image of the RECTANGLE, not of the
///   true trim boundary; the departure is the caller's certified
///   `boundary_defect`.
/// - It sits at certified returns only. A refusing lane is allowed to
///   be arbitrarily wide — that is what the refusal says.
/// - A boundary oscillating faster than the chord schedule resolves is
///   under-measured by the chord half of the denominator; the caller's
///   perimeter is what backstops that, and where a caller supplies a
///   number that is neither (issue 1368) the gauge is running on the
///   chord bound alone.
fn area_gauge_ok(area: Interval, perimeter_lo: f64) -> bool {
    let width = area.width();
    // A certified return whose width is not a number is the clearest
    // form of the bug this tripwire is for — and so is a NaN
    // PERIMETER. The two are the same signal and get the same answer:
    // falling back to the relative arm on a NaN denominator would
    // quietly turn a bug into a softer test.
    if !width.is_finite() || !perimeter_lo.is_finite() {
        return false;
    }
    if perimeter_lo > 0.0 {
        width <= AREA_GAUGE_CEILING * perimeter_lo.powi(2)
    } else if lo_or_refuse(area) > 0.0 {
        // FALLBACK, and it is LIVE: a patch whose whole rectangle
        // boundary collapses to a point certifies with real interior
        // area and a zero-length traversal, which is how a face
        // reaches this arm. The relative gauge on the
        // certified-conservative endpoint is issue 472's named one.
        width <= AREA_GAUGE_REL_CEILING * lo_or_refuse(area)
    } else {
        // Neither gauge has a denominator. Not reachable from the
        // lanes — the face-extent gate certifies `area.lo() > 0`
        // before any call site gets here — but reachable, and tested,
        // from constructed inputs, which is why it is an arm rather
        // than an `unreachable!`.
        true
    }
}

/// The A2 gauge as the patch lanes spend it: one call per certified
/// return ([`area_gauge_ok`] carries the calibration).
///
/// The `cfg!` guard keeps the perimeter bound — the only part of this
/// that costs anything — out of a build that compiles the assertion
/// away, and the perimeter arrives as a thunk for the same reason.
/// The number the gauge is entitled to divide by: the LARGER of the
/// chord bound and the caller's own perimeter.
///
/// **The weighing this encodes** (the two denominators fail in
/// opposite directions, and the directions are not symmetric):
///
/// - An UNDERSTATED denominator shrinks the ceiling as its square and
///   makes the tripwire fire on honest geometry. `debug-assertions`
///   are on in release, so that is a RELEASE PANIC on valid input —
///   which breaks this assertion class's own contract, that its
///   absence never changes shipped semantics.
/// - An OVERSTATED denominator only makes the tripwire quieter. The
///   ceiling already sits far above the corpus and is aimed at
///   bug-scale widths, so even a 10x overstatement leaves orders of
///   detection headroom. This is the monotone-wrong direction, and
///   it is the direction every other guard on the area gauge errs.
///
/// The costs are not comparable, so the denominator must never be too
/// small — and neither input can be trusted alone to guarantee that.
/// The chord bound is certified but can be beaten by a boundary that
/// oscillates faster than any fixed schedule resolves (Nyquist, not a
/// schedule defect: knot-aligning the samples does not help when the
/// knots are themselves the zero crossings — measured, 11x under at
/// 64 spans). The caller's perimeter is built from carrier lengths
/// and control-polygon lengths and so over-estimates in production,
/// but issue 1368 measured 10 of 333 corpus returns where it does
/// not, two of them passing `0.0` outright.
///
/// Taking the maximum is robust to both: an over-large caller
/// perimeter only numbs, an under-small one is ignored, and a chord
/// collapse is backstopped. The gauge therefore no longer claims to
/// divide by a certified lower bound — it claims the weaker and more
/// useful thing, that the displacement it reports is never LARGER
/// than the truth, so a fire is never an artifact of the denominator.
///
/// A non-finite chord bound is not backstopped: that is a refused
/// enclosure, the bug signal itself, and it propagates so the
/// assertion fires.
fn area_gauge_denominator(perimeter_lo: f64, perimeter_caller: f64) -> f64 {
    if !perimeter_lo.is_finite() {
        return f64::NAN;
    }
    if perimeter_caller.is_finite() && perimeter_caller > perimeter_lo {
        perimeter_caller
    } else {
        perimeter_lo
    }
}

/// What a fired gauge says. The two arms fail for different reasons
/// and a message that described only the perimeter arm reported
/// "perimeter is at least 0 — displacement inf" whenever the RELATIVE
/// arm was the one that decided.
fn area_gauge_failure_message(area: Interval, denominator: f64) -> String {
    let width = area.width();
    let head = "the certified area bracket is wider than the A2 tripwire allows. The area rule \
                is O(h) and wide by design, but not by this much: at a CERTIFIED return a width \
                this large means the area pass, its cell tiling or its pad lost the geometry, \
                not that the face is hard. `area_gauge_ok` carries the calibration.";
    if !width.is_finite() || !denominator.is_finite() {
        return format!(
            "{head}\nARM: refused — width {width} over denominator {denominator}. A \
             non-finite value at a certified return is the bug signal itself."
        );
    }
    if denominator > 0.0 {
        format!(
            "{head}\nARM: perimeter. Width {width} on a face whose perimeter is at least \
             {denominator} — a mean edge displacement of {}, which is {}x that perimeter \
             against a ceiling of {AREA_GAUGE_CEILING}x.",
            width / denominator,
            width / denominator.powi(2)
        )
    } else {
        format!(
            "{head}\nARM: relative (the face's whole boundary traversal has zero certified \
             length, so there is no perimeter to divide by). Width {width} against a \
             certified lower area of {} — a relative width of {}, against a ceiling of \
             {AREA_GAUGE_REL_CEILING}.",
            lo_or_refuse(area),
            width / lo_or_refuse(area)
        )
    }
}

/// The A2 gauge as the patch lanes spend it: one call per certified
/// return ([`area_gauge_ok`] carries the calibration).
///
/// The `cfg!` guard keeps the perimeter bound — the only part of this
/// that costs anything — out of a build that compiles the assertion
/// away, and the perimeter arrives as a thunk for the same reason.
/// The message is built only on failure.
#[inline]
pub(super) fn debug_assert_area_gauge(
    area: Interval,
    perimeter_caller: f64,
    perimeter_lo: &dyn Fn() -> f64,
) {
    if cfg!(debug_assertions) {
        let denominator = area_gauge_denominator(perimeter_lo(), perimeter_caller);
        debug_assert!(
            area_gauge_ok(area, denominator),
            "{}",
            area_gauge_failure_message(area, denominator)
        );
    }
}

/// The window between two adjacent cuts of a [`PatchRect`] side.
/// [`knot_aligned_cuts`] of a finite range is a sorted list of numbers,
/// so the pair is ordered and neither end is NaN.
pub(super) fn cut_window(a: f64, b: f64) -> ParamRange {
    ParamRange::new(a, b)
        .unwrap_or_else(|| unreachable!("adjacent cuts of a finite range are ordered numbers"))
}

/// One cell of the shared area grid: its closed extent per axis and the
/// midpoint the rule evaluates at (carried rather than re-derived, so
/// every lane's midpoint is the SAME float).
#[derive(Clone, Copy)]
pub(super) struct AreaBox {
    pub(super) u: ParamRange,
    pub(super) umid: f64,
    pub(super) v: ParamRange,
    pub(super) vmid: f64,
}

/// What a lane reports for one area cell: the THIN midpoint value of
/// the area integrand `g = |S_u×S_v|`, one-sided Lipschitz bounds on
/// its two partials over the cell, and the hull of `g` over the whole
/// cell.
pub(super) struct AreaCell {
    /// `g(midpoint)` — thin, and the reason this rule beats the hull
    /// rule by orders.
    pub(super) g_mid: Interval,
    /// An upper bound on `sup_cell |∂_u g|`.
    pub(super) g_u: f64,
    /// An upper bound on `sup_cell |∂_v g|`.
    pub(super) g_v: f64,
    /// An enclosure of `g` over the whole cell. Both this and the
    /// padded midpoint enclose the cell's MEAN of `g`, so the rule
    /// takes their intersection: the hull needs no smoothness and is
    /// a magnitude (hence never negative), which is what keeps a cell
    /// whose derivative hulls have blown up from dragging the area
    /// bracket across zero; the padded midpoint is orders tighter
    /// wherever the derivative hulls are sane. Neither is a fallback
    /// for the other.
    pub(super) g_hull: Interval,
}

/// **The area rule both patch lanes use**: a fixed-resolution composite
/// midpoint rule with a first-order Taylor (Lipschitz) pad,
///
/// ```text
/// ∫∫_cell g ∈ A_cell·( g(m) ± [ (h_u/2)·G_u + (h_v/2)·G_v ] ),
///     G_d ⊇ sup_cell |∂_d g|
/// ```
///
/// which needs only that `g` be LIPSCHITZ on the cell — so the kink of
/// `|·|` at a vanishing cross product is covered, and the lane's
/// `|∂_d |c|| ≤ |∂_d c|` bound is exactly the Lipschitz constant.
///
/// The one thing it does NOT cover is a genuine JUMP: at an interior
/// knot of multiplicity ≥ degree the surface is only C⁰ and `S_u`
/// jumps, so `g` jumps and no derivative hull bounds the step. The
/// cells are therefore cut ON the interior knots
/// ([`knot_aligned_cuts`]), which removes the case rather than
/// bounding it: a knot on a cell boundary is in no cell's interior,
/// every cell is a smoothness island, and the Lipschitz pad is valid
/// throughout. The alternative — the smoothness-free hull rule
/// `A_cell·hull(g)` on the straddling cells — is what this rule used
/// to fall back to, and it does not converge: the hull is a
/// control-net fact and those are span-granular, so on a FIXED
/// resolution it is a permanent addend proportional to the off-grid
/// knot count. The flux path takes the same cuts for the same
/// reason.
///
/// Why not the plain hull rule everywhere: every hull the patch lanes
/// read off a net is a control-net convexity fact, and those are
/// SPAN-GRANULAR, so on a single-span patch the per-cell hull IS the whole-patch hull and the
/// rule never tightens (the unit square came out `[≈0, 10.4]`). Here
/// only the pad carries hulls, so the width is genuinely O(h).
///
/// The area is a certified DENOMINATOR (the +V meter and the extent
/// gate) — `boundary_defect` pads it directly.
pub(super) fn area_midpoint_taylor<E>(
    rect: PatchRect,
    n: usize,
    boundary_defect: f64,
    knots: (&[f64], &[f64]),
    mut cell: impl FnMut(AreaBox) -> Result<AreaCell, E>,
) -> Result<Interval, E> {
    let (u0, u1, v0, v1) = rect.corners();
    let cuts_u = knot_aligned_cuts(u0, u1, n, knots.0);
    let cuts_v = knot_aligned_cuts(v0, v1, n, knots.1);
    let mut acc = Interval::zero();
    for iu in 0..cuts_u.len() - 1 {
        let (c_ulo, c_uhi) = (cuts_u[iu], cuts_u[iu + 1]);
        let hu = c_uhi - c_ulo;
        for iv in 0..cuts_v.len() - 1 {
            let (c_vlo, c_vhi) = (cuts_v[iv], cuts_v[iv + 1]);
            let hv = c_vhi - c_vlo;
            let c = cell(AreaBox {
                u: cut_window(c_ulo, c_uhi),
                umid: c_ulo.midpoint(c_uhi),
                v: cut_window(c_vlo, c_vhi),
                vmid: c_vlo.midpoint(c_vhi),
            })?;
            // The cell width as an ENCLOSURE, not a rounded float:
            // the cells share their cut points, so they tile the
            // rectangle exactly, and the outward subtraction is what
            // keeps the arithmetic over that tiling enclosing too.
            let cell_area = (pt(c_uhi) - pt(c_ulo)) * (pt(c_vhi) - pt(c_vlo));
            let mean = widen(c.g_mid, 0.5 * hu * c.g_u + 0.5 * hv * c.g_v)
                .clamped_to(lo_or_refuse(c.g_hull), certified_hi(c.g_hull));
            acc = acc + cell_area * mean;
        }
    }
    Ok(widen(acc, boundary_defect))
}
