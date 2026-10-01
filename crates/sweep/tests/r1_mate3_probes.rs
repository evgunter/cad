//! The declared-cusp chain, executed: a `.cusp()` profile validates,
//! `extrude` builds the cusp solid (v/e/f = 6/9/5), and
//! `validate_geometric` passes it — the strut's tangency is
//! jet-determinate, which is what makes a wedge-0 edge legal at rest.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Open, Start};
use sweep::{Extrusion, extrude};
use topo::ContactMark;

/// The lune (PR body's own corpus loop): one lip of the crescent
/// between two internally tangent circles, `.cusp()` at the kiss.
fn lune() -> profile::ClosedLoop<f64> {
    let tol = Tol::witness();
    Open.at(Point2::new(0.0, 4.0))
        .angle(-std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .line(2.0, tol)
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .tangent_arc_to(Point2::new(0.0, 0.0), tol)
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol)
        .unwrap()
}

#[test]
fn r1_cusp_profile_extrudes_and_the_cusp_is_legal_at_rest() {
    let tol = Tol::witness();
    let closed = lune();
    let profile = profile::Profile::new(profile::SketchPlane::xy(), vec![closed.loop_])
        .validate(tol)
        .expect("the declared cusp profile must validate (the data gate accepts)");
    let built = extrude(&profile, Extrusion::Distance(1.0), tol)
        .expect("extrude must BUILD the cusp solid");
    let body = &built.body;
    assert_eq!(
        (
            body.vertices().count(),
            body.edges().count(),
            body.faces().count()
        ),
        (6, 9, 5),
        "the PR's claimed v/e/f for the extruded lune"
    );
    // The at-rest gate passes it, and check 4 JUDGED the strut rather
    // than exempting it: exactly one edge carries a `Tangent` mark, and
    // it is the strut standing on the kiss (x = y = 0) — the cusp
    // joint's, since a `Tangent` mark alone would also fit a π seam.
    assert_eq!(topo::validate_geometric(body, tol), Ok(()));
    let marks = topo::contact_marks(body, tol).expect("the cusp solid is valid");
    let tangent: Vec<_> = marks
        .iter()
        .filter(|(_, m)| **m == ContactMark::Tangent)
        .map(|(e, _)| e)
        .collect();
    let [marked] = tangent.as_slice() else {
        panic!("one Tangent mark: {tangent:?}");
    };
    assert!(
        built.strut_edges.iter().flatten().any(|e| e == marked),
        "the marked edge is a strut"
    );
    let he = body.get_edge(*marked).unwrap().he_plus;
    for v in [
        body.get_half_edge(he).unwrap().start,
        body.half_edge_end(he).unwrap(),
    ] {
        let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        assert!(
            p.x.abs() < 1e-9 && p.y.abs() < 1e-9,
            "the marked strut stands on the kiss: {p:?}"
        );
    }
}
