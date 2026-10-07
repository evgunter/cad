//! **A support face carrying scaffolding never reaches predicate 2's
//! screen.**
//!
//! Two scaffolding states a tier-1-valid body can carry on a support
//! face are features the face-clearance screen cannot read:
//!
//! - a null strut (`Body::mev_null`): an edge with no certified carrier;
//! - a lone-vertex ring (`mev` into the face, then `kemr`): a loop with
//!   no edges.
//!
//! The blend doors take a finished body, so each body is refused where
//! it would be finished, by tier 2, naming the scaffolding. Each row
//! first passes the same request on the clean block, so the refusal is
//! the scaffolding's and nothing else's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol};
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::block;
use topo::{AtRestBody, Body, EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, ValidationError};

/// The unit block, one of its faces, and that face's outer anchor
/// half-edge, at whose start the rows put their scaffolding.
fn block_and_anchor() -> (Body<f64>, FaceKey, HalfEdgeKey) {
    let body = block::<f64>(1.0, 1.0, 1.0, Tol::witness());
    let (face, outer) = body.faces().map(|(k, f)| (k, f.outer)).next().unwrap();
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("a block face's outer loop is a cycle");
    };
    (body, face, first)
}

/// The fillet of every edge the block was built with, at a radius the
/// clean block passes.
fn fillet(body: &Body<f64>, edges: &[EdgeKey]) -> Result<(), BlendError> {
    fillet_edges(
        &sweep::test_support::at_rest(body),
        edges,
        0.2,
        Tol::witness(),
    )
    .map(drop)
    .map_err(|r| r.error)
}

/// What the at-rest gate says of a body that does not finish.
fn unfinished(body: &Body<f64>) -> Vec<ValidationError> {
    AtRestBody::validate(body.clone(), Tol::witness())
        .map(drop)
        .expect_err("a body carrying scaffolding does not finish")
}

fn block_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    assert_eq!(edges.len(), 12, "a block has twelve edges");
    edges
}

#[test]
fn a_null_strut_on_a_support_face_does_not_finish() {
    let (mut body, _face, anchor) = block_and_anchor();
    let edges = block_edges(&body);
    fillet(&body, &edges).expect("the clean block fillets every edge at 0.2");

    let strut = body
        .mev_null(
            topo::MevSite::Fan {
                he1: anchor,
                he2: anchor,
            },
            topo::NewVertexSide::Below,
        )
        .expect("a null strut at the anchor's start");
    assert_eq!(topo::validate(&body), Ok(()), "tier 1 holds with the strut");
    assert_eq!(
        unfinished(&body),
        vec![
            ValidationError::ScaffoldingStrutVertex {
                vertex: strut.vertex,
            },
            ValidationError::NullEdgeAtRest { edge: strut.edge },
        ],
        "tier 2 names the strut's tip and its null edge"
    );
}

#[test]
fn a_lone_vertex_ring_on_a_support_face_does_not_finish() {
    let (mut body, face, anchor) = block_and_anchor();
    let edges = block_edges(&body);
    fillet(&body, &edges).expect("the clean block fillets every edge at 0.2");

    // A strut from the anchor's start to the face's centre, detached
    // into a ring: the centre vertex is left as a lone-vertex loop.
    let u = body.get_half_edge(anchor).unwrap().start;
    let p_u = *body.get_point(body.get_vertex(u).unwrap().point).unwrap();
    let corners: Vec<Point3<f64>> = body
        .loop_cycle(anchor)
        .unwrap()
        .into_iter()
        .map(|he| {
            let v = body.get_half_edge(he).unwrap().start;
            *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
        })
        .collect();
    let centre = Point3::new(
        corners.iter().map(|p| p.x).sum::<f64>() / 4.0,
        corners.iter().map(|p| p.y).sum::<f64>() / 4.0,
        corners.iter().map(|p| p.z).sum::<f64>() / 4.0,
    );
    let chord = body
        .mev(
            topo::MevSite::Fan {
                he1: anchor,
                he2: anchor,
            },
            centre,
            EdgeCurveSpec::line_between(p_u, centre),
            Tol::witness(),
        )
        .expect("a straight strut into the face");
    let ring = body
        .kemr(chord.he_plus, chord.he_minus)
        .expect("the strut detaches into a lone-vertex ring")
        .ring;
    assert_eq!(topo::validate(&body), Ok(()), "tier 1 holds with the ring");
    assert!(
        matches!(
            body.get_loop(ring).unwrap().boundary,
            LoopBoundary::Empty { .. }
        ),
        "the ring is a lone vertex"
    );
    assert_eq!(
        body.get_loop(ring).unwrap().face,
        face,
        "the ring sits on a support of the request"
    );

    assert_eq!(
        unfinished(&body),
        vec![ValidationError::ScaffoldingEmptyLoop { loop_: ring }],
        "tier 2 names the ring"
    );
}
