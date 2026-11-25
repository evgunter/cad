//! **An approximated face read through a motion is the moved face**:
//! the cap of [`box_with_approx_cap`] read in a skewed frame through its
//! motion, the box moved by that motion and read where it stands, and
//! the box capped with the fit as a plain NURBS face read through the
//! motion all sweep alike against a brick moved with them, under a
//! clearance bound that holds and one that is broken. Red if the
//! window's moved arm reads the approximated carrier as anything but its
//! placed fit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::Surface;
use geom_core::{Band, RigidMotion, Tol, UnitVec3, Vec3};
use topo::clearance::{
    BoundVerdict, Budgets, ClearanceBound, DEFAULT_MAX_CELL_DEPTH, Side, Sweep, Which,
};
use topo::{Body, FaceKey};

use crate::common::approx::{box_with_approx_cap, box_with_spline_cap};

#[test]
fn an_approximated_cap_read_through_its_motion_sweeps_as_the_moved_cap() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let axis = UnitVec3::new(Vec3::new(1.0, -2.0, 0.5), "a skew axis", band).unwrap();
    let m = RigidMotion::translation(Vec3::new(7.0, -3.0, 4.0))
        .unwrap()
        .compose(RigidMotion::rotation(axis, 0.9).unwrap())
        .unwrap();
    let (approx, cap) = box_with_approx_cap(0.05, 1e-9);
    let Surface::Approx(a) = approx.face(cap).unwrap().surface().unwrap() else {
        panic!("approx cap")
    };
    let (nurbs, ncap) = box_with_spline_cap(Surface::Nurbs(Arc::new(a.fit().clone())));
    let moved: Body<f64> = topo::transform_rigid(&approx, &m).unwrap();
    assert!(matches!(
        moved.face(cap).unwrap().surface().unwrap(),
        Surface::Approx(_)
    ));
    let brick: Body<f64> = topo::transform_rigid(
        &topo::transform_rigid(
            &sweep::test_support::block::<f64>(0.2, 0.2, 0.2, tol),
            &RigidMotion::translation(Vec3::new(0.9, 0.9, 0.77)).unwrap(),
        )
        .unwrap(),
        &m,
    )
    .unwrap();
    let wb = Side {
        body: &brick,
        faces: brick.faces().map(|(k, _)| k).collect(),
    }
    .windows(Which::Second)
    .unwrap();
    for (c, holds) in [(0.02, true), (0.05, false)] {
        let sweep = Sweep {
            bound: ClearanceBound::at_least(c),
            band,
            budgets: Budgets {
                max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
                max_cell_pairs: 200_000,
            },
        };
        let run = |body: &Body<f64>, face: FaceKey, at: Option<&RigidMotion<f64>>| {
            let wa = Side {
                body,
                faces: vec![face],
            }
            .windows_at(Which::First, at)
            .unwrap();
            sweep.run(&wa, &wb, false, |_| {
                if holds {
                    Err("no witness".to_owned())
                } else {
                    Ok::<(), String>(())
                }
            })
        };
        let (placed, through, fit) = (
            run(&moved, cap, None),
            run(&approx, cap, Some(&m)),
            run(&nurbs, ncap, Some(&m)),
        );
        match (&placed.verdict, holds) {
            (BoundVerdict::Holds, true) | (BoundVerdict::Violated(()), false) => {}
            (v, _) => panic!("c = {c}: placed reads {v:?}"),
        }
        assert_eq!(
            (&placed.verdict, &placed.receipt),
            (&through.verdict, &through.receipt),
            "c = {c}: through the motion"
        );
        assert_eq!(
            (&through.verdict, &through.receipt),
            (&fit.verdict, &fit.receipt),
            "c = {c}: as its fit"
        );
    }
}
