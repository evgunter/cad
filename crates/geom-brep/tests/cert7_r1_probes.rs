//! Adversarial offset-certificate probes on rational bases the
//! shipped suite uses nowhere: a quarter-ellipse wall and an
//! ellipsoid band, both exact rationals whose weights no shipped
//! fixture shares.
//!
//! Every row here holds the same contract from outside the kernel:
//! a certificate that comes back must CONTAIN a densely sampled
//! residual, and anything else must be a typed refusal. The
//! independence of the bases is the point — a probe that reached for
//! a shipped fixture would stop being evidence about the door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use geom::NurbsSurface;
use geom_brep::offset_fit::{approx_offset_surface, certify_offset, fit_offset};
use geom_core::Point3;

use crate::shared::fixture::{arc_weight, kv1, kv2};
use crate::shared::sample::{grid, worst_offset_residual};
use crate::shared::tol::band;

/// A quarter **ellipse** wall: the exact rational quadratic quarter
/// ellipse (the affine image of the classical arc, so the same
/// weights) extruded linearly in `z`. Rational in `u`, and NOT a
/// surface this suite ships anywhere.
fn elliptic_wall(a: f64, b: f64, h: f64) -> NurbsSurface<f64> {
    let s = arc_weight(FRAC_PI_2);
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

/// An **ellipse of revolution** band: the exact rational sphere band
/// (a rational quadratic meridian arc revolved through the classical
/// rational quadratic quarter turn) with `z` scaled by `q` — an
/// affine image, so still exact, still rational in BOTH directions,
/// pole-free, and a surface this suite ships nowhere. This is the
/// case where `w`, `w_u` AND `w_v` are all live in `M̃`.
fn ellipsoid_band(r: f64, q: f64, lat0: f64, lat1: f64) -> NurbsSurface<f64> {
    let theta = 0.5 * (lat1 - lat0);
    let wm = theta.cos();
    let a = (r * lat0.cos(), r * lat0.sin());
    let b = (r * lat1.cos(), r * lat1.sin());
    let mid = (a.0 + b.0, a.1 + b.1);
    let mlen = (mid.0 * mid.0 + mid.1 * mid.1).sqrt();
    let m = (mid.0 / mlen * r / wm, mid.1 / mlen * r / wm);
    let meridian = [(a.0, a.1, 1.0), (m.0, m.1, wm), (b.0, b.1, 1.0)];
    let wr = arc_weight(FRAC_PI_2);
    let mut control = Vec::with_capacity(9);
    let mut weights = Vec::with_capacity(9);
    for iu in 0..3 {
        for (x, z, w) in meridian {
            let z = z * q;
            control.push(match iu {
                0 => Point3::new(x, 0.0, z),
                1 => Point3::new(x, x, z),
                _ => Point3::new(0.0, x, z),
            });
            weights.push(if iu == 1 { w * wr } else { w });
        }
    }
    NurbsSurface::new(kv2(), kv2(), control, weights).unwrap()
}

/// The residual against the exact offset locus on 23 x 19 stations,
/// or INFINITY where the base has no offset point at one of them —
/// these rows hand the door hostile rationals, so a locus with a hole
/// in it is a possible answer, and the containment assertions below
/// must FAIL on it rather than the sampler panicking above them.
fn sampled(base: &NurbsSurface<f64>, fit: &NurbsSurface<f64>, d: f64) -> f64 {
    worst_offset_residual(base, fit, d, &grid(23, 19)).unwrap_or(f64::INFINITY)
}

/// **E2E, rational bases nobody shipped, through the public doors.**
/// The storage door mints and re-derives; the re-derivation door is
/// then handed the minted fit back. Containment is asserted at every
/// station.
#[test]
fn e2e_rational_bases_through_the_storage_and_recertify_doors() {
    let cases: Vec<(&str, NurbsSurface<f64>, f64, f64)> = vec![
        (
            "elliptic wall a=2 b=1",
            elliptic_wall(2.0, 1.0, 1.0),
            0.1,
            1e-3,
        ),
        (
            "elliptic wall a=5 b=1 (hostile aspect)",
            elliptic_wall(5.0, 1.0, 1.0),
            0.05,
            1e-2,
        ),
        (
            "ellipsoid band r=2 q=0.5",
            ellipsoid_band(2.0, 0.5, 0.25, 1.25),
            0.15,
            1e-3,
        ),
        (
            "ellipsoid band r=2 q=0.2 (flat)",
            ellipsoid_band(2.0, 0.2, 0.3, 1.2),
            0.05,
            1e-3,
        ),
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
                assert!(
                    (again.hull_sup - cert.hull_sup).abs() <= 0.0,
                    "{name}: re-derivation moved"
                );
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
                assert!(
                    c.hull_sup >= s,
                    "k={k}: hull_sup {} under-reports {s}",
                    c.hull_sup
                );
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
            (0..n)
                .map(|i| if i % 2 == 0 { 1e-3 } else { 1e3 })
                .collect(),
        ),
        (
            "one weight at 1e-12",
            (0..n)
                .map(|i| if i == n / 2 { 1e-12 } else { 1.0 })
                .collect(),
        ),
        (
            "one weight EXACTLY zero",
            (0..n).map(|i| if i == n / 2 { 0.0 } else { 1.0 }).collect(),
        ),
        (
            "one weight NEGATIVE (straddles zero)",
            (0..n)
                .map(|i| if i == n / 2 { -1.0 } else { 1.0 })
                .collect(),
        ),
        ("all weights negative", (0..n).map(|_| -1.0).collect()),
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

/// **Liveness on the carrier the probes above stand on.** The e2e row
/// admits a typed refusal on every case it drives, because a refusal
/// is a sound answer to a hostile request. That makes it unable to
/// notice a door that has stopped certifying ANYTHING: the elliptic
/// wall at a millimetre offset and a millimetre tolerance is a
/// request this kernel answers, and if it stops, the rows above go
/// quietly vacuous rather than red.
#[test]
fn the_elliptic_wall_certifies_at_the_tolerance_the_probes_ask_of_it() {
    let base = elliptic_wall(2.0, 1.0, 1.0);
    let (d, tol) = (0.1, 1e-3);
    let (fit, cert) = fit_offset(&base, d, tol, band()).unwrap_or_else(|e| {
        panic!("LIVENESS: the elliptic wall refused at d = {d}, tol = {tol}: {e}")
    });
    assert!(
        cert.hull_sup <= tol,
        "LIVENESS: certified sup {} exceeds the tolerance {tol} it was asked for",
        cert.hull_sup
    );
    let worst = sampled(&base, &fit, d);
    assert!(
        worst <= cert.hull_sup,
        "CONTAINMENT: certified sup {} UNDER-reports the sampled max {worst}",
        cert.hull_sup
    );
}
