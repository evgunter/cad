//! **The snowman**: two balls of revolution on distinct centres along
//! one axis, meeting in a real circle, under every boolean.
//!
//! Each ball is the canonical full revolve of a semicircle: two
//! half-bands on one sphere key, their seam meridians crossing the
//! partner sphere. The section is the radical-plane circle, and the
//! rows reach it through the crossing layer's circle × sphere roots and
//! the sphere pair's join. Every body is held to all three validation
//! tiers and to its volume against the two spherical caps the radical
//! plane cuts, computed here from the radii alone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Band, Point2, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

/// A ball of radius `r` centred on the y axis at height `y`.
fn ball(r: f64, y: f64) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, y - r), 1.0),
            (Point2::new(0.0, y + r), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

fn ball_volume(r: f64) -> f64 {
    4.0 / 3.0 * PI * r.powi(3)
}

/// The volume of a spherical cap of height `h` on a sphere of radius `r`.
fn cap_volume(r: f64, h: f64) -> f64 {
    PI * h.powi(2) * (3.0 * r - h) / 3.0
}

/// The lens two balls `r1`, `r2` at centre distance `d` share: the cap
/// of each beyond the radical plane, which sits at
/// `x = (d² + r1² − r2²)/2d` from the first centre.
fn lens_volume(r1: f64, r2: f64, d: f64) -> f64 {
    let x = (d.powi(2) + r1.powi(2) - r2.powi(2)) / (2.0 * d);
    cap_volume(r1, r1 - x) + cap_volume(r2, r2 - (d - x))
}

/// Every tier of validation, a closed tessellation, then the volume
/// against `expected` through the kernel's mass properties — exact on
/// closed-form faces, so the slack is rounding's and a wrong body (a
/// lens counted twice, a cap dropped) misses by orders of magnitude.
fn assert_body(label: &str, body: &Body<f64>, expected: f64) {
    assert_eq!(topo::validate(body), Ok(()), "{label}: validate");
    assert_eq!(
        topo::validate_closed(body),
        Ok(()),
        "{label}: validate_closed"
    );
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "{label}: validate_geometric"
    );
    let m = mesh::tessellate(body, 1e-3, Tol::witness())
        .unwrap_or_else(|e| panic!("{label}: tessellates, got {e:?}"));
    assert_eq!(
        mesh::validate::check_mesh(&m),
        Ok(()),
        "{label}: a closed manifold mesh"
    );
    let p = topo::mass_properties(body, Tol::witness())
        .unwrap_or_else(|e| panic!("{label}: mass properties, got {e:?}"));
    assert_eq!(p.volume_pad, 0.0, "{label}: closed-form faces only");
    assert!(
        // Relative to the result, floored at the operands' unit scale:
        // the divergence sum rounds against the whole spheres' terms.
        (p.volume - expected).abs() <= 1e-9 * expected.max(1.0),
        "{label}: volume {} against the cap closed form {expected}",
        p.volume
    );
}

/// The boolean's body; an empty result or a refusal fails with the
/// payload.
fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let out = match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
    .unwrap_or_else(|e| panic!("{op:?} refused: {e:?}"));
    out.body()
        .unwrap_or_else(|| panic!("{op:?} came back empty"))
        .body
        .clone()
}

const R1: f64 = 1.0;
const R2: f64 = 0.8;
const D: f64 = 1.4;

/// The snowman under ∪, ∩ and ∖ in both orders, each against its
/// closed form.
#[test]
fn the_snowman_builds_under_every_boolean() {
    let (a, b) = (ball(R1, 0.0), ball(R2, D));
    let lens = lens_volume(R1, R2, D);
    let (va, vb) = (ball_volume(R1), ball_volume(R2));
    for (label, op, x, y, expected) in [
        ("A ∪ B", BooleanOp::Union, &a, &b, va + vb - lens),
        ("B ∪ A", BooleanOp::Union, &b, &a, va + vb - lens),
        ("A ∩ B", BooleanOp::Intersect, &a, &b, lens),
        ("B ∩ A", BooleanOp::Intersect, &b, &a, lens),
        ("A ∖ B", BooleanOp::Subtract, &a, &b, va - lens),
        ("B ∖ A", BooleanOp::Subtract, &b, &a, vb - lens),
    ] {
        assert_body(label, &run(op, x, y), expected);
    }
}

/// The boolean's refusal; a body fails with the op named.
fn refusal(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> topo::BooleanError {
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
    .err()
    .unwrap_or_else(|| panic!("{op:?} built a body where the frontier was pinned"))
}

const OPS: [BooleanOp; 3] = [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];

/// **The near-tangent pair**: the same two balls pulled apart until
/// their centres are `r1 + r2 − 1e-4` apart, so the section circle is
/// about two centimetres across and the lens is a sliver. Every op still
/// builds and still meets its closed form.
#[test]
fn a_near_tangent_snowman_builds() {
    let d = R1 + R2 - 1e-4;
    let (a, b) = (ball(R1, 0.0), ball(R2, d));
    let lens = lens_volume(R1, R2, d);
    let (va, vb) = (ball_volume(R1), ball_volume(R2));
    assert_body(
        "near-tangent A ∪ B",
        &run(BooleanOp::Union, &a, &b),
        va + vb - lens,
    );
    assert_body(
        "near-tangent A ∖ B",
        &run(BooleanOp::Subtract, &a, &b),
        va - lens,
    );
    assert_body(
        "near-tangent B ∖ A",
        &run(BooleanOp::Subtract, &b, &a),
        vb - lens,
    );
    assert_body(
        "near-tangent A ∩ B",
        &run(BooleanOp::Intersect, &a, &b),
        lens,
    );
}

/// **The tangent pairs are the honest frontier.** Touching externally
/// (`d = r1 + r2`) or internally (`d = r1 − r2`), the two balls meet at
/// one pole, where a meridian of each touches the other sphere without
/// crossing it. The circle × sphere roots read that as a tangency, which
/// is not a crossing at any order the crossing layer sees, so every op
/// refuses typed at the pierce door rather than guessing a contact.
#[test]
fn a_pole_tangent_pair_refuses_at_the_pierce_door() {
    let a = ball(R1, 0.0);
    for (label, b) in [
        ("external", ball(R2, R1 + R2)),
        ("internal", ball(0.3, R1 - 0.3)),
    ] {
        for op in OPS {
            let e = refusal(op, &a, &b);
            assert!(
                matches!(e, topo::BooleanError::CurvedPierceUnsupported { .. }),
                "{label} tangency under {op:?}: expected the pierce door, got {e:?}"
            );
        }
    }
}

/// **The nested pair**: a small ball strictly inside the big one, off
/// centre, so no edge crosses either sphere. Every op builds — the
/// subtract as a ball with a spherical cavity, the reversed subtract
/// as nothing.
#[test]
fn a_nested_pair_builds_under_every_boolean() {
    let (r, y) = (0.3, 0.2);
    let (a, b) = (ball(R1, 0.0), ball(r, y));
    let (va, vb) = (ball_volume(R1), ball_volume(r));
    for (label, op, x, z, expected) in [
        ("nested A ∪ B", BooleanOp::Union, &a, &b, va),
        ("nested B ∪ A", BooleanOp::Union, &b, &a, va),
        ("nested A ∩ B", BooleanOp::Intersect, &a, &b, vb),
        ("nested B ∩ A", BooleanOp::Intersect, &b, &a, vb),
        ("nested A ∖ B", BooleanOp::Subtract, &a, &b, va - vb),
    ] {
        assert_body(label, &run(op, x, z), expected);
    }
    let out = topo::boolean::subtract(&b, &a, Tol::witness())
        .unwrap_or_else(|e| panic!("nested B ∖ A refused: {e:?}"));
    assert!(out.body().is_none(), "nested B ∖ A is empty");
}

/// **The snowman at the `Interval` scalar**: the same two balls,
/// enclosures throughout, under every op — the circle × sphere roots'
/// `atan2`/`acos` and their rounding meter run on enclosures here, and
/// every body still certifies.
#[test]
fn the_snowman_builds_at_the_interval_scalar() {
    use crate::common::interval::iv;
    use geom_core::Interval;
    let ball_iv = |r: f64, y: f64| -> Body<Interval> {
        sweep::test_support::revolved_about_y_at::<Interval>(
            vec![
                (Point2::new(iv(0.0), iv(y - r)), iv(1.0)),
                (Point2::new(iv(0.0), iv(y + r)), iv(0.0)),
            ],
            Revolution::Full,
            Tol::witness(),
        )
    };
    let (a, b) = (ball_iv(R1, 0.0), ball_iv(R2, D));
    for op in OPS {
        let out = match op {
            BooleanOp::Union => topo::boolean::union(&a, &b, Tol::witness()),
            BooleanOp::Intersect => topo::boolean::intersect(&a, &b, Tol::witness()),
            BooleanOp::Subtract => topo::boolean::subtract(&a, &b, Tol::witness()),
        }
        .unwrap_or_else(|e| panic!("Interval {op:?} refused: {e:?}"));
        let body = &out
            .body()
            .unwrap_or_else(|| panic!("Interval {op:?} came back empty"))
            .body;
        assert_eq!(topo::validate(body), Ok(()), "Interval {op:?}: validate");
        assert_eq!(
            topo::validate_geometric(body, Tol::witness()),
            Ok(()),
            "Interval {op:?}: validate_geometric"
        );
    }
}

/// **A straight edge through a ball** reaches the line × sphere roots
/// through a public op: a square bar poking out of a ball, its long
/// edges straddling the sphere. They pierce, and the op goes on to the
/// pierce point's sector side, where the curved-sector sagitta charge
/// stops it (`work/reach/slab-cut-cylinder-refuses-sector-side.md`). A
/// refusal at the pierce door would mean the root lane went dark.
#[test]
fn a_bar_through_a_ball_crosses_the_sphere() {
    let a = ball(R1, 0.0);
    let bar =
        topo::test_support::brick::<f64>((0.5, 2.0), (-0.3, 0.3), (-0.3, 0.3), Tol::witness());
    for op in OPS {
        let e = refusal(op, &a, &bar);
        assert!(
            matches!(e, topo::BooleanError::CurvedSectorSideUnsupported { .. }),
            "bar through a ball under {op:?}: expected to cross the sphere and stop at the \
             sector side, got {e:?}"
        );
    }
}

/// **The waist fillets through the sphere × sphere blend arm.** The
/// union's waist is a concave crease between two spheres on distinct
/// centres — the configuration `BlendArm::SphereSphereTorus` names —
/// and the ball rolling in it stays outside both, at `Rᵢ + r` from each
/// centre. Its spine is level at `y = ((R1+r)² − (R2+r)² + D²)/2D` and
/// has radius `√((R1+r)² − y²)`, derived here from the fixture.
#[test]
fn the_snowman_waist_fillets() {
    let r = 0.1;
    let body = run(BooleanOp::Union, &ball(R1, 0.0), &ball(R2, D));
    let x = (D.powi(2) + R1.powi(2) - R2.powi(2)) / (2.0 * D);
    let waist = sweep::test_support::rim_arcs_at(&body, (R1.powi(2) - x.powi(2)).sqrt(), x);
    assert!(!waist.is_empty(), "the union has a waist rim");
    let req = sweep::blend::battery::BlendRequest {
        body: &body,
        edges: waist.clone(),
        size: r,
    };
    let verdict = sweep::blend::battery::run_battery(&req, Band::linear(Tol::witness()).unwrap())
        .unwrap_or_else(|e| panic!("the battery passes the waist: {e:?}"));
    let arms: Vec<_> = verdict
        .chains
        .iter()
        .flat_map(|c| c.links().map(|l| l.arm))
        .collect();
    assert!(
        !arms.is_empty()
            && arms
                .iter()
                .all(|a| *a == sweep::blend::BlendArm::SphereSphereTorus),
        "every waist link takes the sphere × sphere arm, got {arms:?}"
    );
    let out = sweep::blend::build::fillet_edges(&body, &waist, r, Tol::witness())
        .unwrap_or_else(|e| panic!("the waist fillets, got {e:?}"));
    assert_eq!(
        topo::validate_geometric(&out.body, Tol::witness()),
        Ok(()),
        "the filleted snowman: validate_geometric"
    );
    let y = ((R1 + r).powi(2) - (R2 + r).powi(2) + D.powi(2)) / (2.0 * D);
    let rho = ((R1 + r).powi(2) - y.powi(2)).sqrt();
    assert!(!out.band_faces.is_empty(), "the fillet mints a band");
    for &f in &out.band_faces {
        match *out
            .body
            .get_surface(out.body.get_face(f).unwrap().surface)
            .unwrap()
        {
            geom::Surface::Torus {
                center,
                major_radius,
                minor_radius,
                ..
            } => {
                assert!(
                    (center.y - y).abs() < 1e-12,
                    "spine height {y}, got {}",
                    center.y
                );
                assert!(
                    (major_radius - rho).abs() < 1e-12,
                    "spine radius {rho}, got {major_radius}"
                );
                assert!(
                    (minor_radius - r).abs() < 1e-15,
                    "tube radius {r}, got {minor_radius}"
                );
            }
            ref other => panic!("the band is a torus, got {other:?}"),
        }
    }
}
