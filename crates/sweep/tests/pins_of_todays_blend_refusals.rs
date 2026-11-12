//! **Pins of today's refusals, which BANDRULE's rows will flip.**
//!
//! Each row below pins a refusal the blend gives TODAY on an ordinary
//! part once its smooth edges are left as they are (REFS). They are not
//! what the blend should do: each names the BANDRULE row whose work
//! flips it, and that row's PR turns its pin into a build.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::blend::{BlendError, fillet_edges};
use sweep::test_support::at_rest;

use crate::a_blend_leaves_a_smooth_edge_as_it_is::{edge_between, slot};

fn tol() -> Tol {
    Tol::witness()
}

/// **A slot's end arc requested alone** ends at vertices where a smooth
/// strut arrives, the arc running on there tangentially into the slot's
/// straight sides: the request stops part way along a smooth run, and
/// refuses saying so, its recourse naming the whole run
/// (`work/band/a-chain-end-beside-a-smooth-seam-folds-to-an-indeterminate-corner.md`,
/// which this flipped from `UnsupportedCorner { Indeterminate }`).
#[test]
fn a_slot_end_arc_alone_refuses_as_a_tangent_run_requested_in_part() {
    let body = slot();
    let arc = topo::query::all_edges(&body)
        .into_iter()
        .find(|&e| {
            let (start, end) = body.edge_vertices(e).unwrap();
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let (a, b) = (p(start), p(end));
            a.z > 0.5 && b.z > 0.5 && a.x > 0.9 && b.x > 0.9 && (a.y - b.y).abs() > 0.9
        })
        .expect("the slot's right top arc");
    let line = edge_between(&body, [-1.0, -0.5, 1.0], [1.0, -0.5, 1.0]);
    assert_ne!(arc, line);
    let got = fillet_edges(&at_rest(&body, tol()), &[arc], 0.05, tol()).map_err(|r| r.error);
    assert!(
        matches!(
            &got,
            Err(BlendError::UnsupportedRunOut { detail, .. })
                if *detail == sweep::blend::battery::PARTIAL_TANGENT_RUN
        ),
        "a slot's end arc alone refuses as a tangent run requested in part, got {got:?}"
    );
}
