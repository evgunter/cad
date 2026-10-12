//! A curved wall resting on a plane face's interior along a ruling
//! (`work/cleave/tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line.md`).
//!
//! A rod (r = 0.5, z ∈ [0.25, 0.75]) rests on the side face y = 2 of a
//! 4 × 4 × 1 plate, tangent along the ruling x = 0, y = 2. The union is
//! the two solids touching along that ruling through the plate face's
//! interior, a face–face curve touch no record the reduction mints can
//! name (`crates/topo/README.md` C7). Two vertex-on-face records would
//! certify only the line's two ends, and a later union bridging the rod
//! to the plate would fuse that answer into one shell with an edgeless
//! contact every at-rest gate passes. So the union refuses
//! `TangentSlitArmUnbuilt`, at `f64` and at `Interval`; subtract and
//! intersect owe no record of the touch and still answer.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::test_support::finished;
use sweep::{Extrusion, extrude};
use topo::boolean::BooleanResult;
use topo::{AtRestPolicy, Body, BooleanError, Operand, intersect, subtract, union};

type Loop = Vec<((f64, f64), f64)>;

fn ext<T: geom_core::Decide + AtRestPolicy>(loops: Vec<Loop>, z0: f64, h: f64) -> Body<T> {
    let tol = Tol::witness();
    let lps = loops
        .into_iter()
        .map(|l| {
            bulge_loop(
                l.into_iter()
                    .map(|((x, y), b)| (Point2::new(x, y).map(T::from_f64), T::from_f64(b)))
                    .collect(),
            )
        })
        .collect();
    let vp = Profile::new(SketchPlane::<T>::xy(), lps)
        .validate(tol)
        .unwrap();
    let b = extrude(
        &vp,
        Extrusion::Distance {
            depth: T::from_f64(h),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z0).map(T::from_f64));
    topo::transform_rigid(
        &b,
        &geom_core::RigidMotion::check_rigid(&up, geom_core::Band::linear(tol).unwrap()).unwrap(),
        tol,
    )
    .unwrap()
}

fn plate<T: geom_core::Decide + AtRestPolicy>() -> Body<T> {
    let square = vec![
        ((-2.0, -2.0), 0.0),
        ((2.0, -2.0), 0.0),
        ((2.0, 2.0), 0.0),
        ((-2.0, 2.0), 0.0),
    ];
    ext(vec![square], 0.0, 1.0)
}

fn rod<T: geom_core::Decide + AtRestPolicy>() -> Body<T> {
    ext(vec![vec![((-0.5, 2.5), 1.0), ((0.5, 2.5), 1.0)]], 0.25, 0.5)
}

/// The union refuses, naming the plate as the operand whose face the
/// locus runs through; subtract answers the plate (the rod only touches
/// it) and intersect answers nothing.
fn the_tangent_rest<T: geom_core::Decide + geom_core::Bounds + AtRestPolicy>(
    volume: impl Fn(&Body<T>) -> (f64, f64),
) {
    let tol = Tol::witness();
    let (a, b) = (
        finished("the plate", plate::<T>(), tol),
        finished("the rod", rod::<T>(), tol),
    );
    match union(&a, &b, tol) {
        Err(BooleanError::TangentSlitArmUnbuilt { interior, .. }) => {
            assert_eq!(interior, Operand::A, "union: the plate's face");
        }
        other => panic!("union: TangentSlitArmUnbuilt, got {:?}", other.map(|_| ())),
    }
    match subtract(&a, &b, tol) {
        Ok(BooleanResult::Body(r)) => {
            let (lo, hi) = volume(&r.body);
            assert!(
                lo - 1e-9 <= 16.0 && 16.0 <= hi + 1e-9,
                "subtract: the plate's volume, got [{lo}, {hi}]"
            );
        }
        other => panic!("subtract: the plate, got {:?}", other.map(|_| ())),
    }
    assert!(
        matches!(intersect(&a, &b, tol), Ok(BooleanResult::Empty)),
        "intersect: empty"
    );
}

#[test]
fn a_tangent_rest_on_a_face_interior_refuses_its_union() {
    the_tangent_rest::<f64>(|b| {
        let v = topo::mass_properties(b, Tol::witness()).unwrap().volume;
        (v, v)
    });
}

#[test]
fn a_tangent_rest_on_a_face_interior_refuses_its_union_at_interval() {
    use geom_core::Bounds;
    the_tangent_rest::<geom_core::Interval>(|b| {
        let v = topo::mass_properties(b, Tol::witness()).unwrap().volume;
        (v.lo(), v.hi())
    });
}
