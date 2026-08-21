//! The cutaway section (#91 C4): the tour's first `pncad::topo::split` — the
//! project box (itself a 15-op boolean result) split by a TILTED
//! plane, both halves validated independently, then translated apart
//! along the section normal by rigid transforms (re-minted witnesses,
//! #84) and rendered as a machinist's section pair.
//!
//! Replaces the void box's translucency hack as the honest way to
//! show an interior. Feasibility probed 2026-07-25: split of a
//! boolean result passes all tiers on both sides, on main.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::geom_core::{Affine3, Point3, Vec3};
use pncad::topo::splitting::{SplitPart, SplitPlane, split};

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::geom_core::Tol;

/// The narration numbers `build` reports alongside the halves:
/// (v_above, v_below, v_box, gap).
pub(crate) type SectionNumbers = (f64, f64, f64, f64);

/// The split + explode, generic (the Probe sweep runs the same ops):
/// returns the two moved halves and the [`SectionNumbers`].
pub(crate) fn build<S: Scalar>(
    boxbody: &pncad::topo::Body<S>,
    tol: Tol,
) -> ((pncad::topo::Body<S>, pncad::topo::Body<S>), SectionNumbers) {
    // A tilted section plane through the box interior: normal
    // (0.75, 0.1875, 1) — no axis alignment, crosses walls, bosses,
    // and cavity floor.
    let normal = Vec3::new(S::from_f64(0.75), S::from_f64(0.1875), S::from_f64(1.0));
    let plane = SplitPlane {
        origin: Point3::new(S::from_f64(1.5), S::from_f64(1.0), S::from_f64(0.75)),
        normal,
    };
    let res = split(boxbody, &plane, tol).expect("split of the boolean-result box");
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&res.above, &res.below) else {
        panic!("the section plane crosses the box: both sides must be bodies");
    };

    // Volumes: the two halves partition the box exactly.
    let v_box = pncad::topo::mass_properties(boxbody, tol)
        .expect("box props")
        .volume
        .f();
    let v_above = pncad::topo::mass_properties(above, tol)
        .expect("above props")
        .volume
        .f();
    let v_below = pncad::topo::mass_properties(below, tol)
        .expect("below props")
        .volume
        .f();
    let gap = (v_above + v_below - v_box).abs();
    assert!(
        gap < 1e-9,
        "split halves must partition the volume (gap {gap:.3e})"
    );

    // Pull the halves apart along the (unnormalized) section normal:
    // rigid transforms re-mint every moved witness (#84).
    let n = normal * (S::from_f64(0.75) / normal.norm());
    let moved_above = pncad::topo::transform_rigid(above, &Affine3::translation(n), tol)
        .expect("translate above half");
    let moved_below = pncad::topo::transform_rigid(below, &Affine3::translation(-n), tol)
        .expect("translate below half");
    ((moved_above, moved_below), (v_above, v_below, v_box, gap))
}

pub fn stops(boxbody: &pncad::topo::Body<f64>, tol: Tol) -> Vec<Stop> {
    let ((moved_above, moved_below), (v_above, v_below, v_box, gap)) = build(boxbody, tol);
    let note = format!(
        "first `topo::split` in the tour, ON a 15-op boolean result; section plane \
         normal (0.75, 0.1875, 1) — tilted, no axis alignment; minted section faces \
         on both sides; halves partition the volume exactly ({v_above:.6} + \
         {v_below:.6} = {v_box:.6}, gap {gap:.1e}); halves then moved apart by \
         rigid transforms (edge witnesses re-minted, #84) and revalidated"
    );
    vec![Stop {
        name: "cutaway",
        caption: "cutaway (split + move)".to_string(),
        montage: true,
        story: "the project box split by a tilted plane and pulled apart — a \
                machinist's section pair showing bosses, pockets, and wall sections",
        ops: "topo::split(projectbox, tilted plane) -> 2 bodies -> 2 transform nodes",
        delta: 1e-2,
        note: Some(note),
        // Split output is not a boolean result: no declared contacts
        // ride it, so both halves go through the plain tier-3 gate.
        // Camera chosen so the section normal is ~48 degrees off the
        // view direction (#91 revision note 6): the below half's
        // section faces the camera and the cut interior (cavity,
        // vents, boss sections) is visible, while the halves still
        // separate laterally on screen.
        view: View {
            elev: 20.0,
            azim: 55.0,
            up: 'z',
        },
        bodies: vec![
            SceneBody::plain("cutaway_above", [0.40, 0.60, 0.72], moved_above),
            SceneBody::plain("cutaway_below", [0.78, 0.60, 0.35], moved_below),
        ],
    }]
}
