//! M5 S11: the `same_sense = .F.` half of the concave-sense unit.
//!
//! S11 makes constructors mint `sense: false` on walls whose material
//! lies against the chart normal — every such wall in this build is
//! CURVED (extrude's concave/hole cylinders, revolve's bore/cone/
//! sphere/torus bands; the one planar case, revolve's under-side
//! annulus, only occurs on bodies that also carry curved walls). This
//! writer's analytic subset is still planes-only (the curved arms are
//! the exporter PR's unit), so a constructor-minted `.F.` face cannot
//! reach the emitter yet. What CAN be pinned now, and never was:
//!
//! 1. `.F.` is actually emitted — `same_sense` IS `topo::Face::sense`
//!    verbatim (the writer's S10 contract), exercised through the
//!    test-only hand-flip door on a planar body. Before S11 no test
//!    had ever produced an `.F.` in real output.
//! 2. The mixed-sense NOTCHED body refuses TYPED — the honest current
//!    disposition: no crash, no silent planar-only emission of a body
//!    whose reversed wall the text could not represent.
//!
//! When the exporter grows its `CYLINDRICAL_SURFACE` arm, row 2 flips
//! into the spec'd end-to-end row: the notched body exports with
//! exactly one `.F.` `ADVANCED_FACE`, the concave wall's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use core::f64::consts::FRAC_PI_8;

use geom_core::{Point2, Tolerance};
use profile::{Profile, ProfileLoop, SketchPlane};
use step_export::{StepExportError, StepOptions, step_string};
use sweep::{Extrusion, extrude};

fn export(body: &topo::Body<f64>) -> String {
    let options = StepOptions {
        product_name: "s11".to_owned(),
        uncertainty_m: Some(1e-9),
        ..StepOptions::default()
    };
    step_string(body, &options).unwrap()
}

/// **Row 1: `.F.` is real output.** Flip one cube face's bit through
/// the S10 instrument; the emitted text must carry exactly one
/// `same_sense = .F.` `ADVANCED_FACE` (and five `.T.`), because
/// `same_sense` is the stored bit verbatim — not a re-derivation from
/// the (unchanged) windings, which would print six `.T.`s.
#[test]
fn flipped_face_emits_exactly_one_f_flag() {
    let advanced_faces = |text: &str, flag: &str| {
        text.lines()
            .filter(|l| l.contains("= ADVANCED_FACE(") && l.contains(flag))
            .count()
    };
    let body = common::cube();
    let honest = export(&body);
    assert_eq!(advanced_faces(&honest, ".F."), 0);
    assert_eq!(advanced_faces(&honest, ".T."), 6);

    let (face, _) = body.faces().next().unwrap();
    let flipped = body.flipped_face_sense_for_tests(face).unwrap();
    let lied = export(&flipped);
    assert_eq!(
        advanced_faces(&lied, ".F."),
        1,
        "exactly the flipped face writes same_sense = .F."
    );
    assert_eq!(advanced_faces(&lied, ".T."), 5);
}

/// **Row 2: the mixed-sense notched body refuses typed.** The writer
/// must refuse rather than emit a text that cannot carry the reversed
/// wall. The first out-of-subset entity it meets is the BOTTOM CAP's
/// arc rim (caps precede walls in shell order, and a face emits its
/// bounds' edge carriers), so the refusal is `UnsupportedCurve
/// { kind: "circle" }` — the cylinder wall itself would refuse one
/// face later. Either way: typed, no silent planar-only output.
#[test]
fn notched_body_export_refuses_on_the_cylinder_wall() {
    let b = FRAC_PI_8.tan();
    let lp = ProfileLoop::builder(Point2::new(0.0, 0.0))
        .arc_to(Point2::new(2.0, 0.0), b)
        .line_to(Point2::new(2.0, 1.5))
        .arc_to(Point2::new(0.0, 1.5), -b)
        .close();
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let body = extrude(&vp, Extrusion::Distance(1.0)).unwrap().body;
    match step_string(&body, &StepOptions::default()) {
        Err(StepExportError::UnsupportedCurve { kind, .. }) => {
            assert_eq!(kind, "circle");
        }
        other => panic!("expected UnsupportedCurve, got {other:?}"),
    }
}
