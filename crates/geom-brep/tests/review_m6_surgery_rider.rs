//! **Adopted blinded-review probes for M6 unit 1's rider** (charter
//! E), adopted verbatim per the standing review policy: falsification
//! attempt on `circle_residual_extremes` — the enclosure must never
//! be beaten by dense sampling (a bound below truth would be a MAJOR
//! unsoundness), and a positive `max(lo, -hi)` margin must imply the
//! circle never changes side across the whole carrier (the semantics
//! the boolean's definite-miss arm rests on). Spheres and cylinders,
//! adversarial near-tangent families; case counts and sample densities
//! are multiples of `CAD_FUZZ_EFFORT` and the seed varies per run
//! (`test_utils::fuzz`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/implicit.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom/src/surfaces/",
    "crates/geom/src/surfaces.rs",
];

use geom::Surface;
use geom_brep::{circle_arc_residual_range, circle_residual_extremes, implicit_residual};
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

fn unit_vec(rng: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let n = v.norm();
        if n > 1e-3 {
            return v / n;
        }
    }
}

fn frame(rng: &mut fuzz::Rng) -> (Vec3<f64>, Vec3<f64>) {
    let a = unit_vec(rng);
    let mut u = unit_vec(rng);
    u = (u - a * a.dot(u)).normalize();
    (a, u)
}

#[test]
fn enclosure_never_beaten_by_dense_sampling() {
    let mut rng = fuzz::start("review_m6_surgery_rider::enclosure_vs_dense_sampling");
    let n_samples = fuzz::scaled(2_048);
    let cases = fuzz::scaled(375);
    let mut worst_slack = f64::INFINITY;
    for case in 0..cases {
        let (axis, u_ref) = frame(&mut rng);
        let center = Point3::new(
            rng.range(-2.0, 2.0),
            rng.range(-2.0, 2.0),
            rng.range(-2.0, 2.0),
        );
        let radius = rng.range(1e-4, 3.0);
        let s: Surface<f64> = match case % 4 {
            0 => {
                let (sa, _) = frame(&mut rng);
                Surface::Sphere {
                    center: Point3::new(
                        rng.range(-2.0, 2.0),
                        rng.range(-2.0, 2.0),
                        rng.range(-2.0, 2.0),
                    ),
                    radius: rng.range(1e-3, 2.5),
                    axis: sa,
                    u_ref: Vec3::unit_x(),
                }
            }
            1 => {
                let (ca, _) = frame(&mut rng);
                Surface::Cylinder {
                    origin: Point3::new(
                        rng.range(-2.0, 2.0),
                        rng.range(-2.0, 2.0),
                        rng.range(-2.0, 2.0),
                    ),
                    axis: ca,
                    radius: rng.range(1e-3, 2.5),
                    u_ref: Vec3::unit_x(),
                }
            }
            2 => {
                // The TORUS family, added at the v6 dual on
                // `f0f46ebb5` (TARM-R1 MINOR-1, TARM-R2 M-1, which
                // converged): the kind's enclosure is SAMPLED rather
                // than closed-form, and until this row carried one no
                // shipped row could see a wrong torus bound.
                let (ta, _) = frame(&mut rng);
                let tu = {
                    let (_, u) = frame(&mut rng);
                    let p = u - ta * ta.dot(u);
                    if p.norm() > 1e-3 {
                        p.normalize()
                    } else {
                        Vec3::unit_x()
                    }
                };
                let big_r = rng.range(0.3, 3.0);
                Surface::Torus {
                    center: Point3::new(
                        rng.range(-2.0, 2.0),
                        rng.range(-2.0, 2.0),
                        rng.range(-2.0, 2.0),
                    ),
                    axis: ta,
                    major_radius: big_r,
                    minor_radius: big_r * rng.range(0.05, 0.6),
                    u_ref: tu,
                }
            }
            _ => {
                // adversarial cylinder families: near-coaxial, contained,
                // near-tangent
                let jitter = rng.range(0.0, 0.05);
                let ca = (axis + Vec3::new(jitter, jitter / 2.0, 0.0)).normalize();
                Surface::Cylinder {
                    origin: center + axis * rng.range(-0.5, 0.5) + u_ref * rng.range(0.0, 0.02),
                    axis: ca,
                    radius: if case % 2 == 0 {
                        radius // near-tangent internal
                    } else {
                        rng.range(1e-3, 2.5)
                    },
                    u_ref: Vec3::unit_x(),
                }
            }
        };
        let Some((lo, hi)) = circle_residual_extremes(&s, center, axis, radius, u_ref) else {
            panic!("an enclosure is expected for sphere/cylinder/torus");
        };
        assert!(
            lo <= hi,
            "malformed range case {case}: [{lo}, {hi}] — {}",
            fuzz::replay()
        );
        let v = axis.cross(u_ref);
        let mut smin = f64::INFINITY;
        let mut smax = f64::NEG_INFINITY;
        for i in 0..n_samples {
            let t = core::f64::consts::TAU * (i as f64) / (n_samples as f64);
            let p = center + (u_ref * t.cos() + v * t.sin()) * radius;
            let r = implicit_residual(&s, p);
            smin = smin.min(r);
            smax = smax.max(r);
        }
        let tol = 1e-9 * (1.0 + smax.abs().max(smin.abs()));
        assert!(
            lo <= smin + tol,
            "case {case}: lo {lo} beats sampled min {smin} on {s:?} (UNSOUND) — {}",
            fuzz::replay()
        );
        assert!(
            hi >= smax - tol,
            "case {case}: hi {hi} beats sampled max {smax} on {s:?} (UNSOUND) — {}",
            fuzz::replay()
        );
        // Sphere arm claims exactness: check tightness there.
        if matches!(s, Surface::Sphere { .. }) {
            let span = (hi - lo).max(1e-12);
            assert!(
                (smin - lo).abs() / span < 1e-4 && (hi - smax).abs() / span < 1e-4,
                "case {case}: sphere arm not tight: [{lo},{hi}] vs sampled [{smin},{smax}] — {}",
                fuzz::replay()
            );
        }
        if lo.is_finite() && hi.is_finite() {
            worst_slack = worst_slack.min((smin - lo).min(hi - smax));
        }
    }
    println!(
        "rider falsifier: {cases} pairs x {n_samples} samples, no enclosure \
         violation; min slack {worst_slack:.3e}"
    );
}

/// The margin the boolean arm uses: max(lo, -hi) > 0 must imply the
/// sampled residual never changes sign across the circle.
#[test]
fn positive_margin_implies_one_sided() {
    let mut rng = fuzz::start("review_m6_surgery_rider::positive_margin_one_sided");
    let mut one_sided_checked = 0u32;
    for _ in 0..fuzz::scaled(2_500) {
        let (axis, u_ref) = frame(&mut rng);
        let center = Point3::new(
            rng.range(-2.0, 2.0),
            rng.range(-2.0, 2.0),
            rng.range(-2.0, 2.0),
        );
        let radius = rng.range(1e-3, 2.0);
        let (sa, _) = frame(&mut rng);
        let s: Surface<f64> = if rng.unit() < 0.5 {
            Surface::Sphere {
                center: Point3::new(
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                ),
                radius: rng.range(1e-2, 2.0),
                axis: sa,
                u_ref: Vec3::unit_x(),
            }
        } else {
            Surface::Cylinder {
                origin: Point3::new(
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                ),
                axis: sa,
                radius: rng.range(1e-2, 2.0),
                u_ref: Vec3::unit_x(),
            }
        };
        let (lo, hi) = circle_residual_extremes(&s, center, axis, radius, u_ref).unwrap();
        let margin = lo.max(-hi);
        if margin <= 0.0 {
            continue;
        }
        one_sided_checked += 1;
        let v = axis.cross(u_ref);
        let mut pos = false;
        let mut neg = false;
        let sweep = fuzz::scaled(512);
        for i in 0..sweep {
            let t = core::f64::consts::TAU * (i as f64) / (sweep as f64);
            let p = center + (u_ref * t.cos() + v * t.sin()) * radius;
            let r = implicit_residual(&s, p);
            pos |= r > 0.0;
            neg |= r < 0.0;
        }
        assert!(
            !(pos && neg),
            "margin {margin} > 0 but the circle straddles the surface: {s:?} — {}",
            fuzz::replay()
        );
    }
    println!("one-sided semantic held on {one_sided_checked} positive-margin pairs");
}

/// **The ARC door's enclosure, against a dense oracle, on random
/// configurations.** Adopted from the v6 dual on `f0f46ebb5` —
/// TARM-R1's probe P2 and TARM-R2's probe C2, which found the same
/// hole from two sides: every shipped enclosure row was a named
/// fixture, and the PR's own M2 mutant (the n-projection terms
/// sign-flipped) produced GENUINE escapes of 2.3e-4 m — four orders
/// above every band — that no fixture row saw.
///
/// The row is deliberately not a fixture family. Random tori, random
/// circles, log-uniform spans from a sliver to a whole turn, and the
/// enclosure compared against a dense sampling of the residual it
/// claims to contain. An arc whose bound is infinite (a circle that
/// may reach the axis) encloses trivially and is counted, not skipped
/// silently.
#[test]
fn the_arc_enclosure_is_never_beaten_by_a_dense_oracle() {
    let mut rng = fuzz::start("review_m6_surgery_rider::arc_enclosure_vs_dense_oracle");
    let cases = fuzz::scaled(200);
    let dense = fuzz::scaled(1_500);
    let (mut infinite, mut checked) = (0u32, 0u32);
    let mut worst_slack = f64::INFINITY;
    for case in 0..cases {
        let (t_axis, _) = frame(&mut rng);
        let t_u = {
            let (_, u) = frame(&mut rng);
            let p = u - t_axis * t_axis.dot(u);
            if p.norm() > 1e-3 {
                p.normalize()
            } else {
                Vec3::unit_x()
            }
        };
        let big_r = rng.range(0.3, 3.0);
        let t_center = Point3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let s = Surface::Torus {
            center: t_center,
            axis: t_axis,
            major_radius: big_r,
            minor_radius: big_r * rng.range(0.05, 0.6),
            u_ref: t_u,
        };
        // A quarter of the circles are drawn close to the torus, where
        // the residual is small and an escape is cheapest to make; one
        // in eight is a RING-PLANE circle reaching from near the axis
        // out past the spine, the only family where taking `D_max` at
        // the far end alone under-bounds.
        let t0 = rng.range(0.0, core::f64::consts::TAU);
        let t1 = t0 + 10f64.powf(rng.range(-3.0, core::f64::consts::TAU.log10()));
        let (center, axis, u_ref, radius) = if case % 8 == 1 {
            (
                t_center + t_u * (big_r * rng.range(0.5, 0.7)),
                t_axis,
                t_u,
                big_r * rng.range(0.4, 0.5),
            )
        } else {
            let spread = if case.is_multiple_of(4) { 0.5 } else { 4.0 };
            let (axis, u_ref) = frame(&mut rng);
            (
                t_center
                    + Vec3::new(
                        rng.range(-spread, spread),
                        rng.range(-spread, spread),
                        rng.range(-spread, spread),
                    ),
                axis,
                u_ref,
                rng.range(0.05, 4.0),
            )
        };
        let (lo, hi) = circle_arc_residual_range(&s, center, axis, radius, u_ref, t0, t1)
            .expect("the torus arm answers");
        if !(lo.is_finite() && hi.is_finite()) {
            infinite += 1;
            continue;
        }
        let v = axis.cross(u_ref);
        let (mut dlo, mut dhi) = (f64::INFINITY, f64::NEG_INFINITY);
        for i in 0..=dense {
            let t = t0 + (t1 - t0) * (i as f64) / (dense as f64);
            let p = center + (u_ref * t.cos() + v * t.sin()) * radius;
            let r = implicit_residual(&s, p);
            dlo = dlo.min(r);
            dhi = dhi.max(r);
        }
        let scale = 1e-12 * (1.0 + dhi.abs().max(dlo.abs()));
        assert!(
            lo <= dlo + scale && hi >= dhi - scale,
            "case {case}: enclosure [{lo}, {hi}] is beaten by the dense range \
             [{dlo}, {dhi}] on {s:?} (UNSOUND) — {}",
            fuzz::replay()
        );
        checked += 1;
        worst_slack = worst_slack.min((dlo - lo).min(hi - dhi));
    }
    println!(
        "arc enclosure: {checked} finite configurations x {dense} samples, no \
         escape; {infinite} through-axis (infinite by arithmetic); min slack \
         {worst_slack:.3e}"
    );
}
