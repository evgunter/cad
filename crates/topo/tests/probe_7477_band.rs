//! Review probe (PR 7477): a finite ε within a factor K of f64::MAX is an
//! admitted tolerance whose K·ε overflows, so `Band::linear` refuses and
//! the door raises `EulerOpError::Band`. Run with
//! `CAD_TOLERANCE_EPS=1e308 CAD_AMBIGUITY_K=10`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use topo::construction::{Construct, MevSite};
use topo::{Body, EulerOpError};

#[test]
fn an_overflowing_k_eps_is_a_band_refusal() {
    if std::env::var("CAD_TOLERANCE_EPS").as_deref() != Ok("1e308") {
        return;
    }
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
    let err = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, 0.0),
            tol,
        )
        .unwrap_err();
    eprintln!("DEBUG: {err:?}\nDISPLAY: {err}");
    assert!(matches!(err, EulerOpError::Band(_)), "{err:?}");
}
