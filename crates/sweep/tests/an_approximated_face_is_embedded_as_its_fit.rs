//! **An approximated face is asked embedded as its fit**
//! (`topo::clearance::embedded`, OFFSET-DESIGN O2: the fit is the
//! geometry, its edges are sections of it, and its certified distance
//! from the described surface is the face's own claim, re-derived at
//! rest).
//!
//! The body is [`box_with_approx_cap`]: the box `[0,2]² × [0,1]` whose
//! top cap is the certified `Approx` of the plane `z = 1`, its edges
//! re-described as isos of the fit's chart. A small brick is grafted in
//! as a second solid over the cap's middle, wholly inside the box's
//! footprint, so the ONLY faces the brick can meet are the cap:
//!
//! - floating clear of the cap, and hanging inside the box just under
//!   it, its top `0.03` below the fit and `0.02` above the description's
//!   base (the cap plane pulled back by `d = 0.05`), the body is
//!   `Embedded`: red if the approximated face is refused at its window,
//!   or read as its base, which the hanging brick pierces;
//! - piercing the cap, the body `Crosses`, the cap one of the witness's
//!   two faces: red if the cap's cells are dropped or read anywhere but
//!   on its fit, since no other pair of faces meets;
//! - each verdict and its receipt equal those of the same box capped
//!   with the fit itself as a plain NURBS face: red if the approximated
//!   face is read as anything but its fit (the description's base, or a
//!   chart padded by its certified distance).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::Surface;
use geom_core::{Band, RigidMotion, Tol, Vec3};
use topo::clearance::{Budgets, DEFAULT_MAX_CELL_DEPTH, Embedding, EmbeddingReport, Side};
use topo::{Body, FaceKey};

use crate::common::approx::{box_with_approx_cap, box_with_spline_cap};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// The verdict door over every face of `body`, at the document door's
/// default budgets.
fn embedding(body: &Body<f64>) -> EmbeddingReport {
    let side = Side {
        body,
        faces: body.faces().map(|(k, _)| k).collect(),
    };
    topo::clearance::embedded(
        &side,
        band(),
        Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: 65_536,
        },
    )
}

/// The offset-capped box and its cap, and the same box capped with the
/// cap's fit as a plain NURBS face.
fn capped() -> [(&'static str, Body<f64>, FaceKey); 2] {
    let (approx, cap) = box_with_approx_cap(0.05, 1e-9);
    let Surface::Approx(a) = approx.face(cap).unwrap().surface().unwrap() else {
        panic!("the cap wears the approximated surface");
    };
    let (nurbs, nurbs_cap) = box_with_spline_cap(Surface::Nurbs(Arc::new(a.fit().clone())));
    [
        ("the approximated cap", approx, cap),
        ("its fit as a NURBS cap", nurbs, nurbs_cap),
    ]
}

/// `body` with a `0.2` brick grafted in over the cap's middle, its floor
/// at height `z`.
fn with_brick(body: &Body<f64>, z: f64) -> Body<f64> {
    let brick = topo::transform_rigid(
        &sweep::test_support::block::<f64>(0.2, 0.2, 0.2, tol()),
        &RigidMotion::translation(Vec3::new(0.9, 0.9, z)).unwrap(),
    )
    .expect("a planar brick moves");
    let mut out = body.clone();
    topo::graft_disjoint(&mut out, &brick).expect("the brick grafts");
    out
}

#[test]
fn an_approximated_cap_reads_as_its_fit_beside_and_through_a_brick() {
    let [(_, approx, cap), (_, nurbs, _)] = capped();
    for (how, z) in [("clear of", 1.05), ("just under", 0.77), ("through", 0.9)] {
        let a = embedding(&with_brick(&approx, z));
        let n = embedding(&with_brick(&nurbs, z));
        println!("[remeter] a brick {how} the approximated cap: {a:?}");
        match (&a.verdict, how != "through") {
            (Embedding::Embedded, true) => {}
            (Embedding::Crosses(c), false) => assert!(
                c.faces.0 == cap || c.faces.1 == cap,
                "a brick {how} the cap crosses only the cap: {c:?}"
            ),
            (verdict, _) => panic!("a brick {how} the approximated cap reads {verdict:?}"),
        }
        assert_eq!(
            a, n,
            "a brick {how} the cap: the approximated face reads as its fit does"
        );
    }
}
