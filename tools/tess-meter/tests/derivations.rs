//! The derivations this crate owns, checked where they are: the split
//! optimizer's answer against the certificate it claims to satisfy,
//! its determinism, the ruled-wall case #320 is about, the resolution
//! the split scan's two constants buy, and the CSV's two row shapes
//! agreeing about their width.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use tess_meter::{
    Bound, CSV_HEADER, Chart, FaceRow, NurbsColumns, SPLIT_SCAN_DECADES, SPLIT_SCAN_SAMPLES,
    SplitScan, best_split_cells, best_split_scan, best_split_steps, divisions,
    floored_worst_excess, optimum_is_unfloored, shipped_split_scan_aspects, split_scan,
    split_scan_aspects, unfloored_worst_excess,
};
use test_utils::fuzz;
use test_utils::vacuity::Exposure;

/// A synthetic [`Bound`] with a plausible seed in its `steps`.
///
/// **This is a FIXTURE, not a second copy of the lane's schedule.**
/// `best_split_steps` treats `steps` as an opaque starting point for
/// its running minimum — it never asks where the pair came from — so
/// the properties asserted below (the answer certifies; the reported
/// count matches the reported steps; a ruled wall improves) hold for
/// any seed; this one keeps the RETIRED AM-GM shape as an arbitrary
/// plausible seed (the kernel now reports the aspect-capped
/// selection). The KERNEL-supplied steps are exercised
/// end to end in `rows.rs`, which tessellates a real body and reads
/// `mesh::budget`'s own `patch_steps` / `CellMeasure::steps`.
fn bound(muu: f64, muv: f64, mvv: f64, delta_s: f64) -> Bound {
    let step = |group: f64| {
        if group > 0.0 {
            (delta_s / (2.0 * group)).sqrt()
        } else {
            f64::INFINITY
        }
    };
    Bound {
        muu,
        muv,
        mvv,
        steps: (step(muu + muv), step(mvv + muv)),
    }
}

/// The claim the whole "split slack" column rests on: the grid
/// [`best_split_steps`] picks satisfies the SAME certificate the lane
/// checks, `muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ δ_s`.
///
/// That inequality is the per-triangle bound `Q/4` at the lane's own
/// two-cells-per-axis budgeting (`a_u ≤ 2h_u`, `a_v ≤ 2h_v`), so a
/// grid satisfying it certifies exactly as the shipped one does.
/// Asserted on the ANSWER over random bounds — including the
/// degenerate corners (a ruled direction's `muu = 0`, a zero cross
/// term) where the closed-form optimum does not exist.
#[test]
fn the_cheapest_split_still_satisfies_the_certificate() {
    let mut rng = fuzz::start("tess_meter::cheapest_split");
    // Log-uniform magnitudes, with a zero in each slot often enough to
    // exercise the degenerate corners (a ruled direction's muu = 0, a
    // dead cross term).
    fn mag(r: &mut fuzz::Rng) -> f64 {
        if r.unit() < 0.2 {
            0.0
        } else {
            10.0f64.powf(r.range(-6.0, 4.0))
        }
    }
    for _ in 0..fuzz::scaled(400) {
        let delta_s = 10.0f64.powf(rng.range(-6.0, -1.0));
        let b = bound(mag(&mut rng), mag(&mut rng), mag(&mut rng), delta_s);
        let (du, dv) = (
            10.0f64.powf(rng.range(-2.0, 2.0)),
            10.0f64.powf(rng.range(-2.0, 2.0)),
        );
        let (cells, hu, hv) = best_split_steps(b, du, dv, delta_s);
        // An infinite step means "one division", which is the whole
        // extent — that is what the certificate must be checked at,
        // not at ∞.
        let (hu, hv) = (hu.min(du), hv.min(dv));
        let q = b.muu * hu.powi(2) + 2.0 * b.muv * hu * hv + b.mvv * hv.powi(2);
        assert!(
            q <= delta_s * (1.0 + 1e-9),
            "cheapest split violates the certificate: q={q:e} > delta_s={delta_s:e} \
             at h=({hu:e},{hv:e}) for {b:?} — {}",
            fuzz::replay()
        );
        // …and the count it reports is the count its OWN steps give.
        //
        // NOT `cells <= divisions(du, b.steps.0) * divisions(dv,
        // b.steps.1)`, which was here and could not fail: the scan
        // SEEDS its running minimum with exactly that product
        // (`best_split_steps`), so "never loses to the lane" is a
        // property of the seed, not of the search. What can fail is
        // the tuple going out of step with itself — a count updated
        // without its steps, or vice versa — and that is what a
        // consumer of `(cells, h_u, h_v)` actually relies on. The
        // search's own claim is the certificate assertion above, and
        // that it is STRICTLY better on the anisotropic case, which
        // `a_ruled_wall_pays_for_its_flat_direction` pins.
        let (_, rhu, rhv) = best_split_steps(b, du, dv, delta_s);
        assert_eq!(
            cells.to_bits(),
            (divisions(du, rhu) * divisions(dv, rhv)).to_bits(),
            "the reported cell count is not the count its own steps give, \
             for {b:?} — {}",
            fuzz::replay()
        );
    }
}

/// The scan is a fixed grid of aspect ratios (D9: structure, never a
/// data-dependent iteration), so the answer is reproducible.
#[test]
fn the_split_scan_is_deterministic() {
    let b = bound(0.0, 2.4, 51.3, 1e-3);
    let x = best_split_cells(b, 1.0, 1.0, 1e-3);
    let y = best_split_cells(b, 1.0, 1.0, 1e-3);
    assert_eq!(x.to_bits(), y.to_bits());
}

/// A ruled wall is the case #320 is about: `muu = 0` with a live cross
/// term. The lane's symmetric split charges the cross term to BOTH
/// directions and grids the flat one anyway; the cheapest split spends
/// its divisions where the curvature is.
#[test]
fn a_ruled_wall_pays_for_its_flat_direction() {
    let b = bound(0.0, 2.4, 51.3, 1e-3);
    let lane = divisions(1.0, b.steps.0) * divisions(1.0, b.steps.1);
    let (best, hu, _) = best_split_steps(b, 1.0, 1.0, 1e-3);
    assert!(
        lane / best > 3.0,
        "expected the ruled wall's split slack to be several-fold, got {:.2}x",
        lane / best
    );
    // Asserted on the DIVISIONS, not on `h_u` itself. The objective is
    // a step function of the aspect ratio, so which sample wins is a
    // property of the scan's lattice: refining the scan moves the
    // winning `h_u` between 1.02 and 0.34 while the answer improves,
    // and an `h_u >= 1.0` line therefore reds on scans that are
    // strictly better. What is stable — and what the finding is about
    // — is that the cheapest split spends an order of magnitude fewer
    // divisions on the flat direction than the lane does.
    let (best_u, lane_u) = (divisions(1.0, hu), divisions(1.0, b.steps.0));
    assert!(
        best_u * 10.0 <= lane_u,
        "the cheapest split should barely divide the FLAT direction: \
         {best_u} divisions against the lane's {lane_u}"
    );
}

/// Which of the continuous objective's two shapes a family member has.
/// Declared per member and CHECKED — the claims below dispatch on it,
/// so a member silently changing class would move which claim it
/// answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    /// The optimum is the interior stationary point `t* = √(muu/mvv)`,
    /// strictly above `divisions`' one-division floor — the domain
    /// [`unfloored_worst_excess`] bounds.
    Unfloored,
    /// The optimum is a KINK on that floor. Outside the closed form's
    /// domain; measured, not bounded.
    Floored,
    /// No optimum at all: the cost is constant in `t` over an interval
    /// wider than the scan can resolve, so there is nothing to bracket
    /// and nothing to resolve.
    Flat,
}

/// The bounds the split scan is measured on: `(name, shape, muu, muv, mvv)`.
///
/// **What a member is for.** Each names an aspect ratio the scan must be
/// wide enough to reach and close enough to land near. The resolution
/// question on the [`Shape::Unfloored`] members is answered in closed
/// form and does not depend on this list, which is why the ceilings
/// below are not fitted to it; on the [`Shape::Floored`] members there
/// is no closed form and the list IS the evidence, which is why the two
/// counterexamples that exposed the missing certificate are members.
///
/// **The two degeneracies, stated because they were not obvious.**
/// `anisotropic, live cross term` shares `t* = √(muu/mvv)` with
/// `mildly anisotropic` and therefore lands on the same sample: on THIS
/// quantity it distinguishes nothing. It is kept because it is
/// load-bearing on the CELL COUNT, where it scores 5.8824% at the
/// shipped pair — it is `S160`'s sixth family member, whose deletion
/// with #783's instrument is what made that finding's table
/// unreproducible from the tree. And `cross term only` is
/// [`Shape::Flat`]: its cost is exactly `U·V·2·muv/δ_s` on the whole
/// plateau where neither floor binds, so its argmin is float dust and
/// the range claim says nothing about it.
const SPLIT_SCAN_FAMILY: [(&str, Shape, f64, f64, f64); 8] = [
    ("ruled wall", Shape::Floored, 0.0, 2.4, 51.3),
    ("isotropic", Shape::Unfloored, 10.0, 0.0, 10.0),
    ("mildly anisotropic", Shape::Unfloored, 0.1, 0.0, 50.0),
    ("cross term only", Shape::Flat, 0.0, 5.0, 0.0),
    ("unit", Shape::Unfloored, 1.0, 1.0, 1.0),
    (
        "anisotropic, live cross term",
        Shape::Unfloored,
        0.1,
        1.0,
        50.0,
    ),
    (
        "floored, cross-term-free",
        Shape::Floored,
        2.9808e-4,
        0.0,
        1.9437e-2,
    ),
    ("floored ruled wall", Shape::Floored, 0.0, 2.4, 3716.36),
];

/// The sizing target and the box the family is measured over. Both
/// cancel out of the aspect-ratio optimum and neither cancels out of
/// the one-division floor — which is the whole distinction
/// [`Shape`] draws — so they are named rather than inlined.
const FAMILY_DELTA_S: f64 = 1e-3;
const FAMILY_EXTENT: f64 = 1.0;

/// `tess-lint`'s `GROWTH_TOLERANCE`, read out of its source text.
///
/// **Read rather than transcribed, and read the way this file already
/// reads that crate** (`the_lints_expected_header_is_this_one`, which
/// argues the case at length): `tess-meter` must not DEPEND on
/// `tess-lint` — that crate's manifest states dependency-freedom as its
/// design — but the ceilings below are derived from this number, and a
/// derivation from a transcribed constant is a transcription. It is a
/// real pin and it is ugly: it breaks if that declaration is
/// reformatted rather than changed, which is the same bargain the
/// header pin makes.
fn lint_growth_tolerance() -> f64 {
    let lint = include_str!("../../tess-lint/src/lib.rs");
    let decl = lint
        .split("pub const GROWTH_TOLERANCE: f64 = ")
        .nth(1)
        .expect("tess-lint declares GROWTH_TOLERANCE");
    let end = decl.find(';').expect("the declaration ends");
    decl[..end]
        .trim()
        .parse()
        .expect("GROWTH_TOLERANCE is a float literal")
}

/// The margin the split column's consumer allows before it calls a
/// movement a finding: `GROWTH_TOLERANCE − 1`, i.e. 5%.
///
/// **Why this is the quantity the instrument answers to.**
/// `tools/tess-lint` fires when a face's recoverable slack
/// `grid_cells / span_opt_cells` grows past `GROWTH_TOLERANCE` against
/// the committed baseline, and that margin is documented at its own site
/// as the allowance for an honest small mover — a face gaining one grid
/// row — with no noise budget in it. This scan sits in that denominator
/// on BOTH rows, so a static excess cancels exactly and a CHANGE in it
/// does not: retuning these constants alone moves every fresh
/// `span_opt_cells` against a baseline taken at the old resolution, and
/// so does a face whose bound moves to a different point relative to the
/// lattice. `GROWTH_TOLERANCE` is boxed to `[1.04, 1.06)` by that
/// crate's own tests, so this cannot drift out from under the row.
fn growth_margin() -> f64 {
    lint_growth_tolerance() - 1.0
}

/// What the scan may leave on the class where the excess IS bounded —
/// a tenth of [`growth_margin`], so the analytically controlled part of
/// the instrument's error is negligible against its consumer.
///
/// **This is a ceiling on [`unfloored_worst_excess`] and on nothing
/// else.** It is not a bound on the instrument's total error: the
/// floored class is not covered by any closed form here, and its
/// measured worst is nearly three times this. Saying "a tenth of the
/// margin" about the whole error would be the claim this row was sent
/// back for.
///
/// **Not a re-pin of the shipped pair.** The bound at `(8, 321)` is
/// 0.16573%, a factor of three below; the ceiling admits any sampling
/// step at or under 0.0868 decades — 186 samples over 8 decades, or 8
/// decades widened to 13.9 at the shipped sample count.
fn unfloored_ceiling() -> f64 {
    growth_margin() / 10.0
}

/// What any member may leave on the CONTINUOUS objective, floored or
/// not: the consumer's whole margin.
///
/// **Read the objective in that sentence, because the consequence
/// attaches to the other one.** Every claim in this row is stated on
/// the continuous objective — the cost with `divisions`' two `ceil`s
/// removed — and `span_opt_cells`, the column `tools/tess-lint`
/// actually divides by, is the `ceil`'d one. On the continuous
/// objective the worst member at the shipped pair is
/// `floored, cross-term-free` at 2.088%, which is 42% of this margin:
/// the same order as the gate's tolerance, not an order below it. On
/// the `ceil`'d objective the instrument is already OVER it —
/// `anisotropic, live cross term` scores **5.8824%**, and along a
/// single smooth geometry change (`mvv` scaled 1× to 100×, counts in
/// the thousands, not a small-count corner) the scan-to-true ratio runs
/// from 1.00000 to **1.0588**. So the sentence *"the meter's own
/// resolution can move a face across its consumer's threshold"* is
/// TRUE, today, of `span_opt_cells` — it is not a risk this ceiling
/// holds off, and the ceiling below does not claim to.
///
/// **What that ceiling is for**: the continuous excess is what the two
/// constants govern smoothly, so it is what a guard on them can box.
/// The `ceil` quantisation on top is the lever recorded at
/// `SPLIT_SCAN_DECADES`, and moving it is not this row's work.
fn total_ceiling() -> f64 {
    growth_margin()
}

/// [`divisions`] with its `ceil` deleted and nothing else changed — the
/// CONTINUOUS objective's counting function.
///
/// The `ceil` is what makes the cell count discontinuous in the scan's
/// two constants; the quantity left when it goes is smooth in the
/// sampling step and in nothing else those constants do not set. The
/// one-division floor STAYS: a grid cannot have less than one division,
/// and dropping it too would move the ruled wall's optimum to `t → 0`
/// and change which quantity is being measured.
fn continuous_divisions(extent: f64, h: f64) -> f64 {
    assert!(h > 0.0, "a grid step of {h} is not a reading");
    assert!(
        extent.is_finite() && extent >= 0.0,
        "an extent of {extent} is not a reading"
    );
    (extent / h).max(1.0)
}

/// A family member as a [`Bound`] with no usable seed — the scan is
/// what is under test, so nothing may pre-empt it.
///
/// The steps are `NaN` on purpose rather than absent: `Bound` carries
/// them, and a path that read this seed by mistake would panic in
/// [`divisions`] instead of quietly winning the running minimum. That
/// matters here more than usual, because the fixture seed of the
/// neighbouring rows IS the continuous optimum whenever `muv = 0` or
/// `muu = mvv` — four of this family's eight members — so a seeded
/// quantity would score exactly zero on them however badly the scan
/// were tuned.
fn unseeded(muu: f64, muv: f64, mvv: f64) -> Bound {
    Bound {
        muu,
        muv,
        mvv,
        steps: (f64::NAN, f64::NAN),
    }
}

/// The shipped scan's answer on one member, in the continuous count.
///
/// **It goes through [`split_scan`] and [`shipped_split_scan_aspects`],
/// which is the point.** An earlier version of this row re-spelled the
/// optimizer's `Q(t)`, its step derivation and its lattice here; it
/// boxed the two constants and could not see the scan's CALL SITE
/// changing under them — `(SPLIT_SCAN_DECADES, 21)` inflated the
/// reported cell count by 12.73%, two and a half times the gate's whole
/// margin, with every row in this file green.
fn scanned(muu: f64, muv: f64, mvv: f64) -> SplitScan {
    split_scan(
        unseeded(muu, muv, mvv),
        FAMILY_EXTENT,
        FAMILY_EXTENT,
        FAMILY_DELTA_S,
        shipped_split_scan_aspects(),
        None,
        continuous_divisions,
    )
}

/// The true minimum of the continuous cost over all `t > 0`, by
/// golden-section search on `log t`.
///
/// **Deliberately not a denser scan of the same shape.** The oracle
/// this row's predecessor used was a lattice containing the subject's
/// lattice as a strict subset, so *reference ≤ subject* held by
/// construction and replacing the reference with the subject left the
/// row green. A bracketing search shares no sample with the scan and
/// converges below any lattice, so the comparison is a claim rather
/// than an identity.
///
/// **Why bracketing is valid here.** In `log t` the cost is
/// non-increasing while the `u` divisions are floored, convex where
/// neither floor binds (`muu/t + 2·muv + mvv·t` in the exponent), and
/// non-decreasing once the `v` divisions are floored — quasiconvex in
/// every case, including the plateau at one cell.
fn continuous_optimum(muu: f64, muv: f64, mvv: f64) -> (f64, f64) {
    // Wide enough that the family's optima are interior by orders of
    // magnitude, and narrow enough that `10^x` and `mvv·t²` stay finite.
    const BRACKET_DECADES: f64 = 80.0;
    const ITERATIONS: usize = 300;
    let phi = 0.5 * (5.0f64.sqrt() - 1.0);
    let at = |x: f64| {
        let t = 10.0f64.powf(x);
        let q = mvv.mul_add(t * t, 2.0f64.mul_add(muv * t, muu));
        assert!(
            q > 0.0,
            "a bound that certifies at every step has no aspect-ratio optimum: \
             muu={muu}, muv={muv}, mvv={mvv} at t={t}"
        );
        let hu = (FAMILY_DELTA_S / q).sqrt();
        continuous_divisions(FAMILY_EXTENT, hu) * continuous_divisions(FAMILY_EXTENT, t * hu)
    };
    let (mut lo, mut hi) = (-BRACKET_DECADES, BRACKET_DECADES);
    let (mut c, mut d) = (hi - phi * (hi - lo), lo + phi * (hi - lo));
    let (mut fc, mut fd) = (at(c), at(d));
    let mut best = if fc <= fd { (fc, c) } else { (fd, d) };
    for _ in 0..ITERATIONS {
        if fc < fd {
            hi = d;
            d = c;
            fd = fc;
            c = hi - phi * (hi - lo);
            fc = at(c);
        } else {
            lo = c;
            c = d;
            fc = fd;
            d = lo + phi * (hi - lo);
            fd = at(d);
        }
        for (v, x) in [(fc, c), (fd, d)] {
            if v < best.0 {
                best = (v, x);
            }
        }
    }
    best
}

/// Whether the continuous optimum sits ON [`divisions`]' one-division
/// floor, read off the argmin the golden section already found.
///
/// **The second, independent answer to the question
/// [`optimum_is_unfloored`] answers**, and the reason there are two: a
/// family member's [`Shape`] decides which claim it answers, so
/// checking that declaration against the one predicate the claims also
/// dispatch on would be checking it against itself. This one searches
/// for the optimum instead of solving for it, and reads the division
/// counts where it lands.
fn optimum_sits_on_the_floor(muu: f64, muv: f64, mvv: f64) -> bool {
    let (_, x) = continuous_optimum(muu, muv, mvv);
    let t = 10.0f64.powf(x);
    let q = mvv.mul_add(t * t, 2.0f64.mul_add(muv * t, muu));
    let hu = (FAMILY_DELTA_S / q).sqrt();
    let (nu, nv) = (FAMILY_EXTENT / hu, FAMILY_EXTENT / (t * hu));
    nu <= 1.0 + 1e-9 || nv <= 1.0 + 1e-9
}

/// **The shipped optimizer IS the shipped scan** — asserted on the
/// COMPOSITION, because boxing the constants and checking the lattice
/// helper both leave the call site free.
///
/// [`best_split_scan`] is supposed to be `split_scan` over
/// [`shipped_split_scan_aspects`], counted with [`divisions`] and
/// seeded with the lane's own grid. Nothing about the constants, and
/// nothing about the helper, says that it is: three retunes at that one
/// call site — a sample count of its own, a range of its own, a dropped
/// seed — passed `fmt`, `clippy -D warnings` and every other row in
/// this file. How far the sample-count retune alone moves the reported
/// cell count was measured once and is stated once, at
/// [`best_split_scan`]'s own docs, together with the fact
/// that nothing re-takes it; it is not restated here, because a
/// measurement written in two places is two things to keep in step and
/// this row is the half that is meant to stay live.
///
/// So this row spells the intended composition itself and compares bit
/// for bit, **sample index included** — the seed retune is the one that
/// can leave `cells` untouched, because on a bound whose lane grid ties
/// with the best sampled aspect the answer is the same number reached
/// from a different place.
///
/// The family is what makes the comparison bite: the ruled wall's
/// optimum is at `t ≈ 2.1e-4`, so a narrowed range moves its answer;
/// its `ceil`'d count moves by 12.73% at 21 samples; and the isotropic
/// bound's lane grid ties with sample 160, so the seed wins there and
/// dropping it moves `sample` from `None` to `Some`.
#[test]
fn the_shipped_optimizer_is_the_shipped_scan() {
    let mut seen = Exposure::new("shipped composition");
    for (name, _, muu, muv, mvv) in SPLIT_SCAN_FAMILY {
        let b = bound(muu, muv, mvv, FAMILY_DELTA_S);
        let want = split_scan(
            b,
            FAMILY_EXTENT,
            FAMILY_EXTENT,
            FAMILY_DELTA_S,
            shipped_split_scan_aspects(),
            Some(b.steps),
            divisions,
        );
        let got = best_split_scan(b, FAMILY_EXTENT, FAMILY_EXTENT, FAMILY_DELTA_S);
        assert_eq!(
            (
                got.cells.to_bits(),
                got.steps.0.to_bits(),
                got.steps.1.to_bits()
            ),
            (
                want.cells.to_bits(),
                want.steps.0.to_bits(),
                want.steps.1.to_bits()
            ),
            "best_split_scan is no longer the shipped scan on the {name}: \
             {got:?} against {want:?}"
        );
        assert_eq!(
            got.sample, want.sample,
            "best_split_scan reaches its answer from a different place on the {name}: \
             sample {:?} against {:?} — a retuned seed can leave the count identical",
            got.sample, want.sample
        );
        match got.sample {
            Some(_) => seen.note("a scanned aspect won"),
            None => seen.note("the lane's own seed won"),
        }
    }
    seen.report();
    seen.require_each(
        &["a scanned aspect won", "the lane's own seed won"],
        1,
        "the comparison can only see a dropped seed on a bound where the seed WINS, \
         and only see a retuned lattice on a bound where a sample does",
    );
}

/// **`SPLIT_SCAN_DECADES` and `SPLIT_SCAN_SAMPLES`, boxed on the
/// quantity they set** — the resolution of the aspect-ratio scan — and
/// the scan's own call site with them.
///
/// A wrong pair is invisible from outside: the optimizer still returns a
/// grid and the grid still certifies, it is merely not the cheapest one,
/// which makes `span_opt_cells` — and `tools/tess-lint`'s slack
/// denominator with it — wrong in the direction that flatters the
/// shipped schedule. CI's own sweep cannot see that: a worse scan RAISES
/// `span_opt_cells`, which LOWERS the recoverable slack, and that gate
/// fires only on growth.
///
/// **Four claims, each with one failure mode.**
///
/// 0. **The lattice under test is the shipped one.** The count and the
///    ends of `shipped_split_scan_aspects` are the constants' own, so a
///    second lattice reaching `best_split_steps` reds here rather than
///    passing under a boxed pair.
/// 1. **The range brackets the optimum.** The continuous cost is
///    quasiconvex in `log t`, so a scan whose argmin is an ENDPOINT is
///    exactly a scan whose range stops short. `DECADES = 2` breaks it:
///    the ruled wall's optimum is at `t ≈ 2.1e-4`, outside `10^±2`.
///    Skipped on the [`Shape::Flat`] member, which has no optimum to
///    bracket — and its flatness is asserted rather than assumed.
/// 2. **The step resolves the class that admits a closed form.**
///    [`unfloored_worst_excess`] must stay under [`unfloored_ceiling`],
///    and each [`Shape::Unfloored`] member must stay under the closed
///    form. This is the direction a family cannot see, since a family is
///    a sample and the closed form is a supremum — and it is ATTAINED,
///    so the per-member comparison carries a float allowance rather than
///    a margin.
/// 3. **Each [`Shape::Floored`] member stays inside
///    [`floored_worst_excess`], and every member stays inside the
///    consumer's whole margin on the continuous objective.** The kink
///    derivation is what makes the first half a bound rather than a
///    measurement; read [`total_ceiling`] for which objective the second
///    half is about, because the gate reads the other one.
///
/// **Measured on this tree at the shipped pair** (continuous excess,
/// unseeded — the seeded column `S160` published is a different
/// quantity and is not this row's evidence): ruled wall 0.01706%,
/// isotropic 0%, mildly anisotropic 0.00007%, cross term only 0%, unit
/// 0%, live cross term 0.00005%, floored cross-term-free **2.08824%**,
/// floored ruled wall **1.15256%**. The two bounds at that pair:
/// 0.16573% unfloored, 2.09180% floored — and
/// `floored, cross-term-free` sits at `r = 0.29808`, which is the kink
/// derivation's analytic argmax, so the family carries the class's
/// worst case rather than a sample of it.
///
/// **What this deliberately does not do.** It says nothing about the
/// cell count these columns report — that quantity is discontinuous in
/// both constants and cannot carry a tolerance at all
/// (`SPLIT_SCAN_DECADES`' own docs). And it does not stop the pair being
/// made needlessly FINE, which costs sweep time and no accuracy.
#[test]
fn the_split_scan_resolves_the_aspect_ratios_its_constants_promise() {
    let mut seen = Exposure::new("split scan resolution");
    // Claim 0.
    let aspects: Vec<f64> = shipped_split_scan_aspects().collect();
    assert_eq!(
        aspects.len(),
        SPLIT_SCAN_SAMPLES,
        "the shipped scan visits {} aspects, not SPLIT_SCAN_SAMPLES = {SPLIT_SCAN_SAMPLES} — \
         the lattice under test is not the one the constants describe",
        aspects.len()
    );
    for (end, want) in [
        (aspects[0], -SPLIT_SCAN_DECADES),
        (aspects[SPLIT_SCAN_SAMPLES - 1], SPLIT_SCAN_DECADES),
    ] {
        assert!(
            (end.log10() - want).abs() < 1e-9,
            "the shipped scan ends at 10^{}, not 10^{want} — the lattice under test is \
             not the one SPLIT_SCAN_DECADES describes",
            end.log10()
        );
    }
    // Claim 2, the family-free half.
    let closed = unfloored_worst_excess(SPLIT_SCAN_DECADES, SPLIT_SCAN_SAMPLES);
    let kinked = floored_worst_excess(SPLIT_SCAN_DECADES, SPLIT_SCAN_SAMPLES);
    assert!(
        closed <= unfloored_ceiling(),
        "the split scan's sampling step leaves up to {:.5}% on the unfloored class, \
         over the {:.5}% a tenth of the slack gate's margin allows — \
         SPLIT_SCAN_DECADES = {SPLIT_SCAN_DECADES} over SPLIT_SCAN_SAMPLES = \
         {SPLIT_SCAN_SAMPLES} is too coarse a step",
        100.0 * closed,
        100.0 * unfloored_ceiling()
    );
    for (name, shape, muu, muv, mvv) in SPLIT_SCAN_FAMILY {
        let b = unseeded(muu, muv, mvv);
        let is_unfloored = optimum_is_unfloored(b, FAMILY_EXTENT, FAMILY_EXTENT, FAMILY_DELTA_S);
        assert_eq!(
            is_unfloored,
            shape == Shape::Unfloored,
            "the {name} is declared {shape:?} and the closed-form floor test disagrees"
        );
        if shape != Shape::Flat {
            assert_eq!(
                optimum_sits_on_the_floor(muu, muv, mvv),
                shape == Shape::Floored,
                "the {name} is declared {shape:?} and the SEARCHED argmin disagrees — \
                 the declaration is what picks which claim this member answers, so it \
                 is checked against a second derivation and not only against the \
                 predicate the claims dispatch on"
            );
        }
        let scan = scanned(muu, muv, mvv);
        let at = scan.sample.expect("an unseeded scan answers with a sample");
        if shape == Shape::Flat {
            // Asserted, not assumed: a member excused from the range
            // claim has to earn it.
            let mid = SPLIT_SCAN_SAMPLES / 2;
            let a = scanned_cost_at(muu, muv, mvv, aspects[mid]);
            let c = scanned_cost_at(muu, muv, mvv, aspects[mid + 1]);
            assert!(
                (a / c - 1.0).abs() < 1e-12,
                "the {name} is declared Flat and its cost moves between adjacent \
                 samples: {a:e} against {c:e}"
            );
            seen.note("flat: no optimum to bracket");
        } else {
            assert!(
                at > 0 && at + 1 < SPLIT_SCAN_SAMPLES,
                "the split scan's cheapest aspect for the {name} is sample {at} of \
                 {SPLIT_SCAN_SAMPLES}, an endpoint — SPLIT_SCAN_DECADES = \
                 {SPLIT_SCAN_DECADES} does not reach that bound's optimum"
            );
        }
        let (optimum, _) = continuous_optimum(muu, muv, mvv);
        assert!(
            optimum <= scan.cells,
            "the reference stopped being the better answer on the {name}: \
             {optimum:e} against the scan's {:e}",
            scan.cells
        );
        let excess = scan.cells / optimum - 1.0;
        // A member whose optimum sits ON a sample scores zero however
        // the scan is tuned, so it is the members that do NOT that
        // carry the resolution claim. The threshold is float dust and
        // not a margin: the smallest real excess in this family is
        // 5e-7, five decades above it.
        let off_lattice = excess > 1e-10;
        match shape {
            Shape::Unfloored => {
                seen.note("unfloored: bounded in closed form");
                if off_lattice {
                    seen.note("unfloored, optimum off the lattice");
                }
                assert!(
                    // The closed form is attained, so the allowance is
                    // float slack and not a margin.
                    excess <= closed * (1.0 + 1e-9),
                    "the {name} leaves {:.5}% on the continuous objective, over the \
                     {:.5}% the sampling step can account for — the scan's excess is no \
                     longer explained by its resolution",
                    100.0 * excess,
                    100.0 * closed
                );
            }
            Shape::Floored => {
                seen.note("floored: bounded by the kink derivation");
                if off_lattice {
                    seen.note("floored, optimum off the lattice");
                }
                assert!(
                    excess <= kinked * (1.0 + 1e-9),
                    "the {name} leaves {:.5}% on the continuous objective, over the \
                     {:.5}% the kink derivation admits — the scan's excess on the \
                     floored class is no longer explained by its resolution",
                    100.0 * excess,
                    100.0 * kinked
                );
            }
            Shape::Flat => {}
        }
        assert!(
            excess <= total_ceiling(),
            "the {name} leaves {:.5}% on the CONTINUOUS objective, over the {:.5}% \
             the slack gate's whole margin allows. The gate reads the `ceil`'d \
             objective, where the instrument is already over that margin \
             (SPLIT_SCAN_DECADES' docs); this row bounds the part the two constants \
             govern smoothly, and that part has stopped being negligible",
            100.0 * excess,
            100.0 * total_ceiling()
        );
    }
    seen.report();
    seen.require_each(
        &[
            "unfloored, optimum off the lattice",
            "floored, optimum off the lattice",
        ],
        1,
        "a family whose every optimum sits on a sample scores zero however badly \
         the scan is tuned, which measures the lattice's luck and not the \
         resolution",
    );
    seen.require_each(
        &[
            "unfloored: bounded in closed form",
            "floored: bounded by the kink derivation",
        ],
        2,
        "both shapes of the objective have to be under test: the closed form covers \
         one of them and the ruled wall this scan exists for is in the other",
    );
}

/// The continuous cost at one aspect, for the flatness assertion.
fn scanned_cost_at(muu: f64, muv: f64, mvv: f64, t: f64) -> f64 {
    let q = mvv.mul_add(t * t, 2.0f64.mul_add(muv * t, muu));
    let hu = (FAMILY_DELTA_S / q).sqrt();
    continuous_divisions(FAMILY_EXTENT, hu) * continuous_divisions(FAMILY_EXTENT, t * hu)
}

/// **The guard above can go red, in both of its directions.** A guard
/// nothing has been seen to fail is a claim, and the two perturbations
/// that fail it are cheap enough to keep in the suite rather than
/// leaving them to a reviewer's local edit.
///
/// The numbers are the ones a reader gets by editing the constants: at
/// `DECADES = 2` the ruled wall's cheapest sampled aspect is sample 0
/// and the floored ruled wall leaves 665.99%; at `DECADES = 40` the
/// closed form is 4.17078% against a ceiling of 0.5%.
#[test]
fn the_split_scan_guard_reds_on_a_narrow_range_and_on_a_coarse_step() {
    let narrow = split_scan(
        unseeded(0.0, 2.4, 51.3),
        FAMILY_EXTENT,
        FAMILY_EXTENT,
        FAMILY_DELTA_S,
        split_scan_aspects(2.0, SPLIT_SCAN_SAMPLES),
        None,
        continuous_divisions,
    );
    assert_eq!(
        narrow.sample,
        Some(0),
        "claim 1 no longer reds: a 2-decade range should stop short of the ruled \
         wall's optimum"
    );
    let coarse = unfloored_worst_excess(40.0, SPLIT_SCAN_SAMPLES);
    assert!(
        coarse > unfloored_ceiling(),
        "claim 2 no longer reds: 40 decades at {SPLIT_SCAN_SAMPLES} samples leaves \
         {:.5}%, which the {:.5}% ceiling should refuse",
        100.0 * coarse,
        100.0 * unfloored_ceiling()
    );
}

/// The empty-tail arm and the filled arm must agree about the row's
/// width, or every consumer's column indices are off by the
/// difference.
#[test]
fn both_row_shapes_have_the_headers_width() {
    let cols = CSV_HEADER.split(',').count();
    let plane = FaceRow {
        face: 0,
        chart: Chart::Plane,
        delta: 1e-3,
        triangles: 2,
        nurbs: None,
    };
    assert_eq!(plane.csv_row("s/b").split(',').count(), cols);
    let nurbs = FaceRow {
        chart: Chart::Nurbs,
        nurbs: Some(NurbsColumns {
            u: (0.0, 1.0),
            v: (0.0, 1.0),
            nu: 4.0,
            nv: 5.0,
            muu: 1.0,
            muv: 2.0,
            mvv: 3.0,
            mu1: 1.5,
            mv1: 2.5,
            cells: 6,
            grid_cells: 12.0,
            patch_cells: 20.0,
            opt_cells: 10.0,
            span_opt_cells: 8.0,
            worst_cert: 1e-4,
            worst_dev: 5e-5,
            dev_samples: 7,
            bands: 3,
            cap_bands: 1,
            snap_bands: 0,
            realized_aspect: 4.2,
        }),
        ..plane
    };
    assert_eq!(nurbs.csv_row("s/b").split(',').count(), cols);
}

/// The header this crate writes and the one `tools/tess-lint` parses
/// are the same bytes.
///
/// **Why source text and not a shared constant.** The obvious answer —
/// a schema-only crate both depend on — would break the property
/// `tess-lint`'s manifest states as its design: *"It has no
/// dependencies at all: it reads the CSV … and the kernel does not
/// appear even as a dev-dependency, its fixtures are CSV text, which
/// is the whole contract between the two halves."* A consumer that
/// shares a constant with its producer can no longer fail as a
/// PARSER when the producer's schema moves, which is the failure mode
/// the lint wants. So the two declarations stay independent and this
/// test reads the other one's source.
///
/// It is a real pin and it is ugly: it parses Rust string
/// continuations out of a sibling crate's `lib.rs`, and it breaks if
/// that declaration is reformatted rather than changed.
#[test]
fn the_lints_expected_header_is_this_one() {
    let lint = include_str!("../../tess-lint/src/lib.rs");
    let quoted = lint
        .split("pub const EXPECTED_HEADER: &str = ")
        .nth(1)
        .expect("tess-lint declares EXPECTED_HEADER");
    let end = quoted.find(';').expect("the declaration ends");
    // Rust string continuations: drop the backslash-newline-indent runs.
    let mut header = String::new();
    let mut chars = quoted[..end].chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => {}
            '\\' => {
                while chars.peek().is_some_and(|c| c.is_whitespace()) {
                    chars.next();
                }
            }
            c => header.push(c),
        }
    }
    assert_eq!(header, CSV_HEADER);
}
