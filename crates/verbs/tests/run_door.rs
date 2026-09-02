//! **The dispatch adds nothing**: `Verb::run` and the op door it calls
//! produce the same body, the same birth record and the same refusal.
//!
//! The point of the crate is that a caller can address an operation by
//! its NAME and get exactly what the door gives. These rows are what
//! makes that checkable: they run both paths on the same fixture and
//! compare bit-for-bit, so a dispatch that reordered arguments, dropped
//! a parameter or re-derived a band would red here rather than showing
//! up as drifted geometry three layers up.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use geom_core::Tol;
use sweep::blend::build::{chamfer_edges, fillet_edges};
use topo::Body;
use verbs::{Verb, VerbError};

fn tol() -> Tol {
    Tol::witness()
}

/// Every vertex point's BITS, plus the entity census — enough that a
/// carve differing anywhere in position or structure differs here.
fn dump(body: &Body<f64>) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "V={} E={} F={}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count()
    );
    for (k, _) in body.vertices() {
        let p = body
            .get_vertex(k)
            .and_then(|v| body.get_point(v.point))
            .unwrap();
        let _ = writeln!(
            s,
            "{k:?} {:016x} {:016x} {:016x}",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits()
        );
    }
    s
}

#[test]
fn the_fillet_dispatch_is_the_fillet_door() {
    let cube = sweep::test_support::cube(1.0, tol());
    let edges: Vec<_> = cube.edges().map(|(k, _)| k).collect();

    let door = fillet_edges(&cube, &edges, 0.1, tol()).unwrap();
    let via = Verb::Fillet {
        edges: edges.clone(),
        radius: 0.1,
    }
    .run(&cube, tol())
    .unwrap();

    assert_eq!(dump(&door.body), dump(&via.body));
    assert_eq!(
        format!("{:?}", door.naming),
        format!("{:?}", via.naming),
        "the birth record is carried across, not rebuilt"
    );
}

#[test]
fn the_chamfer_dispatch_is_the_chamfer_door() {
    let cube = sweep::test_support::cube(1.0, tol());
    let edges: Vec<_> = cube.edges().map(|(k, _)| k).collect();

    let door = chamfer_edges(&cube, &edges, 0.1, tol()).unwrap();
    let via = Verb::Chamfer {
        edges: edges.clone(),
        distance: 0.1,
    }
    .run(&cube, tol())
    .unwrap();

    assert_eq!(dump(&door.body), dump(&via.body));
    assert_eq!(format!("{:?}", door.naming), format!("{:?}", via.naming));
}

/// **A refusal crosses unaltered**, verb label included — the whole
/// reason `VerbError` wraps rather than re-classifies. The fixture is
/// the chamfer door's nonpositive-setback gate, which is the one
/// refusal reachable without building a degenerate body.
#[test]
fn a_refusal_crosses_the_dispatch_unaltered() {
    let cube = sweep::test_support::cube(1.0, tol());
    let edges: Vec<_> = cube.edges().map(|(k, _)| k).collect();

    let door = chamfer_edges(&cube, &edges, 0.0, tol()).unwrap_err();
    let via = Verb::Chamfer {
        edges,
        distance: 0.0,
    }
    .run(&cube, tol())
    .unwrap_err();

    let VerbError::Blend(carried) = via;
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());
}
