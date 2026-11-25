//! **An approximated face is asked embedded as its fit**
//! (`topo::clearance::embedded`, OFFSET-DESIGN O2: the fit is the
//! geometry, its edges are sections of it, and its certified distance
//! from the described surface is the face's own claim, re-derived at
//! rest).
//!
//! The body is the box `[0,2]² × [0,1]` whose top cap is an `Approx`
//! whose fit and description genuinely differ ([`loose_cap`]): the fit
//! is the plane `z = 1`, which the cap's edges lie on; the description
//! is the plane `z = 0.95` offset by `d = 0.04`, the plane `z = 0.99`;
//! and the certifier proves the `0.01` between them at a target of
//! `0.02`. A `0.2` brick is grafted in as a second solid over the cap's
//! middle, wholly inside the box's footprint, so the only face it can
//! meet is the cap:
//!
//! - floating `0.003` above the fit, the body is `Embedded`: red if the
//!   window refuses the face, or reads a fit moved up by that much;
//! - hanging inside the box with its top at `0.995`, between the
//!   description and the fit, the body is `Embedded`: red if the face is
//!   read as its description or its base, both of which the brick
//!   pierces, or as its fit padded by the certified distance, which
//!   reaches it;
//! - piercing the cap, the body `Crosses`, the cap one of the witness's
//!   two faces: red if the cap's cells are dropped, since no other pair
//!   of faces meets;
//! - each verdict and its receipt equal those of the same box capped
//!   with the fit as a plain NURBS face.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{ApproxSurface, ApproxWindow, Surface, SurfaceDescription, SurfaceSpec};
use geom_core::{Band, RigidMotion, Tol, Vec3};
use topo::clearance::{Budgets, DEFAULT_MAX_CELL_DEPTH, Embedding, EmbeddingReport, Side};
use topo::{Body, FaceKey};

use crate::common::approx::{box_with_spline_cap, planar_patch, pulled_back};

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

/// **The fit `z = 1` certified as the plane `z = 0.95` offset by `0.04`**,
/// at a target of `0.02`, by the production certifier.
fn loose_cap() -> Surface<f64> {
    let fit = planar_patch(1.0);
    let base = Arc::new(pulled_back(&fit, 0.05));
    let window = ApproxWindow::of(&*base);
    let spec = SurfaceSpec {
        description: SurfaceDescription::Offset { base, d: 0.04 },
        fit,
        window,
    };
    let approx = ApproxSurface::certify(spec, |description, fit, window| {
        geom_brep::certify_offset_over_at(description, fit, window, 0.02, band())
    })
    .unwrap_or_else(|e| panic!("the fit certifies within 0.02 of its description: {e:?}"));
    let off = approx.certificate().hull_sup;
    assert!(
        (0.009..=0.02).contains(&off),
        "the certificate proves the fit's 0.01 from its description: {off}"
    );
    Surface::Approx(Arc::new(approx))
}

/// The loosely capped box and its cap, and the same box capped with the
/// cap's fit as a plain NURBS face.
fn capped() -> [(Body<f64>, FaceKey); 2] {
    let cap = loose_cap();
    let Surface::Approx(a) = &cap else {
        unreachable!()
    };
    let fit = Surface::Nurbs(Arc::new(a.fit().clone()));
    [box_with_spline_cap(cap), box_with_spline_cap(fit)]
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
    let [(approx, cap), (nurbs, _)] = capped();
    for (how, z, clear) in [
        ("just above", 1.003, true),
        ("between its description and its fit", 0.795, true),
        ("through", 0.9, false),
    ] {
        let a = embedding(&with_brick(&approx, z));
        let n = embedding(&with_brick(&nurbs, z));
        println!("[remeter] a brick {how} the approximated cap: {a:?}");
        match (&a.verdict, clear) {
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
