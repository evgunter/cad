//! The fold at a far gap: a face whose loop is walked from a carrier
//! stated whole periods off its azimuth extracts a polygon that many
//! periods out, and [`super::cylinder_pair_overlap`]'s fold
//! ([`super::period_fold`]) brings it onto its principal twin.

use geom_core::{Band, Tol};

use crate::contact::ContactVerdict;
use crate::pcurves::pole_slit_tests::cylinder_patch_below;
use crate::{Body, ChartOverlap, LoopBoundary};

/// Every start of the half-turn patch's own face, 5 and 10 periods
/// below, overlaps its principal twin with positive area, the walk that
/// starts on the far-stated arc included.
#[test]
fn a_face_walked_from_a_far_stated_arc_folds_onto_its_twin() {
    let band = Band::linear(Tol::witness()).unwrap();
    let twin = cylinder_patch_below(0.0).unwrap();
    let (face, _) = twin.faces().next().unwrap();
    for periods in [5.0, 10.0] {
        let built = cylinder_patch_below(periods).unwrap();
        let outer = built.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = built.get_loop(outer).unwrap().boundary else {
            panic!("the outer loop is a cycle");
        };
        let members = built.loop_walk(first).closed("loop", first);
        let mut far = 0;
        for &start in &members {
            let mut body: Body<f64> = built.clone();
            body.get_loop_mut(outer).unwrap().boundary = LoopBoundary::Cycle { first: start };
            let surface = body
                .get_surface(body.get_face(face).unwrap().surface)
                .unwrap()
                .clone();
            let uv = super::extract_face_uv(&body, face, &surface, super::ChartRead::Minted, band)
                .unwrap();
            if uv.outer.iter().any(|p| p.x.abs() > 20.0) {
                far += 1;
            }
            assert_eq!(
                super::cylinder_pair_overlap(
                    &body,
                    face,
                    &twin,
                    face,
                    ContactVerdict::Definite,
                    band
                ),
                Ok(ChartOverlap::PositiveArea),
                "{periods} periods, walked from {start:?}"
            );
        }
        assert!(
            far > 0,
            "{periods} periods: some start reads the polygon far out"
        );
    }
}
