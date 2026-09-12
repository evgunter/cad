//! **Adopted from a reviewer probe** (the CERT-N3 dual review) as an
//! ordinary row: a second independent conic-box corpus, the branch-cut rows (extremal angle exactly π, bracket rectangles on the axes) and the descending-run wide-bracket row that shows the endpoint hull is load-bearing for span coverage.
//!
//! CERT-N3 R2 blinded-review probes — probe branch only.
//!
//! An INDEPENDENT adversarial corpus for the exact conic box adopted by
//! `topo::boolean::boxes::edge_box` (row S235): the box must be a
//! superset of the arc's locus at every scalar the boolean reads it at.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::float_cmp)]
#![allow(clippy::approx_constant, clippy::vec_init_then_push)]

use bvh::Aabb;
use geom::Curve3;
use geom::curves::boxes::{circle_arc_aabb, conic_arc_aabb, ellipse_arc_aabb};
use geom_core::{Point3, Vec3};

const SAMPLES: u32 = 100_000;

fn escapes(b: &Aabb, p: Point3<f64>, slack: f64) -> bool {
    !(p.x >= b.min_x - slack
        && p.x <= b.max_x + slack
        && p.y >= b.min_y - slack
        && p.y <= b.max_y + slack
        && p.z >= b.min_z - slack
        && p.z <= b.max_z + slack)
}

/// Orthonormal (axis, u_ref) with the axis tilted by `tilt` toward x and
/// then rotated about z by `spin`; `u_ref` further rotated in the plane
/// by `phi`.
fn frame(tilt: f64, spin: f64, phi: f64) -> (Vec3<f64>, Vec3<f64>) {
    let axis = Vec3::new(tilt.sin() * spin.cos(), tilt.sin() * spin.sin(), tilt.cos()).normalize();
    let helper = if axis.z.abs() < 0.9 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let u0 = axis.cross(helper).normalize();
    let v0 = axis.cross(u0);
    let u_ref = (u0 * phi.cos() + v0 * phi.sin()).normalize();
    (axis, u_ref)
}

/// Every span shape the brief names, as `(name, t0, t1)` relative to a
/// per-carrier extremal angle `phi` supplied by the caller.
fn spans(phi: f64) -> Vec<(String, f64, f64)> {
    let tau = core::f64::consts::TAU;
    let mut out = Vec::new();
    // strictly inside
    out.push((
        "crosses phi strictly inside".to_string(),
        phi - 0.3,
        phi + 0.4,
    ));
    // exactly at an endpoint (both ends)
    out.push(("phi exactly at t0".to_string(), phi, phi + 0.9));
    out.push(("phi exactly at t1".to_string(), phi - 0.9, phi));
    // within 1e-9 rad of an endpoint, inside and outside
    out.push(("phi 1e-9 inside t0".to_string(), phi - 1e-9, phi + 0.5));
    out.push(("phi 1e-9 outside t0".to_string(), phi + 1e-9, phi + 0.5));
    out.push(("phi 1e-9 inside t1".to_string(), phi - 0.5, phi + 1e-9));
    out.push(("phi 1e-9 outside t1".to_string(), phi - 0.5, phi - 1e-9));
    // wraps past 2pi
    out.push((
        "wraps past 2pi".to_string(),
        phi - 0.2,
        phi - 0.2 + tau + 0.9,
    ));
    out.push(("wraps twice".to_string(), -1.0, -1.0 + 2.0 * tau + 0.3));
    // starts near the atan2 cut
    out.push((
        "starts at the atan2 cut".to_string(),
        core::f64::consts::PI - 1e-12,
        core::f64::consts::PI + 1.3,
    ));
    out.push((
        "ends at the atan2 cut".to_string(),
        1.0,
        core::f64::consts::PI,
    ));
    // degenerate
    out.push(("degenerate 1e-6".to_string(), phi - 5e-7, phi + 5e-7));
    out.push(("degenerate 1e-6 off phi".to_string(), 0.77, 0.77 + 1e-6));
    // within 1e-6 of a full turn
    out.push(("full turn minus 1e-6".to_string(), 0.31, 0.31 + tau - 1e-6));
    out.push(("full turn plus 1e-6".to_string(), 0.31, 0.31 + tau + 1e-6));
    out.push(("exact full turn".to_string(), 0.0, tau));
    // descending run (t0 > t1)
    out.push(("descending across phi".to_string(), phi + 0.4, phi - 0.3));
    out
}

/// **The circle corpus.** Radii 1e-4 .. 1e4, `u_ref` at every octant
/// boundary, axes tilted, and every span shape above measured against
/// the coordinate's own extremal angle.
#[test]
fn n3r2_circle_box_contains_a_dense_sample() {
    let mut checked = 0usize;
    for &r in &[1e-4_f64, 1e-2, 1.0, 100.0, 1e4] {
        for oct in 0..8u32 {
            let phi_u = f64::from(oct) * core::f64::consts::FRAC_PI_4;
            for &(tilt, spin) in &[
                (0.0, 0.0),
                (0.4, 0.7),
                (1.2, 2.6),
                (core::f64::consts::FRAC_PI_2, 0.0),
            ] {
                let (axis, u_ref) = frame(tilt, spin, phi_u);
                let carrier: Curve3<f64> = Curve3::Circle {
                    center: Point3::new(0.3 * r, -0.2 * r, 0.11 * r),
                    axis,
                    radius: r,
                    u_ref,
                };
                let v_ref = axis.cross(u_ref);
                // The x-coordinate's extremal angle for this carrier.
                let phi = (v_ref.x * r).atan2(u_ref.x * r);
                for (name, t0, t1) in spans(phi) {
                    let b = circle_arc_aabb(&carrier, t0, t1, carrier.eval(t0), carrier.eval(t1))
                        .unwrap();
                    let slack = 1e-12 * (1.0 + r);
                    for i in 0..=SAMPLES {
                        let t = t0 + (t1 - t0) * f64::from(i) / f64::from(SAMPLES);
                        let p = carrier.eval(t);
                        assert!(
                            !escapes(&b, p, slack),
                            "circle r={r} oct={oct} tilt={tilt} {name}: t={t} {p:?} left {b:?}"
                        );
                    }
                    checked += 1;
                }
            }
        }
    }
    assert!(checked > 500, "corpus too small: {checked}");
}

/// **The ellipse corpus.** Tilts to 89.99°, axis ratios to 1e3.
#[test]
fn n3r2_ellipse_box_contains_a_dense_sample() {
    let mut checked = 0usize;
    for &ratio in &[1.0_f64, 10.0, 1e3] {
        for &major in &[1e-4_f64, 1.0, 1e4] {
            let minor = major / ratio;
            for &tilt_deg in &[0.0_f64, 20.0, 60.0, 89.99] {
                for oct in [0u32, 1, 3, 5] {
                    let phi_u = f64::from(oct) * core::f64::consts::FRAC_PI_4;
                    let (axis, u_ref) = frame(tilt_deg.to_radians(), 0.9, phi_u);
                    let carrier: Curve3<f64> = Curve3::Ellipse {
                        center: Point3::new(0.0, 0.0, 0.0),
                        axis,
                        major,
                        minor,
                        u_ref,
                    };
                    let v_ref = axis.cross(u_ref);
                    let phi = (minor * v_ref.y).atan2(major * u_ref.y);
                    for (name, t0, t1) in spans(phi) {
                        let b =
                            ellipse_arc_aabb(&carrier, t0, t1, carrier.eval(t0), carrier.eval(t1))
                                .unwrap();
                        let slack = 1e-12 * (1.0 + major);
                        for i in 0..=SAMPLES {
                            let t = t0 + (t1 - t0) * f64::from(i) / f64::from(SAMPLES);
                            let p = carrier.eval(t);
                            assert!(
                                !escapes(&b, p, slack),
                                "ellipse major={major} ratio={ratio} tilt={tilt_deg} {name}: \
                                 t={t} {p:?} left {b:?}"
                            );
                        }
                        checked += 1;
                    }
                }
            }
        }
    }
    assert!(checked > 500, "corpus too small: {checked}");
}

/// The dispatcher agrees with the kind door it delegates to, and
/// refuses the two non-conic kinds.
#[test]
fn n3r2_dispatcher_is_its_two_arms() {
    let c: Curve3<f64> = Curve3::Circle {
        center: Point3::new(1.0, 2.0, 3.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 2.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let (t0, t1) = (0.2, 2.0);
    assert_eq!(
        format!("{:?}", conic_arc_aabb(&c, t0, t1, c.eval(t0), c.eval(t1))),
        format!("{:?}", circle_arc_aabb(&c, t0, t1, c.eval(t0), c.eval(t1)))
    );
    let e: Curve3<f64> = Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major: 2.0,
        minor: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    assert_eq!(
        format!("{:?}", conic_arc_aabb(&e, t0, t1, e.eval(t0), e.eval(t1))),
        format!("{:?}", ellipse_arc_aabb(&e, t0, t1, e.eval(t0), e.eval(t1)))
    );
    let l: Curve3<f64> = Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    };
    let p = Point3::new(0.0, 0.0, 0.0);
    assert!(conic_arc_aabb(&l, 0.0, 1.0, p, p).is_none());
    let n = Curve3::<f64>::nurbs_placeholder();
    assert!(conic_arc_aabb(&n, 0.0, 1.0, p, p).is_none());
}

/// **Dual64 door.** The value channel brackets as a point, so the box
/// must be the `f64` box; the corpus is the circle corpus, thinned.
#[test]
fn n3r2_dual_box_contains_a_dense_sample() {
    use geom_core::Dual64;
    let d = Dual64::constant;
    for &r in &[1e-4_f64, 1.0, 1e4] {
        for oct in 0..8u32 {
            let (axis, u_ref) = frame(0.4, 0.7, f64::from(oct) * core::f64::consts::FRAC_PI_4);
            let carrier_d: Curve3<Dual64> = Curve3::Circle {
                center: Point3::new(d(0.0), d(0.0), d(0.0)),
                axis: Vec3::new(d(axis.x), d(axis.y), d(axis.z)),
                radius: Dual64::variable(r),
                u_ref: Vec3::new(d(u_ref.x), d(u_ref.y), d(u_ref.z)),
            };
            let real: Curve3<f64> = Curve3::Circle {
                center: Point3::new(0.0, 0.0, 0.0),
                axis,
                radius: r,
                u_ref,
            };
            let v_ref = axis.cross(u_ref);
            let phi = (v_ref.z * r).atan2(u_ref.z * r);
            for (name, t0, t1) in spans(phi) {
                let b = conic_arc_aabb(
                    &carrier_d,
                    d(t0),
                    d(t1),
                    carrier_d.eval(d(t0)),
                    carrier_d.eval(d(t1)),
                )
                .unwrap();
                let slack = 1e-12 * (1.0 + r);
                for i in 0..=2000u32 {
                    let t = t0 + (t1 - t0) * f64::from(i) / 2000.0;
                    assert!(
                        !escapes(&b, real.eval(t), slack),
                        "dual r={r} oct={oct} {name}: t={t} left {b:?}"
                    );
                }
            }
        }
    }
}

/// **The wide-bracket (Interval) lane, ASCENDING runs.** Brackets on
/// `u_ref`, radius, centre and the two span ends, all straddling the
/// coordinate's extremal angle; every `f64` realization of the bracket
/// family must lie in the one box.
#[cfg(feature = "interval")]
#[test]
fn n3r2_interval_box_dominates_every_realization_ascending() {
    use geom_core::{Bounds, Interval, Real};
    let iv = <Interval as Real>::from_f64;
    for w in [1e-9_f64, 1e-6, 1e-3, 1e-1] {
        for &r in &[1e-3_f64, 1.0, 1e3] {
            for oct in 0..8u32 {
                let phi_u = f64::from(oct) * core::f64::consts::FRAC_PI_4;
                let (axis, u0) = frame(0.0, 0.0, phi_u);
                let v0 = axis.cross(u0);
                let phi = (v0.x).atan2(u0.x);
                // Span brackets straddling the extremal angle at both ends.
                for &(t0c, t1c) in &[
                    (phi - 0.5, phi),
                    (phi, phi + 0.5),
                    (phi - 0.5, phi + 0.5),
                    (phi + 1e-12, phi + 0.5),
                ] {
                    let t0 = Interval::from_bounds(t0c - w, t0c + w);
                    let t1 = Interval::from_bounds(t1c - w, t1c + w);
                    let carrier: Curve3<Interval> = Curve3::Circle {
                        center: Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                        axis: Vec3::new(iv(axis.x), iv(axis.y), iv(axis.z)),
                        radius: Interval::from_bounds(r * (1.0 - w), r * (1.0 + w)),
                        u_ref: Vec3::new(
                            Interval::from_bounds(u0.x - w, u0.x + w),
                            iv(u0.y),
                            iv(u0.z),
                        ),
                    };
                    let b = conic_arc_aabb(&carrier, t0, t1, carrier.eval(t0), carrier.eval(t1))
                        .unwrap();
                    for i in 0..=6u32 {
                        let f = f64::from(i) / 6.0;
                        let ux = u0.x - w + 2.0 * w * f;
                        let rr = r * (1.0 - w) + 2.0 * r * w * f;
                        let real: Curve3<f64> = Curve3::Circle {
                            center: Point3::new(0.0, 0.0, 0.0),
                            axis,
                            radius: rr,
                            u_ref: Vec3::new(ux, u0.y, u0.z),
                        };
                        // Every realization of the two span ends.
                        for (ra, rb) in [(t0.lo(), t1.hi()), (t0.hi(), t1.lo()), (t0.lo(), t1.lo())]
                        {
                            for j in 0..=400u32 {
                                let t = ra + (rb - ra) * f64::from(j) / 400.0;
                                assert!(
                                    !escapes(&b, real.eval(t), 1e-9 * (1.0 + r)),
                                    "iv w={w:e} r={r} oct={oct} span=({t0c},{t1c}): t={t} \
                                     escaped {b:?}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

/// **The wide-bracket lane, DESCENDING runs** (`param_start >
/// param_end`, which `boxes.rs`'s own fixture doc says the split lane
/// mints). The span the door reads is
/// `[min(t0.lo(), t1.hi()), max(t0.lo(), t1.hi())]`, so on a descending
/// run it takes the INNER end of each bracket.
#[cfg(feature = "interval")]
#[test]
fn n3r2_interval_box_dominates_every_realization_descending() {
    use geom_core::{Bounds, Interval, Real};
    let iv = <Interval as Real>::from_f64;
    let axis = Vec3::new(0.0, 0.0, 1.0);
    let u0 = Vec3::new(1.0, 0.0, 0.0);
    // x's extremal angle is 0; y's is pi/2.
    for w in [1e-9_f64, 1e-7, 1e-5, 1e-3] {
        // A descending run whose OUTER realization reaches pi/2 (y's
        // extremum) but whose inner realization does not.
        let hi_c = core::f64::consts::FRAC_PI_2;
        let t0 = Interval::from_bounds(hi_c - w, hi_c + w); // start (larger)
        let t1 = Interval::from_bounds(0.2 - w, 0.2 + w); // end (smaller)
        let carrier: Curve3<Interval> = Curve3::Circle {
            center: Point3::new(iv(0.0), iv(0.0), iv(0.0)),
            axis: Vec3::new(iv(axis.x), iv(axis.y), iv(axis.z)),
            radius: iv(1.0),
            u_ref: Vec3::new(iv(u0.x), iv(u0.y), iv(u0.z)),
        };
        let b = conic_arc_aabb(&carrier, t0, t1, carrier.eval(t0), carrier.eval(t1)).unwrap();
        let real: Curve3<f64> = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis,
            radius: 1.0,
            u_ref: u0,
        };
        // The widest realization: from t1.lo() up to t0.hi().
        for j in 0..=4000u32 {
            let t = t1.lo() + (t0.hi() - t1.lo()) * f64::from(j) / 4000.0;
            assert!(
                !escapes(&b, real.eval(t), 1e-12),
                "descending w={w:e}: t={t} {:?} escaped {b:?}",
                real.eval(t)
            );
        }
    }
}

/// **The branch-cut fix, targeted.** A conic whose plane's normal tilts
/// in the `xz` plane puts `v = axis × u_ref` exactly along `y`, so for
/// the `x` and `z` coordinates `v_i` is EXACTLY zero and the extremal
/// angle is exactly `0` or `π` — the rectangle that touches the
/// negative `u` axis (and, at `u > 0`, the positive one). Both signs of
/// `u_i`, both sides of the box, at f64 and under brackets whose
/// rectangle touches an axis exactly or has zero width on one
/// coordinate.
#[test]
fn n3r2_extremal_angle_exactly_pi_stays_sound_and_is_tight() {
    for &alpha in &[0.0_f64, 0.3, 1.2, 1.5707] {
        // axis in the xz plane, u_ref in the xz plane, orthogonal.
        let axis = Vec3::new(alpha.sin(), 0.0, alpha.cos());
        for &flip in &[1.0_f64, -1.0] {
            let u_ref = Vec3::new(flip * alpha.cos(), 0.0, -flip * alpha.sin());
            let v_ref = axis.cross(u_ref);
            assert!(v_ref.x == 0.0 && v_ref.z == 0.0, "v is along y: {v_ref:?}");
            let carrier: Curve3<f64> = Curve3::Circle {
                center: Point3::new(0.0, 0.0, 0.0),
                axis,
                radius: 1.0,
                u_ref,
            };
            for (name, t0, t1) in spans(core::f64::consts::PI) {
                let b =
                    circle_arc_aabb(&carrier, t0, t1, carrier.eval(t0), carrier.eval(t1)).unwrap();
                for i in 0..=20_000u32 {
                    let t = t0 + (t1 - t0) * f64::from(i) / 20_000.0;
                    assert!(
                        !escapes(&b, carrier.eval(t), 1e-12),
                        "pi-case alpha={alpha} flip={flip} {name}: t={t} escaped {b:?}"
                    );
                }
                // Tightness: a run that never crosses pi must not claim
                // the far side of the circle on x.
                if name == "degenerate 1e-6 off phi" {
                    let (lo, hi) = (
                        carrier.eval(t0).x.min(carrier.eval(t1).x),
                        carrier.eval(t0).x.max(carrier.eval(t1).x),
                    );
                    assert!(
                        b.min_x > lo - 1e-6 && b.max_x < hi + 1e-6,
                        "alpha={alpha} flip={flip}: the box is not span-tight: {b:?}"
                    );
                }
            }
        }
    }
}

/// The same shape with BRACKETS placed exactly on the axes: a rectangle
/// touching the positive `u` axis at a corner, one straddling the
/// origin, and one of zero width on a coordinate.
#[cfg(feature = "interval")]
#[test]
fn n3r2_bracket_rectangles_on_the_axes_stay_sound() {
    use geom_core::{Bounds, Interval, Real};
    let iv = <Interval as Real>::from_f64;
    for &(ux_lo, ux_hi) in &[
        (0.0, 0.4),   // touches the positive u axis at a corner
        (-0.4, 0.0),  // touches the negative u axis at a corner
        (-0.4, 0.4),  // straddles: origin inside the rectangle
        (0.3, 0.3),   // zero width, positive
        (-0.3, -0.3), // zero width, negative
    ] {
        for &(vy_lo, vy_hi) in &[(0.0, 0.0), (-0.2, 0.2), (0.0, 0.5), (-0.5, 0.0)] {
            let carrier: Curve3<Interval> = Curve3::Circle {
                center: Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                radius: iv(1.0),
                u_ref: Vec3::new(
                    Interval::from_bounds(ux_lo, ux_hi),
                    Interval::from_bounds(vy_lo, vy_hi),
                    iv(0.0),
                ),
            };
            for &(t0c, t1c) in &[(0.0, 1.0), (1.0, 4.0), (2.5, 3.8), (0.0, 6.3), (3.0, 3.2)] {
                let (t0, t1) = (iv(t0c), iv(t1c));
                let b =
                    conic_arc_aabb(&carrier, t0, t1, carrier.eval(t0), carrier.eval(t1)).unwrap();
                for i in 0..=8u32 {
                    let f = f64::from(i) / 8.0;
                    let real: Curve3<f64> = Curve3::Circle {
                        center: Point3::new(0.0, 0.0, 0.0),
                        axis: Vec3::new(0.0, 0.0, 1.0),
                        radius: 1.0,
                        u_ref: Vec3::new(
                            ux_lo + (ux_hi - ux_lo) * f,
                            vy_lo + (vy_hi - vy_lo) * f,
                            0.0,
                        ),
                    };
                    for j in 0..=1000u32 {
                        let t = t0c + (t1c - t0c) * f64::from(j) / 1000.0;
                        assert!(
                            !escapes(&b, real.eval(t), 1e-12),
                            "u=({ux_lo},{ux_hi}) v=({vy_lo},{vy_hi}) span=({t0c},{t1c}): \
                             t={t} escaped {b:?}"
                        );
                    }
                }
            }
        }
        let _ = iv(0.0).lo();
    }
}
