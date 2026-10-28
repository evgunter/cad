//! R1's probes on INTENT E (`an-operation-computes-in-a-frame-of-its-reads`).
#![allow(clippy::expect_used)]

use crate::common;
use pncad::document::{BodyRead, Node, Operand, VarId};
use pncad::geom_core::Tol;
use viewer::session::DocSession;

/// The session's kept product (`DocSession::landed_body`) and the
/// scene's product (`scene::product_body`) are one aggregate: a turned
/// only copy stands where its placement puts it in both.
#[test]
fn landed_body_stands_where_the_scene_product_stands() {
    let tol = Tol::witness();
    let doc = pncad::document::Doc::empty_derived("r1-landed", tol);
    let (doc, profile) = common::square_profile(&doc, 0.04, tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: common::len(0.01),
        },
        tol,
    );
    let (doc, moved) = common::inserted(
        &doc,
        Node::place(
            BodyRead::plain(Operand::from(extrude)),
            Operand::Var(VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    let (c, s) = (0.6_f64, 0.8_f64);
    let at = pncad::document::Frame {
        columns: [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.25, -0.5, 0.125],
    };
    let doc = common::pin(
        &doc,
        moved,
        &at,
        &pncad::document::EvalOptions::default(),
        tol,
    );
    let doc = crate::fixture::assemble_world_copies(doc);
    let world = viewer::scene::product_body(&doc, tol).expect("the copy gathers");
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let landed = session.landed_body().expect("the copy lands");
    let bbox = |b: &pncad::topo::Body<f64>| {
        b.vertex_points().fold([f64::INFINITY; 3], |m, (_, p)| {
            [m[0].min(p.x), m[1].min(p.y), m[2].min(p.z)]
        })
    };
    eprintln!(
        "world min {:?}  landed min {:?}",
        bbox(&world),
        bbox(landed)
    );
    let delta = viewer::DisplayTolerance::new(1e-4).expect("a delta");
    let fit_w = viewer::scene::fit_delta(&world, delta, tol).expect("fits");
    let fit_l = viewer::scene::fit_delta(landed, delta, tol).expect("fits");
    eprintln!("fit world {fit_w:?}\nfit landed {fit_l:?}");
    let (w, l) = (bbox(&world), bbox(landed));
    assert!(
        (w[0] - l[0]).abs() < 1e-9 && (w[1] - l[1]).abs() < 1e-9 && (w[2] - l[2]).abs() < 1e-9,
        "the landed product stands where the scene's product stands: world min {w:?}, landed min {l:?}"
    );
}
