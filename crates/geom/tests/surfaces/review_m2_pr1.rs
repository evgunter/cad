//! Adversarial e2e review artifact for M2 PR 1 (analytic evaluators +
//! Real-surface additions, 2026-07-17), promoted from the reviewer's
//! session scratchpad into CI per the standing convention. These are
//! **independent derivations** — finite-difference oracles, Rodrigues
//! rotations, hand-computed partials, the PR 2 bulge-arc and PR 3
//! tier-3 consumption patterns — do not "simplify" them to match
//! shipped fixtures; the independence is the regression value.
//!
//! Promotion adaptations (mechanical only): the standard test-lint
//! allows below; everything else is verbatim. Grouped under
//! `tests/surfaces/` because it is the surface PR's review artifact;
//! it consumes curves AND surfaces, both through public API only.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]
// The range-shaped assertions are the reviewer's derivation shapes —
// kept verbatim per the do-not-simplify rule.
#![allow(clippy::manual_range_contains)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, PI, TAU};

use geom::Curve3;
use geom::Surface;
use geom_core::{Affine3, Dual, Point2, Point3, Real, Vec3};

// ---------------------------------------------------------------------
// Independent reference machinery (mine, not the kernel's)
// ---------------------------------------------------------------------

/// Central finite difference of a vector function.
fn fd<F: Fn(f64) -> Point3<f64>>(f: F, t: f64, h: f64) -> Vec3<f64> {
    let a = f(t - h);
    let b = f(t + h);
    Vec3::new(
        (b.x - a.x) / (2.0 * h),
        (b.y - a.y) / (2.0 * h),
        (b.z - a.z) / (2.0 * h),
    )
}

fn fd_v<F: Fn(f64) -> Vec3<f64>>(f: F, t: f64, h: f64) -> Vec3<f64> {
    let a = f(t - h);
    let b = f(t + h);
    Vec3::new(
        (b.x - a.x) / (2.0 * h),
        (b.y - a.y) / (2.0 * h),
        (b.z - a.z) / (2.0 * h),
    )
}

fn close(a: Vec3<f64>, b: Vec3<f64>, tol: f64) -> bool {
    (a.x - b.x).abs() <= tol && (a.y - b.y).abs() <= tol && (a.z - b.z).abs() <= tol
}

fn tilted_axis() -> Vec3<f64> {
    Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0)
}
fn tilted_uref() -> Vec3<f64> {
    Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0)
}

fn all_surfaces() -> Vec<(&'static str, Surface<f64>)> {
    let c = Point3::new(-0.5, 4.0, 1.25);
    vec![
        (
            "plane",
            Surface::Plane {
                origin: c,
                normal: tilted_axis(),
                u_ref: tilted_uref(),
            },
        ),
        (
            "cylinder",
            Surface::Cylinder {
                origin: c,
                axis: tilted_axis(),
                radius: 2.5,
                u_ref: tilted_uref(),
            },
        ),
        (
            "cone",
            Surface::Cone {
                apex: c,
                axis: tilted_axis(),
                half_angle: FRAC_PI_6,
                u_ref: tilted_uref(),
            },
        ),
        (
            "sphere",
            Surface::Sphere {
                center: c,
                radius: 2.5,
                axis: tilted_axis(),
                u_ref: tilted_uref(),
            },
        ),
        (
            "torus",
            Surface::Torus {
                center: c,
                axis: tilted_axis(),
                major_radius: 3.0,
                minor_radius: 1.25,
                u_ref: tilted_uref(),
            },
        ),
    ]
}

// ---------------------------------------------------------------------
// 4. Evaluator correctness: finite differences falsify every derivative
// ---------------------------------------------------------------------

/// Every claimed first/second partial of every variant is checked
/// against central differences of `eval` — a fully independent oracle.
#[test]
fn all_partials_match_finite_differences() {
    let h = 1e-5;
    let pts = [
        (0.0, 0.4),
        (0.7, 0.4),
        (2.3, -1.1),
        (-4.0, 0.9),
        (11.0, -0.3),
    ];
    for (name, s) in all_surfaces() {
        for (u, v) in pts {
            let du = s.deriv_u(u, v);
            let du_fd = fd(|x| s.eval(x, v), u, h);
            assert!(
                close(du, du_fd, 1e-8),
                "{name} ∂u at ({u},{v}): {du:?} vs {du_fd:?}"
            );

            let dv = s.deriv_v(u, v);
            let dv_fd = fd(|x| s.eval(u, x), v, h);
            assert!(close(dv, dv_fd, 1e-8), "{name} ∂v at ({u},{v})");

            let duu = s.deriv_uu(u, v);
            let duu_fd = fd_v(|x| s.deriv_u(x, v), u, h);
            assert!(close(duu, duu_fd, 1e-8), "{name} ∂uu at ({u},{v})");

            let duv = s.deriv_uv(u, v);
            let duv_fd = fd_v(|x| s.deriv_u(u, x), v, h);
            assert!(close(duv, duv_fd, 1e-8), "{name} ∂uv at ({u},{v})");
            // Mixed partial the other way too (symmetry vs my oracle).
            let dvu_fd = fd_v(|x| s.deriv_v(x, v), u, h);
            assert!(close(duv, dvu_fd, 1e-8), "{name} ∂vu at ({u},{v})");

            let dvv = s.deriv_vv(u, v);
            let dvv_fd = fd_v(|x| s.deriv_v(u, x), v, h);
            assert!(close(dvv, dvv_fd, 1e-8), "{name} ∂vv at ({u},{v})");

            // The normal is MY cross product of the FD partials.
            let n = s.normal(u, v);
            let my_n = du_fd.cross(dv_fd).normalize();
            assert!(close(n, my_n, 1e-7), "{name} normal at ({u},{v})");
        }
    }
}

/// Hand-computed torus ∂uv at a nontrivial point (axis-aligned so the
/// closed form is checkable by hand):
/// S = c + radial(u)·(R + r·cos v) + z·(r·sin v);
/// ∂uv = ∂v[tangential·(R + r cos v)] = tangential·(−r·sin v).
/// At u = π/3, v = π/4, R = 3, r = 1:
/// tangential = (−sin π/3, cos π/3, 0) = (−√3/2, 1/2, 0),
/// ∂uv = (√3/2, −1/2, 0)·(√2/2) ≈ (0.61237, −0.35355, 0).
#[test]
fn torus_duv_hand_computed() {
    let t = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 3.0,
        minor_radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let duv = t.deriv_uv(PI / 3.0, FRAC_PI_4);
    let expect = Vec3::new(
        (3.0f64.sqrt() / 2.0) * (2.0f64.sqrt() / 2.0),
        (-0.5) * (2.0f64.sqrt() / 2.0),
        0.0,
    );
    assert!(close(duv, expect, 1e-15), "{duv:?} vs {expect:?}");
}

/// Independent Rodrigues-based oracle for the tilted sphere: rotate the
/// meridian point about the axis by u, starting from the u_ref meridian.
#[test]
fn tilted_sphere_matches_rodrigues_oracle() {
    let center = Point3::new(-0.5, 4.0, 1.25);
    let (axis, uref, r) = (tilted_axis(), tilted_uref(), 2.5);
    let s = Surface::Sphere {
        center,
        radius: r,
        axis,
        u_ref: uref,
    };
    for (u, v) in [(0.9, 0.3), (-2.0, -0.8), (5.0, 1.2)] {
        // Meridian point at azimuth 0: center + (uref·cos v + axis·sin v)·r,
        // then rotate about the axis line through center by u.
        let meridian = center + (uref * v.cos() + axis * v.sin()) * r;
        let rot = Affine3::rotation_about_axis(center, axis, u);
        let expect = rot.transform_point(meridian);
        let got = s.eval(u, v);
        assert!(
            (got.x - expect.x).abs() <= 1e-13
                && (got.y - expect.y).abs() <= 1e-13
                && (got.z - expect.z).abs() <= 1e-13,
            "sphere({u},{v}): {got:?} vs {expect:?}"
        );
    }
}

/// Cone apex: exactly at the apex the normal is poison; near the apex it
/// is finite and equals the documented closed form; v < 0 flips it.
#[test]
fn cone_apex_poison_and_near_apex_honesty() {
    let c = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: FRAC_PI_6,
        u_ref: Vec3::unit_x(),
    };
    for u in [0.0, 0.9, -3.0, 100.0] {
        let n = c.normal(u, 0.0);
        assert!(
            n.x.is_nan() && n.y.is_nan() && n.z.is_nan(),
            "apex at u={u}"
        );
    }
    // Near-apex: tiny but nonzero v gives the same normal as v = 1
    // (the normal is v-independent on a nappe).
    let far = c.normal(0.3, 1.0);
    for v in [1e-6, 1e-12, 1e-100] {
        let near = c.normal(0.3, v);
        assert!(close(near, far, 1e-12), "v = {v}: {near:?} vs {far:?}");
    }
    // OBSERVATION (report material): below v ≈ 1e-162 the cross
    // product's norm² underflows to 0 and normalize divides nonzero
    // components by an exact 0 — the "unit normal" is (±∞, ±∞, ±∞),
    // which is NOT NaN-poison (∞ is not f64 poison per the repo's
    // stance). Any downstream dot/certification does go NaN, so it
    // still fails loudly one step later. Pin the observed behavior.
    let deep = c.normal(0.3, 1e-300);
    assert!(deep.x.is_infinite() && deep.z.is_infinite());
    // And the band just above (subnormal squares) is degraded but
    // finite: |n| deviates from 1 by ~1e-5 at v = 1e-160.
    let degraded = c.normal(0.3, 1e-160);
    assert!(degraded.x.is_finite() && (degraded.norm() - 1.0).abs() < 1e-4);
    // Mirror nappe: negated normal.
    let neg = c.normal(0.3, -1.0);
    assert!(close(neg, far * -1.0, 1e-15));
}

/// The u seam is at u_ref for all four azimuthal surfaces: at u = 0 the
/// point's axis-rejected radial component is a POSITIVE multiple of
/// u_ref (checked via my own reject_from-free math).
#[test]
fn seam_at_uref_everywhere() {
    let center = Point3::new(-0.5, 4.0, 1.25);
    let (axis, uref) = (tilted_axis(), tilted_uref());
    for (name, s) in all_surfaces() {
        if name == "plane" {
            continue;
        }
        let p = s.eval(0.0, 0.4);
        let d = p - center;
        let radial = d - axis * d.dot(axis);
        let coeff = radial.dot(uref);
        assert!(coeff > 0.1, "{name}: seam radial coeff {coeff}");
        // And the radial part is PARALLEL to u_ref (cross ≈ 0).
        let cr = radial.cross(uref);
        assert!(
            cr.norm() <= 1e-12 * (1.0 + radial.norm()),
            "{name}: seam direction"
        );
    }
}

/// Latitude (not colatitude): sphere v = 0 is the equator through
/// u_ref, v = +π/2 is the +axis pole. Torus v = 0 is the OUTER equator.
#[test]
fn latitude_conventions() {
    let s = Surface::Sphere {
        center: Point3::origin(),
        radius: 2.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let eq = s.eval(0.0, 0.0);
    assert_eq!((eq.x, eq.y, eq.z), (2.0, 0.0, 0.0));
    let np = s.eval(0.3, FRAC_PI_2);
    assert!((np.z - 2.0).abs() <= 1e-15);
    let t = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 3.0,
        minor_radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let outer = t.eval(0.0, 0.0);
    assert_eq!(outer.x, 4.0); // R + r: outer equator, farthest from axis
    let top = t.eval(0.0, FRAC_PI_2);
    assert!((top.z - 1.0).abs() <= 1e-15 && (top.x - 3.0).abs() <= 1e-15);
}

/// Non-unit axis: documented garbage-out — well-defined values, no
/// poison, no panic; the "circle" becomes non-circular but total.
#[test]
fn non_unit_axis_is_garbage_out_not_poison() {
    let c = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 2.0), // NOT unit
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    // v_ref = axis × u_ref has norm 2 ⇒ ellipse with semi-axes 1 and 2.
    let p = c.eval(FRAC_PI_2);
    assert!((p.y - 2.0).abs() <= 1e-15, "sheared locus is well-defined");
    assert!(!p.x.is_nan() && !p.y.is_nan() && !p.z.is_nan());
    let s = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 3.0),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let q = s.eval(1.0, 1.0);
    assert!(!q.x.is_nan() && !q.y.is_nan() && !q.z.is_nan());
}

// ---------------------------------------------------------------------
// 7. PR 2 dry run: bulge-chain arc → Circle carrier
// ---------------------------------------------------------------------

/// Builds the Circle carrier for a bulge-chain arc exactly the way PR 2
/// will: sketch plane placement, winding-chosen axis, u_ref at the
/// start vertex, end-vertex parameter via atan2 + reduce_periodic —
/// then checks the whole story composes with the he_plus contract.
#[test]
fn bulge_arc_carrier_composes() {
    // Sketch plane: a tilted placement (orthonormal, right-handed).
    let plane_origin = Point3::new(10.0, -3.0, 2.0);
    let ex = tilted_uref();
    let ez = tilted_axis();
    let ey = ez.cross(ex);
    let to3d = |p: Point2<f64>| plane_origin + ex * p.x + ey * p.y;

    // One bulge-chain segment: P0 → P1 with bulge b = tan(θ/4).
    for (p0, p1, bulge) in [
        (Point2::new(1.0, 0.0), Point2::new(0.0, 1.0), FRAC_PI_8_TAN), // CCW quarter arc
        (Point2::new(1.0, 0.0), Point2::new(0.0, 1.0), -FRAC_PI_8_TAN), // CW quarter arc
        (Point2::new(-2.0, 0.5), Point2::new(1.5, 2.0), 0.7),          // generic CCW
        (Point2::new(-2.0, 0.5), Point2::new(1.5, 2.0), -2.5),         // generic CW, θ > π
    ] {
        let theta = 4.0 * bulge.atan(); // signed included angle
        let chord = ((p1.x - p0.x).powi(2) + (p1.y - p0.y).powi(2)).sqrt();
        let radius = chord / (2.0 * (theta.abs() / 2.0).sin());
        // Center: rotate the chord direction by +90° scaled — the
        // standard bulge construction (apothem offset from midpoint).
        let mx = (p0.x + p1.x) / 2.0;
        let my = (p0.y + p1.y) / 2.0;
        let apothem = radius * (theta.abs() / 2.0).cos();
        // Left normal of the chord for CCW (bulge > 0) is center side
        // when |θ| < π; the sign of the apothem offset flips with both
        // bulge sign and |θ| > π — encode via cot: offset = c/2·cot(θ/2).
        let half_cot = (theta / 2.0).cos() / (theta / 2.0).sin();
        let _ = apothem;
        let nx = -(p1.y - p0.y) / chord;
        let ny = (p1.x - p0.x) / chord;
        let cx = mx + nx * (chord / 2.0) * half_cot;
        let cy = my + ny * (chord / 2.0) * half_cot;
        // (sign convention checked below by |C−P0| = r.)
        let center2 = Point2::new(cx, cy);
        let d0 = ((p0.x - cx).powi(2) + (p0.y - cy).powi(2)).sqrt();
        assert!(
            (d0 - radius).abs() <= 1e-12 * radius,
            "bulge {bulge}: my center construction is self-consistent"
        );

        // 3-D carrier: axis = ±plane normal by winding; u_ref at P0.
        let center = to3d(center2);
        let axis = if bulge >= 0.0 { ez } else { ez * -1.0 };
        let u_ref = (to3d(p0) - center) * (1.0 / radius);
        let carrier = Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        };

        // Start vertex parameter: 0 by construction (u_ref at start).
        let t0 = 0.0;
        let s0 = carrier.eval(t0);
        let q0 = to3d(p0);
        assert!(
            (s0.x - q0.x).abs() <= 1e-12
                && (s0.y - q0.y).abs() <= 1e-12
                && (s0.z - q0.z).abs() <= 1e-12,
            "bulge {bulge}: start vertex"
        );

        // End vertex parameter: atan2 in the carrier frame, canonical
        // representative via reduce_periodic — the PR 2 recipe.
        let v_ref = axis.cross(u_ref);
        let d1 = to3d(p1) - center;
        let t1_raw = d1.dot(v_ref).atan2(d1.dot(u_ref));
        let t1 = t1_raw.reduce_periodic(TAU);
        assert!(
            (t1 - theta.abs()).abs() <= 1e-12,
            "bulge {bulge}: t1 = {t1} vs |θ| = {}",
            theta.abs()
        );
        // he_plus forward contract: increasing parameter runs start→end,
        // i.e. t0 = 0 < t1 < 2π and eval(t1) is the end vertex.
        assert!(t1 > 0.0 && t1 < TAU, "bulge {bulge}: parameter interval");
        let s1 = carrier.eval(t1);
        let q1 = to3d(p1);
        assert!(
            (s1.x - q1.x).abs() <= 1e-12
                && (s1.y - q1.y).abs() <= 1e-12
                && (s1.z - q1.z).abs() <= 1e-12,
            "bulge {bulge}: end vertex"
        );
        // Midpoint sits on the bulge side: the sagitta point. Its
        // distance from the chord midpoint is r − apothem·sign… just
        // check it is r from center and NOT the chord midpoint side
        // reversal: (mid − center)·(P0 − center + P1 − center) > 0 for
        // |θ| < π and the tangent at t0 leans toward P1 for all cases.
        let tangent0 = carrier.deriv(t0);
        let toward_end = q1 - q0;
        assert!(
            tangent0.dot(toward_end) > 0.0 || theta.abs() > PI,
            "bulge {bulge}: departure direction"
        );
    }
}

const FRAC_PI_8_TAN: f64 = 0.41421356237309503; // tan(π/8), the quarter-arc bulge

// ---------------------------------------------------------------------
// 7b. PR 3 dry run: tier-3 residual certification of a cached curve
// ---------------------------------------------------------------------

/// Certifies a cached Curve3 against the implicit forms of the two
/// surfaces it claims to lie on (a cylinder∩plane circle), exactly the
/// shape of PR 3's tier-3 residual pass — then falsifies a WRONG cache.
#[test]
fn tier3_style_cache_certification() {
    // Intersection of cylinder (axis z, r = 2, origin (1,1,0)) with
    // plane z = 3: the circle center (1,1,3), radius 2, axis z.
    let good = Curve3::Circle {
        center: Point3::new(1.0, 1.0, 3.0),
        axis: Vec3::unit_z(),
        radius: 2.0,
        u_ref: Vec3::unit_x(),
    };
    let cyl_residual = |p: Point3<f64>| {
        let dx = p.x - 1.0;
        let dy = p.y - 1.0;
        (dx * dx + dy * dy) - 4.0
    };
    let plane_residual = |p: Point3<f64>| p.z - 3.0;

    let samples: Vec<f64> = (0..64).map(|i| f64::from(i) * TAU / 64.0).collect();
    let max_res = samples
        .iter()
        .map(|&t| {
            let p = good.eval(t);
            cyl_residual(p).abs().max(plane_residual(p).abs())
        })
        .fold(0.0f64, f64::max);
    assert!(max_res <= 1e-14, "good cache certifies: {max_res:e}");

    // A wrong cache (center nudged 1e-6) must FAIL the ε = 1e-9 gate.
    let bad = Curve3::Circle {
        center: Point3::new(1.0 + 1e-6, 1.0, 3.0),
        axis: Vec3::unit_z(),
        radius: 2.0,
        u_ref: Vec3::unit_x(),
    };
    let max_bad = samples
        .iter()
        .map(|&t| {
            let p = bad.eval(t);
            cyl_residual(p).abs().max(plane_residual(p).abs())
        })
        .fold(0.0f64, f64::max);
    assert!(max_bad > 1e-9, "bad cache must fail: {max_bad:e}");
}

// ---------------------------------------------------------------------
// 1. floor / reduce_periodic falsification (f64 + Dual lanes)
// ---------------------------------------------------------------------

/// f64 floor at exact integers and their ulp-neighbors, with the Dual
/// tangent's plateau convention at each.
#[test]
fn floor_at_integers_and_ulp_neighbors() {
    for k in [-3.0f64, -1.0, 0.0, 1.0, 3.0, 1e15] {
        assert_eq!(<f64 as Real>::floor(k), k);
        let up = next_up(k);
        let down = next_down(k);
        assert_eq!(<f64 as Real>::floor(up), k, "floor just above {k}");
        if k.abs() < 1e15 {
            assert_eq!(<f64 as Real>::floor(down), k - 1.0, "floor just below {k}");
        }
        // Dual tangent: 0 at ALL of these (plateau convention).
        for x in [k, up, down] {
            let d = Dual::new(x, 7.0).floor();
            assert_eq!(d.deriv, 0.0, "tangent at {x}");
        }
    }
}

fn next_up(x: f64) -> f64 {
    f64::next_up(x)
}
fn next_down(x: f64) -> f64 {
    f64::next_down(x)
}

/// The documented seam blur is REAL and unclamped: exhibit concrete f64
/// inputs whose reduction lands outside [0, period) — the docs promise
/// no clamping hides them.
#[test]
fn reduce_periodic_seam_blur_is_unclamped() {
    let mut found_negative = false;
    let mut found_at_or_above = false;
    // Scan inputs a hair below/above multiples of an irrational-ish
    // period: x = k·p ± few ulps.
    let p = TAU;
    for k in 1..20_000u32 {
        let m = f64::from(k) * p;
        for x in [next_down(m), next_down(next_down(m)), m, next_up(m)] {
            let r = <f64 as Real>::reduce_periodic(x, p);
            if r < 0.0 {
                found_negative = true;
            }
            if r >= p {
                found_at_or_above = true;
            }
            // Blur bound: a few ulps of the intermediate magnitude.
            assert!(r > -1e-10 && r < p + 1e-10, "x={x}, r={r}");
        }
    }
    assert!(
        found_negative,
        "expected some slightly-negative reductions (seam blur) — if none, \
         the docs overstate the blur or something is clamping"
    );
    // r == p exactly happens when x/p rounds DOWN across the seam; it is
    // allowed by the honest statement (result 'a hair above' includes p).
    // Not required to occur, so no assert — but record the observation.
    let _ = found_at_or_above;
}

/// reduce_periodic at non-2π periods, negative inputs, huge magnitudes:
/// total, value-correct where conditioning allows, honest garbage where
/// it does not (|x| ≳ 2^53·p) — never a panic, never poison for finite
/// nonzero period.
#[test]
fn reduce_periodic_across_magnitudes_and_periods() {
    // Period 0.7 (not exactly representable), negative θ.
    let r = <f64 as Real>::reduce_periodic(-1.5, 0.7);
    // True: −1.5 + 3·0.7 = 0.6.
    assert!((r - 0.6).abs() <= 1e-15, "r = {r}");
    // Negative θ many periods out.
    let r = <f64 as Real>::reduce_periodic(-1234.567, TAU);
    assert!(r >= -1e-12 && r < TAU + 1e-12);
    // The k-scaled blur: |x| = 1e12 keeps ~4 digits of the reduced value.
    let r = <f64 as Real>::reduce_periodic(1e12, TAU);
    assert!(r >= -2e-4 && r < TAU + 2e-4, "r = {r}");
    // Above 2^53·p the quotient exceeds integer resolution: the result
    // is honest garbage but still finite and total.
    let r = <f64 as Real>::reduce_periodic(1e18, TAU);
    assert!(r.is_finite());
    // Negative period: documented as (period, 0]-ish, not intended use.
    let r = <f64 as Real>::reduce_periodic(1.5, -2.0);
    assert!(r <= 0.0 + 1e-15 && r > -2.0 - 1e-15, "r = {r}");
}

/// Dual value-channel bit-identity for reduce_periodic (the
/// compositional-body claim), across sign/magnitude sweeps.
#[test]
fn reduce_periodic_dual_value_bit_identity() {
    for x in [-1e9, -1234.567, -1.5, 0.0, 0.3, 7.7, 1e6, 1e15] {
        for p in [0.7, 1.0, TAU, 12.25] {
            let plain = <f64 as Real>::reduce_periodic(x, p);
            let dual = Real::reduce_periodic(Dual::variable(x), Dual::constant(p));
            assert_eq!(plain.to_bits(), dual.value.to_bits(), "x={x}, p={p}");
        }
    }
}

/// Purity: same bits in → same bits out, run-to-run, for the new ops.
#[test]
fn purity_same_bits_in_same_bits_out() {
    for x in [0.1, -0.0, 3.5, 1e300, -7.25] {
        for s in [-2.0, -0.0, 0.0, 5.0] {
            let a = <f64 as Real>::copysign(x, s);
            let b = <f64 as Real>::copysign(x, s);
            assert_eq!(a.to_bits(), b.to_bits());
            let f1 = <f64 as Real>::floor(x);
            let f2 = <f64 as Real>::floor(x);
            assert_eq!(f1.to_bits(), f2.to_bits());
        }
    }
}

// ---------------------------------------------------------------------
// 2. copysign semantics at f64 / Dual
// ---------------------------------------------------------------------

/// The Dual s-tangent discard where it "matters": differentiating
/// through the sign argument at the jump gives the branch-consistent 0,
/// exactly like min's tie convention — constructed and pinned.
#[test]
fn copysign_sign_tangent_discard_at_the_jump() {
    // f(t) = copysign(2.0, t) at t = 0, dt = 1: the true map jumps
    // (−2 → +2) at t = 0; the dual answer is the plateau derivative 0
    // (value channel picks σ(+0) = +1, i.e. f = +2, and d/dt = 0).
    let f = Real::copysign(Dual::constant(2.0), Dual::variable(0.0));
    assert_eq!(f.value, 2.0);
    assert_eq!(f.deriv, 0.0);
    // At t = −0.0 the OTHER branch is chosen — value −2, deriv still 0:
    // branch-consistency, the derivative of the program as evaluated.
    let f = Real::copysign(Dual::constant(2.0), Dual::variable(-0.0));
    assert_eq!(f.value, -2.0);
    assert_eq!(f.deriv, 0.0);
    // Chain-rule composition where x also depends on t:
    // g(t) = copysign(t², t−1) at t = 0.5: |x|-side derivative flows,
    // σ = −1, d/dt = −2t = −1.
    let t = Dual::variable(0.5);
    let g = Real::copysign(t * t, t - Dual::constant(1.0));
    assert_eq!(g.value, -0.25);
    assert!((g.deriv - -1.0).abs() <= 1e-15);
}

// ---------------------------------------------------------------------
// 3. The Duff basis: adversarial directions
// ---------------------------------------------------------------------

/// Ulp-straddles of the equator and the signed-zero seam: the flip
/// happens EXACTLY at the sign bit of n.z, and both sides are
/// orthonormal right-handed frames.
#[test]
fn basis_equator_seam_is_exactly_the_sign_bit() {
    let base = Vec3::new(0.6, 0.8, 0.0);
    // +0.0 vs −0.0: the seam splits the two zeros.
    let (p1, p2) = Vec3::new(base.x, base.y, 0.0f64).orthonormal_basis();
    let (m1, m2) = Vec3::new(base.x, base.y, -0.0f64).orthonormal_basis();
    assert!((p1.z - -0.6).abs() <= 1e-15, "b1.z above seam: {}", p1.z);
    assert!((m1.z - 0.6).abs() <= 1e-15, "b1.z below seam: {}", m1.z);
    // Smallest subnormal straddle: same flip, still orthonormal.
    let tiny = 5e-324;
    for nz in [tiny, -tiny] {
        let n = Vec3::new(0.6, 0.8, nz); // ||n|| = 1 + O(ulp): unit to f64
        let (b1, b2) = n.orthonormal_basis();
        assert!((b1.norm() - 1.0).abs() <= 1e-14, "nz={nz}");
        assert!((b2.norm() - 1.0).abs() <= 1e-14, "nz={nz}");
        assert!(b1.dot(b2).abs() <= 1e-14, "nz={nz}");
        assert!(b1.dot(n).abs() <= 1e-14, "nz={nz}");
        assert!(b2.dot(n).abs() <= 1e-14, "nz={nz}");
        let c = b1.cross(b2);
        assert!(close(c, n, 1e-14), "right-handed at nz={nz}");
    }
    // All four frames agree in handedness with their own normal.
    for (b1, b2, nz) in [(p1, p2, 0.0f64), (m1, m2, -0.0f64)] {
        let n = Vec3::new(0.6, 0.8, nz);
        assert!(close(b1.cross(b2), n, 1e-14));
    }
}

/// Non-unit input to orthonormal_basis: documented garbage-out (no
/// poison, no panic, NOT orthonormal) — pin the documented posture.
#[test]
fn basis_non_unit_garbage_out() {
    let n = Vec3::new(0.0, 0.0, 2.0);
    let (b1, b2) = n.orthonormal_basis();
    assert!(!b1.x.is_nan() && !b2.y.is_nan(), "well-defined");
    // And it is genuinely garbage (not secretly normalized):
    let c = b1.cross(b2);
    assert!(
        (c.z - n.z).abs() > 0.5,
        "cross = {c:?} ≠ n — garbage as documented"
    );
}

// ---------------------------------------------------------------------
// project/reject association + rotation_about_axis consumer checks
// ---------------------------------------------------------------------

/// Independent check of the documented association: project_onto equals
/// the hand-written onto·(v·n / n·n) EXACTLY (bitwise), since the doc
/// pins that association — a consumer that reads the doc can rely on it.
#[test]
fn project_association_is_the_documented_one() {
    let v = Vec3::new(1.3, -2.7, 0.9);
    let n = Vec3::new(0.4, 2.2, -1.1);
    let p = v.project_onto(n);
    let coeff = v.dot(n) / n.norm_squared();
    let hand = n * coeff;
    assert_eq!(p.x.to_bits(), hand.x.to_bits());
    assert_eq!(p.y.to_bits(), hand.y.to_bits());
    assert_eq!(p.z.to_bits(), hand.z.to_bits());
    let r = v.reject_from(n);
    let hand_r = v - p;
    assert_eq!(r.x.to_bits(), hand_r.x.to_bits());
}

/// Revolve-style consumer: sweep a profile point about an off-origin
/// axis and check the swept circle against a Curve3::Circle carrier
/// built from project/reject — the M2 PR 5 composition in miniature.
#[test]
fn revolve_composition_dry_run() {
    let axis_point = Point3::new(1.0, 2.0, -0.5);
    let axis_dir = tilted_axis();
    let profile_pt = Point3::new(3.0, 0.5, 1.0);

    // The swept circle: center = axis foot of the profile point.
    let d = profile_pt - axis_point;
    let center = axis_point + d.project_onto(axis_dir);
    let radial = d.reject_from(axis_dir);
    let radius = radial.norm();
    let u_ref = radial * (1.0 / radius);
    let carrier = Curve3::Circle {
        center,
        axis: axis_dir,
        radius,
        u_ref,
    };
    // Rotating the profile point by θ about the axis lands ON the
    // carrier at parameter θ, for many θ.
    for theta in [0.0, 0.3, FRAC_PI_2, 2.0, PI, 4.5, -1.2] {
        let rot = Affine3::rotation_about_axis(axis_point, axis_dir, theta);
        let swept = rot.transform_point(profile_pt);
        let on_carrier = carrier.eval(theta);
        assert!(
            (swept.x - on_carrier.x).abs() <= 1e-13
                && (swept.y - on_carrier.y).abs() <= 1e-13
                && (swept.z - on_carrier.z).abs() <= 1e-13,
            "θ = {theta}: revolve and carrier disagree"
        );
    }
}
