//! CERT-3 review lane R1 — probes against PR 1277's claims.
//!
//! Local-only; never pushed. Each probe reproduces one numbered claim
//! from the review brief with its own fixtures (not the unit's).

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Bounds, Interval, Mat3, Point3, Real, Vec3};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn width(e: Interval) -> f64 {
    e.hi() - e.lo()
}

/// Claim 2: reproduce the PR's half-angle enclosure table at θ = [0, 0].
#[test]
fn r1_half_angle_enclosure_table() {
    let th = Interval::zero();
    let (s, c) = th.sin_cos();
    println!("sin(0):  [{:e}, {:e}] width {:e}", s.lo(), s.hi(), width(s));
    println!("cos(0):  [{:e}, {:e}] width {:e}", c.lo(), c.hi(), width(c));
    let t_full = Interval::one() - c;
    println!(
        "1-cos:   [{:e}, {:e}] width {:e}",
        t_full.lo(),
        t_full.hi(),
        width(t_full)
    );
    let (hs, _hc) = (th * iv(0.5)).sin_cos();
    let t_half = iv(2.0) * hs.powi(2);
    println!(
        "2sin^2:  [{:e}, {:e}] width {:e}",
        t_half.lo(),
        t_half.hi(),
        width(t_half)
    );
    // The PR's numbers: sin width ~4e-323; 1-cos floor ~4.44e-16;
    // half-angle form subnormal (~2.5e-323).
    assert!(width(s) > 0.0 && width(s) <= 1.0e-320, "sin(0) not subnormal dust");
    assert!(
        width(t_full) >= 4.0e-16 && width(t_full) <= 5.0e-16,
        "1-cos floor is {:e}, not the claimed ~4.44e-16",
        width(t_full)
    );
    assert!(
        width(t_half) <= 1.0e-320,
        "2sin^2(0/2) width {:e} is not subnormal",
        width(t_half)
    );
    // Deviation 1's impossibility premise: the backend's sin at the
    // exact point 0 is NOT [0, 0], so any trig-factored spelling
    // carries dust.
    assert!(s.lo() < 0.0 && s.hi() > 0.0, "sin(0) came back literal [0,0]");
}

/// Claim 8: the poison contract INCLUDING at angle = 0, both lanes,
/// on my own fixture (the unit's existing poison row uses angle = 1.0
/// at f64 only).
#[test]
fn r1_zero_axis_poison_at_zero_angle_both_lanes() {
    // f64 lane, angle exactly 0.
    let bad = Affine3::rotation_about_axis(Point3::new(1.0f64, 2.0, 3.0), Vec3::zero(), 0.0);
    for c in [bad.linear.c0, bad.linear.c1, bad.linear.c2] {
        assert!(c.x.is_nan() && c.y.is_nan() && c.z.is_nan());
    }
    assert!(
        bad.translation.x.is_nan() && bad.translation.y.is_nan() && bad.translation.z.is_nan(),
        "f64 zero-axis poison does not reach the translation at angle = 0"
    );
    // Interval lane, angle = [0, 0]: the poison manifests as ENTIRE
    // intervals (measured: identical under the retired spelling), not
    // NaI — the contract is "useless, visibly", preserved by the fix.
    let badi = Affine3::rotation_about_axis(
        Point3::new(iv(1.0), iv(2.0), iv(3.0)),
        Vec3::new(Interval::zero(), Interval::zero(), Interval::zero()),
        Interval::zero(),
    );
    for e in [badi.translation.x, badi.translation.y, badi.translation.z] {
        assert!(
            e.lo() == f64::NEG_INFINITY && e.hi() == f64::INFINITY,
            "Interval zero-axis translation at angle 0 is not entire: [{:e}, {:e}]",
            e.lo(),
            e.hi()
        );
    }
    // The operator itself, directly.
    let op = Mat3::identity_minus_rotation_about(Vec3::new(0.0f64, 0.0, 0.0), 0.0);
    for c in [op.c0, op.c1, op.c2] {
        assert!(c.x.is_nan() && c.y.is_nan() && c.z.is_nan());
    }
}

/// Claim 1 (guard teeth): under plain interval semantics the retired
/// spelling pays width(q) + width(R·q) >= 2·width(q) unconditionally,
/// so the guard cannot fire except when (a) a correlation-tracking
/// backend replaces the enclosure arithmetic, or (b) R goes poisoned
/// (NaN width fails the >= comparison). Probe (b) executably; (a) is
/// argued in the report.
#[test]
fn r1_retired_guard_red_paths() {
    let h = 1.0e-9;
    let wide = |c: f64| Interval::from_bounds(c - h, c + h);
    let anchor = Point3::new(wide(1.0), wide(2.0), wide(-3.0));
    let q = anchor - Point3::origin();
    // Path (b): a poisoned R (zero axis) makes the guard's width NaN,
    // and NaN >= 1.9·width(anchor) is false — the guard reds.
    let linear = Mat3::rotation_about(
        Vec3::new(Interval::zero(), Interval::zero(), Interval::zero()),
        Interval::zero(),
    );
    let retired = q - linear * q;
    let w = width(retired.x);
    println!("poisoned retired width: {w:e}");
    // MEASURED REALITY (a finding, not the hoped-for red path): a
    // poisoned R yields entire intervals, width = +inf, and inf >= 1.9w
    // PASSES — so a poison regression does NOT red the guard. Its only
    // red path is the enclosure arithmetic learning cancellation.
    assert!(
        w >= 1.9 * width(anchor.x),
        "unexpected: the guard would red under a poisoned R after all"
    );
    // Sanity: on the healthy fixture the retired spelling pays >= 2w
    // for any correlation-blind backend — measured, not assumed.
    let ok = Mat3::rotation_about(Vec3::new(wide(1.0), wide(-2.0), wide(2.0)), Interval::zero());
    let paid = q - ok * q;
    assert!(width(paid.x) >= 2.0 * width(anchor.x));
}

/// Claims 3 + structural: the small-angle law on MY fixture — oblique
/// wide axis (the unit's scaling row uses an exact +z axis, which is
/// friendlier), and angles the unit did not pick.
#[test]
fn r1_small_angle_scaling_oblique_wide_axis() {
    let h = 1.0e-8;
    let wide = |c: f64| Interval::from_bounds(c - h, c + h);
    let anchor = Point3::new(wide(0.5), wide(-4.0), wide(2.5));
    let qw = width(anchor.x);
    let axis = Vec3::new(wide(2.0), wide(1.0), wide(-2.0));
    let mut ws = [0.0f64; 3];
    for (i, theta) in [3.0e-3f64, 3.0e-5, 3.0e-7].into_iter().enumerate() {
        let rot = Affine3::rotation_about_axis(anchor, axis, iv(theta));
        let w = width(rot.translation.x)
            .max(width(rot.translation.y))
            .max(width(rot.translation.z));
        println!("theta {theta:e}: width {w:e} (theta*qw = {:e})", theta * qw);
        ws[i] = w;
        // The law off the exact axis is θ·(width(q) + C·|q|·width(n)),
        // still angle-proportional; the axis-width term dominates here.
        assert!(
            w <= 40.0 * theta * qw,
            "no angle-proportional law on an oblique wide axis"
        );
    }
    assert!(
        ws[0] >= 50.0 * ws[1] && ws[1] >= 50.0 * ws[2],
        "scaling law fails off the unit's chosen fixture: {:e}/{:e}/{:e}",
        ws[0],
        ws[1],
        ws[2]
    );
}

/// Structural: is the diagonal t·(nj² + nk²) a valid enclosure of the
/// true entry t·(1 − ni²) when the axis is wide (|n| = 1 only over the
/// reals)? Cross-check the two spellings enclose each other's truth:
/// the interval diagonal must contain the exact real value computed at
/// the midpoint axis to high precision.
#[test]
fn r1_diagonal_equivalence_under_wide_axis() {
    let h = 1.0e-6;
    let wide = |c: f64| Interval::from_bounds(c - h, c + h);
    let axis = Vec3::new(wide(1.0), wide(-2.0), wide(2.0));
    let theta = 0.7f64;
    let m = Mat3::identity_minus_rotation_about(axis, iv(theta));
    // Real-arithmetic reference at the exact midpoint axis (1,-2,2)/3.
    let (nx, ny, nz) = (1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0);
    let t = 2.0 * (theta / 2.0).sin().powi(2);
    for (got, njk) in [
        (m.c0.x, ny * ny + nz * nz),
        (m.c1.y, nx * nx + nz * nz),
        (m.c2.z, nx * nx + ny * ny),
    ] {
        let truth = t * njk;
        assert!(
            got.lo() <= truth + 1e-12 && truth - 1e-12 <= got.hi(),
            "diagonal enclosure [{:e}, {:e}] misses the real value {truth:e}",
            got.lo(),
            got.hi()
        );
    }
}

/// Claim 5 helper: the f64 fixed-point residual of the anchor, retired
/// vs new, on my own fixture — checking the reported ~0.6 → ~1.6 ulp
/// order of magnitude and that nothing catastrophic hides behind it.
#[test]
fn r1_fixed_point_residual_f64() {
    let anchor = Point3::new(1.0f64, 2.0, -3.0);
    let axis = Vec3::new(1.0, -2.0, 2.0);
    let mut worst_new = 0.0f64;
    let mut worst_old = 0.0f64;
    for k in 1..=32 {
        let theta = f64::from(k) * 0.2;
        let rot = Affine3::rotation_about_axis(anchor, axis, theta);
        let img = rot.transform_point(anchor);
        // Retired spelling, rebuilt by hand.
        let linear = Mat3::rotation_about(axis, theta);
        let q = anchor - Point3::origin();
        let old = Affine3::from_parts(linear, q - linear * q);
        let img_old = old.transform_point(anchor);
        for (a, b, c) in [
            (img.x, img_old.x, anchor.x),
            (img.y, img_old.y, anchor.y),
            (img.z, img_old.z, anchor.z),
        ] {
            worst_new = worst_new.max(((a - c) / c).abs());
            worst_old = worst_old.max(((b - c) / c).abs());
        }
    }
    println!("fixed-point residual, relative: retired {worst_old:e}, new {worst_new:e}");
    // Both must stay ulp-scale; the new one is allowed to be ~1 ulp
    // worse (the PR reports it), but not more than a few ulps.
    assert!(worst_old <= 1.0e-15 && worst_new <= 2.0e-15);
}
