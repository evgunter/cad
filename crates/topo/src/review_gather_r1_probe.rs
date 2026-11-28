//! Reviewer probe (PR 7723): whether the certificate a gather carries
//! holds its runs in the aggregate's face-arena order, the order
//! `SignCertificate::assembled` states every claim on the type in and
//! `KeptSign::gathered`'s doc says the parts' order is.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{RigidMotion, Tol, Vec3};

use crate::test_support_fixtures::brick;
use crate::{AtRestBody, BooleanResult, FaceKey, Placed};

fn fin(body: crate::Body<f64>) -> AtRestBody<f64> {
    crate::finish(body, Tol::witness()).expect("the fixture finishes")
}

fn b(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> crate::Body<f64> {
    brick::<f64>(x, y, z, Tol::witness())
}

fn boolean(sub: bool, a: crate::Body<f64>, c: crate::Body<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let r = if sub {
        crate::subtract(&fin(a), &fin(c), tol)
    } else {
        crate::union(&fin(a), &fin(c), tol)
    };
    match r {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("the boolean builds: {:?}", other.map(|_| ())),
    }
}

#[test]
fn a_gathers_carried_runs_stand_in_the_aggregates_face_arena_order() {
    let tol = Tol::witness();
    let unit = || b((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let members = [
        ("brick", fin(unit())),
        (
            "union",
            boolean(false, unit(), b((0.5, 1.5), (0.25, 0.75), (0.25, 1.5))),
        ),
        (
            "cavity",
            boolean(true, unit(), b((0.25, 0.75), (0.25, 0.75), (0.25, 0.75))),
        ),
        (
            "two solids",
            boolean(true, unit(), b((0.4, 0.6), (-1.0, 2.0), (-1.0, 2.0))),
        ),
    ];
    let shift = RigidMotion::translation(Vec3::new(3.0, 0.0, 0.0)).unwrap();
    let mut out_of_order = Vec::new();
    for (kind, member) in &members {
        let own: Vec<FaceKey> = member.faces().map(|(k, _)| k).collect();
        if member.probe_run_faces() != own {
            out_of_order.push(format!("{kind}: the member's own finish"));
        }
        for (first, _) in &members {
            let first_body = &members.iter().find(|(k, _)| k == first).unwrap().1;
            let (gathered, _) = crate::boolean::gather_finished(
                &[Placed::own(first_body), Placed::at(member, shift)],
                tol,
            )
            .expect("the gather finishes");
            let arena: Vec<FaceKey> = gathered.faces().map(|(k, _)| k).collect();
            let runs = gathered.probe_run_faces();
            let what = format!("{first} then {kind}");
            if runs.len() != arena.len() {
                out_of_order.push(format!("{what}: {} runs over {} faces", runs.len(), arena.len()));
            } else if runs != arena {
                let at = runs.iter().zip(&arena).position(|(r, a)| r != a).unwrap();
                out_of_order.push(format!("{what}: first differs at run {at} of {}", runs.len()));
            }
            let derived = crate::finish(gathered.into_body(), tol).expect("derives too");
            let d_arena: Vec<FaceKey> = derived.faces().map(|(k, _)| k).collect();
            assert_eq!(derived.probe_run_faces(), d_arena, "{what}: the derived finish is in arena order");
        }
    }
    assert!(out_of_order.is_empty(), "{out_of_order:#?}");
}
