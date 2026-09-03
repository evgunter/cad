//! VERBS-OFF-B r1 reviewer probes — adversarial consumer rows for
//! `geom_brep::offset_fit` and the two meters.
//!
//! **What is independent here is what the rows DO to a carrier, not
//! the carrier.** Two of the exact carriers this file used to declare
//! — the quarter cylinder and the sphere band — were
//! character-for-character `offset_fit.rs`'s, control order and
//! weights included, and are now the one `crate::shared::fixture`
//! pair; the header claimed an independence they did not have. The
//! warp, the sign inversion, the tangential slide and the
//! extreme-weight net below are this file's own, and that is where the
//! adversarial content is.
//!
//! What these rows push on, beyond the shipped suite:
//!
//! - **`certify_offset` on a RATIONAL fit**: the composite reads the
//!   fitted net homogeneously, so a fit with non-unit weights is
//!   bounded as the surface it is. The row asserts the sound
//!   direction — a returned certificate's `hull_sup` contains a dense
//!   sample's max — and goes red if the hull limb under-reports.
//! - **The sign witness**: a fit built for `−d` certified against
//!   `+d` must refuse, never report a finite wrong bound.
//! - **A tangentially slid fit**: pushes the `τ` limb of the
//!   rationalized bound; containment or refusal, never under-report.
//! - **Extreme `d` near the certified collapse reach** on the exact
//!   rational sphere band, both the inward edge and a large outward
//!   offset — containment against a dense independent sample.
//! - **An extreme-weight rational base** (weights spanning 20×): the
//!   certificate either refuses or contains a dense sample's max.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsSurface;
use geom_brep::offset_fit::{OffsetFitError, OffsetLimb, certify_offset, fit_offset};
use geom_brep::offset_meters::{OFFSET_METER_LADDER, patch_collapse};
use geom_brep::patch_bound::patch_cells_refined;
use geom_core::Point3;

use crate::shared::fixture::{kv2, quarter_cylinder, sphere_band};
use crate::shared::sample::{grid, worst_offset_residual};
use crate::shared::tol::band;

/// The true residual of `candidate` against the exact offset locus of
/// `base` at `d`, over 41 x 37 stations — counts coprime to every cell
/// schedule in the unit, so the sample never sits where the fitter
/// already looked.
fn sampled_residual(base: &NurbsSurface<f64>, candidate: &NurbsSurface<f64>, d: f64) -> f64 {
    worst_offset_residual(base, candidate, d, &grid(41, 37)).unwrap()
}

/// `certify_offset` accepts an externally supplied fit with ANY
/// weights, and the hull limb reads the fitted net HOMOGENEOUSLY —
/// the composite is homogeneous in `w̃ = w_base·w_fit`, so the surface
/// it bounds is the one that was handed in. The row asserts
/// containment: the certificate's `hull_sup` is at or above a dense
/// sample's max on the supplied surface. A finite bound below it is
/// the one failure the module promises never to produce.
#[test]
fn r1_certify_offset_on_a_rational_fit_never_under_reports() {
    let base = quarter_cylinder(1.0, 1.0);
    let d = 0.3;
    let (fit, _) = fit_offset(&base, d, 1e-4, band()).unwrap();
    // Perturb one interior weight: the surface moves, and so does the
    // homogeneous net the hull limb reads, which is what makes the
    // certificate below a claim about the surface actually supplied.
    let (cu, cv) = fit.control_counts();
    let mut weights = fit.weights().to_vec();
    weights[(cu / 2) * cv + cv / 2] = 2.0;
    let warped = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        fit.control().to_vec(),
        weights,
    )
    .unwrap();
    let true_residual = sampled_residual(&base, &warped, d);
    // A tolerance the warped surface's on-locus samples clear, so the
    // hull limb is the deciding one — and far enough above the bound
    // that limb reaches that the door's own `≤ tolerance` test is not
    // what caps the ratio below. The CEILING is this row's, asserted,
    // and re-taken on every run.
    let tol = true_residual * 8.0;
    let cert = certify_offset(&base, &warped, d, tol, band()).unwrap_or_else(|e| {
        panic!(
            "certify_offset refused a rational fit it can now bound \
             (true residual {true_residual}): {e}"
        )
    });
    // Containment is the sound direction, and it is the direction that
    // a hull limb reading the fit's net flat would break.
    assert!(
        cert.hull_sup >= true_residual,
        "the hull limb UNDER-reports the supplied surface's residual: \
         hull_sup {} < sampled {true_residual}",
        cert.hull_sup
    );
    // **And the ceiling.** `hull_sup ≥ sampled` gets EASIER as the
    // enclosure degrades, so containment alone cannot tell a tight
    // certificate from a useless one. The slack measured on this
    // fixture is 2.49x; the ceiling sits at 5x, i.e. ~2x headroom, so
    // it reds on a real loss of tightness and not on ring noise.
    assert!(
        cert.hull_sup <= true_residual * 5.0,
        "the hull limb's enclosure has degraded: hull_sup {} is more than 5x \
         the sampled residual {true_residual} (measured 2.49x when written)",
        cert.hull_sup
    );
    eprintln!(
        "R1: rational fit certified, hull_sup={:.3e} sampled={true_residual:.3e} \
         (ratio {:.1}x)",
        cert.hull_sup,
        cert.hull_sup / true_residual
    );
}

/// A fit built for `−d`, certified against `+d`: `E·n` carries the
/// wrong sign everywhere, which is exactly what the `D` witness
/// excludes. The door must refuse; a finite certificate here would be
/// a wrong answer.
#[test]
fn r1_a_fit_for_the_wrong_sign_refuses() {
    let base = quarter_cylinder(1.25, 0.75);
    let (fit_neg, _) = fit_offset(&base, -0.3, 1e-3, band()).unwrap();
    match certify_offset(&base, &fit_neg, 0.3, 10.0, band()) {
        Ok(cert) => panic!(
            "a fit of the OPPOSITE offset certified against +d with hull_sup {}",
            cert.hull_sup
        ),
        // The limb and the bound's SHAPE are pinned, not merely the
        // fact of a refusal (issue 1322): a regression that changed
        // which limb spoke, or that refused with a nonsense finite
        // bound attached, is the failure this row exists to catch.
        // `tol = 10.0` is far above the true residual, so limb 1
        // cannot be what speaks — the sign witness fails, the cell
        // answers `+inf`, and limb 2 refuses naming `HullSup`. Same
        // spelling as the sibling row at `offset_fit.rs`'s
        // `a_fit_for_the_wrong_distance_is_refused_by_the_certifying_limb`.
        Err(OffsetFitError::Limb { limb, bound, .. }) => {
            assert_eq!(limb, OffsetLimb::HullSup);
            assert!(
                bound.is_infinite(),
                "the unproved sign witness must answer +inf, not {bound}"
            );
        }
        Err(e) => panic!("refused, but not at the certifying limb: {e}"),
    }
}

/// A tangentially slid fit — the residual is dominated by the `τ`
/// (tangential) limb of the rationalized bound rather than the
/// normal-distance limb. Containment or refusal, never under-report.
#[test]
fn r1_a_slid_fit_s_tau_limb_never_under_reports() {
    let base = quarter_cylinder(1.0, 1.0);
    let d = 0.25;
    let (fit, _) = fit_offset(&base, d, 1e-4, band()).unwrap();
    // Slide the whole net along +z: on a cylinder about z this is
    // purely tangential, so ‖E‖ barely moves while E × m grows.
    let slid_control: Vec<Point3<f64>> = fit
        .control()
        .iter()
        .map(|p| Point3::new(p.x, p.y, p.z + 5e-4))
        .collect();
    let slid = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        slid_control,
        fit.weights().to_vec(),
    )
    .unwrap();
    let true_residual = sampled_residual(&base, &slid, d);
    assert!(
        true_residual >= 4e-4,
        "the slide did not register: {true_residual}"
    );
    match certify_offset(&base, &slid, d, true_residual * 3.0, band()) {
        Err(_) => {}
        Ok(cert) => assert!(
            cert.hull_sup >= true_residual,
            "the slid fit's certified sup {} UNDER-reports the sampled {}",
            cert.hull_sup,
            true_residual
        ),
    }
}

/// Extreme `d`: 90% of the certified inward reach on the exact
/// rational sphere band, and a large outward offset (a sphere never
/// folds outward). Both must certify at a realistic tolerance and
/// contain an independent dense sample of the closed form.
#[test]
fn r1_extreme_d_near_the_collapse_bound_certifies_and_contains() {
    let r = 2.0;
    let base = sphere_band(r, 0.25, 1.25);
    let cells = patch_cells_refined(&base, OFFSET_METER_LADDER[1]).unwrap();
    let reach = patch_collapse(&cells, -1.0).reach;
    assert!(reach > 0.0 && reach.is_finite());
    for d in [-0.9 * reach, 5.0] {
        let tol = 2e-3;
        let (fit, cert) = fit_offset(&base, d, tol, band())
            .unwrap_or_else(|e| panic!("fit_offset refused at d = {d} (reach {reach}): {e}"));
        let mut worst = 0.0f64;
        for (u, v) in grid(41, 37) {
            let p = base.eval(u, v);
            let k = (r + d) / r;
            let want = Point3::new(p.x * k, p.y * k, p.z * k);
            worst = worst.max((fit.eval(u, v) - want).norm());
        }
        assert!(
            worst <= cert.hull_sup,
            "d = {d}: certified sup {} UNDER-reports the closed-form sample {worst}",
            cert.hull_sup
        );
        assert!(worst <= tol, "d = {d}: sampled max {worst} exceeds {tol}");
        eprintln!(
            "extreme d={d:.4} (reach {reach:.4}): cells={} rounds={} hull_sup={:.3e} sampled={worst:.3e}",
            cert.cells, cert.rounds, cert.hull_sup
        );
    }
}

/// An extreme-weight rational base — weights spanning 20× on a
/// bicubic-degree-2 patch. The hull machinery may be arbitrarily
/// conservative here (refusal, or budget exhaustion, is sound); what
/// it may never do is certify a bound below a dense sample's max.
#[test]
fn r1_an_extreme_weight_rational_base_refuses_or_contains() {
    let control: Vec<Point3<f64>> = (0..3)
        .flat_map(|i| {
            (0..3).map(move |j| {
                #[allow(clippy::cast_precision_loss)]
                let (x, y) = (i as f64 * 0.5, j as f64 * 0.5);
                Point3::new(x, y, 0.3 * ((i * 2 + j) as f64).sin())
            })
        })
        .collect();
    let weights = vec![1.0, 0.05, 1.0, 0.4, 1.0, 0.4, 1.0, 0.05, 1.0];
    let base = NurbsSurface::new(kv2(), kv2(), control, weights).unwrap();
    let d = 0.05;
    match fit_offset(&base, d, 5e-4, band()) {
        Err(e) => eprintln!("extreme-weight base refused (sound): {e}"),
        Ok((fit, cert)) => {
            let worst = sampled_residual(&base, &fit, d);
            assert!(
                worst <= cert.hull_sup,
                "extreme-weight base: certified sup {} UNDER-reports sampled {worst}",
                cert.hull_sup
            );
            eprintln!(
                "extreme-weight base: cells={} rounds={} hull_sup={:.3e} sampled={worst:.3e}",
                cert.cells, cert.rounds, cert.hull_sup
            );
        }
    }
}
