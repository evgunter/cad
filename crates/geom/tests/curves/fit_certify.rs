//! The M5 PR 4 sup-norm certification story, end to end (spec §d
//! acceptance): fit a NURBS to a KNOWN locus sample set, compose its
//! implicit residual, hull-bound it, and verify (i) the bound is sound
//! vs dense sampling, (ii) a fit whose Type-2 loop stopped at bound B
//! has a hull-certified residual consistent with B's order, and (iii)
//! the **OQ2 standing pin**: a PLANTED between-samples excursion
//! (one corrupted control point after fitting) is caught by the hull
//! bound while a 9-point sampling schedule misses it entirely — the
//! argument that made hull bounds an entry requirement (PR 2 measured
//! 59% schedule invisibility; this pins one concrete case in CI
//! forever).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom::NurbsCurve3;
use geom_core::Point3;
use geom_core::spline::compose::{self, CurveRingData, ImplicitSurface};

const R: f64 = 2.5;

fn center() -> Point3<f64> {
    Point3::new(0.5, 1.0, -0.25)
}

/// Quarter-arc samples of the circle (radius `R`, xy-plane at
/// `z = −0.25`) — the known locus the fit approximates.
fn arc_samples(n: usize) -> Vec<Point3<f64>> {
    let c = center();
    (0..n)
        .map(|k| {
            #[allow(clippy::cast_precision_loss)]
            let theta = core::f64::consts::FRAC_PI_2 * (k as f64) / ((n - 1) as f64);
            let (s, co) = theta.sin_cos();
            Point3::new(c.x + R * co, c.y + R * s, c.z)
        })
        .collect()
}

/// The sphere-limb implicit residual at f64 — the oracle, independent
/// of the ring pipeline.
fn sphere_residual(p: Point3<f64>) -> f64 {
    let c = center();
    let d = p - c;
    d.dot(d) - R * R
}

/// The composed, hull-certified sup bound on the curve's sphere
/// residual (meters²) — the one-call certification story.
fn hull_bound(curve: &NurbsCurve3<f64>) -> f64 {
    let c = center();
    let coords = curve.ring_coords();
    let data = CurveRingData::new(curve.knots(), curve.weights(), &coords).unwrap();
    let form = compose::implicit_composite(
        &data,
        &ImplicitSurface::Sphere {
            center: [c.x, c.y, c.z],
            radius: R,
        },
    )
    .unwrap();
    form.sup_bound()
}

/// Max |sphere residual| over `n + 1` uniform parameter samples.
fn sampled_max(curve: &NurbsCurve3<f64>, n: u32) -> f64 {
    let mut worst = 0.0f64;
    for k in 0..=n {
        let t = f64::from(k) / f64::from(n);
        worst = worst.max(sphere_residual(curve.eval(t)).abs());
    }
    worst
}

#[test]
fn fit_then_certify_and_the_oq2_planted_excursion_pin() {
    // --- The fit (the module-doc worked example) ------------------
    let pts = arc_samples(65);
    let tol = 1e-2;
    let fit = NurbsCurve3::approximate(&pts, 3, tol).unwrap();
    assert!(fit.bound <= tol);
    let fit_hull = hull_bound(&fit.curve);
    let fit_dense = sampled_max(&fit.curve, 4096);

    // (i) Soundness: the hull bound dominates dense sampling.
    assert!(
        fit_dense <= fit_hull,
        "dense max {fit_dense:e} exceeds hull bound {fit_hull:e}"
    );

    // (ii) Order consistency with the loop's achieved bound B: a curve
    // within B of the chord interpolant (itself within the chordal
    // sagitta of the locus) has |sphere residual| ≲ 2R·(B + sagitta);
    // the hull bound may pay conservatism on coarse spans but must
    // stay within a small factor of that order.
    let sagitta = {
        let dtheta = core::f64::consts::FRAC_PI_2 / 64.0;
        R * dtheta * dtheta / 8.0
    };
    let expected_scale = 2.0 * R * (fit.bound + sagitta);
    println!(
        "[fit+certify] achieved fit bound B = {:.3e} m (tolerance {tol:.0e}), \
         control {} from {} samples, refit = {}; sphere-residual hull bound \
         {fit_hull:.3e} m² vs dense max {fit_dense:.3e} m² (expected scale \
         2R(B+sag) = {expected_scale:.3e} m²)",
        fit.bound,
        fit.curve.knots().control_count(),
        pts.len(),
        fit.refit_applied,
    );
    // NOTE pin (adversarial review): WHICH refit-skip path fires in
    // this worked example — the compressed structure's C0 corners
    // leave interior controls without data support, so the refit
    // normal system is degenerate and the certified pipeline curve is
    // returned. If structure selection ever changes this, the pin
    // makes the change loud.
    assert!(!fit.refit_applied);
    assert_eq!(
        fit.refit_skip,
        Some(geom::RefitSkip::DegenerateSystem),
        "worked example's refit-skip path moved"
    );
    assert!(
        fit_hull <= 4.0 * expected_scale,
        "hull bound {fit_hull:e} is out of scale with the fit bound \
         {:e} (expected order {expected_scale:e})",
        fit.bound
    );

    // --- The OQ2 pin (spec §d.iii) --------------------------------
    // Interpolate (dense structure: narrow basis supports), certify,
    // then corrupt ONE control point whose support lies strictly
    // between two adjacent 9-point schedule sites.
    let interp = NurbsCurve3::interpolate(&pts, 3).unwrap();
    let honest_hull = hull_bound(&interp);
    let honest_schedule = sampled_max(&interp, 8);

    // Find a control index whose cubic support (u_i, u_{i+p+1}) fits
    // strictly inside the schedule gap (3/8, 1/2).
    let kv = interp.knots();
    let p = kv.degree();
    let idx = (0..kv.control_count())
        .find(|i| kv.knots()[*i] > 0.375 + 0.01 && kv.knots()[i + p + 1] < 0.5 - 0.01)
        .expect("a between-schedule support exists at this density");
    let mut control = interp.control().to_vec();
    let c = center();
    let radial = control[idx] - c;
    let delta = 0.05; // meters — a real excursion, far above the band
    control[idx] = control[idx] + radial * (delta / radial.norm());
    let corrupted = NurbsCurve3::new(kv.clone(), control, interp.weights().to_vec()).unwrap();

    let corrupted_hull = hull_bound(&corrupted);
    let corrupted_schedule = sampled_max(&corrupted, 8);
    let corrupted_dense = sampled_max(&corrupted, 4096);

    println!(
        "[OQ2 pin] honest interpolant: hull {honest_hull:.3e} m², 9-point schedule \
         {honest_schedule:.3e} m². Corrupted control {idx} (support \
         ({:.4}, {:.4}), radial +{delta} m): hull {corrupted_hull:.3e} m², \
         9-point schedule {corrupted_schedule:.3e} m², dense max \
         {corrupted_dense:.3e} m².",
        kv.knots()[idx],
        kv.knots()[idx + p + 1],
    );

    // The certification band (meters²): the honest curve passes it by
    // hull bound — the strongest certificate — with margin.
    let band = 1e-2;
    assert!(honest_hull <= band, "honest hull {honest_hull:e}");
    assert!(honest_schedule <= band);

    // The planted excursion is REAL (dense sampling sees it) …
    assert!(
        corrupted_dense > band,
        "planted excursion invisible even to dense sampling"
    );
    // … the 9-point schedule MISSES it (bit-identical to honest at
    // every schedule site: the corrupted support avoids them all) …
    assert_eq!(
        corrupted_schedule.to_bits(),
        honest_schedule.to_bits(),
        "schedule saw the corruption — support placement broke"
    );
    assert!(corrupted_schedule <= band);
    // … and the hull bound CATCHES it.
    assert!(
        corrupted_hull > band,
        "hull bound {corrupted_hull:e} missed the planted excursion"
    );
    // Soundness on the corrupted curve too.
    assert!(corrupted_dense <= corrupted_hull);
}
