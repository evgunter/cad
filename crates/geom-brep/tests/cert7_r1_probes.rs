//! CERT-7 review lane R1 — adversarial probes (local, not for merge).
//!
//! Nothing here is a shipped row: these exist to attack PR 1319's
//! claims by execution.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use geom::NurbsSurface;
use geom_brep::offset_fit::{
    OffsetFitError, approx_offset_surface, certify_offset, fit_offset, offset_point,
};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn kv2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}

fn kv1() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}

/// A quarter **ellipse** wall: the exact rational quadratic quarter
/// ellipse (the affine image of the classical arc, so the same
/// weights) extruded linearly in `z`. Rational in `u`, and NOT a
/// surface this suite ships anywhere.
fn elliptic_wall(a: f64, b: f64, h: f64) -> NurbsSurface<f64> {
    let s = (FRAC_PI_2 * 0.5).cos();
    let control = vec![
        Point3::new(a, 0.0, 0.0),
        Point3::new(a, 0.0, h),
        Point3::new(a, b, 0.0),
        Point3::new(a, b, h),
        Point3::new(0.0, b, 0.0),
        Point3::new(0.0, b, h),
    ];
    NurbsSurface::new(kv2(), kv1(), control, vec![1.0, 1.0, s, s, 1.0, 1.0]).unwrap()
}

/// An **ellipse of revolution** wall band: an elliptical meridian arc
/// (rational quadratic) revolved through a quarter turn (rational
/// quadratic). Rational in BOTH directions — the case where `w`,
/// `w_u` and `w_v` are all live in `M̃`.
fn ellipsoid_band(a: f64, b: f64) -> NurbsSurface<f64> {
    let s = (FRAC_PI_2 * 0.5).cos();
    // Meridian in (r, z): quarter ellipse from (a, 0) to (0, b) via
    // the tangent-intersection point (a, b) with weight s.
    let meridian = [((a, 0.0), 1.0), ((a, b), s), ((0.0, b), 1.0)];
    // Quarter turn in longitude: (r, 0) -> (r, r) -> (0, r), weight s.
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for ((r, z), wm) in meridian {
        for (fx, fy, wl) in [(1.0, 0.0, 1.0), (1.0, 1.0, s), (0.0, 1.0, 1.0)] {
            control.push(Point3::new(r * fx, r * fy, z));
            weights.push(wm * wl);
        }
    }
    // u = meridian, v = longitude; row-major iu*cv + iv matches.
    NurbsSurface::new(kv2(), kv2(), control, weights).unwrap()
}

fn dense() -> Vec<(f64, f64)> {
    let (nu, nv) = (23usize, 19usize);
    let mut out = Vec::new();
    for i in 0..nu {
        for j in 0..nv {
            #[allow(clippy::cast_precision_loss)]
            out.push((i as f64 / (nu - 1) as f64, j as f64 / (nv - 1) as f64));
        }
    }
    out
}

fn sampled(base: &NurbsSurface<f64>, fit: &NurbsSurface<f64>, d: f64) -> f64 {
    let mut worst = 0.0f64;
    for (u, v) in dense() {
        let Some(t) = offset_point(base, d, u, v) else {
            return f64::INFINITY;
        };
        worst = worst.max((fit.eval(u, v) - t).norm());
    }
    worst
}

/// **E2E, rational bases nobody shipped, through the public doors.**
/// The storage door mints and re-derives; the re-derivation door is
/// then handed the minted fit back. Containment is asserted at every
/// station.
#[test]
fn e2e_rational_bases_through_the_storage_and_recertify_doors() {
    let cases: Vec<(&str, NurbsSurface<f64>, f64, f64)> = vec![
        ("elliptic wall a=2 b=1", elliptic_wall(2.0, 1.0, 1.0), 0.1, 1e-3),
        ("elliptic wall a=5 b=1 (hostile aspect)", elliptic_wall(5.0, 1.0, 1.0), 0.05, 1e-2),
        ("ellipsoid band a=1 b=1 (a sphere octant)", ellipsoid_band(1.0, 1.0), 0.2, 1e-3),
        ("ellipsoid band a=2 b=1", ellipsoid_band(2.0, 1.0), 0.1, 1e-2),
    ];
    for (name, base, d, tol) in cases {
        match approx_offset_surface(std::sync::Arc::new(base.clone()), d, tol, band()) {
            Ok(geom::Surface::Approx(a)) => {
                let cert = a.certificate();
                let worst = sampled(&base, a.fit(), d);
                assert!(
                    cert.hull_sup >= worst,
                    "{name}: stored hull_sup {} UNDER-reports sampled {worst}",
                    cert.hull_sup
                );
                // Hand the minted fit back through the re-derivation door.
                let again = certify_offset(&base, a.fit(), d, tol, band()).unwrap();
                assert!((again.hull_sup - cert.hull_sup).abs() <= 0.0, "{name}: re-derivation moved");
                eprintln!(
                    "E2E {name}: cells={} rounds={} hull_sup={:.4e} sampled={worst:.4e} \
                     (ratio {:.2}x)",
                    cert.cells,
                    cert.rounds,
                    cert.hull_sup,
                    cert.hull_sup / worst
                );
            }
            Ok(other) => panic!("{name}: not an Approx: {other:?}"),
            Err(e) => eprintln!("E2E {name}: REFUSED typed: {e}"),
        }
    }
}

/// **The projective-scale probe.** Multiplying every weight of a fit
/// by one constant leaves the surface identical, so a certificate
/// that reads the net homogeneously must not move. A door that read
/// the net flat would move by that constant.
#[test]
fn a_global_weight_scale_does_not_move_the_certificate() {
    let base = elliptic_wall(1.0, 1.0, 1.0);
    let d = 0.2;
    let (fit, _) = fit_offset(&base, d, 1e-4, band()).unwrap();
    let worst = sampled(&base, &fit, d);
    let tol = worst * 8.0;
    let mut first = f64::NAN;
    for k in [1.0_f64, 1e-6, 1.0e6, 1.0e12] {
        let scaled = NurbsSurface::new(
            fit.knots_u().clone(),
            fit.knots_v().clone(),
            fit.control().to_vec(),
            fit.weights().iter().map(|w| w * k).collect(),
        )
        .unwrap();
        let s = sampled(&base, &scaled, d);
        assert!(
            (s - worst).abs() <= 1e-12 * worst.max(1.0),
            "k={k}: the scaled surface is not the same surface ({s} vs {worst})"
        );
        match certify_offset(&base, &scaled, d, tol, band()) {
            Ok(c) => {
                assert!(c.hull_sup >= s, "k={k}: hull_sup {} under-reports {s}", c.hull_sup);
                if k == 1.0 {
                    first = c.hull_sup;
                }
                eprintln!(
                    "weight scale k={k:.0e}: hull_sup={:.6e} (vs k=1 {first:.6e}, ratio {:.4})",
                    c.hull_sup,
                    c.hull_sup / first
                );
            }
            Err(e) => eprintln!("weight scale k={k:.0e}: REFUSED {e}"),
        }
    }
}

/// **A deliberately hostile weight spread**, including weights that
/// touch and straddle zero. The only sound answers are a typed
/// refusal or a containing certificate.
#[test]
fn a_hostile_weight_spread_never_yields_a_finite_wrong_bound() {
    let base = elliptic_wall(1.0, 1.0, 1.0);
    let d = 0.2;
    let (fit, _) = fit_offset(&base, d, 1e-4, band()).unwrap();
    let n = fit.weights().len();
    let spreads: Vec<(&str, Vec<f64>)> = vec![
        (
            "alternating 1e-3 / 1e3",
            (0..n).map(|i| if i % 2 == 0 { 1e-3 } else { 1e3 }).collect(),
        ),
        (
            "one weight at 1e-12",
            (0..n).map(|i| if i == n / 2 { 1e-12 } else { 1.0 }).collect(),
        ),
        (
            "one weight EXACTLY zero",
            (0..n).map(|i| if i == n / 2 { 0.0 } else { 1.0 }).collect(),
        ),
        (
            "one weight NEGATIVE (straddles zero)",
            (0..n).map(|i| if i == n / 2 { -1.0 } else { 1.0 }).collect(),
        ),
        (
            "all weights negative",
            (0..n).map(|_| -1.0).collect(),
        ),
    ];
    for (name, w) in spreads {
        let Ok(hostile) = NurbsSurface::new(
            fit.knots_u().clone(),
            fit.knots_v().clone(),
            fit.control().to_vec(),
            w,
        ) else {
            eprintln!("hostile [{name}]: the surface constructor refused it");
            continue;
        };
        let s = sampled(&base, &hostile, d);
        let tol = if s.is_finite() { s * 4.0 } else { 1e9 };
        match certify_offset(&base, &hostile, d, tol, band()) {
            Ok(c) => {
                assert!(
                    c.hull_sup >= s,
                    "hostile [{name}]: FINITE WRONG BOUND — hull_sup {} < sampled {s}",
                    c.hull_sup
                );
                eprintln!(
                    "hostile [{name}]: certified hull_sup={:.4e} sampled={s:.4e}",
                    c.hull_sup
                );
            }
            Err(e) => eprintln!("hostile [{name}]: refused typed: {e}"),
        }
    }
}

/// **Recentring, farther than the shipped row goes.** Where does the
/// invariance stop, and is the degradation honest (a refusal or a
/// larger bound) rather than a silent under-report?
#[test]
fn recentring_probed_far_past_the_shipped_stations() {
    let d = 1e-6;
    let mut at_origin = f64::NAN;
    for shift in [0.0_f64, 1e3, 1e5, 1e6, 1e7, 1e8, 1e10] {
        let c = {
            let s = (FRAC_PI_2 * 0.5).cos();
            NurbsSurface::new(
                kv2(),
                kv1(),
                vec![
                    Point3::new(1.0, 0.0, 0.0),
                    Point3::new(1.0, 0.0, 1.0),
                    Point3::new(1.0, 1.0, 0.0),
                    Point3::new(1.0, 1.0, 1.0),
                    Point3::new(0.0, 1.0, 0.0),
                    Point3::new(0.0, 1.0, 1.0),
                ],
                vec![1.0, 1.0, s, s, 1.0, 1.0],
            )
            .unwrap()
        };
        let control: Vec<Point3<f64>> = c
            .control()
            .iter()
            .map(|p| Point3::new(p.x + shift, p.y + shift, p.z))
            .collect();
        let base = NurbsSurface::new(
            c.knots_u().clone(),
            c.knots_v().clone(),
            control,
            c.weights().to_vec(),
        )
        .unwrap();
        match fit_offset(&base, d, 1e-2, band()) {
            Ok((fit, cert)) => {
                let worst = sampled(&base, &fit, d);
                assert!(
                    cert.hull_sup >= worst,
                    "shift {shift:e}: UNDER-REPORT {} < {worst}",
                    cert.hull_sup
                );
                if shift == 0.0 {
                    at_origin = cert.hull_sup;
                }
                eprintln!(
                    "far shift={shift:.0e}: cells={} hull_sup={:.4e} sampled={worst:.3e} \
                     (x origin {:.3})",
                    cert.cells,
                    cert.hull_sup,
                    cert.hull_sup / at_origin
                );
            }
            Err(e) => eprintln!("far shift={shift:.0e}: REFUSED typed: {e}"),
        }
    }
}

/// **The stall hunt.** Deviation 7 says `RefinementStalled`'s
/// integration path is unreached across the fixtures probed. This
/// sweeps for a fixture that genuinely stalls, and reports which
/// typed refusal each request produces.
#[test]
fn hunt_for_a_genuine_refinement_stall() {
    let mut stalls = 0;
    let cases: Vec<(&str, NurbsSurface<f64>, f64)> = vec![
        ("elliptic wall 2:1", elliptic_wall(2.0, 1.0, 1.0), 0.1),
        ("elliptic wall 20:1 (extreme aspect)", elliptic_wall(20.0, 1.0, 1.0), 0.05),
        ("elliptic wall 1:1 tall", elliptic_wall(1.0, 1.0, 50.0), 0.1),
        ("ellipsoid band 3:1", ellipsoid_band(3.0, 1.0), 0.02),
        ("ellipsoid band 1:1", ellipsoid_band(1.0, 1.0), 0.3),
        ("thin wall", elliptic_wall(1.0, 1.0, 1e-4), 0.1),
        ("elliptic wall tiny d", elliptic_wall(1.0, 1.0, 1.0), 1e-9),
        ("elliptic wall huge coords", elliptic_wall(1e4, 1e4, 1e4), 10.0),
    ];
    for (name, base, d) in cases {
        for tol in [1e-6, 1e-9, 1e-11, 1e-13, 1e-15, 1e-17] {
            match fit_offset(&base, d, tol, band()) {
                Ok((_, c)) => eprintln!(
                    "stall-hunt [{name}] tol={tol:.0e}: CERTIFIED cells={} rounds={} sup={:.3e}",
                    c.cells, c.rounds, c.hull_sup
                ),
                Err(OffsetFitError::RefinementStalled {
                    rounds,
                    grid,
                    achieved,
                    ..
                }) => {
                    stalls += 1;
                    eprintln!(
                        "stall-hunt [{name}] tol={tol:.0e}: *** STALLED *** rounds={rounds} \
                         grid={grid:?} achieved={achieved:.4e}"
                    );
                }
                Err(OffsetFitError::BudgetExhausted { grid, achieved, .. }) => eprintln!(
                    "stall-hunt [{name}] tol={tol:.0e}: budget grid={grid:?} achieved={achieved:.4e}"
                ),
                Err(e) => eprintln!("stall-hunt [{name}] tol={tol:.0e}: {e}"),
            }
        }
    }
    eprintln!("stall-hunt: {stalls} genuine stalls found");
}
