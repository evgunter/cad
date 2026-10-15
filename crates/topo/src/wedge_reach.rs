//! **The faces' extent an edge's wedge is levered at** — the one home
//! of the `reach` every reader of the dihedral between an edge's two
//! faces passes to [`geom_brep::folded_lever_arm`]: tier 3's check 4,
//! certification of an `Intersection` or `TangentIntersection`, and the
//! must-carry rule wherever a door decides what a join stores.
//!
//! A dihedral is how far the two faces separate as they leave the edge,
//! so it is levered at how far they run (D4 ¶1's face extent) and how
//! they curve (the fold's curvature arms), never at how long the edge
//! is: an edge's own extent reads a short arc between large faces at
//! dihedral δ as a wedge of δ², where the faces open one of δ. The reach
//! is the farther face's: how far either face's boundary runs from the
//! edge's middle (`splitting::rules::face_reach_from`'s measure), so two
//! small faces at a small angle read a small wedge, and one large face
//! beside a small one reads the gap the large one opens. It is read off
//! the body at every reading and never stored, so a door that changes a
//! face changes the wedge reading of every edge the face bounds.

use geom::Curve3;
use geom_core::{Decide, Point3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey};

/// The reach of the wedge between `faces` along the edge on `carrier`
/// over `(t0, t1)`: the farthest either face's boundary runs from the
/// carrier's middle, the edge's own span folded in as a boundary edge
/// of each. At rest that span bounds both faces already; a door minting
/// the edge names the faces it is about to bound (both one face, where
/// the edge is a strut in it or a chord dividing it) and reads them
/// with the edge in place.
///
/// # Panics
///
/// Where a face, or a record its boundary walk reaches, does not
/// resolve: every caller read `faces` out of `body`.
#[must_use]
pub fn wedge_reach<T: Decide>(
    body: &Body<T>,
    faces: [FaceKey; 2],
    carrier: &Curve3<T>,
    (t0, t1): (T, T),
) -> T {
    let at: Point3<T> = carrier.mid_point(t0, t1);
    let span = geom_brep::Reach::Span {
        carrier: carrier.clone(),
        t0,
        t1,
    };
    let [a, b] = faces;
    let reach = |face| crate::splitting::rules::face_wedge_reach_from(body, face, at, &span);
    if a == b {
        reach(a)
    } else {
        reach(a).max(reach(b))
    }
}

/// [`wedge_reach`] of a live `edge`, over its two faces and its stored
/// carrier; `None` where the edge carries no certified curve (null
/// scaffolding, mid-op).
///
/// # Panics
///
/// Where `edge` or a record on the walk from it does not resolve: every
/// caller read `edge` out of `body`.
#[must_use]
pub fn edge_wedge_reach<T: Decide>(body: &Body<T>, edge: EdgeKey) -> Option<T> {
    let data = &body.edges[edge];
    let curve = body.curves.get(data.curve)?.certified()?;
    Some(wedge_reach(
        body,
        edge_faces(body, edge),
        curve.carrier(),
        curve.params(),
    ))
}

/// `edge`'s two faces, `he_plus`'s first.
///
/// # Panics
///
/// As [`edge_wedge_reach`].
#[must_use]
pub fn edge_faces<T: geom_core::Real>(body: &Body<T>, edge: EdgeKey) -> [FaceKey; 2] {
    let data = &body.edges[edge];
    [data.he_plus, data.he_minus].map(|he| {
        body.face_of_half_edge(he)
            .unwrap_or_else(|| unreachable!("{he:?}, a half of live {edge:?}, has no face"))
    })
}
