//! **R1 review probes for VERBS-GERMARMS PR-1** (the curved pierce
//! ring lane). Lane-private, blinded; these rows are falsification
//! attempts on the PR body's load-bearing claims, not acceptance.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_brep::{EntersMaterial, OutwardNormal, enters_material, implicit_residual};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn cyl(cx: f64, cy: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(p2(cx, cy), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

fn boxx(x0: f64, x1: f64, y0: f64, y1: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp: ProfileLoop<f64> = RawLoop::polygon([p2(x0, y0), p2(x1, y0), p2(x1, y1), p2(x0, y1)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

/// **PROBE 1 — the tangency finding, re-derived from the surfaces
/// rather than from the fold's printout.**
///
/// A's seam ruling in the steinmetz fixture is `x = −1, y = 0`,
/// `z ∈ [−2, 2]`; B's wall is the unit cylinder about the y axis,
/// `x² + z² = 1`. The residual along the seam is therefore `z²/2` —
/// non-negative everywhere, zero at exactly one interior point. That is
/// a TANGENCY, not a crossing, and the PR's claim that the ring lane
/// must not move this row stands on it.
#[test]
fn r1_the_steinmetz_seam_residual_is_a_nonnegative_parabola() {
    let b_wall = geom::Surface::Cylinder {
        origin: Point3::new(0.0, 2.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    for k in -20i32..=20 {
        let z = f64::from(k) * 0.1;
        let r = implicit_residual(&b_wall, Point3::new(-1.0, 0.0, z));
        assert!((r - z * z / 2.0).abs() < 1e-15, "z={z}: {r}");
        assert!(r >= 0.0, "the seam dips through the wall at z={z}");
    }
    // Exactly zero at the section's singular point, and the two section
    // ellipses (y = ±z) do cross there.
    assert_eq!(implicit_residual(&b_wall, Point3::new(-1.0, 0.0, 0.0)), 0.0);

    // The fold's own arithmetic (reduce.rs, the (Positive, Positive)
    // arm), reproduced: f2 = (|d|² − (d·axis)²)/r, q = f2·Δt², m = the
    // endpoint gap, charge = max(0, q/2 − m)/4.
    let dir = Vec3::new(0.0, 0.0, 1.0);
    let axis = Vec3::new(0.0, -1.0, 0.0);
    let f2 = (dir.norm_squared() - dir.dot(axis).powi(2)) / 1.0;
    let q = f2 * 4.0f64.powi(2);
    let r_u = implicit_residual(&b_wall, Point3::new(-1.0, 0.0, -2.0));
    let r_v = implicit_residual(&b_wall, Point3::new(-1.0, 0.0, 2.0));
    let m = (r_v - r_u).abs();
    let charge = (q * 0.5 - m).max(0.0) * 0.25;
    assert_eq!((f2, q, m, charge), (1.0, 16.0, 0.0, 2.0));
    // m = 0 ⇒ the charge IS the true dip (chord 2, true minimum 0), so
    // the bound is exact and the zero it reports is the geometry's.
    assert_eq!(charge, r_u.min(r_v) - 0.0);
    assert_eq!(r_u.min(r_v) - charge, 0.0);

    // The 45°-spun control: the same fold, a definite crossing, and the
    // bound is exact there too (true minimum −0.25).
    let c = (PI / 4.0).cos();
    let spun = Point3::new(-c, -c, 0.0);
    let r_u = implicit_residual(&b_wall, Point3::new(spun.x, spun.y, -2.0));
    let charge = (q * 0.5 - 0.0).max(0.0) * 0.25;
    assert_eq!(r_u, 1.75);
    assert_eq!(r_u - charge, -0.25);
    assert_eq!(implicit_residual(&b_wall, spun), -0.25);
}

/// **PROBE 2 — the missing second-order charge (GERMARMS spec item 3).**
///
/// `side_code` decides a sector bound against the pierced face's
/// TANGENT plane at `p`, with no curvature term anywhere in
/// `enters_material` (its margin is `d̂·n̂ · arm`). On a HOLE wall — the
/// material OUTSIDE the cylinder, so the outward normal points at the
/// axis — a bound direction that first-order says leaves the material
/// can be inside it at the sector's own lever arm, by the sagitta
/// `arm²/(2r)`. The spec named exactly this and required a named
/// trilean plus a planted red; neither is in the PR.
///
/// This row is the witness: a DEFINITE `Exits` whose point at the lever
/// arm is definitely in the material.
#[test]
fn r1_a_first_order_exits_verdict_on_a_hole_wall_is_contradicted_at_its_own_arm() {
    let r = 1.0;
    let arm = 0.5;
    let eps = 0.1; // the first-order slope away from the material
    let p = Point3::new(r, 0.0, 0.0);
    // A hole: the solid is everything OUTSIDE the cylinder, so the
    // face's outward normal points toward the axis.
    let n = OutwardNormal::from_chart(Vec3::new(-1.0, 0.0, 0.0), true);
    let d = Vec3::new(-eps, 1.0, 0.0).normalize();
    assert_eq!(
        enters_material(d, n, arm, band()).unwrap(),
        EntersMaterial::Exits,
        "the first-order verdict is a definite Out"
    );
    // The truth at that very lever arm: the wall curves away, so the
    // point is OUTSIDE the cylinder — which is INSIDE the material.
    let q = p + d * arm;
    let rho = (q.x.powi(2) + q.y.powi(2)).sqrt();
    assert!(
        rho > r,
        "the witness needs the arm point outside the wall: {rho}"
    );
    // And the charge the spec asked for would have caught it: the
    // sagitta exceeds the first-order displacement.
    let first_order = d.dot(n.vec()) * arm;
    let sagitta = arm.powi(2) / (2.0 * r);
    assert!(
        sagitta > first_order,
        "sagitta {sagitta} vs first-order {first_order}"
    );
}

/// **PROBE 3 — the "shared absent arm" claim, measured.**
///
/// The PR argues the curved ring's join refusal is the same absent arm
/// the planar cap pierce already has. The two doors are measured here
/// side by side: they are NOT the same sub-case. The planar one is
/// `SectionLoopMixed`, whose own variant doc reads "kernel bug,
/// loudly"; the curved one is `SectionArcWindow{NoChartedRun}`, whose
/// doc reads "A cylinder face's run ALWAYS carries one on the shipped
/// lane; this is the typed door for a corrupt or frontier-carrier run"
/// — a sentence this PR falsifies and leaves standing.
#[test]
fn r1_the_planar_and_curved_ring_joins_refuse_at_different_gates() {
    let tol = Tol::witness();
    let cap = topo::union(
        &cyl(0.0, 0.0, 1.0, 0.0, 2.0),
        &boxx(-0.3, 0.3, -0.3, 0.3, 1.0, 3.0),
        tol,
    )
    .expect_err("the planar cap pierce has no join arm");
    let wall = topo::union(
        &cyl(0.0, 0.0, 1.0, -2.0, 2.0),
        &boxx(-3.0, 3.0, -0.3, 0.3, -0.3, 0.3),
        tol,
    )
    .expect_err("the curved wall pierce has no join arm");
    assert!(
        matches!(
            cap,
            BooleanError::Join(topo::SplitJoinError::SectionLoopMixed { .. })
        ),
        "planar sibling: {cap:?}"
    );
    assert!(
        matches!(
            wall,
            BooleanError::Join(topo::SplitJoinError::SectionArcWindow {
                case: topo::ArcWindowCase::NoChartedRun,
                ..
            })
        ),
        "curved lane: {wall:?}"
    );
}

/// **PROBE 4 — what door the cone fixture actually reaches.**
///
/// The acceptance row asserts only that the cone is NOT the ring lane's
/// join door — a negative. The spec asked for the cone's OWN door. This
/// row prints and pins it, so the differential says something.
#[test]
fn r1_the_cone_fixture_names_its_own_door() {
    let tol = Tol::witness();
    let frustum = {
        let lp = profile::ProfileLoop::new(
            [(0.2, 0.0), (0.6, 0.0), (0.4, 0.6), (0.2, 0.6)]
                .into_iter()
                .map(|(r, y)| profile::ProfileVertex::new(p2(r, y), 0.0))
                .collect(),
        );
        let vp = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .unwrap();
        sweep::revolve(
            &vp,
            sweep::RevolveAxis {
                origin: p2(0.0, 0.0),
                dir: geom_core::Vec2::new(0.0, 1.0),
            },
            sweep::Revolution::Full,
            tol,
        )
        .unwrap()
        .body
    };
    let err = topo::union(&frustum, &boxx(-1.0, 1.0, -0.05, 0.05, 0.25, 0.35), tol)
        .expect_err("no arm for a cone pierce");
    // Measured, not assumed — the row prints what it found.
    assert!(
        matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
        "the cone's own door: {err:?}"
    );
}

/// **PROBE 5 — the grazing red, sharpened.**
///
/// The acceptance row asserts only the VARIANT of the refusal, so any
/// other frontier of the same kind would satisfy it. This row pins that
/// the refusing edge is a LINE (the tangent long edge), which is what
/// makes it the tangency the row claims to plant.
#[test]
fn r1_the_grazing_red_refuses_on_a_line_carrier() {
    let tol = Tol::witness();
    let bar = boxx(-3.0, 3.0, -1.0, 1.0, -0.3, 0.3);
    let pipe = cyl(0.0, 0.0, 1.0, -2.0, 2.0);
    let err = topo::union(&pipe, &bar, tol).expect_err("a tangency keeps the pierce door");
    let BooleanError::CurvedPierceUnsupported { operand, edge, .. } = err else {
        panic!("not the pierce door: {err:?}");
    };
    let owner = match operand {
        topo::Operand::A => &pipe,
        topo::Operand::B => &bar,
    };
    let Some(topo::CurveGeom::Certified(c)) = owner
        .get_edge(edge)
        .and_then(|e| owner.get_curve_geom(e.curve))
    else {
        panic!("the named edge has no certified curve");
    };
    assert!(
        matches!(c.carrier(), topo::Curve3::Line { .. }),
        "the grazing red must refuse on the tangent LINE: {:?}",
        c.carrier()
    );
}
