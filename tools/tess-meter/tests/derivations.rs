//! The derivations this crate owns, checked where they are: the split
//! optimizer's answer against the certificate it claims to satisfy,
//! its determinism, the ruled-wall case #320 is about, the resolution
//! the split scan's two constants buy, and the CSV's two row shapes
//! agreeing about their width.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use tess_meter::{
    Bound, CSV_HEADER, Chart, FaceRow, NurbsColumns, SPLIT_SCAN_DECADES, SPLIT_SCAN_SAMPLES,
    best_split_cells, best_split_steps, divisions, split_scan_aspects, split_scan_worst_excess,
};
use test_utils::fuzz;

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

/// The bounds the split scan's RANGE is asked to bracket: the ruled
/// wall #320 is about, the degenerate corners where the closed-form
/// interior optimum does not exist, and two ordinary ones.
///
/// **What a member is for.** Each names an aspect ratio the scan must
/// be wide enough to reach; the resolution question is answered in
/// closed form and does not depend on this list, which is why the
/// ceiling below is not fitted to it. The last member carries a live
/// cross term over an anisotropic bound because the four with
/// `muv = 0` have their optimum at exactly `√(muu/mvv)`, which the
/// [`bound`] fixture's own seed already sits on — a family of only
/// those would be measuring the fixture.
const SPLIT_SCAN_FAMILY: [(&str, f64, f64, f64); 6] = [
    ("ruled wall", 0.0, 2.4, 51.3),
    ("isotropic", 10.0, 0.0, 10.0),
    ("mildly anisotropic", 0.1, 0.0, 50.0),
    ("cross term only", 0.0, 5.0, 0.0),
    ("unit", 1.0, 1.0, 1.0),
    ("anisotropic, live cross term", 0.1, 1.0, 50.0),
];

/// The sizing target and the box the family is measured over. Both
/// cancel out of the aspect-ratio optimum and neither cancels out of
/// the one-division floor, which is why they are named rather than
/// inlined.
const FAMILY_DELTA_S: f64 = 1e-3;
const FAMILY_EXTENT: f64 = 1.0;

/// How much relative excess the split column may carry from the scan's
/// own resolution: 0.5%, a tenth of the growth margin the gate that
/// reads it allows.
///
/// **Derived from the consumer, not from today's answer.**
/// `tools/tess-lint` fires when a face's recoverable slack
/// `grid_cells / span_opt_cells` grows past `GROWTH_TOLERANCE` = 1.05
/// against the committed baseline, and that 5% is documented there as
/// the allowance for an honest small mover — a face gaining one grid
/// row — with no noise budget in it. This scan sits in that
/// denominator on BOTH rows, so a static excess cancels and a change
/// in it does not: retuning these two constants alone moves every
/// fresh `span_opt_cells` against a baseline taken at the old
/// resolution, in whichever direction the retune went. Holding the
/// whole of that movement to a tenth of the gate's margin is what this
/// ceiling buys, and `GROWTH_TOLERANCE` is itself boxed to
/// `[1.04, 1.06)` by that crate's own tests, so the quantity it is
/// derived from cannot drift out from under it.
///
/// **It is not a re-pin of the shipped pair.** The closed-form bound
/// at `(8, 321)` is 0.166%, a factor of three below this, and the
/// ceiling admits any sampling step at or under 0.0868 decades — 186
/// samples over 8 decades, or 8 decades widened to 13.9 at the shipped
/// sample count. What it excludes is a step that would let the
/// instrument move an appreciable share of its consumer's verdict.
const RESOLUTION_CEILING: f64 = 0.005;

/// The CONTINUOUS objective at aspect `t`: the cell count
/// [`best_split_cells`] minimizes, with the two `ceil`s of
/// [`divisions`] removed and its one-division floor kept.
///
/// **Written from the certificate, not from the optimizer.** The
/// constraint `muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ δ_s` is
/// homogeneous of degree 2, so at a fixed `t = h_v/h_u` it fixes
/// `h_u`; the cost is then the same extent-over-step product the
/// columns are counted with. Dropping the `ceil`s is the whole point:
/// they are what makes the cell count discontinuous in the scan's two
/// constants, and the quantity left when they go is smooth in the
/// sampling step and in nothing else these constants do not set.
///
/// # Panics
///
/// If the certificate vanishes at `t`. A bound that constrains nothing
/// anywhere has no aspect-ratio optimum to resolve, so it is not a
/// member of this family and a silent answer here would be a
/// measurement of the fallback.
fn continuous_cells(muu: f64, muv: f64, mvv: f64, t: f64) -> f64 {
    let q = mvv.mul_add(t * t, 2.0f64.mul_add(muv * t, muu));
    assert!(
        q > 0.0,
        "a bound that certifies at every step has no aspect-ratio optimum: \
         muu={muu}, muv={muv}, mvv={mvv} at t={t}"
    );
    let hu = (FAMILY_DELTA_S / q).sqrt();
    (FAMILY_EXTENT / hu).max(1.0) * (FAMILY_EXTENT / (t * hu)).max(1.0)
}

/// The true minimum of [`continuous_cells`] over all `t > 0`, by
/// golden-section search on `log t`.
///
/// **Deliberately not a denser scan of the same shape.** The oracle
/// that this row's predecessor used was a lattice containing the
/// subject's lattice as a strict subset, so *reference ≤ subject* held
/// by construction and replacing the reference with the subject left
/// the row green. A bracketing search shares no sample with the scan
/// and converges below any lattice, so the comparison is a claim
/// rather than an identity.
///
/// **Why bracketing is valid here.** In `log t` the cost is
/// non-increasing while the `u` divisions are floored, convex where
/// neither floor binds (`muu/t + 2·muv + mvv·t` in the exponent), and
/// non-decreasing once the `v` divisions are floored — quasiconvex in
/// every case, including the plateau at one cell.
fn continuous_optimum(muu: f64, muv: f64, mvv: f64) -> f64 {
    // Wide enough that the family's optima are interior by orders of
    // magnitude, and narrow enough that `10^x` and `mvv·t²` stay
    // finite.
    const BRACKET_DECADES: f64 = 80.0;
    const ITERATIONS: usize = 300;
    let phi = 0.5 * (5.0f64.sqrt() - 1.0);
    let at = |x: f64| continuous_cells(muu, muv, mvv, 10.0f64.powf(x));
    let (mut lo, mut hi) = (-BRACKET_DECADES, BRACKET_DECADES);
    let (mut c, mut d) = (hi - phi * (hi - lo), lo + phi * (hi - lo));
    let (mut fc, mut fd) = (at(c), at(d));
    let mut best = fc.min(fd);
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
        best = best.min(fc).min(fd);
    }
    best
}

/// The cheapest continuous cost a scan at `(decades, samples)` finds
/// for one bound, and where in the sample run it found it.
///
/// The lane's own steps are NOT admitted here, and that is the
/// difference between measuring the scan and measuring its seed:
/// [`best_split_steps`] starts its running minimum at the grid the
/// lane built, which for a bound with no cross term is already the
/// continuous optimum, so a seeded quantity would score zero on three
/// of this family's six members however badly the scan was tuned.
/// These two constants set the scan's resolution and nothing else, so
/// the scan alone is what they are answerable for.
fn scanned_continuous_cells(
    muu: f64,
    muv: f64,
    mvv: f64,
    decades: f64,
    samples: usize,
) -> (f64, usize) {
    split_scan_aspects(decades, samples)
        .map(|t| continuous_cells(muu, muv, mvv, t))
        .enumerate()
        .fold((f64::INFINITY, 0), |(best, at), (k, n)| {
            if n < best { (n, k) } else { (best, at) }
        })
}

/// **`SPLIT_SCAN_DECADES` and `SPLIT_SCAN_SAMPLES`, boxed on the
/// quantity they set** — the resolution of the aspect-ratio scan — in
/// its two independent directions.
///
/// A wrong pair is invisible from outside: the optimizer still returns
/// a grid and the grid still certifies, it is merely not the cheapest
/// one, which makes `span_opt_cells` — and `tools/tess-lint`'s slack
/// denominator with it — wrong in the direction that flatters the
/// shipped schedule. CI's own sweep cannot see that: a worse scan
/// RAISES `span_opt_cells`, which LOWERS the recoverable slack, and
/// that gate fires only on growth.
///
/// **Three claims, each with one failure mode.**
///
/// 1. **The range brackets the optimum.** The continuous cost is
///    quasiconvex in `log t`, so a scan whose argmin is an ENDPOINT is
///    exactly a scan whose range stops short of the optimum. This is
///    the direction no bound on the sampling step can see, and it is
///    what `DECADES = 2` breaks: the ruled wall's optimum sits at
///    `t ≈ 2.1e-4`, outside `10^±2`.
/// 2. **The step resolves it.** [`split_scan_worst_excess`] is the
///    worst relative excess the step can leave over EVERY bound the
///    range brackets, and it must stay under [`RESOLUTION_CEILING`].
///    This is the direction a family cannot see, since a family is a
///    sample and this is a supremum.
/// 3. **Claim 2 is about this code and not about the theory behind
///    it.** The excess measured on the family never exceeds the closed
///    form. Measured on this tree at the shipped pair: ruled wall
///    0.01706%, isotropic 0%, mildly anisotropic 0.00007%, cross term
///    only 0%, unit 0%, live cross term 0.00005% — against a bound of
///    0.16573%. The bound is ATTAINED rather than generous: an
///    isotropic bound's optimum is `t = 1`, and at an even sample
///    count the lattice straddles it half a step either side, which is
///    why the comparison carries a float allowance.
///
/// **What this deliberately does not do.** It says nothing about the
/// cell count these columns report — that quantity is discontinuous in
/// both constants and cannot carry a tolerance at all
/// (`SPLIT_SCAN_DECADES`' own docs, with the measurement and its
/// witness). And it does not stop the pair being made needlessly FINE,
/// which costs sweep time and no accuracy.
#[test]
fn the_split_scan_resolves_the_aspect_ratios_its_constants_promise() {
    let step_bound = split_scan_worst_excess(SPLIT_SCAN_DECADES, SPLIT_SCAN_SAMPLES);
    assert!(
        step_bound <= RESOLUTION_CEILING,
        "the split scan's sampling step leaves up to {:.4}% on the continuous objective, \
         over the {:.4}% the slack column's gate can absorb — \
         SPLIT_SCAN_DECADES = {SPLIT_SCAN_DECADES} over SPLIT_SCAN_SAMPLES = {SPLIT_SCAN_SAMPLES} \
         is too coarse a step",
        100.0 * step_bound,
        100.0 * RESOLUTION_CEILING
    );
    for (name, muu, muv, mvv) in SPLIT_SCAN_FAMILY {
        let (scanned, at) =
            scanned_continuous_cells(muu, muv, mvv, SPLIT_SCAN_DECADES, SPLIT_SCAN_SAMPLES);
        assert!(
            at > 0 && at + 1 < SPLIT_SCAN_SAMPLES,
            "the split scan's cheapest aspect for the {name} is sample {at} of \
             {SPLIT_SCAN_SAMPLES}, an endpoint — SPLIT_SCAN_DECADES = {SPLIT_SCAN_DECADES} \
             does not reach that bound's optimum"
        );
        let optimum = continuous_optimum(muu, muv, mvv);
        assert!(
            optimum <= scanned,
            "the reference stopped being the better answer on the {name}: \
             {optimum:e} against the scan's {scanned:e}"
        );
        let excess = scanned / optimum - 1.0;
        assert!(
            // The bound is attained, so the allowance is float slack
            // and not a margin.
            excess <= step_bound * (1.0 + 1e-9),
            "the {name} leaves {:.4}% on the continuous objective, over the {:.4}% \
             the sampling step can account for — the scan's excess is no longer \
             explained by its resolution",
            100.0 * excess,
            100.0 * step_bound
        );
    }
}

/// **The guard above can go red, in both of its directions.** A guard
/// nothing has been seen to fail is a claim, and the two perturbations
/// that fail it are cheap enough to keep in the suite rather than
/// leaving them to a reviewer's local edit.
///
/// The numbers are the ones a reader would get by editing the
/// constants: at `DECADES = 2` the ruled wall's cheapest sampled
/// aspect is sample 0 and its excess is 10.44%; at `DECADES = 40` the
/// step's own bound is 4.17%, against a ceiling of 0.5%.
#[test]
fn the_split_scan_guard_reds_on_a_narrow_range_and_on_a_coarse_step() {
    let (_, at) = scanned_continuous_cells(0.0, 2.4, 51.3, 2.0, SPLIT_SCAN_SAMPLES);
    assert_eq!(
        at, 0,
        "claim 1 no longer reds: a 2-decade range should stop short of the ruled wall's optimum"
    );
    let coarse = split_scan_worst_excess(40.0, SPLIT_SCAN_SAMPLES);
    assert!(
        coarse > RESOLUTION_CEILING,
        "claim 2 no longer reds: 40 decades at {SPLIT_SCAN_SAMPLES} samples leaves \
         {:.4}%, which the {:.4}% ceiling should refuse",
        100.0 * coarse,
        100.0 * RESOLUTION_CEILING
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
