//! Review probes for PR 5772 (`flush/nary-union-door`), not for merge.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used, missing_docs)]

use geom_core::{Point2, RigidMotion, Tol, Turns, UnitVec3};
use profile::SketchPlane;
use profile::test_support::bulge_loop;
use sweep::test_support::{brick, finished};
use topo::AtRestBody;

fn tol() -> Tol {
    Tol::witness()
}

fn circle(cx: f64, cy: f64, r: f64) -> profile::ProfileLoop<f64> {
    let at = |t: f64| Point2::new(cx + r * t.cos(), cy + r * t.sin());
    bulge_loop(vec![(at(0.0), 1.0), (at(core::f64::consts::PI), 1.0)])
}

/// A rod along +x from `x0`, of length `len`, radius 0.5, its axis at
/// y = 2.5 and height `z`: tangent to the plane y = 2 along that height.
fn rod_along_x(x0: f64, len: f64, z: f64) -> AtRestBody<f64> {
    // A quarter turn about y maps the sketch's (u, v) to (0, v, -u) and
    // its normal to +x.
    let plane = SketchPlane::from_frame(RigidMotion::turned(
        geom_core::Point3::new(x0, 0.0, 0.0),
        UnitVec3::exact_y(),
        Turns::quarters(1),
    ));
    let body = sweep::test_support::extruded(plane, vec![circle(-z, 2.5, 0.5)], len, tol());
    finished("rod", body, tol())
}

fn nary(
    op: topo::NaryOp,
    bodies: &[&AtRestBody<f64>],
    ranks: &[u32],
) -> Result<topo::NaryResult<f64>, topo::NaryRefusal> {
    let inputs: Vec<topo::MemberInputs> = ranks
        .iter()
        .map(|&rank| topo::MemberInputs {
            rank,
            ..topo::MemberInputs::default()
        })
        .collect();
    let members: Vec<topo::NaryMember<'_, f64>> = bodies
        .iter()
        .zip(&inputs)
        .map(|(&body, inputs)| topo::NaryMember { body, inputs })
        .collect();
    topo::nary_boolean(op, &members, topo::SweepStrategy::Realized, tol())
}

/// CONTACTHOLD m2: two plates overlapping in z whose y = 2 faces are
/// coplanar and overlap in area (the second raised by `dy` inside the
/// band), so their union merges them, and a rod along x tangent to the
/// merged face along z = 0.75, through both faces' interiors and
/// crossing no edge of either. The step joining the rod is handed one
/// given pair for (merged face, rod wall), under the first verdict; its
/// restated row's margin is that verdict's alone, while `restates`
/// names both judged rows.
#[test]
fn m2_two_verdicts_on_one_merged_face() {
    for dy in [0.0, 1e-11, -1e-11] {
        let p1 = finished(
            "p1",
            brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
            tol(),
        );
        let p2 = finished(
            "p2",
            brick::<f64>((-2.0, 2.0), (-2.0, 2.0 + dy), (0.5, 1.5), tol()),
            tol(),
        );
        let rod = rod_along_x(-1.0, 2.0, 0.75);
        let built = match nary(topo::NaryOp::Union, &[&p1, &p2, &rod], &[0, 1, 2]) {
            Ok(topo::NaryResult::Body(b)) => b,
            Ok(topo::NaryResult::Empty) => panic!("dy {dy}: empty union"),
            Err(e) => {
                eprintln!("M2 dy {dy}: REFUSED {e}");
                continue;
            }
        };
        for (j, judged) in built.judgements.iter().enumerate() {
            for (k, row) in judged.body.coincidences.iter().enumerate() {
                if row.face_pair().is_some() {
                    eprintln!(
                        "M2 dy {dy}: judgement {j} ({}, {}) row {k}: {:?} margin {:?}",
                        judged.a, judged.b, row.relation, row.margin
                    );
                }
            }
        }
        for (s, step) in built.steps.iter().enumerate() {
            for (k, row) in step.body.coincidences.iter().enumerate() {
                if row.face_pair().is_some() {
                    eprintln!(
                        "M2 dy {dy}: step {s} row {k}: {:?} margin {:?} restates {:?}",
                        row.relation,
                        row.margin,
                        step.restates(k)
                    );
                }
            }
            eprintln!(
                "M2 dy {dy}: step {s} ff records {}",
                step.body.contacts.face_face().count()
            );
        }
    }
}
