//! **A rod square to a torus's axis: the section certificate's
//! square-wall arm at the operation level.**
//!
//! The donut (`R = 2`, `r = 0.5`, about `y`) against a rod along `x`.
//! Where the rod's wall meets the torus nowhere inside either face —
//! a rod buried in the tube, a rod threading the hole — no crossing
//! layer sees an event, and the fallback's section pass asks the
//! certificate about every torus × wall pair whose boxes overlap. The
//! pose had no arm, so each such pair refused on reach and every op
//! refused with it; the square-wall arm classifies the section (four
//! loops about the rod, essential on its wall) and the ops answer in
//! closed form.
//!
//! A rod that does cross the tube still stops, one wall later, at the
//! join's germ frame: the certificate answers every pair, and the frame
//! of a torus × cylinder germ has no arm
//! (`work/sect/torus-germ-pairs-have-no-section-frame.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;
use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use revolve_common::{axis_y, validated};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Revolution, revolve};
use topo::{AtRestBody, Body, BooleanOp as Op, BooleanResultKind as Kind};

use core::f64::consts::PI;

fn donut() -> AtRestBody<f64> {
    let vp = validated(vec![revolve_common::donut_profile()]);
    let donut = revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .expect("the donut revolves")
        .body;
    finished("the donut", donut, Tol::witness())
}

/// The donut's volume, `2π²Rr²`.
const DONUT: f64 = PI * PI;

/// A rod along `x` from `x0` to `x1`, its axis at `(y, z) = (h, e)`,
/// radius `rc`.
fn rod(x: (f64, f64), (h, e): (f64, f64), rc: f64) -> AtRestBody<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(h - rc, e), 1.0),
        (Point2::new(h + rc, e), 1.0),
    ]);
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_y(), Vec3::unit_z(), Vec3::unit_x()),
        Vec3::new(x.0, 0.0, 0.0),
    ));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("the rod profile validates");
    let b = sweep::extrude(
        &vp,
        sweep::Extrusion::Distance {
            depth: x.1 - x.0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .expect("the rod extrudes")
    .body;
    finished("the rod", b, Tol::witness())
}

fn volume(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, Tol::witness())
        .expect("the volume integrates")
        .volume
}

fn in_solid(b: &Body<f64>, q: Point3<f64>) -> Option<bool> {
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's band");
    match topo::point_in_solid(b, q, band, Tol::witness()) {
        Ok(topo::SolidContainment::In) => Some(true),
        Ok(topo::SolidContainment::Out) => Some(false),
        _ => None,
    }
}

/// Each op's answer: its kind and volume (`None` for empty), and
/// whether each probe point lies inside it.
struct Want {
    op: Op,
    kind: Option<Kind>,
    volume: f64,
    inside: [bool; 3],
}

/// Runs every op on `donut ∘ rod` against its closed form, tier 3 and
/// `point_in_solid` at `probes`.
fn answers(
    what: &str,
    d: &AtRestBody<f64>,
    r: &AtRestBody<f64>,
    probes: [Point3<f64>; 3],
    want: [Want; 3],
) {
    for w in want {
        let res = match w.op {
            Op::Union => topo::union(d, r, Tol::witness()),
            Op::Subtract => topo::subtract(d, r, Tol::witness()),
            Op::Intersect => topo::intersect(d, r, Tol::witness()),
        };
        let res = res.unwrap_or_else(|e| panic!("{what}, {:?}: {e:?}", w.op));
        let Some(kind) = w.kind else {
            assert!(
                matches!(res, topo::BooleanResult::Empty),
                "{what}, {:?}: not empty",
                w.op
            );
            continue;
        };
        let b = res.body().expect("non-empty");
        assert_eq!(b.kind, kind, "{what}, {:?}", w.op);
        assert_eq!(
            topo::validate_geometric(&b.body, Tol::witness()),
            Ok(()),
            "{what}, {:?}: tier 3",
            w.op
        );
        let got = volume(&b.body);
        assert!(
            (got - w.volume).abs() <= 1e-9 * w.volume.max(1.0),
            "{what}, {:?}: volume {got} against the closed form {}",
            w.op,
            w.volume
        );
        for (q, inside) in probes.into_iter().zip(w.inside) {
            assert_eq!(
                in_solid(&b.body, q),
                Some(inside),
                "{what}, {:?} at {q:?}",
                w.op
            );
        }
    }
}

/// **A rod buried in the tube**, on the tube's core or off it: `x` from
/// `1.7` to `2.3`, radius `0.1`. ∪ is the donut, ∩ the rod, and ∖ the
/// donut with a void (two shells), `π² − 0.006π`. Probes: in the rod, in
/// the tube beside it, outside the torus. Refused on reach before the
/// square-wall arm; it is the one wall in the way. Mutant: drop the arm
/// (the pose back to R-reach), and every op refuses.
#[test]
fn a_rod_buried_in_the_tube_answers_every_op() {
    let d = donut();
    let rod_v = PI * 0.01 * 0.6;
    for (what, axis) in [("on the core", (0.0, 0.0)), ("off the core", (0.1, 0.15))] {
        let r = rod((1.7, 2.3), axis, 0.1);
        let probes = [
            Point3::new(2.0, axis.0, axis.1),
            Point3::new(2.0, axis.0 - 0.3, axis.1),
            Point3::new(0.0, 0.0, 0.0),
        ];
        answers(
            what,
            &d,
            &r,
            probes,
            [
                Want {
                    op: Op::Union,
                    kind: Some(Kind::OperandA),
                    volume: DONUT,
                    inside: [true, true, false],
                },
                Want {
                    op: Op::Subtract,
                    kind: Some(Kind::Voided),
                    volume: DONUT - rod_v,
                    inside: [false, true, false],
                },
                Want {
                    op: Op::Intersect,
                    kind: Some(Kind::OperandB),
                    volume: rod_v,
                    inside: [true, false, false],
                },
            ],
        );
    }
}

/// **A rod threading the hole**, `x` from `−1` to `1`, radius `0.3`, its
/// wall's carrier crossing the tube only past its ends. ∪ is the
/// assembly of the two (`π² + 0.18π`), ∖ the donut, ∩ empty. Probes: in
/// the rod, in the tube, in neither. Refused on reach before the
/// square-wall arm. Mutant: as above.
#[test]
fn a_rod_threading_the_hole_answers_every_op() {
    let (d, r) = (donut(), rod((-1.0, 1.0), (0.0, 0.0), 0.3));
    let rod_v = PI * 0.09 * 2.0;
    answers(
        "threading the hole",
        &d,
        &r,
        [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(0.0, 0.0, 1.0),
        ],
        [
            Want {
                op: Op::Union,
                kind: Some(Kind::Assembly),
                volume: DONUT + rod_v,
                inside: [true, true, false],
            },
            Want {
                op: Op::Subtract,
                kind: Some(Kind::OperandA),
                volume: DONUT,
                inside: [false, true, false],
            },
            Want {
                op: Op::Intersect,
                kind: None,
                volume: 0.0,
                inside: [false; 3],
            },
        ],
    );
}

/// **A radial hole through the tube stops at the germ frame, with every
/// certificate pair answered.** Through both walls, through the inner
/// side only, through the outer side only, and off the axis: the section
/// report holds no refusal (each torus × wall pair was R-reach before
/// the arm), and ∖ refuses at the join, naming the torus and the rod's
/// wall. Mutant: drop the arm, and the report carries `Err(Reach)`.
#[test]
fn a_radial_hole_through_the_tube_stops_at_the_germ_frame() {
    let d = donut();
    for (what, r) in [
        ("through both walls", rod((1.0, 3.0), (0.0, 0.0), 0.2)),
        ("through the inner side", rod((1.0, 2.0), (0.0, 0.0), 0.2)),
        ("through the outer side", rod((2.0, 3.0), (0.0, 0.0), 0.2)),
        ("off the axis", rod((1.0, 3.0), (0.1, 0.15), 0.2)),
    ] {
        let report = topo::test_support::section_report(Op::Subtract, &d, &r, Tol::witness())
            .expect("the reduction runs");
        assert!(!report.is_empty(), "{what}: the certificate is asked");
        for (_, _, v) in &report {
            assert!(v.starts_with("Ok("), "{what}: {v}");
        }
        match topo::subtract(&d, &r, Tol::witness()) {
            Err(topo::BooleanError::GermFrameUnsupported { a_kind, b_kind, .. }) => {
                assert_eq!(
                    (a_kind, b_kind),
                    (geom::SurfaceKind::Torus, geom::SurfaceKind::Cylinder),
                    "{what}"
                );
            }
            other => panic!("{what}: {:?}", other.map(|r| r.body().map(|b| b.kind))),
        }
    }
}
