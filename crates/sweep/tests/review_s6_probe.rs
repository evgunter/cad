//! S6 adversarial review probe (e2e): trigger the extrusion pair
//! (pair 6) through the real extrude API and eyeball/assert the
//! rendered messages. Adopted from the
//! reviewer's probe set in the S6 fix pass.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{COINCIDENCE_RECOURSE, Point2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{ExtrudeError, Extrusion, extrude};
use geom_core::Tol;

fn square() -> ValidatedProfile<f64> {
    let l = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.0, 1.0),
    ]);
    Profile::new(SketchPlane::xy(), vec![l])
        .validate(Tol::witness())
        .unwrap()
}

fn assert_unified(msg: &str) {
    assert_eq!(
        msg.matches(COINCIDENCE_RECOURSE).count(),
        1,
        "carrier must appear exactly once: {msg}"
    );
    assert!(!msg.contains("MarginDiag"), "Debug leakage: {msg}");
}

#[test]
fn probe_extrusion_pair_e2e() {
    let vp = square();
    // Definite arm: in-plane vector, no normal component at all.
    let err = extrude(&vp, Extrusion::Vector(Vec3::new(1.0, 0.0, 0.0)), Tol::witness()).unwrap_err();
    assert_eq!(err, ExtrudeError::DegenerateExtrusion);
    let msg = err.to_string();
    eprintln!("[probe] extrude definite:\n  {msg}\n");
    assert_unified(&msg);

    // Escalated arm: sliver distance strictly inside the band.
    let eps = Tol::witness().get().eps;
    let err = extrude(&vp, Extrusion::Distance(3.0 * eps), Tol::witness()).unwrap_err();
    let msg = err.to_string();
    eprintln!("[probe] extrude in-band:\n  {msg}\n");
    assert!(
        matches!(err, ExtrudeError::ExtrusionEscalated { .. }),
        "{err:?}"
    );
    assert_unified(&msg);
    assert!(
        msg.contains("ambiguity band"),
        "margin payload must survive: {msg}"
    );
}
