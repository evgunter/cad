//! R2 REVIEW PROBES for MATE-3 (PR 1423) — the two corpus residuals
//! the arm re-cut. Committed to the reviewer's branch only.
//!
//! Claim under test (PR §"What the arm found in the corpus"): the
//! `LaminaWedge` catches on `cut_cylinder`'s single-wall flip and on
//! the half-flipped rimless ball are GENUINE — the flipped body really
//! carries a lamina wedge at those edges — and are not over-refusals.
//!
//! `LaminaWedge`'s user-facing message asserts, verbatim: *"That is a
//! lamina: a zero-volume geometric defect, and no contact declaration
//! cures it — move the geometry"*. These probes MEASURE the volume of
//! each body the arm calls a lamina and print it beside the verdict, so
//! that sentence is checkable rather than asserted.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{ball, cut_cylinder};
use geom_core::Tol;
use topo::{ValidationError, validate_geometric};

#[test]
fn r2_lamina_verdicts_versus_measured_volume() {
    let tol = Tol::witness();

    // --- the conic-trim residual -------------------------------------
    let body = cut_cylinder();
    let v0 = topo::mass_properties(&body, tol).map(|m| m.volume);
    println!(
        "R2 cut_cylinder: clean volume {v0:?} verdict {:?}",
        validate_geometric(&body, tol).is_ok()
    );
    let walls: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    for k in walls {
        let flipped = body.flipped_face_sense_for_tests(k).expect("live face");
        let errs = validate_geometric(&flipped, tol).unwrap_err();
        let vol = topo::mass_properties(&flipped, tol).map(|m| m.volume);
        let all_lamina = errs
            .iter()
            .all(|e| matches!(e, ValidationError::LaminaWedge { .. }));
        println!(
            "R2 cut_cylinder wall {k:?}: volume {vol:?}; all_lamina={all_lamina}; errs={errs:?}"
        );
    }

    // --- the rimless-ball residual -----------------------------------
    let b = ball();
    let v0 = topo::mass_properties(&b, tol).map(|m| m.volume);
    println!("R2 ball: clean volume {v0:?}");
    let bands: Vec<_> = b.faces().map(|(k, _)| k).collect();
    for k in bands {
        let flipped = b.flipped_face_sense_for_tests(k).expect("live face");
        let errs = validate_geometric(&flipped, tol);
        let vol = topo::mass_properties(&flipped, tol).map(|m| m.volume);
        println!("R2 ball band {k:?}: volume {vol:?}; verdict {errs:?}");
    }
}
