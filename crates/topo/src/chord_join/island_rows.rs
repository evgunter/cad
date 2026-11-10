//! **What the ring rows on a cone, torus or sphere sheet share.**
//!
//! A sheet's outer loop runs counterclockwise about the face's outward
//! normal, so the face is the region the loop bounds at either sense: a
//! sense-`false` face's normal points the other way, and its loop is
//! walked from the other ruling or meridian. A sheet built with one loop
//! for both senses is, at sense `false`, the surface less that region,
//! and an island planted in it sits in the face's hole.

use super::*;

/// **The loop rule's start**: the chart coordinate a sheet's loop
/// starts from, `at` at sense `true` and its mirror at sense `false`, so
/// the loop is walked from the other ruling or meridian and the face is
/// the region it bounds at either sense (module docs). The one spelling
/// of the rule the sheet builders read.
pub(super) fn loop_start(sense: bool, at: f64) -> f64 {
    if sense { at } else { -at }
}

/// **The points lie in the face**: each is placed `In` the face by the
/// face's own region door, read on a copy of the face holding its outer
/// loop alone, or on its boundary or in band of it where the row sets an
/// island a hair inside an edge (`hair`). A point placed out, or that
/// the door refuses on, fails, and so does one in band where the row
/// does not set it there: an island or ring there is no ring of the
/// face, and the readings the rows hold to their oracles rest on its
/// being one.
pub(super) fn assert_in_face(
    body: &Body<f64>,
    face: FaceKey,
    points: impl IntoIterator<Item = Point3<f64>>,
    hair: bool,
) {
    let mut outer_only = body.clone();
    outer_only
        .get_face_mut(face)
        .expect("the sheet's face")
        .rings
        .clear();
    let band = Band::linear(Tol::witness()).expect("the witness band");
    for p in points {
        use crate::boolean::{ContainError, FaceContainment};
        let at = crate::contain::curved_face_containment(&outer_only, face, p, band);
        let near = matches!(
            at,
            Ok(Some(
                FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_)
            )) | Err(ContainError::Escalated { .. })
        );
        assert!(
            matches!(at, Ok(Some(FaceContainment::In))) || (hair && near),
            "a point the row plants in its face is not read in it, at {p:?}: {at:?}"
        );
    }
}

/// `carrier` at `n` interior stations of its span `(t0, t1)`, evenly.
pub(super) fn stations(
    carrier: &geom::Curve3<f64>,
    (t0, t1): (f64, f64),
    n: u32,
) -> impl Iterator<Item = Point3<f64>> + '_ {
    (1..=n).map(move |k| carrier.eval(t0 + (t1 - t0) * f64::from(k) / f64::from(n + 1)))
}
