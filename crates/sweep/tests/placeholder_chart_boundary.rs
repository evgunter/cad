//! **A chart boundary on the mvfs placeholder refuses, typed.**
//!
//! `topo::chart_boundary` takes its chart from the caller. Handed the
//! placeholder for a face whose edges are described on the face's OWN
//! chart, the walk never evaluates the chart — it reads each image off
//! the description — so nothing poison stops it, and before the door
//! refused it metred every joint gap through the placeholder's unit
//! lever arms and returned a description of a region on a chart that
//! has no locus. A lofted body's planar caps are such faces.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use topo::{PcurveMintError, chart_boundary};

use crate::common::approx::{band, twisted_loft};

#[test]
fn chart_boundary_refuses_the_placeholder_chart_on_every_face() {
    let body = twisted_loft(0.3);
    let placeholder = Surface::<f64>::nurbs_placeholder();
    let mut own_described = 0usize;
    for (face, f) in body.faces() {
        let own = body.get_surface(f.surface).unwrap();
        if chart_boundary(&body, face, own, band()).is_ok() {
            own_described += 1;
        }
        let refused = chart_boundary(&body, face, &placeholder, band())
            .expect_err("no region of the placeholder is described");
        assert_eq!(
            refused,
            PcurveMintError::PlaceholderChart { face },
            "face {face:?}: the refusal names the placeholder, not a symptom of its poison"
        );
    }
    assert!(
        own_described >= 2,
        "the fixture must hold faces that describe on their own chart, or the \
         refusal above is not shown to be the placeholder's: {own_described}"
    );
}
