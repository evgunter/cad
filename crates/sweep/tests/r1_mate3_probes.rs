//! MATE-3 R1 review probes — reachability honesty (claim 7) and the
//! declared-cusp chain, re-executed rather than trusted.
//!
//! PR #1423 §3 claims: a `.cusp()` profile validates, `extrude` BUILDS
//! the cusp solid (v/e/f = 6/9/5), and `validate_geometric` refuses it
//! typed `UndeclaredCusp { wedge: Cusp }` — nothing silent, nothing
//! auto-declared. This file executes that chain.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Open, Start};
use sweep::{Extrusion, extrude};
use topo::{ContactClass, DeclaredContact, ValidationError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The lune (PR body's own corpus loop): one lip of the crescent
/// between two internally tangent circles, `.cusp()` at the kiss.
fn lune() -> profile::ClosedLoop<f64> {
    let tol = Tol::witness();
    Open.at(p2(0.0, 4.0))
        .angle(-std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .line(2.0, tol)
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .tangent_arc_to(p2(0.0, 0.0), tol)
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol)
        .unwrap()
}

#[test]
fn r1_cusp_profile_extrudes_and_the_at_rest_gate_refuses_typed() {
    let tol = Tol::witness();
    let closed = lune();
    let profile = profile::Profile::new(profile::SketchPlane::xy(), vec![closed.loop_])
        .validate(tol)
        .expect("the declared cusp profile must validate (the data gate accepts)");
    let built = extrude(&profile, Extrusion::Distance(1.0), tol)
        .expect("extrude must BUILD the cusp solid");
    let body = built.body;
    assert_eq!(
        (
            body.vertices().count(),
            body.edges().count(),
            body.faces().count()
        ),
        (6, 9, 5),
        "the PR's claimed v/e/f for the extruded lune"
    );
    // The at-rest gate refuses typed — nothing proceeded silently and
    // nothing emitted a declaration the author never made.
    let errs = topo::validate_geometric(&body, tol)
        .expect_err("the undeclared result must refuse at rest");
    assert_eq!(errs.len(), 1, "{errs:?}");
    let (cusp_edge, wedge) = match &errs[0] {
        ValidationError::UndeclaredCusp { edge, wedge } => (*edge, *wedge),
        other => panic!("expected UndeclaredCusp, got {other:?}"),
    };
    assert_eq!(wedge, geom_brep::MaterialWedge::Cusp);
    // Supplying the declaration BY HAND (the caller's job until the
    // sweep-side emission handoff lands) legalizes exactly that edge.
    let e = body.get_edge(cusp_edge).unwrap();
    let face_of = |he| {
        let l = body.get_half_edge(he).unwrap().parent_loop;
        body.get_loop(l).unwrap().face
    };
    let declared = [DeclaredContact {
        a: face_of(e.he_plus),
        b: face_of(e.he_minus),
        class: ContactClass::Tangent,
    }];
    assert_eq!(
        topo::validate_geometric_declared(&body, &declared, tol),
        Ok(())
    );
}
