//! The window walk's first image, placed: [`super::run_azimuth_images`]
//! derives each image over its own edge's span in the chart's principal
//! window (C4), so started at every member of a loop whose carriers are
//! stated whole periods off, it reads its principal twin's window.

use geom_core::{Band, Tol};

use super::{azimuth_hull, run_azimuth_images};
use crate::pcurves::pole_slit_tests::{cylinder_patch_below, slit_cap_below};
use crate::{Body, FaceKey, LoopBoundary};

/// The face's outer cycle walked from each of its members in turn, and
/// the window each walk reads.
fn windows_from_every_start(body: &Body<f64>, face: FaceKey, band: Band) -> Vec<(f64, f64)> {
    let data = body.get_face(face).unwrap();
    let surface = body.get_surface(data.surface).unwrap().clone();
    let LoopBoundary::Cycle { first } = body.get_loop(data.outer).unwrap().boundary else {
        panic!("the outer loop is a cycle");
    };
    let halves = body.loop_walk(first).closed("loop", first);
    (0..halves.len())
        .map(|start| {
            let rotated: Vec<_> = halves[start..]
                .iter()
                .chain(&halves[..start])
                .copied()
                .collect();
            let images = run_azimuth_images(body, &surface, face, &rotated, band).unwrap();
            azimuth_hull(&images).unwrap()
        })
        .collect()
}

fn assert_twins(far: &[(f64, f64)], near: &[(f64, f64)], what: &str) {
    assert_eq!(far.len(), near.len(), "{what}");
    for (start, (w, n)) in far.iter().zip(near).enumerate() {
        assert!(
            (w.0 - n.0).abs() < 1e-12 && (w.1 - n.1).abs() < 1e-12,
            "{what}, walked from member {start}: {w:?} against {n:?}"
        );
    }
}

/// The cylinder patch 5 and 10 periods below, each face, and the slit
/// cap 3 and 10 below: from whichever member the walk starts, its first
/// image is placed in the chart's principal window over its own span,
/// so the window is the twin's walked from the same member. The half-turn patch reads
/// one window from every start; the cap wraps a whole turn, so where
/// its window sits is the start's own azimuth.
#[test]
fn the_window_walk_reads_a_far_stated_face_in_its_twins_window_from_any_start() {
    let band = Band::linear(Tol::witness()).unwrap();
    let twin = cylinder_patch_below(0.0).unwrap();
    for (f, _) in twin.faces() {
        let near = windows_from_every_start(&twin, f, band);
        assert_twins(
            &near,
            &vec![near[0]; near.len()],
            &format!("the twin's {f:?}"),
        );
        for periods in [5.0, 10.0] {
            let body = cylinder_patch_below(periods).unwrap();
            let far = windows_from_every_start(&body, f, band);
            assert_twins(&far, &near, &format!("{periods} periods, {f:?}"));
        }
    }
    let (twin, cap, _) = slit_cap_below(0.0).unwrap();
    let near = windows_from_every_start(&twin, cap, band);
    for periods in [3.0, 10.0] {
        let (body, far_cap, _) = slit_cap_below(periods).unwrap();
        let far = windows_from_every_start(&body, far_cap, band);
        assert_twins(&far, &near, &format!("the cap {periods} periods below"));
    }
}
