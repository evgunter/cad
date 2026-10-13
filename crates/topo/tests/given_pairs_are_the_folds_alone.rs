//! **A given pair is a verdict a boolean decided.** A caller cannot
//! state one (`GivenPair`'s `compile_fail` doc-test); it reads one off
//! a result ([`topo::BooleanBody::verdicts`]) and hands it to a step
//! that holds the judged faces.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use common::{brick, finished};
use geom_core::Tol;
use topo::{Body, BooleanInputs, BooleanOp, BooleanResult, FaceKey, Relation, Surface};

fn tol() -> Tol {
    Tol::witness()
}

fn face_at_z(body: &Body<f64>, z: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.z.abs() == 1.0 && origin.z == z
            )
        })
        .map(|(k, _)| k)
        .expect("a face on z")
}

/// A brick resting on a cube's top: the union's one verdict is the
/// touch it decided, and handed back to the same two operands it glues
/// that pair and decides nothing else.
#[test]
fn a_rest_touch_is_the_unions_one_verdict_and_glues_as_given() {
    let a = finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let b = finished(
        "b",
        brick((0.25, 0.75), (0.25, 0.75), (1.0, 2.0), tol()),
        tol(),
    );
    let run = |inputs: &BooleanInputs| match topo::boolean_op_with(
        BooleanOp::Union,
        &a,
        &b,
        inputs,
        topo::SweepStrategy::Realized,
        tol(),
    )
    .unwrap()
    {
        BooleanResult::Body(out) => out,
        BooleanResult::Empty => panic!("a union of two bodies is not empty"),
    };
    let decided = run(&BooleanInputs::default());
    let [verdict] = decided.verdicts[..] else {
        panic!("one verdict, the touch: {:?}", decided.verdicts);
    };
    assert_eq!(
        (verdict.a(), verdict.b(), verdict.relation()),
        (
            face_at_z(&a, 1.0),
            face_at_z(&b, 1.0),
            Relation::SameOpposite
        ),
        "the verdict is the cube's top against the brick's bottom, opposed"
    );
    let given = run(&BooleanInputs {
        given: Some(vec![verdict.given_at(verdict.a())]),
        ..BooleanInputs::default()
    });
    assert!(
        given.coincidences.is_empty() && given.verdicts.is_empty(),
        "a given step decides no pair of its own: {:?}",
        given.coincidences
    );
    let volume =
        |out: &topo::BooleanBody<f64>| topo::mass_properties(&out.body, tol()).unwrap().volume;
    assert_eq!(
        (volume(&given), given.body.faces().count()),
        (volume(&decided), decided.body.faces().count()),
        "the given glue builds the decided body"
    );
}
