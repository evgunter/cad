//! **What a reversed chart costs a body that already has edges on it.**
//!
//! `NurbsSurface::reversed_v` preserves the POINT SET, not the
//! parameterization: the reversed chart answers at `v` what the source
//! answered at `lo + hi − v`. Every parameter already recorded against
//! the old chart — a pcurve, an edge description's interval — therefore
//! goes stale the moment the chart is swapped under its face, and
//! nothing in the swap can say so: `Body::set_face_surface` reports
//! whether the face key resolves, and its own docs put the geometric
//! consequence on tier 3 (attach surfaces BEFORE upgrading edge
//! descriptions).
//!
//! This row is the hazard, pinned: structural validation stays green
//! over the surgery and the geometric-structural tier reports the stale
//! parameters on every edge of every reversed face. It exists so that a
//! caller who reads `reversed_v`'s "What this does not do" paragraph can
//! see what the door does not do, rather than take its word for it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Tol, Vec3};
use std::sync::Arc;
use topo::{Body, FaceSurface, ValidationError};

fn prism() -> Body<f64> {
    let square = || crate::common::quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]);
    let sections = vec![square(), square(), square()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.5, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the offset square prism builds")
        .body
}

#[test]
fn reversing_a_chart_under_its_face_strands_the_parameters_on_it() {
    let tol = Tol::witness();
    let mut body = prism();
    assert!(
        topo::validate(&body).is_ok(),
        "the lofted body validates structurally before the surgery"
    );
    assert!(
        topo::validate_geometric_structural(&body, tol).is_ok(),
        "and geometrically-structurally too: the pcurves match their charts"
    );

    let faces: Vec<_> = body.faces().map(|(fk, f)| (fk, f.surface)).collect();
    let mut reversed = 0usize;
    for (fk, sk) in faces {
        let Some(Surface::Nurbs(n)) = body.get_surface(sk) else {
            continue;
        };
        let n = (**n).clone();
        let Ok(r) = n.reversed_v() else {
            continue;
        };
        body.set_face_surface(fk, FaceSurface::New(Surface::Nurbs(Arc::new(r))))
            .expect("the face key resolves");
        reversed += 1;
    }
    assert_eq!(
        reversed, 4,
        "the prism's four walls are NURBS charts with a mirror-symmetric knots_v"
    );

    assert!(
        topo::validate(&body).is_ok(),
        "structural validation stays green: nothing structural moved, which is \
         exactly why a caller can be surprised here"
    );

    let errs = topo::validate_geometric_structural(&body, tol)
        .expect_err("the reversed charts stranded every parameter recorded against them");
    let mut stale_descriptions = 0usize;
    let mut stale_pcurves = 0usize;
    for e in &errs {
        match e {
            ValidationError::DescriptionNotAdjacent { .. } => stale_descriptions += 1,
            ValidationError::Pcurve { .. } => stale_pcurves += 1,
            other => panic!("an unexpected finding after a chart reversal: {other:?}"),
        }
    }
    assert_eq!(
        stale_descriptions, 4,
        "one per reversed wall: the edge description's interval means what it \
         meant in the OLD chart ({errs:?})"
    );
    assert_eq!(
        stale_pcurves, 16,
        "four per reversed wall: every pcurve on the face now maps its samples \
         somewhere else on the same point set ({errs:?})"
    );
}
