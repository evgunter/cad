//! M5 S11: the `same_sense = .F.` half of the concave-sense unit.
//!
//! S11 makes constructors mint `sense: false` on walls whose material
//! lies against the chart normal — every such wall in this build is
//! CURVED (extrude's concave/hole cylinders, revolve's bore/cone/
//! sphere/torus bands; the one planar case, revolve's under-side
//! annulus, only occurs on bodies that also carry curved walls). The
//! two rows:
//!
//! 1. `.F.` is actually emitted — `same_sense` IS `topo::Face::sense`
//!    verbatim (the writer's S10 contract), exercised through the
//!    test-only hand-flip door on a planar body. Before S11 no test
//!    had ever produced an `.F.` in real output.
//! 2. The mixed-sense NOTCHED body exports end to end, with exactly
//!    one `.F.` `ADVANCED_FACE` — the concave wall's.
//!
//! **Row 2 is the M5 PR 13 flip.** It was written as a typed-REFUSAL
//! row (`UnsupportedCurve { kind: "circle" }`, the bottom cap's arc
//! rim being the first out-of-subset entity the surface-first walk
//! met) with its own instruction for what to become once the writer
//! grew its curved arms. It has: the arms landed, the body exports,
//! and the row now asserts the end-to-end fact the refusal stood in
//! for. Nothing about the CONSTRUCTOR changed — the same body, the
//! same one reversed wall; only the exporter's reach did.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use core::f64::consts::FRAC_PI_8;

use geom_core::Point2;
use geom_core::Tol;
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use step_export::{StepOptions, step_string};
use sweep::{Extrusion, extrude};

fn export(body: &topo::Body<f64>) -> String {
    let options = StepOptions {
        product_name: "s11".to_owned(),
        uncertainty_m: Some(1e-9),
        ..StepOptions::default()
    };
    step_string(body, &options, Tol::witness()).unwrap()
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

/// **Row 2 (the PR 13 flip): the mixed-sense notched body exports, and
/// exactly one of its six `ADVANCED_FACE`s writes `.F.`** — the
/// CONCAVE cylinder wall, the only face the constructor minted with
/// `sense: false`. The two convex/planar cylinder-and-plane faces stay
/// `.T.`, so this is not "the writer prints whatever it is handed":
/// the body has both senses and the text distinguishes them.
///
/// The `.F.` also has to land on the RIGHT face. The assertion below
/// reads the flag off the concave wall specifically, by requiring the
/// one `.F.` `ADVANCED_FACE` to reference a `CYLINDRICAL_SURFACE` —
/// and there are two cylinder walls in this body (one convex, one
/// concave), so a writer that flipped the wrong one, or flipped both,
/// fails here.
#[test]
fn notched_body_exports_with_exactly_one_reversed_cylinder_wall() {
    let b = FRAC_PI_8.tan();
    // Leaving bulges: the bottom arc bows out (+b), the top one bows
    // into the region (-b); the two sides are straight.
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(Point2::new(0.0, 0.0), b),
        ProfileVertex::new(Point2::new(2.0, 0.0), 0.0),
        ProfileVertex::new(Point2::new(2.0, 1.5), -b),
        ProfileVertex::new(Point2::new(0.0, 1.5), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(&vp, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body;
    // The kernel-side fact this row mirrors: one reversed face.
    assert_eq!(
        body.faces().filter(|(_, f)| !f.sense).count(),
        1,
        "the notched prism has exactly one concave wall"
    );
    let text = export(&body);

    let faces: Vec<&str> = text
        .lines()
        .filter(|l| l.contains("= ADVANCED_FACE("))
        .collect();
    assert_eq!(faces.len(), 6, "four planar faces and two cylinder walls");
    let reversed: Vec<&&str> = faces.iter().filter(|l| l.contains(".F.")).collect();
    assert_eq!(reversed.len(), 1, "exactly one same_sense = .F.");

    // Resolve that face's surface reference and demand a cylinder.
    let surface_ref = reversed[0]
        .rsplit_once("), #")
        .expect("ADVANCED_FACE names its surface last")
        .1
        .split(',')
        .next()
        .expect("surface id")
        .to_owned();
    assert!(
        text.contains(&format!("#{surface_ref} = CYLINDRICAL_SURFACE(")),
        "the reversed face is a cylinder wall, not a cap"
    );
    // Both cylinder walls are present — so the row really did pick one
    // of two, and the OTHER one is not reversed.
    assert_eq!(
        text.matches("= CYLINDRICAL_SURFACE(").count(),
        2,
        "convex wall and concave wall"
    );
}
