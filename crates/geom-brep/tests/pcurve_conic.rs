//! M5 PR 5 §5/§6: conic pcurve constructors (plane chart exact,
//! cylinder chart fitted), the ellipse-vs-rational-quadratic-NURBS
//! differential (the §7.3 shape-factor form), and the no-wildcard
//! table discipline row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom::{Curve3, NurbsCurve3};
use geom_brep::intersect::{PlaneCylinderSection, plane_cylinder_section};
use geom_brep::{ellipse_pcurve_on_cylinder, ellipse_pcurve_on_plane, implicit_residual};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Vec3};
use geom_core::Tol;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn cyl() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(1.0, 2.0, 3.0),
        axis: Vec3::unit_z(),
        radius: 2.0,
        u_ref: Vec3::unit_x(),
    }
}

fn plane(phi: f64) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 7.0),
        normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
        u_ref: Vec3::new(phi.cos(), 0.0, -phi.sin()),
    }
}

fn section_ellipse(phi: f64) -> Curve3<f64> {
    match plane_cylinder_section(&plane(phi), &cyl(), 1.0, band()).unwrap() {
        PlaneCylinderSection::TiltedEllipse(e) => e,
        other => panic!("expected the tilted ellipse, got {other:?}"),
    }
}

/// Maps a plane-chart point to 3-D.
fn chart_to_3d(p: &Surface<f64>, u: f64, v: f64) -> Point3<f64> {
    let Surface::Plane {
        origin,
        normal,
        u_ref,
    } = p
    else {
        panic!("plane chart");
    };
    let pv = normal.cross(*u_ref);
    *origin + *u_ref * u + pv * v
}

// ---------------------------------------------------------------------
// §5: the plane-chart pcurve is locus-exact
// ---------------------------------------------------------------------

#[test]
fn plane_chart_pcurve_is_locus_exact() {
    let phi = 0.5f64;
    let e = section_ellipse(phi);
    let pl = plane(phi);
    for (t0, t1) in [
        (0.0, 1.2),
        (0.3, 2.9),
        (-1.0, 4.9),
        (0.0, core::f64::consts::TAU),
    ] {
        let pc = ellipse_pcurve_on_plane(&e, &pl, t0, t1).unwrap();
        // Dense samples of the pcurve map into points on BOTH surfaces
        // (in meters through the map — the §5 claim, sampled form).
        for i in 0..=64u32 {
            let s = f64::from(i) / 64.0;
            let q2 = pc.eval(s);
            let q3 = chart_to_3d(&pl, q2.x, q2.y);
            assert!(
                implicit_residual(&pl, q3).abs() < 1e-12,
                "plane residual at s = {s}"
            );
            assert!(
                implicit_residual(&cyl(), q3).abs() < 1e-12,
                "cylinder residual at s = {s}: {:e}",
                implicit_residual(&cyl(), q3)
            );
        }
        // Endpoint correspondence: pcurve(0)/pcurve(1) are the carrier
        // arc's endpoints in the chart.
        for (s, theta) in [(0.0, t0), (1.0, t1)] {
            let q2 = pc.eval(s);
            let q3 = chart_to_3d(&pl, q2.x, q2.y);
            assert!(q3.distance(e.eval(theta)) < 1e-12, "endpoint at {theta}");
        }
        // The §7.3 shape factor: every rational-quadratic segment has
        // w₀·w₂/w₁² = 1/cos²(δ/2) > 1 — the elliptic class.
        let w = pc.weights();
        for seg in 0..(w.len() - 1) / 2 {
            let k = (w[2 * seg] * w[2 * seg + 2]) / (w[2 * seg + 1] * w[2 * seg + 1]);
            assert!(k > 1.0, "shape factor {k} must be elliptic-class");
        }
    }
}

/// The per-segment symmetry pins the θ correspondence at NURBS
/// parameter 0, ½, 1 of each segment (arc midpoints map to sweep
/// midpoints for symmetric rational quadratics).
#[test]
fn plane_chart_pcurve_segment_midpoints_hit_sweep_midpoints() {
    let phi = 0.4f64;
    let e = section_ellipse(phi);
    let pl = plane(phi);
    let (t0, t1) = (0.2, 2.6);
    let pc = ellipse_pcurve_on_plane(&e, &pl, t0, t1).unwrap();
    // Two segments (span 2.4 > π/2): segment boundaries at s = 0, ½, 1;
    // segment midpoints at s = ¼, ¾ correspond to sweep midpoints.
    let delta = (t1 - t0) / 2.0;
    for (s, theta) in [
        (0.0, t0),
        (0.25, t0 + delta * 0.5),
        (0.5, t0 + delta),
        (0.75, t0 + delta * 1.5),
        (1.0, t1),
    ] {
        let q2 = pc.eval(s);
        let q3 = chart_to_3d(&pl, q2.x, q2.y);
        assert!(
            q3.distance(e.eval(theta)) < 1e-12,
            "s = {s} must map to θ = {theta}"
        );
    }
}

#[test]
fn pcurve_wrong_lanes_and_degenerate_spans_are_typed() {
    let e = section_ellipse(0.5);
    let circle = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    assert!(ellipse_pcurve_on_plane(&circle, &plane(0.5), 0.0, 1.0).is_err());
    assert!(ellipse_pcurve_on_plane(&e, &cyl(), 0.0, 1.0).is_err());
    assert!(ellipse_pcurve_on_plane(&e, &plane(0.5), 1.0, 1.0).is_err());
    assert!(ellipse_pcurve_on_cylinder(&circle, &cyl(), 0.0, 1.0, 3, 1e-7).is_err());
    assert!(ellipse_pcurve_on_cylinder(&e, &plane(0.5), 0.0, 1.0, 3, 1e-7).is_err());
    assert!(ellipse_pcurve_on_cylinder(&e, &cyl(), 1.0, f64::NAN, 3, 1e-7).is_err());
    // n2: a span beyond one full period is a self-overlapping chain —
    // refused typed by BOTH constructors (the winding-gate bound).
    let err = ellipse_pcurve_on_plane(&e, &plane(0.5), 0.0, 6.5).unwrap_err();
    assert!(
        matches!(err, geom_brep::PcurveError::SpanExceedsPeriod),
        "{err:?}"
    );
    let err = ellipse_pcurve_on_cylinder(&e, &cyl(), 0.0, 6.5, 3, 1e-7).unwrap_err();
    assert!(
        matches!(err, geom_brep::PcurveError::SpanExceedsPeriod),
        "{err:?}"
    );
}

// ---------------------------------------------------------------------
// §5: the cylinder-chart pcurve is fitted (PR 4 machinery, C6 pinning)
// ---------------------------------------------------------------------

#[test]
fn cylinder_chart_pcurve_fits_and_replays_deterministically() {
    let phi = 0.5f64;
    let e = section_ellipse(phi);
    let c = cyl();
    let (t0, t1) = (0.2, 5.8);
    let fit = ellipse_pcurve_on_cylinder(&e, &c, t0, t1, 3, 1e-6).unwrap();
    assert!(fit.bound <= 1e-6, "achieved bound {:e}", fit.bound);
    // Mapped through the chart, the fitted pcurve stays near the true
    // locus: on the cylinder exactly (the chart lives on it), within
    // the fit + sampling budget of the plane.
    let Surface::Cylinder {
        origin,
        axis,
        radius,
        u_ref,
    } = c
    else {
        panic!("ellipse filter guarantees the variant");
    };
    let v_ref = axis.cross(u_ref);
    for i in 0..=64u32 {
        let s = f64::from(i) / 64.0;
        let q2 = fit.curve.eval(s);
        let (su, cu) = q2.x.sin_cos();
        let q3 = origin + (u_ref * cu + v_ref * su) * radius + axis * q2.y;
        assert!(implicit_residual(&c, q3).abs() < 1e-12, "on-chart");
        // Budget: the fit bound certifies deviation from the CHORD
        // INTERPOLANT (1e-6 here); the interpolant itself deviates from
        // the true graph by ≤ (Δu)²·|v″|/8 ≈ (5.6/128)²·(r·tan φ)/8
        // ≈ 2.6e-4 — the dominant term. 5e-4 covers both with margin.
        assert!(
            implicit_residual(&plane(phi), q3).abs() < 5e-4,
            "plane residual {:e} exceeds the fit + sampling budget",
            implicit_residual(&plane(phi), q3)
        );
    }
    // C6/D9: structure selection is deterministic — same inputs, same
    // knots, same bits.
    let again = ellipse_pcurve_on_cylinder(&e, &c, t0, t1, 3, 1e-6).unwrap();
    assert_eq!(
        format!("{:?}", fit.curve),
        format!("{:?}", again.curve),
        "bit-identical refit"
    );
}

// ---------------------------------------------------------------------
// §6: the ellipse evaluator differential vs the exact
// rational-quadratic NURBS form (test-side oracle; the export form
// itself is PR 11/13's)
// ---------------------------------------------------------------------

/// Builds the exact rational-quadratic NURBS 3-D form of an ellipse
/// arc (Book §7.3, ≤ 90° segments) — the TEST oracle only.
fn rational_quadratic_arc(e: &Curve3<f64>, t0: f64, t1: f64) -> NurbsCurve3<f64> {
    let Curve3::Ellipse {
        center,
        axis,
        major,
        minor,
        u_ref,
    } = *e
    else {
        panic!("oracle needs an ellipse");
    };
    let v_ref = axis.cross(u_ref);
    let at = |theta: f64| -> Point3<f64> {
        let (s, c) = theta.sin_cos();
        center + (u_ref * (major * c) + v_ref * (minor * s))
    };
    let shoulder = |alpha: f64, delta: f64| -> Point3<f64> {
        let m = alpha + delta * 0.5;
        let (s, c) = m.sin_cos();
        let k = 1.0 / (delta * 0.5).cos();
        center + (u_ref * (major * c) + v_ref * (minor * s)) * k
    };
    let span = t1 - t0;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let segments = {
        let raw = (span / core::f64::consts::FRAC_PI_2).ceil();
        if raw < 1.0 { 1 } else { raw as usize }
    };
    #[allow(clippy::cast_precision_loss)]
    let delta = span / segments as f64;
    let w_mid = (delta * 0.5).cos();
    let mut control = vec![at(t0)];
    let mut weights = vec![1.0];
    let mut knots = vec![0.0, 0.0, 0.0];
    for i in 0..segments {
        #[allow(clippy::cast_precision_loss)]
        let alpha = t0 + delta * i as f64;
        control.push(shoulder(alpha, delta));
        weights.push(w_mid);
        control.push(at(alpha + delta));
        weights.push(1.0);
        #[allow(clippy::cast_precision_loss)]
        let s_end = (i + 1) as f64 / segments as f64;
        if i + 1 < segments {
            knots.extend_from_slice(&[s_end, s_end]);
        }
    }
    knots.extend_from_slice(&[1.0, 1.0, 1.0]);
    NurbsCurve3::new(KnotVector::clamped(knots, 2).unwrap(), control, weights).unwrap()
}

/// Fuzzed configurations: the NURBS oracle's points satisfy the
/// ellipse's frame quadratic to rounding, and the known-parameter
/// correspondences agree pointwise — the evaluator differential.
#[test]
fn ellipse_evaluator_differential_vs_rational_quadratic() {
    // Deterministic fuzz over frames/axes/spans (no RNG in kernel
    // tests without proptest; a fixed lattice fuzz is replayable).
    let frames = [
        (Vec3::unit_z(), Vec3::unit_x()),
        (
            Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0),
            Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0),
        ),
    ];
    let axes = [(2.0, 0.5), (1.5, 1.49), (10.0, 0.1)];
    let spans = [(0.0, 1.0), (0.7, 3.4), (-2.0, 4.0)];
    let b = geom_core::Band::new(1e-9, 1e-8).unwrap();
    for &(axis, u_ref) in &frames {
        for &(major, minor) in &axes {
            for &(t0, t1) in &spans {
                let e = Curve3::ellipse(Point3::new(-0.5, 4.0, 1.25), axis, major, minor, u_ref, b)
                    .unwrap();
                let oracle = rational_quadratic_arc(&e, t0, t1);
                let v_ref = axis.cross(u_ref);
                let center = Point3::new(-0.5, 4.0, 1.25);
                // Locus agreement at fuzzed parameters: every oracle
                // point satisfies the frame quadratic to rounding.
                for i in 0..=32u32 {
                    let s = f64::from(i) / 32.0;
                    let p = oracle.eval(s);
                    let d = p - center;
                    let (x, y) = (d.dot(u_ref), d.dot(v_ref));
                    let quad = (x / major).powi(2) + (y / minor).powi(2) - 1.0;
                    assert!(quad.abs() < 1e-11, "quadratic residual {quad:e} at s = {s}");
                }
                // Pointwise agreement where the parameter maps are
                // pinned (segment ends + symmetric midpoints).
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let segments = {
                    let raw = ((t1 - t0) / core::f64::consts::FRAC_PI_2).ceil();
                    if raw < 1.0 { 1 } else { raw as usize }
                };
                #[allow(clippy::cast_precision_loss)]
                let delta = (t1 - t0) / segments as f64;
                for seg in 0..segments {
                    #[allow(clippy::cast_precision_loss)]
                    let s0 = seg as f64 / segments as f64;
                    #[allow(clippy::cast_precision_loss)]
                    let half = 0.5 / segments as f64;
                    #[allow(clippy::cast_precision_loss)]
                    let alpha = t0 + delta * seg as f64;
                    for (s, theta) in [
                        (s0, alpha),
                        (s0 + half, alpha + delta * 0.5),
                        (s0 + 2.0 * half, alpha + delta),
                    ] {
                        let p = oracle.eval(s);
                        let q = e.eval(theta);
                        assert!(
                            p.distance(q) < 1e-11,
                            "oracle({s}) vs carrier({theta}): {:e}",
                            p.distance(q)
                        );
                    }
                }
            }
        }
    }
}

/// The interval lane of the differential: the oracle's enclosures
/// contain points whose frame-quadratic residual encloses zero —
/// agreement to certified enclosure width (§6).
#[cfg(feature = "interval")]
#[test]
fn ellipse_differential_interval_lane() {
    use geom_core::{Bounds, Interval, Real};
    let iv = <Interval as Real>::from_f64;
    let e = Curve3::ellipse(
        Point3::new(-0.5, 4.0, 1.25),
        Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0),
        2.5,
        1.0,
        Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0),
        geom_core::Band::new(1e-9, 1e-8).unwrap(),
    )
    .unwrap();
    let oracle = rational_quadratic_arc(&e, 0.3, 2.9);
    // Lift the oracle to Interval (same structure, bracketed data).
    let ctrl: Vec<Point3<Interval>> = oracle
        .control()
        .iter()
        .map(|p| Point3::new(iv(p.x), iv(p.y), iv(p.z)))
        .collect();
    let oracle_iv =
        geom::NurbsCurve3::<Interval>::new(oracle.knots().clone(), ctrl, oracle.weights().to_vec())
            .unwrap();
    let center = Point3::new(iv(-0.5), iv(4.0), iv(1.25));
    let axis = Vec3::new(iv(2.0 / 3.0), iv(2.0 / 3.0), iv(1.0 / 3.0));
    let u_ref = Vec3::new(iv(1.0 / 3.0), iv(-2.0 / 3.0), iv(2.0 / 3.0));
    let v_ref = axis.cross(u_ref);
    for s in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let p = oracle_iv.eval(iv(s));
        let d = p - center;
        let x = d.dot(u_ref) / iv(2.5);
        let y = d.dot(v_ref) / iv(1.0);
        let quad = x * x + y * y - Interval::one();
        assert!(
            quad.lo() <= 1e-11 && -1e-11 <= quad.hi(),
            "quadratic enclosure [{}, {}] at s = {s}",
            quad.lo(),
            quad.hi()
        );
        assert!(quad.hi() - quad.lo() < 1e-10, "enclosure width at s = {s}");
    }
}

// ---------------------------------------------------------------------
// §6: the no-wildcard table discipline row (grep-able)
// ---------------------------------------------------------------------

/// THE table's match carries no wildcard arm — pinned against the
/// source text (the compile-break form of the discipline is a
/// doc-note on `route`, not a committed test).
#[test]
fn route_table_has_no_wildcard_arm() {
    let src = include_str!("../src/intersect.rs");
    let start = src.find("pub fn route(").expect("route fn present");
    let end = src[start..]
        .find("\n}\n")
        .map(|i| start + i)
        .expect("route fn closes");
    let table = &src[start..end];
    for forbidden in ["_ =>", "(_,", ", _)", "| _", "_ |"] {
        assert!(
            !table.contains(forbidden),
            "wildcard-shaped token {forbidden:?} found in THE table"
        );
    }
}
