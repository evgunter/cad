//! CERT-3 review lane R2 — independent probes. Not a unit deliverable.
//!
//! These reproduce the PR's measured claims from scratch rather than
//! reading its numbers back.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Bounds, Interval, Mat3, Point3, Real, Vec3};

fn w(e: Interval) -> f64 {
    e.hi() - e.lo()
}

/// CLAIM 2: the backend enclosure table at theta = [0, 0].
#[test]
fn r2_half_angle_enclosure_table() {
    let z = Interval::zero();
    let (s, c) = z.sin_cos();
    println!("sin[0,0]  = [{:e}, {:e}] width {:e}", s.lo(), s.hi(), w(s));
    println!("cos[0,0]  = [{:e}, {:e}] width {:e}", c.lo(), c.hi(), w(c));
    let t_full = Interval::one() - c;
    println!(
        "1-cos     = [{:e}, {:e}] width {:e}",
        t_full.lo(),
        t_full.hi(),
        w(t_full)
    );
    let (hs, hc) = (z * Interval::from_f64(0.5)).sin_cos();
    let t_half = Interval::from_f64(2.0) * hs.powi(2);
    let s_half = Interval::from_f64(2.0) * hs * hc;
    println!(
        "2sin^2(h) = [{:e}, {:e}] width {:e}",
        t_half.lo(),
        t_half.hi(),
        w(t_half)
    );
    println!(
        "2 s h c h = [{:e}, {:e}] width {:e}",
        s_half.lo(),
        s_half.hi(),
        w(s_half)
    );
    // The load-bearing comparison, whatever the exact digits are.
    assert!(w(t_full) > 1.0e-16, "1-cos floor vanished: {:e}", w(t_full));
    assert!(w(t_half) < 1.0e-300, "half-angle t is not subnormal-scale");
}

/// CLAIM 1: independence of the anchor's width, swept far wider than
/// the unit's own two rows, and at several axis orientations.
#[test]
fn r2_zero_angle_independence_swept_hard() {
    for (name, ax) in [
        ("+z", [0.0, 0.0, 1.0]),
        ("+x", [1.0, 0.0, 0.0]),
        ("oblique", [1.0, -2.0, 2.0]),
        ("near-axis", [1.0, 1.0e-9, 0.0]),
        ("huge", [1.0e12, 3.0e11, -7.0e11]),
    ] {
        // The ANCHOR is widened across twelve orders; the axis is held
        // exact, so the fixture stays non-degenerate (a widened axis
        // that straddles zero normalizes to inf and is a different
        // question, probed separately).
        for h in [0.0f64, 1.0e-15, 1.0e-9, 1.0e-3, 1.0, 1.0e6] {
            let wi = |c: f64| Interval::from_bounds(c - h, c + h);
            let anchor = Point3::new(wi(1.0), wi(2.0), wi(-3.0));
            let axis = Vec3::new(
                Interval::from_f64(ax[0]),
                Interval::from_f64(ax[1]),
                Interval::from_f64(ax[2]),
            );
            let r = Affine3::rotation_about_axis(anchor, axis, Interval::zero());
            let tw = w(r.translation.x)
                .max(w(r.translation.y))
                .max(w(r.translation.z));
            println!("axis {name} anchor half-width {h:e}: translation width {tw:e}");
            // NOT independent of the anchor's width: the operator is
            // subnormal-scale but it still MULTIPLIES the anchor, so the
            // residue grows with |q| and width(q). The unit's own bound
            // of 1e-320 holds only while the anchor width stays small;
            // it is exceeded at half-width 1e6 (measured 1.68e-316).
            let bound = 1.0e-320 + 1.0e-321 * h;
            assert!(
                tw <= bound,
                "axis {name}, anchor half-width {h:e}: translation {tw:e} \
                 wide, over the proportional bound {bound:e}"
            );
        }
    }
}

/// CLAIM 3: does the scaling row survive angles the unit did not pick,
/// and does a constant floor really not survive?
#[test]
fn r2_small_angle_scaling_extended() {
    let h = 1.0e-6;
    let wi = |c: f64| Interval::from_bounds(c - h, c + h);
    let anchor = Point3::new(wi(1.0), wi(2.0), wi(-3.0));
    let qw = w(anchor.x);
    for axname in ["z", "oblique"] {
        let axis = if axname == "z" {
            Vec3::new(Interval::zero(), Interval::zero(), Interval::one())
        } else {
            Vec3::new(
                Interval::from_f64(1.0),
                Interval::from_f64(-2.0),
                Interval::from_f64(2.0),
            )
        };
        for theta in [1.0e-1f64, 1.0e-2, 1.0e-4, 1.0e-6, 1.0e-8, 1.0e-12, 1.0e-16] {
            let r = Affine3::rotation_about_axis(anchor, axis, Interval::from_f64(theta));
            let tw = w(r.translation.x)
                .max(w(r.translation.y))
                .max(w(r.translation.z));
            println!(
                "axis {axname} theta {theta:e}: width {tw:e}, ratio to theta*qw = {:e}",
                tw / (theta * qw)
            );
        }
    }
}

/// CLAIM 8: the poison contract, including at angle zero, and on the
/// new operator directly.
#[test]
fn r2_poison_contract_at_zero_angle() {
    for angle in [0.0f64, 1.0e-30, 1.0, core::f64::consts::TAU] {
        for axis in [
            Vec3::<Interval>::zero(),
            Vec3::new(
                Interval::from_f64(f64::NAN),
                Interval::zero(),
                Interval::zero(),
            ),
        ] {
            let m = Mat3::identity_minus_rotation_about(axis, Interval::from_f64(angle));
            for v in [m.c0, m.c1, m.c2] {
                for e in [v.x, v.y, v.z] {
                    let poisoned = !e.is_certified() || e.lo().is_nan() || e.hi().is_nan();
                    assert!(
                        poisoned,
                        "angle {angle:e}: operator entry [{:e}, {:e}] is a \
                         CERTIFIED finite enclosure on a poisoned axis",
                        e.lo(),
                        e.hi()
                    );
                }
            }
            let a = Affine3::rotation_about_axis(
                Point3::new(
                    Interval::from_f64(1.0),
                    Interval::from_f64(2.0),
                    Interval::from_f64(3.0),
                ),
                axis,
                Interval::from_f64(angle),
            );
            let t = a.translation.x;
            println!(
                "angle {angle:e}: translation.x = [{:e}, {:e}] certified={}",
                t.lo(),
                t.hi(),
                t.is_certified()
            );
            assert!(
                !t.is_certified() || t.lo().is_nan() || t.hi().is_nan(),
                "angle {angle:e}: translation [{:e}, {:e}] is a CERTIFIED \
                 finite enclosure on a poisoned axis",
                t.lo(),
                t.hi()
            );
        }
    }
}

/// STRUCTURAL: is the operator actually `I - rotation_about` to within
/// rounding, at f64, over a broad sweep? An algebraic slip in the
/// hand-written entries would not be caught by any row in this PR.
#[test]
fn r2_operator_agrees_with_identity_minus_rotation_f64() {
    let mut worst = 0.0f64;
    let mut worst_at = (0.0f64, [0.0f64; 3]);
    for ax in [
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, -2.0, 2.0],
        [-3.0, 1.0, 0.5],
        [0.1, 0.2, -0.97],
    ] {
        let axis = Vec3::new(ax[0], ax[1], ax[2]);
        for &angle in &[
            1.0e-12,
            1.0e-6,
            0.1,
            0.5,
            1.0,
            core::f64::consts::FRAC_PI_2,
            2.0,
            3.0,
            core::f64::consts::PI,
            4.0,
            5.0,
            core::f64::consts::TAU,
            -1.0,
            -core::f64::consts::PI,
            10.0,
            100.0,
        ] {
            let r = Mat3::rotation_about(axis, angle);
            let d = Mat3::identity_minus_rotation_about(axis, angle);
            let id = Mat3::<f64>::identity();
            for (col, (dc, (rc, ic))) in [
                (d.c0, (r.c0, id.c0)),
                (d.c1, (r.c1, id.c1)),
                (d.c2, (r.c2, id.c2)),
            ]
            .into_iter()
            .enumerate()
            {
                for (row, (dv, (rv, iv))) in
                    [(dc.x, (rc.x, ic.x)), (dc.y, (rc.y, ic.y)), (dc.z, (rc.z, ic.z))]
                        .into_iter()
                        .enumerate()
                {
                    let expect = iv - rv;
                    let e = (dv - expect).abs();
                    if e > worst {
                        worst = e;
                        worst_at = (angle, ax);
                    }
                    assert!(
                        e <= 1.0e-14,
                        "col {col} row {row}, axis {ax:?}, angle {angle}: \
                         operator {dv:e} vs I-R {expect:e}, err {e:e}"
                    );
                }
            }
        }
    }
    println!("worst |operator - (I-R)| = {worst:e} at angle {worst_at:?}");
}

/// STRUCTURAL: the diagonal claims equivalence "through |n| = 1". Is
/// that premise enforced? `rotation_about` normalizes internally, so
/// probe whether an axis whose normalize is inexact breaks the
/// operator's agreement with I - R more than rounding.
#[test]
fn r2_diagonal_equivalence_under_inexact_normalization() {
    let mut worst = 0.0f64;
    for ax in [
        [1.0, 1.0, 1.0],
        [1.0, 1.0e-12, 0.0],
        [1.0e-160, 1.0e-160, 1.0e-160],
        [1.0e160, 1.0, 1.0],
        [3.0, 4.0, 0.0],
        [1.0, 2.0, 3.0],
    ] {
        let axis = Vec3::new(ax[0], ax[1], ax[2]);
        let n = axis.normalize();
        let sq = n.x * n.x + n.y * n.y + n.z * n.z;
        for &angle in &[1.0e-8, 0.3, 1.0, 3.0] {
            let d = Mat3::identity_minus_rotation_about(axis, angle);
            let r = Mat3::rotation_about(axis, angle);
            let id = Mat3::<f64>::identity();
            let e = (d.c0.x - (id.c0.x - r.c0.x))
                .abs()
                .max((d.c1.y - (id.c1.y - r.c1.y)).abs())
                .max((d.c2.z - (id.c2.z - r.c2.z)).abs());
            if e > worst {
                worst = e;
            }
            println!("axis {ax:?} |n|^2-1 = {:e} angle {angle}: diag err {e:e}", sq - 1.0);
        }
    }
    println!("worst diagonal disagreement {worst:e}");
    assert!(worst.is_finite());
}

/// CLAIM 5: reproduce the bit-movement sweep at the constructor and the
/// fixed-point residual regression, independently.
#[test]
fn r2_bit_movement_and_fixed_point_residual() {
    let retired = |q: Vec3<f64>, axis: Vec3<f64>, angle: f64| {
        let linear = Mat3::rotation_about(axis, angle);
        q - linear * q
    };
    let new = |q: Vec3<f64>, axis: Vec3<f64>, angle: f64| {
        Mat3::identity_minus_rotation_about(axis, angle) * q
    };
    let mut moved = 0usize;
    let mut total = 0usize;
    let mut max_abs = 0.0f64;
    let mut max_rel = 0.0f64;
    let mut res_old = 0.0f64;
    let mut res_new = 0.0f64;
    for ax in [[0.0, 0.0, 1.0], [1.0, -2.0, 2.0], [1.0, 0.0, 0.0]] {
        let axis = Vec3::new(ax[0], ax[1], ax[2]);
        for anchor in [[1.0, 2.0, -3.0], [100.0, -250.0, 30.0], [0.001, 0.002, -0.003]] {
            let p = Point3::new(anchor[0], anchor[1], anchor[2]);
            let q = p - Point3::origin();
            let mag = (q.x * q.x + q.y * q.y + q.z * q.z).sqrt();
            for &angle in &[
                1.0e-9,
                1.0e-6,
                0.001,
                0.1,
                core::f64::consts::FRAC_PI_2,
                core::f64::consts::PI,
                5.0,
                core::f64::consts::TAU,
                2.0,
            ] {
                let a = retired(q, axis, angle);
                let b = new(q, axis, angle);
                for (u, v) in [(a.x, b.x), (a.y, b.y), (a.z, b.z)] {
                    total += 1;
                    if u.to_bits() != v.to_bits() {
                        moved += 1;
                    }
                    let d = (u - v).abs();
                    if d > max_abs {
                        max_abs = d;
                    }
                    if mag > 0.0 && d / mag > max_rel {
                        max_rel = d / mag;
                    }
                }
                // fixed-point residual: the anchor maps to itself
                let r_old = Affine3::from_parts(Mat3::rotation_about(axis, angle), a);
                let r_new = Affine3::from_parts(Mat3::rotation_about(axis, angle), b);
                for (map, acc) in [(r_old, &mut res_old), (r_new, &mut res_new)] {
                    let img = map.transform_point(p);
                    let e = ((img.x - p.x).powi(2) + (img.y - p.y).powi(2) + (img.z - p.z).powi(2))
                        .sqrt()
                        / mag;
                    if e > *acc {
                        *acc = e;
                    }
                }
            }
        }
    }
    println!("moved {moved} of {total}; max abs {max_abs:e}; max rel {max_rel:e}");
    println!("fixed-point residual relative: retired {res_old:e}, new {res_new:e}");
}

/// STRUCTURAL / CLAIM 5+8: the regime the PR's 3-axis bit sweep did not
/// visit — axes whose `normalize` is NOT unit. `rotation_about` divides
/// by `norm()`, so an axis whose norm-squared overflows normalizes to
/// the ZERO vector (not poison), and one at subnormal scale normalizes
/// inexactly. The operator's diagonal `t·(nⱼ²+nₖ²)` is equal to
/// `1 − (t·nᵢ²+c)` only through `|n| = 1`, so in those regimes the new
/// translation and the retired one are not the same map. Measured, not
/// asserted: this row prints the divergence.
#[test]
fn r2_anchor_fixed_point_under_degenerate_normalization() {
    let p = Point3::new(1.0f64, 2.0, -3.0);
    let q = p - Point3::origin();
    let mag = (q.x * q.x + q.y * q.y + q.z * q.z).sqrt();
    for (name, ax) in [
        ("well-scaled", [1.0f64, -2.0, 2.0]),
        ("large-but-ok", [1.0e100, 1.0e100, 1.0e100]),
        ("norm2-overflows", [1.0e160, 1.0, 1.0]),
        ("norm2-overflows-2", [1.0e200, 1.0e200, 0.0]),
        ("subnormal-scale", [1.0e-160, 1.0e-160, 1.0e-160]),
        ("subnormal-2", [1.0e-170, 0.0, 0.0]),
    ] {
        let axis = Vec3::new(ax[0], ax[1], ax[2]);
        let n = axis.normalize();
        let nsq = n.x * n.x + n.y * n.y + n.z * n.z;
        for &angle in &[1.0e-8f64, 1.0, 3.0] {
            let r = Mat3::rotation_about(axis, angle);
            let old_t = q - r * q;
            let new_t = Mat3::identity_minus_rotation_about(axis, angle) * q;
            let old_map = Affine3::from_parts(r, old_t);
            let new_map = Affine3::from_parts(r, new_t);
            let resid = |m: Affine3<f64>| {
                let i = m.transform_point(p);
                (((i.x - p.x).powi(2) + (i.y - p.y).powi(2) + (i.z - p.z).powi(2)).sqrt()) / mag
            };
            println!(
                "{name} |n|^2={nsq:e} angle {angle}: |old_t - new_t| = {:e}, \
                 anchor residual old {:e} new {:e}",
                ((old_t.x - new_t.x).powi(2)
                    + (old_t.y - new_t.y).powi(2)
                    + (old_t.z - new_t.z).powi(2))
                .sqrt(),
                resid(old_map),
                resid(new_map),
            );
        }
    }
}

/// CLAIM 4 (attribution): the PR attributes the whole residual
/// 2.66e-15 in the RevolvedPoint table to `Mat3::rotation_about`'s
/// `1 − cos` floor spread over the placed point (`4.44e-16 × |p| ≈ 3`).
/// Measured directly: how wide is `rotation_about(axis, [0,0]) · p`?
#[test]
fn r2_where_does_the_residue_actually_come_from() {
    let p = Point3::new(
        Interval::from_f64(2.0),
        Interval::from_f64(2.0),
        Interval::from_f64(3.0),
    );
    let mag = (2.0f64 * 2.0 + 2.0 * 2.0 + 3.0 * 3.0).sqrt();
    let axis = Vec3::new(Interval::zero(), Interval::zero(), Interval::one());
    let r = Mat3::rotation_about(axis, Interval::zero());
    for (nm, col) in [("c0", r.c0), ("c1", r.c1), ("c2", r.c2)] {
        println!(
            "R.{nm} widths: {:e} {:e} {:e}",
            w(col.x),
            w(col.y),
            w(col.z)
        );
    }
    let q = p - Point3::origin();
    let rp = r * q;
    println!(
        "|p| = {mag}; width(R*p) = {:e} {:e} {:e}",
        w(rp.x),
        w(rp.y),
        w(rp.z)
    );
    let a = Affine3::rotation_about_axis(p, axis, Interval::zero());
    let img = a.transform_point(p);
    println!(
        "width(transform_point(p)) = {:e} {:e} {:e}",
        w(img.x),
        w(img.y),
        w(img.z)
    );
}
