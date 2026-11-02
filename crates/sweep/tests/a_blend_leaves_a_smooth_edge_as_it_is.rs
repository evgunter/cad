//! **A blend leaves a requested smooth edge as it is, and names it.**
//!
//! An edge whose two faces continue one another (wedge π) is the
//! blend's own identity: a ball resting on both faces touches them
//! along the edge itself. Both verbs read it through the kernel's one
//! edge-wedge door (`geom_brep::classify_edge_wedge`), leave it out of
//! every chain, and name it in the result's `smooth` row; a request
//! whose every edge is smooth returns the body as it is. A cusp or slit
//! (wedge 0 or 2π) still refuses (`wedge_end_doors`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec3};
use sweep::blend::{BlendError, Blended, chamfer_edges, fillet_edges};
use sweep::test_support::{
    arc_polygon, at_rest, ball_poled_z, bored_block_of_arcs, revolved_about_y,
};
use topo::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

/// The edges whose two faces lie on one cylinder: a bore's struts.
fn struts(body: &Body<f64>) -> Vec<EdgeKey> {
    let on_cylinder = |he| {
        body.get_half_edge(he)
            .and_then(|h| body.get_loop(h.parent_loop))
            .and_then(|l| body.get_face(l.face))
            .and_then(|f| body.get_surface(f.surface))
            .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }))
    };
    let mut struts: Vec<EdgeKey> = body
        .edges()
        .filter(|(_, e)| on_cylinder(e.he_plus) && on_cylinder(e.he_minus))
        .map(|(k, _)| k)
        .collect();
    struts.sort_unstable();
    struts
}

/// Every edge of `body`, in key order: the "select all edges" request.
fn all_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let mut all = topo::query::all_edges(body);
    all.sort_unstable();
    all
}

fn sorted_smooth(out: &Blended<f64>) -> Vec<EdgeKey> {
    let mut smooth = out
        .naming
        .as_ref()
        .expect("the door writes its records")
        .smooth
        .clone();
    smooth.sort_unstable();
    smooth
}

/// **A block bored by a two-arc circle, every edge requested**: the
/// bore's two struts (straight cut edges between its two half-cylinder
/// faces, a wedge of exactly π) are left as they are and named, and the
/// rest — the block's twelve convex edges and the two bore rims — is
/// filleted. The chamfer leaves the struts too, and refuses at a rim,
/// whose cylinder support it has no strip for.
#[test]
fn a_two_arc_bore_with_every_edge_requested_rounds_and_names_its_struts() {
    let body = bored_block_of_arcs(2, 0.06, 0.005, 0.003, tol());
    let struts = struts(&body);
    assert_eq!(struts.len(), 2, "a two-arc bore has two struts");
    let operand = at_rest(&body, tol());
    let all = all_edges(&body);
    let out = fillet_edges(&operand, &all, 0.001, tol())
        .unwrap_or_else(|e| panic!("the fillet of every edge refused: {e}"));
    assert_eq!(
        sorted_smooth(&out),
        struts,
        "the fillet names the struts it left"
    );
    assert_eq!(
        out.blend_faces.len(),
        12,
        "the fillet rounds the block's twelve edges"
    );
    assert_eq!(out.band_faces.len(), 2, "and both bore rims");
    topo::validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("the fillet carves tier-3 valid, got {e:?}"));
    match chamfer_edges(&operand, &all, 0.001, tol()).map_err(|r| r.error) {
        Err(BlendError::ChamferArmUnsupported { edge, .. }) => assert!(
            !struts.contains(&edge),
            "the chamfer refuses at a rim, never at a strut it leaves"
        ),
        other => panic!("the chamfer refuses its curved rims, got {other:?}"),
    }
}

/// **A body every edge of which is smooth comes back as it is**: a ball
/// (its seam meridians) and a torus (its seam meridians and the
/// latitude circles its profile's two arcs sweep), filleted and
/// chamfered all round, each naming every edge it left.
#[test]
fn an_all_smooth_request_returns_the_body_unchanged_and_names_every_edge() {
    let ball = ball_poled_z(0.5, Vec3::new(0.0, 0.0, 0.0), tol());
    let torus = revolved_about_y(
        arc_polygon(2, 0.3, Point2::new(1.0, 0.0)),
        sweep::Revolution::Full,
        tol(),
    );
    for (name, body) in [("ball", ball), ("torus", torus)] {
        let all = all_edges(&body);
        assert!(!all.is_empty(), "the {name} has seams to request");
        let operand = at_rest(&body, tol());
        let fillet = fillet_edges(&operand, &all, 0.01, tol());
        let chamfer = chamfer_edges(&operand, &all, 0.01, tol());
        for (verb, out) in [("fillet", fillet), ("chamfer", chamfer)] {
            let out = out.unwrap_or_else(|e| panic!("the {name} {verb} refused: {e}"));
            assert_eq!(
                sorted_smooth(&out),
                all,
                "the {name} {verb} names every edge"
            );
            assert_eq!(
                (out.body.faces().count(), out.body.edges().count()),
                (body.faces().count(), body.edges().count()),
                "the {name} {verb} returns the body as it is"
            );
            assert!(
                out.blend_faces.is_empty()
                    && out.band_faces.is_empty()
                    && out.corner_faces.is_empty(),
                "the {name} {verb} minted a band"
            );
        }
    }
}
