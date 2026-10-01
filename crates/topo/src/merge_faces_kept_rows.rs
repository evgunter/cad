//! **The merge door's kept boundaries, at rest.** A group on distinct
//! surface keys leaves the absorbed face's boundary edges on the
//! survivor; the door re-describes them before it returns, so the body
//! it hands back passes tier 3's adjacency check with no caller
//! repairing it.
//!
//! The fixture is a prism over a profile with one straight corner: its
//! front wall is two coplanar faces, each on its own plane key, and
//! every edge between two faces that meet at an angle is an
//! `Intersection` of their surfaces.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::EdgeDescription;
use geom_core::Tol;

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey};
use crate::geometry::SurfaceKey;
use crate::merge_faces::MergeCoplanarOutcome;
use crate::source::GeomSource;
use crate::test_support_fixtures::{describe_as_intersections, prism_z};
use crate::validate::{ValidationError, validate_geometric};

/// The five-corner prism whose front wall (`y = 0`) is the two faces
/// over `x ∈ [0, 1]` and `x ∈ [1, 2]`: `(body, kept, absorbed)`, the
/// faces in the order the door keeps and absorbs them.
fn split_wall_prism(tol: Tol) -> (Body<f64>, FaceKey, FaceKey) {
    let prism = prism_z::<f64>(
        &[(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
        0.0,
        1.0,
        tol,
    );
    (prism.body, prism.side_faces[0], prism.side_faces[1])
}

fn surface_of(body: &Body<f64>, face: FaceKey) -> SurfaceKey {
    body.get_face(face).expect("the face is live").surface
}

/// The edges whose stored `Intersection` names `key`, in edge-arena
/// order.
fn intersections_naming(body: &Body<f64>, key: SurfaceKey) -> Vec<EdgeKey> {
    body.edges()
        .filter(|(_, edge)| {
            body.get_curve_geom(edge.curve)
                .and_then(crate::null::CurveGeom::certified)
                .is_some_and(|curve| match curve.description() {
                    EdgeDescription::Intersection { s1, s2, .. }
                    | EdgeDescription::TangentIntersection { s1, s2, .. } => {
                        *s1 == key || *s2 == key
                    }
                    _ => false,
                })
        })
        .map(|(key, _)| key)
        .collect()
}

/// The edges tier 3 reports `DescriptionNotAdjacent` on.
fn not_adjacent(body: &Body<f64>, tol: Tol) -> Vec<EdgeKey> {
    match validate_geometric(body, tol) {
        Ok(()) => Vec::new(),
        Err(errors) => errors
            .iter()
            .filter_map(|error| match error {
                ValidationError::DescriptionNotAdjacent { edge } => Some(*edge),
                _ => None,
            })
            .collect(),
    }
}

/// The one group `outcome` merged: `kept` survived and absorbed
/// `absorbed` alone.
fn assert_one_group(outcome: &MergeCoplanarOutcome, kept: FaceKey, absorbed: FaceKey) {
    assert_eq!(outcome.groups.len(), 1, "one group merges: {outcome:?}");
    assert_eq!(outcome.groups[0].kept, kept, "the first wall face survives");
    assert_eq!(
        outcome.groups[0].absorbed,
        vec![absorbed],
        "the second wall face is absorbed"
    );
}

/// **The witness.** A declared merge of the two wall faces, on distinct
/// plane keys, returns a body tier 3 accepts: the absorbed face's
/// three surviving boundary edges (its bottom, its top and its far
/// strut, each an `Intersection` naming its key) are described again
/// against the survivor.
#[test]
fn a_declared_merge_on_distinct_keys_returns_no_strand() {
    let tol = Tol::witness();
    let (mut body, kept, absorbed) = split_wall_prism(tol);
    let (ks, ka) = (surface_of(&body, kept), surface_of(&body, absorbed));
    assert_ne!(ks, ka, "the fixture's wall faces are on two keys");
    assert_eq!(
        intersections_naming(&body, ka).len(),
        3,
        "the absorbed face's bottom, top and far strut are Intersections naming its key \
         (the shared strut between the two wall faces is smooth and named by none)"
    );

    let outcome = body
        .merge_coplanar_faces_declared(&[(ks, ka)], tol)
        .expect("the declared coplanar pair merges");

    assert_one_group(&outcome, kept, absorbed);
    assert_eq!(
        intersections_naming(&body, ka),
        Vec::<EdgeKey>::new(),
        "no edge is described against the absorbed key"
    );
    assert_eq!(
        validate_geometric(&body, tol),
        Ok(()),
        "the merged body is tier-3 valid as the door returns it"
    );
}

/// The undeclared door reaches distinct keys through a shared surface
/// source, and returns no strand there either.
#[test]
fn a_same_source_merge_on_distinct_keys_returns_no_strand() {
    let tol = Tol::witness();
    let (mut body, kept, absorbed) = split_wall_prism(tol);
    let (ks, ka) = (surface_of(&body, kept), surface_of(&body, absorbed));
    // One recipe surface evaluates to one description (N6): the
    // absorbed key takes the survivor's description bit for bit — the
    // same plane, so every certificate naming it still holds — and
    // both keys carry one source.
    let plane = body.get_surface(ks).expect("live").clone();
    *body.surfaces.get_mut(ka).expect("live") = plane;
    body.set_surface_source(ks, GeomSource::minted(1, 0))
        .expect("live");
    body.set_surface_source(ka, GeomSource::minted(1, 0))
        .expect("live");
    assert_eq!(intersections_naming(&body, ka).len(), 3);

    let outcome = body
        .merge_coplanar_faces(tol)
        .expect("the same-source coplanar pair merges");

    assert_one_group(&outcome, kept, absorbed);
    assert_eq!(intersections_naming(&body, ka), Vec::<EdgeKey>::new());
    assert_eq!(validate_geometric(&body, tol), Ok(()));
}

/// **No over-refusal on one key.** The wall faces on ONE plane key —
/// the merge the door has always made — merge as before, and what comes
/// back is tier-3 valid.
#[test]
fn a_same_key_merge_is_not_refused() {
    let tol = Tol::witness();
    let (mut body, kept, absorbed) = split_wall_prism(tol);
    let (ks, ka) = (surface_of(&body, kept), surface_of(&body, absorbed));
    body.faces.get_mut(absorbed).expect("live").surface = ks;
    // The construction's description step again, now against the one
    // key; the absorbed face's old key goes with the last curve that
    // named it.
    describe_as_intersections(&mut body, tol);
    assert!(
        body.get_surface(ka).is_none(),
        "nothing holds the old key any more"
    );

    let outcome = body
        .merge_coplanar_faces(tol)
        .expect("a same-key coplanar pair merges");

    assert_one_group(&outcome, kept, absorbed);
    assert_eq!(validate_geometric(&body, tol), Ok(()));
}

/// The door's re-description is what holds the witness: without it the
/// same merge strands exactly the absorbed face's three edges, which is
/// what the door refuses rather than return.
#[test]
fn the_strand_the_door_repairs_is_the_absorbed_faces_boundary() {
    let tol = Tol::witness();
    let (body, kept, absorbed) = split_wall_prism(tol);
    let named = intersections_naming(&body, surface_of(&body, absorbed));
    // The absorption alone, as the door's surgery runs it.
    let mut work = body.clone();
    work.merge_group(kept, &[absorbed], crate::merge_faces::MergeKind::Plane, tol)
        .expect("the pair absorbs");
    assert_eq!(
        not_adjacent(&work, tol),
        named,
        "the absorption strands exactly the edges that named the absorbed key"
    );
}
