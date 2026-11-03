//! **A rim circle crossing a cylinder wall**, end to end: the poses the
//! conic × quadric root door (`topo::boolean::conic_quadric`)
//! settles, and the door each one reaches next.
//!
//! - **A crossing of the CARRIER outside the wall's trim builds.** A
//!   D-shaped prism beside a cylinder, its half-round wall turned away:
//!   the cylinder's rim circles cross the prism's wall carrier on the
//!   side the wall face does not cover, and the certified roots place
//!   both crossings outside its trim — no event, where the crossing
//!   layer used to keep its pierce door. Square to the axes (the door's
//!   first-harmonic arm) and with the prism tilted (its half-angle
//!   ladder), every boolean builds and meters at the closed form.
//! - **A genuine pierce reaches the cylinder pair's join.** Two
//!   parallel equal-radius cylinders staggered in height: each rim
//!   circle pierces the other wall, the lane certifies where, the
//!   pierce's sector side certifies, and the join refuses the wall ×
//!   wall germ pair it has no chord lane for
//!   (`work/tang/cylinder-pair-germ-has-no-join-arm.md`).
//!
//! #347's own poses — one height, coaxial, Steinmetz — are pinned with
//! their doors in `verbs_cylcyl_probe.rs` and `verbs_germarms2.rs`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use sweep::ExtrudeSide;

use crate::common::crossed_cylinders::{self as crossed, Cylinder, Op};
use crate::common::differential::{VOLUME_CLAIM, judged_within, resolution_pad};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError, BooleanOp, SweepStrategy};

/// The cylinder of radius `r` about `(cx, cy)`, `z ∈ [z0, z1]`, through
/// the public circle and extrude doors.
fn cyl(cx: f64, cy: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(cx, cy), r, tol).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body
}

/// The D-shaped prism: the half disc of radius `r` about `(d, 0)` on
/// the `+x` side of its flat `x = d`, `z ∈ [z0, z1]`.
fn d_prism(d: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = bulge_loop(vec![(Point2::new(d, -r), 1.0), (Point2::new(d, r), 0.0)]);
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body
}

/// `b` turned by `deg` about the line through `(0, 0, 1)` along `x`.
fn tilt(b: &Body<f64>, deg: f64) -> Body<f64> {
    let turn = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        deg.to_radians(),
    );
    topo::transform_rigid(
        b,
        &geom_core::test_support::rigid(&turn, Tol::witness()),
        Tol::witness(),
    )
    .unwrap()
}

/// The area of the unit disc's segment beyond the chord at `x = a`.
fn segment(a: f64) -> f64 {
    a.acos() - a * (1.0 - a * a).sqrt()
}

fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<Body<f64>, BooleanError> {
    let tol = Tol::witness();
    let (a, b) = (
        &finished("operand A", a.clone(), tol),
        &finished("operand B", b.clone(), tol),
    );
    let out = match op {
        BooleanOp::Union => topo::union(a, b, tol),
        BooleanOp::Intersect => topo::intersect(a, b, tol),
        BooleanOp::Subtract => topo::subtract(a, b, tol),
    }?;
    Ok(out
        .body()
        .unwrap_or_else(|| panic!("{op:?} came back empty"))
        .body
        .clone()
        .into_body())
}

/// The datum a refusal past the crossing layer does not carry: how many
/// (edge, face) pairs the sweep accepted an event on with a CIRCLE
/// carrier against a CYLINDER face, in `a`'s edges against `b`'s faces
/// and in `b`'s against `a`'s — the circle × cylinder cell, firing.
fn circle_wall_events(a: &Body<f64>, b: &Body<f64>) -> (usize, usize) {
    let (ab, ba) = topo::sweep_traces(a, b, SweepStrategy::Realized, None, Tol::witness())
        .unwrap_or_else(|e| panic!("the sweep refused: {e:?}"));
    let count =
        |x: &Body<f64>, y: &Body<f64>, trace: &topo::SweepPairs<(topo::EdgeKey, topo::FaceKey)>| {
            trace
                .accepted()
                .iter()
                .filter(|(e, f)| {
                    let circle = x
                        .get_edge(*e)
                        .and_then(|e| x.get_curve_geom(e.curve))
                        .and_then(topo::CurveGeom::certified)
                        .is_some_and(|c| matches!(c.carrier(), topo::Curve3::Circle(..)));
                    let wall = matches!(
                        y.get_face(*f).and_then(|f| y.get_surface(f.surface)),
                        Some(topo::Surface::Cylinder { .. })
                    );
                    circle && wall
                })
                .count()
        };
    (count(a, b, &ab), count(b, a, &ba))
}

/// Every validation tier, then the volume against `expected`.
fn assert_body(label: &str, body: &Body<f64>, expected: f64) {
    let tol = Tol::witness();
    assert_eq!(topo::validate(body), Ok(()), "{label}: validate");
    assert_eq!(
        topo::validate_closed(body),
        Ok(()),
        "{label}: validate_closed"
    );
    assert_eq!(
        topo::validate_geometric(body, tol),
        Ok(()),
        "{label}: validate_geometric"
    );
    let v = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(body), tol),
        tol,
    )
    .volume;
    assert!(
        (v - expected).abs() <= 1e-9 * expected.max(1.0),
        "{label}: volume {v} against the closed form {expected}"
    );
}

/// **The D-prism beside the cylinder builds under every boolean.** The
/// unit cylinder `A` (`z ∈ [0, 2]`) against a D-prism whose flat
/// `x = d` cuts it and whose half-round wall stands clear of it, tall
/// enough that `A`'s whole segment beyond the flat lies inside the
/// prism: the common part is that segment times `A`'s height, whatever
/// the prism's radius and tilt (each pose is chosen so; the tilts turn
/// the prism about a line along `x`, which keeps its flat in `x = d`).
///
/// `A`'s rim circles cross the prism's wall CARRIER at `x < d`, beside
/// the flat, which the wall face does not cover: the root lane places
/// both crossings outside its trim. Square to the axes that is the
/// door's first-harmonic arm, tilted its half-angle quartic.
#[test]
fn a_d_prism_beside_a_cylinder_builds_under_every_boolean() {
    let a = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let va = 2.0 * PI;
    let mut poses = Vec::new();
    for d in [0.2, 0.5, 0.8] {
        let vb = PI / 2.0 * 3.0;
        poses.push((format!("square, d {d}"), d_prism(d, 1.0, -0.5, 2.5), d, vb));
    }
    for (r, deg) in [(1.2, 10.0), (1.4, 30.0), (1.4, 45.0)] {
        let b = tilt(&d_prism(0.5, r, -1.5, 3.5), deg);
        let vb = PI * r * r / 2.0 * 5.0;
        poses.push((format!("tilted {deg}°, r {r}"), b, 0.5, vb));
    }
    for (label, b, d, vb) in &poses {
        let common = segment(*d) * 2.0;
        for (op, expected) in [
            (BooleanOp::Union, va + vb - common),
            (BooleanOp::Subtract, va - common),
            (BooleanOp::Intersect, common),
        ] {
            let label = format!("{label}, {op:?}");
            let body = run(op, &a, b).unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
            assert_body(&label, &body, expected);
        }
    }
}

/// **Two parallel equal-radius cylinders that pierce build at the closed
/// form.** Staggered in height, each rim circle crosses the other wall
/// inside its trim: a pierce, certified by the root lane, whose sector
/// side certifies, at every offset from deep overlap to a thin lens. The
/// section's germ pair is then two walls, which the join splits along
/// their rulings. The trace is asked which events the sweep took: each
/// operand's rim circles on the other's wall. The solids share the lens
/// of two unit discs `d` apart over 1.5 of height.
#[test]
fn parallel_cylinders_that_pierce_build_at_the_closed_form() {
    let a = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let (va, vb) = (2.0 * PI, 2.0 * PI);
    for d in [0.3, 0.8, 1.2, 1.6, 1.9] {
        let b = cyl(d, 0.0, 1.0, 0.5, 2.5);
        let (ab, ba) = circle_wall_events(&a, &b);
        assert!(
            ab > 0 && ba > 0,
            "d {d}: each rim circle meets the other wall: {ab} + {ba}"
        );
        let common = 2.0 * segment(d / 2.0) * 1.5;
        for (op, x, y, expected) in [
            (BooleanOp::Union, &a, &b, va + vb - common),
            (BooleanOp::Subtract, &a, &b, va - common),
            (BooleanOp::Subtract, &b, &a, vb - common),
            (BooleanOp::Intersect, &a, &b, common),
        ] {
            let label = format!("d {d}, {op:?}");
            let body = run(op, x, y).unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
            assert_body(&label, &body, expected);
        }
    }
}

/// **A tilted rod through a rim takes the half-angle arm and builds at
/// its sliced volume.** A thin rod tilted 30° off the cylinder's axis
/// passes through its top rim: the rim circle against the rod's wall is
/// a degree-2 residual the ladder certifies, the pierce's sector side
/// certifies, and the two walls' axes are skew, a pair the join reads on
/// its section value and carries, marched; every operation builds, its
/// spline-chorded walls measured by the rotational lane, at the crossed
/// cylinders' sliced volume (`common::crossed_cylinders`). ∖ and ∩ are
/// SOUND; ∪'s tier 3′ refuses at the census, whose edge-in-curved-face
/// lane reads the rod's ruling inside a chorded cylinder face through a
/// containment door that does not read its trim
/// (`work/inside/the-census-reads-no-ruling-inside-a-chorded-cylinder-face.md`),
/// and its volume is held to the oracle all the same.
///
/// The enclosure is read against [`resolution_pad`] over the operands'
/// area: each chorded wall's rule accepts its first round inside the
/// pinned resolution, so the body's half-width (`1.9e-7` at ε `1e-9`)
/// is past the shared budget.
#[test]
fn a_tilted_rod_through_a_rim_builds_at_its_sliced_volume() {
    let tol = Tol::witness();
    let a = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let turn = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        PI / 6.0,
    );
    let shift = Vec3::new(0.95, 0.1, 2.0);
    let rod = topo::transform_rigid(
        &cyl(0.0, 0.0, 0.3, -1.5, 1.5),
        &geom_core::test_support::rigid(&turn, tol),
        tol,
    )
    .unwrap();
    let rod = topo::transform_rigid(
        &rod,
        &geom_core::RigidMotion::translation(shift).unwrap(),
        tol,
    )
    .unwrap();
    let (ab, _) = circle_wall_events(&a, &rod);
    assert!(ab > 0, "the cylinder's rim circle meets the rod's wall");
    let truth = |op| {
        crossed::volume(
            Cylinder::upright(0.0, 0.0, 1.0, (0.0, 2.0)),
            Cylinder::upright(0.0, 0.0, 0.3, (-1.5, 1.5))
                .under(turn)
                .under(Affine3::translation(shift)),
            op,
        )
    };
    let (a, rod) = (
        finished("operand A", a, tol),
        finished("operand B", rod, tol),
    );
    // The operands' area together: the drum's `6π`, the rod's
    // `2π·0.3·3 + 2π·0.09`.
    let budget = resolution_pad(tol, 6.0 * PI + 2.0 * PI * (0.9 + 0.09));
    for (label, op, r) in [
        ("∖", Op::Subtract, topo::subtract(&a, &rod, tol)),
        ("∩", Op::Intersect, topo::intersect(&a, &rod, tol)),
    ] {
        let line = judged_within(r, Some(truth(op)), tol, budget).line;
        assert!(line.starts_with("OK SOUND"), "{label}: {line}");
    }
    let union = topo::union(&a, &rod, tol);
    let census = union.as_ref().ok().and_then(|r| r.body()).map(|bb| {
        matches!(
            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
                .as_ref()
                .map_err(Vec::as_slice),
            Err([topo::ValidationError::CensusUnsupported {
                cause: topo::CensusUnsupportedCause::ContactLane(
                    topo::ContactRefusal::NotCertifiable { what }
                ),
                ..
            }]) if what.starts_with("a line edge on a curved face's ruling")
        )
    });
    let read = judged_within(union, Some(truth(Op::Union)), tol, budget);
    let held = read.volume.zip(read.pad).is_some_and(|(v, pad)| {
        (v - truth(Op::Union)).abs() <= VOLUME_CLAIM + pad && pad <= budget
    });
    assert!(
        census == Some(true)
            && held
            && read
                .line
                .contains("t2=true t3p=false cert=true operand=true"),
        "∪: tier 3′ refuses at the census and nothing else, the volume holds: {}",
        read.line
    );
}

/// **A rim circle on a parallel wall tangent to its own, or crossing or
/// clearing it by less than the zero band, is a tangency.**
/// Two parallel unit cylinders `2 + δ` apart, staggered, `|δ|` at most
/// half the zero band: each rim circle's extreme residual against the
/// other wall is `δ`, and the two walls touch along a ruling within the
/// zero band. The boolean verifies that tangency by its witness, and
/// every op answers as for two cylinders touching from outside: the
/// union holds both, each difference is its first operand, and the
/// intersection is empty. The half-angle ladder, which does not decide
/// on the residual's range and can certify an in-band configuration as
/// a miss (`work/germ/the-half-angle-ladder-certifies-in-band-configurations.md`),
/// is not what answers.
#[test]
fn a_rim_circle_within_the_band_of_a_parallel_wall_is_a_tangency() {
    let tol = Tol::witness();
    let half_zero = geom_core::Band::linear(tol).unwrap().zero() / 2.0;
    let a = finished("operand A", cyl(0.0, 0.0, 1.0, 0.0, 2.0), tol);
    for delta in [0.0, -half_zero, half_zero] {
        let b = finished("operand B", cyl(2.0 + delta, 0.0, 1.0, 0.5, 2.5), tol);
        for (label, got, want) in [
            ("A ∪ B", topo::union(&a, &b, tol), Some(4.0 * PI)),
            ("A ∖ B", topo::subtract(&a, &b, tol), Some(2.0 * PI)),
            ("B ∖ A", topo::subtract(&b, &a, tol), Some(2.0 * PI)),
            ("A ∩ B", topo::intersect(&a, &b, tol), None),
        ] {
            let label = format!("δ {delta}, {label}");
            match (got, want) {
                (Ok(topo::BooleanResult::Body(r)), Some(v)) => {
                    assert_body(&label, &r.body.clone().into_body(), v);
                }
                (Ok(topo::BooleanResult::Empty), None) => {}
                (got, _) => panic!("{label}: {got:?}"),
            }
        }
    }
}
