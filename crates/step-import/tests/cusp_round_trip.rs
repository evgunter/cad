//! **A file's knife edge imports as its native twin's legal state.**
//! Import transcribes the file's shared `EDGE_CURVE` — the structural
//! record D1 tier 3 reads a wedge end off — and mints no tangency of
//! its own, exactly as it admits every π seam; a jet-determinate cusp
//! is legal at rest, so it ships (D1 tier 3, D7 step 4; Ev on PR 3317).
//! The row: a native `.cusp()` lune extrude round-trips through the
//! writer and the reader under default options, and the reader's body
//! carries the cusp where the native one does.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol};
use profile::{Open, Profile, SketchPlane, Start};
use step_import::{ImportOptions, StepImport, import_step};
use sweep::{Extrusion, extrude};

/// The lune between the internally tangent circles `(0,1) r 1` and
/// `(0,2) r 2`, `.cusp()` at the kiss, extruded by 1.
fn cusp_lune() -> topo::Body<f64> {
    let tol = Tol::witness();
    let lune = Open
        .at(Point2::new(0.0, 4.0))
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
        .into();
    let profile = Profile::new(SketchPlane::xy(), vec![lune])
        .validate(tol)
        .expect("the declared cusp validates");
    extrude(&profile, Extrusion::Distance(1.0), tol)
        .expect("the cusp lune extrudes")
        .body
}

/// The endpoints of every edge check 4 marks `Tangent`.
fn tangent_edges(body: &topo::Body<f64>) -> Vec<[Point3<f64>; 2]> {
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    topo::contact_marks(body, Tol::witness())
        .expect("the body is tier-3 valid")
        .iter()
        .filter(|(_, m)| **m == topo::ContactMark::Tangent)
        .map(|(e, _)| {
            let he = body.get_edge(e).unwrap().he_plus;
            [
                point(body.get_half_edge(he).unwrap().start),
                point(body.half_edge_end(he).unwrap()),
            ]
        })
        .collect()
}

#[test]
fn a_native_cusp_round_trips_through_step_as_a_solid() {
    let native = cusp_lune();
    let on_the_kiss = |p: &Point3<f64>| p.x.abs() < 1e-9 && p.y.abs() < 1e-9;
    let before = tangent_edges(&native);
    assert_eq!(before.len(), 1, "the native strut, {before:?}");

    let text = step_export::step_string(
        &native,
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect("the cusp lune exports");
    let back = match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("the cusp lune must re-import as a solid, got {other:?}"),
    };

    let after = tangent_edges(&back);
    assert_eq!(after.len(), 1, "one tangent edge after the trip, {after:?}");
    assert!(
        after[0].iter().all(on_the_kiss),
        "the tangent edge is the strut on the kiss line, {after:?}"
    );
}
