//! **Predicate 2's screen reads every boundary feature of a support
//! face, or refuses.**
//!
//! A feature the face-clearance screen leaves out of its pair sweep is
//! metered against nothing, so the screen would report the face clear
//! having never looked at it. Two scaffolding states a tier-1-valid
//! body can carry on a support face are exactly the features the screen
//! cannot read, and each refuses typed, the way the surgery's own
//! closed-form meters refuse them:
//!
//! - a null strut (`Body::mev_null`): an edge with no certified carrier;
//! - a lone-vertex ring (`mev` into the face, then `kemr`): a loop with
//!   no edges.
//!
//! Each row first passes the same request on the clean block, so the
//! refusal is the scaffolding's and nothing else's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol};
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::block;
use topo::{Body, EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopBoundary};

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
/// clean block passes; the scaffolding edges are not requested.
fn fillet(body: &Body<f64>, edges: &[EdgeKey]) -> Result<(), BlendError> {
    fillet_edges(body, edges, 0.2, Tol::witness())
        .map(drop)
        .map_err(|r| r.error)
}

fn block_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    assert_eq!(edges.len(), 12, "a block has twelve edges");
    edges
}

#[test]
fn a_null_strut_on_a_support_face_refuses_as_an_uncertified_carrier() {
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
    let strut_edge = body.get_half_edge(strut.he_plus).unwrap().edge;

    match fillet(&body, &edges) {
        Err(BlendError::UnsupportedGeometry { at, detail }) => {
            assert_eq!(
                at,
                EntityId::Edge(strut_edge),
                "the refusal names the strut"
            );
            assert!(
                detail.contains("no certified carrier"),
                "the refusal says the carrier is what it cannot read: {detail}"
            );
        }
        other => panic!("expected the screen to refuse the null strut, got {other:?}"),
    }
}

#[test]
fn a_lone_vertex_ring_on_a_support_face_refuses_as_a_lone_vertex_cycle() {
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

    match fillet(&body, &edges) {
        Err(BlendError::UnsupportedGeometry { at, detail }) => {
            assert_eq!(at, EntityId::Loop(ring), "the refusal names the ring");
            assert!(
                detail.contains("lone-vertex cycle"),
                "the refusal says the loop is what it cannot read: {detail}"
            );
        }
        other => panic!("expected the screen to refuse the lone-vertex ring, got {other:?}"),
    }
}
