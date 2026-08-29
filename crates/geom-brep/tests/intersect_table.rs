//! THE C5 dispatch table (M5 PR 5): routing inventory pins, the rung-2
//! closed forms' residuals (zero-by-construction, both lanes), and the
//! per-predicate trilean trios (definite / exactly-degenerate /
//! in-band-escalation — the F6/K pattern).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::implicit_residual;
use geom_brep::intersect::{
    EqualCylinderSection, PlaneConeSection, PlaneCylinderSection, RadiusEvidence, Rung,
    SectionError, SurfaceKind, cylinder_cylinder_section, plane_cone_section,
    plane_cylinder_section, route,
};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn eps() -> f64 {
    Tol::witness().get().eps
}

/// The general rung EXISTS (M5 PR 7). So an arm that still refuses owes
/// what it is MISSING — a trace shape, a certificate, a conversion —
/// and may not spend the refusal on "unimplemented" or on a schedule
/// naming SSI, which arrived.
///
/// The rule was written in a comment in this file and asserted only in
/// its positive half, so five notes broke it while this test iterated
/// over them. This is the negative half, and it is deliberately NOT
/// scoped to `Rung::General`: the wording it bans appeared on
/// **conic-rung** notes describing what routes onward, which a rung-3
/// guard would never have visited.
fn refusal_is_grounded(text: &str, what: &str) {
    for banned in ["unimplemented until", "until SSI", "not ready until"] {
        assert!(
            !text.contains(banned),
            "{what}: a refusal must name what is MISSING, not {banned:?} — \
             the general rung has existed since M5 PR 7: {text}"
        );
    }
}

// ---------------------------------------------------------------------
// The table: full arm inventory, pinned
// ---------------------------------------------------------------------

/// Every unordered pair's routing decision, pinned row by row — the
/// table is data, and this is its committed inventory. (Adding a
/// `SurfaceKind` breaks `route`'s build before it breaks this pin —
/// the compile-break discipline is a doc-note per spec §6, not a
/// committed test.)
#[test]
fn route_inventory() {
    use SurfaceKind::{Cone, Cylinder, Nurbs, Plane, Sphere, Torus};
    let rows: &[(SurfaceKind, SurfaceKind, Rung, bool)] = &[
        (Plane, Plane, Rung::Closed, true),
        (Plane, Cylinder, Rung::Conic, true),
        (Plane, Cone, Rung::Conic, true),
        // M5 S13 retired this arm: the closed-form Circle
        // (plane_sphere_section — the die-pips join lane's row).
        (Plane, Sphere, Rung::Closed, true),
        (Plane, Torus, Rung::General, false),
        // M5 PR 7b retired this arm: the ℝ⁴ parametric-pair march of
        // PR 7 plus the tensor-composite sup bound for limb 2.
        (Plane, Nurbs, Rung::General, true),
        (Cylinder, Cylinder, Rung::Conic, true),
        (Cylinder, Cone, Rung::General, false),
        // M5 PR 7 retired this arm: the ℝ³ implicit-pair march, all
        // three C2 limbs, in-op exhaustiveness.
        (Cylinder, Sphere, Rung::General, true),
        (Cylinder, Torus, Rung::General, false),
        (Cylinder, Nurbs, Rung::General, false),
        (Cone, Cone, Rung::General, false),
        (Cone, Sphere, Rung::General, false),
        (Cone, Torus, Rung::General, false),
        (Cone, Nurbs, Rung::General, false),
        // VERBS-SPHSPH retired this arm: the closed-form radical-plane
        // Circle (sphere_sphere_section — the same algebra profile's
        // arc/arc carrier gate runs in 2-D).
        (Sphere, Sphere, Rung::Closed, true),
        (Sphere, Torus, Rung::General, false),
        (Sphere, Nurbs, Rung::General, false),
        (Torus, Torus, Rung::General, false),
        (Torus, Nurbs, Rung::General, false),
        (Nurbs, Nurbs, Rung::General, false),
    ];
    assert_eq!(rows.len(), 21, "6 kinds ⇒ 21 unordered pairs");
    for &(a, b, rung, implemented) in rows {
        for (x, y) in [(a, b), (b, a)] {
            let r = route(x, y);
            assert_eq!(r.rung, rung, "{}×{}", x.name(), y.name());
            assert_eq!(r.implemented, implemented, "{}×{}", x.name(), y.name());
            // Every unimplemented arm's refusal names its routing.
            if !implemented {
                let msg = r.refusal(x, y);
                assert!(msg.contains("routes to"), "{msg}");
            }
            // The rule above, enforced for EVERY row rather than only
            // the rung-3 ones — see `refusal_is_grounded`.
            refusal_is_grounded(r.note, &format!("{}x{} note", x.name(), y.name()));
            // Every rung-3 arm names its TRACE SHAPE — a compile-time
            // decision per C5.
            if rung == Rung::General {
                assert!(
                    r.note.contains("IMPLICIT") || r.note.contains("PARAMETRIC"),
                    "{}×{}: {}",
                    x.name(),
                    y.name(),
                    r.note
                );
            }
        }
    }
    // Symmetry: the table is order-independent pair by pair.
    for &(a, b, ..) in rows {
        assert_eq!(route(a, b), route(b, a));
    }
}

// ---------------------------------------------------------------------
// plane × cylinder (§3.1)
// ---------------------------------------------------------------------

fn cyl_z(r: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(1.0, 2.0, 3.0),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    }
}

/// A plane whose normal makes angle `phi` with the cylinder axis (z),
/// through `q`.
fn tilted_plane(phi: f64, q: Point3<f64>) -> Surface<f64> {
    Surface::Plane {
        origin: q,
        normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
        u_ref: Vec3::new(phi.cos(), 0.0, -phi.sin()),
    }
}

#[test]
fn plane_cylinder_tilted_is_the_exact_ellipse() {
    let cyl = cyl_z(2.0);
    let phi = 0.5f64;
    let plane = tilted_plane(phi, Point3::new(0.0, 0.0, 7.0));
    let s = plane_cylinder_section(&plane, &cyl, 1.0, band()).unwrap();
    let PlaneCylinderSection::TiltedEllipse(e) = s else {
        panic!("expected the tilted ellipse, got {s:?}");
    };
    // Axes: a = r/cos φ, b = r.
    let Curve3::Ellipse {
        major,
        minor,
        center,
        axis,
        ..
    } = e
    else {
        panic!("carrier is an ellipse");
    };
    assert!((major - 2.0 / phi.cos()).abs() < 1e-12);
    assert_eq!(minor, 2.0);
    // The carrier's plane is the cutting plane; the center is on the
    // cylinder axis and on the plane.
    assert!((center.x - 1.0).abs() < 1e-12 && (center.y - 2.0).abs() < 1e-12);
    assert!(implicit_residual(&plane, center).abs() < 1e-12);
    assert!(axis.dot(Vec3::new(phi.sin(), 0.0, phi.cos())) > 0.99);
    // Zero-residual-by-construction (D4 ¶2 identically zero in ℝ):
    // every sampled point satisfies BOTH implicit forms to rounding.
    for i in 0..=32 {
        let t = f64::from(i) / 32.0 * core::f64::consts::TAU;
        let p = e.eval(t);
        assert!(
            implicit_residual(&plane, p).abs() < 1e-13,
            "plane residual at {t}"
        );
        assert!(
            implicit_residual(&cyl, p).abs() < 1e-13,
            "cylinder residual at {t}"
        );
    }
}

#[test]
fn plane_cylinder_rim_stays_rung_1_circle() {
    let cyl = cyl_z(2.0);
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 7.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let s = plane_cylinder_section(&plane, &cyl, 1.0, band()).unwrap();
    let PlaneCylinderSection::Rim(c) = s else {
        panic!("expected the rim circle, got {s:?}");
    };
    let Curve3::Circle {
        center,
        axis,
        radius,
        ..
    } = c
    else {
        panic!("carrier is a circle");
    };
    assert_eq!(radius, 2.0);
    assert_eq!(axis.z, 1.0);
    assert_eq!((center.x, center.y, center.z), (1.0, 2.0, 7.0));
}

#[test]
fn plane_cylinder_parallel_trio() {
    let cyl = cyl_z(2.0);
    // Axis in-plane (normal ⊥ z), gap = 1 < r: two rulings.
    let mk_plane = |gap: f64| Surface::Plane {
        origin: Point3::new(1.0 + gap, 0.0, 0.0),
        normal: Vec3::unit_x(),
        u_ref: Vec3::unit_y(),
    };
    let s = plane_cylinder_section(&mk_plane(1.0), &cyl, 1.0, band()).unwrap();
    let PlaneCylinderSection::ParallelLines { l1, l2 } = s else {
        panic!("expected two rulings, got {s:?}");
    };
    for l in [&l1, &l2] {
        let Curve3::Line { origin, dir } = l else {
            panic!("ruling is a line");
        };
        assert_eq!(dir.z, 1.0);
        assert!(implicit_residual(&mk_plane(1.0), *origin).abs() < 1e-12);
        assert!(implicit_residual(&cyl, *origin).abs() < 1e-12);
    }
    // Gap exactly r: the tangency ruling (classification data).
    let s = plane_cylinder_section(&mk_plane(2.0), &cyl, 1.0, band()).unwrap();
    assert!(matches!(s, PlaneCylinderSection::TangentLine(_)), "{s:?}");
    // Gap definitely > r: empty.
    let s = plane_cylinder_section(&mk_plane(3.0), &cyl, 1.0, band()).unwrap();
    assert!(matches!(s, PlaneCylinderSection::Empty), "{s:?}");
    // Gap in the band (r + 3ε): escalated typed — F6, with the shared
    // recourse riding the Indeterminate Display exactly once.
    let err = plane_cylinder_section(&mk_plane(2.0 + 3.0 * eps()), &cyl, 1.0, band()).unwrap_err();
    let SectionError::Escalated(_) = err else {
        panic!("expected escalation, got {err:?}");
    };
    let msg = err.to_string();
    assert_eq!(msg.matches(geom_core::COINCIDENCE_RECOURSE).count(), 1);
    assert!(msg.contains("ill-conditioned operand pair"), "{msg}");
}

#[test]
fn plane_cylinder_axis_angle_trios() {
    let cyl = cyl_z(2.0);
    // pc_axis_plane_parallel in-band: axis·normal = 3ε (at extent 1).
    let t = 3.0 * eps();
    let plane = Surface::Plane {
        origin: Point3::new(10.0, 0.0, 0.0),
        normal: Vec3::new((1.0 - t * t).sqrt(), 0.0, t),
        u_ref: Vec3::unit_y(),
    };
    let err = plane_cylinder_section(&plane, &cyl, 1.0, band()).unwrap_err();
    assert!(matches!(err, SectionError::Escalated(_)), "{err:?}");

    // pc_rim_alignment in-band: ‖axis×normal‖·r = 3ε ⇒ tilt sine
    // 1.5ε at r = 2.
    let s = 1.5 * eps();
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 7.0),
        normal: Vec3::new(s, 0.0, (1.0 - s * s).sqrt()),
        u_ref: Vec3::unit_y(),
    };
    let err = plane_cylinder_section(&plane, &cyl, 1.0, band()).unwrap_err();
    assert!(matches!(err, SectionError::Escalated(_)), "{err:?}");

    // Between the rim gate and the ellipse constructor: a small-but-
    // definite tilt whose axis separation a − b = r(1/cos φ − 1) lands
    // in the constructor's band — the constructor is the one deciding
    // door for the KIND, and its escalation wins (documented double
    // gate). sin φ = 1e-4 ⇒ margin ‖a×n‖·r = 2e-4 (definite),
    // a − b ≈ r·φ²/2 = 1e-8 = in-band at ε = 1e-9… definite at
    // ε = 1e-12, in the Zero band at ε = 1e-6 — so this row only runs
    // at the default ε.
    if (eps() - 1e-9).abs() < 1e-15 {
        let s = 5.5e-5;
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 7.0),
            normal: Vec3::new(s, 0.0, (1.0 - s * s).sqrt()),
            u_ref: Vec3::unit_y(),
        };
        let err = plane_cylinder_section(&plane, &cyl, 1.0, band()).unwrap_err();
        assert!(matches!(err, SectionError::Carrier(_)), "{err:?}");
        let msg = err.to_string();
        assert_eq!(msg.matches(geom_core::COINCIDENCE_RECOURSE).count(), 1);
    }
}

#[test]
fn wrong_lane_is_typed() {
    let cyl = cyl_z(1.0);
    let err = plane_cylinder_section(&cyl, &cyl, 1.0, band()).unwrap_err();
    assert!(matches!(err, SectionError::WrongLane { .. }));
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let plane = tilted_plane(0.3, Point3::origin());
    let err = plane_cylinder_section(&plane, &sphere, 1.0, band()).unwrap_err();
    assert!(matches!(err, SectionError::WrongLane { .. }));
}

// ---------------------------------------------------------------------
// cylinder × cylinder, equal radii (§3.2)
// ---------------------------------------------------------------------

/// Two equal-radius cylinders with axes crossing at the origin in the
/// xz-plane, each tilted `gamma` off z.
fn crossing_pair(r: f64, gamma: f64) -> (Surface<f64>, Surface<f64>) {
    let a1 = Vec3::new(gamma.sin(), 0.0, gamma.cos());
    let a2 = Vec3::new(-gamma.sin(), 0.0, gamma.cos());
    let mk = |axis: Vec3<f64>| Surface::Cylinder {
        origin: Point3::origin(),
        axis,
        radius: r,
        u_ref: Vec3::unit_y(),
    };
    (mk(a1), mk(a2))
}

#[test]
fn equal_cylinders_split_into_two_ellipses() {
    let (c1, c2) = crossing_pair(1.5, 0.6);
    let s = cylinder_cylinder_section(&c1, &c2, RadiusEvidence::Declared, 1.0, band()).unwrap();
    let EqualCylinderSection::TwoEllipses { e1, e2 } = s else {
        panic!("expected two ellipses, got {s:?}");
    };
    // The two-ellipse split verified against parametric substitution
    // residuals (spec §6): every sampled point of BOTH ellipses lies on
    // BOTH cylinders — exact in ℝ, rounding-scale at f64.
    for (name, e) in [("e1", &e1), ("e2", &e2)] {
        for i in 0..=32 {
            let t = f64::from(i) / 32.0 * core::f64::consts::TAU;
            let p = e.eval(t);
            assert!(
                implicit_residual(&c1, p).abs() < 1e-13,
                "{name}: cylinder-1 residual at {t}: {}",
                implicit_residual(&c1, p)
            );
            assert!(
                implicit_residual(&c2, p).abs() < 1e-13,
                "{name}: cylinder-2 residual at {t}: {}",
                implicit_residual(&c2, p)
            );
        }
    }
    // Both are genuine ellipses centered at the axes' intersection.
    for e in [&e1, &e2] {
        let Curve3::Ellipse { center, minor, .. } = e else {
            panic!("carrier is an ellipse");
        };
        assert!(center.x.abs() < 1e-12 && center.y.abs() < 1e-12 && center.z.abs() < 1e-12);
        assert_eq!(*minor, 1.5);
    }
}

#[test]
fn radius_equality_is_never_inferred_from_values() {
    // Bitwise-equal radii WITHOUT ladder evidence: routes to rung 3 —
    // the never-infer rule, pinned.
    let (c1, c2) = crossing_pair(1.5, 0.6);
    let err = cylinder_cylinder_section(&c1, &c2, RadiusEvidence::None, 1.0, band()).unwrap_err();
    let SectionError::RoutesToGeneralRung { why, .. } = err else {
        panic!("expected the rung-3 routing refusal, got {err:?}");
    };
    refusal_is_grounded(why, "cylinder x cylinder, undeclared");
    // Pinned on wording unique to THIS note (`:840`), not on tokens it
    // shares with the skew one.
    assert!(why.contains("never inferred from values"), "{why}");
    assert!(
        why.contains("the undeclared pair routes to the general rung"),
        "{why}"
    );
    assert!(
        why.contains("cylinder×cylinder arm has not retired"),
        "{why}"
    );
}

#[test]
fn declared_radius_equality_is_verified() {
    // Definitely unequal radii under a (false) declaration: verified
    // and contradicted, typed.
    let (c1, _) = crossing_pair(1.5, 0.6);
    let c2 = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::new(-0.6f64.sin(), 0.0, 0.6f64.cos()),
        radius: 1.75,
        u_ref: Vec3::unit_y(),
    };
    let err =
        cylinder_cylinder_section(&c1, &c2, RadiusEvidence::Declared, 1.0, band()).unwrap_err();
    assert!(
        matches!(err, SectionError::RadiusDeclarationContradicted),
        "{err:?}"
    );
    // In-band radius difference: escalated (the declared pair is
    // ill-conditioned at this ε, F6).
    let c2 = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::new(-0.6f64.sin(), 0.0, 0.6f64.cos()),
        radius: 1.5 + 3.0 * eps(),
        u_ref: Vec3::unit_y(),
    };
    let err =
        cylinder_cylinder_section(&c1, &c2, RadiusEvidence::Declared, 1.0, band()).unwrap_err();
    assert!(matches!(err, SectionError::Escalated(_)), "{err:?}");
}

#[test]
fn skew_axes_route_to_rung_3() {
    let (c1, mut c2) = crossing_pair(1.5, 0.6);
    if let Surface::Cylinder { origin, .. } = &mut c2 {
        *origin = Point3::new(0.0, 0.5, 0.0); // definitely off-plane
    }
    let err =
        cylinder_cylinder_section(&c1, &c2, RadiusEvidence::Declared, 1.0, band()).unwrap_err();
    let SectionError::RoutesToGeneralRung { why, .. } = err else {
        panic!("expected the rung-3 routing refusal, got {err:?}");
    };
    refusal_is_grounded(why, "cylinder x cylinder, skew axes");
    assert!(why.contains("skew axes have no conic section"), "{why}");

    // In-band axis offset: escalated.
    if let Surface::Cylinder { origin, .. } = &mut c2 {
        *origin = Point3::new(0.0, 3.0 * eps(), 0.0);
    }
    let err =
        cylinder_cylinder_section(&c1, &c2, RadiusEvidence::Declared, 1.0, band()).unwrap_err();
    assert!(matches!(err, SectionError::Escalated(_)), "{err:?}");
}

#[test]
fn parallel_equal_cylinders_trio() {
    let mk = |offset: f64| Surface::Cylinder {
        origin: Point3::new(offset, 0.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let c1 = mk(0.0);
    // Overlapping (gap 1 < 2r = 2): two rulings, on both surfaces.
    let s =
        cylinder_cylinder_section(&c1, &mk(1.0), RadiusEvidence::Declared, 1.0, band()).unwrap();
    let EqualCylinderSection::ParallelLines { l1, l2 } = s else {
        panic!("expected two rulings, got {s:?}");
    };
    for l in [&l1, &l2] {
        let Curve3::Line { origin, .. } = l else {
            panic!("ruling is a line");
        };
        assert!(implicit_residual(&c1, *origin).abs() < 1e-12);
        assert!(implicit_residual(&mk(1.0), *origin).abs() < 1e-12);
    }
    // Exactly tangent (gap = 2r): the tangency ruling.
    let s =
        cylinder_cylinder_section(&c1, &mk(2.0), RadiusEvidence::Declared, 1.0, band()).unwrap();
    assert!(matches!(s, EqualCylinderSection::TangentLine(_)), "{s:?}");
    // Definitely apart: empty.
    let s =
        cylinder_cylinder_section(&c1, &mk(3.0), RadiusEvidence::Declared, 1.0, band()).unwrap();
    assert!(matches!(s, EqualCylinderSection::Empty), "{s:?}");
    // In-band gap: escalated (F6).
    let err = cylinder_cylinder_section(
        &c1,
        &mk(2.0 + 3.0 * eps()),
        RadiusEvidence::Declared,
        1.0,
        band(),
    )
    .unwrap_err();
    assert!(matches!(err, SectionError::Escalated(_)), "{err:?}");
    // Coaxial equal-radius: the coincident-surface refusal, carrying
    // the shared recourse exactly once.
    let err = cylinder_cylinder_section(&c1, &mk(0.0), RadiusEvidence::Declared, 1.0, band())
        .unwrap_err();
    assert!(matches!(err, SectionError::CoincidentSurfaces), "{err:?}");
    let msg = err.to_string();
    assert_eq!(msg.matches(geom_core::COINCIDENCE_RECOURSE).count(), 1);
}

// ---------------------------------------------------------------------
// plane × cone, exact-degenerates only (§3.3, R1)
// ---------------------------------------------------------------------

fn cone_z(half_angle: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::unit_z(),
        half_angle,
        u_ref: Vec3::unit_x(),
    }
}

#[test]
fn plane_cone_apex_trio() {
    let cone = cone_z(core::f64::consts::FRAC_PI_6); // α = 30°
    // A plane through the apex, steep enough to dip inside the cone
    // (angle from axis < α): two generator lines. Normal ⊥ z would be
    // a plane CONTAINING the axis — definitely two lines.
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 1.0),
        normal: Vec3::unit_x(),
        u_ref: Vec3::unit_y(),
    };
    let s = plane_cone_section(&plane, &cone, 1.0, band()).unwrap();
    let PlaneConeSection::ApexLinePair { l1, l2 } = s else {
        panic!("expected the apex line pair, got {s:?}");
    };
    for l in [&l1, &l2] {
        let Curve3::Line { origin, dir } = l else {
            panic!("generator is a line");
        };
        assert_eq!((origin.x, origin.y, origin.z), (0.0, 0.0, 1.0));
        // On the plane and on the cone: dir ⊥ normal, dir at angle α
        // to the axis.
        assert!(dir.x.abs() < 1e-12, "in-plane");
        assert!(
            (dir.dot(Vec3::unit_z()) - core::f64::consts::FRAC_PI_6.cos()).abs() < 1e-12,
            "generator angle"
        );
    }
    // A plane through the apex tilted just at the cone: tangent along
    // one generator — exactly-degenerate. Plane normal at angle
    // (π/2 − α) from the axis touches the cone along one generator.
    let a = core::f64::consts::FRAC_PI_6;
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 1.0),
        normal: Vec3::new(
            (core::f64::consts::FRAC_PI_2 - a).sin(),
            0.0,
            -(core::f64::consts::FRAC_PI_2 - a).cos(),
        ),
        u_ref: Vec3::unit_y(),
    };
    let s = plane_cone_section(&plane, &cone, 1.0, band()).unwrap();
    let PlaneConeSection::ApexTangentLine(l) = s else {
        panic!("expected the tangent generator, got {s:?}");
    };
    let Curve3::Line { dir, .. } = l else {
        panic!("generator is a line");
    };
    // The tangent generator lies in the plane.
    assert!(
        dir.dot(Vec3::new(
            (core::f64::consts::FRAC_PI_2 - a).sin(),
            0.0,
            -(core::f64::consts::FRAC_PI_2 - a).cos()
        ))
        .abs()
            < 1e-9,
        "tangent generator in-plane"
    );
    // A shallow plane through the apex (angle from axis > α): the apex
    // point alone.
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 1.0),
        normal: Vec3::new(0.3f64.sin(), 0.0, 0.3f64.cos()),
        u_ref: Vec3::unit_y(),
    };
    let s = plane_cone_section(&plane, &cone, 1.0, band()).unwrap();
    let PlaneConeSection::ApexPoint(p) = s else {
        panic!("expected the apex point, got {s:?}");
    };
    assert_eq!((p.x, p.y, p.z), (0.0, 0.0, 1.0));
    // In-band apex offset: escalated (F6).
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 1.0 + 3.0 * eps()),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let err = plane_cone_section(&plane, &cone, 1.0, band()).unwrap_err();
    assert!(matches!(err, SectionError::Escalated(_)), "{err:?}");
}

#[test]
fn plane_cone_axis_normal_circle() {
    let cone = cone_z(core::f64::consts::FRAC_PI_6);
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 3.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let s = plane_cone_section(&plane, &cone, 1.0, band()).unwrap();
    let PlaneConeSection::AxisNormalCircle(c) = s else {
        panic!("expected the circle, got {s:?}");
    };
    let Curve3::Circle {
        center,
        radius,
        axis,
        ..
    } = c
    else {
        panic!("carrier is a circle");
    };
    // h = 2 above the apex ⇒ r = 2·tan 30°.
    assert!((radius - 2.0 * core::f64::consts::FRAC_PI_6.tan()).abs() < 1e-12);
    assert_eq!((center.x, center.y, center.z), (0.0, 0.0, 3.0));
    assert_eq!(axis.z, 1.0);
    // Residuals: on the cone and on the plane, everywhere.
    for i in 0..=16 {
        let t = f64::from(i) / 16.0 * core::f64::consts::TAU;
        let p = c.eval(t);
        assert!(implicit_residual(&plane, p).abs() < 1e-13);
        assert!(implicit_residual(&cone, p).abs() < 1e-13);
    }
}

#[test]
fn plane_cone_generic_tilt_refuses_typed_r1() {
    let cone = cone_z(core::f64::consts::FRAC_PI_6);
    let plane = tilted_plane(0.4, Point3::new(0.0, 0.0, 5.0));
    let err = plane_cone_section(&plane, &cone, 1.0, band()).unwrap_err();
    let SectionError::RoutesToGeneralRung { pair, why } = err else {
        panic!("expected the R1 routing refusal, got {err:?}");
    };
    assert_eq!(pair, "plane×cone");
    refusal_is_grounded(why, "plane x cone, generic tilt");
    assert!(why.contains("PERMANENTLY"), "{why}");
    // Unique to this note: the permanence has a REASON, and the reason
    // is what a rewrite must not drop.
    assert!(why.contains("parabola/hyperbola"), "{why}");
    assert!(why.contains("The general rung is implemented"), "{why}");
    assert!(why.contains("not waiting on it"), "{why}");
}

// ---------------------------------------------------------------------
// The interval lane: classification replays and residuals enclose zero
// ---------------------------------------------------------------------

#[cfg(feature = "interval")]
mod interval {
    use super::*;
    use geom_core::{Bounds, Interval, Real};

    fn ip(p: Point3<f64>) -> Point3<Interval> {
        Point3::new(
            Interval::from_f64(p.x),
            Interval::from_f64(p.y),
            Interval::from_f64(p.z),
        )
    }

    fn iv(v: Vec3<f64>) -> Vec3<Interval> {
        Vec3::new(
            Interval::from_f64(v.x),
            Interval::from_f64(v.y),
            Interval::from_f64(v.z),
        )
    }

    /// The tilted plane×cylinder classification runs at `T = Interval`
    /// and its ellipse's residual enclosures contain zero — the
    /// exact-in-ℝ claim, certified.
    #[test]
    fn tilted_ellipse_residuals_enclose_zero_at_interval() {
        let phi = 0.5f64;
        let plane: Surface<Interval> = Surface::Plane {
            origin: ip(Point3::new(0.0, 0.0, 7.0)),
            normal: iv(Vec3::new(phi.sin(), 0.0, phi.cos())),
            u_ref: iv(Vec3::new(phi.cos(), 0.0, -phi.sin())),
        };
        let cyl: Surface<Interval> = Surface::Cylinder {
            origin: ip(Point3::new(1.0, 2.0, 3.0)),
            axis: iv(Vec3::unit_z()),
            radius: Interval::from_f64(2.0),
            u_ref: iv(Vec3::unit_x()),
        };
        let s = plane_cylinder_section(&plane, &cyl, Interval::one(), band()).unwrap();
        let PlaneCylinderSection::TiltedEllipse(e) = s else {
            panic!("expected the tilted ellipse, got {s:?}");
        };
        for t in [0.0, 0.7, 2.9, -1.3, 4.4] {
            let p = e.eval(Interval::from_f64(t));
            for (name, r) in [
                ("plane", implicit_residual(&plane, p)),
                ("cylinder", implicit_residual(&cyl, p)),
            ] {
                assert!(
                    r.lo() <= 0.0 && 0.0 <= r.hi(),
                    "{name} residual at {t}: [{}, {}]",
                    r.lo(),
                    r.hi()
                );
                assert!(r.hi() - r.lo() < 1e-12, "{name} width at {t}");
            }
        }
    }

    /// The two-ellipse split's residual enclosures contain zero against
    /// BOTH cylinders (spec §6: exact in ℝ; enclosure width in f64).
    #[test]
    fn two_ellipse_split_residuals_enclose_zero_at_interval() {
        let gamma = 0.6f64;
        let mk = |sign: f64| -> Surface<Interval> {
            Surface::Cylinder {
                origin: ip(Point3::origin()),
                axis: iv(Vec3::new(sign * gamma.sin(), 0.0, gamma.cos())),
                radius: Interval::from_f64(1.5),
                u_ref: iv(Vec3::unit_y()),
            }
        };
        let (c1, c2) = (mk(1.0), mk(-1.0));
        let s =
            cylinder_cylinder_section(&c1, &c2, RadiusEvidence::Declared, Interval::one(), band())
                .unwrap();
        let EqualCylinderSection::TwoEllipses { e1, e2 } = s else {
            panic!("expected two ellipses, got {s:?}");
        };
        for e in [&e1, &e2] {
            for t in [0.0, 0.9, 2.2, -2.8] {
                let p = e.eval(Interval::from_f64(t));
                for c in [&c1, &c2] {
                    let r = implicit_residual(c, p);
                    assert!(r.lo() <= 0.0 && 0.0 <= r.hi(), "[{}, {}]", r.lo(), r.hi());
                    assert!(r.hi() - r.lo() < 1e-12);
                }
            }
        }
    }

    /// The sphere×sphere radical-plane circle runs at `T = Interval`
    /// UNCHANGED — no lane fork, because the form is `atan2`-free and
    /// branch-cut-free by construction — and its residual enclosures
    /// contain zero against BOTH spheres.
    #[test]
    fn sphere_sphere_residuals_enclose_zero_at_interval() {
        let mk = |c: Point3<f64>, r: f64| -> Surface<Interval> {
            Surface::Sphere {
                center: ip(c),
                radius: Interval::from_f64(r),
                axis: iv(Vec3::unit_y()),
                u_ref: iv(Vec3::unit_x()),
            }
        };
        let a = mk(Point3::new(1.0, 2.0, 3.0), 2.0);
        let b = mk(Point3::new(2.0, 0.5, 4.5), 1.75);
        let s = geom_brep::intersect::sphere_sphere_section(&a, &b, band()).unwrap();
        let geom_brep::intersect::SphereSphereSection::Circle(c) = s else {
            panic!("expected the circle, got {s:?}");
        };
        for t in [0.0, 0.9, 2.2, -2.8, 5.1] {
            let p = c.eval(Interval::from_f64(t));
            for (name, s) in [("sphere-1", &a), ("sphere-2", &b)] {
                let r = implicit_residual(s, p);
                assert!(
                    r.lo() <= 0.0 && 0.0 <= r.hi(),
                    "{name} residual at {t}: [{}, {}]",
                    r.lo(),
                    r.hi()
                );
                assert!(r.hi() - r.lo() < 1e-12, "{name} width at {t}");
            }
        }
    }
}

// ---------------------------------------------------------------------
// Fix-pass M3 rows: the remaining in-band escalation arms
// ---------------------------------------------------------------------

/// `cc_axes_parallel` in-band: an axis pair 3ε off parallel (sine at
/// extent 1) escalates typed after the radius declaration verifies.
#[test]
fn cc_axes_parallel_in_band_escalates() {
    let s = 3.0 * eps();
    let c1 = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let c2 = Surface::Cylinder {
        origin: Point3::new(1.0, 0.0, 0.0),
        axis: Vec3::new(s, 0.0, (1.0 - s * s).sqrt()),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let err =
        cylinder_cylinder_section(&c1, &c2, RadiusEvidence::Declared, 1.0, band()).unwrap_err();
    let SectionError::Escalated(diag) = err else {
        panic!("expected escalation, got {err:?}");
    };
    assert_eq!(diag.predicate, Some("cc_axes_parallel"));
}

/// `cc_coaxial` in-band: parallel equal-radius cylinders 3ε apart are
/// too close to the coincident-surface coincidence — escalated typed.
#[test]
fn cc_coaxial_in_band_escalates() {
    let mk = |offset: f64| Surface::Cylinder {
        origin: Point3::new(offset, 0.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let err = cylinder_cylinder_section(
        &mk(0.0),
        &mk(3.0 * eps()),
        RadiusEvidence::Declared,
        1.0,
        band(),
    )
    .unwrap_err();
    let SectionError::Escalated(diag) = err else {
        panic!("expected escalation, got {err:?}");
    };
    assert_eq!(diag.predicate, Some("cc_coaxial"));
}

/// `pn_apex_section` in-band: an apex-through plane whose two-line
/// discriminant sits 3ε inside the band (ψ = π/2 − α + 3ε at extent 1)
/// escalates typed.
#[test]
fn pn_apex_section_in_band_escalates() {
    let a = core::f64::consts::FRAC_PI_6;
    let cone = cone_z(a);
    let psi = core::f64::consts::FRAC_PI_2 - a + 3.0 * eps();
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 1.0), // through the apex
        normal: Vec3::new(psi.sin(), 0.0, psi.cos()),
        u_ref: Vec3::unit_y(),
    };
    let err = plane_cone_section(&plane, &cone, 1.0, band()).unwrap_err();
    let SectionError::Escalated(diag) = err else {
        panic!("expected escalation, got {err:?}");
    };
    assert_eq!(diag.predicate, Some("pn_apex_section"));
}

/// `pn_axis_normal` in-band: an off-apex plane tilted so the
/// alignment sine at the cut-radius arm lands in the band escalates
/// typed (neither the circle nor the R1 routing may be guessed).
#[test]
fn pn_axis_normal_in_band_escalates() {
    let a = core::f64::consts::FRAC_PI_6;
    let cone = cone_z(a);
    let rim_r = 2.0 * a.tan(); // h = 2 above the apex
    let s = 3.0 * eps() / rim_r;
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 3.0),
        normal: Vec3::new(s, 0.0, (1.0 - s * s).sqrt()),
        u_ref: Vec3::unit_y(),
    };
    let err = plane_cone_section(&plane, &cone, 1.0, band()).unwrap_err();
    let SectionError::Escalated(diag) = err else {
        panic!("expected escalation, got {err:?}");
    };
    assert_eq!(diag.predicate, Some("pn_axis_normal"));
}

// ---------------------------------------------------------------------
// plane × sphere (M5 S13 — the die-pips row)
// ---------------------------------------------------------------------

fn sphere_at(c: Point3<f64>, r: f64) -> Surface<f64> {
    Surface::Sphere {
        center: c,
        radius: r,
        axis: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
    }
}

/// The definite cut is the exact Circle — center at the sphere
/// center's plane foot, radius √(r² − s²), axis the plane normal —
/// and the residual is identically zero in ℝ against BOTH surfaces,
/// attacked with a tilted plane (the review charter's angle).
#[test]
fn plane_sphere_cut_is_the_exact_circle_zero_residual() {
    use geom_brep::intersect::{PlaneSphereSection, plane_sphere_section};
    let sph = sphere_at(Point3::new(1.0, 2.0, 3.0), 2.0);
    // Deliberately tilted: normal nowhere near a chart axis.
    let n = Vec3::new(1.0, -2.0, 2.0) / 3.0;
    let plane = Surface::Plane {
        origin: Point3::new(0.5, 1.0, 2.0),
        normal: n,
        u_ref: Vec3::new(2.0, 2.0, 1.0) / 3.0,
    };
    let s = plane_sphere_section(&plane, &sph, band()).unwrap();
    let PlaneSphereSection::Circle(c) = s else {
        panic!("expected the circle, got {s:?}");
    };
    let Curve3::Circle {
        center,
        axis,
        radius,
        ..
    } = c
    else {
        panic!("carrier is a circle");
    };
    let gap = (Point3::new(1.0, 2.0, 3.0) - plane_origin(&plane)).dot(n);
    assert!((radius - (4.0 - gap * gap).sqrt()).abs() < 1e-12);
    assert!(axis.dot(n) > 0.999_999);
    assert!(implicit_residual(&plane, center).abs() < 1e-12);
    for k in 0..17 {
        let p = c.eval(0.37 * k as f64);
        assert!(
            implicit_residual(&plane, p).abs() < 1e-12,
            "plane residual at sample {k}"
        );
        assert!(
            implicit_residual(&sph, p).abs() < 1e-12,
            "sphere residual at sample {k}"
        );
    }
}

fn plane_origin(p: &Surface<f64>) -> Point3<f64> {
    let Surface::Plane { origin, .. } = p else {
        panic!("plane")
    };
    *origin
}

/// The `ps_center_gap` trilean trio: definite cut / exact tangency
/// (a POINT, classification data — C7, never a carrier) / definite
/// empty — and the in-band twin of all three escalates typed (F6),
/// the two-tolerance pair on one named predicate.
#[test]
fn plane_sphere_gap_trilean_trio() {
    use geom_brep::intersect::{PlaneSphereSection, plane_sphere_section};
    let sph = sphere_at(Point3::new(0.0, 0.0, 0.0), 1.0);
    let plane_z = |z: f64| Surface::Plane {
        origin: Point3::new(10.0, -3.0, z),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    // Definite cut.
    assert!(matches!(
        plane_sphere_section(&plane_z(0.25), &sph, band()).unwrap(),
        PlaneSphereSection::Circle(_)
    ));
    // Exact tangency: the point, at the pole.
    let s = plane_sphere_section(&plane_z(1.0), &sph, band()).unwrap();
    let PlaneSphereSection::TangentPoint(p) = s else {
        panic!("expected the tangent point, got {s:?}");
    };
    assert!(p.x.abs() < 1e-15 && p.y.abs() < 1e-15 && (p.z - 1.0).abs() < 1e-15);
    // Definite empty.
    assert!(matches!(
        plane_sphere_section(&plane_z(1.5), &sph, band()).unwrap(),
        PlaneSphereSection::Empty
    ));
    // In-band: |s| = r + 3ε — neither tangent nor clear; escalated.
    let err = plane_sphere_section(&plane_z(1.0 + 3.0 * eps()), &sph, band())
        .expect_err("in-band gap must escalate");
    let SectionError::Escalated(_) = err else {
        panic!("expected escalation, got {err:?}");
    };
    // Wrong-lane kinds refuse typed.
    let err = plane_sphere_section(&sph, &sph, band()).expect_err("wrong lane");
    assert!(matches!(err, SectionError::WrongLane { .. }));
}

/// A sphere with a **tilted, non-chart-aligned** centre offset — the
/// generic pair — cuts in the exact radical-plane Circle: the residual
/// is identically zero in ℝ against BOTH surfaces, the axis is the unit
/// centre direction, and the radius matches the closed form derived
/// independently here.
#[test]
fn sphere_sphere_cut_is_the_exact_circle_zero_residual() {
    use geom_brep::intersect::{SphereSphereSection, sphere_sphere_section};
    let (c1, r1) = (Point3::new(1.0, 2.0, 3.0), 2.0);
    let (c2, r2) = (Point3::new(2.0, 0.5, 4.5), 1.75);
    let a = sphere_at(c1, r1);
    let b = sphere_at(c2, r2);
    let SphereSphereSection::Circle(c) = sphere_sphere_section(&a, &b, band()).unwrap() else {
        panic!("expected the circle");
    };
    let Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = c
    else {
        panic!("carrier is a circle");
    };
    let delta = c2 - c1;
    let d = delta.norm();
    let n = delta / d;
    let off = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    assert!((radius - (r1 * r1 - off * off).sqrt()).abs() < 1e-12);
    assert!((center - (c1 + n * off)).norm() < 1e-12);
    assert!(axis.dot(n) > 0.999_999_999);
    // The placement is a unit vector in the section plane (D2/D9).
    assert!((u_ref.norm() - 1.0).abs() < 1e-12);
    assert!(u_ref.dot(n).abs() < 1e-12);
    for k in 0..17 {
        let p = c.eval(0.37 * k as f64);
        assert!(
            implicit_residual(&a, p).abs() < 1e-12,
            "sphere-1 residual at sample {k}"
        );
        assert!(
            implicit_residual(&b, p).abs() < 1e-12,
            "sphere-2 residual at sample {k}"
        );
    }
}

/// The `ss_carrier_*` trilean trio, in gate order: coincident surfaces
/// refuse typed (a whole-sphere REST is a coincidence to declare, never
/// a section); separated and strictly nested are both `Empty`; external
/// and internal tangency are both the one POINT (C7 — classification
/// data, never a carrier), and both land at `c₁ + n̂·r₁`; the in-band
/// twin of each definite verdict escalates through its own named
/// predicate (F6), the two-tolerance pair.
#[test]
fn sphere_sphere_carrier_trilean_trio() {
    use geom_brep::intersect::{SphereSphereSection, sphere_sphere_section};
    let unit = sphere_at(Point3::origin(), 1.0);
    let at = |x: f64, r: f64| sphere_at(Point3::new(x, 0.0, 0.0), r);
    let sec = |x: f64, r: f64| sphere_sphere_section(&unit, &at(x, r), band());

    // One sphere given twice: the identity gate, refused typed.
    let err = sphere_sphere_section(&unit, &unit, band()).expect_err("coincident");
    assert!(matches!(err, SectionError::CoincidentSurfaces));
    // Concentric but unequal is NOT coincident — it is strictly nested.
    assert!(matches!(sec(0.0, 0.5).unwrap(), SphereSphereSection::Empty));
    // Definitely separated / definitely nested.
    assert!(matches!(sec(3.0, 1.0).unwrap(), SphereSphereSection::Empty));
    assert!(matches!(
        sec(0.25, 0.5).unwrap(),
        SphereSphereSection::Empty
    ));
    // Externally tangent: d = r₁ + r₂, the point at +x on sphere 1.
    let SphereSphereSection::TangentPoint(p) = sec(1.5, 0.5).unwrap() else {
        panic!("expected external tangency");
    };
    assert!((p - Point3::new(1.0, 0.0, 0.0)).norm() < 1e-15);
    // Internally tangent: d = |r₁ − r₂|, the SAME point.
    let SphereSphereSection::TangentPoint(p) = sec(0.5, 0.5).unwrap() else {
        panic!("expected internal tangency");
    };
    assert!((p - Point3::new(1.0, 0.0, 0.0)).norm() < 1e-15);
    // Proper secant between the two tangencies.
    assert!(matches!(
        sec(1.0, 0.5).unwrap(),
        SphereSphereSection::Circle(_)
    ));

    // Each definite verdict's in-band twin escalates, on its own
    // predicate: the external clearance, the internal clearance, and
    // the identity gate.
    for (x, r, what) in [
        (1.5 + 3.0 * eps(), 0.5, "external"),
        (0.5 + 3.0 * eps(), 0.5, "internal"),
        (3.0 * eps(), 1.0, "identity"),
    ] {
        let err = sec(x, r).expect_err(what);
        let SectionError::Escalated(_) = err else {
            panic!("{what}: expected escalation, got {err:?}");
        };
    }

    // Wrong-lane kinds refuse typed, both sides.
    let plane = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    for (a, b) in [(&plane, &unit), (&unit, &plane)] {
        let err = sphere_sphere_section(a, b, band()).expect_err("wrong lane");
        assert!(matches!(err, SectionError::WrongLane { .. }));
    }
}
