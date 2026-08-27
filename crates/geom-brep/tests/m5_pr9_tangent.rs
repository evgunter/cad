//! M5 PR 9 (C7): `TangentIntersection` — the authored tangent pair
//! classifies and certifies with the full jet schedule, the
//! second-order margin's two-tolerance pair is pinned on both sides
//! (G2-zero definite vs in-band escalation), corruption rows scale
//! from the resolved band, and the `tangent_*` predicate family is
//! visible in the K funnel's verdict log from birth.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::SurfaceKey;
use geom_brep::{CertifyError, EdgeCurve, EdgeCurveSpec, EdgeGeometry, PlaneCylinderSection};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The authored tangent pair: the unit cylinder about z and the plane
/// x = 1, tangent along the ruling {(1, 0, z)}.
fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn tangent_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(1.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        u_ref: Vec3::new(0.0, 0.0, 1.0),
    }
}

/// An arena of surfaces and its two keys (the established test
/// idiom: a typed slotmap plus a cloning resolver).
fn arena2(
    s1: Surface<f64>,
    s2: Surface<f64>,
) -> (
    SurfaceKey,
    SurfaceKey,
    slotmap::SlotMap<SurfaceKey, Surface<f64>>,
) {
    let mut map: slotmap::SlotMap<SurfaceKey, Surface<f64>> = slotmap::SlotMap::with_key();
    let k1 = map.insert(s1);
    let k2 = map.insert(s2);
    (k1, k2, map)
}

fn keys2() -> (
    SurfaceKey,
    SurfaceKey,
    slotmap::SlotMap<SurfaceKey, Surface<f64>>,
) {
    arena2(cylinder(), tangent_plane())
}

/// The ruling-line edge spec over `z ∈ [0, 1]`, witness at carrier
/// mid (the S2 pin).
fn ruling_spec(k1: SurfaceKey, k2: SurfaceKey) -> EdgeCurveSpec<f64> {
    let carrier = Curve3::Line {
        origin: Point3::new(1.0, 0.0, 0.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    };
    EdgeCurveSpec {
        description: EdgeGeometry::TangentIntersection {
            s1: k1,
            s2: k2,
            witness: carrier.eval(0.5),
        },
        carrier,
        param_start: 0.0,
        param_end: 1.0,
    }
}

#[test]
fn the_c5_table_classifies_the_authored_pair_as_a_tangent_line() {
    // Construction is BY CLASSIFICATION, never by marching: the
    // table's tangent arm names the locus and hands back the exact
    // line carrier the TangentIntersection edge stores.
    let out = geom_brep::plane_cylinder_section(&tangent_plane(), &cylinder(), 1.0, band())
        .expect("a clean tangency classifies definitely");
    let PlaneCylinderSection::TangentLine(line) = out else {
        panic!("the authored pair is the tangent configuration: {out:?}");
    };
    let Curve3::Line { origin, dir } = line else {
        panic!("a tangent locus of plane×cylinder is a line");
    };
    assert!((origin.x - 1.0).abs() < 1e-12 && origin.y.abs() < 1e-12);
    assert!(dir.cross(Vec3::new(0.0, 0.0, 1.0)).norm() < 1e-12);
}

#[test]
fn the_authored_tangent_pair_certifies_with_the_full_jet_schedule() {
    let (k1, k2, map) = keys2();
    let spec = ruling_spec(k1, k2);
    let (p0, p1) = (spec.carrier.eval(0.0), spec.carrier.eval(1.0));
    let curve = EdgeCurve::certify(spec, p0, p1, |k| map.get(k).cloned(), band())
        .expect("the kernel's first certified TangentIntersection");
    assert!(matches!(
        curve.description(),
        EdgeGeometry::TangentIntersection { .. }
    ));
    // The certificate is byte-honest: zero residual on an exact
    // ruling (every sample satisfies both implicit forms exactly).
    assert_eq!(curve.certificate().max_residual, 0.0);
}

#[test]
fn the_tangent_predicates_reach_the_k_funnel() {
    // Telemetry from birth (C7/C12.2): the family the PR 14
    // K-snapshot will read, visible by name in the verdict log.
    use geom_core::k_stats::{start_verdict_log, take_verdict_log};
    let (k1, k2, map) = keys2();
    let spec = ruling_spec(k1, k2);
    let (p0, p1) = (spec.carrier.eval(0.0), spec.carrier.eval(1.0));
    start_verdict_log();
    let _ = EdgeCurve::certify(spec, p0, p1, |k| map.get(k).cloned(), band()).unwrap();
    let v = take_verdict_log();
    for name in [
        "tangent_on_surface_1",
        "tangent_on_surface_2",
        "tangent_second_order",
        "tangent_normal_parallel",
        "tangent_hull_sup",
        "tangent_tube_margin",
    ] {
        assert!(
            v.iter().any(|x| x.predicate == name),
            "{name} never reached the funnel (recorded: {:?})",
            v.iter()
                .map(|x| x.predicate)
                .collect::<std::collections::BTreeSet<_>>()
        );
    }
}

#[test]
fn a_g2_flat_pair_refuses_second_order_definitely() {
    // The zero-side of the OQ7 discriminator, pinned as a DEFINITE
    // typed refusal: two parallel planes "tangent" everywhere have
    // κ_rel exactly zero — the surfaces under-determine the locus,
    // which is exactly why a G2 conventional join keeps MappedCurve.
    let (k1, k2, map) = arena2(
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            u_ref: Vec3::new(0.0, 0.0, 1.0),
        },
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        },
    );
    let carrier = Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    };
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::TangentIntersection {
            s1: k1,
            s2: k2,
            witness: carrier.eval(0.5),
        },
        carrier,
        param_start: 0.0,
        param_end: 1.0,
    };
    let (p0, p1) = (spec.carrier.eval(0.0), spec.carrier.eval(1.0));
    let err = EdgeCurve::certify(spec, p0, p1, |k| map.get(k).cloned(), band()).unwrap_err();
    let CertifyError::NotSecondOrderSeparated { band: b, .. } = err else {
        panic!("the zero-side margin is the definite refusal: {err}");
    };
    // The two-tolerance shape: the definite arm quotes the band that
    // decided and carries the shared recourse.
    let msg = format!("{err}");
    assert!(msg.contains("under-determine"), "{msg}");
    assert!(msg.contains(&format!("{:e}", b.zero())), "{msg}");
    assert!(msg.contains("declare the coincidence"), "{msg}");
}

#[test]
fn an_in_band_second_order_margin_escalates_f6() {
    // One band-width away from the zero-side: a cylinder so flat its
    // second-order displacement sits INSIDE the resolved band — an
    // osculating pair at this ε is a sliver (F6), escalated, never
    // classified. Placement scales from the resolved band.
    let b = band();
    let inside = 0.5 * (b.zero() + b.escalate());
    // margin = κ_rel·r_fold²/2 with r_fold = min(R, extent = 1) = 1
    // for the huge-R cylinder, so κ_rel = 1/R = 2·inside ⇒ margin =
    // exactly `inside`.
    let radius = 1.0 / (2.0 * inside);
    let (k1, k2, map) = arena2(
        Surface::Cylinder {
            origin: Point3::new(1.0 - radius, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        tangent_plane(),
    );
    // The plane x = 1 is tangent to this near-flat wall along the
    // same ruling {(1, 0, z)} — the cylinder bulges toward −x.
    let carrier = Curve3::Line {
        origin: Point3::new(1.0, 0.0, 0.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    };
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::TangentIntersection {
            s1: k1,
            s2: k2,
            witness: carrier.eval(0.5),
        },
        carrier,
        param_start: 0.0,
        param_end: 1.0,
    };
    let (p0, p1) = (spec.carrier.eval(0.0), spec.carrier.eval(1.0));
    let err = EdgeCurve::certify(spec, p0, p1, |k| map.get(k).cloned(), band()).unwrap_err();
    let CertifyError::Escalated { check, cause, .. } = err else {
        panic!("in-band second order escalates (F6): {err}");
    };
    assert_eq!(check, geom_brep::CertCheck::TangentSecondOrder);
    assert_eq!(cause.predicate, Some("tangent_second_order"));
}

#[test]
fn a_skewed_carrier_fails_normal_parallelism() {
    // Corruption row: tilt the line so it leaves the ruling — the
    // gradients' angle grows along the edge and the parallelism
    // check (lever arm 1/κ_rel) must catch it definitely. The tilt
    // is placed WELL outside the band at every ε (a macroscopic
    // 0.05 tilt over a 1 m edge: sinθ ≈ y/1 up to 0.05, margin ≈
    // 0.05 m ≫ escalate at any battery ε).
    let (k1, k2, map) = keys2();
    let carrier = Curve3::Line {
        origin: Point3::new(1.0, 0.0, 0.0),
        dir: Vec3::new(0.0, 0.05, 1.0),
    };
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::TangentIntersection {
            s1: k1,
            s2: k2,
            witness: carrier.eval(0.5),
        },
        carrier,
        param_start: 0.0,
        param_end: 1.0,
    };
    let (p0, p1) = (spec.carrier.eval(0.0), spec.carrier.eval(1.0));
    let err = EdgeCurve::certify(spec, p0, p1, |k| map.get(k).cloned(), band()).unwrap_err();
    // The tilted line leaves the CYLINDER surface too, so the first
    // failing check is a typed residual/parallelism refusal — either
    // way a definite refusal naming its check, never acceptance.
    match err {
        CertifyError::ResidualExceeded { .. } => {}
        CertifyError::Escalated { .. } => panic!("macroscopic corruption must refuse definitely"),
        other => panic!("unexpected: {other}"),
    }
}

#[test]
fn an_off_surface_carrier_fails_the_residual_schedule_at_band_scale() {
    // The corruption magnitude scales from the resolved band:
    // definitely outside at every ε (escalate·100), so the row is
    // honest at 1e-6 and 1e-12 alike (the #146 lesson).
    let b = band();
    let off = b.escalate() * 100.0;
    let (k1, k2, map) = keys2();
    let carrier = Curve3::Line {
        origin: Point3::new(1.0 + off, 0.0, 0.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    };
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::TangentIntersection {
            s1: k1,
            s2: k2,
            witness: carrier.eval(0.5),
        },
        carrier,
        param_start: 0.0,
        param_end: 1.0,
    };
    let (p0, p1) = (spec.carrier.eval(0.0), spec.carrier.eval(1.0));
    let err = EdgeCurve::certify(spec, p0, p1, |k| map.get(k).cloned(), band()).unwrap_err();
    assert!(
        matches!(err, CertifyError::ResidualExceeded { .. }),
        "off-surface by 100·escalate refuses definitely: {err}"
    );
}

#[test]
fn the_coaxial_circle_class_was_retired_into_the_lane_at_pr_12() {
    // HISTORY: at PR 9 this exact configuration — a torus tangent to
    // a plane along its bottom equator, carried on a CIRCLE — was the
    // out-of-lane row, because the line arm's span bounds had no
    // torus entry and no non-line carrier. M5 PR 12 retired the class
    // WITH ITS PROOF (C12.1's per-class retirement): `κ_rel` and the
    // implicit residual are isometry invariants, the coaxial circle's
    // motion is a symmetry flow of both surfaces, so both span bounds
    // are EXACTLY zero. The row is kept, flipped, as the record that
    // the boundary moved for a reason.
    let (k1, k2, map) = arena2(
        Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: 1.0,
            minor_radius: 0.5,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, -0.5),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
    );
    let carrier = Curve3::Circle {
        center: Point3::new(0.0, 0.0, -0.5),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::TangentIntersection {
            s1: k1,
            s2: k2,
            witness: carrier.eval(0.75),
        },
        carrier,
        param_start: 0.0,
        param_end: 1.5,
    };
    let (p0, p1) = (spec.carrier.eval(0.0), spec.carrier.eval(1.5));
    let curve = EdgeCurve::certify(spec, p0, p1, |k| map.get(k).cloned(), band())
        .expect("the coaxial circle arm certifies this class since M5 PR 12");
    assert!(curve.certificate().max_residual < 1e-12);
}

#[test]
fn a_coaxial_cone_sphere_contact_circle_certifies() {
    // The inscribed sphere touches the cone along a COAXIAL circle —
    // the class a sphere-cone fillet band's contact circles belong to,
    // which cannot be described at rest without the circle arm's cone
    // row. Every one of that row's bounds is exactly zero on a coaxial
    // carrier.
    let (k1, k2, map) = arena2(
        Surface::Cone {
            apex: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::new(0.0, 0.0, -1.0),
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: core::f64::consts::FRAC_1_SQRT_2,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
    );
    let carrier = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.5),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::TangentIntersection {
            s1: k1,
            s2: k2,
            witness: carrier.eval(0.75),
        },
        carrier,
        param_start: 0.0,
        param_end: 1.5,
    };
    let (p0, p1) = (spec.carrier.eval(0.0), spec.carrier.eval(1.5));
    let curve = EdgeCurve::certify(spec, p0, p1, |k| map.get(k).cloned(), band())
        .expect("a coaxial cone-sphere contact circle is inside the circle arm");
    assert!(curve.certificate().max_residual < 1e-12);
}

#[test]
fn outside_the_span_bound_lane_refuses_typed() {
    // The lane boundary: the LINE arm carries no cone row (a cone
    // tangency along a generator is not a configuration this kernel
    // constructs), so a tangency carried on one is a routing refusal —
    // typed, named, no fallback.
    let s2 = core::f64::consts::FRAC_1_SQRT_2;
    let (k1, k2, map) = arena2(
        Surface::Cone {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        // The tangent plane along the generator through (1, 0, 1): it
        // contains the apex and that whole ruling, so the contact locus
        // is the ruling itself.
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(s2, 0.0, -s2),
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        },
    );
    let carrier = Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(s2, 0.0, s2),
    };
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::TangentIntersection {
            s1: k1,
            s2: k2,
            witness: carrier.eval(1.0),
        },
        carrier,
        param_start: 0.5,
        param_end: 1.5,
    };
    let (p0, p1) = (spec.carrier.eval(0.5), spec.carrier.eval(1.5));
    let err = EdgeCurve::certify(spec, p0, p1, |k| map.get(k).cloned(), band()).unwrap_err();
    assert!(
        matches!(err, CertifyError::TangentCertificateUnsupported),
        "outside the lane is a routing refusal: {err}"
    );
    let msg = format!("{err}");
    assert!(msg.contains("span-bound lane"), "{msg}");
}
